use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};

use crate::utils::macros::log_info;

// pub fn start(){

// }

pub async fn start(port: u16) -> std::io::Result<()> {
    log_info!("Starting server on http://0.0.0.0:{} ", port);
    HttpServer::new(|| App::new().route("/{path:.*}", web::get().to(serve_file)))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}

async fn serve_file(req: HttpRequest) -> HttpResponse {
    let path = req.match_info().get("path").unwrap_or("index.html");
    log_info!("Received request for path: /{}", path);

    let mut response = HttpResponse::Ok();
    response
        .insert_header(("Content-Type", "text/html; charset=utf-8"))
        .body(format!(
            "<html><body><h1>Hello, World! at {}</h1></body></html>",
            path
        ))
    // match cache.get(path) {
    //     Some(file) => {
    //         let mut response = HttpResponse::Ok();

    //         if file.is_gzipped {
    //             response.insert_header(("Content-Encoding", "gzip"));
    //         }

    //         response
    //             .insert_header(("Content-Type", file.mime_type))
    //             .body(file.bytes.clone())
    //     }
    //     None => HttpResponse::NotFound().body("404 Not Found"),
    // }
}
