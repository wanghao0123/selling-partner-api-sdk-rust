# \VendorShippingLabelsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shipping_label**](VendorShippingLabelsApi.md#get_shipping_label) | **Get** /vendor/directFulfillment/shipping/v1/shippingLabels/{purchaseOrderNumber} | 
[**get_shipping_labels**](VendorShippingLabelsApi.md#get_shipping_labels) | **Get** /vendor/directFulfillment/shipping/v1/shippingLabels | 
[**submit_shipping_label_request**](VendorShippingLabelsApi.md#submit_shipping_label_request) | **Post** /vendor/directFulfillment/shipping/v1/shippingLabels | 


# **get_shipping_label**
> ::models::GetShippingLabelResponse get_shipping_label(purchase_order_number)


Returns a shipping label for the purchaseOrderNumber that you specify.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **purchase_order_number** | **String**| The purchase order number for which you want to return the shipping label. It should be the same purchaseOrderNumber as received in the order. | 

### Return type

[**::models::GetShippingLabelResponse**](GetShippingLabelResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_shipping_labels**
> ::models::GetShippingLabelListResponse get_shipping_labels(created_after, created_before, optional)


Returns a list of shipping labels created during the time frame that you specify. You define that time frame using the createdAfter and createdBefore parameters. You must use both of these parameters. The date range to search must not be more than 7 days.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **created_after** | **String**| Shipping labels that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
  **created_before** | **String**| Shipping labels that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **created_after** | **String**| Shipping labels that became available after this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **created_before** | **String**| Shipping labels that became available before this date and time will be included in the result. Must be in ISO-8601 date/time format. | 
 **ship_from_party_id** | **String**| The vendor warehouseId for order fulfillment. If not specified, the result will contain orders for all warehouses. | 
 **limit** | **i32**| The limit to the number of records returned. | 
 **sort_order** | **String**| Sort ASC or DESC by order creation date. | [default to ASC]
 **next_token** | **String**| Used for pagination when there are more ship labels than the specified result size limit. The token value is returned in the previous API call. | 

### Return type

[**::models::GetShippingLabelListResponse**](GetShippingLabelListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **submit_shipping_label_request**
> ::models::SubmitShippingLabelsResponse submit_shipping_label_request(body)


Creates a shipping label for a purchase order and returns a transactionId for reference.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**SubmitShippingLabelsRequest**](SubmitShippingLabelsRequest.md)|  | 

### Return type

[**::models::SubmitShippingLabelsResponse**](SubmitShippingLabelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

