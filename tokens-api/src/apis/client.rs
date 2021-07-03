use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  tokens_api: Box<::apis::TokensApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      tokens_api: Box::new(::apis::TokensApiClient::new(rc.clone())),
    }
  }

  pub fn tokens_api(&self) -> &::apis::TokensApi{
    self.tokens_api.as_ref()
  }


}
