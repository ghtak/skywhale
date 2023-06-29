mod simple_http_server;
use simple_http_server::run_server;

mod nonnull;
use nonnull::address_check;


fn main(){
    address_check()
}
