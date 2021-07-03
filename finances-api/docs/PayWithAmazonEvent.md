# PayWithAmazonEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_order_id** | **String** | An order identifier that is specified by the seller. | [optional] [default to null]
**transaction_posted_date** | [***::models::Date**](Date.md) | The date and time when the payment transaction is posted. In ISO 8601 date time format. | [optional] [default to null]
**business_object_type** | **String** | The type of business object. | [optional] [default to null]
**sales_channel** | **String** | The sales channel for the transaction. | [optional] [default to null]
**charge** | [***::models::ChargeComponent**](ChargeComponent.md) | The charge associated with the event. | [optional] [default to null]
**fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of fees associated with the event. | [optional] [default to null]
**payment_amount_type** | **String** | The type of payment.  Possible values:  * Sales | [optional] [default to null]
**amount_description** | **String** | A short description of this payment event. | [optional] [default to null]
**fulfillment_channel** | **String** | The fulfillment channel.  Possible values:  * AFN - Amazon Fulfillment Network (Fulfillment by Amazon)  * MFN - Merchant Fulfillment Network (self-fulfilled) | [optional] [default to null]
**store_name** | **String** | The store name where the event occurred. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


