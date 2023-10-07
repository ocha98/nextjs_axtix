use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{web, App, HttpServer, cookie::Key};
use auth::GithubOAuthToken;
use diesel::{r2d2, prelude::*};
use dotenv::dotenv;

mod auth;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

fn init_db_pool() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let github_oauth_token = web::Data::new(GithubOAuthToken {
        client_id: std::env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_OAUTH_CLIENT_ID not set"),
        client_secret: std::env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("GITHUB_OAUTH_CLIENT_SECRET not set"),
    });

    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";

    let db_pool = init_db_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(redis_connection_string),
                    secret_key.clone()
                )
            )
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::clone(&github_oauth_token))
            .service(auth::callback_github)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}