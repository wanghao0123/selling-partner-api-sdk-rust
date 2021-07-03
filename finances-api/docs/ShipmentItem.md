# ShipmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. The seller SKU is qualified by the seller&#39;s seller ID, which is included with every call to the Selling Partner API. | [optional] [default to null]
**order_item_id** | **String** | An Amazon-defined order item identifier. | [optional] [default to null]
**order_adjustment_item_id** | **String** | An Amazon-defined order adjustment identifier defined for refunds, guarantee claims, and chargeback events. | [optional] [default to null]
**quantity_shipped** | **i32** | The number of items shipped. | [optional] [default to null]
**item_charge_list** | [***::models::ChargeComponentList**](ChargeComponentList.md) | A list of charges associated with the shipment item. | [optional] [default to null]
**item_charge_adjustment_list** | [***::models::ChargeComponentList**](ChargeComponentList.md) | A list of charge adjustments associated with the shipment item. This value is only returned for refunds, guarantee claims, and chargeback events. | [optional] [default to null]
**item_fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of fees associated with the shipment item. | [optional] [default to null]
**item_fee_adjustment_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of fee adjustments associated with the shipment item. This value is only returned for refunds, guarantee claims, and chargeback events. | [optional] [default to null]
**item_tax_withheld_list** | [***::models::TaxWithheldComponentList**](TaxWithheldComponentList.md) | A list of taxes withheld information for a shipment item. | [optional] [default to null]
**promotion_list** | [***::models::PromotionList**](PromotionList.md) |  | [optional] [default to null]
**promotion_adjustment_list** | [***::models::PromotionList**](PromotionList.md) | A list of promotion adjustments associated with the shipment item. This value is only returned for refunds, guarantee claims, and chargeback events. | [optional] [default to null]
**cost_of_points_granted** | [***::models::Currency**](Currency.md) | The cost of Amazon Points granted for a shipment item. | [optional] [default to null]
**cost_of_points_returned** | [***::models::Currency**](Currency.md) | The cost of Amazon Points returned for a shipment item. This value is only returned for refunds, guarantee claims, and chargeback events. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


