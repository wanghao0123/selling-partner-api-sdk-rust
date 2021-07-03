# PartneredLtlDataInput

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact** | [***::models::Contact**](Contact.md) | Contact information for the person in the seller&#39;s organization who is responsible for the shipment. Used by the carrier if they have questions about the shipment. | [optional] [default to null]
**box_count** | [***::models::UnsignedIntType**](UnsignedIntType.md) | The number of boxes in the shipment. | [optional] [default to null]
**seller_freight_class** | [***::models::SellerFreightClass**](SellerFreightClass.md) |  | [optional] [default to null]
**freight_ready_date** | [***::models::DateStringType**](DateStringType.md) | The date that the shipment will be ready to be picked up by the carrier. | [optional] [default to null]
**pallet_list** | [***::models::PalletList**](PalletList.md) |  | [optional] [default to null]
**total_weight** | [***::models::Weight**](Weight.md) | The total weight of the shipment. | [optional] [default to null]
**seller_declared_value** | [***::models::Amount**](Amount.md) | The declaration of the total value of the inventory in the shipment. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


