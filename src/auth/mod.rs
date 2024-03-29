use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::str;

pub async fn search_schools(school: &str) -> SduiResult<Vec<School>> {
    let response = CLIENT
        .get(format!("https://api.sdui.app/v1/leads?search={}", school))
        .send()
        .await
        .map_err(SduiError::RequestError)?;
    let rate_limit = RateLimit::from_headers(response.headers());
    let data = response
        .json::<GenericSduiResponse>()
        .await
        .map_err(SduiError::RequestError)?;
    let schools = data
        .data
        .as_array()
        .ok_or(SduiError::JSONError)?
        .iter()
        .filter_map(School::from_value)
        .collect();
    Ok((schools, rate_limit))
}

pub async fn login(data: &LoginData) -> SduiResult<LoginResponse> {
    let response = CLIENT
        .post("https://api.sdui.app/v1/auth/login")
        .json(data)
        .send()
        .await
        .map_err(SduiError::RequestError)?;
    let rate_limit = RateLimit::from_headers(response.headers());
    let data: GenericSduiResponse = response.json().await.map_err(|_| SduiError::JSONError)?;
    Ok((
        LoginResponse::from_value(data.data).ok_or(SduiError::LoginError)?,
        rate_limit,
    ))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LoginResponse {
    access_token: String,
    expires_in: u64,
}

impl LoginResponse {
    fn from_value(value: serde_json::Value) -> Option<Self> {
        Some(LoginResponse {
            access_token: value
                .as_object()?
                .get("access_token")?
                .as_str()?
                .to_string(),
            expires_in: value.as_object()?.get("expires_in")?.as_u64()?,
        })
    }
    pub fn get_token(&self) -> String {
        self.access_token.clone()
    }
    pub fn get_expires_in(&self) -> u64 {
        self.expires_in
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    pub identifier: String,
    pub password: String,
    pub slink: String,
    pub stay_logged_in: bool,
    pub show_error: bool,
}
