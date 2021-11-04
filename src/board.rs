use crate::square::Square;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Board { link, value: 0 }
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
        let status = "Next player: X";

        html! {
            <div>
                <div class="status">{status}</div>
                <div class="board-row">
                    <Square />
                    <Square />
                    <Square />
                </div>
                <div class="board-row">
                    <Square />
                    <Square />
                    <Square />
                </div>
                <div class="board-row">
                    <Square />
                    <Square />
                    <Square />
                </div>
            </div>
        }
    }
}
