# DebtRecoveryEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**debt_recovery_type** | **String** | The debt recovery type.  Possible values:  * DebtPayment  * DebtPaymentFailure  *DebtAdjustment | [optional] [default to null]
**recovery_amount** | [***::models::Currency**](Currency.md) | The amount applied for recovery. | [optional] [default to null]
**over_payment_credit** | [***::models::Currency**](Currency.md) | The amount returned for overpayment. | [optional] [default to null]
**debt_recovery_item_list** | [***::models::DebtRecoveryItemList**](DebtRecoveryItemList.md) |  | [optional] [default to null]
**charge_instrument_list** | [***::models::ChargeInstrumentList**](ChargeInstrumentList.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


