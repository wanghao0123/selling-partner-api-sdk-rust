# TransportHeader

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_id** | **String** | The Amazon seller identifier. | [default to null]
**shipment_id** | **String** | A shipment identifier originally returned by the createInboundShipmentPlan operation. | [default to null]
**is_partnered** | **bool** | Indicates whether a putTransportDetails request is for a partnered carrier.  Possible values:  * true – Request is for an Amazon-partnered carrier.  * false – Request is for a non-Amazon-partnered carrier. | [default to null]
**shipment_type** | [***::models::ShipmentType**](ShipmentType.md) |  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


