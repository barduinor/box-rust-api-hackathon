/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RepresentationsEntriesInnerInfo : An object containing the URL that can be used to fetch more info on this representation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepresentationsEntriesInnerInfo {
    /// The API URL that can be used to get more info on this file representation. Make sure to make an authenticated API call to this endpoint.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RepresentationsEntriesInnerInfo {
    /// An object containing the URL that can be used to fetch more info on this representation.
    pub fn new() -> RepresentationsEntriesInnerInfo {
        RepresentationsEntriesInnerInfo {
            url: None,
        }
    }
}


