use yew::{html, Component, Context, Html};

pub struct NotFound;

impl Component for NotFound {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Here's the 404 page!!"}</h1>
        }
    }
}
