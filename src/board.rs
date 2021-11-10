use yew::prelude::*;

use crate::square::Square;

pub struct Board {
    link: ComponentLink<Self>,
    squares: Vec<&'static str>,
    x_is_next: bool,
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
            squares: vec![""; 9],
            x_is_next: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(i) => {
                if self.calculate_winner().is_some() || !self.squares[i].is_empty() {
                    return false;
                }
                self.squares[i] = if self.x_is_next { "X" } else { "O" };
                self.x_is_next = !self.x_is_next;
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
        // let status = format!("Next player: {}", if self.x_is_next { "X" } else { "O" });
        let status;
        match self.calculate_winner() {
            Some(w) => status = format!("Winner: {}", w),
            None => status = format!("Next player: {}", if self.x_is_next { "X" } else { "O" }),
        }

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
                value=self.squares[i]
                on_click=self.link.callback(move |_| Msg::Click(i))
            />
        }
    }

    fn calculate_winner(&self) -> Option<&'static str> {
        let squares = &self.squares;
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for i in 0..lines.len() {
            let [a, b, c] = lines[i];
            if !squares[a].is_empty() && squares[a] == squares[b] && squares[a] == squares[c] {
                return Some(squares[a]);
            }
        }
        None
    }
}
