use crate::common::{set_err, CVI_USER_NAME};
use crate::Context;
use std::error::Error;
use v_module::v_api::IndvOp;
use v_module::v_onto::individual::Individual;
use v_module::veda_backend::*;

pub fn prepare_stop_process(prepared_indv: &mut Individual, ctx: &mut Context, module: &mut Backend, _signal: &str) -> Result<(), Box<dyn Error>> {
    if let Some(process_instance_id) = prepared_indv.get_first_literal("bpmn:processInstanceId") {
        prepared_indv.parse_all();
        match ctx.camunda_client.process_instance_api().delete_process_instance(&process_instance_id, None, Some(true), Some(false), None) {
            Ok(_) => {
                prepared_indv.set_bool("v-wf:isCompleted", true);
                prepared_indv.set_uri("v-s:lastEditor", CVI_USER_NAME);
                module.api.update_or_err(&ctx.sys_ticket, "", "prepare-decision-process", IndvOp::Put, &prepared_indv)?;
                info!("stop_process: success send stop process instance, id={}", process_instance_id);
            }
            Err(e) => {
                error!("stop_process: failed to send delete_form, err={:?}", e);
                set_err(module, &ctx.sys_ticket, prepared_indv, &format!("{:?}", e));
            }
        }
    } else {
        error!("stop_process: {} not content bpmn:processInstanceId", prepared_indv.get_id());
    }
    Ok(())
}
