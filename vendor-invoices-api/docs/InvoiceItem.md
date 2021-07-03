# InvoiceItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **i32** | Unique number related to this line item. | [default to null]
**amazon_product_identifier** | **String** | Amazon Standard Identification Number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identifier of the item. Should be the same as was provided in the purchase order. | [optional] [default to null]
**invoiced_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Invoiced quantity of this item. Quantity must be greater than zero. | [default to null]
**net_cost** | [***::models::Money**](Money.md) | The item cost to Amazon, which should match the cost on the order. Price information should not be zero or negative. It indicates net unit price. Net cost means VAT is not included in cost. | [default to null]
**purchase_order_number** | **String** | The Amazon purchase order number for this invoiced line item. Formatting Notes: 8-character alpha-numeric code. This value is mandatory only when invoiceType is Invoice, and is not required when invoiceType is CreditNote. | [optional] [default to null]
**hsn_code** | **String** | HSN Tax code. The HSN number cannot contain alphabets. | [optional] [default to null]
**credit_note_details** | [***::models::CreditNoteDetails**](CreditNoteDetails.md) | Details required in order to process a credit note. This information is required only if invoiceType is CreditNote. | [optional] [default to null]
**tax_details** | [**Vec<::models::TaxDetails>**](TaxDetails.md) | Individual tax details per line item. | [optional] [default to null]
**charge_details** | [**Vec<::models::ChargeDetails>**](ChargeDetails.md) | Individual charge details per line item. | [optional] [default to null]
**allowance_details** | [**Vec<::models::AllowanceDetails>**](AllowanceDetails.md) | Individual allowance details per line item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


