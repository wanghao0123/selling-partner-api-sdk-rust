# FulfillmentOrder

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_fulfillment_order_id** | **String** | The fulfillment order identifier submitted with the createFulfillmentOrder operation. | [default to null]
**marketplace_id** | **String** | The identifier for the marketplace the fulfillment order is placed against. | [default to null]
**displayable_order_id** | **String** | A fulfillment order identifier submitted with the createFulfillmentOrder operation. Displays as the order identifier in recipient-facing materials such as the packing slip. | [default to null]
**displayable_order_date** | [***::models::Timestamp**](Timestamp.md) | A date and time submitted with the createFulfillmentOrder operation. Displays as the order date in recipient-facing materials such as the packing slip. | [default to null]
**displayable_order_comment** | **String** | A text block submitted with the createFulfillmentOrder operation. Displays in recipient-facing materials such as the packing slip. | [default to null]
**shipping_speed_category** | [***::models::ShippingSpeedCategory**](ShippingSpeedCategory.md) |  | [default to null]
**delivery_window** | [***::models::DeliveryWindow**](DeliveryWindow.md) |  | [optional] [default to null]
**destination_address** | [***::models::Address**](Address.md) | The destination address submitted with the createFulfillmentOrder operation. | [default to null]
**fulfillment_action** | [***::models::FulfillmentAction**](FulfillmentAction.md) |  | [optional] [default to null]
**fulfillment_policy** | [***::models::FulfillmentPolicy**](FulfillmentPolicy.md) |  | [optional] [default to null]
**cod_settings** | [***::models::CodSettings**](CODSettings.md) |  | [optional] [default to null]
**received_date** | [***::models::Timestamp**](Timestamp.md) | The date and time that the fulfillment order was received by an Amazon fulfillment center. | [default to null]
**fulfillment_order_status** | [***::models::FulfillmentOrderStatus**](FulfillmentOrderStatus.md) |  | [default to null]
**status_updated_date** | [***::models::Timestamp**](Timestamp.md) | The date and time that the status of the fulfillment order last changed, in ISO 8601 date time format. | [default to null]
**notification_emails** | [***::models::NotificationEmailList**](NotificationEmailList.md) |  | [optional] [default to null]
**feature_constraints** | [**Vec<::models::FeatureSettings>**](FeatureSettings.md) | A list of features and their fulfillment policies to apply to the order. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


