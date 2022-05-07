use yew::{html, Component, Context, Html};

use crate::components::{Page, Paragraph};

pub struct NotFound;

impl Component for NotFound {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Page>
                <Paragraph>
                    {"Here's the 404 page!!"}
                </Paragraph>
            </Page>
        }
    }
}
