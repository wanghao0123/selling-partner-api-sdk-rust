# TaxDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_type** | **String** | Type of the tax applied. | [default to null]
**tax_rate** | [***::models::Decimal**](Decimal.md) | Tax percentage applied. Percentage must be expressed in decimal. | [optional] [default to null]
**tax_amount** | [***::models::Money**](Money.md) | Total tax amount applied on invoice total or an item total. | [default to null]
**taxable_amount** | [***::models::Money**](Money.md) | This field will contain the invoice amount that is taxable at the rate specified in the tax rate field. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


