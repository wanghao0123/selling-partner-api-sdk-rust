use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  customer_invoices_api: Box<::apis::CustomerInvoicesApi>,
  vendor_shipping_api: Box<::apis::VendorShippingApi>,
  vendor_shipping_labels_api: Box<::apis::VendorShippingLabelsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      customer_invoices_api: Box::new(::apis::CustomerInvoicesApiClient::new(rc.clone())),
      vendor_shipping_api: Box::new(::apis::VendorShippingApiClient::new(rc.clone())),
      vendor_shipping_labels_api: Box::new(::apis::VendorShippingLabelsApiClient::new(rc.clone())),
    }
  }

  pub fn customer_invoices_api(&self) -> &::apis::CustomerInvoicesApi{
    self.customer_invoices_api.as_ref()
  }

  pub fn vendor_shipping_api(&self) -> &::apis::VendorShippingApi{
    self.vendor_shipping_api.as_ref()
  }

  pub fn vendor_shipping_labels_api(&self) -> &::apis::VendorShippingLabelsApi{
    self.vendor_shipping_labels_api.as_ref()
  }


}
