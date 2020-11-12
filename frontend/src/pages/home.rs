use yew::prelude::*;

use crate::components::{Navbar, Button};

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <Navbar />
                <div>
                    <span>{ "Home" }</span>
                    <Button>{ "Test" }</Button>
                </div>
            </div>
        }
    }
}
