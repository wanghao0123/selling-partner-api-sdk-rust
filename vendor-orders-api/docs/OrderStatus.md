# OrderStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The buyer&#39;s purchase order number for this order. Formatting Notes: 8-character alpha-numeric code. | [default to null]
**purchase_order_status** | **String** | The status of the buyer&#39;s purchase order for this order. | [default to null]
**purchase_order_date** | **String** | The date the purchase order was placed. Must be in ISO-8601 date/time format. | [default to null]
**last_updated_date** | **String** | The date when the purchase order was last updated. Must be in ISO-8601 date/time format. | [optional] [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the selling party. | [default to null]
**ship_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the ship to party. | [default to null]
**item_status** | [***::models::ItemStatus**](ItemStatus.md) | Detailed order status. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


