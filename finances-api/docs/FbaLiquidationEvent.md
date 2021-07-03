# FbaLiquidationEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**original_removal_order_id** | **String** | The identifier for the original removal order. | [optional] [default to null]
**liquidation_proceeds_amount** | [***::models::Currency**](Currency.md) | The amount paid by the liquidator for the seller&#39;s inventory. The seller receives this amount minus LiquidationFeeAmount. | [optional] [default to null]
**liquidation_fee_amount** | [***::models::Currency**](Currency.md) | The fee charged to the seller by Amazon for liquidating the seller&#39;s FBA inventory. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


