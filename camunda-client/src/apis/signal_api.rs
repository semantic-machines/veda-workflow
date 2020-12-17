/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct SignalApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SignalApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SignalApiClient {
        SignalApiClient {
            configuration,
        }
    }
}

pub trait SignalApi {
    fn throw_signal(&self, signal_dto: Option<crate::models::SignalDto>) -> Result<(), Error>;
}

impl SignalApi for SignalApiClient {
    fn throw_signal(&self, signal_dto: Option<crate::models::SignalDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/signal", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&signal_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

}
