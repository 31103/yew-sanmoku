use board::Board;
use yew::prelude::*;

mod board;
mod square;

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="game">
                    <div class="game-board">
                        <Board />
                    </div>
                </div>
                <div class="game-info">
                    <div>/* status*/</div>
                    <ol>/* TODO */</ol>
                </div>
            </>
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}
