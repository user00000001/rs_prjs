use actix_web::{get, post, web, guard, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/state")]
async fn index_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

async fn index_mutable_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

async fn me() -> impl Responder {
    "me"
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(||async{HttpResponse::Ok().body("test")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed))   
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(||async{HttpResponse::Ok().body("app")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });
    HttpServer::new(move ||{
        App::new()
            .configure(config)
            .service(hello)
            .service(echo)
            .service(index_state)
            .route("/hey", web::get().to(manual_hello))
            .route("/mutable_state", web::get().to(index_mutable_state))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index)),
            )
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .service(
                web::scope("/users").service(web::resource("/me").to(me))
            )
            .service(
                web::scope("/guard")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(||async {HttpResponse::Ok().body("www")}))   
            )
            .service(
                web::scope("/guard")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(||async {HttpResponse::Ok().body("users")}))   
            )
            .service(web::scope("/api").configure(scoped_config))
            // .configure(config) // /app not available after scope("/app")
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}