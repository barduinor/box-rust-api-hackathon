/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RetentionPoliciesAllOf1 : The part of an API response that describes marker based pagination



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetentionPoliciesAllOf1 {
    /// The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. The maximum value varies by API.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The marker for the start of the next page of results.
    #[serde(rename = "next_marker", skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl RetentionPoliciesAllOf1 {
    /// The part of an API response that describes marker based pagination
    pub fn new() -> RetentionPoliciesAllOf1 {
        RetentionPoliciesAllOf1 {
            limit: None,
            next_marker: None,
        }
    }
}


