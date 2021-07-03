# OrderAcknowledgementItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Line item sequence number for the item. | [optional] [default to null]
**amazon_product_identifier** | **String** | Amazon Standard Identification Number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. Should be the same as was sent in the purchase order. | [optional] [default to null]
**ordered_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | The quantity of this item ordered. | [default to null]
**net_cost** | [***::models::Money**](Money.md) | The cost to Amazon, which should match the cost on the invoice. This is a required field. If this is left blank the file will reject in Amazon systems. Price information should not be zero or negative. Indicates a net unit price. | [default to null]
**list_price** | [***::models::Money**](Money.md) | The list price. This is required only if a vendor sells books with a list price. | [optional] [default to null]
**discount_multiplier** | **String** | The discount multiplier that should be applied to the price if a vendor sells books with a list price. This is a multiplier factor to arrive at a final discounted price. A multiplier of .90 would be the factor if a 10% discount is given. | [optional] [default to null]
**item_acknowledgements** | [**Vec<::models::OrderItemAcknowledgement>**](OrderItemAcknowledgement.md) | This is used to indicate acknowledged quantity. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


