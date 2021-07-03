# OrderDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_order_number** | **String** | The customer order number. | [default to null]
**order_date** | **String** | The date the order was placed. This field is expected to be in ISO-8601 date/time format, for example:2018-07-16T23:00:00Z/ 2018-07-16T23:00:00-05:00 /2018-07-16T23:00:00-08:00. If no time zone is specified, UTC should be assumed. | [default to null]
**order_status** | **String** | Current status of the order. | [optional] [default to null]
**shipment_details** | [***::models::ShipmentDetails**](ShipmentDetails.md) |  | [default to null]
**tax_total** | [***Value**](Value.md) |  | [optional] [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | PartyID of vendor code. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | PartyID of vendor&#39;s warehouse. | [default to null]
**ship_to_party** | [***::models::Address**](Address.md) | Name/Address and tax details of the ship to party. | [default to null]
**bill_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the bill to party. | [default to null]
**items** | [**Vec<::models::OrderItem>**](OrderItem.md) | A list of items in this purchase order. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


