mod mongo_db;

use crate::mongo_db::User;

use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use actix_web::{
    web, middleware,
    post, get,
    HttpServer, HttpResponse,
    App,
};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{"username": 1})
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should successed");
}

#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    println!("{:?}", form);
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc!{"username": &username}, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {}", username))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // docker run --rm -it -p 27017:27017 mongo:5.0.7
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move||{
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(HttpResponse::Ok))
            .service(add_user)
            .service(get_user)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}