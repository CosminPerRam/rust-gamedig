use crate::{GDError, GDResult};
use crate::protocols::minecraft::{LegacyVersion, Response, Server};
use crate::protocols::minecraft::protocol::java::Java;
use crate::protocols::minecraft::protocol::legacy_v1_4::LegacyV1_4;
use crate::protocols::minecraft::protocol::legacy_v1_6::LegacyV1_6;
use crate::protocols::minecraft::protocol::legacy_bv1_8::LegacyBV1_8;
use crate::protocols::types::TimeoutSettings;

mod java;
mod legacy_v1_4;
mod legacy_v1_6;
mod legacy_bv1_8;

pub fn query(address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    if let Ok(response) = query_specific(Server::Java, address, port, timeout_settings.clone()) {
        return Ok(response);
    }

    if let Ok(response) = query_specific(Server::Legacy(LegacyVersion::V1_6), address, port, timeout_settings.clone()) {
        return Ok(response);
    }

    if let Ok(response) = query_specific(Server::Legacy(LegacyVersion::V1_4), address, port, timeout_settings.clone()) {
        return Ok(response);
    }

    if let Ok(response) = query_specific(Server::Legacy(LegacyVersion::BV1_8), address, port, timeout_settings.clone()) {
        return Ok(response);
    }

    Err(GDError::AutoQuery("No protocol returned a response.".to_string()))
}

pub fn query_specific(mc_type: Server, address: &str, port: u16, timeout_settings: Option<TimeoutSettings>) -> GDResult<Response> {
    match mc_type {
        Server::Java => Java::query(address, port, timeout_settings),
        Server::Legacy(category) => match category {
            LegacyVersion::V1_6 => LegacyV1_6::query(address, port, timeout_settings),
            LegacyVersion::V1_4 => LegacyV1_4::query(address, port, timeout_settings),
            LegacyVersion::BV1_8 => LegacyBV1_8::query(address, port, timeout_settings),
        }
    }
}
