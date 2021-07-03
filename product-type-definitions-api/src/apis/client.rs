use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  definitions_api: Box<::apis::DefinitionsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      definitions_api: Box::new(::apis::DefinitionsApiClient::new(rc.clone())),
    }
  }

  pub fn definitions_api(&self) -> &::apis::DefinitionsApi{
    self.definitions_api.as_ref()
  }


}
