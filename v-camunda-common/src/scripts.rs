use camunda_client::apis::client::APIClient;
use serde_json::Value as JSONValue;
use std::sync::Mutex;
use v8::json::stringify;
use v_ft_xapian::xapian_reader::XapianReader;
use v_module::module::PrepareError;
use v_module::v_api::app::ResultCode;
use v_module::v_api::APIClient as VedaClient;
use v_module::v_onto::individual::Individual;
use v_module::v_search::common::FTQuery;
use v_v8::callback::*;
use v_v8::common::v8obj_into_individual;
use v_v8::common::HashVec;
use v_v8::common::ScriptInfo;
use v_v8::rusty_v8 as v8;
use v_v8::rusty_v8::ContextScope;
use v_v8::scripts_workplace::ScriptsWorkPlace;
use v_v8::session_cache::CallbackSharedData;
use v_v8::session_cache::{commit, Transaction};

pub struct ScriptInfoContext {
    pub trigger_by_event: HashVec<String>,
    pub script_type: HashVec<String>,
}

impl Default for ScriptInfoContext {
    fn default() -> Self {
        Self {
            trigger_by_event: Default::default(),
            script_type: Default::default()
        }
    }
}

lazy_static! {
    static ref INIT_LOCK: Mutex<u32> = Mutex::new(0);
}

pub struct Context<'a> {
    pub sys_ticket: String,
    //onto: Onto,
    pub xr: XapianReader,
    pub workplace: ScriptsWorkPlace<'a, ScriptInfoContext>,
    pub camunda_client: APIClient,
    pub veda_client: VedaClient,
    pub count_exec: i64,
}

pub enum OutValue {
    Json(JSONValue),
    Bool(bool),
    List(Vec<String>),
    Individual(Individual),
    None,
}

#[must_use]
pub struct SetupGuard {}

impl Drop for SetupGuard {
    fn drop(&mut self) {
        // TODO shutdown process cleanly.
    }
}

pub fn load_task_scripts(wp: &mut ScriptsWorkPlace<ScriptInfoContext>, xr: &mut XapianReader, task_type: &str, js_vars: &[&str]) {
    let res = xr.query(FTQuery::new_with_user("cfg:VedaSystem", &format!("'rdf:type' === '{}'", task_type)), &mut wp.module.storage);

    if res.result_code == ResultCode::Ok && res.count > 0 {
        for id in &res.result {
            if let Some(ev_indv) = wp.module.get_individual(id, &mut Individual::default()) {
                prepare_script(wp, ev_indv, js_vars);
            }
        }
    }
    info!("load scripts from db: {:?}", wp.scripts_order);
}

fn set_variables(js_vars: &[&str]) -> String {
    let mut out_str = String::new();

    for el in js_vars {
        if *el == "ticket" || el.ends_with("_id") || el.ends_with("Id") {
            out_str.push_str(&format!("var {} = get_env_str_var ('${}'); ", el, el));
        } else {
            out_str.push_str(&format!("var {} = JSON.parse(get_env_str_var ('${}')); ", el, el));
        }
    }
    out_str
}

pub(crate) fn prepare_script(wp: &mut ScriptsWorkPlace<ScriptInfoContext>, ev_indv: &mut Individual, js_vars: &[&str]) {
    if ev_indv.is_exists_bool("v-s:deleted", true) || ev_indv.is_exists_bool("v-s:disabled", true) {
        info!("disable script {}", ev_indv.get_id());
        if let Some(scr_inf) = wp.scripts.get_mut(ev_indv.get_id()) {
            scr_inf.compiled_script = None;
        }
        return;
    }

    if let Some(script_text) = ev_indv.get_first_literal("bpmn:script") {
        let str_script = "\
      (function () { \
        try { \
          "
        .to_owned()
            + &set_variables(js_vars)
            + &script_text
            + " \
         } catch (e) { log_trace (e.stack); } \
      })();";

        let id = if let Some(v) = ev_indv.get_first_literal("bpmn:triggerByTopic") {
            v
        } else {
            ev_indv.get_id().to_owned()
        };

        let mut scr_inf: ScriptInfo<ScriptInfoContext> = ScriptInfo::new_with_src(&id, &str_script);
        scr_inf.context.trigger_by_event = HashVec::new(ev_indv.get_literals("bpmn:triggerByEvent").unwrap_or_default());
        scr_inf.context.script_type = HashVec::new(ev_indv.get_literals("rdf:type").unwrap_or_default());

        wp.add_to_order(&scr_inf);

        let scope = &mut v8::ContextScope::new(&mut wp.scope, wp.context);

        scr_inf.compile_script(ev_indv.get_id(), scope);
        wp.scripts.insert(scr_inf.id.to_string(), scr_inf);
    } else {
        error!("v-s:script no found");
    }
}

pub fn execute_js_and_return_data(session_data: CallbackSharedData, script_id: &str, ctx: &mut Context, out: &mut OutValue) -> Result<bool, PrepareError> {
    let compiled_script = if let Some(script) = ctx.workplace.scripts.get(script_id) {
        script.compiled_script
    } else {
        None
    };

    if let Some(c) = compiled_script {
        let mut sh_g_vars = G_VARS.lock().unwrap();
        let g_vars = sh_g_vars.get_mut();
        *g_vars = session_data;
        drop(sh_g_vars);

        let hs = ContextScope::new(&mut ctx.workplace.scope, ctx.workplace.context);
        let mut local_scope = hs;

        let mut sh_tnx = G_TRANSACTION.lock().unwrap();
        let tnx = sh_tnx.get_mut();
        *tnx = Transaction::default();
        tnx.id = 0;
        tnx.sys_ticket = ctx.sys_ticket.to_owned();
        drop(sh_tnx);

        if let Some(res) = c.run(&mut local_scope) {
            match out {
                OutValue::Bool(ov) => {
                    if res.is_boolean() {
                        if res.to_integer(local_scope.as_mut()).unwrap().value() != 0 {
                            *ov = true;
                        } else {
                            *ov = false;
                        }
                    }
                }
                OutValue::Json(ov) => {
                    if let Some(jo) = stringify(&mut local_scope, res) {
                        let js_str = jo.to_rust_string_lossy(&mut local_scope);
                        if let Ok(v) = serde_json::from_str(&js_str) {
                            *ov = v;
                        }
                    }
                }
                OutValue::List(ov) => {
                    if let Some(obj) = res.to_object(&mut local_scope) {
                        if let Some(key_list) = obj.get_property_names(&mut local_scope) {
                            for resources_idx in 0..key_list.length() {
                                let j_resources_idx = v8::Integer::new(&mut local_scope, resources_idx as i32);
                                if let Some(v) = obj.get(&mut local_scope, j_resources_idx.into()) {
                                    if let Some(s) = v.to_string(&mut local_scope) {
                                        let ss = s.to_rust_string_lossy(&mut local_scope);
                                        ov.push(ss);
                                    }
                                }
                            }
                        }
                    }
                }
                OutValue::Individual(v) => {
                    if let Some(obj) = res.to_object(&mut local_scope) {
                        v8obj_into_individual(&mut local_scope, obj, v);
                    }
                }
                _ => {}
            }
        }
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
        return Ok(true);
    } else {
        return Ok(false);
    }
}
