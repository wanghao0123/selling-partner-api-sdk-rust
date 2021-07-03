# Carton

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carton_identifiers** | [**Vec<::models::ContainerIdentification>**](ContainerIdentification.md) | A list of carton identifiers. | [optional] [default to null]
**carton_sequence_number** | **String** | Carton sequence number for the carton. The first carton will be 001, the second 002, and so on. This number is used as a reference to refer to this carton from the pallet level. | [default to null]
**dimensions** | [***::models::Dimensions**](Dimensions.md) |  | [optional] [default to null]
**weight** | [***::models::Weight**](Weight.md) |  | [optional] [default to null]
**tracking_number** | **String** | This is required to be provided for every carton in the small parcel shipments. | [optional] [default to null]
**items** | [**Vec<::models::ContainerItem>**](ContainerItem.md) | A list of container item details. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


