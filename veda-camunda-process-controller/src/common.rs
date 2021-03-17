use std::error::Error;
use std::fmt;
use v_module::module::Module;
use v_module::v_api::app::ResultCode;
use v_module::v_api::IndvOp;
use v_module::v_onto::datatype::Lang;
use v_module::v_onto::individual::Individual;
use v_module::v_onto::onto::Onto;

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}
/*
pub fn add_right(subj_uri: &str, obj_uri: &str, ctx: &mut Context, module: &mut Module) -> Result<(), Box<dyn Error>> {
    if subj_uri.is_empty() || obj_uri.is_empty() {
        return Err(Box::new(MyError("empty subj_uri or obj_uri".into())));
    }

    let mut right = Individual::default();
    right.set_id(&generate_unique_uri("wd:r_", ""));
    right.add_uri("rdf:type", "v-s:PermissionStatement");
    right.add_uri("v-s:permissionObject", obj_uri);
    right.add_uri("v-s:permissionSubject", subj_uri);
    right.add_bool("v-s:canRead", true);
    right.add_bool("v-s:canUpdate", true);

    module.api.update_or_err(&ctx.sys_ticket, "", "add-right", IndvOp::Put, &right)?;
    Ok(())
}
*/

pub fn get_individual(module: &mut Module, uri: &str) -> Result<Individual, Box<dyn Error>> {
    let mut indv = Individual::default();
    if uri.is_empty() || !module.storage.get_individual(uri, &mut indv) {
        return Err(Box::new(MyError(format!("individual {} not found", uri))));
    }
    Ok(indv)
}

pub(crate) fn is_content_type(rdf_types: &[String], check_type: &str, onto: &mut Onto) -> bool {
    for itype in rdf_types {
        if itype == check_type {
            return true;
        }
        if onto.is_some_entered(&itype, &[check_type]) {
            return true;
        }
    }
    false
}

pub fn set_err(module: &mut Module, sys_ticket: &str, indv: &mut Individual, err_text: &str) {
    indv.parse_all();
    indv.set_string("v-s:errorMessage", err_text, Lang::RU);
    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    let res = module.api.update(sys_ticket, IndvOp::Put, indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", indv.get_id(), res.result);
    } else {
        //info!("success update, uri={}", indv.get_id());
    }
}
