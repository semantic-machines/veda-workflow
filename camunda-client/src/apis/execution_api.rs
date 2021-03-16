
use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct ExecutionApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ExecutionApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ExecutionApiClient {
        ExecutionApiClient {
            configuration,
        }
    }
}

pub trait ExecutionApi {
    fn get_execution(&self, id: &str) -> Result<crate::models::ExecutionDto, Error>;
    fn get_variables(&self, id: &str, variable_names: Option<&str>, deserialize_values: Option<bool>) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, Error>;
}

impl ExecutionApi for ExecutionApiClient {
    fn get_execution(&self, id: &str) -> Result<crate::models::ExecutionDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/execution/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_variables(&self, id: &str, variable_names: Option<&str>, deserialize_values: Option<bool>) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/task/{id}/localVariables", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = variable_names {
            req_builder = req_builder.query(&[("variableNames", &s.to_string())]);
        }
        if let Some(ref s) = deserialize_values {
            req_builder = req_builder.query(&[("deserializeValues", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
