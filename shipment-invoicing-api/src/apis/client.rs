use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  shipment_invoice_api: Box<::apis::ShipmentInvoiceApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      shipment_invoice_api: Box::new(::apis::ShipmentInvoiceApiClient::new(rc.clone())),
    }
  }

  pub fn shipment_invoice_api(&self) -> &::apis::ShipmentInvoiceApi{
    self.shipment_invoice_api.as_ref()
  }


}
