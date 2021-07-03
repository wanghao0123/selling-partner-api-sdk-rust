# ShippingLabelRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | Purchase order number of the order for which to create a shipping label. | [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | ID of the selling party or vendor. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Warehouse code of vendor. | [default to null]
**containers** | [**Vec<::models::Container>**](Container.md) | A list of the packages in this shipment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


