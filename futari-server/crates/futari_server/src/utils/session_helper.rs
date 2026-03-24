use crate::service::auth::session_types::SessionContext;
use futari_errors::errors::Errors;
use sea_orm::prelude::IpNetwork;
use uuid::Uuid;

///
/// # Returns
///
///
/// # Errors
pub fn extract_user_or_ip(
    session: Option<&SessionContext>,
    ip_address: &str,
) -> Result<(Option<Uuid>, Option<IpNetwork>), Errors> {
    let ip = ip_address
        .parse::<IpNetwork>()
        .map_err(|_| Errors::InvalidIpAddress)?;

    match session {
        Some(s) => Ok((Some(s.user_id), Some(ip))),
        None => Ok((None, Some(ip))),
    }
}
