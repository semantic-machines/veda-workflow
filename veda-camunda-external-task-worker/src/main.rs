#[macro_use]
extern crate log;
#[macro_use]
extern crate scan_fmt;

use crate::common::out_value_2_complete_external_task;
use crate::v8_script::execute_external_js_task;
use camunda_client::apis::client::APIClient;
use camunda_client::apis::configuration::Configuration;
use camunda_client::models::{FetchExternalTaskTopicDto, FetchExternalTasksDto};
use serde_json::Value as JSONValue;
use std::str;
use std::{thread, time};
use v_camunda_common::scripts::{get_camunda_event_queue, load_task_scripts, Context, OutValue, REST_TIMEOUT};
use v_ft_xapian::xapian_reader::XapianReader;
use v_module::module::PrepareError;
use v_module::module::{get_info_of_module, init_log, wait_load_ontology, wait_module, Module};
use v_module::remote_indv_r_storage::inproc_storage_manager;
use v_module::v_api::APIClient as VedaClient;
use v_module::v_onto::individual::RawObj;
use v_module::veda_backend::*;
use v_queue::consumer::Consumer;
use v_v8::jsruntime::JsRuntime;
use v_v8::scripts_workplace::ScriptsWorkPlace;

mod common;
mod v8_script;

fn main() -> Result<(), i32> {
    init_log("CAMUNDA-EXTERNAL-TASK");
    thread::spawn(move || inproc_storage_manager());

    let mut js_runtime = JsRuntime::new();
    listen_queue(&mut js_runtime)
}

fn listen_queue<'a>(js_runtime: &'a mut JsRuntime) -> Result<(), i32> {
    if get_info_of_module("fulltext_indexer").unwrap_or((0, 0)).0 == 0 {
        wait_module("fulltext_indexer", wait_load_ontology());
    }

    let mut module = Module::default();
    let mut backend = Backend::default();

    while !backend.api.connect() {
        error!("main module not ready, sleep and repeat");
        thread::sleep(time::Duration::from_millis(1000));
    }
    let sys_ticket;
    if let Ok(t) = backend.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("fail get sys_ticket");
        return Err(-1);
    }

    if let Some(xr) = XapianReader::new("russian", &mut backend.storage) {
        let mut ctx = Context {
            workplace: ScriptsWorkPlace::new(js_runtime.v8_isolate()),
            sys_ticket,
            xr,
            camunda_client: APIClient::new(Configuration::default()),
            veda_client: VedaClient::new(Module::get_property("main_module_url").unwrap_or_default()),
            count_exec: 0,
        };
        ctx.workplace.load_ext_scripts(&ctx.sys_ticket);

        load_task_scripts(&mut ctx.workplace, &mut ctx.xr, "bpmn:ExternalTaskHandler", &[("ticket", "string"), ("task", "object")]);

        module.listen_queue_raw(
            &mut get_camunda_event_queue("camunda-external-task-worker"),
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

fn prepare_and_err<'a>(_module: &mut Backend, ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    if let Ok(el_str) = str::from_utf8(queue_element.data.as_slice()) {
        if let (
            Some(event_type),
            Some(_event),
            Some(_id),
            Some(_process_instance_id),
            Some(_super_process_instance_id),
            Some(_business_key),
            Some(_process_definition_key),
            Some(element_type),
            Some(_element_id),
        ) = scan_fmt_some!(el_str, "{}:{},{},{},{},{},{},{},{}", String, String, String, String, String, String, String, String, String)
        {
            if !(event_type == "ExecutionEvent" && element_type == "serviceTask") {
                return Ok(true);
            }
            let worker_id = "camunda-external-task";

            thread::sleep(REST_TIMEOUT);

            match ctx.camunda_client.external_task_api().get_external_tasks(
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ) {
                Ok(res) => {
                    for task in res {
                        if task.worker_id.is_some() {
                            continue;
                        }
                        let mut fetch_task_arg = FetchExternalTasksDto::new(worker_id.to_owned(), Some(1));
                        fetch_task_arg.topics = Some(vec![FetchExternalTaskTopicDto::new(task.topic_name.unwrap_or_default(), Some(60 * 60 * 1000))]);
                        match ctx.camunda_client.external_task_api().fetch_and_lock(Some(fetch_task_arg)) {
                            Ok(locked_tasks) => {
                                for i_task in locked_tasks.iter() {
                                    let execution_id = i_task.id.as_deref().unwrap_or_default();
                                    let topic_id = i_task.topic_name.as_deref().unwrap_or_default();
                                    let mut res = OutValue::Json(JSONValue::default());
                                    if let Ok(is_executed) = execute_external_js_task(i_task, topic_id, ctx, &mut res) {
                                        if is_executed {
                                            let out_data = out_value_2_complete_external_task(worker_id, res);
                                            if let Err(e) = ctx.camunda_client.external_task_api().complete_external_task_resource(&execution_id, Some(out_data)) {
                                                error!("complete_external_task_resource, error={:?}", e);
                                            }
                                        } else {
                                            warn!("not found script id={}", topic_id);
                                            if let Err(e) = ctx.camunda_client.external_task_api().unlock(&execution_id) {
                                                error!("filed to unlock task, execution_id={}, err={:?}", execution_id, e);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => error!("fetch_and_lock, error={:?}", e),
                        }
                    }
                }
                Err(e) => error!("get_external_tasks, error={:?}", e),
            }
        }
    }
    Ok(true)
}
