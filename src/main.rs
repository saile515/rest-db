use actix_web::HttpRequest;
use actix_web::{get, web, App, HttpServer, Responder};
use std::fs;
use std::str;
use actix_web::http::StatusCode;
use querystring::querify;

#[get("/{database}/{document}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let path = req.path().trim_matches('/');
    let query = querify(req.query_string());
    let byte_vec: Result<Vec<u8>, std::io::Error> = fs::read(format!("{}.json", path));
    if let Ok(res) = byte_vec {
        if query.len() > 0 {
        }
        return (format!("{:?}", str::from_utf8(&res).unwrap()), StatusCode::OK);
    } else {
        return ("404: Not Found!".to_string(), StatusCode::NOT_FOUND);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 28256))?
    .run()
    .await
}