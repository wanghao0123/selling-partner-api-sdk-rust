# Rust API client for swagger

The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: v0
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [https://sellercentral.amazon.com/gp/mws/contactus.html](https://sellercentral.amazon.com/gp/mws/contactus.html)

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ShipmentInvoiceApi* | [**get_invoice_status**](docs/ShipmentInvoiceApi.md#get_invoice_status) | **Get** /fba/outbound/brazil/v0/shipments/{shipmentId}/invoice/status | 
*ShipmentInvoiceApi* | [**get_shipment_details**](docs/ShipmentInvoiceApi.md#get_shipment_details) | **Get** /fba/outbound/brazil/v0/shipments/{shipmentId} | 
*ShipmentInvoiceApi* | [**submit_invoice**](docs/ShipmentInvoiceApi.md#submit_invoice) | **Post** /fba/outbound/brazil/v0/shipments/{shipmentId}/invoice | 


## Documentation For Models

 - [Address](docs/Address.md)
 - [AddressTypeEnum](docs/AddressTypeEnum.md)
 - [Blob](docs/Blob.md)
 - [BuyerTaxInfo](docs/BuyerTaxInfo.md)
 - [Error](docs/Error.md)
 - [ErrorList](docs/ErrorList.md)
 - [GetInvoiceStatusResponse](docs/GetInvoiceStatusResponse.md)
 - [GetShipmentDetailsResponse](docs/GetShipmentDetailsResponse.md)
 - [Money](docs/Money.md)
 - [PaymentMethodDetailItemList](docs/PaymentMethodDetailItemList.md)
 - [SerialNumbersList](docs/SerialNumbersList.md)
 - [ShipmentDetail](docs/ShipmentDetail.md)
 - [ShipmentInvoiceStatus](docs/ShipmentInvoiceStatus.md)
 - [ShipmentInvoiceStatusInfo](docs/ShipmentInvoiceStatusInfo.md)
 - [ShipmentInvoiceStatusResponse](docs/ShipmentInvoiceStatusResponse.md)
 - [ShipmentItem](docs/ShipmentItem.md)
 - [ShipmentItems](docs/ShipmentItems.md)
 - [SubmitInvoiceRequest](docs/SubmitInvoiceRequest.md)
 - [SubmitInvoiceResponse](docs/SubmitInvoiceResponse.md)
 - [TaxClassification](docs/TaxClassification.md)
 - [TaxClassificationList](docs/TaxClassificationList.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



