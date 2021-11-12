mod board;
mod game;
mod info;
mod square;

pub fn main() {
    yew::start_app::<game::Game>();
}
