use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub value: String,
}

pub struct Square {
    props: Props,
    link: ComponentLink<Self>,
    value: String,
}

pub enum Msg {
    Click,
}

impl Component for Square {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            value: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.value = "X".into(),
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
        html! {
            <button class="square" onclick=self.link.callback(|_|Msg::Click)>
                {&self.value}
            </button>
        }
    }
}
