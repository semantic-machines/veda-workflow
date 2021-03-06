#[macro_use]
extern crate log;
#[macro_use]
extern crate scan_fmt;

use camunda_client::apis::client::APIClient;
use camunda_client::apis::configuration::Configuration;
use std::error::Error;
use std::str;
use v_camunda_common::scripts::{load_task_scripts, Context};
use v_ft_xapian::xapian_reader::XapianReader;
use v_module::common::load_onto;
use v_module::module::{init_log, Module, PrepareError};
use v_module::v_onto::individual::RawObj;
use v_module::v_onto::onto::Onto;
use v_queue::consumer::Consumer;
use v_v8::jsruntime::JsRuntime;
use v_v8::scripts_workplace::ScriptsWorkPlace;

fn main() -> Result<(), i32> {
    init_log("VEDA-CAMUNDA-USER-TASK");

    let mut js_runtime = JsRuntime::new();
    listen_queue(&mut js_runtime)
}

fn listen_queue<'a>(js_runtime: &'a mut JsRuntime) -> Result<(), i32> {
    let mut module = Module::default();
    let sys_ticket;
    if let Ok(t) = module.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("fail get systicket");
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
            api_client: APIClient::new(Configuration::default()),
        };
        ctx.workplace.load_ext_scripts(&ctx.sys_ticket);

        load_task_scripts(&mut ctx.workplace, &mut ctx.xr, "bpmn:UserTaskHandler");

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

fn prepare_and_err<'a>(_module: &mut Module, _ctx: &mut Context<'a>, queue_element: &RawObj, _my_consumer: &Consumer) -> Result<bool, Box<dyn Error>> {
    if let Ok(el_str) = str::from_utf8(queue_element.data.as_slice()) {
        let (event, ttype, id) = scan_fmt_some!(el_str, "{},{}[{}]", String, String, String);

        if event.is_none() || ttype.is_none() || id.is_none() {
            error!("failed to parse queue element, data={:?}", queue_element.data);
            return Ok(true);
        }

        if ttype.unwrap() == "Task" {}
    } else {
        error!("failed to parse queue element to utf8, data={:?}", queue_element.data);
    }

    Ok(true)
}
