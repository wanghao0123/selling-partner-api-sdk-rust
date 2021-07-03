use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  vendor_orders_api: Box<::apis::VendorOrdersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      vendor_orders_api: Box::new(::apis::VendorOrdersApiClient::new(rc.clone())),
    }
  }

  pub fn vendor_orders_api(&self) -> &::apis::VendorOrdersApi{
    self.vendor_orders_api.as_ref()
  }


}
