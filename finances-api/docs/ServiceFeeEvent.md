# ServiceFeeEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined identifier for an order. | [optional] [default to null]
**fee_reason** | **String** | A short description of the service fee reason. | [optional] [default to null]
**fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of fee components associated with the service fee. | [optional] [default to null]
**seller_sku** | **String** | The seller SKU of the item. The seller SKU is qualified by the seller&#39;s seller ID, which is included with every call to the Selling Partner API. | [optional] [default to null]
**fn_sku** | **String** | A unique identifier assigned by Amazon to products stored in and fulfilled from an Amazon fulfillment center. | [optional] [default to null]
**fee_description** | **String** | A short description of the service fee event. | [optional] [default to null]
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


