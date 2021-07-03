# TrialShipmentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined identifier for an order. | [optional] [default to null]
**financial_event_group_id** | **String** | The identifier of the financial event group. | [optional] [default to null]
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**SKU** | **String** | The seller SKU of the item. The seller SKU is qualified by the seller&#39;s seller ID, which is included with every call to the Selling Partner API. | [optional] [default to null]
**fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of fees charged by Amazon for trial shipments. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


