use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use actix_rt::System;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use log::info;

use server::MyWebSocket;

use crate::{config::Config, server};

/// To check service state
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

async fn version() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}

async fn exit_pro() -> impl Responder {
    HttpResponse::Ok()
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

pub(crate) fn web_runner(config_path: PathBuf, log_path: PathBuf) -> (Server, System) {
    let (tx, rx) = mpsc::channel();

    Config::init_logging(log_path);

    let port = Config::get_server_port(config_path);

    info!("starting HTTP server at http://localhost:{}", port);

    thread::spawn(move || {
        let sys = System::new();

        let srv = HttpServer::new(|| {
            App::new()
                .service(web::resource("/").to(index))
                .service(web::resource("/version").to(version))
                .service(web::resource("/exit").to(exit_pro))
                .service(web::resource("/ws").route(web::get().to(echo_ws)))
                .wrap(middleware::Logger::default())
        })
        .system_exit()
        .disable_signals()
        .bind(("0.0.0.0", port))
        .unwrap()
        .run();

        let _ = tx.send((srv, System::current()));
        let _ = sys.run();
    });
    let (srv, sys) = rx.recv().unwrap();
    (srv, sys)
    // srv.handle().stop(false);
    // thread::sleep(Duration::from_millis(1000));
    // let _ = sys.stop();
}
