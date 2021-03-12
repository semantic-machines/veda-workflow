use crate::Context;
use serde_json::json;

use camunda_client::models::{CompleteTaskDto, VariableValueDto};
use std::collections::HashMap;
use std::error::Error;
use v_module::module::Module;
use v_module::v_api::IndvOp;
use v_module::v_onto::individual::Individual;

pub fn prepare_decision_form(decision_form: &mut Individual, ctx: &mut Context, module: &mut Module, _signal: &str) -> Result<(), Box<dyn Error>> {
    if let Some(decision) = decision_form.get_first_literal("v-wf:takenDecision") {
        if let Some(task_id) = decision_form.get_first_literal("bpmn:taskId") {
            let mut vars = HashMap::new();
            let mut var = VariableValueDto::new();
            var.value = Some(json!(decision));
            var._type = Some("json".to_owned());
            vars.insert("takenDecision".to_owned(), var);
            let mut params = CompleteTaskDto::new();
            params.variables = Some(vars);

            match ctx.camunda_client.task_api().complete(&task_id, Some(params)) {
                Ok(_) => {
                    decision_form.parse_all();
                    module.api.update_or_err(&ctx.sys_ticket, "", "prepare-decision-process", IndvOp::Put, &decision_form)?;
                }
                Err(e) => {
                    error!("failed to send task complete, err={:?}", e)
                }
            }
        }
    }

    Ok(())
}
