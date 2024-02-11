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
    use actix_web::{Result, HttpResponse, Responder};
    use askama::Template;
    use std::{fs, path::Path};
    use crate::sudoku::Sudoku;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct Index {
        id: u8,
        sudoku: Vec<Vec<Option<u8>>>,
        numbers: Vec<u8>,
    }

    pub async fn index() -> impl Responder {
        match fs::read_to_string(Path::new("output1.txt")) {
            Ok(data) => {
                let (first, _) = data.split_once("=").unwrap_or_default();
                let play = first.lines().nth(1).unwrap_or_default();

                let template = Index {
                    id: 1,
                    sudoku: Sudoku::from(play.trim()).get_grid(),
                    numbers: (1..10).collect()
                }.render();

                match template {
                    Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                    Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
                }
            },
            Err(_) => {
                eprintln!("Err:: could not load the sudoku file data");
                return HttpResponse::InternalServerError().body("Could not load sudoku")
            }
        }
    }
}


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
