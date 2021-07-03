# \FbaInboundApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_item_eligibility_preview**](FbaInboundApi.md#get_item_eligibility_preview) | **Get** /fba/inbound/v1/eligibility/itemPreview | 


# **get_item_eligibility_preview**
> ::models::GetItemEligibilityPreviewResponse get_item_eligibility_preview(asin, program, optional)


This operation gets an eligibility preview for an item that you specify. You can specify the type of eligibility preview that you want (INBOUND or COMMINGLING). For INBOUND previews, you can specify the marketplace in which you want to determine the item's eligibility.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **asin** | **String**| The ASIN of the item for which you want an eligibility preview. | 
  **program** | **String**| The program that you want to check eligibility against. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **asin** | **String**| The ASIN of the item for which you want an eligibility preview. | 
 **program** | **String**| The program that you want to check eligibility against. | 
 **marketplace_ids** | [**Vec&lt;String&gt;**](String.md)| The identifier for the marketplace in which you want to determine eligibility. Required only when program&#x3D;INBOUND. | 

### Return type

[**::models::GetItemEligibilityPreviewResponse**](GetItemEligibilityPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

