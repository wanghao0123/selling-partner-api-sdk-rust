/* 
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
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

pub struct ShipmentInvoiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ShipmentInvoiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ShipmentInvoiceApiClient<C> {
        ShipmentInvoiceApiClient {
            configuration: configuration,
        }
    }
}

pub trait ShipmentInvoiceApi {
    fn get_invoice_status(&self, shipment_id: &str) -> Box<Future<Item = ::models::GetInvoiceStatusResponse, Error = Error<serde_json::Value>>>;
    fn get_shipment_details(&self, shipment_id: &str) -> Box<Future<Item = ::models::GetShipmentDetailsResponse, Error = Error<serde_json::Value>>>;
    fn submit_invoice(&self, shipment_id: &str, body: ::models::SubmitInvoiceRequest) -> Box<Future<Item = ::models::SubmitInvoiceResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ShipmentInvoiceApi for ShipmentInvoiceApiClient<C> {
    fn get_invoice_status(&self, shipment_id: &str) -> Box<Future<Item = ::models::GetInvoiceStatusResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/fba/outbound/brazil/v0/shipments/{shipmentId}/invoice/status?{}", configuration.base_path, query_string, shipmentId=shipment_id);

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
                let parsed: Result<::models::GetInvoiceStatusResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_shipment_details(&self, shipment_id: &str) -> Box<Future<Item = ::models::GetShipmentDetailsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/fba/outbound/brazil/v0/shipments/{shipmentId}?{}", configuration.base_path, query_string, shipmentId=shipment_id);

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
                let parsed: Result<::models::GetShipmentDetailsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn submit_invoice(&self, shipment_id: &str, body: ::models::SubmitInvoiceRequest) -> Box<Future<Item = ::models::SubmitInvoiceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/fba/outbound/brazil/v0/shipments/{shipmentId}/invoice?{}", configuration.base_path, query_string, shipmentId=shipment_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }



        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

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
                let parsed: Result<::models::SubmitInvoiceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
