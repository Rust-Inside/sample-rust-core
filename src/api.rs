use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub id: String,
    pub handle: String,
    pub display_name: String,
}

pub struct AccountClient {
    session: Result<Session, String>,
}

impl AccountClient {
    pub fn new(base_url: String, authorization: String) -> Self {
        Self {
            session: Session::new(&base_url, &authorization),
        }
    }

    pub async fn get_me(&self) -> Result<Me, String> {
        let session = self.session.as_ref().map_err(Clone::clone)?;
        let response = session
            .http
            .get(session.endpoint.clone())
            .timeout(Duration::from_secs(15))
            .send()
            .await
            .map_err(|error| format!("Profile request failed: {error}"))?;
        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status().as_u16()));
        }
        response
            .json::<Me>()
            .await
            .map_err(|_| "Invalid profile response".into())
    }
}

#[rust_inside::server]
pub mod server {
    use super::Me;

    /// The account is selected by the authenticated session, so this route has no ID.
    pub fn me_path() -> String {
        "/v1/me".into()
    }

    pub fn me_response_json(id: String, handle: String, nickname: String) -> String {
        let display_name = match nickname.trim() {
            "" => format!("@{handle}"),
            name => name.to_owned(),
        };
        let me = Me {
            id,
            handle,
            display_name,
        };
        serde_json::to_string(&me).expect("profile fields are strings")
    }
}

struct Session {
    http: Client,
    endpoint: Url,
}

impl Session {
    fn new(base_url: &str, authorization: &str) -> Result<Self, String> {
        let mut endpoint = Url::parse(base_url).map_err(|_| "Invalid API origin")?;
        if !matches!(endpoint.scheme(), "http" | "https")
            || endpoint.host().is_none()
            || !endpoint.username().is_empty()
            || endpoint.password().is_some()
        {
            return Err("Expected an HTTP(S) API origin without embedded credentials".into());
        }
        endpoint.set_path(&server::me_path());
        endpoint.set_query(None);
        endpoint.set_fragment(None);
        let mut headers = HeaderMap::new();
        if !authorization.is_empty() {
            let mut value =
                HeaderValue::from_str(authorization).map_err(|_| "Invalid authorization header")?;
            value.set_sensitive(true);
            headers.insert(AUTHORIZATION, value);
        }
        let http = http_client()?
            .default_headers(headers)
            .build()
            .map_err(|error| format!("Cannot initialize profile client: {error}"))?;
        Ok(Self { http, endpoint })
    }
}

#[cfg(target_arch = "wasm32")]
fn http_client() -> Result<reqwest::ClientBuilder, String> {
    // Browser Fetch supplies TLS and same-origin session cookies.
    Ok(Client::builder())
}

#[cfg(not(target_arch = "wasm32"))]
fn http_client() -> Result<reqwest::ClientBuilder, String> {
    // Bundle Mozilla's certificate roots so mobile callers need no platform TLS setup.
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let tls = rustls::ClientConfig::builder_with_provider(
        rustls::crypto::ring::default_provider().into(),
    )
    .with_safe_default_protocol_versions()
    .map_err(|error| format!("Cannot initialize TLS: {error}"))?
    .with_root_certificates(roots)
    .with_no_client_auth();
    Ok(Client::builder()
        .tls_backend_preconfigured(tls)
        .redirect(reqwest::redirect::Policy::none()))
}

#[cfg(test)]
mod tests {
    use super::server::{me_path, me_response_json};
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn public_profile_uses_the_session_route_and_falls_back_to_the_handle() {
        assert_eq!(me_path(), "/v1/me");
        let body = me_response_json("u42".into(), "martin".into(), "  ".into());
        assert_eq!(
            serde_json::from_str::<Value>(&body).unwrap(),
            json!({ "id": "u42", "handle": "martin", "displayName": "@martin" })
        );
    }

    #[test]
    fn trims_and_escapes_the_nickname_and_returns_only_public_fields() {
        let body = me_response_json("u42".into(), "martin".into(), "  Martin \"마틴\"  ".into());
        assert_eq!(
            serde_json::from_str::<Value>(&body).unwrap(),
            json!({ "id": "u42", "handle": "martin", "displayName": "Martin \"마틴\"" })
        );
    }

    #[test]
    fn invalid_origins_and_headers_are_rejected_before_a_request() {
        for origin in [
            "not a URL",
            "file:///tmp/me",
            "https://user:password@example.org",
        ] {
            assert!(Session::new(origin, "Bearer sample").is_err(), "{origin}");
        }
        assert!(Session::new("https://example.org", "invalid\nheader").is_err());
    }

    #[test]
    fn a_client_can_be_configured_before_entering_an_async_runtime() {
        let client = AccountClient::new(
            "https://example.org:8443/path?query=1#section".into(),
            "Bearer sample".into(),
        );
        let session = client.session.unwrap();
        assert_eq!(session.endpoint.as_str(), "https://example.org:8443/v1/me");
    }
}
