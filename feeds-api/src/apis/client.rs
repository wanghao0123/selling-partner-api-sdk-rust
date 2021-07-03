use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  feeds_api: Box<::apis::FeedsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      feeds_api: Box::new(::apis::FeedsApiClient::new(rc.clone())),
    }
  }

  pub fn feeds_api(&self) -> &::apis::FeedsApi{
    self.feeds_api.as_ref()
  }


}
