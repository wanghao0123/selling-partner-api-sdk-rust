# \ShipmentInvoiceApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_invoice_status**](ShipmentInvoiceApi.md#get_invoice_status) | **Get** /fba/outbound/brazil/v0/shipments/{shipmentId}/invoice/status | 
[**get_shipment_details**](ShipmentInvoiceApi.md#get_shipment_details) | **Get** /fba/outbound/brazil/v0/shipments/{shipmentId} | 
[**submit_invoice**](ShipmentInvoiceApi.md#submit_invoice) | **Post** /fba/outbound/brazil/v0/shipments/{shipmentId}/invoice | 


# **get_invoice_status**
> ::models::GetInvoiceStatusResponse get_invoice_status(shipment_id)


Returns the invoice status for the shipment you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 1.133 | 25 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **shipment_id** | **String**| The shipment identifier for the shipment. | 

### Return type

[**::models::GetInvoiceStatusResponse**](GetInvoiceStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_shipment_details**
> ::models::GetShipmentDetailsResponse get_shipment_details(shipment_id)


Returns the shipment details required to issue an invoice for the specified shipment.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 1.133 | 25 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **shipment_id** | **String**| The identifier for the shipment. Get this value from the FBAOutboundShipmentStatus notification. For information about subscribing to notifications, see the [Notifications API Use Case Guide](https://github.com/amzn/selling-partner-api-docs/blob/main/guides/en-US/use-case-guides/notifications-api-use-case-guide/notifications-use-case-guide-v1.md). | 

### Return type

[**::models::GetShipmentDetailsResponse**](GetShipmentDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **submit_invoice**
> ::models::SubmitInvoiceResponse submit_invoice(shipment_id, body)


Submits a shipment invoice document for a given shipment.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 1.133 | 25 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **shipment_id** | **String**| The identifier for the shipment. | 
  **body** | [**SubmitInvoiceRequest**](SubmitInvoiceRequest.md)|  | 

### Return type

[**::models::SubmitInvoiceResponse**](SubmitInvoiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

