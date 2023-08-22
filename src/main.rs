use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpServer, Responder};
use actix_web_validator::Json;
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
struct Pessoa {
    pub id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub nome: String,
    #[validate(length(min = 1, max = 32))]
    pub apelido: String,
    pub nascimento: NaiveDate,
    pub stack: Option<Vec<String>>,
}

#[post("/pessoas")]
async fn pessoas(json: Json<Pessoa>) -> impl Responder {
    web::Json(Pessoa {
        id: Some(Uuid::new_v4()),
        nome: json.nome.to_owned(),
        apelido: json.apelido.to_owned(),
        nascimento: json.nascimento.to_owned(),
        stack: json.stack.to_owned(),
    })
}

// #[get("/pessoas/[:id]")]
// #[get("/pessoas?t=[:termo da busca]")]
// #[get("/contagem-pessoa")]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(create_app_config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn create_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(pessoas);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        dev::Service,
        http::{
            header::{self, ContentType},
            StatusCode,
        },
        test, App,
    };

    #[actix_web::test]
    async fn test_post_pessoas_ok() {
        let app = test::init_service(App::new().configure(create_app_config)).await;
        let req = test::TestRequest::post()
            .uri("/pessoas")
            .insert_header(ContentType::json())
            .set_json(Pessoa {
                id: None,
                nome: "Ruan".to_owned(),
                apelido: "leitaoprogramador".to_owned(),
                nascimento: NaiveDate::from_ymd_opt(2000, 1, 10).unwrap(),
                stack: Some(vec!["Rust".to_owned(), "Java".to_owned()]),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        println!("{:?}", resp);
        // let resp = super::pessoas(req).await;
        // assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
// tests
// curl -XPOST localhost:8080/pessoas -d '{"nome": "ruan", "apelido": "rugs", "nascimento":"1994-04-26", "stack": []}' -H 'Content-Type: application/json'
// curl -XPOST localhost:8080/pessoas -d '{"nome": "ruan", "apelido": "rugs", "nascimento":"1994-04-26", "stack": null}' -H 'Content-Type: application/json'
// curl -XPOST localhost:8080/pessoas -d '{"nome": "ruan", "apelido": "rugs", "nascimento":"1994-04-26"}' -H 'Content-Type: application/json'
