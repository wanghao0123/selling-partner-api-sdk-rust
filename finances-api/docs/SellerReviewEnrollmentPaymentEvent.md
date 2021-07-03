# SellerReviewEnrollmentPaymentEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was posted. | [optional] [default to null]
**enrollment_id** | **String** | An enrollment identifier. | [optional] [default to null]
**parent_asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item that was enrolled in the Early Reviewer Program. | [optional] [default to null]
**fee_component** | [***::models::FeeComponent**](FeeComponent.md) |  | [optional] [default to null]
**charge_component** | [***::models::ChargeComponent**](ChargeComponent.md) |  | [optional] [default to null]
**total_amount** | [***::models::Currency**](Currency.md) | The FeeComponent value plus the ChargeComponent value. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


