/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Collaboration : Collaborations define access permissions for users and groups to files and folders, similar to access control lists. A collaboration object grants a user or group access to a file or folder with permissions defined by a specific role.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Collaboration {
    /// The unique identifier for this collaboration.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `collaboration`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "item", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item: Option<Option<Box<crate::models::CollaborationItem>>>,
    #[serde(rename = "accessible_by", skip_serializing_if = "Option::is_none")]
    pub accessible_by: Option<Box<crate::models::CollaborationAccessibleBy>>,
    /// The email address used to invite an unregistered collaborator, if they are not a registered user.
    #[serde(rename = "invite_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invite_email: Option<Option<String>>,
    /// The level of access granted.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// When the collaboration will expire, or `null` if no expiration date is set.
    #[serde(rename = "expires_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<String>>,
    /// The status of the collaboration invitation. If the status is `pending`, `login` and `name` return an empty string.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// When the `status` of the collaboration object changed to `accepted` or `rejected`.
    #[serde(rename = "acknowledged_at", skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::CollaborationCreatedBy>>,
    /// When the collaboration object was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the collaboration object was last modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "acceptance_requirements_status", skip_serializing_if = "Option::is_none")]
    pub acceptance_requirements_status: Option<Box<crate::models::CollaborationAcceptanceRequirementsStatus>>,
}

impl Collaboration {
    /// Collaborations define access permissions for users and groups to files and folders, similar to access control lists. A collaboration object grants a user or group access to a file or folder with permissions defined by a specific role.
    pub fn new() -> Collaboration {
        Collaboration {
            id: None,
            r#type: None,
            item: None,
            accessible_by: None,
            invite_email: None,
            role: None,
            expires_at: None,
            status: None,
            acknowledged_at: None,
            created_by: None,
            created_at: None,
            modified_at: None,
            acceptance_requirements_status: None,
        }
    }
}

/// `collaboration`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "collaboration")]
    Collaboration,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Collaboration
    }
}
/// The level of access granted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "viewer")]
    Viewer,
    #[serde(rename = "previewer")]
    Previewer,
    #[serde(rename = "uploader")]
    Uploader,
    #[serde(rename = "previewer uploader")]
    PreviewerUploader,
    #[serde(rename = "viewer uploader")]
    ViewerUploader,
    #[serde(rename = "co-owner")]
    CoOwner,
    #[serde(rename = "owner")]
    Owner,
}

impl Default for Role {
    fn default() -> Role {
        Self::Editor
    }
}
/// The status of the collaboration invitation. If the status is `pending`, `login` and `name` return an empty string.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for Status {
    fn default() -> Status {
        Self::Accepted
    }
}
