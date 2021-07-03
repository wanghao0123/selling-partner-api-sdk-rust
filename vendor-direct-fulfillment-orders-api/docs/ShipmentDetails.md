# ShipmentDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_priority_shipment** | **bool** | When true, this is a priority shipment. | [default to null]
**is_scheduled_delivery_shipment** | **bool** | When true, this order is part of a scheduled delivery program. | [optional] [default to null]
**is_pslip_required** | **bool** | When true, a packing slip is required to be sent to the customer. | [default to null]
**is_gift** | **bool** | When true, the order contain a gift. Include the gift message and gift wrap information. | [optional] [default to null]
**ship_method** | **String** | Ship method to be used for shipping the order. Amazon defines ship method codes indicating the shipping carrier and shipment service level. To see the full list of ship methods in use, including both the code and the friendly name, search the &#39;Help&#39; section on Vendor Central for &#39;ship methods&#39;. | [default to null]
**shipment_dates** | [***::models::ShipmentDates**](ShipmentDates.md) |  | [default to null]
**message_to_customer** | **String** | Message to customer for order status. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


