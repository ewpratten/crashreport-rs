use url::Url;

use super::provider::RepositoryProvider;

/// Given a URL, tries to figure out the service provider of the repository.
pub fn validate_repository(url: &Url) -> Option<RepositoryProvider> {
    match url.host_str() {
        Some("github.com") => Some(RepositoryProvider::GitHub(url.clone())),
        _ => None,
    }
}
