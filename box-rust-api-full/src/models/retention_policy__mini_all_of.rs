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
pub struct RetentionPolicyMiniAllOf {
    /// The name given to the retention policy.
    #[serde(rename = "policy_name", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// The length of the retention policy. This value specifies the duration in days that the retention policy will be active for after being assigned to content.  If the policy has a `policy_type` of `indefinite`, the `retention_length` will also be `indefinite`.
    #[serde(rename = "retention_length", skip_serializing_if = "Option::is_none")]
    pub retention_length: Option<String>,
    /// The disposition action of the retention policy. This action can be `permanently_delete`, which will cause the content retained by the policy to be permanently deleted, or `remove_retention`, which will lift the retention policy from the content, allowing it to be deleted by users, once the retention policy has expired.
    #[serde(rename = "disposition_action", skip_serializing_if = "Option::is_none")]
    pub disposition_action: Option<DispositionAction>,
}

impl RetentionPolicyMiniAllOf {
    pub fn new() -> RetentionPolicyMiniAllOf {
        RetentionPolicyMiniAllOf {
            policy_name: None,
            retention_length: None,
            disposition_action: None,
        }
    }
}

/// The disposition action of the retention policy. This action can be `permanently_delete`, which will cause the content retained by the policy to be permanently deleted, or `remove_retention`, which will lift the retention policy from the content, allowing it to be deleted by users, once the retention policy has expired.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DispositionAction {
    #[serde(rename = "permanently_delete")]
    PermanentlyDelete,
    #[serde(rename = "remove_retention")]
    RemoveRetention,
}

impl Default for DispositionAction {
    fn default() -> DispositionAction {
        Self::PermanentlyDelete
    }
}

