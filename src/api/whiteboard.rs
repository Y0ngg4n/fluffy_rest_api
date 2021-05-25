use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post};
use serde::{Deserialize, Serialize};

// use constants;

#[derive(Debug, Deserialize, Serialize)]
pub struct Whiteboard {
    pub id: String,
}

impl Whiteboard {
    pub fn new(message: String) -> Self {
        Self {
            id: message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WhiteboardRequest {
    pub message: Option<String>,
}

impl WhiteboardRequest {
    pub fn to_whiteboard(&self) -> Option<Whiteboard> {
        match &self.message {
            Some(message) => Some(Whiteboard::new(message.to_string())),
            None => None,
        }
    }
}

/// create a whiteboard `/whiteboard/create`
#[post("/whiteboard/create")]
pub async fn create(whiteboard_req: Json<WhiteboardRequest>) -> HttpResponse {
    HttpResponse::Ok()
        // .content_type(constants::APPLICATION_JSON)
        .json(Whiteboard{id: String::from("WhiteboardTest") })
}
