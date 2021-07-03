use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  update_inventory_api: Box<::apis::UpdateInventoryApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      update_inventory_api: Box::new(::apis::UpdateInventoryApiClient::new(rc.clone())),
    }
  }

  pub fn update_inventory_api(&self) -> &::apis::UpdateInventoryApi{
    self.update_inventory_api.as_ref()
  }


}
