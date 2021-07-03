# PaymentTerms

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The payment term type for the invoice. | [optional] [default to null]
**discount_percent** | [***::models::Decimal**](Decimal.md) | The discount percent value, which is good until the discount due date. | [optional] [default to null]
**discount_due_days** | **f32** | The number of calendar days from the Base date (Invoice date) until the discount is no longer valid. | [optional] [default to null]
**net_due_days** | **f32** | The number of calendar days from the base date (invoice date) until the total amount on the invoice is due. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


