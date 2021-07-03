# NetworkComminglingTransactionEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_type** | **String** | The type of network item swap.  Possible values:  * NetCo - A Fulfillment by Amazon inventory pooling transaction. Available only in the India marketplace.  * ComminglingVAT - A commingling VAT transaction. Available only in the UK, Spain, France, Germany, and Italy marketplaces. | [optional] [default to null]
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**net_co_transaction_id** | **String** | The identifier for the network item swap. | [optional] [default to null]
**swap_reason** | **String** | The reason for the network item swap. | [optional] [default to null]
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the swapped item. | [optional] [default to null]
**marketplace_id** | **String** | The marketplace in which the event took place. | [optional] [default to null]
**tax_exclusive_amount** | [***::models::Currency**](Currency.md) | The price of the swapped item minus TaxAmount. | [optional] [default to null]
**tax_amount** | [***::models::Currency**](Currency.md) | The tax on the network item swap paid by the seller. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


