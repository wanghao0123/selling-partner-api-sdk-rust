# OrderDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_date** | **String** | The date the purchase order was placed. Must be in ISO-8601 date/time format. | [default to null]
**purchase_order_changed_date** | **String** | The date when purchase order was last changed by Amazon after the order was placed. This date will be greater than &#39;purchaseOrderDate&#39;. This means the PO data was changed on that date and vendors are required to fulfill the  updated PO. The PO changes can be related to Item Quantity, Ship to Location, Ship Window etc. This field will not be present in orders that have not changed after creation. Must be in ISO-8601 date/time format. | [optional] [default to null]
**purchase_order_state_changed_date** | **String** | The date when current purchase order state was changed. Current purchase order state is available in the field &#39;purchaseOrderState&#39;. Must be in ISO-8601 date/time format. | [default to null]
**purchase_order_type** | **String** | Type of purchase order. | [optional] [default to null]
**import_details** | [***::models::ImportDetails**](ImportDetails.md) | If the purchase order is an import order, the details for the import order. | [optional] [default to null]
**deal_code** | **String** | If requested by the recipient, this field will contain a promotional/deal number. The discount code line is optional. It is used to obtain a price discount on items on the order. | [optional] [default to null]
**payment_method** | **String** | Payment method used. | [optional] [default to null]
**buying_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the buying party. | [optional] [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the selling party. | [optional] [default to null]
**ship_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the ship to party. | [optional] [default to null]
**bill_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name/Address and tax details of the bill to party. | [optional] [default to null]
**ship_window** | [***::models::DateTimeInterval**](DateTimeInterval.md) | This indicates the ship window. Format is start and end date separated by double hyphen (--). For example, 2007-03-01T13:00:00Z--2007-03-11T15:30:00Z. | [optional] [default to null]
**delivery_window** | [***::models::DateTimeInterval**](DateTimeInterval.md) | This indicates the delivery window. Format is start and end date separated by double hyphen (--). For example, 2007-03-01T13:00:00Z--2007-03-11T15:30:00Z. | [optional] [default to null]
**items** | [**Vec<::models::OrderItem>**](OrderItem.md) | A list of items in this purchase order. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


