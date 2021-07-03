use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  vendor_payments_api: Box<::apis::VendorPaymentsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      vendor_payments_api: Box::new(::apis::VendorPaymentsApiClient::new(rc.clone())),
    }
  }

  pub fn vendor_payments_api(&self) -> &::apis::VendorPaymentsApi{
    self.vendor_payments_api.as_ref()
  }


}
