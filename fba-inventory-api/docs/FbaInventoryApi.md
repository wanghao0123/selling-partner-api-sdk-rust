# \FbaInventoryApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_inventory_summaries**](FbaInventoryApi.md#get_inventory_summaries) | **Get** /fba/inventory/v1/summaries | 


# **get_inventory_summaries**
> ::models::GetInventorySummariesResponse get_inventory_summaries(granularity_type, granularity_id, marketplace_ids, optional)


Returns a list of inventory summaries. The summaries returned depend on the presence or absence of the startDateTime and sellerSkus parameters:  - All inventory summaries with available details are returned when the startDateTime and sellerSkus parameters are omitted. - When startDateTime is provided, the operation returns inventory summaries that have had changes after the date and time specified. The sellerSkus parameter is ignored. - When the sellerSkus parameter is provided, the operation returns inventory summaries for only the specified sellerSkus.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 90 | 150 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **granularity_type** | **String**| The granularity type for the inventory aggregation level. | 
  **granularity_id** | **String**| The granularity ID for the inventory aggregation level. | 
  **marketplace_ids** | [**Vec&lt;String&gt;**](String.md)| The marketplace ID for the marketplace for which to return inventory summaries. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **granularity_type** | **String**| The granularity type for the inventory aggregation level. | 
 **granularity_id** | **String**| The granularity ID for the inventory aggregation level. | 
 **marketplace_ids** | [**Vec&lt;String&gt;**](String.md)| The marketplace ID for the marketplace for which to return inventory summaries. | 
 **details** | **bool**| true to return inventory summaries with additional summarized inventory details and quantities. Otherwise, returns inventory summaries only (default value). | [default to false]
 **start_date_time** | **String**| A start date and time in ISO8601 format. If specified, all inventory summaries that have changed since then are returned. You must specify a date and time that is no earlier than 18 months prior to the date and time when you call the API. Note: Changes in inboundWorkingQuantity, inboundShippedQuantity and inboundReceivingQuantity are not detected. | 
 **seller_skus** | [**Vec&lt;String&gt;**](String.md)| A list of seller SKUs for which to return inventory summaries. You may specify up to 50 SKUs. | 
 **next_token** | **String**| String token returned in the response of your previous request. | 

### Return type

[**::models::GetInventorySummariesResponse**](GetInventorySummariesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

