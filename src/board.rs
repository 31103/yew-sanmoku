use yew::prelude::*;

use crate::game;
use crate::square::Square;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub squares: Vec<game::Square>,
    pub on_click: Callback<usize>,
}

pub struct Board {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click(usize),
}

impl Component for Board {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(i) => {
                self.props.on_click.emit(i);
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
        html! {
            <div>
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
                value=self.props.squares[i]
                on_click=self.link.callback(move |_| Msg::Click(i))
            />
        }
    }
}
