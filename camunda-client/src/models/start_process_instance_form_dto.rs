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
pub struct StartProcessInstanceFormDto {
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
    /// The business key the process instance is to be initialized with. The business key uniquely identifies the process instance in the context of the given process definition.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
}

impl StartProcessInstanceFormDto {
    pub fn new() -> StartProcessInstanceFormDto {
        StartProcessInstanceFormDto {
            variables: None,
            business_key: None,
        }
    }
}


