# InboundShipmentInfo

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | The shipment identifier submitted in the request. | [optional] [default to null]
**shipment_name** | **String** | The name for the inbound shipment. | [optional] [default to null]
**ship_from_address** | [***::models::Address**](Address.md) | The return address. | [default to null]
**destination_fulfillment_center_id** | **String** | An Amazon fulfillment center identifier created by Amazon. | [optional] [default to null]
**shipment_status** | [***::models::ShipmentStatus**](ShipmentStatus.md) |  | [optional] [default to null]
**label_prep_type** | [***::models::LabelPrepType**](LabelPrepType.md) |  | [optional] [default to null]
**are_cases_required** | **bool** | Indicates whether or not an inbound shipment contains case-packed boxes. When AreCasesRequired &#x3D; true for an inbound shipment, all items in the inbound shipment must be case packed. | [default to null]
**confirmed_need_by_date** | [***::models::DateStringType**](DateStringType.md) | Date by which the shipment must arrive at the Amazon fulfillment center to avoid delivery promise breaks for pre-ordered items. | [optional] [default to null]
**box_contents_source** | [***::models::BoxContentsSource**](BoxContentsSource.md) |  | [optional] [default to null]
**estimated_box_contents_fee** | [***::models::BoxContentsFeeDetails**](BoxContentsFeeDetails.md) | An estimate of the manual processing fee charged by Amazon for boxes without box content information. This is only returned when BoxContentsSource is NONE. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


