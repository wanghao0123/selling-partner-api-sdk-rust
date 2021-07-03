# JobListing

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_result_size** | **i32** | Total result size of the query result. | [optional] [default to null]
**next_page_token** | **String** | A generated string used to pass information to your next request.If nextPageToken is returned, pass the value of nextPageToken to the pageToken to get next results. | [optional] [default to null]
**previous_page_token** | **String** | A generated string used to pass information to your next request.If previousPageToken is returned, pass the value of previousPageToken to the pageToken to get previous page results. | [optional] [default to null]
**jobs** | [**Vec<::models::ServiceJob>**](ServiceJob.md) | List of job details for the given input. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


