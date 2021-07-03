# \FbaOutboundApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_fulfillment_order**](FbaOutboundApi.md#cancel_fulfillment_order) | **Put** /fba/outbound/2020-07-01/fulfillmentOrders/{sellerFulfillmentOrderId}/cancel | 
[**create_fulfillment_order**](FbaOutboundApi.md#create_fulfillment_order) | **Post** /fba/outbound/2020-07-01/fulfillmentOrders | 
[**create_fulfillment_return**](FbaOutboundApi.md#create_fulfillment_return) | **Put** /fba/outbound/2020-07-01/fulfillmentOrders/{sellerFulfillmentOrderId}/return | 
[**get_feature_inventory**](FbaOutboundApi.md#get_feature_inventory) | **Get** /fba/outbound/2020-07-01/features/inventory/{featureName} | 
[**get_feature_sku**](FbaOutboundApi.md#get_feature_sku) | **Get** /fba/outbound/2020-07-01/features/inventory/{featureName}/{sellerSku} | 
[**get_features**](FbaOutboundApi.md#get_features) | **Get** /fba/outbound/2020-07-01/features | 
[**get_fulfillment_order**](FbaOutboundApi.md#get_fulfillment_order) | **Get** /fba/outbound/2020-07-01/fulfillmentOrders/{sellerFulfillmentOrderId} | 
[**get_fulfillment_preview**](FbaOutboundApi.md#get_fulfillment_preview) | **Post** /fba/outbound/2020-07-01/fulfillmentOrders/preview | 
[**get_package_tracking_details**](FbaOutboundApi.md#get_package_tracking_details) | **Get** /fba/outbound/2020-07-01/tracking | 
[**list_all_fulfillment_orders**](FbaOutboundApi.md#list_all_fulfillment_orders) | **Get** /fba/outbound/2020-07-01/fulfillmentOrders | 
[**list_return_reason_codes**](FbaOutboundApi.md#list_return_reason_codes) | **Get** /fba/outbound/2020-07-01/returnReasonCodes | 
[**update_fulfillment_order**](FbaOutboundApi.md#update_fulfillment_order) | **Put** /fba/outbound/2020-07-01/fulfillmentOrders/{sellerFulfillmentOrderId} | 


# **cancel_fulfillment_order**
> ::models::CancelFulfillmentOrderResponse cancel_fulfillment_order(seller_fulfillment_order_id)


Requests that Amazon stop attempting to fulfill the fulfillment order indicated by the specified order identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **seller_fulfillment_order_id** | **String**| The identifier assigned to the item by the seller when the fulfillment order was created. | 

### Return type

[**::models::CancelFulfillmentOrderResponse**](CancelFulfillmentOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_fulfillment_order**
> ::models::CreateFulfillmentOrderResponse create_fulfillment_order(body)


Requests that Amazon ship items from the seller's inventory in Amazon's fulfillment network to a destination address.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**CreateFulfillmentOrderRequest**](CreateFulfillmentOrderRequest.md)|  | 

### Return type

[**::models::CreateFulfillmentOrderResponse**](CreateFulfillmentOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_fulfillment_return**
> ::models::CreateFulfillmentReturnResponse create_fulfillment_return(body, seller_fulfillment_order_id)


Creates a fulfillment return.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**CreateFulfillmentReturnRequest**](CreateFulfillmentReturnRequest.md)|  | 
  **seller_fulfillment_order_id** | **String**| An identifier assigned by the seller to the fulfillment order at the time it was created. The seller uses their own records to find the correct SellerFulfillmentOrderId value based on the buyer&#39;s request to return items. | 

### Return type

[**::models::CreateFulfillmentReturnResponse**](CreateFulfillmentReturnResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_feature_inventory**
> ::models::GetFeatureInventoryResponse get_feature_inventory(marketplace_id, feature_name, optional)


Returns a list of inventory items that are eligible for the fulfillment feature you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| The marketplace for which to return a list of the inventory that is eligible for the specified feature. | 
  **feature_name** | **String**| The name of the feature for which to return a list of eligible inventory. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **marketplace_id** | **String**| The marketplace for which to return a list of the inventory that is eligible for the specified feature. | 
 **feature_name** | **String**| The name of the feature for which to return a list of eligible inventory. | 
 **next_token** | **String**| A string token returned in the response to your previous request that is used to return the next response page. A value of null will return the first page. | 

### Return type

[**::models::GetFeatureInventoryResponse**](GetFeatureInventoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_feature_sku**
> ::models::GetFeatureSkuResponse get_feature_sku(marketplace_id, feature_name, seller_sku)


Returns the number of items with the sellerSKU you specify that can have orders fulfilled using the specified feature. Note that if the sellerSKU isn't eligible, the response will contain an empty skuInfo object.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| The marketplace for which to return the count. | 
  **feature_name** | **String**| The name of the feature. | 
  **seller_sku** | **String**| Used to identify an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit. | 

### Return type

[**::models::GetFeatureSkuResponse**](GetFeatureSkuResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_features**
> ::models::GetFeaturesResponse get_features(marketplace_id)


Returns a list of features available for Multi-Channel Fulfillment orders in the marketplace you specify, and whether the seller for which you made the call is enrolled for each feature.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| The marketplace for which to return the list of features. | 

### Return type

[**::models::GetFeaturesResponse**](GetFeaturesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fulfillment_order**
> ::models::GetFulfillmentOrderResponse get_fulfillment_order(seller_fulfillment_order_id)


Returns the fulfillment order indicated by the specified order identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **seller_fulfillment_order_id** | **String**| The identifier assigned to the item by the seller when the fulfillment order was created. | 

### Return type

[**::models::GetFulfillmentOrderResponse**](GetFulfillmentOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fulfillment_preview**
> ::models::GetFulfillmentPreviewResponse get_fulfillment_preview(body)


Returns a list of fulfillment order previews based on shipping criteria that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**GetFulfillmentPreviewRequest**](GetFulfillmentPreviewRequest.md)|  | 

### Return type

[**::models::GetFulfillmentPreviewResponse**](GetFulfillmentPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_package_tracking_details**
> ::models::GetPackageTrackingDetailsResponse get_package_tracking_details(package_number)


Returns delivery tracking information for a package in an outbound shipment for a Multi-Channel Fulfillment order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **package_number** | **i32**| The unencrypted package identifier returned by the getFulfillmentOrder operation. | 

### Return type

[**::models::GetPackageTrackingDetailsResponse**](GetPackageTrackingDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_all_fulfillment_orders**
> ::models::ListAllFulfillmentOrdersResponse list_all_fulfillment_orders(optional)


Returns a list of fulfillment orders fulfilled after (or at) a specified date-time, or indicated by the next token parameter.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query_start_date** | **String**| A date used to select fulfillment orders that were last updated after (or at) a specified time. An update is defined as any change in fulfillment order status, including the creation of a new fulfillment order. | 
 **next_token** | **String**| A string token returned in the response to your previous request. | 

### Return type

[**::models::ListAllFulfillmentOrdersResponse**](ListAllFulfillmentOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_return_reason_codes**
> ::models::ListReturnReasonCodesResponse list_return_reason_codes(seller_sku, language, optional)


Returns a list of return reason codes for a seller SKU in a given marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **seller_sku** | **String**| The seller SKU for which return reason codes are required. | 
  **language** | **String**| The language that the TranslatedDescription property of the ReasonCodeDetails response object should be translated into. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **seller_sku** | **String**| The seller SKU for which return reason codes are required. | 
 **language** | **String**| The language that the TranslatedDescription property of the ReasonCodeDetails response object should be translated into. | 
 **marketplace_id** | **String**| The marketplace for which the seller wants return reason codes. | 
 **seller_fulfillment_order_id** | **String**| The identifier assigned to the item by the seller when the fulfillment order was created. The service uses this value to determine the marketplace for which the seller wants return reason codes. | 

### Return type

[**::models::ListReturnReasonCodesResponse**](ListReturnReasonCodesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_fulfillment_order**
> ::models::UpdateFulfillmentOrderResponse update_fulfillment_order(body, seller_fulfillment_order_id)


Updates and/or requests shipment for a fulfillment order with an order hold on it.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**UpdateFulfillmentOrderRequest**](UpdateFulfillmentOrderRequest.md)|  | 
  **seller_fulfillment_order_id** | **String**| The identifier assigned to the item by the seller when the fulfillment order was created. | 

### Return type

[**::models::UpdateFulfillmentOrderResponse**](UpdateFulfillmentOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

