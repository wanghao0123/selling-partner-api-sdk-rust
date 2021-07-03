use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  fba_outbound_api: Box<::apis::FbaOutboundApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      fba_outbound_api: Box::new(::apis::FbaOutboundApiClient::new(rc.clone())),
    }
  }

  pub fn fba_outbound_api(&self) -> &::apis::FbaOutboundApi{
    self.fba_outbound_api.as_ref()
  }


}
