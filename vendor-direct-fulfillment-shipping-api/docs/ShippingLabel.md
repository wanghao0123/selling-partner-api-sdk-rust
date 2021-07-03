# ShippingLabel

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | This field will contain the Purchase Order Number for this order. | [default to null]
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | ID of the selling party or vendor. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Warehouse code of vendor. | [default to null]
**label_format** | **String** | Format of the label. | [default to null]
**label_data** | [**Vec<::models::LabelData>**](LabelData.md) | Provides the details of the packages in this shipment. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


