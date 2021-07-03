use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  messaging_api: Box<::apis::MessagingApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      messaging_api: Box::new(::apis::MessagingApiClient::new(rc.clone())),
    }
  }

  pub fn messaging_api(&self) -> &::apis::MessagingApi{
    self.messaging_api.as_ref()
  }


}
