# Item

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Item sequence number for the item. The first item will be 001, the second 002, and so on. This number is used as a reference to refer to this item from the carton or pallet level. | [default to null]
**amazon_product_identifier** | **String** | Amazon Standard Identification Number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. Should be the same as was sent in the purchase order. | [optional] [default to null]
**shipped_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Total item quantity shipped in this shipment. | [default to null]
**item_details** | [***::models::ItemDetails**](ItemDetails.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


