#[macro_use]
extern crate log;
#[macro_use]
extern crate scan_fmt;

mod common;

use crate::common::{check_filters, execute_js};
use camunda_client::apis::client::APIClient;
use camunda_client::apis::configuration::Configuration;
use serde_json::json;
use std::{str, thread};
use v_camunda_common::scripts::{get_camunda_event_queue, load_task_scripts, Context, REST_TIMEOUT};
use v_ft_xapian::xapian_reader::XapianReader;
use v_module::module::{init_log, Module, PrepareError};
use v_module::remote_indv_r_storage::*;
use v_module::v_api::APIClient as VedaClient;
use v_module::v_onto::individual::RawObj;
use v_module::veda_backend::*;
use v_queue::consumer::Consumer;
use v_v8::jsruntime::JsRuntime;
use v_v8::scripts_workplace::ScriptsWorkPlace;
use v_v8::session_cache::CallbackSharedData;

fn main() -> Result<(), i32> {
    init_log("CAMUNDA-EVENT-HANDLER");
    thread::spawn(move || inproc_storage_manager());

    let mut js_runtime = JsRuntime::new();
    listen_queue(&mut js_runtime)
}

fn listen_queue<'a>(js_runtime: &'a mut JsRuntime) -> Result<(), i32> {
    let mut module = Module::default();
    let mut backend = Backend::default();
    let sys_ticket;
    if let Ok(t) = backend.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("failed to get system ticket");
        return Ok(());
    }

    if let Some(xr) = XapianReader::new("russian", &mut backend.storage) {
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

        load_task_scripts(
            &mut ctx.workplace,
            &mut ctx.xr,
            "bpmn:UserTaskHandler",
            &[
                ("event", "string"),
                ("taskId", "string"),
                ("processInstanceId", "string"),
                ("superProcessInstanceId", "string"),
                ("businessKey", "string"),
                ("processDefinitionKey", "string"),
                ("elementType", "string"),
                ("elementId", "string"),
                ("task", "object"),
                ("variables", "object"),
            ],
        );

        load_task_scripts(
            &mut ctx.workplace,
            &mut ctx.xr,
            "bpmn:ExecutionHandler",
            &[
                ("event", "string"),
                ("executionId", "string"),
                ("processInstanceId", "string"),
                ("superProcessInstanceId", "string"),
                ("businessKey", "string"),
                ("processDefinitionKey", "string"),
                ("elementType", "string"),
                ("elementId", "string"),
                ("execution", "object"),
                ("variables", "object"),
            ],
        );

        module.listen_queue_raw(
            &mut get_camunda_event_queue("camunda-event-handler"),
            &mut ctx,
            &mut (before_batch as fn(&mut Backend, &mut Context<'a>, batch_size: u32) -> Option<u32>),
            &mut (prepare as fn(&mut Backend, &mut Context<'a>, &RawObj, my_consumer: &Consumer) -> Result<bool, PrepareError>),
            &mut (after_batch as fn(&mut Backend, &mut Context<'a>, prepared_batch_size: u32) -> Result<bool, PrepareError>),
            &mut (heartbeat as fn(&mut Backend, &mut Context<'a>) -> Result<(), PrepareError>),
            &mut backend,
        );
    }
    Ok(())
}

fn heartbeat<'a>(_module: &mut Backend, _ctx: &mut Context<'a>) -> Result<(), PrepareError> {
    Ok(())
}

fn before_batch<'a>(_module: &mut Backend, _ctx: &mut Context<'a>, _size_batch: u32) -> Option<u32> {
    None
}

fn after_batch<'a>(_module: &mut Backend, _ctx: &mut Context<'a>, _prepared_batch_size: u32) -> Result<bool, PrepareError> {
    Ok(false)
}

fn prepare<'a>(module: &mut Backend, ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    match prepare_and_err(module, ctx, queue_element, _my_consumer) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("prepare err={:?}", e);
            Err(PrepareError::Recoverable)
        }
    }
}

pub struct QueueElement {
    rtype: String,
    event_type: String,
    event: String,
    id: String,
    process_instance_id: String,
    super_process_instance_id: String,
    business_key: String,
    process_definition_key: String,
    element_type: String,
    element_id: String,
}

fn prepare_and_err<'a>(_module: &mut Backend, ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    if let Ok(el_str) = str::from_utf8(queue_element.data.as_slice()) {
        if let (
            Some(event_type),
            Some(event),
            Some(id),
            Some(process_instance_id),
            Some(super_process_instance_id),
            Some(business_key),
            Some(process_definition_key),
            Some(element_type),
            Some(element_id),
        ) = scan_fmt_some!(el_str, "{}:{},{},{},{},{},{},{},{}", String, String, String, String, String, String, String, String, String)
        {
            let qel = if event_type == "UserTaskEvent" {
                QueueElement {
                    rtype: "bpmn:UserTaskHandler".to_owned(),
                    event_type,
                    event,
                    id,
                    process_instance_id,
                    super_process_instance_id,
                    business_key,
                    process_definition_key,
                    element_type,
                    element_id,
                }
            } else if event_type == "ExecutionEvent" {
                QueueElement {
                    rtype: "bpmn:ExecutionHandler".to_owned(),
                    event_type,
                    event,
                    id,
                    process_instance_id,
                    super_process_instance_id,
                    business_key,
                    process_definition_key,
                    element_type,
                    element_id,
                }
            } else {
                QueueElement {
                    rtype: "".to_string(),
                    event_type: "".to_string(),
                    event: "".to_string(),
                    id: "".to_string(),
                    process_instance_id: "".to_string(),
                    super_process_instance_id: "".to_string(),
                    business_key: "".to_string(),
                    process_definition_key: "".to_string(),
                    element_type: "".to_string(),
                    element_id: "".to_string(),
                }
            };

            let mut is_found_script = false;
            let mut is_fetch_event_data = false;

            for script_id in ctx.workplace.scripts_order.iter() {
                if let Some(script) = ctx.workplace.scripts.get(script_id) {
                    if check_filters(script, &qel) {
                        if script.context.fetch_event_data {
                            is_fetch_event_data = true;
                        }
                        is_found_script = true;
                    }
                }
            }

            if !is_found_script {
                return Ok(true);
            }

            let mut session_data = CallbackSharedData::default();
            session_data.g_key2attr.insert("$ticket".to_owned(), ctx.sys_ticket.to_owned());
            session_data.g_key2attr.insert("$processInstanceId".to_owned(), qel.process_instance_id.to_owned());
            session_data.g_key2attr.insert("$superProcessInstanceId".to_owned(), qel.super_process_instance_id.to_owned());
            session_data.g_key2attr.insert("$businessKey".to_owned(), qel.business_key.to_owned());
            session_data.g_key2attr.insert("$processDefinitionKey".to_owned(), qel.process_definition_key.to_owned());
            session_data.g_key2attr.insert("$event".to_owned(), qel.event.to_owned());
            session_data.g_key2attr.insert("$elementType".to_owned(), qel.element_type.to_owned());
            session_data.g_key2attr.insert("$elementId".to_owned(), qel.element_id.to_owned());

            if qel.event_type == "UserTaskEvent" {
                session_data.g_key2attr.insert("$taskId".to_owned(), qel.id.to_owned());

                if is_fetch_event_data {
                    thread::sleep(REST_TIMEOUT);

                    match ctx.camunda_client.task_api().get_task(&qel.id) {
                        Ok(v) => {
                            session_data.g_key2attr.insert("$task".to_owned(), json!(v).to_string());
                        }
                        Err(e) => {
                            error!("failed to read task {:?}", e);
                        }
                    }

                    match ctx.camunda_client.task_api().get_variables(&qel.id, None, Some(false)) {
                        Ok(v) => {
                            session_data.g_key2attr.insert("$variables".to_owned(), json!(v).to_string());
                        }
                        Err(e) => {
                            error!("failed to read variables {:?}", e);
                        }
                    }
                }
            } else if qel.event_type == "ExecutionEvent" {
                session_data.g_key2attr.insert("$executionId".to_owned(), qel.id.to_owned());

                if is_fetch_event_data {
                    thread::sleep(REST_TIMEOUT);

                    match ctx.camunda_client.execution_api().get_execution(&qel.id) {
                        Ok(p) => {
                            session_data.g_key2attr.insert("$execution".to_owned(), json!(p).to_string());
                        }
                        Err(e) => {
                            error!("failed to read execution {:?}", e);
                        }
                    }

                    match ctx.camunda_client.execution_api().get_variables(&qel.id, None, Some(false)) {
                        Ok(v) => {
                            session_data.g_key2attr.insert("$variables".to_owned(), json!(v).to_string());
                        }
                        Err(e) => {
                            error!("failed to read variables {:?}", e);
                        }
                    }
                }
            }

            match execute_js(&qel, session_data, ctx) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            error!("failed to parse queue element, data={:?}", el_str);
            return Ok(true);
        }
    } else {
        error!("failed to parse queue element to utf8, data={:?}", queue_element.data);
    }

    Ok(true)
}
