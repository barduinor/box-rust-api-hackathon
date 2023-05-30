/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostTaskAssignmentsRequestAssignTo : The user to assign the task to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostTaskAssignmentsRequestAssignTo {
    /// The ID of the user to assign to the task.  To specify a user by their email address use the `login` parameter.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The email address of the user to assign to the task. To specify a user by their user ID please use the `id` parameter.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
}

impl PostTaskAssignmentsRequestAssignTo {
    /// The user to assign the task to.
    pub fn new() -> PostTaskAssignmentsRequestAssignTo {
        PostTaskAssignmentsRequestAssignTo {
            id: None,
            login: None,
        }
    }
}


