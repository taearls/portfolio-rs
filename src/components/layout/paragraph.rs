use yew::{classes, function_component, html, Children, Classes, Properties};

#[derive(Properties, PartialEq)]
pub struct ParagraphProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Paragraph)]
pub fn paragraph(props: &ParagraphProps) -> Html {
    let ParagraphProps {
        children,
        class,
        style,
    } = props;
    html! {
        <p
            class={classes!(class.clone(), "text-soft-black dark:text-white my-4 text-lg md:text-xl leading-normal".to_string())}
            style={style.clone()}
        >
            { for children.iter() }
        </p>
    }
}
