use yew::{html, Component, Context, Html};

pub struct Home;

impl Component for Home {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Here's the home page!!"}</h1>
        }
    }
}
