/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TaskAssignment : A task assignment defines which task is assigned to which user to complete.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskAssignment {
    /// The unique identifier for this task assignment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `task_assignment`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::TaskAssignmentItem>>,
    #[serde(rename = "assigned_to", skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Box<crate::models::TaskAssignmentAssignedTo>>,
    /// A message that will is included with the task assignment. This is visible to the assigned user in the web and mobile UI.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The date at which this task assignment was completed. This will be `null` if the task is not completed yet.
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// The date at which this task was assigned to the user.
    #[serde(rename = "assigned_at", skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<String>,
    /// The date at which the assigned user was reminded of this task assignment.
    #[serde(rename = "reminded_at", skip_serializing_if = "Option::is_none")]
    pub reminded_at: Option<String>,
    /// The current state of the assignment. The available states depend on the `action` value of the task object.
    #[serde(rename = "resolution_state", skip_serializing_if = "Option::is_none")]
    pub resolution_state: Option<ResolutionState>,
    #[serde(rename = "assigned_by", skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<Box<crate::models::TaskAssignmentAssignedBy>>,
}

impl TaskAssignment {
    /// A task assignment defines which task is assigned to which user to complete.
    pub fn new() -> TaskAssignment {
        TaskAssignment {
            id: None,
            r#type: None,
            item: None,
            assigned_to: None,
            message: None,
            completed_at: None,
            assigned_at: None,
            reminded_at: None,
            resolution_state: None,
            assigned_by: None,
        }
    }
}

/// `task_assignment`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "task_assignment")]
    TaskAssignment,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TaskAssignment
    }
}
/// The current state of the assignment. The available states depend on the `action` value of the task object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResolutionState {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for ResolutionState {
    fn default() -> ResolutionState {
        Self::Completed
    }
}
