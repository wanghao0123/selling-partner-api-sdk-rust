# ItemDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The Amazon purchase order number for the shipment being confirmed. If the items in this shipment belong to multiple purchase order numbers that are in particular carton or pallet within the shipment, then provide the purchaseOrderNumber at the appropriate carton or pallet level. Formatting Notes: 8-character alpha-numeric code. | [optional] [default to null]
**lot_number** | **String** | The batch or lot number associates an item with information the manufacturer considers relevant for traceability of the trade item to which the Element String is applied. The data may refer to the trade item itself or to items contained. This field is mandatory for all perishable items. | [optional] [default to null]
**expiry** | [***::models::Expiry**](Expiry.md) | Either expiryDate or mfgDate and expiryAfterDuration are mandatory for perishable items. | [optional] [default to null]
**maximum_retail_price** | [***::models::Money**](Money.md) | Maximum retail price of the item being shipped. | [optional] [default to null]
**handling_code** | **String** | Identification of the instructions on how specified item/carton/pallet should be handled. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


