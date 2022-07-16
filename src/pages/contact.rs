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
                {"I'm currently between full-time positions and am actively looking for my next role. Specifically, I'm looking for a fully remote, full-time position with opportunities for growth and internal promotion."}
            </Paragraph>
            <Paragraph>
                {"My resume is available upon request. In it I detail my years of fullstack experience: building enterprise-scale frontend applications with React and Vue; maintaining and updating backend applications and AWS microservices in Node.js and Java. I have been learning Rust (which this website is written in) independently for 2 years as well, which I would be thrilled to put into practice full-time."}
            </Paragraph>
            <Paragraph>
                {"If you're interested in hiring me for coding work, my music, or just want to say hello—I'd love to hear from you. I'm a voracious learner, and nothing is too nerdy or niche for my taste."}
            </Paragraph>
            <Paragraph>
                {"The best way to reach me is via email at "}
                <MailtoLink></MailtoLink>
                {"."}
            </Paragraph>
            // uncomment to re-add contact form
            // <ContactForm></ContactForm>
        </Page>
    }
}
