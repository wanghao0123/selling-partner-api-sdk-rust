# ServiceJob

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | **String** | The date and time of the creation of the job, in ISO 8601 format. | [optional] [default to null]
**service_job_id** | [***::models::ServiceJobId**](ServiceJobId.md) | The service job identifier. | [optional] [default to null]
**service_job_status** | **String** | The status of the service job. | [optional] [default to null]
**scope_of_work** | [***::models::ScopeOfWork**](ScopeOfWork.md) | The scope of work for the order. | [optional] [default to null]
**seller** | [***::models::Seller**](Seller.md) | Information about the seller of the service job. | [optional] [default to null]
**service_job_provider** | [***::models::ServiceJobProvider**](ServiceJobProvider.md) | Information about the service job provider. | [optional] [default to null]
**preferred_appointment_times** | [**Vec<::models::AppointmentTime>**](AppointmentTime.md) | A list of appointment windows preferred by the buyer. Included only if the buyer selected appointment windows when creating the order. | [optional] [default to null]
**appointments** | [**Vec<::models::Appointment>**](Appointment.md) | A list of appointments. | [optional] [default to null]
**service_order_id** | [***::models::OrderId**](OrderId.md) | The Amazon-defined identifier for an order placed by the buyer, in 3-7-7 format. | [optional] [default to null]
**marketplace_id** | **String** | The marketplace identifier. | [optional] [default to null]
**buyer** | [***::models::Buyer**](Buyer.md) | Information about the buyer. | [optional] [default to null]
**associated_items** | [**Vec<::models::AssociatedItem>**](AssociatedItem.md) | A list of items associated with the service job. | [optional] [default to null]
**service_location** | [***::models::ServiceLocation**](ServiceLocation.md) | Information about the location of the service job. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


