# \CustomerInvoicesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_customer_invoice**](CustomerInvoicesApi.md#get_customer_invoice) | **Get** /vendor/directFulfillment/shipping/v1/customerInvoices/{purchaseOrderNumber} | 
[**get_customer_invoices**](CustomerInvoicesApi.md#get_customer_invoices) | **Get** /vendor/directFulfillment/shipping/v1/customerInvoices | 


# **get_customer_invoice**
> ::models::GetCustomerInvoiceResponse get_customer_invoice(purchase_order_number)


Returns a customer invoice based on the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **purchase_order_number** | **String**| Purchase order number of the shipment for which to return the invoice. | 

### Return type

[**::models::GetCustomerInvoiceResponse**](GetCustomerInvoiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_customer_invoices**
> ::models::GetCustomerInvoicesResponse get_customer_invoices(created_after, created_before, optional)


Returns a list of customer invoices created during a time frame that you specify. You define the  time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must be no more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **created_after** | **String**| Orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
  **created_before** | **String**| Orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **created_after** | **String**| Orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **created_before** | **String**| Orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **ship_from_party_id** | **String**| The vendor warehouseId for order fulfillment. If not specified, the result will contain orders for all warehouses. | 
 **limit** | **i32**| The limit to the number of records returned | 
 **sort_order** | **String**| Sort ASC or DESC by order creation date. | 
 **next_token** | **String**| Used for pagination when there are more orders than the specified result size limit. The token value is returned in the previous API call. | 

### Return type

[**::models::GetCustomerInvoicesResponse**](GetCustomerInvoicesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

