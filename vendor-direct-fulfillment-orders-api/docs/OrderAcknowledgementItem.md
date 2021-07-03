# OrderAcknowledgementItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number for this order. Formatting Notes: alpha-numeric code. | [default to null]
**vendor_order_number** | **String** | The vendor&#39;s order number for this order. | [default to null]
**acknowledgement_date** | **String** | The date and time when the order is acknowledged, in ISO-8601 date/time format. For example: 2018-07-16T23:00:00Z / 2018-07-16T23:00:00-05:00 / 2018-07-16T23:00:00-08:00. | [default to null]
**acknowledgement_status** | [***::models::AcknowledgementStatus**](AcknowledgementStatus.md) | Status of acknowledgement. | [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | PartyID as vendor code. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | PartyID as the vendor&#39;s warehouseId. | [default to null]
**item_acknowledgements** | [**Vec<::models::OrderItemAcknowledgement>**](OrderItemAcknowledgement.md) | Item details including acknowledged quantity. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


