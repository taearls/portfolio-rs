use yew::{function_component, html};

use crate::components::{Page, Paragraph};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Page>
            <Paragraph>
                {"Here's the 404 page!!"}
            </Paragraph>
        </Page>
    }
}
