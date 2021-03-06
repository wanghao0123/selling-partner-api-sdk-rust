# Rust API client for swagger

With the FBA Inbound Eligibility API, you can build applications that let sellers get eligibility previews for items before shipping them to Amazon's fulfillment centers. With this API you can find out if an item is eligible for inbound shipment to Amazon's fulfillment centers in a specific marketplace. You can also find out if an item is eligible for using the manufacturer barcode for FBA inventory tracking. Sellers can use this information to inform their decisions about which items to ship Amazon's fulfillment centers.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*FbaInboundApi* | [**get_item_eligibility_preview**](docs/FbaInboundApi.md#get_item_eligibility_preview) | **Get** /fba/inbound/v1/eligibility/itemPreview | 


## Documentation For Models

 - [Error](docs/Error.md)
 - [ErrorList](docs/ErrorList.md)
 - [GetItemEligibilityPreviewResponse](docs/GetItemEligibilityPreviewResponse.md)
 - [ItemEligibilityPreview](docs/ItemEligibilityPreview.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



