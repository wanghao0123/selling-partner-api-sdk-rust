# \SellersApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_marketplace_participations**](SellersApi.md#get_marketplace_participations) | **Get** /sellers/v1/marketplaceParticipations | 


# **get_marketplace_participations**
> ::models::GetMarketplaceParticipationsResponse get_marketplace_participations()


Returns a list of marketplaces that the seller submitting the request can sell in and information about the seller's participation in those marketplaces.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | .016 | 15 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GetMarketplaceParticipationsResponse**](GetMarketplaceParticipationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

