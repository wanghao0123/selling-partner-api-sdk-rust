/* 
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
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

pub struct VendorInvoiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> VendorInvoiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> VendorInvoiceApiClient<C> {
        VendorInvoiceApiClient {
            configuration: configuration,
        }
    }
}

pub trait VendorInvoiceApi {
    fn submit_invoice(&self, body: ::models::SubmitInvoiceRequest) -> Box<Future<Item = ::models::SubmitInvoiceResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>VendorInvoiceApi for VendorInvoiceApiClient<C> {
    fn submit_invoice(&self, body: ::models::SubmitInvoiceRequest) -> Box<Future<Item = ::models::SubmitInvoiceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/vendor/directFulfillment/payments/v1/invoices?{}", configuration.base_path, query_string);

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
