#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use actix_web::{
    web, get, middleware,
    HttpServer, HttpResponse, Responder, 
    App,
};
use std::sync::Arc;
use tokio::time::timeout;
use lazy_static::lazy_static;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;

#[crud_table(table_name:testing.users)]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            email: None,
            first_name: None,
            last_name: None,
            username: None
        }
    }
}

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[get("/rb/webdata")] // curl -v 127.0.0.1:8080/rb/webdata |jq -r '.'
async fn index(rb: web::Data<Arc<Rbatis>>) -> impl Responder {
    let v = rb.fetch::<serde_json::Value>("select * from nyc_weather limit 0, 10", vec![]).await.unwrap();
    HttpResponse::Ok().insert_header(("Content-Type", "text/json;charset=UTF-8")).json(v)
}

#[get("/rb/static")] // curl -v 127.0.0.1:8080/rb/static |jq -r '.'
async fn index1() -> impl Responder {
    let v = RB.fetch_list::<User>().await.unwrap_or_default();
    HttpResponse::Ok().insert_header(("Content-Type", "text/json;charset=UTF-8")).json(v)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let mut a = vec![100;100];
    println!("{:}", a.len());
    // {
    //     a;
    // }
    drop(a);
    a = vec![2000;100];
    println!("{:}", a.len());


    let result = tokio::select! {
        _ = timeout(std::time::Duration::from_millis(1000), tokio::time::sleep(std::time::Duration::from_millis(1000))) => {
            println!("function timeout");
            "function timeout"
        } 
        _ = tokio::time::sleep(std::time::Duration::from_millis(1500)) => {
            println!("select timeout");
            "select timeout"
        }
    };
    println!("{}", result);

    
    let rb = rbatis::rbatis::Rbatis::new();
    rb.link("sqlite://../rs060/target/weather.db").await.unwrap();
    let result = rb.fetch::<serde_json::Value>("select * from nyc_weather limit 0, 10", vec![]).await?;
    for r in result.as_array().unwrap() {
        println!("{}", r);
    }
    

    RB.link("postgres://test_user:testing@localhost:5432/testing_db").await.unwrap();
    let result = RB.fetch::<serde_json::Value>("select * from testing.users limit 10 offset 0", vec![]).await.unwrap();
    let records = result.as_array().unwrap();
    println!("{:?}", records);
    for r in records.iter() {
        println!("{}", r);
    }
    for r in RB.fetch_list::<User>().await.unwrap() {
        println!("{:?}", r);
    }
    let rb_static_arc = Arc::new(rb);
    HttpServer::new(move||{
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(rb_static_arc.clone()))
            .route("/", web::get().to(HttpResponse::Ok))
            .service(index)
            .service(index1)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
