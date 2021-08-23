#[macro_use]
extern crate log;

use crate::common::{is_content_type, CVI_USER_NAME};
use crate::decision_form::prepare_decision_form;
use crate::start_form::prepare_start_form;
use crate::stop_process::prepare_stop_process;
use camunda_client::apis::client::APIClient;
use camunda_client::apis::configuration::Configuration;
use v_common::module::common::load_onto;
use v_common::module::info::ModuleInfo;
use v_common::module::module::{get_cmd, get_info_of_module, get_inner_binobj_as_individual, init_log, wait_load_ontology, wait_module, Module, PrepareError};
use v_common::module::veda_backend::Backend;
use v_common::onto::individual::Individual;
use v_common::onto::onto::Onto;
use v_common::v_api::api_client::ApiError;
use v_queue::consumer::Consumer;

mod common;
mod decision_form;
mod start_form;
mod stop_process;

pub struct Context {
    sys_ticket: String,
    onto: Onto,
    pub camunda_client: APIClient,
}

fn main() -> Result<(), i32> {
    init_log("CAMUNDA-CONNECTOR");

    if get_info_of_module("fulltext_indexer").unwrap_or((0, 0)).0 == 0 {
        wait_module("fulltext_indexer", wait_load_ontology());
    }

    listen_queue()
}

fn listen_queue() -> Result<(), i32> {
    let mut backend = Backend::default();
    let mut module = Module::default();
    let systicket;
    if let Ok(t) = backend.get_sys_ticket_id() {
        systicket = t;
    } else {
        error!("fail get systicket");
        return Ok(());
    }

    let mut onto = Onto::default();

    info!("load onto start");
    load_onto(&mut backend.storage, &mut onto);
    info!("load onto end");

    let module_info = ModuleInfo::new("./data", "camunda-connector", true);
    if module_info.is_err() {
        error!("{:?}", module_info.err());
        return Err(-1);
    }

    //wait_load_ontology();

    let mut queue_consumer = Consumer::new("./data/queue", "camunda_connector", "individuals-flow").expect("!!!!!!!!! FAIL QUEUE");

    let configuration = Configuration::default();

    let mut ctx = Context {
        sys_ticket: systicket,
        onto,
        camunda_client: APIClient::new(configuration),
    };

    module.listen_queue(
        &mut queue_consumer,
        &mut ctx,
        &mut (before_batch as fn(&mut Backend, &mut Context, batch_size: u32) -> Option<u32>),
        &mut (prepare as fn(&mut Backend, &mut Context, &mut Individual, my_consumer: &Consumer) -> Result<bool, PrepareError>),
        &mut (after_batch as fn(&mut Backend, &mut Context, prepared_batch_size: u32) -> Result<bool, PrepareError>),
        &mut (heartbeat as fn(&mut Backend, &mut Context) -> Result<(), PrepareError>),
        &mut backend,
    );
    Ok(())
}

fn heartbeat(_module: &mut Backend, _ctx: &mut Context) -> Result<(), PrepareError> {
    Ok(())
}

fn before_batch(_module: &mut Backend, _ctx: &mut Context, _size_batch: u32) -> Option<u32> {
    None
}

fn after_batch(_module: &mut Backend, _ctx: &mut Context, _prepared_batch_size: u32) -> Result<bool, PrepareError> {
    Ok(false)
}

fn prepare(module: &mut Backend, ctx: &mut Context, queue_element: &mut Individual, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    match prepare_and_err(module, ctx, queue_element, _my_consumer) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("prepare err={:?}", e);
            Err(PrepareError::Recoverable)
        }
    }
}

fn prepare_and_err(module: &mut Backend, ctx: &mut Context, queue_element: &mut Individual, _my_consumer: &Consumer) -> Result<bool, ApiError> {
    let cmd = get_cmd(queue_element);
    if cmd.is_none() {
        error!("cmd is none");
        return Ok(true);
    }
    let signal = queue_element.get_first_literal("src").unwrap_or_default();

    let mut new_state = Individual::default();
    get_inner_binobj_as_individual(queue_element, "new_state", &mut new_state);

    if let Some(v) = new_state.get_first_literal("v-s:lastEditor") {
        if v == CVI_USER_NAME {
            return Ok(true);
        }
    }

    let rdf_types = new_state.get_literals("rdf:type").unwrap_or_default();

    if is_content_type(&rdf_types, "bpmn:StartForm", &mut ctx.onto) && signal == "?" {
        prepare_start_form(&mut new_state, ctx, module, &signal)?;
        return Ok(true);
    }

    if is_content_type(&rdf_types, "bpmn:DecisionForm", &mut ctx.onto) && signal == "?" {
        prepare_decision_form(&mut new_state, ctx, module, &signal)?;
        return Ok(true);
    }

    if is_content_type(&rdf_types, "bpmn:ProcessInstanceStopRequest", &mut ctx.onto) && signal == "?" {
        prepare_stop_process(&mut new_state, ctx, module, &signal)?;
        return Ok(true);
    }

    Ok(true)
}
