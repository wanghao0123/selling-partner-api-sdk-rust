# GetFeatureSkuResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The requested marketplace. | [default to null]
**feature_name** | **String** | The name of the feature. | [default to null]
**is_eligible** | **bool** | When true, the seller SKU is eligible for the requested feature. | [default to null]
**ineligible_reasons** | **Vec<String>** | A list of one or more reasons that the seller SKU is ineligibile for the feature.  Possible values: * MERCHANT_NOT_ENROLLED - The merchant isn&#39;t enrolled for the feature. * SKU_NOT_ELIGIBLE - The SKU doesn&#39;t reside in a warehouse that supports the feature. * INVALID_SKU - There is an issue with the SKU provided. | [optional] [default to null]
**sku_info** | [***::models::FeatureSku**](FeatureSku.md) | Information about the SKU, including the count available, identifiers, and a list of overlapping SKUs that share the same inventory pool. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


