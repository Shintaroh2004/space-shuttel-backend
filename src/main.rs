use actix_files as fs;
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use actix_files::NamedFile;
use actix_web::{get,Responder};

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("assets/index.html").await
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(fs::Files::new("/", "assets/").show_files_listing());
    };
    Ok(config.into())
}
