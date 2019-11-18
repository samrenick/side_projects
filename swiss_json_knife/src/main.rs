use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer};

pub fn main() {
    run();
}

fn run() {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://192.168.3.69")
                    // .allowed_origin("http://portal.courierexpress.net")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(
                web::resource(r"/parse_json/filename/{filename}/columns/{columns}")
                    .route(web::get().to(swiss_json_knife::index))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
    })
    .bind("192.168.3.69:8088")
    // .bind("portal.courierexpress.net:8088")
    .unwrap()
    .run()
    .unwrap();
}
