use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  vendor_shipping_api: Box<::apis::VendorShippingApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      vendor_shipping_api: Box::new(::apis::VendorShippingApiClient::new(rc.clone())),
    }
  }

  pub fn vendor_shipping_api(&self) -> &::apis::VendorShippingApi{
    self.vendor_shipping_api.as_ref()
  }


}
