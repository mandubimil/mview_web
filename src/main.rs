mod routes;
mod comm;

use actix_web::{web, App, HttpServer, middleware};
use actix_session::CookieSession;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_files as fs;
use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();       
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let pool = comm::db_postgres::connect_db().unwrap();
    let service_ip_port = comm::util::get_service_ip_port().unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/view/**/*")).unwrap();

        App::new()
            .data(pool.clone())
            .data(tera)

            //.wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%t %P %r %s"))
            .wrap(CookieSession::signed(&[0; 32]).secure(false))

            .service(web::resource("/view/{html_id}").route(web::get().to(routes::html_view::get_html)))
            .service(fs::Files::new("/static", "static/"))
            .service(web::resource("/common_get/{id}").route(web::get().to(routes::common::get_common)))

            .service(web::resource("/connect").route(web::post().to(routes::common::connect)))
            .service(web::resource("/mdan_10_ar/{id}").route(web::post().to(routes::mdan_10_ar::post_job)))

    })
    .bind_openssl(service_ip_port, builder)?
    .run()
    .await
}


















