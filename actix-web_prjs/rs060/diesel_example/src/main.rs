#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use actix_web::{
    web, middleware, get, post,
    HttpServer, HttpResponse,
    App, Error,
};
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    let user = web::block(move||{
        let conn = pool.get()?;
        actions::find_user_by_uid(user_id, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_id));
        Ok(res)
    }
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move||{
        let conn = pool.get()?;
        actions::insert_new_user(&form.name, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

// cd .. && echo "DATABASE_URL=target/test.db" >> ../.env
// cargo install diesel_cli --no-default-features --features sqlite
// diesel migration --database-url ../target/test.db --migration-dir migrations run

// curl -s localhost:8080/user -H "Content-Type: application/json" -d '{"name": "myname"}'
// http POST localhost:8080/user name=myname
// curl -s localhost:8080/user/${id}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(HttpResponse::Ok))
            .service(get_user)
            .service(add_user)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn user_routes() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<SqliteConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(get_user)
                .service(add_user),
        )
        .await;

        // Insert a user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&models::NewUser {
                name: "Test user".to_owned(),
            })
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", resp.id))
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(resp.id)))
            .execute(&pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }
}
