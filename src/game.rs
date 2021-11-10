use yew::prelude::*;

use crate::board::Board;

pub struct Game {
    link: ComponentLink<Self>,
    history: Vec<Vec<&'static str>>,
    x_is_next: bool,
}

pub enum Msg {
    Click(usize),
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            history: vec![vec![""; 9]],
            x_is_next: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(i) => {
                let history = &self.history;
                let mut current = history.iter().last().unwrap().clone();
                if calculate_winner(&current).is_some() || !current[i].is_empty() {
                    return false;
                }
                current[i] = if self.x_is_next { "X" } else { "O" };
                self.history.push(current);
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
        let history = &self.history;
        let current = history.iter().last().unwrap();
        let status;
        match calculate_winner(current) {
            Some(w) => status = format!("Winner: {}", w),
            None => status = format!("Next player: {}", if self.x_is_next { "X" } else { "O" }),
        }

        html! {
            <div class="game">
                <div class="game-board">
                    <Board squares=current.clone() on_click=self.link.callback(move |i:usize|Msg::Click(i)) />
                </div>
                <div class="game-info">
                    <div>{ status }</div>
                    <ol>/* TODO */</ol>
                </div>
            </div>
        }
    }
}

fn calculate_winner(squares: &Vec<&'static str>) -> Option<&'static str> {
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
