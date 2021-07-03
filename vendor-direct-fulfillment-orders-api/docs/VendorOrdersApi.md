# \VendorOrdersApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_order**](VendorOrdersApi.md#get_order) | **Get** /vendor/directFulfillment/orders/v1/purchaseOrders/{purchaseOrderNumber} | 
[**get_orders**](VendorOrdersApi.md#get_orders) | **Get** /vendor/directFulfillment/orders/v1/purchaseOrders | 
[**submit_acknowledgement**](VendorOrdersApi.md#submit_acknowledgement) | **Post** /vendor/directFulfillment/orders/v1/acknowledgements | 


# **get_order**
> ::models::GetOrderResponse get_order(purchase_order_number)


Returns purchase order information for the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **purchase_order_number** | **String**| The order identifier for the purchase order that you want. Formatting Notes: alpha-numeric code. | 

### Return type

[**::models::GetOrderResponse**](GetOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_orders**
> ::models::GetOrdersResponse get_orders(created_after, created_before, optional)


Returns a list of purchase orders created during the time frame that you specify. You define the time frame using the createdAfter and createdBefore parameters. You must use both parameters. You can choose to get only the purchase order numbers by setting the includeDetails parameter to false. In that case, the operation returns a list of purchase order numbers. You can then call the getOrder operation to return the details of a specific order.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **created_after** | **String**| Purchase orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
  **created_before** | **String**| Purchase orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **created_after** | **String**| Purchase orders that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **created_before** | **String**| Purchase orders that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **ship_from_party_id** | **String**| The vendor warehouse identifier for the fulfillment warehouse. If not specified, the result will contain orders for all warehouses. | 
 **status** | **String**| Returns only the purchase orders that match the specified status. If not specified, the result will contain orders that match any status. | 
 **limit** | **i64**| The limit to the number of purchase orders returned. | 
 **sort_order** | **String**| Sort the list in ascending or descending order by order creation date. | 
 **next_token** | **String**| Used for pagination when there are more orders than the specified result size limit. The token value is returned in the previous API call. | 
 **include_details** | **String**| When true, returns the complete purchase order details. Otherwise, only purchase order numbers are returned. | [default to true]

### Return type

[**::models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **submit_acknowledgement**
> ::models::SubmitAcknowledgementResponse submit_acknowledgement(body)


Submits acknowledgements for one or more purchase orders.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SubmitAcknowledgementRequest**](SubmitAcknowledgementRequest.md)|  | 

### Return type

[**::models::SubmitAcknowledgementResponse**](SubmitAcknowledgementResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

