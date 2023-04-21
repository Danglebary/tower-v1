use hyper::{service::make_service_fn, Server};
use std::{convert::Infallible, net::SocketAddr};
use tower_v1::service::hello_world::HelloWorldService;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Construct our SocketAddr to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // MakeService to handle each connection
    let make_service = make_service_fn(|_conn| async {
        let svc = HelloWorldService::new();

        Ok::<_, Infallible>(svc)
    });

    // Then bind and serve
    let server = Server::bind(&addr).serve(make_service);

    // And run forever
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
