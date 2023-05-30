/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RecentItem : A recent item accessed by a user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecentItem {
    /// `recent_item`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::RecentItemItem>>,
    /// The most recent type of access the user performed on the item.
    #[serde(rename = "interaction_type", skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<InteractionType>,
    /// The time of the most recent interaction.
    #[serde(rename = "interacted_at", skip_serializing_if = "Option::is_none")]
    pub interacted_at: Option<String>,
    /// If the item was accessed through a shared link it will appear here, otherwise this will be null.
    #[serde(rename = "interaction_shared_link", skip_serializing_if = "Option::is_none")]
    pub interaction_shared_link: Option<String>,
}

impl RecentItem {
    /// A recent item accessed by a user.
    pub fn new() -> RecentItem {
        RecentItem {
            r#type: None,
            item: None,
            interaction_type: None,
            interacted_at: None,
            interaction_shared_link: None,
        }
    }
}

/// The most recent type of access the user performed on the item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    #[serde(rename = "item_preview")]
    Preview,
    #[serde(rename = "item_upload")]
    Upload,
    #[serde(rename = "item_comment")]
    Comment,
    #[serde(rename = "item_open")]
    Open,
    #[serde(rename = "item_modify")]
    Modify,
}

impl Default for InteractionType {
    fn default() -> InteractionType {
        Self::Preview
    }
}

