# CreateFulfillmentOrderRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_id** | **String** | The marketplace the fulfillment order is placed against. | [optional] [default to null]
**seller_fulfillment_order_id** | **String** | A fulfillment order identifier that the seller creates to track their fulfillment order. The SellerFulfillmentOrderId must be unique for each fulfillment order that a seller creates. If the seller&#39;s system already creates unique order identifiers, then these might be good values for them to use. | [default to null]
**displayable_order_id** | **String** | A fulfillment order identifier that the seller creates. This value displays as the order identifier in recipient-facing materials such as the outbound shipment packing slip. The value of DisplayableOrderId should match the order identifier that the seller provides to the recipient. The seller can use the SellerFulfillmentOrderId for this value or they can specify an alternate value if they want the recipient to reference an alternate order identifier.  The value must be an alpha-numeric or ISO 8859-1 compliant string from one to 40 characters in length. Cannot contain two spaces in a row. Leading and trailing white space is removed. | [default to null]
**displayable_order_date** | [***::models::Timestamp**](Timestamp.md) | The date and time of the fulfillment order. Displays as the order date in recipient-facing materials such as the outbound shipment packing slip. | [default to null]
**displayable_order_comment** | **String** | Order-specific text that appears in recipient-facing materials such as the outbound shipment packing slip. | [default to null]
**shipping_speed_category** | [***::models::ShippingSpeedCategory**](ShippingSpeedCategory.md) | The shipping method for the fulfillment order. | [default to null]
**delivery_window** | [***::models::DeliveryWindow**](DeliveryWindow.md) |  | [optional] [default to null]
**destination_address** | [***::models::Address**](Address.md) | The destination address for the fulfillment order. | [default to null]
**fulfillment_action** | [***::models::FulfillmentAction**](FulfillmentAction.md) |  | [optional] [default to null]
**fulfillment_policy** | [***::models::FulfillmentPolicy**](FulfillmentPolicy.md) |  | [optional] [default to null]
**cod_settings** | [***::models::CodSettings**](CODSettings.md) |  | [optional] [default to null]
**ship_from_country_code** | **String** | The two-character country code for the country from which the fulfillment order ships. Must be in ISO 3166-1 alpha-2 format. | [optional] [default to null]
**notification_emails** | [***::models::NotificationEmailList**](NotificationEmailList.md) |  | [optional] [default to null]
**feature_constraints** | [**Vec<::models::FeatureSettings>**](FeatureSettings.md) | A list of features and their fulfillment policies to apply to the order. | [optional] [default to null]
**items** | [***::models::CreateFulfillmentOrderItemList**](CreateFulfillmentOrderItemList.md) | A list of items to include in the fulfillment order preview, including quantity. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


