use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let mut resp = String::new();
    let hdmap = req.headers();
    for (k, v) in hdmap {
        let key = k.to_string();
        let val = v.to_str().expect("failed to get header value");
        resp.push_str(&key);
        resp.push_str(": ");
        resp.push_str(val);
        resp.push_str("\n");
    }
    HttpResponse::Ok().body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
