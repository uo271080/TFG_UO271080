use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
import YATE from 'perfectkb-yate';
export function initializeYate() {
    var yate = YATE.fromTextArea(document.getElementById('editor-yate'), {})
    window.yateInstance = yate;
}
")]
extern "C" {
    fn initializeYate();
}

#[wasm_bindgen(inline_js = "
import YASHE from 'yashe';
export function initializeYashe() {
    var yashe = YASHE.fromTextArea(document.getElementById('editor-yashe'), {});
    window.yasheInstance = yashe;
}
")]
extern "C" {
    fn initializeYashe();
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub shapemap_value: String,
    pub on_update_shapemap_value: Callback<String>,
    pub on_validate: Callback<()>,
    pub rdf_parameters: Vec<String>,
    pub shex_parameters: Vec<String>,
    pub shapemap_parameters: Vec<String>,
}

pub struct Editor {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    UpdateShapeMapValue(String),
    Validate,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateShapeMapValue(value) => {
                self.props.on_update_shapemap_value.emit(value);
                true
            }
            Msg::Validate => {
                self.props.on_validate.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            initializeYate();
            initializeYashe();
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="editors-container">
                <div class="yashe-container">
                    <h3 class="title-editor">{"RDF"}</h3>
                    <textarea id="editor-yate"></textarea>
                    { self.view_parameters(&self.props.rdf_parameters, "rdf") }
                    <div class="shapemap-container">
                        <h3 class="title-editor">{"ShapeMap"}</h3>
                        <textarea
                            class="shapemap-editor"
                            value=&self.props.shapemap_value
                            oninput=self.link.callback(|e: InputData| Msg::UpdateShapeMapValue(e.value))
                        />
                        { self.view_parameters(&self.props.shapemap_parameters, "shapemap") }
                    </div>
                </div>
                <div class="yate-container">
                    <h3 class="title-editor">{"ShEx"}</h3>
                    <textarea id="editor-yashe"></textarea>
                    { self.view_parameters(&self.props.shex_parameters, "shex") }
                    <div style="margin-top: auto;">
                        <button class="clear-completed button-27" onclick=self.link.callback(|_| Msg::Validate)>
                            { "VALIDATE" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl Editor {
    fn view_parameters(&self, options: &Vec<String>, filter: &str) -> Html {
        let select_class = format!("select parameters param-{}", filter);
        let id = format!("select-{}", filter);
        html! {
            <select class={select_class} id={id}>
                  { for options.iter().map(|option| html! { <option class="option-parameters" value={option}>{option}</option> }) }
            </select>
            }
        }
}
