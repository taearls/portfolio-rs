use yew::{html, Component, Context, Html};

use crate::components::{
    web_project::{WebProject, WebProjectAnalytics, WebProjectProps},
    HeadingOne, InlineAnchor,
};
pub struct Web;

impl Component for Web {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // TODO: find a cleaner way to initialize this data
        let web_projects: Vec<WebProjectProps> = vec![
            WebProjectProps {
                alt: "Image of Cuckoo and the Birds Website".to_string(),
                analytics: Some(WebProjectAnalytics {
                  campaign: "portfolio".to_string(),
                  medium: "web".to_string(),
                  source: "portfolio".to_string(),
                }),
                cloudinary_id: "screenshots/v2/cuckoo-mobile".to_string(),
                // cloudinarySrc: "https://res.cloudinary.com/taearls/image/upload/q_auto/v1/screenshots/v2/cuckoo-mobile.png",
                cursor_style: "pointer".to_string(),
                descriptions: vec![
                  "A website I made for my band, Cuckoo and the Birds, where you can find all our info, social media links, and press.".to_string(),
                  "If you're interested to listen, please consider supporting our music by streaming our EP, Twin Stars, on Spotify.".to_string(),
                  "Built mobile-first with Nuxt, Tailwind, and a (mostly) healthy dose of rock 'n' roll 🤘".to_string(),
                ],
                emoji: "🎵".to_string(),
                href: "https://www.cuckooandthebirds.com".to_string(),
                name: "Cuckoo and the Birds Website".to_string(),
                tagline: "Give our music a listen!".to_string(),
                image_extension: "png".to_string(),
                is_last: false,
              },
              WebProjectProps {
                alt: "Image of Road Ranger Banner".to_string(),
                analytics: None,
                cloudinary_id: "screenshots/v2/roadranger-desktop".to_string(),
                // cloudinarySrc:
                //   "https://res.cloudinary.com/taearls/image/upload/q_auto/v1/screenshots/v2/roadranger-desktop.png",
                descriptions: vec![
                  "A navigation header that I built while freelancing for Trekk.".to_string(),
                  "Both the mobile and desktop versions are based on designs their web designers prepared for the client and had me implement. Built with SCSS, JavaScript, and PHP.".to_string(),
                ],
                cursor_style: "pointer".to_string(),
                href: "https://www.roadrangerusa.com".to_string(),
                emoji: "⛽".to_string(),
                name: "Road Ranger".to_string(),
                tagline: "Check it out!".to_string(),
                image_extension: "jpg".to_string(),
                is_last: false,
              },
              WebProjectProps {
                alt: "Image of Space Clones Title Screen".to_string(),
                analytics: None,
                cloudinary_id: "screenshots/v2/space-clones-game".to_string(),
                // cloudinarySrc:
                //   "https://res.cloudinary.com/taearls/image/upload/q_auto/v1/screenshots/v2/space-clones-game.png",
                cursor_style: "url(images/space-clones-cursor.png), pointer".to_string(),
                descriptions: vec![
                  "An original space shooting video game inspired by Space Invaders, the 1978 arcade classic. Defeat the clone army and then their mothership to advance to the next level.".to_string(),
                  "If you get a high enough score, you can earn extra lives. Play solo, or take turns with a friend. The galaxy is yours to save from the invading clone army! Created using HTML5, CSS3, JavaScript, and jQuery.".to_string(),
                ],
                emoji: "😉".to_string(),
                href: "https://space-clones.netlify.com".to_string(),
                name: "Space Clones".to_string(),
                tagline: "Beat my high score!".to_string(),
                image_extension: "png".to_string(),
                is_last: true,
              },
        ];
        html! {
            <section class="px-4 mx-auto max-w-none w-4/5 leading-8">
                <HeadingOne>
                    {"Web Projects"}
                </HeadingOne>
                <p class="text-soft-black dark:text-white text-lg md:text-xl px-2 mt-4 mb-10 leading-7">
                    {"Here's a sample of some of my coding work.
                    In addition to this website, which has been rebuilt with Rust and Yew, 
                    my work includes the personal and freelance projects listed below. If you're interested to see more, feel free to stalk me on "}
                    <InlineAnchor href="https://github.com/taearls" aria_label="Go to Tyler's Github">
                        {"Github"}
                    </InlineAnchor>
                    {"."}
                </p>
                <p class="text-soft-black dark:text-white text-lg md:text-xl px-2 mt-4 mb-10 leading-7">
                    {"Since the Covid-19 pandemic started, I have been particularly interested in Rust. I'm currently working through the Rust track on Exercism, in addition to maintaining this website. I'm also developing a library to leverage music theory concepts generate chords and scales from user input. If you'd like, check out the "}
                    <InlineAnchor href="https://github.com/taearls/audiate" aria_label="Go to the documentation for Audiate">
                        {"documentation"}
                    </InlineAnchor>
                    {" for Audiate to keep up-to-date with my progress."}
                </p>

                {
                    web_projects.into_iter().enumerate().map(|(index, web_project)| {
                        html! {
                            <WebProject
                                key={index}
                                analytics={web_project.analytics}
                                cloudinary_id={web_project.cloudinary_id}
                                alt={web_project.alt}
                                cursor_style={web_project.cursor_style}
                                descriptions={web_project.descriptions}
                                emoji={web_project.emoji}
                                href={web_project.href}
                                image_extension={web_project.image_extension}
                                name={web_project.name}
                                tagline={web_project.tagline}
                                is_last={web_project.is_last}
                            />
                        }
                    }).collect::<Html>()
                }
            </section>
        }
    }
}
