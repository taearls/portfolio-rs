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
                <br />
                {"Please hire me!"}
            </HeadingOne>

            <Paragraph>
                {"I'm currently between full-time positions. I have experience working on frontend applications with React and Vue, and I have backend experience in Node and Java. I have been learning Rust (which this website is written in) independently for 2 years as well."}
            </Paragraph>
            <Paragraph>
                {"I have a passion for learning new things and I'm always looking to learn more. If you're hiring, or if you simply want to say hello, please feel free to email me at "}
                <InlineAnchor href="mailto:tyler.a.earls@gmail.com" aria_label="Send Tyler Earls an email">{"tyler.a.earls@gmail.com"}</InlineAnchor>
                {"."}
            </Paragraph>
            <Paragraph>
                {"Since the start of the Covid-19 pandemic, I have been studying Rust, and now I use it regularly both on side projects (including this website!) and on Exercism's "}
                <InlineAnchor href="https://www.exercism.io" aria_label="Go to Exercism">{"Rust track"}</InlineAnchor>
                {". Ideally my next role will be at a company that's interested in adopting Rust."}
            </Paragraph>
            <Paragraph>
                {"I'm also developing a Rust crate that leverages music theory to generate valid chords and scales from user input. If you'd like, check out the "}
                <InlineAnchor href="https://github.com/taearls/audiate" aria_label="Go to the documentation for Audiate">
                    {"documentation"}
                </InlineAnchor>
                {" for Audiate to keep up-to-date with my progress."}
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
