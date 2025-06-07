use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

use super::util;

pub struct Client {
    user_id: Option<String>,
    token_state: Mutex<TokenState>,
}

impl Client {
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let auth_info = AuthInfo::new(email, password);
        let auth_token = Self::get_auth_token(&auth_info).await?;
        let user_id = Some(util::extract_user_id_from_jwt(&auth_token)?);
        let token_state = Self::get_oauth_token(&auth_token).await?;

        Ok(Self {
            user_id,
            token_state: Mutex::new(token_state),
        })
    }

    pub fn user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    pub async fn get_authorization(&self) -> Option<String> {
        let mut token = self.token_state.lock().await;
        if token.requires_refresh() {
            let refreshed = Self::refresh_oauth_token(&token.refresh_token.as_ref().unwrap())
                .await
                .unwrap();
            token.id_token = refreshed.id_token;
            token.refresh_token = refreshed.refresh_token;
            token.expires_at = refreshed.expires_at;
        }

        token.id_token.as_ref().map(|t| format!("Bearer {}", t))
    }

    async fn get_auth_token(auth_info: &AuthInfo) -> Result<String> {
        let client = reqwest::Client::new();
        let url = util::get_auth_url();
        let headers = util::get_auth_headers();

        let auth_response: AuthResponse = client
            .post(url)
            .headers(headers)
            .json(&auth_info)
            .send()
            .await?
            .json()
            .await?;

        Ok(auth_response.token)
    }

    async fn get_oauth_token(auth_token: &str) -> Result<TokenState> {
        let now = Instant::now();
        let client = reqwest::Client::new();
        let url = util::get_oauth_login_url()?;
        let headers = util::get_ios_headers();
        let request = OAuthRequestBody::new(&auth_token);

        let response: OAuthRequestResponse = client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        let expiration_seconds: u64 = response.expires_in.trim().parse().unwrap();

        Ok(TokenState {
            id_token: Some(response.id_token),
            refresh_token: Some(response.refresh_token),
            expires_at: now.checked_add(Duration::from_secs(expiration_seconds)),
        })
    }

    async fn refresh_oauth_token(refresh_token: &str) -> Result<TokenState> {
        let now = Instant::now();
        let client = reqwest::Client::new();
        let url = util::get_oauth_refresh_url()?;
        let headers = util::get_ios_headers();
        let request = OAuthRefreshBody::new(&refresh_token);

        let response: OAuthRefreshResponse = client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        let expiration_seconds: u64 = response.expires_in.trim().parse().unwrap();

        Ok(TokenState {
            id_token: Some(response.id_token),
            refresh_token: Some(response.refresh_token),
            expires_at: now.checked_add(Duration::from_secs(expiration_seconds)),
        })
    }
}

#[derive(Serialize)]
struct AuthInfo {
    email: String,
    password: String,
}

impl AuthInfo {
    fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct TokenState {
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<Instant>,
}

impl TokenState {
    pub fn requires_refresh(&self) -> bool {
        match self.expires_at {
            Some(expiration) => {
                let now = Instant::now();
                expiration.saturating_duration_since(now) < std::time::Duration::from_secs(60)
            }
            None => true,
        }
    }
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OAuthRequestBody {
    return_secure_token: bool,
    token: String,
}

impl OAuthRequestBody {
    fn new(token: &str) -> Self {
        Self {
            return_secure_token: true,
            token: token.to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OAuthRequestResponse {
    id_token: String,
    refresh_token: String,
    expires_in: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OAuthRefreshBody {
    grant_type: String,
    refresh_token: String,
}

impl OAuthRefreshBody {
    fn new(refresh_token: &str) -> Self {
        Self {
            grant_type: "refresh_token".to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OAuthRefreshResponse {
    refresh_token: String,
    expires_in: String,
    id_token: String,
}
