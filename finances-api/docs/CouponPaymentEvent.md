# CouponPaymentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**coupon_id** | **String** | A coupon identifier. | [optional] [default to null]
**seller_coupon_description** | **String** | The description provided by the seller when they created the coupon. | [optional] [default to null]
**clip_or_redemption_count** | **i64** | The number of coupon clips or redemptions. | [optional] [default to null]
**payment_event_id** | **String** | A payment event identifier. | [optional] [default to null]
**fee_component** | [***::models::FeeComponent**](FeeComponent.md) |  | [optional] [default to null]
**charge_component** | [***::models::ChargeComponent**](ChargeComponent.md) |  | [optional] [default to null]
**total_amount** | [***::models::Currency**](Currency.md) | The FeeComponent value plus the ChargeComponent value. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


