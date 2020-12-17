/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActivityInstanceDto : A JSON object corresponding to the Activity Instance tree of the given process instance.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityInstanceDto {
    /// The id of the activity instance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the parent activity instance, for example a sub process instance.
    #[serde(rename = "parentActivityInstanceId", skip_serializing_if = "Option::is_none")]
    pub parent_activity_instance_id: Option<String>,
    /// The id of the activity.
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The name of the activity
    #[serde(rename = "activityName", skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    /// The type of activity (corresponds to the XML element name in the BPMN 2.0, e.g., 'userTask')
    #[serde(rename = "activityType", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    /// The id of the process instance this activity instance is part of.
    #[serde(rename = "processInstanceId", skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    /// The id of the process definition.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// A list of child activity instances.
    #[serde(rename = "childActivityInstances", skip_serializing_if = "Option::is_none")]
    pub child_activity_instances: Option<Vec<crate::models::ActivityInstanceDto>>,
    /// A list of child transition instances. A transition instance represents an execution waiting in an asynchronous continuation.
    #[serde(rename = "childTransitionInstances", skip_serializing_if = "Option::is_none")]
    pub child_transition_instances: Option<Vec<crate::models::TransitionInstanceDto>>,
    /// A list of execution ids.
    #[serde(rename = "executionIds", skip_serializing_if = "Option::is_none")]
    pub execution_ids: Option<Vec<String>>,
    /// A list of incident ids.
    #[serde(rename = "incidentIds", skip_serializing_if = "Option::is_none")]
    pub incident_ids: Option<Vec<String>>,
    /// A list of JSON objects containing incident specific properties: * `id`: the id of the incident * `activityId`: the activity id in which the incident occurred
    #[serde(rename = "incidents", skip_serializing_if = "Option::is_none")]
    pub incidents: Option<Vec<crate::models::ActivityInstanceIncidentDto>>,
}

impl ActivityInstanceDto {
    /// A JSON object corresponding to the Activity Instance tree of the given process instance.
    pub fn new() -> ActivityInstanceDto {
        ActivityInstanceDto {
            id: None,
            parent_activity_instance_id: None,
            activity_id: None,
            activity_name: None,
            activity_type: None,
            process_instance_id: None,
            process_definition_id: None,
            child_activity_instances: None,
            child_transition_instances: None,
            execution_ids: None,
            incident_ids: None,
            incidents: None,
        }
    }
}


