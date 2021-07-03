# FeesEstimateRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. | [default to null]
**is_amazon_fulfilled** | **bool** | When true, the offer is fulfilled by Amazon. | [optional] [default to null]
**price_to_estimate_fees** | [***::models::PriceToEstimateFees**](PriceToEstimateFees.md) | The product price that the fee estimate is based on. | [default to null]
**identifier** | **String** | A unique identifier provided by the caller to track this request. | [default to null]
**optional_fulfillment_program** | [***::models::OptionalFulfillmentProgram**](OptionalFulfillmentProgram.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


