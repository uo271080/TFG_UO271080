// use std::ffi::c_void;
use std::vec;

use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::convert::IntoWasmAbi;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

// use shex_validation::Validator;
// use shex_ast::{ast::Schema as SchemaJson, compiled::compiled_schema::CompiledSchema};
// use shex_compact::ShExParser;

const KEY: &str = "yew.todomvc.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    rdf_parameters:Vec<String>,
    shex_parameters:Vec<String>,
    shapemap_parameters:Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    filter: Filter,
    show_result: bool,
    rdf_value:String,
    shex_value:String,
    shapemap_value:String,
    edit_value: String,
}

pub enum Msg {
    SetFilter(Filter),
    Validate,
    UpdateInput(String),
    Nope
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state: State = State {
            filter: Filter::RDF,
            show_result:false,
            edit_value: "".into(),
            rdf_value:"".into(),
            shex_value:"".into(),
            shapemap_value:"".into()
        };
        App {
            link,
            storage,
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
            Msg::SetFilter(filter) => {
                filter.update(&mut self.state);
            }
            Msg::Validate => {
                print!("Incompleto");
                // let schema = ShExParser::parse(&self.state.shex_value, None).unwrap();
                // // let mut schema: CompiledSchema = CompiledSchema::new();
                // let mut validator = Validator::new(schema).with_max_steps(1);
                // debug!("Validating with max_steps: {}", 1);
                // let mut shapemap = match shapemap {
                //     None => QueryShapeMap::new(),
                //     Some(shapemap_buf) => parse_shapemap('', '')?,
                // };
                // let result = match &data {
                //     Data::Endpoint(_) => validator.validate_shapemap(&shapemap, endpoint),
                //     Data::RDFData(data) => validator.validate_shapemap(&shapemap, data),
                // };
        
                self.state.show_result=true;
            }
            Msg::UpdateInput(val) =>{
                println!("Input: {}", val);
                self.state.edit_value = val.clone();
                match self.state.filter {
                    Filter::RDF => self.state.rdf_value = val.clone(),
                    Filter::ShEx => self.state.shex_value = val.clone(),
                    Filter::ShapeMap => self.state.shapemap_value = val.clone(),
                }
                //self.state.update_edit_value();
            }
            Msg::Nope => {}
        }
        //self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="app">
                    <header class="header">
                        <h1>{ "RDF Shape Validator" }</h1>
                    </header>
                    <div class="footer">
                        <div class="multi-button">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </div>
                        <div>
                            { self.view_input() }
                        </div>
                        <div class="parameters">
                            { self.view_parameters() }
                        </div>
                        <button class="clear-completed text-white ml-auto mr-40 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" onclick=self.link.callback(|_| Msg::Validate)>
                            { format!("Validate") }
                        </button>
                        <div class="result">
                            {self.render_result()}
                        </div>
                        <div class="bg-blue w-10">
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter) -> Html {
        let flt = filter.clone();
        html! {
            <button class=if self.state.filter == flt { "selected" } else { "not-selected" } onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone()))>
                    { filter }
            </button>
        }
    }

    fn render_result(&self) -> Html {
        info!("Show result: {}", self.state.show_result);
        if self.state.show_result {
            html! {
                <div class="result">
                    {"EL RESULTADO SE MUESTRA!!!"}
                </div>
            }
        } else {
            html! {  
                <></>
            }
        }
    }

    fn view_input(&self) -> Html {
        info!("{}", &self.state.edit_value);
        info!("RDF VALUE: {}", self.state.rdf_value);
        info!("SHEX VALUE: {}", self.state.shex_value);
        info!("SHAPE MAP VALUE: {}", self.state.shapemap_value);
        html! {
            <textarea id="showcase" 
            class="new-todo" 
            placeholder="Escribe tu código aquí..."
            value=&self.state.edit_value
                   oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
            >
            </textarea>
        }
    }

    fn view_parameters(&self) -> Html {
        match self.state.filter {
            Filter::RDF => self.view_select(&self.rdf_parameters),
            Filter::ShEx => self.view_select(&self.shex_parameters),
            Filter::ShapeMap => self.view_select(&self.shapemap_parameters),
        }
    }

    fn view_select(&self, options: &Vec<String>) -> Html {
        html! {
            <select>
                {for options.iter().map(|opcion| {
                    html! {
                        <option value={opcion}>{opcion}</option>
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

impl Filter {
    fn update(&self,state:&mut State) -> bool{
        state.filter = self.clone();
        match *self { 
            Filter::RDF => state.edit_value = state.rdf_value.clone(),
            Filter::ShEx => state.edit_value = state.shex_value.clone(),
            Filter::ShapeMap => state.edit_value = state.shapemap_value.clone(),
        }
        return true;
    }
}

impl State {
    fn update_edit_value(&mut self) {
        match self.filter {
            Filter::RDF => self.edit_value = self.rdf_value.clone(),
            Filter::ShEx => self.edit_value = self.shex_value.clone(),
            Filter::ShapeMap => self.edit_value = self.shapemap_value.clone(),
        }
    }
}
