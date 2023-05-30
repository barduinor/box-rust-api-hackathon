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
pub struct PutIntegrationMappingsSlackIdRequest {
    #[serde(rename = "box_item", skip_serializing_if = "Option::is_none")]
    pub box_item: Option<Box<crate::models::PutIntegrationMappingsSlackIdRequestBoxItem>>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::PutIntegrationMappingsSlackIdRequestOptions>>,
}

impl PutIntegrationMappingsSlackIdRequest {
    pub fn new() -> PutIntegrationMappingsSlackIdRequest {
        PutIntegrationMappingsSlackIdRequest {
            box_item: None,
            options: None,
        }
    }
}

