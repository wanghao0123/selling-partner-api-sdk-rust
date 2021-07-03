use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  fba_inventory_api: Box<::apis::FbaInventoryApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      fba_inventory_api: Box::new(::apis::FbaInventoryApiClient::new(rc.clone())),
    }
  }

  pub fn fba_inventory_api(&self) -> &::apis::FbaInventoryApi{
    self.fba_inventory_api.as_ref()
  }


}
