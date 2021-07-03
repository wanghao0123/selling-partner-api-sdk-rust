# \CatalogApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_item**](CatalogApi.md#get_catalog_item) | **Get** /catalog/v0/items/{asin} | 
[**list_catalog_categories**](CatalogApi.md#list_catalog_categories) | **Get** /catalog/v0/categories | 
[**list_catalog_items**](CatalogApi.md#list_catalog_items) | **Get** /catalog/v0/items | 


# **get_catalog_item**
> ::models::GetCatalogItemResponse get_catalog_item(marketplace_id, asin)


Returns a specified item and its attributes.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 2 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| A marketplace identifier. Specifies the marketplace for the item. | 
  **asin** | **String**| The Amazon Standard Identification Number (ASIN) of the item. | 

### Return type

[**::models::GetCatalogItemResponse**](GetCatalogItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_catalog_categories**
> ::models::ListCatalogCategoriesResponse list_catalog_categories(marketplace_id, optional)


Returns the parent categories to which an item belongs, based on the specified ASIN or SellerSKU.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 1 | 40 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| A marketplace identifier. Specifies the marketplace for the item. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **marketplace_id** | **String**| A marketplace identifier. Specifies the marketplace for the item. | 
 **ASIN** | **String**| The Amazon Standard Identification Number (ASIN) of the item. | 
 **seller_sku** | **String**| Used to identify items in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit. | 

### Return type

[**::models::ListCatalogCategoriesResponse**](ListCatalogCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_catalog_items**
> ::models::ListCatalogItemsResponse list_catalog_items(marketplace_id, optional)


Returns a list of items and their attributes, based on a search query or item identifiers that you specify. When based on a search query, provide the Query parameter and optionally, the QueryContextId parameter. When based on item identifiers, provide a single appropriate parameter based on the identifier type, and specify the associated item value.  MarketplaceId is always required. At least one of Query, SellerSKU, UPC, EAN, ISBN, JAN is also required.  This operation returns a maximum of ten products and does not return non-buyable products.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 6 | 40 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_id** | **String**| A marketplace identifier. Specifies the marketplace for which items are returned. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **marketplace_id** | **String**| A marketplace identifier. Specifies the marketplace for which items are returned. | 
 **query** | **String**| Keyword(s) to use to search for items in the catalog. Example: &#39;harry potter books&#39;. | 
 **query_context_id** | **String**| An identifier for the context within which the given search will be performed. A marketplace might provide mechanisms for constraining a search to a subset of potential items. For example, the retail marketplace allows queries to be constrained to a specific category. The QueryContextId parameter specifies such a subset. If it is omitted, the search will be performed using the default context for the marketplace, which will typically contain the largest set of items. | 
 **seller_sku** | **String**| Used to identify an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit. | 
 **UPC** | **String**| A 12-digit bar code used for retail packaging. | 
 **EAN** | **String**| A European article number that uniquely identifies the catalog item, manufacturer, and its attributes. | 
 **ISBN** | **String**| The unique commercial book identifier used to identify books internationally. | 
 **JAN** | **String**| A Japanese article number that uniquely identifies the product, manufacturer, and its attributes. | 

### Return type

[**::models::ListCatalogItemsResponse**](ListCatalogItemsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

