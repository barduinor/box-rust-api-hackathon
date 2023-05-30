/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationMapping : A standard representation of an integration mapping object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegrationMapping {
    /// A unique identifier of a folder mapping (part of a composite key together with `integration_type`)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`)
    #[serde(rename = "integration_type", skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,
    /// Mapping type
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "partner_item")]
    pub partner_item: Box<crate::models::IntegrationMappingAllOfPartnerItem>,
    #[serde(rename = "box_item")]
    pub box_item: Box<crate::models::IntegrationMappingAllOfBoxItem>,
    /// Identifies whether the mapping has been manually set (as opposed to being automatically created)
    #[serde(rename = "is_manually_created", skip_serializing_if = "Option::is_none")]
    pub is_manually_created: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::IntegrationMappingAllOfOptions>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::IntegrationMappingAllOfCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::IntegrationMappingAllOfModifiedBy>>,
    /// When the integration mapping object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the integration mapping object was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl IntegrationMapping {
    /// A standard representation of an integration mapping object.
    pub fn new(r#type: RHashType, partner_item: crate::models::IntegrationMappingAllOfPartnerItem, box_item: crate::models::IntegrationMappingAllOfBoxItem) -> IntegrationMapping {
        IntegrationMapping {
            id: None,
            integration_type: None,
            r#type,
            partner_item: Box::new(partner_item),
            box_item: Box::new(box_item),
            is_manually_created: None,
            options: None,
            created_by: None,
            modified_by: None,
            created_at: None,
            modified_at: None,
        }
    }
}

/// Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntegrationType {
    #[serde(rename = "slack")]
    Slack,
}

impl Default for IntegrationType {
    fn default() -> IntegrationType {
        Self::Slack
    }
}
/// Mapping type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "integration_mapping")]
    IntegrationMapping,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::IntegrationMapping
    }
}
