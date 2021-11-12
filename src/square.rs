use yew::prelude::*;

use crate::game;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub value: game::Square,
    pub on_click: Callback<()>,
}

pub struct Square {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for Square {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.props.on_click.emit(()),
        }
        true
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
            <button
                class="square"
                onclick=self.link.callback(|_|Msg::Click)
            >
                {&self.props.value}
            </button>
        }
    }
}
