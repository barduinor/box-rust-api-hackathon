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


/// struct for typed errors of method [`delete_files_id_metadata_id_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFilesIdMetadataIdIdError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_files_id_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilesIdMetadataError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_files_id_metadata_id_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilesIdMetadataIdIdError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_files_id_metadata_id_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostFilesIdMetadataIdIdError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_files_id_metadata_id_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutFilesIdMetadataIdIdError {
    Status400(crate::models::ClientError),
    Status500(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a piece of file metadata.
pub async fn delete_files_id_metadata_id_id(configuration: &configuration::Configuration, file_id: &str, scope: &str, template_key: &str) -> Result<(), Error<DeleteFilesIdMetadataIdIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/metadata/{scope}/{template_key}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id), scope=crate::apis::urlencode(scope), template_key=crate::apis::urlencode(template_key));
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
        let local_var_entity: Option<DeleteFilesIdMetadataIdIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves all metadata for a given file.
pub async fn get_files_id_metadata(configuration: &configuration::Configuration, file_id: &str) -> Result<crate::models::Metadatas, Error<GetFilesIdMetadataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/metadata", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id));
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
        let local_var_entity: Option<GetFilesIdMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the instance of a metadata template that has been applied to a file.
pub async fn get_files_id_metadata_id_id(configuration: &configuration::Configuration, file_id: &str, scope: &str, template_key: &str) -> Result<crate::models::Metadata, Error<GetFilesIdMetadataIdIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/metadata/{scope}/{template_key}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id), scope=crate::apis::urlencode(scope), template_key=crate::apis::urlencode(template_key));
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
        let local_var_entity: Option<GetFilesIdMetadataIdIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Applies an instance of a metadata template to a file.  In most cases only values that are present in the metadata template will be accepted, except for the `global.properties` template which accepts any key-value pair.
pub async fn post_files_id_metadata_id_id(configuration: &configuration::Configuration, file_id: &str, scope: &str, template_key: &str, request_body: Option<::std::collections::HashMap<String, String>>) -> Result<crate::models::Metadata, Error<PostFilesIdMetadataIdIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/metadata/{scope}/{template_key}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id), scope=crate::apis::urlencode(scope), template_key=crate::apis::urlencode(template_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostFilesIdMetadataIdIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a piece of metadata on a file.  The metadata instance can only be updated if the template has already been applied to the file before. When editing metadata, only values that match the metadata template schema will be accepted.  The update is applied atomically. If any errors occur during the application of the operations, the metadata instance will not be changed.
pub async fn put_files_id_metadata_id_id(configuration: &configuration::Configuration, file_id: &str, scope: &str, template_key: &str, a_metadata_instance_update_operation: Option<Vec<crate::models::AMetadataInstanceUpdateOperation>>) -> Result<crate::models::Metadata, Error<PutFilesIdMetadataIdIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/metadata/{scope}/{template_key}", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id), scope=crate::apis::urlencode(scope), template_key=crate::apis::urlencode(template_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&a_metadata_instance_update_operation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutFilesIdMetadataIdIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
