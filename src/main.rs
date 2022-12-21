use standard::game::GameImpl;


mod interfaces;
mod standard;
fn main() {
    let game = GameImpl::new(String::from("test level"), String::from("a test level"));
}
