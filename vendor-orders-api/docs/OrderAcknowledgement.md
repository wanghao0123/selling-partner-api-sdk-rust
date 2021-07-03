# OrderAcknowledgement

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number. Formatting Notes: 8-character alpha-numeric code. | [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party receiving a shipment of products. | [default to null]
**acknowledgement_date** | **String** | The date and time when the purchase order is acknowledged, in ISO-8601 date/time format. | [default to null]
**items** | [**Vec<::models::OrderAcknowledgementItem>**](OrderAcknowledgementItem.md) | A list of the items being acknowledged with associated details. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


