use anyhow::Result;
use base64::Engine;
use reqwest::header::HeaderMap;
use serde::Deserialize;

const AUTH_API_KEY: &str = "w2tPFbjlP13GUmb8dMjUL5B2YyPVD3pJ7Ey6fz8v";
const AUTH_API_URL: &str = "https://42nk7qrhdg.execute-api.us-east-1.amazonaws.com/prod/login";

const IOS_HEADER_KEY: &str = "x-ios-bundle-identifier";
const IOS_HEADER_VAL: &str = "com.whisker.ios";

const OAUTH_LOGIN_URL: &str =
    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyCustomToken";
const OAUTH_REFRESH_ENDPOINT: &str = "https://securetoken.googleapis.com/v1/token";
const OAUTH_KEY: &str = "QUl6YVN5Q3Y4NGplbDdKa0NRbHNncXJfc2xYZjNmM3gtY01HMTVR";

const LR_API_KEY: &str = "p7ndMoj61npRZP5CVz9v4Uj0bG769xy6758QRBPb";
const LR_API_BASE_URI: &str = "https://v2.api.whisker.iothings.site";

pub fn get_auth_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", AUTH_API_KEY.parse().unwrap());

    headers
}

pub fn get_ios_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(IOS_HEADER_KEY, IOS_HEADER_VAL.parse().unwrap());

    headers
}

pub fn get_api_headers(bearer_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", bearer_token.parse().unwrap());
    headers.insert("x-api-key", LR_API_KEY.parse().unwrap());

    headers
}

#[derive(Deserialize)]
struct JWT {
    claims: Claims,
}

#[derive(Deserialize)]
struct Claims {
    mid: String,
}

pub fn extract_user_id_from_jwt(auth_token: &str) -> Result<String> {
    let sections: Vec<&str> = auth_token.split(".").collect();
    let payload = sections.get(1).unwrap();
    let decoded = base64::prelude::BASE64_STANDARD_NO_PAD.decode(*payload)?;
    let jwt: JWT = serde_json::from_slice(&decoded)?;

    Ok(jwt.claims.mid)
}

pub fn get_auth_url() -> String {
    AUTH_API_URL.to_string()
}

pub fn get_oauth_login_url() -> Result<String> {
    get_oauth_url(OAUTH_LOGIN_URL)
}

pub fn get_oauth_refresh_url() -> Result<String> {
    get_oauth_url(OAUTH_REFRESH_ENDPOINT)
}

fn get_oauth_url(host: &str) -> Result<String> {
    let bytes = base64::prelude::BASE64_STANDARD.decode(OAUTH_KEY)?;
    let key = String::from_utf8(bytes)?;

    Ok(format!("{host}?key={key}"))
}

pub fn get_robots_url(user_id: &str) -> String {
    format!("{LR_API_BASE_URI}/users/{user_id}/robots")
}

pub fn get_robot_by_id_url(user_id: &str, robot_id: &str) -> String {
    let base = get_robots_url(user_id);
    format!("{base}/{robot_id}")
}

pub fn get_dispatch_command_url(user_id: &str, robot_id: &str) -> String {
    let base = get_robot_by_id_url(user_id, robot_id);
    format!("{base}/{robot_id}/dispatch-commands")
}
