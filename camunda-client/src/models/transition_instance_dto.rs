/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransitionInstanceDto : A JSON object corresponding to the Activity Instance tree of the given process instance.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionInstanceDto {
    /// The id of the transition instance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the parent activity instance, for example a sub process instance.
    #[serde(rename = "parentActivityInstanceId", skip_serializing_if = "Option::is_none")]
    pub parent_activity_instance_id: Option<String>,
    /// The id of the activity that this instance enters (asyncBefore job) or leaves (asyncAfter job)
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The name of the activity that this instance enters (asyncBefore job) or leaves (asyncAfter job)
    #[serde(rename = "activityName", skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    /// The type of the activity that this instance enters (asyncBefore job) or leaves (asyncAfter job)
    #[serde(rename = "activityType", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    /// The id of the process instance this instance is part of.
    #[serde(rename = "processInstanceId", skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    /// The id of the process definition.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// The execution id.
    #[serde(rename = "executionId", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// A list of incident ids.
    #[serde(rename = "incidentIds", skip_serializing_if = "Option::is_none")]
    pub incident_ids: Option<Vec<String>>,
    /// A list of JSON objects containing incident specific properties: * `id`: the id of the incident * `activityId`: the activity id in which the incident occurred
    #[serde(rename = "incidents", skip_serializing_if = "Option::is_none")]
    pub incidents: Option<Vec<crate::models::ActivityInstanceIncidentDto>>,
}

impl TransitionInstanceDto {
    /// A JSON object corresponding to the Activity Instance tree of the given process instance.
    pub fn new() -> TransitionInstanceDto {
        TransitionInstanceDto {
            id: None,
            parent_activity_instance_id: None,
            activity_id: None,
            activity_name: None,
            activity_type: None,
            process_instance_id: None,
            process_definition_id: None,
            execution_id: None,
            incident_ids: None,
            incidents: None,
        }
    }
}


