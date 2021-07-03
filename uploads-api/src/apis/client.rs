use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  uploads_api: Box<::apis::UploadsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      uploads_api: Box::new(::apis::UploadsApiClient::new(rc.clone())),
    }
  }

  pub fn uploads_api(&self) -> &::apis::UploadsApi{
    self.uploads_api.as_ref()
  }


}
