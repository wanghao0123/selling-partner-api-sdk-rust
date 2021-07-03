# CodSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_cod_required** | **bool** | When true, this fulfillment order requires a COD (Cash On Delivery) payment. | [default to null]
**cod_charge** | [***::models::Money**](Money.md) | The amount of the COD charge to be collected from the recipient for a COD order. | [optional] [default to null]
**cod_charge_tax** | [***::models::Money**](Money.md) | The amount of the tax on the COD charge to be collected from the recipient for a COD order. | [optional] [default to null]
**shipping_charge** | [***::models::Money**](Money.md) | The amount of the tax on the COD charge to be collected from the recipient for a COD order. | [optional] [default to null]
**shipping_charge_tax** | [***::models::Money**](Money.md) | The amount of the tax on the shipping charge to be collected from the recipient for a COD order. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


