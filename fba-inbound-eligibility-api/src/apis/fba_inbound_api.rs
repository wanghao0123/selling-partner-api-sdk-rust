/* 
 * Selling Partner API for FBA Inbound Eligibilty
 *
 * With the FBA Inbound Eligibility API, you can build applications that let sellers get eligibility previews for items before shipping them to Amazon's fulfillment centers. With this API you can find out if an item is eligible for inbound shipment to Amazon's fulfillment centers in a specific marketplace. You can also find out if an item is eligible for using the manufacturer barcode for FBA inventory tracking. Sellers can use this information to inform their decisions about which items to ship Amazon's fulfillment centers.
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

pub struct FbaInboundApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FbaInboundApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FbaInboundApiClient<C> {
        FbaInboundApiClient {
            configuration: configuration,
        }
    }
}

pub trait FbaInboundApi {
    fn get_item_eligibility_preview(&self, asin: &str, program: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::GetItemEligibilityPreviewResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>FbaInboundApi for FbaInboundApiClient<C> {
    fn get_item_eligibility_preview(&self, asin: &str, program: &str, marketplace_ids: Vec<String>) -> Box<Future<Item = ::models::GetItemEligibilityPreviewResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("marketplaceIds", &marketplace_ids.join(",").to_string());
            query.append_pair("asin", &asin.to_string());
            query.append_pair("program", &program.to_string());
            query.finish()
        };
        let uri_str = format!("{}/fba/inbound/v1/eligibility/itemPreview?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::GetItemEligibilityPreviewResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
