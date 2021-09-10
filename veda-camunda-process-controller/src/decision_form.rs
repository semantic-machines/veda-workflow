use crate::common::{set_err, CVI_USER_NAME};
use crate::Context;
use camunda_client::models::{CompleteTaskDto, VariableValueDto};
use serde_json::json;
use std::collections::HashMap;
use v_common::module::veda_backend::Backend;
use v_common::onto::individual::Individual;
use v_common::v_api::api_client::{ApiError, IndvOp};

pub fn prepare_decision_form(decision_form: &mut Individual, ctx: &mut Context, module: &mut Backend, _signal: &str) -> Result<(), ApiError> {
    if let Some(decision_id) = decision_form.get_first_literal("v-wf:takenDecision") {
        if let Some(task_id) = decision_form.get_first_literal("bpmn:taskId") {
            let mut vars = HashMap::new();
            let mut var = VariableValueDto::new();
            if let Some(decision) = module.get_individual(&decision_id, &mut Individual::default()) {
                decision.parse_all();
                var.value = Some(json!(decision.get_obj().as_json().to_string()));
                var._type = Some("json".to_owned());
                vars.insert("takenDecision".to_owned(), var);
                let mut params = CompleteTaskDto::new();
                params.variables = Some(vars);
                params.with_variables_in_return = Some(true);

                match ctx.camunda_client.task_api().complete(&task_id, Some(params)) {
                    Ok(_) => {
                        decision_form.parse_all();
                        decision_form.set_bool("v-wf:isCompleted", true);
                        decision_form.set_uri("v-s:lastEditor", CVI_USER_NAME);
                        module.mstorage_api.update_or_err(&ctx.sys_ticket, "", "prepare-decision-process", IndvOp::Put, &decision_form)?;
                        info!("prepare_decision_form: success send task complete");
                    }
                    Err(e) => {
                        error!("prepare_decision_form: failed to send task complete, err={:?}", e);
                        set_err(module, &ctx.sys_ticket, decision_form, &format!("{:?}", e));
                    }
                }
            } else {
                error!("prepare_decision_form: not found {}", decision_id);
            }
        } else {
            error!("prepare_decision_form: {} not content bpmn:taskId", decision_form.get_id());
        }
    }

    Ok(())
}
