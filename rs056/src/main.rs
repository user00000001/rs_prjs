use actix_web::{
    web, guard, error, middleware, get, post, 
    Result, Error, http::header::ContentType,
    App, HttpServer, http::header::ContentEncoding,
    HttpResponse, 
    Responder,
};
use std::io::Write;
use actix_multipart::Multipart;
use futures::StreamExt;
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

static HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
    0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00,
    0x00, 0x00,
]; // echo -n 'hello world'|gzip -9 |hexdump -C

#[post("/payload")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().insert_header(ContentEncoding::Identity).json(obj)) // tell client this route has disabled Accept-Encoding
}

#[get("/payload/{name}")]
async fn index_manual1(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
        number: 0,
    };
    Ok(web::Json(obj))
}

async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {} json!", info.username))
}

async fn index1(info: web::Form<Info>) -> impl Responder {
    format!("Webcome {} form!", info.username)
}

async fn index2() -> HttpResponse {
    let html = r#"<html>
    <head><title>Upload Test</title></head>
    <body>
        <form target="/upload" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <button type="submit">Submit</button>
        </form>
    </body>
</html>"#;
    HttpResponse::Ok().body(html)
}

async fn index3() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}

// curl -s localhost:8080/gzip -H "Accept-Encoding: gzip" |gzip -d
async fn index4() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentEncoding::Gzip)
        .body(HELLO_WORLD)
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename()
            .map_or_else(||Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./target/tmp/{}", filename);
        let mut f = web::block(||std::fs::File::create(filepath)).await??;
        while let Some(chunk) = field.try_next().await? {
            f = web::block(move||f.write_all(&chunk).map(|_|f)).await??;
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::fs::create_dir_all("./target/tmp/")?;
    HttpServer::new(||{
        App::new().wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default()) // -H "Accept-Encoding: gzip, deflate, br"
            // .wrap(middleware::Compress::new(ContentEncoding::Gzip)) // curl -s ... -H "Accept-Encoding: gzip" |gzip -d
            .route("/", web::route().guard(guard::All(guard::Post()).and(guard::Header("Content-Type", "application/json"))).to(index)) // curl -v localhost:8080/ -H "Content-Type: application/json" -d '{"username":"myname"}'
            .route("/", web::route().guard(guard::All(guard::Post()).and(guard::Header("Content-Type", "application/x-www-form-urlencoded"))).to(index1)) // curl -v localhost:8080/ -H "Content-Type: application/x-www-form-urlencoded" -d 'username=myname'
            .service(index_manual) // curl -v localhost:8080/payload -H "Content-Type: application/x-www-form-urlencoded" -d '{"name":"myname", "number": 100}'; curl -v localhost:8080/payload -H "Content-Type: application/json" -d '{"name":"myname", "number": 100}'; curl -v localhost:8080/payload -d '{"name":"myname", "number": 100}'
            .service(
                web::resource("/upload")
                    .route(web::get().to(index2))
                    .route(web::post().to(save_file))
            )
            .route("/plaintext", web::get().to(index3))
            .route("/gzip", web::get().to(index4))
            .service(index_manual1)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
