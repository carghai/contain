// #![feature(const_for)]
pub mod time_manger;
pub mod encryption;

slint::include_modules!();
fn main() {
    HelloWorld::new().unwrap().run().unwrap();
}
