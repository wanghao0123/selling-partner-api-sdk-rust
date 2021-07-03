use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  fees_api: Box<::apis::FeesApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      fees_api: Box::new(::apis::FeesApiClient::new(rc.clone())),
    }
  }

  pub fn fees_api(&self) -> &::apis::FeesApi{
    self.fees_api.as_ref()
  }


}
