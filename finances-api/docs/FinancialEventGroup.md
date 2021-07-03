# FinancialEventGroup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**financial_event_group_id** | **String** | A unique identifier for the financial event group. | [optional] [default to null]
**processing_status** | **String** | The processing status of the financial event group indicates whether the balance of the financial event group is settled.  Possible values:  * Open  * Closed | [optional] [default to null]
**fund_transfer_status** | **String** | The status of the fund transfer. | [optional] [default to null]
**original_total** | [***::models::Currency**](Currency.md) | The total amount in the currency of the marketplace in which the transactions occurred. | [optional] [default to null]
**converted_total** | [***::models::Currency**](Currency.md) | The total amount in the currency of the marketplace in which the funds were disbursed. | [optional] [default to null]
**fund_transfer_date** | [***::models::Date**](Date.md) | The date and time when the disbursement or charge was initiated. Only present for closed settlements. In ISO 8601 date time format. | [optional] [default to null]
**trace_id** | **String** | The trace identifier used by sellers to look up transactions externally. | [optional] [default to null]
**account_tail** | **String** | The account tail of the payment instrument. | [optional] [default to null]
**beginning_balance** | [***::models::Currency**](Currency.md) | The balance at the beginning of the settlement period. | [optional] [default to null]
**financial_event_group_start** | [***::models::Date**](Date.md) | The date and time at which the financial event group is opened. In ISO 8601 date time format. | [optional] [default to null]
**financial_event_group_end** | [***::models::Date**](Date.md) | The date and time at which the financial event group is closed. In ISO 8601 date time format. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


