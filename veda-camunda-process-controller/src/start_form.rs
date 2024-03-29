use crate::common::{get_individual, set_err, CVI_USER_NAME};
use crate::Context;
use camunda_client::models::{StartProcessInstanceDto, VariableValueDto};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;
use v_common::module::veda_backend::Backend;
use v_common::onto::datatype::{DataType, Lang};
use v_common::onto::individual::Individual;
use v_common::v_api::api_client::{ApiError, IndvOp};

pub fn prepare_start_form(start_form: &mut Individual, ctx: &mut Context, module: &mut Backend, _signal: &str) -> Result<(), ApiError> {
    if start_form.any_exists("bpmn:hasStatus", &["bpmn:ToBeStarted"]) {
        if let Some(process_id) = start_form.get_first_literal("bpmn:processDefinitionKey") {
            //let start_form_id = start_form.get_id().to_owned();
            start_form.parse_all();
            start_form.remove("rdfs:isDefinedBy");
            start_form.remove("bpmn:hasStatus");

            let mut params = StartProcessInstanceDto::new();

            let mut vars = HashMap::new();
            let mut var = VariableValueDto::new();
            var.value = Some(json!(start_form.get_obj().as_json().to_string()));
            var._type = Some("json".to_owned());
            vars.insert("startForm".to_owned(), var);

            for p in start_form.get_predicates_of_type(DataType::Uri) {
                if let Some(rs) = start_form.get_resources(&p) {
                    for r in rs {
                        if let Ok(mut indv) = get_individual(module, r.get_uri()) {
                            indv.parse_all();
                            let mut var = VariableValueDto::new();
                            var.value = Some(json!(indv.get_obj().as_json().to_string().to_owned()));
                            var._type = Some("json".to_owned());
                            vars.insert(indv.get_id().to_owned(), var);
                        }
                    }
                }
            }

            params.variables = Some(vars);
            params.business_key = Some(Uuid::new_v4().to_string());
            match ctx.camunda_client.process_definition_api().start_process_instance_by_key(&process_id, Some(params)) {
                Ok(res) => {
                    info!("res={:?}", res);

                    // set status [STARTED] into start form
                    let mut updated_start_form = Individual::default();
                    updated_start_form.set_id(start_form.get_id());
                    updated_start_form.set_uri("bpmn:hasStatus", "bpmn:Started");
                    updated_start_form.set_string("bpmn:processInstanceId", &res.id.unwrap_or_default(), Lang::NONE);
                    updated_start_form.set_uri("v-s:lastEditor", CVI_USER_NAME);
                    module.mstorage_api.update_or_err(&ctx.sys_ticket, "", "start-process", IndvOp::SetIn, &updated_start_form)?;
                }
                Err(e) => {
                    error!("fail execute process instance, err={:?}", e);
                    set_err(module, &ctx.sys_ticket, start_form, &format!("{:?}", e));
                }
            };
        }
    }

    Ok(())
}
