#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate log;
// #[macro_use]
// extern crate serde_json;
// #[macro_use]
// extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use dotenv::dotenv;
use redis::cluster::ClusterClient;
use redis::Commands;
use std::env;
use sqlx::any::AnyPoolOptions;
use sqlx::{Any, Pool};

#[get("/")]
async fn hello(redis_pool: Data<r2d2::Pool<ClusterClient>>) -> impl Responder {
    let mut conn = redis_pool.get().unwrap();
    let s: String = conn.get("hello").unwrap();
    HttpResponse::Ok().body(s)
}

#[get("/db")]
async fn db(db_pool: Data<Pool<Any>>) -> impl Responder {
    let row: (String,) = sqlx::query_as(r#"SELECT version() v"#)
        .fetch_one(&**db_pool)
        .await.unwrap();
    let s = row.0;
    HttpResponse::Ok().body(s)
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

lazy_static! {
    pub static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    env_logger::init();
    let db_pool: Pool<Any> = AnyPoolOptions::default()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await?;

    let nodes = vec![
        "redis://127.0.0.1:7001/",
        "redis://127.0.0.1:7002/",
        "redis://127.0.0.1:7003/",
        "redis://127.0.0.1:7004/",
        "redis://127.0.0.1:7005/",
        "redis://127.0.0.1:7006/",
    ];
    let redis_client = ClusterClient::open(nodes).expect("redis not ready");

    let redis_pool = r2d2::Pool::builder().build(redis_client).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .service(hello)
            .service(db)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
