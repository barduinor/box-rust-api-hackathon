/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ZipDownloadRequestItemsInner : An item to add to the `zip` archive. This can be a file or a folder.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ZipDownloadRequestItemsInner {
    /// The type of the item to add to the archive.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The identifier of the item to add to the archive. When this item is a folder then this can not be the root folder with ID `0`.
    #[serde(rename = "id")]
    pub id: String,
}

impl ZipDownloadRequestItemsInner {
    /// An item to add to the `zip` archive. This can be a file or a folder.
    pub fn new(r#type: RHashType, id: String) -> ZipDownloadRequestItemsInner {
        ZipDownloadRequestItemsInner {
            r#type,
            id,
        }
    }
}

/// The type of the item to add to the archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "folder.")]
    FolderPeriod,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::File
    }
}

