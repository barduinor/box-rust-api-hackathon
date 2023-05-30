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
pub struct FileVersionAllOfRestoredBy {
    /// The unique identifier for this user
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `user`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The display name of this user
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The primary email address of this user
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
}

impl FileVersionAllOfRestoredBy {
    pub fn new(r#type: RHashType) -> FileVersionAllOfRestoredBy {
        FileVersionAllOfRestoredBy {
            id: None,
            r#type,
            name: None,
            login: None,
        }
    }
}

/// `user`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}

