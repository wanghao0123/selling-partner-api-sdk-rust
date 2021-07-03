use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  catalog_api: Box<::apis::CatalogApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      catalog_api: Box::new(::apis::CatalogApiClient::new(rc.clone())),
    }
  }

  pub fn catalog_api(&self) -> &::apis::CatalogApi{
    self.catalog_api.as_ref()
  }


}
