use yew::prelude::*;

/// Componente `Header` que proporciona una interfaz de usuario para la navegación y la carga de ejemplos.
///
/// Este componente incluye un menú de navegación con opciones para cargar ejemplos predefinidos.
/// Cada opción emite un callback con el identificador del ejemplo seleccionado, que puede ser utilizado
/// por otros componentes para realizar acciones específicas, como cargar datos o configurar el entorno.
pub struct Header {
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_load_example: Callback<String>,
}

pub enum Msg {
    /// Mensaje que indica la carga de un ejemplo específico.
    /// Contiene el identificador del ejemplo a cargar.
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
                                <button id="examples-dropdown" class="dropbtn">{"LOAD EXAMPLE"}</button>
                                <div class="dropdown-content">
                                    <a id="example-1" class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example1".to_string()))>{"Example 1"}</a>
                                    <a id="example-2" class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example2".to_string()))>{"Example 2"}</a>
                                    <a id="example-3" class="dropdown-btn" href="#" onclick=self.link.callback(|_| Msg::LoadExample("example3".to_string()))>{"Example 3"}</a>
                                </div>
                            </li>
                        </ul>
                    </div>
                </nav>
            </header>
        }
    }
}
