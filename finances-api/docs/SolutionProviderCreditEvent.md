# SolutionProviderCreditEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_transaction_type** | **String** | The transaction type. | [optional] [default to null]
**seller_order_id** | **String** | A seller-defined identifier for an order. | [optional] [default to null]
**marketplace_id** | **String** | The identifier of the marketplace where the order was placed. | [optional] [default to null]
**marketplace_country_code** | **String** | The two-letter country code of the country associated with the marketplace where the order was placed. | [optional] [default to null]
**seller_id** | **String** | The Amazon-defined identifier of the seller. | [optional] [default to null]
**seller_store_name** | **String** | The store name where the payment event occurred. | [optional] [default to null]
**provider_id** | **String** | The Amazon-defined identifier of the solution provider. | [optional] [default to null]
**provider_store_name** | **String** | The store name where the payment event occurred. | [optional] [default to null]
**transaction_amount** | [***::models::Currency**](Currency.md) | The amount of the credit. | [optional] [default to null]
**transaction_creation_date** | [***::models::Date**](Date.md) | The date and time that the credit transaction was created, in ISO 8601 date time format. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


