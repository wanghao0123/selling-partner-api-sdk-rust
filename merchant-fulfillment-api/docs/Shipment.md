# Shipment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | [***::models::ShipmentId**](ShipmentId.md) |  | [default to null]
**amazon_order_id** | [***::models::AmazonOrderId**](AmazonOrderId.md) |  | [default to null]
**seller_order_id** | [***::models::SellerOrderId**](SellerOrderId.md) |  | [optional] [default to null]
**item_list** | [***::models::ItemList**](ItemList.md) |  | [default to null]
**ship_from_address** | [***::models::Address**](Address.md) | The address of the sender. | [default to null]
**ship_to_address** | [***::models::Address**](Address.md) | The destination address for the shipment. | [default to null]
**package_dimensions** | [***::models::PackageDimensions**](PackageDimensions.md) |  | [default to null]
**weight** | [***::models::Weight**](Weight.md) | The package weight. | [default to null]
**insurance** | [***::models::CurrencyAmount**](CurrencyAmount.md) | If DeclaredValue was specified in a previous call to the createShipment operation, then Insurance indicates the amount that the carrier will use to insure the shipment. If DeclaredValue was not specified with a previous call to the createShipment operation, then the shipment will be insured for the carrier&#39;s minimum insurance amount, or the combined sale prices that the items are listed for in the shipment, whichever is less. | [default to null]
**shipping_service** | [***::models::ShippingService**](ShippingService.md) |  | [default to null]
**label** | [***::models::Label**](Label.md) | Data for creating a shipping label and dimensions for printing the label. If the shipment is canceled, an empty Label is returned. | [default to null]
**status** | [***::models::ShipmentStatus**](ShipmentStatus.md) | The shipment status. | [default to null]
**tracking_id** | [***::models::TrackingId**](TrackingId.md) |  | [optional] [default to null]
**created_date** | [***::models::Timestamp**](Timestamp.md) | The date and time the shipment was created. | [default to null]
**last_updated_date** | [***::models::Timestamp**](Timestamp.md) | The date and time of the last update. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


