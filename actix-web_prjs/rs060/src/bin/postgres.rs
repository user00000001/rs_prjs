mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table="users")]
    pub struct User {
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}

mod db {
    use crate::{errors::MyError, models::User};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
        let _stmt = include_str!("../../db/add_user.sql");
        let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &user_info.email,
                    &user_info.first_name,
                    &user_info.last_name,
                    &user_info.username,
                ],
            )
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
            .ok_or(MyError::NotFound)
    }
}

mod handlers {
    use crate::{db, errors::MyError, models::User};
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    pub async fn add_user(
        user: web::Json<User>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let user_info: User = user.into_inner();
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let new_user = db::add_user(&client, user_info).await?;
        Ok(HttpResponse::Ok().json(new_user))
    }
}

use actix_web::{
    web, middleware,
    HttpServer, HttpResponse,
    App,
};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::add_user;


// docker run --rm -it --name pg_test -e POSTGRES_PASSWORD=mypassword -p 5432:5432 -v `pwd`/db:/db postgres:14.2-alpine3.15

// docker exec -it --user postgres pg_test createuser -P test_user

// docker exec -it --user postgres pg_test createdb -O test_user testing_db

// docker exec -it --user postgres pg_test psql -f /db/schema.sql testing_db

// docker exec -it --user postgres pg_test psql -d testing_db -c 'GRANT ALL ON ALL TABLES IN SCHEMA testing TO test_user; GRANT ALL ON ALL SEQUENCES IN SCHEMA testing TO test_user; GRANT ALL ON ALL FUNCTIONS IN SCHEMA testing TO test_user; grant USAGE on SCHEMA testing to test_user; '

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // docker exec -it --user postgres pg_test psql -d testing_db

    // curl -d '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users

    // sudo apt install httpie -y
    // echo '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' | http -f --json --print h POST http://127.0.0.1:8080/users
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move||{
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(HttpResponse::Ok))
            .service(web::resource("/users").route(web::post().to(add_user)))
    }).bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}