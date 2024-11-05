use actix_web::{web, App, HttpServer};
mod book;
mod error;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let books = web::Data::new(book::BookStore::new());

    println!("Starting server at ws://127.0.0.1:8080/ws");

    HttpServer::new(move || {
        App::new()
            .app_data(books.clone())
            .service(web::resource("/ws").to(ws::ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
