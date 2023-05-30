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

/// struct for passing parameters to the method [`get_zip_downloads_id_content`]
#[derive(Clone, Debug, Default)]
pub struct GetZipDownloadsIdContentParams {
    /// The unique identifier that represent this `zip` archive.
    pub zip_download_id: String
}

/// struct for passing parameters to the method [`get_zip_downloads_id_status`]
#[derive(Clone, Debug, Default)]
pub struct GetZipDownloadsIdStatusParams {
    /// The unique identifier that represent this `zip` archive.
    pub zip_download_id: String
}

/// struct for passing parameters to the method [`post_zip_downloads`]
#[derive(Clone, Debug, Default)]
pub struct PostZipDownloadsParams {
    pub zip_download_request: Option<crate::models::ZipDownloadRequest>
}


/// struct for typed errors of method [`get_zip_downloads_id_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZipDownloadsIdContentError {
    Status404(crate::models::ClientError),
    Status429(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_zip_downloads_id_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZipDownloadsIdStatusError {
    Status401(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_zip_downloads`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostZipDownloadsError {
    Status400(crate::models::ClientError),
    Status401(crate::models::ClientError),
    Status403(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Returns the contents of a `zip` archive in binary format. This URL does not require any form of authentication and could be used in a user's browser to download the archive to a user's device.  By default, this URL is only valid for a few seconds from the creation of the request for this archive. Once a download has started it can not be stopped and resumed, instead a new request for a zip archive would need to be created.  The URL of this endpoint should not be considered as fixed. Instead, use the [Create zip download](e://post_zip_downloads) API to request to create a `zip` archive, and then follow the `download_url` field in the response to this endpoint.
pub async fn get_zip_downloads_id_content(configuration: &configuration::Configuration, params: GetZipDownloadsIdContentParams) -> Result<std::path::PathBuf, Error<GetZipDownloadsIdContentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let zip_download_id = params.zip_download_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/zip_downloads/{zip_download_id}/content", local_var_configuration.base_path, zip_download_id=crate::apis::urlencode(zip_download_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetZipDownloadsIdContentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the download status of a `zip` archive, allowing an application to inspect the progress of the download as well as the number of items that might have been skipped.  This endpoint can only be accessed once the download has started. Subsequently this endpoint is valid for 12 hours from the start of the download.  The URL of this endpoint should not be considered as fixed. Instead, use the [Create zip download](e://post_zip_downloads) API to request to create a `zip` archive, and then follow the `status_url` field in the response to this endpoint.
pub async fn get_zip_downloads_id_status(configuration: &configuration::Configuration, params: GetZipDownloadsIdStatusParams) -> Result<crate::models::ZipDownloadStatus, Error<GetZipDownloadsIdStatusError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let zip_download_id = params.zip_download_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/zip_downloads/{zip_download_id}/status", local_var_configuration.base_path, zip_download_id=crate::apis::urlencode(zip_download_id));
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
        let local_var_entity: Option<GetZipDownloadsIdStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a request to download multiple files and folders as a single `zip` archive file. This API does not return the archive but instead performs all the checks to ensure that the user has access to all the items, and then returns a `download_url` and a `status_url` that can be used to download the archive.  The limit for an archive is either the Account's upload limit or 10,000 files, whichever is met first
pub async fn post_zip_downloads(configuration: &configuration::Configuration, params: PostZipDownloadsParams) -> Result<crate::models::ZipDownload, Error<PostZipDownloadsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let zip_download_request = params.zip_download_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/zip_downloads", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&zip_download_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostZipDownloadsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

