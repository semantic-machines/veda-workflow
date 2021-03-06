use camunda_client::models::LockedExternalTaskDto;
use serde_json::json;
use v_camunda_common::scripts::{execute_js, Context, OutValue};
use v_v8::session_cache::CallbackSharedData;

pub fn execute_external_js_task(task: &LockedExternalTaskDto, script_id: &str, ctx: &mut Context, out: &mut OutValue) -> bool {
    let mut session_data = CallbackSharedData::default();
    session_data.g_key2attr.insert("$ticket".to_owned(), ctx.sys_ticket.to_owned());
    session_data.g_key2attr.insert("$task".to_owned(), json!(task).to_string());
    execute_js(session_data, script_id, ctx, out)
}

//pub fn get_script_identity(id: &str, text: &str) -> String {
//    format!("{}+{}", id, text)
//}
