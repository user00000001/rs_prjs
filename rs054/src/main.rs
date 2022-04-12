use actix_web::{
    HttpServer, middleware::Logger,
    App, get, error, 
    Result, HttpResponse,
    http::{header::ContentType, StatusCode},
};
use derive_more::{Display, Error};
use log::info;

#[derive(Debug, Display, Error)]
enum UserError1 {
    #[display(fmt = "An internal error occurred, Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError1 {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError1::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/error4")]
async fn index4() -> Result<&'static str, UserError1> {
    generate_error().map_err(|_e| UserError1::InternalError)?;
    Ok("success!")
}

fn generate_error() ->Result<(), UserError1> {
    // Err(UserError1::InternalError)
    Ok(())
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/error3")]
async fn index3() -> Result<&'static str, UserError> {
    Err(UserError::ValidationError { field: "My User Error".to_owned() })
}

#[derive(Debug, Display, Error)]
#[allow(unused)]
enum MyError1 {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError1 {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn  status_code(&self) -> StatusCode {
        match *self {
            MyError1::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError1::BadClientData => StatusCode::BAD_REQUEST,
            MyError1::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/error1")]
async fn index1() -> Result<&'static str, MyError1> {
    Err(MyError1::BadClientData)
}

#[get("/error2")]
async fn index2() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test error" });
    Ok(result.map_err(|e|error::ErrorBadRequest(e.name))?)
}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    let err = MyError{name: "test"};
    info!("{}", err);
    Err(err)
}

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(||{
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(index)
            .service(index1)
            .service(index2)
            .service(index3)
            .service(index4)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
