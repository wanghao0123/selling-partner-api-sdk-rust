# SellerDealPaymentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**deal_id** | **String** | The unique identifier of the deal. | [optional] [default to null]
**deal_description** | **String** | The internal description of the deal. | [optional] [default to null]
**event_type** | **String** | The type of event: SellerDealComplete. | [optional] [default to null]
**fee_type** | **String** | The type of fee: RunLightningDealFee. | [optional] [default to null]
**fee_amount** | [***::models::Currency**](Currency.md) | The monetary amount of the fee. | [optional] [default to null]
**tax_amount** | [***::models::Currency**](Currency.md) | The monetary amount of the tax applied. | [optional] [default to null]
**total_amount** | [***::models::Currency**](Currency.md) | The total monetary amount paid. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


