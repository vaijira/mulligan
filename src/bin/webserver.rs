use actix_files as fs;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[get("/resource1/{name}/index.html")]
async fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {req:?}");
    format!("Hello: {name}!\r\n")
}

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!\r\n"
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./web").show_files_listing())
            .service(index)
            .service(no_params)
            .service(
                web::resource("/resource2/index.html")
                    .wrap(middleware::DefaultHeaders::new().add(("X-Version-R2", "0.3")))
                    .default_service(web::route().to(HttpResponse::MethodNotAllowed))
                    .route(web::get().to(index_async)),
            )
            .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await
}
