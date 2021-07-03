# OrderItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [default to null]
**seller_sku** | **String** | The seller stock keeping unit (SKU) of the item. | [optional] [default to null]
**order_item_id** | **String** | An Amazon-defined order item identifier. | [default to null]
**title** | **String** | The name of the item. | [optional] [default to null]
**quantity_ordered** | **i32** | The number of items in the order.  | [default to null]
**quantity_shipped** | **i32** | The number of items shipped. | [optional] [default to null]
**product_info** | [***::models::ProductInfoDetail**](ProductInfoDetail.md) | Product information for the item. | [optional] [default to null]
**points_granted** | [***::models::PointsGrantedDetail**](PointsGrantedDetail.md) | The number and value of Amazon Points granted with the purchase of an item. | [optional] [default to null]
**item_price** | [***::models::Money**](Money.md) | The selling price of the order item. Note that an order item is an item and a quantity. This means that the value of ItemPrice is equal to the selling price of the item multiplied by the quantity ordered. Note that ItemPrice excludes ShippingPrice and GiftWrapPrice. | [optional] [default to null]
**shipping_price** | [***::models::Money**](Money.md) | The shipping price of the item. | [optional] [default to null]
**item_tax** | [***::models::Money**](Money.md) | The tax on the item price. | [optional] [default to null]
**shipping_tax** | [***::models::Money**](Money.md) | The tax on the shipping price. | [optional] [default to null]
**shipping_discount** | [***::models::Money**](Money.md) | The discount on the shipping price. | [optional] [default to null]
**shipping_discount_tax** | [***::models::Money**](Money.md) | The tax on the discount on the shipping price. | [optional] [default to null]
**promotion_discount** | [***::models::Money**](Money.md) | The total of all promotional discounts in the offer. | [optional] [default to null]
**promotion_discount_tax** | [***::models::Money**](Money.md) | The tax on the total of all promotional discounts in the offer. | [optional] [default to null]
**promotion_ids** | [***::models::PromotionIdList**](PromotionIdList.md) |  | [optional] [default to null]
**cod_fee** | [***::models::Money**](Money.md) | The fee charged for COD service. | [optional] [default to null]
**cod_fee_discount** | [***::models::Money**](Money.md) | The discount on the COD fee. | [optional] [default to null]
**is_gift** | **bool** | When true, the item is a gift. | [optional] [default to null]
**condition_note** | **String** | The condition of the item as described by the seller. | [optional] [default to null]
**condition_id** | **String** | The condition of the item.  Possible values: New, Used, Collectible, Refurbished, Preorder, Club. | [optional] [default to null]
**condition_subtype_id** | **String** | The subcondition of the item.  Possible values: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, Any, Other. | [optional] [default to null]
**scheduled_delivery_start_date** | **String** | The start date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format. | [optional] [default to null]
**scheduled_delivery_end_date** | **String** | The end date of the scheduled delivery window in the time zone of the order destination. In ISO 8601 date time format. | [optional] [default to null]
**price_designation** | **String** | Indicates that the selling price is a special price that is available only for Amazon Business orders. For more information about the Amazon Business Seller Program, see the [Amazon Business website](https://www.amazon.com/b2b/info/amazon-business).   Possible values: BusinessPrice - A special price that is available only for Amazon Business orders. | [optional] [default to null]
**tax_collection** | [***::models::TaxCollection**](TaxCollection.md) | Information about withheld taxes. | [optional] [default to null]
**serial_number_required** | **bool** | When true, the product type for this item has a serial number.  Returned only for Amazon Easy Ship orders. | [optional] [default to null]
**is_transparency** | **bool** | When true, transparency codes are required. | [optional] [default to null]
**ioss_number** | **String** | The IOSS number of the seller. Sellers selling in the EU will be assigned a unique IOSS number that must be listed on all packages sent to the EU. | [optional] [default to null]
**store_chain_store_id** | **String** | The store chain store identifier. Linked to a specific store in a store chain. | [optional] [default to null]
**deemed_reseller_category** | **String** | The category of deemed reseller. This applies to selling partners that are not based in the EU and is used to help them meet the VAT Deemed Reseller tax laws in the EU and UK. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


