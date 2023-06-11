pub mod battlefield;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}