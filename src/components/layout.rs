use axum::response::{Html, IntoResponse, Response};
use maud::{html, Markup};

const TAILWINDCSS_ROUTE: &str = "/public/styles/twout.css";

pub struct Layout {
    pub title: String,
    pub description: String,
    pub styles: Vec<Markup>,
}

impl Layout {
    pub fn render(self, template: Markup) -> Response {
        let with_layout = html! {
            head {
                title {
                    (self.title)
                }

                meta name="description" content=(self.description) {}

                @for style in &self.styles {
                    (*style)
                }

                script
                    src="https://unpkg.com/htmx.org@1.9.12"
                    integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2"
                    crossorigin="anonymous" {}

                div #root {
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
    pub styles: Vec<Markup>,
}

impl LayoutBuilder {
    pub fn default() -> Self {
        Self {
            title: "Soup.rs".into(),
            description: "Registry for distribution of app templates".into(),
            styles: Vec::new(),
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

    /// No tailwindcss
    pub fn tailwindcss(&mut self) -> &mut Self {
        self.link_style(TAILWINDCSS_ROUTE.into())
    }

    /// Add new linked style
    pub fn link_style(&mut self, stylesheet: String) -> &mut Self {
        self.styles
            .push(html! { link href=(stylesheet) rel="stylesheet"; });
        self
    }

    pub fn build(&mut self) -> Layout {
        Layout {
            title: self.title.clone(),
            description: self.description.clone(),
            styles: self.styles.clone(),
        }
    }
}
