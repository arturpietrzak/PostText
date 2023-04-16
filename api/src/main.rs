use api::http;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = http::Config {
        address: SocketAddr::from(([127, 0, 0, 1], 8080)),
        database_url: String::from("mysql://root:@localhost:3306/post_text"),
        max_db_connections: 10,
    };

    http::serve(config).await;
}
