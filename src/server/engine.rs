use crate::server::routes::{RouteExportKind, RouteManifest, Routes};
use crate::utils::macros::{log_info, log_verbose};
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, get, web};

#[derive(Clone, Debug)]
struct AppState {
    routes: Routes,
}
pub async fn start(port: u16, routes: Routes) -> std::io::Result<()> {
    log_info!("Starting server on http://0.0.0.0:{} ", port);
    let state = web::Data::new(AppState { routes });

    HttpServer::new(move || App::new().app_data(state.clone()).service(serve))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
#[get("/{path:.*}")]
async fn serve(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let path = req.match_info().get("path").unwrap_or("index.html");
    log_info!("Requested: {}", path);
    let f = serve_file(path, &data.routes).await;
    match f {
        Some((contents,gzip)) => {
            log_info!("{}", contents.content_type());

            let mut res = contents.into_response(&req);
            if gzip{
                res.headers_mut().insert(
                    header::CONTENT_ENCODING,
                    header::HeaderValue::from_static("gzip"),
                );
            }

            res
        }
        None => {
            log_verbose!("File read responded with none.");
            not_found()
        }
    }
}

async fn serve_file(path: &str, routes: &Routes) -> Option<(NamedFile,bool)> {
    let route: Option<&RouteManifest>;
    if path == "/" || path == "" {
        route = routes.get("index.html");
    } else {
        route = routes.get(path);
    }
    match route {
        Some(route) => {
            let file = route.file.read();
            match file {
                Some(file) => {
                    log_verbose!("File read responded with file.");
                    Some((file,route.gzip))
                }
                None => {
                    log_verbose!("File read responded with none.");
                    None
                }
            }

             },
        None => None,
    }
}
fn internal_error() -> HttpResponse {
    HttpResponse::InternalServerError()
        .body("<html><body><h1>Sorry, an error occurred.</h1></body></html>")
}
fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("<html><body><h1>Sorry, 404.</h1></body></html>")
}
