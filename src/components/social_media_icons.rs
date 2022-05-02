use yew::{classes, html, Component, Context, Html};

pub struct SocialMediaIcon {
    aria_label: String,
    href: String,
    class: String,
}

pub struct SocialMediaIcons;

impl Component for SocialMediaIcons {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let icons: Vec<SocialMediaIcon> = vec![
            SocialMediaIcon {
                aria_label: "Go to Tyler's LinkedIn".to_string(),
                href: "https://www.linkedin.com/in/tylerearls".to_string(),
                class: "fa-brands fa-linkedin".to_string(),
            },
            SocialMediaIcon {
                aria_label: "Go to Tyler's Github".to_string(),
                href: "https://www.github.com/taearls".to_string(),
                class: "fa-brands fa-github".to_string(),
            },
            SocialMediaIcon {
                aria_label: "Go to Tyler's Twitter".to_string(),
                href: "https://www.twitter.com/taearls".to_string(),
                class: "fa-brands fa-twitter".to_string(),
            },
            SocialMediaIcon {
                aria_label: "Go to Tyler's Bandcamp".to_string(),
                href: "https://cuckooandthebirds.bandcamp.com".to_string(),
                class: "fa-brands fa-bandcamp".to_string(),
            },
        ];
        html! {
            <div class="flex justify-center">
                {
                    icons.into_iter().enumerate().map(|(index, icon)| {
                        html!{
                            <a
                                key={index}
                                href={icon.href.clone()}
                                aria-label={icon.aria_label.clone()}
                                class="px-2 py-1 rounded-sm focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                target="_blank"
                            >
                                <span
                                    class="text-purple-700 dark:text-purple-400 rounded-sm hover:text-cyan-400 dark:hover:text-cyan-300"
                                >
                                    <i
                                        class={classes!("fa-2xl", icon.class.clone())}>
                                    </i>
                                </span>
                            </a>
                        }
                    }).collect::<Html>()
                }
            </div>
        }
    }
}
