# Axum HTTP Server Module Implementation Plan

## Goal

`skywhale_core` 사용자에게 Axum `Router`와 기존 `HttpConfig`를 넘겨 HTTP
서버를 실행하는 작은 편의 API를 제공한다. 첫 구현은 TCP 바인딩, 서버 실행,
바인딩 주소 로그, 운영체제 종료 신호에 대한 정상 종료만 담당한다.

## Scope and boundaries

### Included in the first implementation

- `Router`와 `HttpConfig`를 받는 공개 비동기 실행 함수
- `HttpConfig::host`와 `HttpConfig::port`를 `SocketAddr`로 변환하고 `TcpListener`에 바인딩
- Axum 서버 실행 및 실제 바인딩 주소의 `tracing::info!` 로그
- Ctrl-C 및 Unix의 `SIGTERM`을 기다린 뒤 새 연결 수락을 멈추는 graceful shutdown
- 바인딩 실패, 잘못된 호스트 주소, 서버 실행 실패를 기존 `skywhale_core::Error`로 전파
- 선택한 공개 API와 종료 동작을 검증하는 테스트

### Explicitly excluded

- 기본 라우트, health check, metrics, 인증/인가, CORS, TLS의 자동 추가
- 요청별 tracing, timeout, body-size 제한, rate limit 같은 라우터 미들웨어 강제
- 설정 파일 구조 변경 및 새로운 HTTP 설정 항목
- 애플리케이션 바이너리, 배포 설정, 실제 네트워크 포트를 사용하는 통합 테스트

실행 모듈은 서버 lifecycle만 소유한다. 라우팅과 HTTP 정책은 호출자가 완성한
`Router`에 남긴다. 따라서 이 모듈은 Axum의 `Router`를 감싸는 별도 추상화나
자체 라우터 빌더를 만들지 않는다.

## Target public API

초기 API 후보는 다음과 같다. 구현 시 Axum 및 Tokio의 정확한 trait 경계에 맞춰
반환 타입만 조정할 수 있지만, 호출 방식은 유지한다.

```rust
use axum::Router;
use skywhale_core::{HttpConfig, Result};

pub async fn serve(router: Router, config: &HttpConfig) -> Result<()>;
```

사용 예시는 다음과 같다.

```rust
let router = Router::new().route("/health", axum::routing::get(health));

skywhale_core::http::serve(router, &config.http).await?;
```

`HttpConfig`를 참조로 받는 이유는 이미 `SkywhaleConfig`의 일부로 로드되며,
실행 중 설정을 소유할 필요가 없기 때문이다. 주소 문자열은 공개 설정 구조를
바꾸지 않고 `SocketAddr` 파싱 시점에 검증한다. `host:port` 문자열 조합 대신
`(host.as_str(), port)`를 통해 `tokio::net::ToSocketAddrs`를 사용하면 IPv4,
IPv6, DNS 이름을 수용할 수 있다. DNS 이름을 지원하지 않기로 결정하면 구현
전에 `IpAddr` 파싱으로 제한하고 그 결정을 API 문서에 명시한다.

## File layout

- Modify: `skywhale/skywhale-core/Cargo.toml` — Axum, Tokio 및 graceful
  shutdown에 필요한 feature를 직접 의존성으로 선언한다.
- Create: `skywhale/skywhale-core/src/http.rs` — 공개 `serve` 함수와 비공개
  종료 신호 대기 함수를 둔다.
- Modify: `skywhale/skywhale-core/src/lib.rs` — `pub mod http;`를 추가한다.
- Modify only if needed: `skywhale/skywhale-core/src/error.rs` — 호출자가
  재시도/표시 정책을 달리해야 하는 오류가 생길 때만 전용 variant를 추가한다.

## Implementation steps

1. 의존성 버전과 feature를 현재 Axum 문서 및 프로젝트의 MSRV 정책에 맞춰 결정한다.
   `axum`의 서버 API와 `tokio`의 `net`, `signal`, `macros`, runtime feature가
   충분한지 확인한다. 기존 `tokio` dev-dependency는 runtime용 일반 dependency로
   승격하거나 필요한 feature를 통합한다.

2. `http` 모듈의 공개 문서와 실패 조건을 먼저 작성한다. `serve`는 listener가
   만들어지고 서버가 끝날 때까지 반환하지 않으며, 반환 성공은 정상 종료를
   뜻한다는 계약을 명확히 한다.

3. 실패하는 테스트를 추가한다.

   - **기본 라우터 연동:** `Router`는 Tower `Service`이므로 `tower::ServiceExt`
     (`util` feature)의 `.oneshot(request)`로 HTTP 요청 한 건을 직접 전달한다.
     이 테스트는 listener나 실제 포트 없이 route 연결, handler, router에 붙인
     middleware의 응답을 검증한다. 구현 시 `tower = { version = "…",
     features = ["util"] }`를 `dev-dependencies`에 추가하고, Axum이 사용하는
     Tower 버전과 호환되는지 확인한다.

     ```rust
     use axum::{
         body::Body,
         http::{Request, StatusCode},
         routing::get,
         Router,
     };
     use tower::ServiceExt;

     #[tokio::test]
     async fn router_handles_a_basic_request() {
         let app = Router::new().route("/health", get(|| async { StatusCode::OK }));

         let response = app
             .oneshot(
                 Request::builder()
                     .uri("/health")
                     .body(Body::empty())
                     .expect("valid test request"),
             )
             .await
             .expect("router response");

         assert_eq!(response.status(), StatusCode::OK);
     }
     ```

     `.oneshot()`은 Service를 소비하므로, 여러 요청을 보낼 테스트에서는 router를
     `clone()`하거나 매 요청마다 새 router를 만든다. 이 방식은 `serve`의 listener
     바인딩·실제 종료 신호·socket 수락을 검증하지 않으므로 아래 lifecycle 테스트와
     대체하지 않는다.

   - 잘못된 `host` 또는 해석 불가능한 주소가 `Err`가 되는지
   - 사용 중인 로컬 포트에 바인딩하면 `Err`가 되는지
   - 종료 future가 완료되면 서버 future가 정상 반환하는지

   운영체제 신호 자체는 테스트에서 보내지 않는다. 대신 내부적으로
   `serve_with_shutdown(router, config, shutdown_future)` 같은 `pub(crate)`
   helper를 두어 완료 가능한 테스트 future를 주입한다. 공개 `serve`만 실제
   signal future를 전달한다.

4. `HttpConfig`에서 listener를 바인딩한다. 바인딩 주소는 `listener.local_addr()`로
   읽어 로그에 기록하여 포트 `0`을 쓸 때에도 실제 포트를 알 수 있게 한다.
   컨텍스트를 붙인 I/O 오류는 `?`로 기존 `Error::Other`에 보존한다.

5. `axum::serve(listener, router)`에 graceful shutdown future를 연결한다.
   Windows에서는 Ctrl-C를, Unix에서는 Ctrl-C와 `SIGTERM`을 모두 기다린다.
   `cfg(unix)`로 platform-specific signal 코드를 격리해 Windows 빌드를 유지한다.

6. crate root에서 `pub mod http;`로 노출하고 rustdoc 예제가 실제 공개 경로를
   사용하도록 확인한다. 기본 `Router`에 미들웨어나 라우트를 추가하지 않는다.

7. `cargo fmt --check`, `cargo test --workspace`,
   `cargo clippy --workspace --all-targets -- -D warnings`, `cargo doc --workspace
   --no-deps`, `git diff --check`를 실행한다. listener를 직접 열어야 하는 테스트는
   병렬 실행 및 포트 충돌을 피하도록 port `0`을 사용한다.

## Error and logging policy

- 오류를 로그와 반환으로 중복 보고하지 않는다. `serve`는 오류에 실행 문맥을
  추가해 반환하고, 최종 애플리케이션 entry point가 한 번 기록한다.
- 주소 문자열과 포트가 잘못되었거나 바인딩이 실패한 경우는 현재의
  `Error::Other(anyhow::Error)`로 충분하다. 호출자가 오류 종류에 따라 다른
  행동을 해야 한다는 구체적 요구가 생길 때만 HTTP 전용 error variant를 검토한다.
- 서버 시작 및 정상 종료는 각각 `info` 수준으로 남긴다. 요청 로그는 기존 tracing
  설정을 존중하되, 초기 실행 모듈이 강제로 추가하지 않는다.

## Deferred additions (reference only)

아래는 현재 구현 대상이 아니다. 실제 요구가 생길 때 영향 범위와 기본값을 함께
결정한 뒤 별도 변경으로 추가한다.

| Need | Candidate addition | Decision points |
| --- | --- | --- |
| 종료 유예 시간 | shutdown deadline 및 강제 종료 정책 | 진행 중 요청의 최대 대기 시간, timeout 후 반환 오류 |
| 요청 관측성 | `TraceLayer` 기반 요청 ID·상태·지연 로그 | 개인정보가 포함된 header/path 기록 금지 정책 |
| 자원 제한 | request timeout, concurrency limit, body limit | 서비스별 기본값, streaming/WebSocket 예외 |
| 운영 상태 확인 | opt-in health/readiness route helper | 경로 소유권, DB 등 의존성 readiness 기준 |
| CORS/security headers | 별도 opt-in router layer helper | 허용 origin·credentials·캐시 정책 |
| TLS | rustls listener 또는 reverse proxy 문서화 | 인증서 갱신, HTTP/2, 프록시 종료 지점 |
| 프록시 지원 | trusted proxy 설정과 forwarded header 처리 | 신뢰 CIDR, spoofing 방지, client IP 계약 |
| 테스트 편의 | bound listener 또는 server handle 반환 API | 기존 `serve`의 단순성 유지와 lifecycle 소유권 |

## Acceptance criteria

- 사용자는 완성된 Axum `Router`와 기존 `HttpConfig`만으로 서버를 실행할 수 있다.
- host/port 설정이 listener 바인딩에 반영되고, 포트 `0`일 때 실제 주소가 로그에 남는다.
- 바인딩과 주소 해석 실패는 호출자에게 context를 유지한 오류로 반환된다.
- Ctrl-C, 그리고 Unix의 `SIGTERM`에서 새 연결 수락을 멈추고 서버 future가 정상 완료한다.
- 모듈이 기본 라우트·미들웨어·보안 정책을 몰래 추가하지 않는다.
- workspace format, test, clippy, doc 검증을 통과한다.
