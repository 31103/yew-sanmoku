use crate::game;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub history: Vec<game::History>,
    pub step_number: usize,
    pub x_is_next: bool,
    pub jump: Callback<usize>,
}

pub struct Info {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Jump(usize),
}

impl Component for Info {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Jump(i) => {
                self.props.jump.emit(i);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let history = &self.props.history;
        let current = history[self.props.step_number].clone();
        let status;
        match game::calculate_winner(&current.squares) {
            Some(w) => status = format!("Winner: {}", w),
            None => {
                status = format!(
                    "Next player: {}",
                    if self.props.x_is_next { "X" } else { "O" }
                )
            }
        }

        html! {
            <div class="game-info">
                <div>{ status }</div>
                <ol>{ for self.render_history() }</ol>
            </div>
        }
    }
}

impl Info {
    fn render_history(&self) -> Vec<Html> {
        self.props.history
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let desc = if i != 0 {
                    let plyaer = if i % 2 != 0 { "X" } else { "O" };
                    format!(
                        "Go to move #{} (player: {} col: {} row: {})",
                        i,
                        plyaer,
                        self.props.history[i].point.col + 1,
                        self.props.history[i].point.row + 1
                    )
                } else {
                    "Go to game start".into()
                };

                // 選択されている盤面であれば、強調表示にする
                let cls: String;
                if self.props.step_number == i {
                    cls = "selected".into()
                } else {
                    cls = "".into()
                }

                html! {
                    <li>
                        <button class=cls onclick=self.link.callback(move |_|Msg::Jump(i))>{ desc }</button>
                    </li>
                }
            })
            .collect()
    }
}
