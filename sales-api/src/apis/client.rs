use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  sales_api: Box<::apis::SalesApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      sales_api: Box::new(::apis::SalesApiClient::new(rc.clone())),
    }
  }

  pub fn sales_api(&self) -> &::apis::SalesApi{
    self.sales_api.as_ref()
  }


}
