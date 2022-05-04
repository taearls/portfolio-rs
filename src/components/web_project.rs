use serde::{Deserialize, Serialize};
use yew::{html, Component, Context, Html, Properties};

use crate::components::CloudinaryImage;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct WebProjectAnalytics {
    pub campaign: String,
    pub medium: String,
    pub source: String,
}

#[derive(Properties, PartialEq)]
pub struct WebProjectProps {
    #[prop_or_default]
    pub analytics: Option<WebProjectAnalytics>,
    pub cloudinary_id: String,
    pub image_extension: String,
    pub alt: String,
    #[prop_or("pointer".to_string())]
    pub cursor_style: String,
    #[prop_or_default]
    pub descriptions: Vec<String>,
    pub emoji: String,
    pub href: String,
    pub name: String,
    pub tagline: String,
    #[prop_or_default]
    pub is_last: bool,
}

pub struct WebProject;

impl Component for WebProject {
    type Properties = WebProjectProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="mx-auto mt-12">
                <h2
                    class="text-center font-bold text-3xl mb-8 text-purple-700 dark:text-purple-400"
                >
                    { ctx.props().name.clone() }
                </h2>
                <div class="flow-root mb-8">
                    <div class="mx-auto text-center w-11/12 mb-2 sm:mb-0 sm:w-1/2 sm:float-left sm:clearfix sm:mr-4">
                        <div class="flex justify-center">
                            <a
                                class="block rounded-sm focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                                // :rel="project.analytics ? 'noopener' : 'noreferrer'"
                                target="_blank"
                                href={ctx.props().href.clone()}
                                // :href="project.analytics !== null ? getAnalyticsLink(project.href, project.analytics) : project.href"
                                style={format!("cursor: {}", ctx.props().cursor_style)}
                            >
                                <CloudinaryImage alt={ctx.props().alt.clone()} public_id={ctx.props().cloudinary_id.clone()} transformations={vec!["q_auto".to_string(), "w_400".to_string()]} extension={ctx.props().image_extension.clone()} />
                            </a>
                        </div>
                        <a
                            class="block cursor-pointer font-semibold rounded-sm mt-1 text-purple-700 dark:text-purple-400 focus:outline-none focus:shadow-outline-light dark:focus:shadow-outline-dark"
                            target="_blank"
                            rel="noreferrer"
                            href={ctx.props().href.clone()}
                        >
                            <span class="text-purple-700 md:text-lg dark:text-purple-400">{ ctx.props().tagline.clone() }</span> {" "} { ctx.props().emoji.clone() }
                        </a>
                    </div>
                    <div>
                        {
                            ctx.props().descriptions.clone().into_iter().enumerate().map(|(index, description)| {
                                html! {
                                    <p
                                        key={index}
                                        class="my-4 px-2 sm:px-0 text-soft-black dark:text-white text-justify text-lg md:text-xl leading-7"
                                    >
                                        {description}
                                    </p>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
                if !ctx.props().is_last {
                    <hr class="line-break" />
                }
            </div>
        }
    }
}
