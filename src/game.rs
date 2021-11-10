use yew::prelude::*;

use crate::board::Board;

pub struct Game {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Game {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        // }
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
