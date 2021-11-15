mod board;
mod game;
mod info;
mod square;

extern crate wee_alloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() {
    yew::start_app::<game::Game>();
}
