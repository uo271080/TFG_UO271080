use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_example: Callback<()>,
}

pub enum Msg {
    LoadExample,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadExample => {
                self.props.on_load_example.emit(());
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
                        <input type="radio" name="slider" id="menu-btn" />
                        <input type="radio" name="slider" id="close-btn" />
                        <ul class="nav-links">
                            <label for="close-btn" class="btn close-btn"><i class="fas fa-times"></i></label>
                            <li class="menu-btn">
                                <a class="load-example" onclick=self.link.callback(|_| Msg::LoadExample)>{"LOAD EXAMPLE"}</a>
                            </li>
                        </ul>
                        <label for="menu-btn" class="btn menu-btn"><i class="fas fa-bars"></i></label>
                    </div>
                </nav>
            </header>
        }
    }
}
