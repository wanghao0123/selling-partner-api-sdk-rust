use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  small_and_light_api: Box<::apis::SmallAndLightApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      small_and_light_api: Box::new(::apis::SmallAndLightApiClient::new(rc.clone())),
    }
  }

  pub fn small_and_light_api(&self) -> &::apis::SmallAndLightApi{
    self.small_and_light_api.as_ref()
  }


}
