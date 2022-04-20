use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use futures_util::future::try_join_all;
use redis_async::{resp::RespValue, resp_array};
use serde::Deserialize;

use actix_web::{
    web, middleware, error,
    HttpServer, HttpResponse,
    App, Result
};

#[derive(Deserialize)]
pub struct CacheInfo {
    one: String,
    two: String,
    three: String,
}

async fn cache_stuff(info: web::Json<CacheInfo>, redis: web::Data<Addr<RedisActor>>)
    -> Result<HttpResponse> {
        let info = info.into_inner();

        let one = redis.send(Command(resp_array!["SET", "mydomain:one", info.one]));
        let two = redis.send(Command(resp_array!["SET", "mydomain:two", info.two]));
        let three = redis.send(Command(resp_array!["SET", "mydomain:three", info.three]));

        let res = try_join_all([one, two, three])
            .await
            .map_err(error::ErrorInternalServerError)?
            .into_iter()
            .map(|item| item.map_err(error::ErrorInternalServerError))
            .collect::<Result<Vec<_>, _>>()?;
        
        if res.iter().all(|res| matches!(res, RespValue::SimpleString(x) if x == "OK")) {
            Ok(HttpResponse::Ok().body("successfully cached values"))
        } else {
            Ok(HttpResponse::InternalServerError().finish())
        }
}

async fn del_stuff(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse> {
    let res = redis.send(Command(resp_array![
        "DEL",
        "mydomain:one",
        "mydomain:two",
        "mydomain:three"
    ]))
    .await
    .map_err(error::ErrorInternalServerError)?
    .map_err(error::ErrorInternalServerError)?;
    match res {
        RespValue::Integer(x) if x == 3 => {
            Ok(HttpResponse::Ok().body("successfully deleted values"))
        }
        _ => {
            log::error!("{:?}", res);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(||{
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        App::new()
            .app_data(web::Data::new(redis_addr))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(HttpResponse::Ok))
            .service(
                web::resource("/stuff")
                    .route(web::post().to(cache_stuff)) // curl -s localhost:8080/stucation/json' -d '{"one": "1", "two": "2", "three": "3"}' -v
                    .route(web::delete().to(del_stuff)) // curl -s -XDELETE localhost:8080/stuff
            )
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}