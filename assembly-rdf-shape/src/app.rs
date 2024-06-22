mod api;
mod examples_manager;
mod rdf_properties;


use api::{ShapeMap, ShapeMapEntry};
use log::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::prelude::*;


pub struct App {
    link: ComponentLink<Self>,
    state: State,
    rdf_parameters:Vec<String>,
    shex_parameters:Vec<String>,
    shapemap_parameters:Vec<String>,
}

#[wasm_bindgen(inline_js = r#"
export function exportCsv(csvContent, fileName) {
    // Crear un Blob con el contenido del CSV
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });

    // Crear un enlace para la descarga
    const link = document.createElement('a');

    // Usar URL.createObjectURL para obtener una URL para el blob
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', fileName);

    // Asegurarse que el link sea no visible y añadirlo al DOM
    link.style.visibility = 'hidden';
    document.body.appendChild(link);

    // Hacer clic en el enlace para descargar el archivo
    link.click();

    // Limpiar y remover el enlace del DOM
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
}
"#)]
extern "C" {
    pub fn exportCsv(csvContent: &str, fileName: &str);
}



#[wasm_bindgen(inline_js = "
import YATE from 'perfectkb-yate';
export function setYate(input) {
    return window.yateInstance.setValue(input);
}
")]
extern "C" {
    fn setYate(input: &str);
}

#[wasm_bindgen(inline_js = "
export function setYashe(input) {
    return window.yasheInstance.setValue(input);
}
")]
extern "C" {
    fn setYashe(input: &str);
}

#[wasm_bindgen(inline_js = "
export function getYate() {
	return window.yateInstance.getValue();
}
")]
extern "C" {
    fn getYate() -> String;
}

#[wasm_bindgen(inline_js = "
export function getYashe() {
	return window.yasheInstance.getValue();
}
")]
extern "C" {
    fn getYashe() -> String;
}

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

#[wasm_bindgen(inline_js = "
export function scrollToElement(id) {
  const element = document.getElementById(id);
  if (element) {
    const elementRect = element.getBoundingClientRect();
    const viewportHeight = window.innerHeight || document.documentElement.clientHeight;

    // Calculate scroll position for smooth scrolling to element's midpoint
    const scrollY = elementRect.top + window.pageYOffset - (viewportHeight / 2);

    element.scrollIntoView({
      behavior: 'smooth',
      block: 'center', // Ensure vertical centering
    });
  }
}
")]
extern "C" {
    fn scrollToElement(id: &str);
}

#[derive(Serialize, Deserialize)]
pub struct State {
    filter: Filter,
    show_result: bool,
    scroll_needed: bool,
    shapemap_value:String,
    edit_value: String,
    search_text: String,
    validation_result:Option<api::ValidationResult>,
    api_error:String
}

#[derive(Serialize, Deserialize)]
pub struct ExampleData {
    rdf: String,
    shex: String,
    shapemap: String
}

pub enum Msg {
    Validate,
    ValidationResult(api::ValidationResult,String),
    UpdateSearch(String),
    UpdateShapeMapValue(String),
    LoadExample,
    CloseAlert,
    ExportToCsv,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let storage = StorageService::new(Area::Local).unwrap();
        let state: State = State {
            filter: Filter::RDF,
            show_result:false,
            scroll_needed: false,
            edit_value: "".into(),
            shapemap_value:"".into(),
            search_text: "".into(),
            validation_result:None,
            api_error:"".into()
        };
        App {
            link,
            state,
            rdf_parameters : vec![
                "Turtle".to_string(),
                "N-Triples".to_string(),
                "N-Quads".to_string(),
                "TriG".to_string(),
                "JSON-LD".to_string(),
                "RDF/XML".to_string(),
                "RDF/JSON".to_string(),
                "mixed".to_string(),
                "html-rdfa11".to_string(),
                "html-microdata".to_string(),
            ],
            shex_parameters : vec![
                "ShExC".to_string(),
                "ShExJ".to_string(),
            ],
            shapemap_parameters : vec![
                "Compact".to_string(),
                "JSON".to_string(),
            ]
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Validate => {
                print!("Incompleto");
                // rdf_properties::testOxttl();
                self.state.api_error = "".to_string();
                self.state.validation_result = Default::default();
                self.state.show_result = true;
                self.state.scroll_needed = true;
                let rdf_content = getYate();
                let shex_content = getYashe();
                let shapemap_content = self.state.shapemap_value.clone();
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = api::call_validation_api(rdf_content, shex_content, shapemap_content).await;
                    link.send_message(Msg::ValidationResult(result.0,result.1)); // Manejando la respuesta de validación
                });
            },
            Msg::CloseAlert  =>{
                self.state.api_error="".to_string();
            },
            Msg::ExportToCsv =>{
                let csv_data = App::format_csv_data(&self);
                exportCsv(&csv_data, "export.csv");
                // let blob = Blob::new_with_str_sequence(&wasm_bindgen::JsValue::from_serde(&[&csv_data]).unwrap()).unwrap();
                // let url = Url::create_object_url_with_blob(&blob).unwrap();
                // let window = window().unwrap();
                // window.location().set_href(&url).unwrap();
            }
            Msg::UpdateShapeMapValue(new_value) => {
                self.state.shapemap_value = new_value;
            },
            Msg::ValidationResult(result,error) => {
                if !error.is_empty(){
                    self.state.api_error = error;
                }
                else{
                    self.state.validation_result = Some(result);  
                }
            },
            Msg::UpdateSearch(text) => {
                self.state.search_text = text.to_lowercase();
            },
            Msg::LoadExample =>{
                let yate = r#"PREFIX :       <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf:   <http://xmlns.com/foaf/0.1/>

:alice schema:name           "Alice" ;            # %* Passes{:User} *)
       schema:gender         schema:Female ;
       schema:knows          :bob .

:bob   schema:gender         schema:Male ;        # %* Passes{:User} *)
       schema:name           "Robert";
       schema:birthDate      "1980-03-10"^^xsd:date .

:carol schema:name           "Carol" ;            # %* Passes{:User} *)
       schema:gender         "unspecified" ;
       foaf:name             "Carol" .

:dave  schema:name           "Dave";         # %* Fails{:User} *)
       schema:gender         "XYY";          #
       schema:birthDate      1980 .          # %* 1980 is not an xsd:date *)

:emily schema:name "Emily", "Emilee" ;       # %* Fails{:User} *)
       schema:gender         schema:Female . # %* too many schema:names *)

:frank foaf:name             "Frank" ;       # %* Fails{:User} *)
       schema:gender:        schema:Male .   # %* missing schema:name *)

:grace schema:name           "Grace" ;       # %* Fails{:User} *)
       schema:gender         schema:Male ;   #
       schema:knows          _:x .           # %* _:x is not an IRI *)

:harold schema:name         "Harold" ;    # %* Fails{:User} *)
        schema:gender       schema:Male ;
        schema:knows        :grace .      # %* :grace does not conform to :User *)
    "#;
                setYate(&yate);
                setYashe("PREFIX : <http://example.org/>\nPREFIX schema: <http://schema.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n\n:User {\n  schema:name xsd:string ;\n  schema:birthDate xsd:date? ;\n  schema:gender [ schema:Male schema:Female ] OR xsd:string ;\n  schema:knows IRI @:User*\n}");
                self.state.shapemap_value = ":alice@:User,:bob@:User,:carol@:User,:emily@:User,:frank@:User,:grace@:User,:harold@:User".to_string();
            }
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if self.state.scroll_needed && !first_render {
           scrollToElement("result");
            self.state.scroll_needed = false;
        }
        if first_render{
            initializeYate();
            initializeYashe();
        }
        // showYashe();
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="app">
                    <header class="header">
                    <nav>
                            <div class="wrapper">
                                <div class="logo"><a href="#">{"WASM - RDF VALIDATOR"}</a></div>
                                <input type="radio" name="slider" id="menu-btn" />
                                <input type="radio" name="slider" id="close-btn" />
                                <ul class="nav-links">
                                    <label for="close-btn" class="btn close-btn"><i class="fas fa-times"></i></label>
                                    <li class="menu-btn"><a class="load-example" onclick=self.link.callback(|_| Msg::LoadExample)>{"LOAD EXAMPLE"}</a></li>
                                </ul>
                                <label for="menu-btn" class="btn menu-btn"><i class="fas fa-bars"></i></label>
                            </div>
                        </nav>
                    </header>
                    <div class="content">
                        <div class="editors-container">
                            <div class="yashe-container">
                                <h3 class="title-editor">{"RDF"}</h3>
                                <textarea id="editor-yate"></textarea>
                                { self.view_parameters(Filter::RDF) }
                                <div class="shapemap-container">
                                    <h3 class="title-editor">{"ShapeMap"}</h3>
                                    <textarea class="shapemap-editor" oninput=self.link.callback(|e: InputData| Msg::UpdateShapeMapValue(e.value))>
                                        {self.state.shapemap_value.clone()}
                                    </textarea>
                                    { self.view_parameters(Filter::ShapeMap) }
                                </div>
                            </div>
                            <div class="yate-container">
                                <h3 class="title-editor">{"ShEx"}</h3>
                                <textarea id="editor-yashe"></textarea>
                                <div>
                                    { self.view_parameters(Filter::ShEx) }
                                </div>
                                <div style="margin-top: auto;">
                                    <button class="clear-completed button-27" onclick=self.link.callback(|_| Msg::Validate)>
                                        { format!("VALIDATE") }
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="footer-options">
                        // <button class="clear-completed validate-btn" onclick=self.link.callback(|_| Msg::LoadExample)>
                        //     { format!("Testing") }
                        // </button>
                        </div>
                        <div class="result-container">
                            {self.render_result()}
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}

impl App {

    fn format_csv_data(&self) -> String {
        if let Some(result) = &self.state.validation_result {
            // Añadir el encabezado con Reason incluido
            let header = "Node;Shape;Status;Reason\n".to_string(); // Asegúrate de que el delimitador usado en el encabezado y los datos coincida
            let entries: Vec<_> = result.result.shape_map.iter()
                .map(|entry| {
                    // Directamente se incluye entry.reason ya que ahora es un String
                    format!("{};{};{};{}\n", entry.node, entry.shape, entry.status, entry.reason)
                })
                .collect();
    
            let csv_data = entries.into_iter().fold(header, |acc, line| acc + &line);
            csv_data
        } else {
            "".to_string()
        }
    }
    
    
    fn render_modal(&self,shape:ShapeMapEntry) -> Html{
        html!{
            <div class="reason-modal">
                <h4>{shape.node}</h4>
            </div>
        }
    }

    fn render_result(&self) -> Html {
        info!("Show result: {}", self.state.show_result);
        info!("Show result: {}", self.state.api_error);

        if self.state.show_result && self.state.api_error.is_empty() {
            let search_text = self.state.search_text.clone();
            html! {
                <div class="result" id="result">
                // <div class="spinner-border" role="status">
                //     <span class="visually-hidden"></span>
                // </div>
                    <table>
                        <tr>
                            <th>{"Node"}</th>
                            <th>{"Shape"}</th>
                            <th>{"Status"}</th>
                            <th class="details-col">{"Details"}</th>
                        </tr>
                        { self.render_rows(&search_text) }
                    </table>
                    <div class="result-options">
                        <input type="text" class="search" placeholder="Buscar..." oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value)) />
                        <button onclick=self.link.callback(|_| Msg::ExportToCsv)>{ "Export to CSV" }</button>
                    </div>
                </div>
            }
        } else if !self.state.api_error.is_empty(){
            html!{
                <div class="alert-error">
                    {"Error en la validación. Revise las entradas."}
                    <button class={"close-btn "} onclick=self.link.callback(|_| Msg::CloseAlert)>{ "X" }</button>
                </div>
            }   
        }else {
            html! {
                <></>
            }
        }
    }

    fn render_rows(&self, search_text: &str) -> Html {
        if let Some(result) = &self.state.validation_result {
            let mut entries: Vec<_> = result.result.shape_map.iter()
                .filter(|entry| entry.node.contains(search_text))
                .collect();

            entries.sort_by(|a, b| b.status.cmp(&a.status));

            entries.into_iter().map(|entry| {
                html! {
                    <tr class={ if entry.status == "Valid" { "valid" } else { "invalid" } }>
                        <td>{ &entry.node }</td>
                        <td>{ &entry.shape }</td>
                        <td class="details-row">{ &entry.status }</td>
                        <td>
                            <button type="button" class="btn btn-primary" data-toggle="modal" data-target="#exampleModalCenter">
                                {"Launch demo modal"}
                            </button>
                        </td>
                    </tr>
                }
            }).collect()
        } else {
            html! {}
        }
    }


    
    fn view_parameters(&self,filter: Filter) -> Html {
        match filter {
            Filter::RDF => self.view_select(&self.rdf_parameters,"rdf".to_string()),
            Filter::ShEx => self.view_select(&self.shex_parameters,"shex".to_string()),
            Filter::ShapeMap => self.view_select(&self.shapemap_parameters,"shapemap".to_string()),
        }
    }

    fn view_select(&self, options: &Vec<String>,filter:String) -> Html {
        let select_class = format!("select parameters param-{}", filter);
        let id = format!("select-{}",filter);
        html! {
            <select class={select_class} id={id}>
                {for options.iter().map(|opcion| {
                    html! {
                        <option class="option-parameters" value={opcion}>{opcion}</option>
                    }
                })}
            </select>
        }
    }

}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    RDF,
    ShEx,
    ShapeMap,
}

impl State {

}
