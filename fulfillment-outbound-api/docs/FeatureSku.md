# FeatureSku

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | Used to identify an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit. | [optional] [default to null]
**fn_sku** | **String** | The unique SKU used by Amazon&#39;s fulfillment network. | [optional] [default to null]
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [optional] [default to null]
**sku_count** | **f32** | The number of SKUs available for this service. | [optional] [default to null]
**overlapping_skus** | **Vec<String>** | Other seller SKUs that are shared across the same inventory. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


