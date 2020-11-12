use yew::prelude::*;

pub struct Navbar {}

/*
pub struct Navbar {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props_or_default]
    pub children: Children,

    #[props_or_default]
    pub class: Classes,
}
*/

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Navbar {}
    }

    /*
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Navbar { props }
    }
    */

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // { self.props.children.render() }

        html! {
            <nav>
                <div class="nav-wrapper">
                    <a href="#" class="brand-logo">{ "Logo" }</a>
                    <ul id="nav-mobile" class="right hide-on-med-and-down">
                        <li><a href="#">{ "Sass " }</a></li>
                        <li><a href="#">{ "Component" }</a></li>
                        <li><a href="#">{ "Rust" }</a></li>
                    </ul>
                </div>
            </nav>
        }
    }
}
