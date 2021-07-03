# Container

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_type** | **String** | The type of container. | [default to null]
**container_identifier** | **String** | The container identifier. | [default to null]
**tracking_number** | **String** | The tracking number. | [optional] [default to null]
**manifest_id** | **String** | The manifest identifier. | [optional] [default to null]
**manifest_date** | **String** | The date of the manifest. | [optional] [default to null]
**ship_method** | **String** | The shipment method. | [optional] [default to null]
**scac_code** | **String** | SCAC code required for NA VOC vendors only. | [optional] [default to null]
**carrier** | **String** | Carrier required for EU VOC vendors only. | [optional] [default to null]
**container_sequence_number** | **i32** | An integer that must be submitted for multi-box shipments only, where one item may come in separate packages. | [optional] [default to null]
**dimensions** | [***::models::Dimensions**](Dimensions.md) |  | [optional] [default to null]
**weight** | [***::models::Weight**](Weight.md) |  | [optional] [default to null]
**packed_items** | [**Vec<::models::PackedItem>**](PackedItem.md) | A list of packed items. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


