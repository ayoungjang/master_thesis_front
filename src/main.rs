use actix_web::{dev::ResourcePath, web, App, HttpResponse, HttpServer, Responder};
use askama::{Template};

#[derive(Template)]
#[template(path = "base.html")]
struct LayoutTemplate {
    title: String,
    content:String
}

async fn home() -> impl Responder{
    let home_content = include_str!("../templates/home.html").to_string();
    let rendered = LayoutTemplate {
        title: "Home".to_string(),
        content: home_content
    }
    .render()
    .unwrap();  // Render the template with the content.

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn login() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/login.html"))
        
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Ensure that the App instance is the last expression in the closure
        // which returns the App itself, not `()`.
        App::new()
            .route("/", web::get().to(login))
            .route("/home", web::get().to(home))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
