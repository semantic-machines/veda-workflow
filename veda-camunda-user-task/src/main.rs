#[macro_use]
extern crate log;
#[macro_use]
extern crate scan_fmt;

mod common;

use crate::common::execute_js;
use camunda_client::apis::client::APIClient;
use camunda_client::apis::configuration::Configuration;
use serde_json::json;
use std::{str, thread, time};
use v_camunda_common::scripts::{load_task_scripts, Context};
use v_ft_xapian::xapian_reader::XapianReader;
use v_module::common::load_onto;
use v_module::module::{init_log, Module, PrepareError};
use v_module::remote_indv_r_storage::*;
use v_module::v_api::APIClient as VedaClient;
use v_module::v_onto::individual::RawObj;
use v_module::v_onto::onto::Onto;
use v_queue::consumer::Consumer;
use v_v8::jsruntime::JsRuntime;
use v_v8::scripts_workplace::ScriptsWorkPlace;
use v_v8::session_cache::CallbackSharedData;

fn main() -> Result<(), i32> {
    init_log("CAMUNDA-USER-TASK");
    thread::spawn(move || inproc_storage_manager());

    let mut js_runtime = JsRuntime::new();
    listen_queue(&mut js_runtime)
}

fn listen_queue<'a>(js_runtime: &'a mut JsRuntime) -> Result<(), i32> {
    let mut module = Module::default();
    let sys_ticket;
    if let Ok(t) = module.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("failed to get system ticket");
        return Ok(());
    }

    let mut onto = Onto::default();

    info!("load onto start");
    load_onto(&mut module.storage, &mut onto);
    info!("load onto end");

    //wait_load_ontology();
    let mut queue_consumer = Consumer::new("./camunda/queue/camunda-events", "camunda_user_task", "camunda-event").expect("!!!!!!!!! FAIL QUEUE");

    //let configuration = Configuration::default();

    if let Some(xr) = XapianReader::new("russian", &mut module.storage) {
        let mut ctx = Context {
            workplace: ScriptsWorkPlace::new(js_runtime.v8_isolate()),
            //onto,
            sys_ticket,
            xr,
            camunda_client: APIClient::new(Configuration::default()),
            veda_client: VedaClient::new(Module::get_property("main_module_url").unwrap_or_default()),
            count_exec: 0,
        };
        ctx.workplace.load_ext_scripts(&ctx.sys_ticket);

        load_task_scripts(&mut ctx.workplace, &mut ctx.xr, "bpmn:UserTaskHandler", &["ticket", "task", "variables", "form_variables"]);
        load_task_scripts(&mut ctx.workplace, &mut ctx.xr, "bpmn:ProcessInstanceHandler", &["ticket", "processInstanceId", "processInstance"]);

        module.listen_queue_raw(
            &mut queue_consumer,
            &mut ctx,
            &mut (before_batch as fn(&mut Module, &mut Context<'a>, batch_size: u32) -> Option<u32>),
            &mut (prepare as fn(&mut Module, &mut Context<'a>, &RawObj, my_consumer: &Consumer) -> Result<bool, PrepareError>),
            &mut (after_batch as fn(&mut Module, &mut Context<'a>, prepared_batch_size: u32) -> Result<bool, PrepareError>),
            &mut (heartbeat as fn(&mut Module, &mut Context<'a>) -> Result<(), PrepareError>),
        );
    }
    Ok(())
}

fn heartbeat<'a>(_module: &mut Module, _ctx: &mut Context<'a>) -> Result<(), PrepareError> {
    Ok(())
}

fn before_batch<'a>(_module: &mut Module, _ctx: &mut Context<'a>, _size_batch: u32) -> Option<u32> {
    None
}

fn after_batch<'a>(_module: &mut Module, _ctx: &mut Context<'a>, _prepared_batch_size: u32) -> Result<bool, PrepareError> {
    Ok(false)
}

fn prepare<'a>(module: &mut Module, ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    match prepare_and_err(module, ctx, queue_element, _my_consumer) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("prepare err={:?}", e);
            Err(PrepareError::Recoverable)
        }
    }
}

fn prepare_and_err<'a>(_module: &mut Module, ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    if let Ok(el_str) = str::from_utf8(queue_element.data.as_slice()) {
        let (event, ttype, id) = scan_fmt_some!(el_str, "{},{}[{}]", String, String, String);

        if event.is_none() || ttype.is_none() || id.is_none() {
            error!("failed to parse queue element, data={:?}", queue_element.data);
            return Ok(true);
        }

        let event = event.unwrap();
        let ttype = ttype.unwrap();
        let stype = if ttype == "Task" {
            "bpmn:UserTaskHandler"
        } else if ttype == "ProcessInstance" {
            "bpmn:ProcessInstanceHandler"
        } else {
            ""
        };

        let mut is_found_script = false;
        for script_id in ctx.workplace.scripts_order.iter() {
            if let Some(script) = ctx.workplace.scripts.get(script_id) {
                if script.context.trigger_by_event.hash.contains(&event) && script.context.script_type.hash.contains(stype) {
                    is_found_script = true;
                    break;
                }
            }
        }

        if !is_found_script {
            return Ok(true);
        }

        if ttype == "Task" {
            let task_id = id.unwrap();

            thread::sleep(time::Duration::from_millis(100));

            let task = match ctx.camunda_client.task_api().get_task(&task_id) {
                Ok(t) => Some(json!(t).to_string()),
                Err(e) => {
                    error!("failed to read task {:?}", e);
                    None
                }
            };

            let vars = match ctx.camunda_client.task_api().get_variables(&task_id, None, Some(false)) {
                Ok(res) => Some(json!(res).to_string()),
                Err(e) => {
                    error!("failed to read variables {:?}", e);
                    None
                }
            };

            let form_vars = match ctx.camunda_client.task_api().get_form_variables(&task_id, None, Some(false)) {
                Ok(res) => Some(json!(res).to_string()),
                Err(e) => {
                    error!("failed to read form variables {:?}", e);
                    None
                }
            };

            match execute_js_user_task(&event, stype, task, vars, form_vars, ctx) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        } else if ttype == "ProcessInstance" {
            let process_instance_id = id.unwrap();
            thread::sleep(time::Duration::from_millis(1000));
            let process_instance = if event == "start" {
                match ctx.camunda_client.process_instance_api().get_process_instance(&process_instance_id) {
                    Ok(p) => Some(json!(p).to_string()),
                    Err(e) => {
                        error!("failed to read processInstance {:?}", e);
                        None
                    }
                }
            } else {
                None
            };

            match execute_js_process_instance(&event, stype, Some(process_instance_id), process_instance, ctx) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
    } else {
        error!("failed to parse queue element to utf8, data={:?}", queue_element.data);
    }

    Ok(true)
}

pub fn execute_js_user_task(
    event: &str,
    stype: &str,
    task: Option<String>,
    vars: Option<String>,
    form_vars: Option<String>,
    ctx: &mut Context,
) -> Result<i64, PrepareError> {
    let mut session_data = CallbackSharedData::default();
    session_data.g_key2attr.insert("$ticket".to_owned(), ctx.sys_ticket.to_owned());
    if let Some(v) = task {
        session_data.g_key2attr.insert("$task".to_owned(), v);
    }

    if let Some(v) = vars {
        session_data.g_key2attr.insert("$variables".to_owned(), v);
    }

    if let Some(v) = form_vars {
        session_data.g_key2attr.insert("$form_variables".to_owned(), v);
    }

    execute_js(event, stype, session_data, ctx)
}

pub fn execute_js_process_instance(
    event: &str,
    stype: &str,
    process_instance_id: Option<String>,
    process_instance: Option<String>,
    ctx: &mut Context,
) -> Result<i64, PrepareError> {
    let mut session_data = CallbackSharedData::default();
    session_data.g_key2attr.insert("$ticket".to_owned(), ctx.sys_ticket.to_owned());
    session_data.g_key2attr.insert("$processInstance".to_owned(), process_instance.unwrap_or("[]".to_owned()));
    session_data.g_key2attr.insert("$processInstanceId".to_owned(), process_instance_id.unwrap_or_default());

    execute_js(event, stype, session_data, ctx)
}
