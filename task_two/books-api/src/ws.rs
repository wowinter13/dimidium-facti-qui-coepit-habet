use crate::{
    book::{BookStore, Command, Response, ResponseData},
    error::ApiError,
};
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    books: web::Data<BookStore>,
) -> Result<HttpResponse, Error> {
    ws::start(BookSession { books }, &req, stream)
}

struct BookSession {
    books: web::Data<BookStore>,
}

impl Actor for BookSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BookSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            println!("Received message: {}", text); // Debug
            let response = match serde_json::from_str::<Command>(&text) {
                Ok(cmd) => match cmd {
                    Command::GetBooks => Response::Success {
                        data: ResponseData::Books(self.books.get_books()),
                    },
                    Command::GetBook { id } => match self.books.get_book(&id) {
                        Some(book) => Response::Success {
                            data: ResponseData::Book(book),
                        },
                        None => Response::Error {
                            message: ApiError::BookNotFound.to_string(),
                        },
                    },
                    Command::AddBook { book } => {
                        let id = self.books.add_book(book);
                        Response::Success {
                            data: ResponseData::Id(id),
                        }
                    }
                    Command::UpdateBook { id, book } => {
                        if self.books.update_book(&id, book) {
                            Response::Success {
                                data: ResponseData::None,
                            }
                        } else {
                            Response::Error {
                                message: ApiError::BookNotFound.to_string(),
                            }
                        }
                    }
                    Command::DeleteBook { id } => {
                        if self.books.delete_book(&id) {
                            Response::Success {
                                data: ResponseData::None,
                            }
                        } else {
                            Response::Error {
                                message: ApiError::BookNotFound.to_string(),
                            }
                        }
                    }
                },
                Err(e) => Response::Error {
                    message: ApiError::InvalidCommand(e.to_string()).to_string(),
                },
            };

            if let Ok(response_text) = serde_json::to_string(&response) {
                println!("Sending response: {}", response_text); // Debug
                ctx.text(response_text);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{web, App, HttpServer};
    use futures_util::{SinkExt, StreamExt};
    use serde_json::json;
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    async fn setup_test_server() {
        let books = web::Data::new(BookStore::new());

        let server = HttpServer::new(move || {
            App::new()
                .app_data(books.clone())
                .service(web::resource("/ws").to(ws_index))
        })
        .bind("127.0.0.1:8081")
        .unwrap()
        .run();

        tokio::spawn(server);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    #[actix_rt::test]
    async fn test_book_flow() -> Result<(), Box<dyn std::error::Error>> {
        setup_test_server().await;

        let (mut ws_stream, _) = connect_async("ws://127.0.0.1:8081/ws")
            .await
            .expect("Failed to connect");

        let add_command = json!({
            "action": "add_book",
            "book": {
                "title": "The Rust Programming Language",
                "author": "Steve Klabnik",
                "year": 2023
            }
        });

        ws_stream
            .send(Message::Text(add_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        let book_id = match response {
            Response::Success {
                data: ResponseData::Id(id),
            } => id,
            _ => panic!("Unexpected response: {:?}", response),
        };

        let get_command = json!({
            "action": "get_book",
            "id": book_id
        });

        ws_stream
            .send(Message::Text(get_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        match response {
            Response::Success {
                data: ResponseData::Book(book),
            } => {
                assert_eq!(book.title, "The Rust Programming Language");
                assert_eq!(book.author, "Steve Klabnik");
                assert_eq!(book.year, 2023);
            }
            _ => panic!("Unexpected response: {:?}", response),
        }

        let update_command = json!({
            "action": "update_book",
            "id": book_id,
            "book": {
                "title": "The Rust Programming Language - Second Edition",
                "author": "Steve Klabnik",
                "year": 2024
            }
        });

        ws_stream
            .send(Message::Text(update_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        assert!(matches!(
            response,
            Response::Success {
                data: ResponseData::None
            }
        ));

        ws_stream
            .send(Message::Text(get_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        match response {
            Response::Success {
                data: ResponseData::Book(book),
            } => {
                assert_eq!(book.title, "The Rust Programming Language - Second Edition");
                assert_eq!(book.year, 2024);
            }
            _ => panic!("Unexpected response: {:?}", response),
        }

        let delete_command = json!({
            "action": "delete_book",
            "id": book_id
        });

        ws_stream
            .send(Message::Text(delete_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        assert!(matches!(
            response,
            Response::Success {
                data: ResponseData::None
            }
        ));

        ws_stream
            .send(Message::Text(get_command.to_string()))
            .await?;

        let response = if let Some(Ok(Message::Text(resp))) = ws_stream.next().await {
            println!("Received response: {}", resp);
            serde_json::from_str::<Response>(&resp)?
        } else {
            panic!("Failed to get response");
        };

        assert!(matches!(response, Response::Error { .. }));

        ws_stream.close(None).await?;

        Ok(())
    }
}
