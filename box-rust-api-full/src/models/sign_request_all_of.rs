/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignRequestAllOf {
    /// object type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// List of files to create a signing document from. This is currently limited to ten files. Only the ID and type fields are required for each file.
    #[serde(rename = "source_files", skip_serializing_if = "Option::is_none")]
    pub source_files: Option<Vec<crate::models::FileBase>>,
    /// Array of signers for the sign request
    #[serde(rename = "signers", skip_serializing_if = "Option::is_none")]
    pub signers: Option<Vec<crate::models::SignRequestSigner>>,
    /// Force a specific color for the signature (blue, black, or red).
    #[serde(rename = "signature_color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signature_color: Option<Option<String>>,
    /// Sign request ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This URL is returned if `is_document_preparation_needed` is set to `true` in the request. It is used to prepare the sign request via UI. The sign request is not sent until preparation is complete.
    #[serde(rename = "prepare_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prepare_url: Option<Option<String>>,
    #[serde(rename = "signing_log", skip_serializing_if = "Option::is_none")]
    pub signing_log: Option<Box<crate::models::SignRequestAllOfSigningLog>>,
    /// Describes the status of the sign request
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "sign_files", skip_serializing_if = "Option::is_none")]
    pub sign_files: Option<Box<crate::models::SignRequestAllOfSignFiles>>,
    /// Uses `days_valid` to calculate the date and time, in GMT, the sign request will expire if unsigned.
    #[serde(rename = "auto_expire_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auto_expire_at: Option<Option<String>>,
}

impl SignRequestAllOf {
    pub fn new() -> SignRequestAllOf {
        SignRequestAllOf {
            r#type: None,
            source_files: None,
            signers: None,
            signature_color: None,
            id: None,
            prepare_url: None,
            signing_log: None,
            status: None,
            sign_files: None,
            auto_expire_at: None,
        }
    }
}

/// object type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "sign-request")]
    SignRequest,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::SignRequest
    }
}
/// Describes the status of the sign request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "converting")]
    Converting,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "viewed")]
    Viewed,
    #[serde(rename = "signed")]
    Signed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "error_converting")]
    ErrorConverting,
    #[serde(rename = "error_sending")]
    ErrorSending,
    #[serde(rename = "expired")]
    Expired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Converting
    }
}

