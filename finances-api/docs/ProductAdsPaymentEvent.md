# ProductAdsPaymentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**transaction_type** | **String** | Indicates if the transaction is for a charge or a refund.  Possible values:  * charge - Charge  * refund - Refund | [optional] [default to null]
**invoice_id** | **String** | Identifier for the invoice that the transaction appears in. | [optional] [default to null]
**base_value** | [***::models::Currency**](Currency.md) | Base amount of the transaction, before tax. | [optional] [default to null]
**tax_value** | [***::models::Currency**](Currency.md) | Tax amount of the transaction. | [optional] [default to null]
**transaction_value** | [***::models::Currency**](Currency.md) | The total amount of the transaction. Equal to baseValue + taxValue. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


