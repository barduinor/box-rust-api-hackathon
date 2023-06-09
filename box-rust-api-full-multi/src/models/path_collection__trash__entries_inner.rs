/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PathCollectionTrashEntriesInner : The parent folder for this item



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PathCollectionTrashEntriesInner {
    /// `folder`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The unique identifier that represent a folder.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// This field is null for the Trash folder
    #[serde(rename = "sequence_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Option<String>>,
    /// This field is null for the Trash folder
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// The name of the Trash folder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PathCollectionTrashEntriesInner {
    /// The parent folder for this item
    pub fn new() -> PathCollectionTrashEntriesInner {
        PathCollectionTrashEntriesInner {
            r#type: None,
            id: None,
            sequence_id: None,
            etag: None,
            name: None,
        }
    }
}

/// `folder`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "folder")]
    Folder,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Folder
    }
}
