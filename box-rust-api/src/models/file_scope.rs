/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// FileScope : A relation between a file and the scopes for which the file can be accessed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileScope {
    /// The file scopes for the file access
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<crate::models::FileScopeObject>>,
}

impl FileScope {
    /// A relation between a file and the scopes for which the file can be accessed
    pub fn new() -> FileScope {
        FileScope {
            scope: None,
            object: None,
        }
    }
}

/// The file scopes for the file access
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "annotation_edit")]
    AnnotationEdit,
    #[serde(rename = "annotation_view_all")]
    AnnotationViewAll,
    #[serde(rename = "annotation_view_self")]
    AnnotationViewSelf,
    #[serde(rename = "base_explorer")]
    BaseExplorer,
    #[serde(rename = "base_picker")]
    BasePicker,
    #[serde(rename = "base_preview")]
    BasePreview,
    #[serde(rename = "base_upload")]
    BaseUpload,
    #[serde(rename = "item_delete")]
    ItemDelete,
    #[serde(rename = "item_download")]
    ItemDownload,
    #[serde(rename = "item_preview")]
    ItemPreview,
    #[serde(rename = "item_rename")]
    ItemRename,
    #[serde(rename = "item_share")]
    ItemShare,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::AnnotationEdit
    }
}

