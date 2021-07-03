use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  sellers_api: Box<::apis::SellersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      sellers_api: Box::new(::apis::SellersApiClient::new(rc.clone())),
    }
  }

  pub fn sellers_api(&self) -> &::apis::SellersApi{
    self.sellers_api.as_ref()
  }


}
