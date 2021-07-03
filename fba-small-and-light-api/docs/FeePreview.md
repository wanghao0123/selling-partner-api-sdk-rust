# FeePreview

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) value used to identify the item. | [optional] [default to null]
**price** | [***::models::MoneyType**](MoneyType.md) | The price that the seller plans to charge for the item. | [optional] [default to null]
**fee_breakdown** | [**Vec<::models::FeeLineItem>**](FeeLineItem.md) | A list of the Small and Light fees for the item. | [optional] [default to null]
**total_fees** | [***::models::MoneyType**](MoneyType.md) | The total fees charged if the item participated in the Small and Light program. | [optional] [default to null]
**errors** | [***::models::ErrorList**](ErrorList.md) | One or more unexpected errors occurred during the getSmallAndLightFeePreview operation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


