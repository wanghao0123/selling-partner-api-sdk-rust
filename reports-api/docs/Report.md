# Report

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_ids** | **Vec<String>** | A list of marketplace identifiers for the report. | [optional] [default to null]
**report_id** | **String** | The identifier for the report. This identifier is unique only in combination with a seller ID. | [default to null]
**report_type** | **String** | The report type. | [default to null]
**data_start_time** | **String** | The start of a date and time range used for selecting the data to report. | [optional] [default to null]
**data_end_time** | **String** | The end of a date and time range used for selecting the data to report. | [optional] [default to null]
**report_schedule_id** | **String** | The identifier of the report schedule that created this report (if any). This identifier is unique only in combination with a seller ID. | [optional] [default to null]
**created_time** | **String** | The date and time when the report was created. | [default to null]
**processing_status** | **String** | The processing status of the report. | [default to null]
**processing_start_time** | **String** | The date and time when the report processing started, in ISO 8601 date time format. | [optional] [default to null]
**processing_end_time** | **String** | The date and time when the report processing completed, in ISO 8601 date time format. | [optional] [default to null]
**report_document_id** | **String** | The identifier for the report document. Pass this into the getReportDocument operation to get the information you will need to retrieve the report document&#39;s contents. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


