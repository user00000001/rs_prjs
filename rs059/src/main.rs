use actix::{Actor, StreamHandler};
use actix_web::{
    web,
    App, Error,
    HttpServer, Responder,
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let rsp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", rsp);
    rsp
}

async fn index1(_req: HttpRequest) -> impl Responder {
    "Hello."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout target/key.pem -out target/cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("target/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("target/cert.pem").unwrap();
    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .route("/ws/", web::get().to(index)) // cargo run --bin client -- wss://localhost:8080/ws/
            .service(web::resource("/h2").route(web::get().to(index1))) // curl https://localhost:8080/h2 -v -k
    }).bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
