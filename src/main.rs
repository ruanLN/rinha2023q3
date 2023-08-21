use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct Pessoa {
    pub id: Option<String>,
    pub nome: String,
    pub apelido: String,
    pub nascimento: String,
    pub stack: Vec<String>,
}

#[post("/pessoas")]
async fn pessoas(json: web::Json<Pessoa>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/pessoas/[:id]")]
// #[get("/pessoas?t=[:termo da busca]")]
// #[get("/contagem-pessoa")]



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(pessoas)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
