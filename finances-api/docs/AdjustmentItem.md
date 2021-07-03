# AdjustmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quantity** | **String** | Represents the number of units in the seller&#39;s inventory when the AdustmentType is FBAInventoryReimbursement. | [optional] [default to null]
**per_unit_amount** | [***::models::Currency**](Currency.md) | The per unit value of the item. | [optional] [default to null]
**total_amount** | [***::models::Currency**](Currency.md) | The total value of the item. | [optional] [default to null]
**seller_sku** | **String** | The seller SKU of the item. The seller SKU is qualified by the seller&#39;s seller ID, which is included with every call to the Selling Partner API. | [optional] [default to null]
**fn_sku** | **String** | A unique identifier assigned to products stored in and fulfilled from a fulfillment center. | [optional] [default to null]
**product_description** | **String** | A short description of the item. | [optional] [default to null]
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


