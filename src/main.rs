use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpResponse, HttpServer};

mod core;
mod shared;
mod webserver;

use webserver::controllers::config_all_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    println!("MASTER: {:?}", secret_key.master());
    println!("ENCRYPTION: {:?}", secret_key.encryption());
    println!("SIGNING: {:?}", secret_key.signing());

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .configure(config_all_routes)
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
