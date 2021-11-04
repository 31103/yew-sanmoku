use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub value: i64,
}

pub struct Square {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Square {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        html! {
            <button class="square">
                {self.props.value}
            </button>
        }
    }
}
