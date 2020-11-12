use yew::prelude::*;

pub struct Button {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <button class="waves-effect waves-light btn">
                { self.props.children.clone() }
            </button>
        }
    }
}
