/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetJobRetriesByProcessDto {
    /// A list of process instance ids to fetch jobs, for which retries will be set.
    #[serde(rename = "processInstances", skip_serializing_if = "Option::is_none")]
    pub process_instances: Option<Vec<String>>,
    /// An integer representing the number of retries. Please note that the value cannot be negative or null.
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "processInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub process_instance_query: Option<crate::models::ProcessInstanceQueryDto>,
    #[serde(rename = "historicProcessInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub historic_process_instance_query: Option<crate::models::HistoricProcessInstanceQueryDto>,
}

impl SetJobRetriesByProcessDto {
    pub fn new() -> SetJobRetriesByProcessDto {
        SetJobRetriesByProcessDto {
            process_instances: None,
            retries: None,
            process_instance_query: None,
            historic_process_instance_query: None,
        }
    }
}


