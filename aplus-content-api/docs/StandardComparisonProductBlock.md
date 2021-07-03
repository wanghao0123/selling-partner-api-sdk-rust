# StandardComparisonProductBlock

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**position** | **i32** | The rank or index of this comparison product block within the module. Different blocks cannot occupy the same position within a single module. | [default to null]
**image** | [***::models::ImageComponent**](ImageComponent.md) |  | [optional] [default to null]
**title** | **String** | The comparison product title. | [optional] [default to null]
**asin** | [***::models::Asin**](Asin.md) |  | [optional] [default to null]
**highlight** | **bool** | Determines whether this block of content is visually highlighted. | [optional] [default to null]
**metrics** | [**Vec<::models::PlainTextItem>**](PlainTextItem.md) | Comparison metrics for the product. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


