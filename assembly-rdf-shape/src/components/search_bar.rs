use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_search: Callback<String>,
}

pub struct SearchBar {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Search(String),
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(value) => {
                self.props.on_search.emit(value);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <input type="text"
        class="search"
        placeholder="Buscar..."
        oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />        }
    }
}
