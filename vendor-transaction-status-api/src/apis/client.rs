use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  vendor_transaction_api: Box<::apis::VendorTransactionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      vendor_transaction_api: Box::new(::apis::VendorTransactionApiClient::new(rc.clone())),
    }
  }

  pub fn vendor_transaction_api(&self) -> &::apis::VendorTransactionApi{
    self.vendor_transaction_api.as_ref()
  }


}
