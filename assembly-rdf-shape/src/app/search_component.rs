use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::events::InputData;

pub struct SearchComponent {
    link: ComponentLink<Self>,
    search_text: String,
}

pub enum Msg {
    UpdateSearch(String),
}

impl Component for SearchComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            search_text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateSearch(value) => {
                self.search_text = value;
                true // Indicates that the Component should re-render
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form class="search-form">
                    <input type="search"
                           value={self.search_text.clone()}
                           placeholder="Search"
                           class="search-input"
                           oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value)) />
                    <button type="submit" class="search-button">
                        
                    </button>
                </form>
                <svg xmlns="http://www.w3.org/2000/svg" width="0" height="0" display="none">
                    <symbol id="search" viewBox="0 0 32 32">
                        <path d="M 19.5 3 C 14.26514 3 10 7.2651394 10 12.5 C 10 14.749977 10.810825 16.807458 12.125 18.4375 L 3.28125 27.28125 L 4.71875 28.71875 L 13.5625 19.875 C 15.192542 21.189175 17.250023 22 19.5 22 C 24.73486 22 29 17.73486 29 12.5 C 29 7.2651394 24.73486 3 19.5 3 z M 19.5 5 C 23.65398 5 27 8.3460198 27 12.5 C 27 16.65398 23.65398 20 19.5 20 C 15.34602 20 12 16.65398 12 12.5 C 12 8.3460198 15.34602 5 19.5 5 z" />
                    </symbol>
                </svg>
            </>
        }
    }
}
