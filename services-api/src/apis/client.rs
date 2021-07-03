use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  service_api: Box<::apis::ServiceApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      service_api: Box::new(::apis::ServiceApiClient::new(rc.clone())),
    }
  }

  pub fn service_api(&self) -> &::apis::ServiceApi{
    self.service_api.as_ref()
  }


}
