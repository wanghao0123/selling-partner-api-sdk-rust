# InboundShipmentHeader

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_name** | **String** | The name for the shipment. Use a naming convention that helps distinguish between shipments over time, such as the date the shipment was created. | [default to null]
**ship_from_address** | [***::models::Address**](Address.md) | The return address. | [default to null]
**destination_fulfillment_center_id** | **String** | The identifier for the fulfillment center to which the shipment will be shipped. Get this value from the InboundShipmentPlan object in the response returned by the createInboundShipmentPlan operation. | [default to null]
**are_cases_required** | **bool** | Indicates whether or not an inbound shipment contains case-packed boxes. Note: A shipment must contain either all case-packed boxes or all individually packed boxes.  Possible values:  true - All boxes in the shipment must be case packed.  false - All boxes in the shipment must be individually packed.  Note: If AreCasesRequired &#x3D; true for an inbound shipment, then the value of QuantityInCase must be greater than zero for every item in the shipment. Otherwise the service returns an error. | [optional] [default to null]
**shipment_status** | [***::models::ShipmentStatus**](ShipmentStatus.md) |  | [default to null]
**label_prep_preference** | [***::models::LabelPrepPreference**](LabelPrepPreference.md) | The preference for label preparation for an inbound shipment. | [default to null]
**intended_box_contents_source** | [***::models::IntendedBoxContentsSource**](IntendedBoxContentsSource.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


