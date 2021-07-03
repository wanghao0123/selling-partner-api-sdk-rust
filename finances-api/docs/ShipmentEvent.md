# ShipmentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined identifier for an order. | [optional] [default to null]
**seller_order_id** | **String** | A seller-defined identifier for an order. | [optional] [default to null]
**marketplace_name** | **String** | The name of the marketplace where the event occurred. | [optional] [default to null]
**order_charge_list** | [***::models::ChargeComponentList**](ChargeComponentList.md) | A list of order-level charges. These charges are applicable to Multi-Channel Fulfillment COD orders. | [optional] [default to null]
**order_charge_adjustment_list** | [***::models::ChargeComponentList**](ChargeComponentList.md) | A list of order-level charge adjustments. These adjustments are applicable to Multi-Channel Fulfillment COD orders. | [optional] [default to null]
**shipment_fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of shipment-level fees. | [optional] [default to null]
**shipment_fee_adjustment_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of shipment-level fee adjustments. | [optional] [default to null]
**order_fee_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of order-level fees. These charges are applicable to Multi-Channel Fulfillment orders. | [optional] [default to null]
**order_fee_adjustment_list** | [***::models::FeeComponentList**](FeeComponentList.md) | A list of order-level fee adjustments. These adjustments are applicable to Multi-Channel Fulfillment orders. | [optional] [default to null]
**direct_payment_list** | [***::models::DirectPaymentList**](DirectPaymentList.md) | A list of transactions where buyers pay Amazon through one of the credit cards offered by Amazon or where buyers pay a seller directly through COD. | [optional] [default to null]
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**shipment_item_list** | [***::models::ShipmentItemList**](ShipmentItemList.md) |  | [optional] [default to null]
**shipment_item_adjustment_list** | [***::models::ShipmentItemList**](ShipmentItemList.md) | A list of shipment item adjustments. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


