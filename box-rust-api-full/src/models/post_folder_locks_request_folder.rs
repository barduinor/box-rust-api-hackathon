/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostFolderLocksRequestFolder : The folder to apply the lock to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostFolderLocksRequestFolder {
    /// The content type the lock is being applied to. Only `folder` is supported.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The ID of the folder.
    #[serde(rename = "id")]
    pub id: String,
}

impl PostFolderLocksRequestFolder {
    /// The folder to apply the lock to.
    pub fn new(r#type: String, id: String) -> PostFolderLocksRequestFolder {
        PostFolderLocksRequestFolder {
            r#type,
            id,
        }
    }
}


