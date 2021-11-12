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
        let mut rows = vec![];

        for i in 0..3 {
            let mut squares = vec![];

            // １行にマスを３個つめる
            for j in 0..3 {
                squares.push(html! {
                    { self.render_square(j + i * 3) }
                })
            }

            // ３行用意して9*9のマスを作る
            rows.push(html! {
                <div class="board-row">
                    {for squares}
                </div>
            })
        }

        html! {
            <div>
                {for rows}
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
