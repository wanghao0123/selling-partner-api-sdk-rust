# AffordabilityExpenseEvent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined identifier for an order. | [optional] [default to null]
**posted_date** | [***::models::Date**](Date.md) | The date and time when the financial event was created. | [optional] [default to null]
**marketplace_id** | **String** | An encrypted, Amazon-defined marketplace identifier. | [optional] [default to null]
**transaction_type** | **String** | Indicates the type of transaction.   Possible values:  * Charge - For an affordability promotion expense.  * Refund - For an affordability promotion expense reversal. | [optional] [default to null]
**base_expense** | [***::models::Currency**](Currency.md) | The amount charged for clicks incurred under the Sponsored Products program. | [optional] [default to null]
**tax_type_cgst** | [***::models::Currency**](Currency.md) | Central Goods and Service Tax, charged and collected by the central government. | [default to null]
**tax_type_sgst** | [***::models::Currency**](Currency.md) | State Goods and Service Tax, charged and collected by the state government. | [default to null]
**tax_type_igst** | [***::models::Currency**](Currency.md) | Integrated Goods and Service Tax, charged and collected by the central government. | [default to null]
**total_expense** | [***::models::Currency**](Currency.md) | The total amount charged to the seller. TotalExpense &#x3D; BaseExpense + TaxTypeIGST + TaxTypeCGST + TaxTypeSGST. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


