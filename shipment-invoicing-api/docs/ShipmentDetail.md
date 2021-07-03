# ShipmentDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**warehouse_id** | **String** | The Amazon-defined identifier for the warehouse. | [optional] [default to null]
**amazon_order_id** | **String** | The Amazon-defined identifier for the order. | [optional] [default to null]
**amazon_shipment_id** | **String** | The Amazon-defined identifier for the shipment. | [optional] [default to null]
**purchase_date** | **String** | The date and time when the order was created. | [optional] [default to null]
**shipping_address** | [***::models::Address**](Address.md) |  | [optional] [default to null]
**payment_method_details** | [***::models::PaymentMethodDetailItemList**](PaymentMethodDetailItemList.md) |  | [optional] [default to null]
**marketplace_id** | **String** | The identifier for the marketplace where the order was placed. | [optional] [default to null]
**seller_id** | **String** | The seller identifier. | [optional] [default to null]
**buyer_name** | **String** | The name of the buyer. | [optional] [default to null]
**buyer_county** | **String** | The county of the buyer. | [optional] [default to null]
**buyer_tax_info** | [***::models::BuyerTaxInfo**](BuyerTaxInfo.md) |  | [optional] [default to null]
**shipment_items** | [***::models::ShipmentItems**](ShipmentItems.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


