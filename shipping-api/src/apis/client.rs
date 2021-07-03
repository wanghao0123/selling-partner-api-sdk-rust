use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  shipping_api: Box<::apis::ShippingApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      shipping_api: Box::new(::apis::ShippingApiClient::new(rc.clone())),
    }
  }

  pub fn shipping_api(&self) -> &::apis::ShippingApi{
    self.shipping_api.as_ref()
  }


}
