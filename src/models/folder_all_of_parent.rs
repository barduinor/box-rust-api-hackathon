/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderAllOfParent {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`.
    #[serde(rename = "id")]
    pub id: String,
    /// The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// `folder`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The name of the folder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource.
    #[serde(rename = "sequence_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Option<String>>,
}

impl FolderAllOfParent {
    pub fn new(id: String, r#type: RHashType) -> FolderAllOfParent {
        FolderAllOfParent {
            id,
            etag: None,
            r#type,
            name: None,
            sequence_id: None,
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

