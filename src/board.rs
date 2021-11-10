use crate::square::Square;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    state: Vec<&'static str>,
}

pub enum Msg {
    Click(usize),
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Board {
            link,
            state: vec![""; 9],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(i) => {
                self.state[i]="X";
                true
            }
        }
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
                <div class="status">{ status }</div>
                <div class="board-row">
                    {self.render_square(0)}
                    {self.render_square(1)}
                    {self.render_square(2)}
                </div>
                <div class="board-row">
                    {self.render_square(3)}
                    {self.render_square(4)}
                    {self.render_square(5)}
                </div>
                <div class="board-row">
                    {self.render_square(6)}
                    {self.render_square(7)}
                    {self.render_square(8)}
                </div>
            </div>
        }
    }
}

impl Board {
    fn render_square(&self, i: usize) -> Html {
        html! {
            <Square
                value=self.state[i]
                onClick=self.link.callback(move |_| Msg::Click(i))
            />
        }
    }
}
