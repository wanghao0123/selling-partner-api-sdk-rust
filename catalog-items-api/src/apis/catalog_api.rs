/* 
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * OpenAPI spec version: v0
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

pub struct CatalogApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> CatalogApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> CatalogApiClient<C> {
        CatalogApiClient {
            configuration: configuration,
        }
    }
}

pub trait CatalogApi {
    fn get_catalog_item(&self, marketplace_id: &str, asin: &str) -> Box<Future<Item = ::models::GetCatalogItemResponse, Error = Error<serde_json::Value>>>;
    fn list_catalog_categories(&self, marketplace_id: &str, ASIN: &str, seller_sku: &str) -> Box<Future<Item = ::models::ListCatalogCategoriesResponse, Error = Error<serde_json::Value>>>;
    fn list_catalog_items(&self, marketplace_id: &str, query: &str, query_context_id: &str, seller_sku: &str, UPC: &str, EAN: &str, ISBN: &str, JAN: &str) -> Box<Future<Item = ::models::ListCatalogItemsResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>CatalogApi for CatalogApiClient<C> {
    fn get_catalog_item(&self, marketplace_id: &str, asin: &str) -> Box<Future<Item = ::models::GetCatalogItemResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("MarketplaceId", &marketplace_id.to_string());
            query.finish()
        };
        let uri_str = format!("{}/catalog/v0/items/{asin}?{}", configuration.base_path, query_string, asin=asin);

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
                let parsed: Result<::models::GetCatalogItemResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn list_catalog_categories(&self, marketplace_id: &str, ASIN: &str, seller_sku: &str) -> Box<Future<Item = ::models::ListCatalogCategoriesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("MarketplaceId", &marketplace_id.to_string());
            query.append_pair("ASIN", &ASIN.to_string());
            query.append_pair("SellerSKU", &seller_sku.to_string());
            query.finish()
        };
        let uri_str = format!("{}/catalog/v0/categories?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::ListCatalogCategoriesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn list_catalog_items(&self, marketplace_id: &str, query: &str, query_context_id: &str, seller_sku: &str, UPC: &str, EAN: &str, ISBN: &str, JAN: &str) -> Box<Future<Item = ::models::ListCatalogItemsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("MarketplaceId", &marketplace_id.to_string());
            query.append_pair("Query", &query.to_string());
            query.append_pair("QueryContextId", &query_context_id.to_string());
            query.append_pair("SellerSKU", &seller_sku.to_string());
            query.append_pair("UPC", &UPC.to_string());
            query.append_pair("EAN", &EAN.to_string());
            query.append_pair("ISBN", &ISBN.to_string());
            query.append_pair("JAN", &JAN.to_string());
            query.finish()
        };
        let uri_str = format!("{}/catalog/v0/items?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::ListCatalogItemsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
