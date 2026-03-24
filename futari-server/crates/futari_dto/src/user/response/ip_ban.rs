use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use sea_orm::prelude::IpNetwork;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
/// Response payload for IP ban.
pub struct IpBanResponse {
    pub id: Uuid,
    #[schema(value_type = String, example = "192.168.1.0/24")]
    pub ip_address: IpNetwork,
    pub expires_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl IntoResponse for IpBanResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
/// Response payload for listing active IP bans.
pub struct IpBanListResponse {
    pub data: Vec<IpBanResponse>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for IpBanListResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
/// Response payload for IP unban.
pub struct UnbanIpResponse {
    pub id: Uuid,
}

impl IntoResponse for UnbanIpResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
