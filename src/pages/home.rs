use super::super::components::util::InlineAnchor;
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
            <section class="px-4 mx-auto max-w-none w-4/5 leading-8">
                <h1
                    class="text-left mb-4 text-purple-700 dark:text-purple-400 font-extrabold text-4xl"
                    style="line-height: 1.1;"
                >
                    {"Hi there!"}
                    <br />
                    {"My name is Tyler Earls."}
                    <br />
                    {"I am a software engineer and musician."}
                </h1>
                <p
                    class="text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal"
                >
                    {"Currently, I work at "}
                        <InlineAnchor href="https://cquence.app" aria_label="Go to Cquence">
                            {"Cquence"}
                        </InlineAnchor>
                    {", where I help develop video software that empowers film makers and content creators in their post production workflow."}
                </p>
                <p
                    class="text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal"
                >
                    {"Aside from this work, I occasionally take on freelance frontend work. I love building clean and accessible UI components that enrich everyone's web experience."}
                </p>
                <p
                    class="text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal"
                >
                    {"Outside of tech, I write songs and lead a band called "}
                        <InlineAnchor href="https://www.cuckooandthebirds.com/" aria_label="Go the Cuckoo and the Birds website">
                            {"Cuckoo and the Birds"}
                        </InlineAnchor>
                    {". If you'd like, you can listen at our "}
                        <InlineAnchor href="https://cuckooandthebirds.bandcamp.com" aria_label="Go to the Cuckoo and the Birds bandcamp">
                            {"Bandcamp"}
                        </InlineAnchor>
                    {"."}
                </p>
                <p
                    class="text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal"
                >
                    {"I'm also a very avid Star Trek fan. During these difficult times, I find comfort in its optimistic view on the potential of humanity and its future."}
                </p>
            </section>
        }
    }
}
