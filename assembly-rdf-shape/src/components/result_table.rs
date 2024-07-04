/// Componente `ResultTable` que gestiona la visualización de resultados en formato tabla.
///
/// Este componente proporciona funcionalidades para paginar, buscar, exportar a CSV, y abrir modales con detalles
/// de cada entrada. Utiliza propiedades y mensajes para interactuar con el estado y las acciones del usuario.
use crate::app::api::ShapeMapEntry;
use crate::components::search_bar::SearchBar;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/// Declaración de una función JavaScript para exportar contenido CSV
#[wasm_bindgen(inline_js = r#"
    export function exportCsv(csvContent, fileName) {
        const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
        const link = document.createElement('a');
        const url = URL.createObjectURL(blob);
        link.setAttribute('href', url);
        link.setAttribute('download', fileName);
        link.style.visibility = 'hidden';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
    }
"#)]
extern "C" {
    pub fn exportCsv(csvContent: &str, fileName: &str);
}

#[derive(Properties, Clone)]
pub struct Props {
    /// Entradas del ShapeMap para mostrar en la tabla.
    pub entries: Vec<ShapeMapEntry>,
    /// Texto de búsqueda actual para filtrar las entradas.
    pub search_text: String,
    /// Callback para abrir un modal con información detallada.
    pub on_open_modal: Callback<(String, String)>,
}

/// Estado y lógica del componente `ResultTable`.
pub struct ResultTable {
    link: ComponentLink<Self>,
    props: Props,
    current_page: usize,
    entries_per_page: usize,
    show_all: bool,
}

/// Mensajes utilizados por `ResultTable` para manejar eventos de la interfaz de usuario.
pub enum Msg {
    /// Abre un modal con detalles específicos.
    OpenModal(String, String),
    /// Avanza a la próxima página de resultados.
    NextPage,
    /// Regresa a la página anterior de resultados.
    PreviousPage,
    /// Va directamente a una página especificada.
    GoToPage(usize),
    /// Actualiza el texto de búsqueda y reinicia la paginación.
    UpdateSearchText(String),
    /// Exporta las entradas actuales a un archivo CSV.
    ExportToCsv,
    /// Muestra todas las entradas en una sola página.
    ShowAll,
}

impl Component for ResultTable {
    type Message = Msg;
    type Properties = Props;

    /// Crea una instancia del componente con propiedades y enlace especificados.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            current_page: 0,
            entries_per_page: 5,
            show_all: false,
        }
    }

    /// Actualiza el estado del componente en respuesta a los mensajes.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenModal(node, reason) => {
                self.props.on_open_modal.emit((node, reason));
                true
            }
            Msg::UpdateSearchText(text) => {
                self.props.search_text = text;
                self.current_page = 0;
                self.show_all = false;
                true
            }
            Msg::NextPage => {
                self.show_all = false;
                if self.current_page < self.max_page() {
                    self.current_page += 1;
                }
                true
            }
            Msg::PreviousPage => {
                self.show_all = false;
                if self.current_page > 0 {
                    self.current_page -= 1;
                }
                true
            }
            Msg::GoToPage(page) => {
                self.show_all = false;
                if page <= self.max_page() {
                    self.current_page = page;
                }
                true
            }
            Msg::ExportToCsv => {
                self.export_to_csv();
                true
            }
            Msg::ShowAll => {
                self.show_all = true;
                self.current_page = self.max_page() + 1;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    /// Renderiza el componente como HTML.

    fn view(&self) -> Html {
        let filtered_entries: Vec<&ShapeMapEntry> = self
            .props
            .entries
            .iter()
            .filter(|entry| {
                entry
                    .node
                    .to_lowercase()
                    .contains(&self.props.search_text.to_lowercase())
            })
            .collect();

        let entries_to_display: Vec<&ShapeMapEntry> = if self.show_all {
            filtered_entries.clone()
        } else {
            filtered_entries
                .iter()
                .skip(self.current_page * self.entries_per_page)
                .take(self.entries_per_page)
                .cloned()
                .collect()
        };

        html! {
            <div class="result" id="result">
                <div class="table-controls">
                    <SearchBar on_search=self.link.callback(Msg::UpdateSearchText) />
                    <button id="export-btn" class="download-btn" onclick=self.link.callback(|_| Msg::ExportToCsv)> <i class="fas fa-download"></i></button>
                </div>
                <table id="result-table">
                    <tr>
                        <th>{"Node"}</th>
                        <th>{"Shape"}</th>
                        <th>{"Status"}</th>
                        <th class="details-col">{"Details"}</th>
                    </tr>
                    { for entries_to_display.iter().map(|entry| self.view_entry(entry)) }
                </table>
                { self.view_pagination() }
            </div>
        }
    }
}

impl ResultTable {
    /// Exporta las entradas filtradas a un archivo CSV.
    fn export_to_csv(&self) {
        let csv_data = self.format_csv_data();
        exportCsv(&csv_data, "export.csv");
    }

    /// Formatea las entradas actuales para la exportación CSV.
    fn format_csv_data(&self) -> String {
        let header = "Node;Shape;Status;Reason\n".to_string();
        let csv_data = self.props.entries.iter().fold(header, |acc, entry| {
            format!(
                "{}{};{};{};{}\n",
                acc,
                entry.node,
                entry.shape,
                entry.status,
                entry.reason.replace('\n', "")
            )
        });
        csv_data
    }

    /// Renderiza una entrada individual en la tabla.
    fn view_entry(&self, entry: &ShapeMapEntry) -> Html {
        let cloned_entry = entry.clone();
        html! {
            <tr class={ if entry.status == "Valid" { "valid" } else { "invalid" } }>
                <td>{ &entry.node }</td>
                <td>{ &entry.shape }</td>
                <td class="details-row">{ &entry.status }</td>
                <td>
                    <button type="button" class="show-btn" onclick=self.link.callback(move |_| Msg::OpenModal(cloned_entry.node.clone(), cloned_entry.reason.clone()))>
                        <i class="fas fa-plus"></i>
                    </button>
                </td>
            </tr>
        }
    }

    /// Renderiza los controles de paginación.
    fn view_pagination(&self) -> Html {
        let max_page = self.max_page();
        let is_prev_active = self.current_page > 0 && !self.show_all;
        let is_next_active = self.current_page < max_page && !self.show_all;

        html! {
            <ul class="page">
                <li class={ if is_prev_active { "page__btn active" } else { "page__btn" } }
                    onclick=self.link.callback(|_| Msg::PreviousPage)>
                    <span class="material-icons">{"chevron_left"}</span>
                </li>
                { for (0..=max_page).map(|page| self.view_page_button(page)) }
                <li id="show-all" class={ if self.show_all == true { "page__numbers active" } else { "page__numbers" } } onclick=self.link.callback(|_| Msg::ShowAll)>
                    { "All" }
                </li>
                <li class={ if is_next_active { "page__btn active" } else { "page__btn" } }
                    onclick=self.link.callback(|_| Msg::NextPage)>
                    <span class="material-icons">{"chevron_right"}</span>
                </li>
            </ul>
        }
    }

    /// Renderiza los botones de la paginación
    fn view_page_button(&self, page: usize) -> Html {
        html! {
            <li
                class={ if self.current_page == page { "page__numbers active" } else { "page__numbers" } }
                onclick=self.link.callback(move |_| Msg::GoToPage(page))
            >
                { page + 1 }
            </li>
        }
    }

    /// Calcula el número máximo de páginas basado en las entradas y la configuración de paginación.
    fn max_page(&self) -> usize {
        if self.props.entries.is_empty() {
            0
        } else {
            (self.props.entries.len() - 1) / self.entries_per_page
        }
    }
}
