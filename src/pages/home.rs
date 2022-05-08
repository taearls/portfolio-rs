use yew::{function_component, html};

use crate::components::{HeadingAlignment, HeadingOne, InlineAnchor, Page, Paragraph};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Page>
            <HeadingOne
                align={HeadingAlignment::Left}
                style={"line-height: 1.1;".to_string()}
            >
                {"Hi there!"}
                <br />
                {"My name is Tyler Earls."}
                <br />
                {"I am a software engineer and musician."}
            </HeadingOne>
            <Paragraph>
                {"Currently, I work at "}
                    <InlineAnchor href="https://cquence.app" aria_label="Go to Cquence">
                        {"Cquence"}
                    </InlineAnchor>
                {", where I help develop video software that empowers film makers and content creators in their post production workflow."}
            </Paragraph>
            <Paragraph>
                {"Aside from this work, I occasionally take on freelance frontend work. I love building clean and accessible UI components that enrich everyone's web experience."}
            </Paragraph>
            <Paragraph>
                {"Outside of tech, I write songs and lead a band called "}
                    <InlineAnchor href="https://www.cuckooandthebirds.com/" aria_label="Go the Cuckoo and the Birds website">
                        {"Cuckoo and the Birds"}
                    </InlineAnchor>
                {". If you'd like, you can listen at our "}
                    <InlineAnchor href="https://cuckooandthebirds.bandcamp.com" aria_label="Go to the Cuckoo and the Birds bandcamp">
                        {"Bandcamp"}
                    </InlineAnchor>
                {"."}
            </Paragraph>
            <Paragraph>
                {"I'm also a very avid Star Trek fan. During these difficult times, I find comfort in its optimistic view on the potential of humanity and its future."}
            </Paragraph>
        </Page>
    }
}
