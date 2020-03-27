use yew::html::Children;
use yew::prelude::*;

pub struct Header {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
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
