use crate::QueueElement;
use v_camunda_common::scripts::{Context, ScriptInfoContext};
use v_module::module::PrepareError;
use v_module::v_api::app::ResultCode;
use v_v8::callback::*;
use v_v8::common::ScriptInfo;
use v_v8::rusty_v8 as v8;
use v_v8::session_cache::CallbackSharedData;
use v_v8::session_cache::{commit, Transaction};

pub fn execute_js(qel: &QueueElement, session_data: CallbackSharedData, ctx: &mut Context) -> Result<i64, PrepareError> {
    let mut sh_g_vars = G_VARS.lock().unwrap();
    let g_vars = sh_g_vars.get_mut();
    *g_vars = session_data;
    drop(sh_g_vars);

    let hs = v8::ContextScope::new(&mut ctx.workplace.scope, ctx.workplace.context);
    let mut local_scope = hs;

    for script_id in ctx.workplace.scripts_order.iter() {
        if let Some(script) = ctx.workplace.scripts.get(script_id) {
            if !check_filters(script, qel) {
                continue;
            }
            if let Some(compiled_script) = script.compiled_script {
                let mut sh_tnx = G_TRANSACTION.lock().unwrap();
                let tnx = sh_tnx.get_mut();
                *tnx = Transaction::default();
                tnx.id = 0;
                //tnx.event_id = run_script_id.to_owned() + ";" + &event_id;
                tnx.sys_ticket = ctx.sys_ticket.to_owned();
                drop(sh_tnx);

                compiled_script.run(&mut local_scope);
                ctx.count_exec += 1;

                sh_tnx = G_TRANSACTION.lock().unwrap();
                let tnx = sh_tnx.get_mut();

                let res = commit(tnx, &mut ctx.veda_client);

                for item in tnx.queue.iter() {
                    info!("tnx item: cmd={:?}, uri={}, res={:?}", item.cmd, item.indv.get_id(), item.rc);
                }

                drop(sh_tnx);

                info!("{}, {}", ctx.count_exec, script_id);

                if res != ResultCode::Ok {
                    info!("fail exec event script : {}, result={:?}", script_id, res);
                    return Err(PrepareError::Fatal);
                }
            }
        }
    }

    Ok(0)
}

pub fn check_filters(script: &ScriptInfo<ScriptInfoContext>, qel: &QueueElement) -> bool {
    if !script.context.script_type.hash.contains(&qel.rtype) {
        return false;
    }
    if let Some(h) = &script.context.trigger_by_event {
        if !h.hash.contains(&qel.event) {
            return false;
        }
    }
    if let Some(h) = &script.context.trigger_by_process_definition_key {
        if !h.hash.contains(&qel.process_definition_key) {
            return false;
        }
    }
    if let Some(h) = &script.context.trigger_by_element_type {
        if !h.hash.contains(&qel.element_type) {
            return false;
        }
    }
    if let Some(h) = &script.context.trigger_by_element_id {
        if !h.hash.contains(&qel.element_id) {
            return false;
        }
    }
    true
}
