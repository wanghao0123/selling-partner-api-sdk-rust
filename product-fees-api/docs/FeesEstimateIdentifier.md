# FeesEstimateIdentifier

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. | [optional] [default to null]
**seller_id** | **String** | The seller identifier. | [optional] [default to null]
**id_type** | **String** | The type of item identifier specified. | [optional] [default to null]
**id_value** | **String** | The item identifier. | [optional] [default to null]
**is_amazon_fulfilled** | **bool** | When true, the offer is fulfilled by Amazon. | [optional] [default to null]
**price_to_estimate_fees** | [***::models::PriceToEstimateFees**](PriceToEstimateFees.md) | The item price on which the fee estimate is based. | [optional] [default to null]
**seller_input_identifier** | **String** | A unique identifier provided by the caller to track this request. | [optional] [default to null]
**optional_fulfillment_program** | [***::models::OptionalFulfillmentProgram**](OptionalFulfillmentProgram.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


