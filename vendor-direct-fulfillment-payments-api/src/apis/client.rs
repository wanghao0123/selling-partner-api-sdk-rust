use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  vendor_invoice_api: Box<::apis::VendorInvoiceApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      vendor_invoice_api: Box::new(::apis::VendorInvoiceApiClient::new(rc.clone())),
    }
  }

  pub fn vendor_invoice_api(&self) -> &::apis::VendorInvoiceApi{
    self.vendor_invoice_api.as_ref()
  }


}
