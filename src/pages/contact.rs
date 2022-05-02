use yew::{html, Component, Context, Html};

pub struct Contact;

impl Component for Contact {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p
                class="text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal"
            >
                {"Here's the contact page!!"}
            </p>
        }
    }
}
