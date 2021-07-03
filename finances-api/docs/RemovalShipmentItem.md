# RemovalShipmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**removal_shipment_item_id** | **String** | An identifier for an item in a removal shipment. | [optional] [default to null]
**tax_collection_model** | **String** | The tax collection model applied to the item.  Possible values:  * MarketplaceFacilitator - Tax is withheld and remitted to the taxing authority by Amazon on behalf of the seller.  * Standard - Tax is paid to the seller and not remitted to the taxing authority by Amazon. | [optional] [default to null]
**fulfillment_network_sku** | **String** | The Amazon fulfillment network SKU for the item. | [optional] [default to null]
**quantity** | **i32** | The quantity of the item. | [optional] [default to null]
**revenue** | [***::models::Currency**](Currency.md) | The total amount paid to the seller for the removed item. | [optional] [default to null]
**fee_amount** | [***::models::Currency**](Currency.md) | The fee that Amazon charged to the seller for the removal of the item. The amount is a negative number. | [optional] [default to null]
**tax_amount** | [***::models::Currency**](Currency.md) | Tax collected on the revenue. | [optional] [default to null]
**tax_withheld** | [***::models::Currency**](Currency.md) | The tax withheld and remitted to the taxing authority by Amazon on behalf of the seller. If TaxCollectionModel&#x3D;MarketplaceFacilitator, then TaxWithheld&#x3D;TaxAmount (except the TaxWithheld amount is a negative number). Otherwise TaxWithheld&#x3D;0. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


