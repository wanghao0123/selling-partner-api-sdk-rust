# OrderItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | [default to null]
**amazon_product_identifier** | **String** | Amazon Standard Identification Number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. | [optional] [default to null]
**ordered_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Item quantity ordered. | [default to null]
**is_back_order_allowed** | **bool** | When true, we will accept backorder confirmations for this item. | [default to null]
**net_cost** | [***::models::Money**](Money.md) | The price to Amazon each (cost). | [optional] [default to null]
**list_price** | [***::models::Money**](Money.md) | The price to Amazon each (list). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


