# OrderItemStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | [default to null]
**buyer_product_identifier** | **String** | Buyer&#39;s Standard Identification Number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. | [optional] [default to null]
**net_cost** | [***::models::Money**](Money.md) | The net cost to Amazon each (cost). | [optional] [default to null]
**list_price** | [***::models::Money**](Money.md) | The list Price to Amazon each (list). | [optional] [default to null]
**ordered_quantity** | [***Value**](Value.md) | Ordered quantity information. | [optional] [default to null]
**acknowledgement_status** | [***Value**](Value.md) | Acknowledgement status information. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


