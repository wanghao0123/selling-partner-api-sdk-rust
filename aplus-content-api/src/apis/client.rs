use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  aplus_content_api: Box<::apis::AplusContentApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      aplus_content_api: Box::new(::apis::AplusContentApiClient::new(rc.clone())),
    }
  }

  pub fn aplus_content_api(&self) -> &::apis::AplusContentApi{
    self.aplus_content_api.as_ref()
  }


}
