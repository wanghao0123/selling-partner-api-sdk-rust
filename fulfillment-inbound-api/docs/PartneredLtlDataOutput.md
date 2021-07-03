# PartneredLtlDataOutput

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact** | [***::models::Contact**](Contact.md) | Contact information for the person in the seller&#39;s organization who is responsible for the shipment. Used by the carrier if they have questions about the shipment. | [default to null]
**box_count** | [***::models::UnsignedIntType**](UnsignedIntType.md) | The number of boxes in the shipment. | [default to null]
**seller_freight_class** | [***::models::SellerFreightClass**](SellerFreightClass.md) |  | [optional] [default to null]
**freight_ready_date** | [***::models::DateStringType**](DateStringType.md) | The date that the shipment will be ready to be picked up by the carrier. Must be in YYYY-MM-DD format. | [default to null]
**pallet_list** | [***::models::PalletList**](PalletList.md) |  | [default to null]
**total_weight** | [***::models::Weight**](Weight.md) | The total weight of the shipment. | [default to null]
**seller_declared_value** | [***::models::Amount**](Amount.md) | Your declaration of the total value of the inventory in the shipment. | [optional] [default to null]
**amazon_calculated_value** | [***::models::Amount**](Amount.md) | Estimate by Amazon of the total value of the inventory in the shipment. | [optional] [default to null]
**preview_pickup_date** | [***::models::DateStringType**](DateStringType.md) | The estimated date that the shipment will be picked up by the carrier, in YYYY-MM-DD format. | [default to null]
**preview_delivery_date** | [***::models::DateStringType**](DateStringType.md) | The estimated date that the shipment will be delivered to an Amazon fulfillment center, in YYYY-MM-DD format. | [default to null]
**preview_freight_class** | [***::models::SellerFreightClass**](SellerFreightClass.md) | The freight class of the shipment as estimated by Amazon if you did not include a freight class when you called the putTransportDetails operation. | [default to null]
**amazon_reference_id** | **String** | A unique identifier created by Amazon that identifies this Amazon-partnered, Less Than Truckload/Full Truckload (LTL/FTL) shipment. | [default to null]
**is_bill_of_lading_available** | **bool** | Indicates whether the bill of lading for the shipment is available. | [default to null]
**partnered_estimate** | [***::models::PartneredEstimate**](PartneredEstimate.md) | The estimated shipping cost using an Amazon-partnered carrier. | [optional] [default to null]
**carrier_name** | **String** | The carrier for the inbound shipment. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


