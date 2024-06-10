use serde::Deserialize;

/// `Env` contains mappings of environment variables relevant to the
/// soup web application. This includes information for authentication.
#[derive(Deserialize, Debug)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: String,

    pub github_oauth_client_id: Option<String>,
    pub gitlab_oauth_client_id: Option<String>,
}

fn default_port() -> String {
    "5173".into()
}

impl Default for Env {
    fn default() -> Self {
        Self {
            port: "5173".into(),
            github_oauth_client_id: None,
            gitlab_oauth_client_id: None,
        }
    }
}
