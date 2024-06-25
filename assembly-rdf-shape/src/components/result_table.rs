use crate::app::api::ShapeMapEntry;
use crate::components::search_bar::SearchBar;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

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
    pub entries: Vec<ShapeMapEntry>,
    pub search_text: String,
    pub on_open_modal: Callback<(String, String)>,
}

pub struct ResultTable {
    link: ComponentLink<Self>,
    props: Props,
    current_page: usize,
    entries_per_page: usize,
}

pub enum Msg {
    OpenModal(String, String),
    NextPage,
    PreviousPage,
    GoToPage(usize),
    UpdateSearchText(String),
    ExportToCsv,
}

impl Component for ResultTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            current_page: 0,
            entries_per_page: 5, // Set to 5 entries per page
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenModal(node, reason) => {
                self.props.on_open_modal.emit((node, reason));
                true
            }
            Msg::UpdateSearchText(text) => {
                self.props.search_text = text;
                self.current_page = 0;
                true
            }
            Msg::NextPage => {
                if self.current_page < self.max_page() {
                    self.current_page += 1;
                }
                true
            }
            Msg::PreviousPage => {
                if self.current_page > 0 {
                    self.current_page -= 1;
                }
                true
            }
            Msg::GoToPage(page) => {
                if page <= self.max_page() {
                    self.current_page = page;
                }
                true
            }
            Msg::ExportToCsv => {
                self.export_to_csv();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let filtered_entries = self
            .props
            .entries
            .iter()
            .filter(|entry| {
                entry
                    .node
                    .to_lowercase()
                    .contains(&self.props.search_text.to_lowercase())
            })
            .collect::<Vec<_>>();

        let entries_to_display = filtered_entries
            .iter()
            .skip(self.current_page * self.entries_per_page)
            .take(self.entries_per_page)
            .collect::<Vec<_>>();

        html! {
            <div class="result" id="result">
                <div class="table-controls">
                    <SearchBar on_search=self.link.callback(Msg::UpdateSearchText) />
                    <button onclick=self.link.callback(|_| Msg::ExportToCsv)>{ "Export to CSV" }</button>
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
                { self.view_pagination(filtered_entries.len()) }
            </div>
        }
    }
}

impl ResultTable {
    fn export_to_csv(&self) {
        let csv_data = self.format_csv_data();
        exportCsv(&csv_data, "export.csv");
    }

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

    fn view_entry(&self, entry: &ShapeMapEntry) -> Html {
        let cloned_entry = entry.clone();
        html! {
            <tr class={ if entry.status == "Valid" { "valid" } else { "invalid" } }>
                <td>{ &entry.node }</td>
                <td>{ &entry.shape }</td>
                <td class="details-row">{ &entry.status }</td>
                <td>
                    <button type="button" class="show-btn" onclick=self.link.callback(move |_| Msg::OpenModal(cloned_entry.node.clone(), cloned_entry.reason.clone()))>
                        { "Show" }
                    </button>
                </td>
            </tr>
        }
    }

    fn view_pagination(&self, total_entries: usize) -> Html {
        let max_page = self.max_page();
        let is_prev_active = self.current_page > 0;
        let is_next_active = self.current_page < max_page;

        html! {
            <ul class="page">
                <li class={ if is_prev_active { "page__btn active" } else { "page__btn" } }
                    onclick=self.link.callback(|_| Msg::PreviousPage)>
                    <span class="material-icons">{"chevron_left"}</span>
                </li>
                { for (0..=max_page).map(|page| self.view_page_button(page)) }
                <li class={ if is_next_active { "page__btn active" } else { "page__btn" } }
                    onclick=self.link.callback(|_| Msg::NextPage)>
                    <span class="material-icons">{"chevron_right"}</span>
                </li>
            </ul>
        }
    }

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

    fn max_page(&self) -> usize {
        if self.props.entries.is_empty() {
            0
        } else {
            (self.props.entries.len() - 1) / self.entries_per_page
        }
    }
}
