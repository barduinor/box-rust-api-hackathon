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

/// struct for passing parameters to the method [`delete_folders_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFoldersIdParams {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`.
    pub folder_id: String,
    /// Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since.
    pub if_match: Option<String>,
    /// Delete a folder that is not empty by recursively deleting the folder and all of its content.
    pub recursive: Option<bool>
}

/// struct for passing parameters to the method [`get_folders_id`]
#[derive(Clone, Debug, Default)]
pub struct GetFoldersIdParams {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`.
    pub folder_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.  Additionally this field can be used to query any metadata applied to the file by specifying the `metadata` field as well as the scope and key of the template to retrieve, for example `?field=metadata.enterprise_12345.contractTemplate`.
    pub fields: Option<Vec<String>>,
    /// Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since.
    pub if_none_match: Option<String>,
    /// The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item.
    pub boxapi: Option<String>
}

/// struct for passing parameters to the method [`get_folders_id_items`]
#[derive(Clone, Debug, Default)]
pub struct GetFoldersIdItemsParams {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`.
    pub folder_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.  Additionally this field can be used to query any metadata applied to the file by specifying the `metadata` field as well as the scope and key of the template to retrieve, for example `?field=metadata.enterprise_12345.contractTemplate`.
    pub fields: Option<Vec<String>>,
    /// Specifies whether to use marker-based pagination instead of offset-based pagination. Only one pagination method can be used at a time.  By setting this value to true, the API will return a `marker` field that can be passed as a parameter to this endpoint to get the next page of the response.
    pub usemarker: Option<bool>,
    /// Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`.
    pub marker: Option<String>,
    /// The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response.
    pub offset: Option<i64>,
    /// The maximum number of items to return per page.
    pub limit: Option<i64>,
    /// The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item.
    pub boxapi: Option<String>,
    /// Defines the **second** attribute by which items are sorted.  Items are always sorted by their `type` first, with folders listed before files, and files listed before web links.  This parameter is not supported for marker-based pagination on the root folder (the folder with an ID of `0`).
    pub sort: Option<String>,
    /// The direction to sort results in. This can be either in alphabetical ascending (`ASC`) or descending (`DESC`) order.
    pub direction: Option<String>
}

/// struct for passing parameters to the method [`post_folders`]
#[derive(Clone, Debug, Default)]
pub struct PostFoldersParams {
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    pub post_folders_request: Option<crate::models::PostFoldersRequest>
}

/// struct for passing parameters to the method [`post_folders_id_copy`]
#[derive(Clone, Debug, Default)]
pub struct PostFoldersIdCopyParams {
    /// The unique identifier of the folder to copy.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder with the ID `0` can not be copied.
    pub folder_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    pub post_folders_id_copy_request: Option<crate::models::PostFoldersIdCopyRequest>
}

/// struct for passing parameters to the method [`put_folders_id`]
#[derive(Clone, Debug, Default)]
pub struct PutFoldersIdParams {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`.
    pub folder_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    /// Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since.
    pub if_match: Option<String>,
    pub put_folders_id_request: Option<crate::models::PutFoldersIdRequest>
}


/// struct for typed errors of method [`delete_folders_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFoldersIdError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    Status412(crate::models::ClientError),
    Status503(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_folders_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFoldersIdError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_folders_id_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFoldersIdItemsError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_folders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostFoldersError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_folders_id_copy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostFoldersIdCopyError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    Status500(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_folders_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutFoldersIdError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    Status412(crate::models::ClientError),
    Status503(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a folder, either permanently or by moving it to the trash.
pub async fn delete_folders_id(configuration: &configuration::Configuration, params: DeleteFoldersIdParams) -> Result<(), Error<DeleteFoldersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let if_match = params.if_match;
    let recursive = params.recursive;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_match {
        local_var_req_builder = local_var_req_builder.header("if-match", local_var_param_value.to_string());
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
        let local_var_entity: Option<DeleteFoldersIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves details for a folder, including the first 100 entries in the folder.  To fetch more items within the folder, please use the [Get items in a folder](#get-folders-id-items) endpoint.
pub async fn get_folders_id(configuration: &configuration::Configuration, params: GetFoldersIdParams) -> Result<crate::models::FolderFull, Error<GetFoldersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let fields = params.fields;
    let if_none_match = params.if_none_match;
    let boxapi = params.boxapi;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("if-none-match", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = boxapi {
        local_var_req_builder = local_var_req_builder.header("boxapi", local_var_param_value.to_string());
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
        let local_var_entity: Option<GetFoldersIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a page of items in a folder. These items can be files, folders, and web links.  To request more information about the folder itself, like its size, please use the [Get a folder](#get-folders-id) endpoint instead.
pub async fn get_folders_id_items(configuration: &configuration::Configuration, params: GetFoldersIdItemsParams) -> Result<crate::models::Items, Error<GetFoldersIdItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let fields = params.fields;
    let usemarker = params.usemarker;
    let marker = params.marker;
    let offset = params.offset;
    let limit = params.limit;
    let boxapi = params.boxapi;
    let sort = params.sort;
    let direction = params.direction;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/items", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = usemarker {
        local_var_req_builder = local_var_req_builder.query(&[("usemarker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = direction {
        local_var_req_builder = local_var_req_builder.query(&[("direction", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = boxapi {
        local_var_req_builder = local_var_req_builder.header("boxapi", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        // println!("local_var:\n{}\n", local_var_content);
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFoldersIdItemsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new empty folder within the specified parent folder.
pub async fn post_folders(configuration: &configuration::Configuration, params: PostFoldersParams) -> Result<crate::models::FolderFull, Error<PostFoldersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fields = params.fields;
    let post_folders_request = params.post_folders_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_folders_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostFoldersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a copy of a folder within a destination folder.  The original folder will not be changed.
pub async fn post_folders_id_copy(configuration: &configuration::Configuration, params: PostFoldersIdCopyParams) -> Result<crate::models::FolderFull, Error<PostFoldersIdCopyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let fields = params.fields;
    let post_folders_id_copy_request = params.post_folders_id_copy_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/copy", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_folders_id_copy_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostFoldersIdCopyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a folder. This can be also be used to move the folder, create shared links, update collaborations, and more.
pub async fn put_folders_id(configuration: &configuration::Configuration, params: PutFoldersIdParams) -> Result<crate::models::FolderFull, Error<PutFoldersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let fields = params.fields;
    let if_match = params.if_match;
    let put_folders_id_request = params.put_folders_id_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_match {
        local_var_req_builder = local_var_req_builder.header("if-match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_folders_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutFoldersIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
