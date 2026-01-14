use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind address");
    run(listner)?.await
}
