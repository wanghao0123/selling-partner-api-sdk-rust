# RetrochargeEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retrocharge_event_type** | **String** | The type of event.  Possible values:  * Retrocharge  * RetrochargeReversal | [optional] [default to null]
**amazon_order_id** | **String** | An Amazon-defined identifier for an order. | [optional] [default to null]
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**base_tax** | [***::models::Currency**](Currency.md) | The base tax associated with the retrocharge event. | [optional] [default to null]
**shipping_tax** | [***::models::Currency**](Currency.md) | The shipping tax associated with the retrocharge event. | [optional] [default to null]
**marketplace_name** | **String** | The name of the marketplace where the retrocharge event occurred. | [optional] [default to null]
**retrocharge_tax_withheld_list** | [***::models::TaxWithheldComponentList**](TaxWithheldComponentList.md) | A list of information about taxes withheld. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


