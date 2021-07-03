# InvoiceItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | [default to null]
**buyer_product_identifier** | **String** | Buyer&#39;s standard identification number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. | [optional] [default to null]
**invoiced_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Item quantity invoiced. | [default to null]
**net_cost** | [***::models::Money**](Money.md) | Net price (before tax) to vendor with currency details. | [default to null]
**purchase_order_number** | **String** | The purchase order number for this order. Formatting Notes: 8-character alpha-numeric code. | [default to null]
**vendor_order_number** | **String** | The vendor&#39;s order number for this order. | [optional] [default to null]
**hsn_code** | **String** | HSN tax code. The HSN number cannot contain alphabets. | [optional] [default to null]
**tax_details** | [**Vec<::models::TaxDetail>**](TaxDetail.md) | Individual tax details per line item. | [optional] [default to null]
**charge_details** | [**Vec<::models::ChargeDetails>**](ChargeDetails.md) | Individual charge details per line item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


