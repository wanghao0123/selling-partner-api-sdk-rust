use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  fba_inbound_api: Box<::apis::FbaInboundApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      fba_inbound_api: Box::new(::apis::FbaInboundApiClient::new(rc.clone())),
    }
  }

  pub fn fba_inbound_api(&self) -> &::apis::FbaInboundApi{
    self.fba_inbound_api.as_ref()
  }


}
