pub mod battlefield;
pub mod armor;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}