use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  listings_api: Box<::apis::ListingsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      listings_api: Box::new(::apis::ListingsApiClient::new(rc.clone())),
    }
  }

  pub fn listings_api(&self) -> &::apis::ListingsApi{
    self.listings_api.as_ref()
  }


}
