# Container

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_type** | **String** | The type of physical container being used. (always &#39;PACKAGE&#39;) | [optional] [default to null]
**container_reference_id** | [***::models::ContainerReferenceId**](ContainerReferenceId.md) |  | [default to null]
**value** | [***::models::Currency**](Currency.md) | The total value of all items in the container. | [default to null]
**dimensions** | [***::models::Dimensions**](Dimensions.md) | The length, width, height, and weight of the container. | [default to null]
**items** | [**Vec<::models::ContainerItem>**](ContainerItem.md) | A list of the items in the container. | [default to null]
**weight** | [***::models::Weight**](Weight.md) | The weight of the container. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


