# ShippingService

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_service_name** | **String** | A plain text representation of a carrier&#39;s shipping service. For example, \&quot;UPS Ground\&quot; or \&quot;FedEx Standard Overnight\&quot;.  | [default to null]
**carrier_name** | **String** | The name of the carrier. | [default to null]
**shipping_service_id** | [***::models::ShippingServiceIdentifier**](ShippingServiceIdentifier.md) |  | [default to null]
**shipping_service_offer_id** | **String** | An Amazon-defined shipping service offer identifier. | [default to null]
**ship_date** | [***::models::Timestamp**](Timestamp.md) | The date that the carrier will ship the package. | [default to null]
**earliest_estimated_delivery_date** | [***::models::Timestamp**](Timestamp.md) | The earliest date by which the shipment will be delivered. | [optional] [default to null]
**latest_estimated_delivery_date** | [***::models::Timestamp**](Timestamp.md) | The latest date by which the shipment will be delivered. | [optional] [default to null]
**rate** | [***::models::CurrencyAmount**](CurrencyAmount.md) | The amount that the carrier will charge for the shipment. | [default to null]
**shipping_service_options** | [***::models::ShippingServiceOptions**](ShippingServiceOptions.md) | Extra services offered by the carrier. | [default to null]
**available_shipping_service_options** | [***::models::AvailableShippingServiceOptions**](AvailableShippingServiceOptions.md) |  | [optional] [default to null]
**available_label_formats** | [***::models::LabelFormatList**](LabelFormatList.md) |  | [optional] [default to null]
**available_format_options_for_label** | [***::models::AvailableFormatOptionsForLabelList**](AvailableFormatOptionsForLabelList.md) |  | [optional] [default to null]
**requires_additional_seller_inputs** | **bool** | When true, additional seller inputs are required. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


