/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SkillInvocationToken : The read-only and read-write access tokens for this item



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SkillInvocationToken {
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<Box<crate::models::SkillInvocationTokenRead>>,
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<Box<crate::models::SkillInvocationTokenRead>>,
}

impl SkillInvocationToken {
    /// The read-only and read-write access tokens for this item
    pub fn new() -> SkillInvocationToken {
        SkillInvocationToken {
            read: None,
            write: None,
        }
    }
}


