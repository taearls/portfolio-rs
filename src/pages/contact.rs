use lazy_static::lazy_static;

use yew::{function_component, html};

#[cfg(any(feature = "email-service", feature = "contact-form-mailto-link"))]
use crate::components::ContactForm;
use crate::components::{HeadingOne, MailtoLink, Page, Paragraph};

lazy_static! {
    static ref PORTFOLIO_EMAIL: &'static str = "tyler.a.earls@gmail.com";
}

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <Page>
            <HeadingOne>
                {"Contact Tyler Earls"}
            </HeadingOne>
            <Paragraph>
                {"If you're interested in hiring me for coding work, my music, or just want to say hello—I'd love to hear from you. I'm a voracious learner, and nothing is too nerdy or niche for my taste."}
            </Paragraph>
            <Paragraph>
                {"The best way to reach me is via email at "}
                <MailtoLink></MailtoLink>
            </Paragraph>
            // uncomment to re-add contact form
            // <ContactForm></ContactForm>
        </Page>
    }
}
