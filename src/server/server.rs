use actix_web::{
    App,
    web, 
    HttpServer,
    middleware::Logger,
};
use actix_files;
use env_logger::Env;

mod api {
    use serde::{Serialize, Deserialize};
    use actix_web::{
        Responder,
        HttpResponse,
    };

    #[derive(Serialize, Deserialize)]
    struct ApiPing {
        status: String,
    }


    pub async fn ping() -> impl Responder {
        return HttpResponse::Ok().json(ApiPing{
            status: String::from("success"),
        });
    }
}

mod templates {
    use actix_web::Responder;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct Index {}

    pub async fn index() -> impl Responder {
        return Index{}; 
    }
}


#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        return App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                .route("", web::get().to(api::ping))
                )
            .service(
                web::scope("") 
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .route("/", web::get().to(templates::index))
                );
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
