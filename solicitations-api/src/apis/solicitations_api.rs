/* 
 * Selling Partner API for Solicitations
 *
 * With the Solicitations API you can build applications that send non-critical solicitations to buyers. You can get a list of solicitation types that are available for an order that you specify, then call an operation that sends a solicitation to the buyer for that order. Buyers cannot respond to solicitations sent by this API, and these solicitations do not appear in the Messaging section of Seller Central or in the recipient's Message Center. The Solicitations API returns responses that are formed according to the <a href=https://tools.ietf.org/html/draft-kelly-json-hal-08>JSON Hypertext Application Language</a> (HAL) standard.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct SolicitationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SolicitationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SolicitationsApiClient<C> {
        SolicitationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait SolicitationsApi {
    fn create_product_review_and_seller_feedback_solicitation(&self, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::CreateProductReviewAndSellerFeedbackSolicitationResponse, Error = Error<serde_json::Value>>>;
    fn get_solicitation_actions_for_order(&self, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::GetSolicitationActionsForOrderResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>SolicitationsApi for SolicitationsApiClient<C> {
    fn create_product_review_and_seller_feedback_solicitation(&self, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::CreateProductReviewAndSellerFeedbackSolicitationResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("marketplaceIds", &marketplace_ids.join(",").to_string());
            query.finish()
        };
        let uri_str = format!("{}/solicitations/v1/orders/{amazonOrderId}/solicitations/productReviewAndSellerFeedback?{}", configuration.base_path, query_string, amazonOrderId=amazon_order_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }




        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::CreateProductReviewAndSellerFeedbackSolicitationResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_solicitation_actions_for_order(&self, amazon_order_id: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::GetSolicitationActionsForOrderResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("marketplaceIds", &marketplace_ids.join(",").to_string());
            query.finish()
        };
        let uri_str = format!("{}/solicitations/v1/orders/{amazonOrderId}?{}", configuration.base_path, query_string, amazonOrderId=amazon_order_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }




        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::GetSolicitationActionsForOrderResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
