use actix_web::{
    dev::Server,
    App,
    web, 
    HttpServer,
    middleware::Logger,
};
use tokio;
use std::net::TcpListener;
use actix_files;
use env_logger::Env;

mod api {
    use actix_web::{
        Responder,
        HttpResponse,
    };

    pub async fn ping() -> impl Responder {
        HttpResponse::Ok()
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


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(|| {
        return App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                .route("", web::get().to(api::ping))
                )
            .service(
                web::scope("") 
                .service(actix_files::Files::new("/static", "./static").show_files_listing())
                .route("/", web::get().to(templates::index))
                );

    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind server port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    address
}

