/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ConflictError : The error that occurs when a file can not be created due to a conflict.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConflictError {
    /// `error`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The HTTP status of the response.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// A Box-specific error code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    /// A short message describing the error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "context_info", skip_serializing_if = "Option::is_none")]
    pub context_info: Option<Box<crate::models::ConflictErrorAllOfContextInfo>>,
    /// A URL that links to more information about why this error occurred.
    #[serde(rename = "help_url", skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    /// A unique identifier for this response, which can be used when contacting Box support.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl ConflictError {
    /// The error that occurs when a file can not be created due to a conflict.
    pub fn new() -> ConflictError {
        ConflictError {
            r#type: None,
            status: None,
            code: None,
            message: None,
            context_info: None,
            help_url: None,
            request_id: None,
        }
    }
}

/// `error`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "error")]
    Error,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Error
    }
}
/// A Box-specific error code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "no_content")]
    NoContent,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "not_modified")]
    NotModified,
    #[serde(rename = "bad_request")]
    BadRequest,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "method_not_allowed")]
    MethodNotAllowed,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "precondition_failed")]
    PreconditionFailed,
    #[serde(rename = "too_many_requests")]
    TooManyRequests,
    #[serde(rename = "internal_server_error")]
    InternalServerError,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "item_name_invalid")]
    ItemNameInvalid,
    #[serde(rename = "insufficient_scope")]
    InsufficientScope,
}

impl Default for Code {
    fn default() -> Code {
        Self::Created
    }
}

