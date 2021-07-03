use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  solicitations_api: Box<::apis::SolicitationsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      solicitations_api: Box::new(::apis::SolicitationsApiClient::new(rc.clone())),
    }
  }

  pub fn solicitations_api(&self) -> &::apis::SolicitationsApi{
    self.solicitations_api.as_ref()
  }


}
