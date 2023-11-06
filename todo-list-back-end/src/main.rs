mod Handlers;


use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use actix_cors::Cors;
use Handlers::Handlers::get_handler;
use Handlers::Handlers::sign_in;
use Handlers::Handlers::sign_up;
use Handlers::Handlers::task_creation;


fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/Sign-Up").route(web::post().to(sign_up)));
}

fn configure_task(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create-task").route(web::post().to(task_creation)));
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(Cors::default().allow_any_origin().allow_any_header()
            .allow_any_method().max_age(3600))
            .route("/", web::get().to(get_handler))
            .route("/Sign-In", web::get().to(sign_in))
            .configure(configure)
            .configure(configure_task)
    }).bind(("127.0.0.1", 8080)).unwrap().run().await
}
