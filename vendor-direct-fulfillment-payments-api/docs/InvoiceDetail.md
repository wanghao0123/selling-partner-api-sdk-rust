# InvoiceDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_number** | **String** | The unique invoice number. | [default to null]
**invoice_date** | **String** | Invoice date. | [default to null]
**reference_number** | **String** | An additional unique reference number used for regulatory or other purposes. | [optional] [default to null]
**remit_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party receiving the payment of this invoice. | [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Warehouse code of the vendor as in the order. | [default to null]
**bill_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party to whom this invoice is issued. | [optional] [default to null]
**ship_to_country_code** | **String** | Ship-to country code. | [optional] [default to null]
**payment_terms_code** | **String** | The payment terms for the invoice. | [optional] [default to null]
**invoice_total** | [***::models::Money**](Money.md) | Total amount details of the invoice. | [default to null]
**tax_totals** | [**Vec<::models::TaxDetail>**](TaxDetail.md) | Individual tax details per line item. | [optional] [default to null]
**additional_details** | [**Vec<::models::AdditionalDetails>**](AdditionalDetails.md) | Additional details provided by the selling party, for tax related or other purposes. | [optional] [default to null]
**charge_details** | [**Vec<::models::ChargeDetails>**](ChargeDetails.md) | Total charge amount details for all line items. | [optional] [default to null]
**items** | [**Vec<::models::InvoiceItem>**](InvoiceItem.md) | Provides the details of the items in this invoice. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


