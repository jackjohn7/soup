use axum::response::{Html, IntoResponse, Response};
use maud::{html, Markup};

pub struct Layout {
    pub title: String,
    pub description: String,
}

impl Layout {
    pub fn render(self, template: Markup) -> Response {
        let with_layout = html! {
            head {
                title {
                    (self.title)
                }

                meta name="description" content=(self.description) {}

                div {
                    (template)
                }
            }
        }
        .into_string();
        Html(with_layout).into_response()
    }

    pub fn builder() -> LayoutBuilder {
        LayoutBuilder::default()
    }
}

pub struct LayoutBuilder {
    pub title: String,
    pub description: String,
}

impl LayoutBuilder {
    pub fn default() -> Self {
        Self {
            title: "Soup.rs".into(),
            description: "Registry for distribution of app templates".into(),
        }
    }

    /// Set the title of the page
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    /// Set the description of the page
    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    pub fn build(self) -> Layout {
        Layout {
            title: self.title,
            description: self.description,
        }
    }
}
