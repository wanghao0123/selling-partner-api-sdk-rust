# ShipmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [optional] [default to null]
**seller_sku** | **String** | The seller SKU of the item. | [optional] [default to null]
**order_item_id** | **String** | The Amazon-defined identifier for the order item. | [optional] [default to null]
**title** | **String** | The name of the item. | [optional] [default to null]
**quantity_ordered** | **f32** | The number of items ordered. | [optional] [default to null]
**item_price** | [***::models::Money**](Money.md) | The selling price of the item multiplied by the quantity ordered. Note that ItemPrice excludes ShippingPrice and GiftWrapPrice. | [optional] [default to null]
**shipping_price** | [***::models::Money**](Money.md) | The shipping price of the item. | [optional] [default to null]
**gift_wrap_price** | [***::models::Money**](Money.md) | The gift wrap price of the item. | [optional] [default to null]
**shipping_discount** | [***::models::Money**](Money.md) | The discount on the shipping price. | [optional] [default to null]
**promotion_discount** | [***::models::Money**](Money.md) | The total of all promotional discounts in the offer. | [optional] [default to null]
**serial_numbers** | [***::models::SerialNumbersList**](SerialNumbersList.md) | The list of serial numbers. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


