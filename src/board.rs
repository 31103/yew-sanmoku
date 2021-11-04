use crate::square::Square;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    state: Vec<String>,
}

pub enum Msg {}

impl Component for Board {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Board {
            link,
            state: vec!["".into(); 9],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
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

        // fn renderSquare(i: usize) -> Html {
        //     html! {
        //         <Square value=self.state[0].clone() />
        //     }
        // }

        let render_square = |i: usize| {
            html! {
                <Square value=self.state[i].clone() />
            }
        };

        html! {
            <div>
                <div class="status">{status}</div>
                <div class="board-row">
                    {render_square(0)}
                    {render_square(1)}
                    {render_square(2)}
                </div>
                <div class="board-row">
                    {render_square(3)}
                    {render_square(4)}
                    {render_square(5)}
                </div>
                <div class="board-row">
                    {render_square(6)}
                    {render_square(7)}
                    {render_square(8)}
                </div>
            </div>
        }
    }
}
