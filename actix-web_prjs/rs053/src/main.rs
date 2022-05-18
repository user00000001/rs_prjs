use actix_web::{error, web, get, post, put, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, Deserialize)]
struct MyInfo {
    id: u32,
    username: String,
}
#[derive(Deserialize)]
struct Info{
    user_id: u32,
    friend: String,
}

#[post("/users/{user_id}/{friend}")] // curl http://localhost:8080/users/1/aaa -H "Content-Type: application/json" -d '{"id":1000, "username": "bbb"}'
async fn index(path: web::Path<Info>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.user_id, path.friend, json.id, json.username)
}

#[get("/users/{user_id}/{friend}")] // curl http://localhost:8080/users/1/aaa
async fn index1(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("{} {}", user_id, friend))
}

#[put("/user1/{user_id}/{friend}")] // curl -XPUT http://localhost:8080/user1/1/aaa
async fn index2(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();
    Ok(format!("Welcome {}, user_id {}", name, userid))
}

#[derive(Deserialize)]
struct Info1 {
    username: String,
}

#[get("/users")] // curl http://localhost:8080/users?username=aaaa
async fn index3(info: web::Query<Info1>) -> String {
    format!("Welcome {}!", info.username)
}

// curl http://localhost:8080/users -H "Content-Type: application/json" -d '{"username": "aaaaaa"}'
async fn index4(info: web::Json<Info1>) -> String {
    format!("Welcome {}!", info.username)
}

#[post("/user1")] // curl http://localhost:8080/user1 -d 'username=aaaaaa'
async fn index5(info: web::Form<Info1>) -> String {
    format!("Welcome {}!", info.username)
}

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);
    format!("count: {}", data.count.get())
}

#[derive(Clone)]
struct AppState1 {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>
}

async fn show_count1(data: web::Data<AppState1>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get(),
    )
}

async fn add_one1(data: web::Data<AppState1>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);
    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState{
        count: Cell::new(0),
    };
    let data1 = AppState1 {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };
    HttpServer::new( move ||{
        let json_config = web::JsonConfig::default()
            .limit(4096)
            // .limit(30) // curl http://localhost:8080/users -H "Content-Type: application/json" -d '{"username": "111111111111111111"}' -v
            .error_handler(|err, _req|{
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .app_data(web::Data::new(data.clone()))
            .app_data(web::Data::new(data1.clone()))
            .service(index)
            .service(index1)
            .service(index2)
            .service(index3)
            .service(
                web::resource("/users")
                    .app_data(json_config)
                    .route(web::post().to(index4))
            )
            .service(index5)
            .route("/count", web::to(show_count))
            .route("/count/add", web::to(add_one))
            .route("/count1", web::to(show_count1))
            .route("/count1/add", web::to(add_one1))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
