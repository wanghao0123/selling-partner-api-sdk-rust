# ShipmentRequestDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | [***::models::AmazonOrderId**](AmazonOrderId.md) | An Amazon-defined order identifier in 3-7-7 format. | [default to null]
**seller_order_id** | [***::models::SellerOrderId**](SellerOrderId.md) | A seller-defined order identifier. | [optional] [default to null]
**item_list** | [***::models::ItemList**](ItemList.md) |  | [default to null]
**ship_from_address** | [***::models::Address**](Address.md) | The address of the sender. | [default to null]
**package_dimensions** | [***::models::PackageDimensions**](PackageDimensions.md) | The package dimensions. | [default to null]
**weight** | [***::models::Weight**](Weight.md) | The package weight. | [default to null]
**must_arrive_by_date** | [***::models::Timestamp**](Timestamp.md) | The date by which the package must arrive to keep the promise to the customer, in ISO 8601 datetime format. If MustArriveByDate is specified, only shipping service offers that can be delivered by that date are returned. | [optional] [default to null]
**ship_date** | [***::models::Timestamp**](Timestamp.md) | When used in a request, this is the date and time that the seller wants to ship the package. When used in a response, this is the date and time that the package can be shipped by the indicated method. | [optional] [default to null]
**shipping_service_options** | [***::models::ShippingServiceOptions**](ShippingServiceOptions.md) | Extra services offered by the carrier. | [default to null]
**label_customization** | [***::models::LabelCustomization**](LabelCustomization.md) | Label customization options. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


