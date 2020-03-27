use yew::html::Children;
use yew::prelude::*;

pub struct Root {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Root {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header>
              {"This is a component"}
              <button>{"button"}</button>
            </header>
        }
    }
}
