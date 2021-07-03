# OrderItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | [default to null]
**buyer_product_identifier** | **String** | Buyer&#39;s standard identification number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. | [optional] [default to null]
**title** | **String** | Title for the item. | [optional] [default to null]
**ordered_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Item quantity ordered. | [default to null]
**scheduled_delivery_shipment** | [***::models::ScheduledDeliveryShipment**](ScheduledDeliveryShipment.md) | Details for the scheduled delivery shipment. | [optional] [default to null]
**gift_details** | [***::models::GiftDetails**](GiftDetails.md) | Gift message and wrapId details. | [optional] [default to null]
**net_price** | [***::models::Money**](Money.md) | Net price (before tax) to vendor with currency details. | [default to null]
**tax_details** | [***Value**](Value.md) | Total tax details for the line item. | [optional] [default to null]
**total_price** | [***::models::Money**](Money.md) | The price to Amazon each (cost). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


