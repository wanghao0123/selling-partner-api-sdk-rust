# RejectedShippingService

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carrier_name** | **String** | The rejected shipping carrier name. e.g. USPS | [default to null]
**shipping_service_name** | **String** | The rejected shipping service localized name. e.g. FedEx Standard Overnight | [default to null]
**shipping_service_id** | [***::models::ShippingServiceIdentifier**](ShippingServiceIdentifier.md) | The rejected shipping service identifier. e.g. FEDEX_PTP_STANDARD_OVERNIGHT | [default to null]
**rejection_reason_code** | **String** | A reason code meant to be consumed programatically. e.g. CARRIER_CANNOT_SHIP_TO_POBOX | [default to null]
**rejection_reason_message** | **String** | A localized human readable description of the rejected reason. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


