use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};

pub mod battlefield;
pub mod armor;
pub mod weapons;

#[derive(Debug)]
pub struct ApiResponse {
    pub json: String,
    pub status: Status,
}

impl<'r> Responder<'r,'static> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}


#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}