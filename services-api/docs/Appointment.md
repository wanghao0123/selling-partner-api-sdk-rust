# Appointment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**appointment_id** | [***::models::AppointmentId**](AppointmentId.md) | The appointment identifier. | [optional] [default to null]
**appointment_status** | **String** | The status of the appointment. | [optional] [default to null]
**appointment_time** | [***::models::AppointmentTime**](AppointmentTime.md) | The time of the appointment window. | [optional] [default to null]
**assigned_technicians** | [**Vec<::models::Technician>**](Technician.md) | A list of technicians assigned to the service job. | [optional] [default to null]
**rescheduled_appointment_id** | [***::models::AppointmentId**](AppointmentId.md) | The identifier of a rescheduled appointment. | [optional] [default to null]
**poa** | [***::models::Poa**](Poa.md) | Proof of Appointment (POA) details. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


