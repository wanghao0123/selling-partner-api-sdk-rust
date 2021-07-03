use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  orders_v0_api: Box<::apis::OrdersV0Api>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      orders_v0_api: Box::new(::apis::OrdersV0ApiClient::new(rc.clone())),
    }
  }

  pub fn orders_v0_api(&self) -> &::apis::OrdersV0Api{
    self.orders_v0_api.as_ref()
  }


}
