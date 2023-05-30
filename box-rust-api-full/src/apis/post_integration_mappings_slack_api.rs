/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`post_integration_mappings_slack`]
#[derive(Clone, Debug, Default)]
pub struct PostIntegrationMappingsSlackParams {
    pub integration_mapping_slack_create_request: Option<crate::models::IntegrationMappingSlackCreateRequest>
}


/// struct for typed errors of method [`post_integration_mappings_slack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostIntegrationMappingsSlackError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Creates a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack) by mapping a Slack channel to a Box item.  You need Admin or Co-Admin role to use this endpoint.
pub async fn post_integration_mappings_slack(configuration: &configuration::Configuration, params: PostIntegrationMappingsSlackParams) -> Result<crate::models::IntegrationMapping, Error<PostIntegrationMappingsSlackError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_mapping_slack_create_request = params.integration_mapping_slack_create_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/integration_mappings/slack", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&integration_mapping_slack_create_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostIntegrationMappingsSlackError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

