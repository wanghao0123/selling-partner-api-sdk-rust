# CreateShipmentRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_request_details** | [***::models::ShipmentRequestDetails**](ShipmentRequestDetails.md) | Shipment information required for creating a shipment. | [default to null]
**shipping_service_id** | [***::models::ShippingServiceIdentifier**](ShippingServiceIdentifier.md) |  | [default to null]
**shipping_service_offer_id** | **String** | Identifies a shipping service order made by a carrier. | [optional] [default to null]
**hazmat_type** | [***::models::HazmatType**](HazmatType.md) | Hazardous materials options for a package. Consult the terms and conditions for each carrier for more information about hazardous materials. | [optional] [default to null]
**label_format_option** | [***::models::LabelFormatOptionRequest**](LabelFormatOptionRequest.md) |  | [optional] [default to null]
**shipment_level_seller_inputs_list** | [***::models::AdditionalSellerInputsList**](AdditionalSellerInputsList.md) | A list of additional seller inputs required to ship this shipment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


