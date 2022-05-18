use actix_web::{
    http::KeepAlive, 
    web, get, App, 
    HttpRequest, 
    HttpResponse, 
    HttpServer, 
    Responder, Error,
    body::BoxBody, 
    http::header::ContentType
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Serialize;
use futures::{future::ok, stream::once};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body>{
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
 }

async fn sync_handler() -> impl Responder {
    std::thread::sleep(std::time::Duration::from_secs(5));
    "sync response" // time ab -c 5 -n 20 127.0.0.1:8080/sync
}

async fn async_handler() -> impl Responder {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    "async response" // time ab -c 5 -n 20 127.0.0.1:8080/async
}

#[get("/user")]
async fn index(_req: HttpRequest) -> impl Responder {
    MyObj { name: "user" }
}
// async fn index1(_req: HttpRequest) -> &'static str {
//     "Welcome!"
// }
// async fn index2(_req: HttpRequest) -> String {
//     "Welcome!".to_owned()
// }
// async fn index3(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
//     ...
// }

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout target/key.pem -out target/cert.pem -days 365 -subj '/CN=localhost'
    builder.set_private_key_file("target/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("target/cert.pem").unwrap();
    HttpServer::new(||{
        App::new().route("/", web::get().to(HttpResponse::Ok))
            .route("/sync", web::get().to(sync_handler))
            .route("/async", web::get().to(async_handler))
            .service(stream)
            .service(index)
    })
        .workers(4)
        .bind_openssl("127.0.0.1:8080", builder)? // curl https://localhost:8080/sync -v -k
        // .keep_alive(std::time::Duration::from_secs(75))
        .keep_alive(KeepAlive::Os)
        // .keep_alive(KeepAlive::Disabled)
        // .keep_alive(None)
        .run()
        .await
}