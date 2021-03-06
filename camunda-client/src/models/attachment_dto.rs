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
pub struct AttachmentDto {
    /// The id of the task attachment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the task attachment.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the task attachment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The id of the task to which the attachment belongs.
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Indication of the type of content that this attachment refers to. Can be MIME type or any other indication.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The url to the remote content of the task attachment.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The time the variable was inserted. [Default format](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/) `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// The time after which the attachment should be removed by the History Cleanup job. [Default format](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/) `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "removalTime", skip_serializing_if = "Option::is_none")]
    pub removal_time: Option<String>,
    /// The process instance id of the root process instance that initiated the process containing the task.
    #[serde(rename = "rootProcessInstanceId", skip_serializing_if = "Option::is_none")]
    pub root_process_instance_id: Option<String>,
    /// The links associated to this resource, with `method`, `href` and `rel`.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::AtomLink>>,
}

impl AttachmentDto {
    pub fn new() -> AttachmentDto {
        AttachmentDto {
            id: None,
            name: None,
            description: None,
            task_id: None,
            _type: None,
            url: None,
            create_time: None,
            removal_time: None,
            root_process_instance_id: None,
            links: None,
        }
    }
}


