use actix_web::{
    web, http::{self, header::ContentEncoding, StatusCode},
    // get,
    App, Error,
    HttpServer, 
    // Responder,
    HttpRequest, HttpResponse,
};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::task::Poll;
use futures::stream;

#[derive(Serialize, Deserialize)]
struct AppStatus {
    count: Mutex<i32>,
}

// #[get("/")] // this leads `index` not to a function
async fn index(req: HttpRequest) -> HttpResponse {
    if req.headers().contains_key("Content-Type") {
        HttpResponse::Ok().body("Hello World!")
    } else {
        HttpResponse::BadRequest().body("bad request")
    }
    
}

async fn index1(status: web::Data<AppStatus>) -> HttpResponse {
    let mut counter = status.count.lock().unwrap();
    let rsp = HttpResponse::Ok().json(AppStatus{count: Mutex::new(*counter)});
    *counter += 1;
    rsp
}

async fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;
    let server_events = 
        stream::poll_fn(move|_cx| -> Poll<Option<Result<web::Bytes, Error>>>{
            if counter == 0 {
                return Poll::Ready(None);
            }
            let payload = format!("data: {}\n\n", counter);
            counter -= 1;
            Poll::Ready(Some(Ok(web::Bytes::from(payload))))
        });
    HttpResponse::build(StatusCode::OK)
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(ContentEncoding::Identity)
        .streaming(server_events)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_status = web::Data::new(AppStatus{count: Mutex::new(0)});
    HttpServer::new(move||{
        App::new()
            .app_data(app_status.clone())
            .route("/", web::get().to(index))
            .route("/", web::post().to(index1))
            .route("/sse", web::get().to(sse))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use std::task::Poll;
    use actix_web::{
        http::{self, header::ContentType},
        body::{MessageBody as _, to_bytes}, rt::pin, web, App,
        test,
    };

    #[actix_web::test]
    async fn test_stream() {
        let app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();
        let rsp = test::call_service(&app, req).await;
        assert!(rsp.status().is_success());

        let body = rsp.into_body();
        pin!(body);

        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 4\n\n")
        );

        // or

        // let bytes = test::read_body(rsp).await;
        // assert_eq!(
        //     bytes,
        //     web::Bytes::from_static(b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n")
        // );
    }

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let rsp = index(req).await;
        assert_eq!(rsp.status(), http::StatusCode::OK)
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default()
            .to_http_request();
        let rsp = index(req).await;
        assert_eq!(rsp.status(), http::StatusCode::BAD_REQUEST)
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let rsp = test::call_service(&app, req).await;
        assert!(rsp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_get1() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppStatus{count: Mutex::new(1)}))
                .route("/", web::get().to(index1))
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let rsp: AppStatus = test::call_and_read_body_json(&app, req).await;
        let rsp_count = rsp.count.lock().unwrap();
        assert_eq!(*rsp_count, 1);
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::post().uri("/")
            .insert_header(ContentType::plaintext())
            .to_request();
        let rsp = test::call_service(&app, req).await;
        assert!(rsp.status().is_client_error());
    }
}