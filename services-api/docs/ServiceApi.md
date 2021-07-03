# \ServiceApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_appointment_for_service_job_by_service_job_id**](ServiceApi.md#add_appointment_for_service_job_by_service_job_id) | **Post** /service/v1/serviceJobs/{serviceJobId}/appointments | 
[**cancel_service_job_by_service_job_id**](ServiceApi.md#cancel_service_job_by_service_job_id) | **Put** /service/v1/serviceJobs/{serviceJobId}/cancellations | 
[**complete_service_job_by_service_job_id**](ServiceApi.md#complete_service_job_by_service_job_id) | **Put** /service/v1/serviceJobs/{serviceJobId}/completions | 
[**get_service_job_by_service_job_id**](ServiceApi.md#get_service_job_by_service_job_id) | **Get** /service/v1/serviceJobs/{serviceJobId} | 
[**get_service_jobs**](ServiceApi.md#get_service_jobs) | **Get** /service/v1/serviceJobs | 
[**reschedule_appointment_for_service_job_by_service_job_id**](ServiceApi.md#reschedule_appointment_for_service_job_by_service_job_id) | **Post** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId} | 


# **add_appointment_for_service_job_by_service_job_id**
> ::models::SetAppointmentResponse add_appointment_for_service_job_by_service_job_id(service_job_id, body)


Adds an appointment to the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **service_job_id** | **String**| An Amazon defined service job identifier. | 
  **body** | [**AddAppointmentRequest**](AddAppointmentRequest.md)| Add appointment operation input details. | 

### Return type

[**::models::SetAppointmentResponse**](SetAppointmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **cancel_service_job_by_service_job_id**
> ::models::CancelServiceJobByServiceJobIdResponse cancel_service_job_by_service_job_id(service_job_id, cancellation_reason_code)


Cancels the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **service_job_id** | **String**| An Amazon defined service job identifier. | 
  **cancellation_reason_code** | **String**| A cancel reason code that specifies the reason for cancelling a service job. | 

### Return type

[**::models::CancelServiceJobByServiceJobIdResponse**](CancelServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **complete_service_job_by_service_job_id**
> ::models::CompleteServiceJobByServiceJobIdResponse complete_service_job_by_service_job_id(service_job_id)


Completes the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **service_job_id** | **String**| An Amazon defined service job identifier. | 

### Return type

[**::models::CompleteServiceJobByServiceJobIdResponse**](CompleteServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_service_job_by_service_job_id**
> ::models::GetServiceJobByServiceJobIdResponse get_service_job_by_service_job_id(service_job_id)


Gets service job details for the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **service_job_id** | **String**| A service job identifier. | 

### Return type

[**::models::GetServiceJobByServiceJobIdResponse**](GetServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_service_jobs**
> ::models::GetServiceJobsResponse get_service_jobs(marketplace_ids, optional)


Gets service job details for the specified filter query.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 40 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **marketplace_ids** | [**Vec&lt;String&gt;**](String.md)| Used to select jobs that were placed in the specified marketplaces.  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **marketplace_ids** | [**Vec&lt;String&gt;**](String.md)| Used to select jobs that were placed in the specified marketplaces.  | 
 **service_order_ids** | [**Vec&lt;String&gt;**](String.md)| List of service order ids for the query you want to perform.Max values supported 20.  | 
 **service_job_status** | [**Vec&lt;String&gt;**](String.md)| A list of one or more job status by which to filter the list of jobs. | 
 **page_token** | **String**| String returned in the response of your previous request. | 
 **page_size** | **i32**| A non-negative integer that indicates the maximum number of jobs to return in the list, Value must be 1 - 20. Default 20.  | [default to 20]
 **sort_field** | **String**| Sort fields on which you want to sort the output. | 
 **sort_order** | **String**| Sort order for the query you want to perform. | 
 **created_after** | **String**| A date used for selecting jobs created after (or at) a specified time must be in ISO 8601 format. Required if LastUpdatedAfter is not specified.Specifying both CreatedAfter and LastUpdatedAfter returns an error.  | 
 **created_before** | **String**| A date used for selecting jobs created before (or at) a specified time must be in ISO 8601 format.  | 
 **last_updated_after** | **String**| A date used for selecting jobs updated after (or at) a specified time must be in ISO 8601 format. Required if createdAfter is not specified.Specifying both CreatedAfter and LastUpdatedAfter returns an error.  | 
 **last_updated_before** | **String**| A date used for selecting jobs updated before (or at) a specified time must be in ISO 8601 format.  | 
 **schedule_start_date** | **String**| A date used for filtering jobs schedule after (or at) a specified time must be in ISO 8601 format. schedule end date should not be earlier than schedule start date.  | 
 **schedule_end_date** | **String**| A date used for filtering jobs schedule before (or at) a specified time must be in ISO 8601 format. schedule end date should not be earlier than schedule start date.  | 

### Return type

[**::models::GetServiceJobsResponse**](GetServiceJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reschedule_appointment_for_service_job_by_service_job_id**
> ::models::SetAppointmentResponse reschedule_appointment_for_service_job_by_service_job_id(service_job_id, appointment_id, body)


Reschedules an appointment for the service job indicated by the service job identifier you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **service_job_id** | **String**| An Amazon defined service job identifier. | 
  **appointment_id** | **String**| An existing appointment identifier for the Service Job. | 
  **body** | [**RescheduleAppointmentRequest**](RescheduleAppointmentRequest.md)| Reschedule appointment operation input details. | 

### Return type

[**::models::SetAppointmentResponse**](SetAppointmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

