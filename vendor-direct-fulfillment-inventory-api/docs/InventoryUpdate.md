# InventoryUpdate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**selling_party** | [***::models::PartyIdentification**](PartyIdentification.md) | ID of the selling party or vendor. | [default to null]
**is_full_update** | **bool** | When true, this request contains a full feed. Otherwise, this request contains a partial feed. When sending a full feed, you must send information about all items in the warehouse. Any items not in the full feed are updated as not available. When sending a partial feed, only include the items that need an update to inventory. The status of other items will remain unchanged. | [default to null]
**items** | [**Vec<::models::ItemDetails>**](ItemDetails.md) | A list of inventory items with updated details, including quantity available. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


