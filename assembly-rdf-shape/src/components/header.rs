use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_example: Callback<String>,
}

pub enum Msg {
    LoadExample(String),
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadExample(example) => {
                self.props.on_load_example.emit(example);
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
            <header class="header">
                <nav>
                    <div class="wrapper">
                        <div class="logo"><a href="#">{"WASM - RDF VALIDATOR"}</a></div>
                        <ul class="nav-links">
                            <li class="dropdown">
                                <button class="dropbtn">{"LOAD EXAMPLE"}</button>
                                <div class="dropdown-content">
                                    <a class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example1".to_string()))>{"Example 1"}</a>
                                    <a class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example2".to_string()))>{"Example 2"}</a>
                                    <a class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example3".to_string()))>{"Example 3"}</a>
                                </div>
                            </li>
                        </ul>
                    </div>
                </nav>
            </header>
        }
    }
}
