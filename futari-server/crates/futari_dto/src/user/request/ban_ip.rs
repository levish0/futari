use crate::validator::datetime_validator::validate_future_datetime;
use crate::validator::string_validator::validate_not_blank;
use chrono::{DateTime, Utc};
use sea_orm::prelude::IpNetwork;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
/// Request payload for IP ban request.
pub struct BanIpRequest {
    /// IP address or CIDR range (e.g., "192.168.1.5" or "192.168.1.0/24")
    #[schema(value_type = String, example = "192.168.1.0/24")]
    pub ip_address: IpNetwork,
    /// Ban expiration time (None = permanent ban)
    #[validate(custom(function = "validate_future_datetime"))]
    pub expires_at: Option<DateTime<Utc>>,
    #[validate(length(
        min = 1,
        max = 1000,
        message = "Reason must be between 1 and 1000 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub reason: String,
}
