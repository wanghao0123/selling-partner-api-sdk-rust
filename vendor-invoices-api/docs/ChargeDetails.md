# ChargeDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | Type of the charge applied. | [default to null]
**description** | **String** | Description of the charge. | [optional] [default to null]
**charge_amount** | [***::models::Money**](Money.md) | Total monetary amount related to this charge. | [default to null]
**tax_details** | [**Vec<::models::TaxDetails>**](TaxDetails.md) | Tax amount details applied on this charge. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


