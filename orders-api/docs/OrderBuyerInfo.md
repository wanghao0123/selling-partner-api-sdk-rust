# OrderBuyerInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | [default to null]
**buyer_email** | **String** | The anonymized email address of the buyer. | [optional] [default to null]
**buyer_name** | **String** | The name of the buyer. | [optional] [default to null]
**buyer_county** | **String** | The county of the buyer. | [optional] [default to null]
**buyer_tax_info** | [***::models::BuyerTaxInfo**](BuyerTaxInfo.md) | Tax information about the buyer. | [optional] [default to null]
**purchase_order_number** | **String** | The purchase order (PO) number entered by the buyer at checkout. Returned only for orders where the buyer entered a PO number at checkout. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


