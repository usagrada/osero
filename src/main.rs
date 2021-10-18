use actix::ActorContext;
use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use once_cell::sync::Lazy;
use serde_json::Value;
mod osero;
static mut FIELD: Lazy<osero::Fields> = Lazy::new(|| osero::Fields::new());

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // println!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                println!("ping message");
                ctx.pong(&msg)
            }
            ws::Message::Text(text) => {
                println!("text message: {}", text);
                let v: Value = serde_json::from_str(&text.clone()).unwrap();
                if v["type"] == "put black" {
                    // global 変数を書き換えるには unsafe をつけないといけない
                    println!("{} {:?}", v["type"], v["data"]);
                    println!(
                        "{} {} {}",
                        v["type"], v["data"]["color"], v["data"]["index"]
                    );
                    unsafe {
                        let result = FIELD.calc(v);
                        ctx.text(&result);
                    }
                }
                ctx.text(text)
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(reason) => {
                println!("socket close");
                ctx.close(reason);
                // ctx.stop();
            }
            ws::Message::Pong(msg) => {
                println!("pong message");
                ctx.pong(&msg)
            }
            _ => ctx.text("not specify"),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    // println!("resp\n{:?}", resp);
    println!("access");
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let _cors = Cors::default()
            // .allowed_origin("https://www.rust-lang.org/")
            // .allowed_origin("http://localhost:3001")
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            // .wrap(cors)
            .route("/ws/", web::get().to(index))
            .service(fs::Files::new("/", "./static").show_files_listing())
        // .route("/socket.io/", web::get().to(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
