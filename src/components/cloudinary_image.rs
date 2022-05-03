use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct CloudinaryImageProps {
    #[prop_or_default]
    pub transformations: Option<Vec<String>>,
    pub alt: String,
    #[prop_or("jpg".to_string())]
    pub extension: String,
    pub public_id: String,
}

pub struct CloudinaryImage;

impl Component for CloudinaryImage {
    type Properties = CloudinaryImageProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let public_id = ctx.props().public_id.clone();
        let extension = ctx.props().extension.clone();
        let src = match ctx.props().transformations.clone() {
            Some(transformations) => {
                let transformations: String = transformations.join(",");
                format!("https://res.cloudinary.com/taearls/image/upload/{transformations}/v1/{public_id}.{extension}")
            }
            None => {
                format!("https://res.cloudinary.com/taearls/image/upload/{public_id}.{extension}")
            }
        };
        html! {
            <img
                {src}
                alt={ctx.props().alt.clone()}
            />
        }
    }
}
