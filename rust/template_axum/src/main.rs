mod error;

use axum::{
    routing::get,
    Router
};
use dotenv::dotenv;
use crate::error::ErrorCode;
use rand::Rng;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").expect("PORT must be set.");

    let main_router = Router::new().route("/", get(get_impl));

    let addr = format!("0.0.0.0:{}", port);

    tracing::debug!("listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(main_router.into_make_service())
        .await
        .unwrap();
    /*
    let code0 = Code{
        category:&Category::DEFAULT,
    };
    let code1 = Code{
        category:&Category::DEFAULT,
    };

    code0.print();
    code1.print();
     */
}


async fn random_hello_world() -> Result<&'static str, ErrorCode> {
    let mut rng = rand::thread_rng();
    return if rng.gen::<i32>() % 2 == 0 {
        Ok("Hello World")
    } else {
        Err(ErrorCode::INVALID_PARAMETER)
    }
}

async fn get_impl() -> Result<&'static str, ErrorCode> {
    let ret = random_hello_world().await?;
    return Ok(ret)
}

struct Category{}

impl Category {
    pub const DEFAULT: Category = Category{};

    pub fn print(&self) -> &'static str{
        "Category"
    }
}

struct Code{
    category : &'static Category,
}

impl Code{
    pub fn print(&self) {
        println!("{:?}", self.category.print());
    }
}
