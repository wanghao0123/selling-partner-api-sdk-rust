# ReportSchedule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_schedule_id** | **String** | The identifier for the report schedule. This identifier is unique only in combination with a seller ID. | [default to null]
**report_type** | **String** | The report type. | [default to null]
**marketplace_ids** | **Vec<String>** | A list of marketplace identifiers. The report document&#39;s contents will contain data for all of the specified marketplaces, unless the report type indicates otherwise. | [optional] [default to null]
**report_options** | [***::models::ReportOptions**](ReportOptions.md) |  | [optional] [default to null]
**period** | **String** | An ISO 8601 period value that indicates how often a report should be created. | [default to null]
**next_report_creation_time** | **String** | The date and time when the schedule will create its next report, in ISO 8601 date time format. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


