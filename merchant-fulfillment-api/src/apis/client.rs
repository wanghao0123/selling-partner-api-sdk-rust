use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  merchant_fulfillment_api: Box<::apis::MerchantFulfillmentApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      merchant_fulfillment_api: Box::new(::apis::MerchantFulfillmentApiClient::new(rc.clone())),
    }
  }

  pub fn merchant_fulfillment_api(&self) -> &::apis::MerchantFulfillmentApi{
    self.merchant_fulfillment_api.as_ref()
  }


}
