/* 
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network. Today this API is available only in the North America region. In 2021 we plan to release this API in the Europe and Far East regions.
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

pub struct FbaInventoryApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FbaInventoryApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FbaInventoryApiClient<C> {
        FbaInventoryApiClient {
            configuration: configuration,
        }
    }
}

pub trait FbaInventoryApi {
    fn get_inventory_summaries(&self, granularity_type: &str, granularity_id: &str, marketplace_ids: Vec<String>, details: bool, start_date_time: String, seller_skus: Vec<String>, next_token: &str) -> Box<Future<Item = ::models::GetInventorySummariesResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>FbaInventoryApi for FbaInventoryApiClient<C> {
    fn get_inventory_summaries(&self, granularity_type: &str, granularity_id: &str, marketplace_ids: Vec<String>, details: bool, start_date_time: String, seller_skus: Vec<String>, next_token: &str) -> Box<Future<Item = ::models::GetInventorySummariesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("details", &details.to_string());
            query.append_pair("granularityType", &granularity_type.to_string());
            query.append_pair("granularityId", &granularity_id.to_string());
            query.append_pair("startDateTime", &start_date_time.to_string());
            query.append_pair("sellerSkus", &seller_skus.join(",").to_string());
            query.append_pair("nextToken", &next_token.to_string());
            query.append_pair("marketplaceIds", &marketplace_ids.join(",").to_string());
            query.finish()
        };
        let uri_str = format!("{}/fba/inventory/v1/summaries?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::GetInventorySummariesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
