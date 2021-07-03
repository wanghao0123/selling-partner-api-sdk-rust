# CreateReportScheduleSpecification

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_type** | **String** | The report type. | [default to null]
**marketplace_ids** | **Vec<String>** | A list of marketplace identifiers for the report schedule. | [default to null]
**report_options** | [***::models::ReportOptions**](ReportOptions.md) |  | [optional] [default to null]
**period** | **String** | One of a set of predefined ISO 8601 periods that specifies how often a report should be created. | [default to null]
**next_report_creation_time** | **String** | The date and time when the schedule will create its next report, in ISO 8601 date time format. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


