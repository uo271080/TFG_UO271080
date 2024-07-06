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
    pub on_open_modal: Callback<(String, String)>,
}

pub enum Msg {
    /// Mensaje que indica la carga de un ejemplo específico.
    /// Contiene el identificador del ejemplo a cargar.
    LoadExample(String),
    Help(),
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
            Msg::Help() => {
                let title = "HELP";
                let body = "Welcome to WASM - RDF Validator. This tool allows you to validate RDF data using Shex and ShapeMap.\n\n\
    TO GET STARTED, FOLLOW THESE STEPS:\n\
    1. Insert your RDF data in the editor labeled 'RDF'.\n\
    2. Insert your Shex data in the editor labeled 'Shex'. Here you define the shapes that will be used to validate your RDF data.\n\
    3. Insert your ShapeMap data in the editor labeled 'ShapeMap'. The ShapeMap links your RDF data with the shapes defined in Shex.\n\n\
    FORMAT OPTIONS:\n\
    -> RDF: You can choose between different formats:\n\t • Turtle \n\t • N-Triples \n\t • N-Quads \n\t • TriG \n\t • JSON-LD \n\t • RDF/XML \n\t • RDF/JSON \n\t • Mixed \n\t • html-rdfa11 \n\t • html-microdata \n\
    -> Shex: You can choose between ShexC and ShExJ formats.\n\
    -> ShapeMap: You can choose between Compact or JSON formats.\n\n\
    ---\n\
    ANALYSIS AND VALIDATION:\n\
    - Use the 'Analyze' button to analyze the Shex and RDF inputs separately.\n\
    - Use the 'Validate' button to validate your data. The results will be displayed in a table indicating whether your data complies with the defined shapes.\n\n\
    ---\n\
    WHAT IS VALIDATION?\n\
    Validation is the process of verifying that the RDF data conforms to the rules defined in the Shex shapes. This ensures that the data is correctly structured and meets the expected requirements.\n\n\
    ";
                self.props
                    .on_open_modal
                    .emit((title.to_string(), body.to_string()));
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
                            <button class="help-btn" onclick=self.link.callback(|_| Msg::Help())>{"HELP"}</button>
                        </ul>
                    </div>
                </nav>
            </header>
        }
    }
}
