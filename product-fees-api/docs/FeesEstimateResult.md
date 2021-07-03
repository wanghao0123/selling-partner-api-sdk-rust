# FeesEstimateResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | The status of the fee request. Possible values: Success, ClientError, ServiceError. | [optional] [default to null]
**fees_estimate_identifier** | [***::models::FeesEstimateIdentifier**](FeesEstimateIdentifier.md) | Information used to identify a fees estimate request. | [optional] [default to null]
**fees_estimate** | [***::models::FeesEstimate**](FeesEstimate.md) | The total estimated fees for an item and a list of details. | [optional] [default to null]
**error** | [***::models::FeesEstimateError**](FeesEstimateError.md) | An error object with a type, code, and message. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


