use yew::{html, Component, Context, Html};

pub struct Web;

impl Component for Web {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Here's the web page!!"}</h1>
        }
    }
}
