# ShipmentConfirmation

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | Purchase order number corresponding to the shipment. | [default to null]
**shipment_details** | [***::models::ShipmentDetails**](ShipmentDetails.md) | Shipment information. | [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | ID of the selling party or vendor. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Warehouse code of vendor. | [default to null]
**items** | [**Vec<::models::Item>**](Item.md) | Provide the details of the items in this shipment. If any of the item details field is common at a package or a pallet level, then provide them at the corresponding package. | [default to null]
**containers** | [**Vec<::models::Container>**](Container.md) | Provide the details of the items in this shipment. If any of the item details field is common at a package or a pallet level, then provide them at the corresponding package. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


