# ShipmentDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipped_date** | **String** | This field indicates the date of the departure of the shipment from vendor&#39;s location. Vendors are requested to send ASNs within 30 minutes of departure from their warehouse/distribution center or at least 6 hours prior to the appointment time at the Amazon destination warehouse, whichever is sooner. Shipped date mentioned in the Shipment Confirmation should not be in the future. | [default to null]
**shipment_status** | **String** | Indicate the shipment status. | [default to null]
**is_priority_shipment** | **bool** | Provide the priority of the shipment. | [optional] [default to null]
**vendor_order_number** | **String** | The vendor order number is a unique identifier generated by a vendor for their reference. | [optional] [default to null]
**estimated_delivery_date** | **String** | Date on which the shipment is expected to reach the buyer&#39;s warehouse. It needs to be an estimate based on the average transit time between the ship-from location and the destination. The exact appointment time will be provided by buyer and is potentially not known when creating the shipment confirmation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


