# GetFeatureInventoryResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The requested marketplace. | [default to null]
**feature_name** | **String** | The name of the feature. | [default to null]
**next_token** | **String** | When present and not empty, pass this string token in the next request to return the next response page. | [optional] [default to null]
**feature_skus** | [**Vec<::models::FeatureSku>**](FeatureSku.md) | An array of SKUs eligible for this feature and the quantity available. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


