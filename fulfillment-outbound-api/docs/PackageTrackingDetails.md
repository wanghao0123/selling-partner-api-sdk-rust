# PackageTrackingDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_number** | **i32** | The package identifier. | [default to null]
**tracking_number** | **String** | The tracking number for the package. | [optional] [default to null]
**customer_tracking_link** | **String** | Link on swiship.com that allows customers to track the package. | [optional] [default to null]
**carrier_code** | **String** | The name of the carrier. | [optional] [default to null]
**carrier_phone_number** | **String** | The phone number of the carrier. | [optional] [default to null]
**carrier_url** | **String** | The URL of the carrier’s website. | [optional] [default to null]
**ship_date** | [***::models::Timestamp**](Timestamp.md) | The shipping date for the package. | [optional] [default to null]
**estimated_arrival_date** | [***::models::Timestamp**](Timestamp.md) | The estimated arrival date. | [optional] [default to null]
**ship_to_address** | [***::models::TrackingAddress**](TrackingAddress.md) | The destination city for the package. | [optional] [default to null]
**current_status** | [***::models::CurrentStatus**](CurrentStatus.md) |  | [optional] [default to null]
**current_status_description** | **String** | Description corresponding to the CurrentStatus value. | [optional] [default to null]
**signed_for_by** | **String** | The name of the person who signed for the package. | [optional] [default to null]
**additional_location_info** | [***::models::AdditionalLocationInfo**](AdditionalLocationInfo.md) |  | [optional] [default to null]
**tracking_events** | [***::models::TrackingEventList**](TrackingEventList.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


