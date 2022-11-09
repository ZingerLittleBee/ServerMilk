use crate::{config::Config, server};

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use log::info;

use server::MyWebSocket;

/// To check service state
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

async fn version() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

pub(crate) async fn web_runner() -> std::io::Result<()> {
    Config::init_logging();

    let port = Config::get_server_port();

    info!("starting HTTP server at http://localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/version").to(version))
            // websocket route
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
