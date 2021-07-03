# Invoice

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_type** | **String** | Identifies the type of invoice. | [default to null]
**id** | **String** | Unique number relating to the charges defined in this document. This will be invoice number if the document type is Invoice or CreditNote number if the document type is Credit Note. Failure to provide this reference will result in a rejection. | [default to null]
**reference_number** | **String** | An additional unique reference number used for regulatory or other purposes. | [optional] [default to null]
**date** | **String** | Date when the invoice/credit note information was generated in the origin&#39;s accounting system. The invoice date should be on or after the purchase order creation date. | [default to null]
**remit_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party receiving the payment of this invoice. | [default to null]
**ship_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party receiving a shipment of products. | [optional] [default to null]
**ship_from_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party sending a shipment of products. | [optional] [default to null]
**bill_to_party** | [***::models::PartyIdentification**](PartyIdentification.md) | Name, address and tax details of the party to whom this invoice is issued. | [optional] [default to null]
**payment_terms** | [***::models::PaymentTerms**](PaymentTerms.md) | The payment terms for the invoice. | [optional] [default to null]
**invoice_total** | [***::models::Money**](Money.md) | Total monetary amount charged in the invoice or full value of credit note to be paid including all relevant taxes. It is the total amount of invoice (including charges, less allowances) before terms discount (if discount is applicable). | [default to null]
**tax_details** | [**Vec<::models::TaxDetails>**](TaxDetails.md) | Total tax amount details for all line items. | [optional] [default to null]
**additional_details** | [**Vec<::models::AdditionalDetails>**](AdditionalDetails.md) | Additional details provided by the selling party, for tax related or other purposes. | [optional] [default to null]
**charge_details** | [**Vec<::models::ChargeDetails>**](ChargeDetails.md) | Total charge amount details for all line items. | [optional] [default to null]
**allowance_details** | [**Vec<::models::AllowanceDetails>**](AllowanceDetails.md) | Total allowance amount details for all line items. | [optional] [default to null]
**items** | [**Vec<::models::InvoiceItem>**](InvoiceItem.md) | The list of invoice items. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


