# InboundShipmentPlan

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | A shipment identifier originally returned by the createInboundShipmentPlan operation. | [default to null]
**destination_fulfillment_center_id** | **String** | An Amazon fulfillment center identifier created by Amazon. | [default to null]
**ship_to_address** | [***::models::Address**](Address.md) | The address of the Amazon fulfillment center to which to ship the items. | [default to null]
**label_prep_type** | [***::models::LabelPrepType**](LabelPrepType.md) |  | [default to null]
**items** | [***::models::InboundShipmentPlanItemList**](InboundShipmentPlanItemList.md) | SKU and quantity information for the items in the shipment. | [default to null]
**estimated_box_contents_fee** | [***::models::BoxContentsFeeDetails**](BoxContentsFeeDetails.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


