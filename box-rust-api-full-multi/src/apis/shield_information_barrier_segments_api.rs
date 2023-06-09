/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`delete_shield_information_barrier_segments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteShieldInformationBarrierSegmentsIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shield_information_barrier_segments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShieldInformationBarrierSegmentsError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shield_information_barrier_segments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShieldInformationBarrierSegmentsIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_shield_information_barrier_segments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostShieldInformationBarrierSegmentsError {
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_shield_information_barrier_segments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutShieldInformationBarrierSegmentsIdError {
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes the shield information barrier segment based on provided ID.
pub async fn delete_shield_information_barrier_segments_id(configuration: &configuration::Configuration, shield_information_barrier_segment_id: &str) -> Result<(), Error<DeleteShieldInformationBarrierSegmentsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/shield_information_barrier_segments/{shield_information_barrier_segment_id}", local_var_configuration.base_path, shield_information_barrier_segment_id=crate::apis::urlencode(shield_information_barrier_segment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteShieldInformationBarrierSegmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a list of shield information barrier segment objects for the specified Information Barrier ID.
pub async fn get_shield_information_barrier_segments(configuration: &configuration::Configuration, shield_information_barrier_id: &str, marker: Option<&str>, limit: Option<i64>) -> Result<crate::models::GetShieldInformationBarrierSegments200Response, Error<GetShieldInformationBarrierSegmentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/shield_information_barrier_segments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("shield_information_barrier_id", &shield_information_barrier_id.to_string())]);
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetShieldInformationBarrierSegmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves shield information barrier segment based on provided ID..
pub async fn get_shield_information_barrier_segments_id(configuration: &configuration::Configuration, shield_information_barrier_segment_id: &str) -> Result<crate::models::ShieldInformationBarrierSegment, Error<GetShieldInformationBarrierSegmentsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/shield_information_barrier_segments/{shield_information_barrier_segment_id}", local_var_configuration.base_path, shield_information_barrier_segment_id=crate::apis::urlencode(shield_information_barrier_segment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetShieldInformationBarrierSegmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a shield information barrier segment.
pub async fn post_shield_information_barrier_segments(configuration: &configuration::Configuration, post_shield_information_barrier_segments_request: Option<crate::models::PostShieldInformationBarrierSegmentsRequest>) -> Result<crate::models::ShieldInformationBarrierSegment, Error<PostShieldInformationBarrierSegmentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/shield_information_barrier_segments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_shield_information_barrier_segments_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostShieldInformationBarrierSegmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the shield information barrier segment based on provided ID..
pub async fn put_shield_information_barrier_segments_id(configuration: &configuration::Configuration, shield_information_barrier_segment_id: &str, put_shield_information_barrier_segments_id_request: Option<crate::models::PutShieldInformationBarrierSegmentsIdRequest>) -> Result<crate::models::ShieldInformationBarrierSegment, Error<PutShieldInformationBarrierSegmentsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/shield_information_barrier_segments/{shield_information_barrier_segment_id}", local_var_configuration.base_path, shield_information_barrier_segment_id=crate::apis::urlencode(shield_information_barrier_segment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_shield_information_barrier_segments_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutShieldInformationBarrierSegmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
