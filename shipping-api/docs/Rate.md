# Rate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rate_id** | **String** | An identifier for the rate. | [optional] [default to null]
**total_charge** | [***::models::Currency**](Currency.md) | The total charge that will be billed for the rate. | [optional] [default to null]
**billed_weight** | [***::models::Weight**](Weight.md) | The weight that was used to calculate the totalCharge. | [optional] [default to null]
**expiration_time** | **String** | The time after which the offering will expire. | [optional] [default to null]
**service_type** | [***::models::ServiceType**](ServiceType.md) |  | [optional] [default to null]
**promise** | [***::models::ShippingPromiseSet**](ShippingPromiseSet.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


