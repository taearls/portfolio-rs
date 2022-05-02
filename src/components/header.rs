use yew::{html, Component, Context, Html};

use super::Navigation;

pub struct Header;

impl Component for Header {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Navigation />
            </div>
        }
    }
}
