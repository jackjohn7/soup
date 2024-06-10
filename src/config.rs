use serde::Deserialize;

/// `Env` contains mappings of environment variables relevant to the
/// soup web application. This includes information for authentication.
#[derive(Deserialize, Debug)]
pub struct Env {
    pub github_oauth_client_id: Option<String>,
    pub gitlab_oauth_client_id: Option<String>,
}
