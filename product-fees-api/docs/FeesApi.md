# \FeesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_fees_estimate_for_asin**](FeesApi.md#get_my_fees_estimate_for_asin) | **Post** /products/fees/v0/items/{Asin}/feesEstimate | 
[**get_my_fees_estimate_for_sku**](FeesApi.md#get_my_fees_estimate_for_sku) | **Post** /products/fees/v0/listings/{SellerSKU}/feesEstimate | 


# **get_my_fees_estimate_for_asin**
> ::models::GetMyFeesEstimateResponse get_my_fees_estimate_for_asin(body, asin)


Returns the estimated fees for the item indicated by the specified Asin in the marketplace specified in the request body.  You can call getMyFeesEstimateForASIN for an item on behalf of a seller before the seller sets the item's price. They can then take estimated fees into account. With each product fees request, you must include an original identifier. This identifier is included in the fees estimate so you can correlate a fees estimate with the original request.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**GetMyFeesEstimateRequest**](GetMyFeesEstimateRequest.md)|  | 
  **asin** | **String**| The Amazon Standard Identification Number (ASIN) of the item. | 

### Return type

[**::models::GetMyFeesEstimateResponse**](GetMyFeesEstimateResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_my_fees_estimate_for_sku**
> ::models::GetMyFeesEstimateResponse get_my_fees_estimate_for_sku(body, seller_sku)


Returns the estimated fees for the item indicated by the specified seller SKU in the marketplace specified in the request body.  You can call getMyFeesEstimateForSKU for an item on behalf of a seller before the seller sets the item's price. They can then take estimated fees into account. With each fees estimate request, you must include an original identifier. This identifier is included in the fees estimate so you can correlate a fees estimate with the original request.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**GetMyFeesEstimateRequest**](GetMyFeesEstimateRequest.md)|  | 
  **seller_sku** | **String**| Used to identify an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit. | 

### Return type

[**::models::GetMyFeesEstimateResponse**](GetMyFeesEstimateResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

