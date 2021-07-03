use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  notifications_api: Box<::apis::NotificationsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      notifications_api: Box::new(::apis::NotificationsApiClient::new(rc.clone())),
    }
  }

  pub fn notifications_api(&self) -> &::apis::NotificationsApi{
    self.notifications_api.as_ref()
  }


}
