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
pub struct WebhookAllOf {
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::WebhookAllOfCreatedBy>>,
    /// A timestamp identifying the time that the webhook was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The URL that is notified by this webhook
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// An array of event names that this webhook is to be triggered for
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<Triggers>>,
}

impl WebhookAllOf {
    pub fn new() -> WebhookAllOf {
        WebhookAllOf {
            created_by: None,
            created_at: None,
            address: None,
            triggers: None,
        }
    }
}

/// An array of event names that this webhook is to be triggered for
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Triggers {
    #[serde(rename = "FILE.UPLOADED")]
    FilePeriodUploaded,
    #[serde(rename = "FILE.PREVIEWED")]
    FilePeriodPreviewed,
    #[serde(rename = "FILE.DOWNLOADED")]
    FilePeriodDownloaded,
    #[serde(rename = "FILE.TRASHED")]
    FilePeriodTrashed,
    #[serde(rename = "FILE.DELETED")]
    FilePeriodDeleted,
    #[serde(rename = "FILE.RESTORED")]
    FilePeriodRestored,
    #[serde(rename = "FILE.COPIED")]
    FilePeriodCopied,
    #[serde(rename = "FILE.MOVED")]
    FilePeriodMoved,
    #[serde(rename = "FILE.LOCKED")]
    FilePeriodLocked,
    #[serde(rename = "FILE.UNLOCKED")]
    FilePeriodUnlocked,
    #[serde(rename = "FILE.RENAMED")]
    FilePeriodRenamed,
    #[serde(rename = "COMMENT.CREATED")]
    CommentPeriodCreated,
    #[serde(rename = "COMMENT.UPDATED")]
    CommentPeriodUpdated,
    #[serde(rename = "COMMENT.DELETED")]
    CommentPeriodDeleted,
    #[serde(rename = "TASK_ASSIGNMENT.CREATED")]
    TaskAssignmentPeriodCreated,
    #[serde(rename = "TASK_ASSIGNMENT.UPDATED")]
    TaskAssignmentPeriodUpdated,
    #[serde(rename = "METADATA_INSTANCE.CREATED")]
    MetadataInstancePeriodCreated,
    #[serde(rename = "METADATA_INSTANCE.UPDATED")]
    MetadataInstancePeriodUpdated,
    #[serde(rename = "METADATA_INSTANCE.DELETED")]
    MetadataInstancePeriodDeleted,
    #[serde(rename = "FOLDER.CREATED")]
    FolderPeriodCreated,
    #[serde(rename = "FOLDER.RENAMED")]
    FolderPeriodRenamed,
    #[serde(rename = "FOLDER.DOWNLOADED")]
    FolderPeriodDownloaded,
    #[serde(rename = "FOLDER.RESTORED")]
    FolderPeriodRestored,
    #[serde(rename = "FOLDER.DELETED")]
    FolderPeriodDeleted,
    #[serde(rename = "FOLDER.COPIED")]
    FolderPeriodCopied,
    #[serde(rename = "FOLDER.MOVED")]
    FolderPeriodMoved,
    #[serde(rename = "FOLDER.TRASHED")]
    FolderPeriodTrashed,
    #[serde(rename = "WEBHOOK.DELETED")]
    WebhookPeriodDeleted,
    #[serde(rename = "COLLABORATION.CREATED")]
    CollaborationPeriodCreated,
    #[serde(rename = "COLLABORATION.ACCEPTED")]
    CollaborationPeriodAccepted,
    #[serde(rename = "COLLABORATION.REJECTED")]
    CollaborationPeriodRejected,
    #[serde(rename = "COLLABORATION.REMOVED")]
    CollaborationPeriodRemoved,
    #[serde(rename = "COLLABORATION.UPDATED")]
    CollaborationPeriodUpdated,
    #[serde(rename = "SHARED_LINK.DELETED")]
    SharedLinkPeriodDeleted,
    #[serde(rename = "SHARED_LINK.CREATED")]
    SharedLinkPeriodCreated,
    #[serde(rename = "SHARED_LINK.UPDATED")]
    SharedLinkPeriodUpdated,
    #[serde(rename = "SIGN_REQUEST.COMPLETED")]
    SignRequestPeriodCompleted,
    #[serde(rename = "SIGN_REQUEST.DECLINED")]
    SignRequestPeriodDeclined,
    #[serde(rename = "SIGN_REQUEST.EXPIRED")]
    SignRequestPeriodExpired,
    #[serde(rename = "SIGN_REQUEST.SIGNER_EMAIL_BOUNCED")]
    SignRequestPeriodSignerEmailBounced,
}

impl Default for Triggers {
    fn default() -> Triggers {
        Self::FilePeriodUploaded
    }
}

