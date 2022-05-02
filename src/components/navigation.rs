use yew::{html, Component, Context, Html, Properties};

use super::util::InlineAnchor;

pub struct Navigation;

#[derive(Properties, PartialEq, Clone)]
struct NavigationLink {
    aria_label: String,
    display_text: String,
    href: String,
    #[prop_or_default]
    is_external: bool,
}

impl Component for Navigation {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let links = vec![
            NavigationLink {
                aria_label: "Visit Home Page".to_string(),
                display_text: "Home".to_string(),
                href: "/".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Visit Web Projects Page".to_string(),
                display_text: "Web".to_string(),
                href: "/web-projects".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Visit Contact Page".to_string(),
                display_text: "Contact".to_string(),
                href: "/contact".to_string(),
                is_external: false,
            },
            NavigationLink {
                aria_label: "Listen to Tyler's music on Bandcamp".to_string(),
                display_text: "Music".to_string(),
                href: "https://cuckooandthebirds.bandcamp.com".to_string(),
                is_external: true,
            },
        ];
        html! {
            <div>
                <ul class="flex flex-col h-auto justify-center sm:flex-row sm:justify-end">
                    {
                        links.into_iter().enumerate().map(|(index, link)| {
                            html! {
                                <li
                                    class="flex justify-center sm:inline-block mx-auto py-2 text-center w-1/3 border border-gray-400 dark:border-gray-500 border-t-0 border-l-0 border-r-0 border-b-1 sm:border-none sm:mx-0 w-auto"
                                    key={index}
                                >
                                    if link.is_external {
                                        <InlineAnchor aria_label={link.aria_label} href={link.href} is_external={link.is_external} classes="pl-4 pr-10 m-0">
                                            {link.display_text}
                                        </InlineAnchor>
                                    } else {
                                        <InlineAnchor aria_label={link.aria_label} href={link.href} is_external={link.is_external} classes="px-4 m-0">
                                            {link.display_text}
                                        </InlineAnchor>
                                    }
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        }
    }
}
