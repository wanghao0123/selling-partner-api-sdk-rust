# Package

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_identifier** | **String** | Package identifier for the package. The first package will be 001, the second 002, and so on. This number is used as a reference to refer to this package from the pallet level. | [default to null]
**tracking_number** | **String** | This is required to be provided for every package in the small parcel shipments. | [optional] [default to null]
**manifest_id** | **String** | Carrier manifest Id (Applicable for LTL shipments). | [optional] [default to null]
**manifest_date** | **String** | Carrier manifest Date (Applicable for LTL shipments). | [optional] [default to null]
**ship_method** | **String** | Shipment method. | [optional] [default to null]
**weight** | [***::models::Weight**](Weight.md) |  | [default to null]
**dimensions** | [***::models::Dimensions**](Dimensions.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


