use actix_web::{
    http, web, guard, get, App, HttpServer, 
    HttpRequest, HttpResponse, Responder,
    Result, middleware,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    age: u32,
}

struct ContentTypeHeader;

impl guard::Guard for ContentTypeHeader {
    fn check(&self, ctx: &guard::GuardContext) -> bool {
        ctx.head().headers().contains_key(http::header::CONTENT_TYPE)
    }
}

async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let path1 = path.clone();
    HttpResponse::Ok().body(format!("path1: {:?}.{:?} {:?}", path1.0, path1.1, path.into_inner()))
}

async fn index1(req: HttpRequest) -> impl Responder {
    let foo = req.match_info().get("foo").unwrap();
    let bar = req.match_info().get("bar").unwrap();
    HttpResponse::Ok().body(format!("path2: {}, {}", foo, bar))
}

#[get("/path3/{v1}/{v2}/")]
async fn index2(req: HttpRequest) -> impl Responder {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (_v1,_v2): (u8, u8) = req.match_info().load().unwrap();
    HttpResponse::Ok().body(format!("{:?}", (v1, v2, _v1, _v2)))
}

#[get("/path4/{name}/{age}")]
async fn index3(info: web::Path<Info>) -> Result<String> {
    Ok(format!("{:?}", info))
}

async fn index4(req: HttpRequest, path: web::Path<Info>) -> Result<String> {
    let tp: (String, u32) = req.match_info().load().unwrap();
    Ok(format!("/heihei/{}/{} {:?}", path.name, path.age, tp))

}

#[get("/heihei1")]
async fn index5(req: HttpRequest, info: web::Query<Info>) -> Result<HttpResponse> {
    let url = req.url_for("name_age", &[&info.name, info.age.to_string().as_str()])?;
    Ok(HttpResponse::Found()
        .insert_header((http::header::LOCATION, url.as_str()))
        .finish()
    )
}

#[get("/path5")]
async fn index6(req: HttpRequest) -> impl Responder {
    let url = req.url_for("baidu_search", ["keywords"]).unwrap();
    format!("{}", url)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .wrap(middleware::NormalizePath::default())
            .route("/path1/{path1:\\d+}.{ext}", web::route().guard(guard::Any(guard::Post()).or(guard::Get())).to(index))
            .route("/path2/{foo}/{bar:.*}", web::post().to(index1))
            .service(index2)
            .service(index3)
            .service(
                web::resource("/haha")
                    .name("user_detail")
                    .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(HttpResponse::Ok))
                    .route(web::put().to(HttpResponse::Ok))
                )
            .service(
                web::scope("hoho").service(
                    web::resource("/hoho")
                        .route(
                            web::route()
                                .guard(guard::All(guard::Get()).and(ContentTypeHeader))
                                .to(HttpResponse::Ok)
                        )
                        .route(
                            web::route()
                                .method(http::Method::HEAD)
                                .guard(guard::Header("Content-Type", "application/x-www-form-urlencoded"))
                                .to(HttpResponse::Ok)
                        )
                )
                .service(index5)
            )
            .service(
                web::resource("heihei/{name}/{age}")
                    .name("name_age")
                    .route(web::get().to(index4))
            )
            .service(index5)
            .external_resource("baidu_search", "https://www.baidu.com?s={kw}")
            .service(index6)
            .route("/", web::route().guard(guard::Not(guard::Get())).to(HttpResponse::MethodNotAllowed))
            .default_service(web::route().method(http::Method::GET).to(HttpResponse::Forbidden))
            // .default_service(web::route().guard(guard::Not(guard::Get())).to(HttpResponse::MethodNotAllowed))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
