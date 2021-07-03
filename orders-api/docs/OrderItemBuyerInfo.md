# OrderItemBuyerInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_item_id** | **String** | An Amazon-defined order item identifier. | [default to null]
**buyer_customized_info** | [***::models::BuyerCustomizedInfoDetail**](BuyerCustomizedInfoDetail.md) | Buyer information for custom orders from the Amazon Custom program. | [optional] [default to null]
**gift_wrap_price** | [***::models::Money**](Money.md) | The gift wrap price of the item. | [optional] [default to null]
**gift_wrap_tax** | [***::models::Money**](Money.md) | The tax on the gift wrap price. | [optional] [default to null]
**gift_message_text** | **String** | A gift message provided by the buyer. | [optional] [default to null]
**gift_wrap_level** | **String** | The gift wrap level specified by the buyer. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


