use crate::app::api::ShapeMapEntry;
use crate::components::search_bar::SearchBar;
use yew::prelude::*;

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
}

impl Component for ResultTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            current_page: 0,
            entries_per_page: 10, // Número de entradas por página
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenModal(node, shape) => {
                let reason = self
                    .props
                    .entries
                    .iter()
                    .find(|entry| entry.node == node && entry.shape == shape)
                    .map(|entry| entry.reason.clone())
                    .unwrap_or_default();
                self.props.on_open_modal.emit((node, shape));
                true
            }
            Msg::UpdateSearchText(text) => {
                self.props.search_text = text;
                self.current_page = 0; // Restablecer a la primera página con nueva búsqueda
                true
            }
            Msg::NextPage => {
                let max_page = self.max_page();
                if self.current_page < max_page {
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
                let max_page = self.max_page();
                if page <= max_page {
                    self.current_page = page;
                }
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
                <table>
                    <tr>
                        <th>{"Node"}</th>
                        <th>{"Shape"}</th>
                        <th>{"Status"}</th>
                        <th class="details-col">{"Details"}</th>
                    </tr>
                    { for entries_to_display.iter().map(|entry| self.view_entry(entry)) }
                </table>
                { self.view_pagination(filtered_entries.len()) }
                <SearchBar on_search=self.link.callback(Msg::UpdateSearchText) />
            </div>
        }
    }
}

impl ResultTable {
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

        html! {
            <div class="pagination">
                <button onclick=self.link.callback(|_| Msg::PreviousPage) disabled=self.current_page == 0>{ "Previous" }</button>
                { (0..=max_page).map(|page| self.view_page_button(page)).collect::<Html>() }
                <button onclick=self.link.callback(|_| Msg::NextPage) disabled=self.current_page == max_page>{ "Next" }</button>
            </div>
        }
    }

    fn view_page_button(&self, page: usize) -> Html {
        html! {
            <button
                onclick=self.link.callback(move |_| Msg::GoToPage(page))
                class={ if self.current_page == page { "active" } else { "" } }
            >
                { page + 1 }
            </button>
        }
    }

    fn max_page(&self) -> usize {
        let total_filtered_entries = self
            .props
            .entries
            .iter()
            .filter(|entry| entry.node.contains(&self.props.search_text))
            .count();
        (total_filtered_entries + self.entries_per_page - 1) / self.entries_per_page - 1
    }
}
