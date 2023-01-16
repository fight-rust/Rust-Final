use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())

    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}
