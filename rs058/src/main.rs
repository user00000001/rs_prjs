use actix_web::{
    dev::Service as _, middleware::{Logger, DefaultHeaders},
    web, get, HttpResponse, Error, dev, 
    http::{header, header::{ContentDisposition, DispositionType}, StatusCode},
    App, HttpServer, HttpRequest, Responder, Result,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
};
use futures_util::future::FutureExt;
use env_logger::Env;
use actix_session::{Session, CookieSession};
use actix_files::NamedFile;
use std::path::PathBuf;

mod lib;
// use lib::SayHi;
use crate::lib::SayHi;

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

async fn index1(session: Session) -> Result<HttpResponse, Error> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }
    Ok(HttpResponse::Ok().body(format!{
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    }))
}

fn add_error_handler<B>(mut rsp: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    rsp.response_mut().headers_mut().insert(
        header::CONTENT_TYPE, header::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(rsp.map_into_left_body()))
}

async fn index2(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn index3(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = actix_files::NamedFile::open(path)?;
    Ok(
        file.use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![]
        })
    )

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(||{
        App::new()
            .wrap_fn(|req, srv|{
                println!("Hi from start1. You requested: {}", req.path());
                srv.call(req).map(|rsp|{
                    println!("Hi from response1");
                    rsp
                })
            })
            .wrap(SayHi)
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(CookieSession::signed(&[0;32]).secure(false))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_handler)
            )   
            .service(index)
            .route("/session", web::get().to(index1)) // browser explorer to /session
            .route("/index.html", web::get().to(||async { "Hello, middleware!" }))
            .service(web::resource("/error").route(web::get().to(HttpResponse::InternalServerError))) // curl -s localhost:8080/error -v
            .route("/nf/{filename:.*}", web::get().to(index2)) // browser explorer to /nf/src/main.rs
            .route("/nf_with_options/{filename:.*}", web::get().to(index3)) // browser explorer to /nf_with_options/src/main.rs
            .service(actix_files::Files::new("/static", ".").show_files_listing()) // browser explorer to /static
            .service(
                actix_files::Files::new("/static_with_options", ".")
                .show_files_listing().use_last_modified(true)
            ) // browser explorer to /static_with_options
            .service(actix_files::Files::new("/static1", ".").index_file("src/lib.rs")) // browser explorer to /static1
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
