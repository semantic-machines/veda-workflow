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
pub struct ProcessDefinitionDiagramDto {
    /// The id of the process definition.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An escaped XML string containing the XML that this definition was deployed with. Carriage returns, line feeds and quotation marks are escaped.
    #[serde(rename = "bpmn20Xml", skip_serializing_if = "Option::is_none")]
    pub bpmn20_xml: Option<String>,
}

impl ProcessDefinitionDiagramDto {
    pub fn new() -> ProcessDefinitionDiagramDto {
        ProcessDefinitionDiagramDto {
            id: None,
            bpmn20_xml: None,
        }
    }
}


