# QualifiersType

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_condition** | **String** | The condition of the item. Possible values: New, Used, Collectible, Refurbished, or Club. | [default to null]
**item_subcondition** | **String** | The item subcondition for the offer listing. Possible values: New, Mint, Very Good, Good, Acceptable, Poor, Club, OEM, Warranty, Refurbished Warranty, Refurbished, Open Box, or Other. | [default to null]
**fulfillment_channel** | **String** | The fulfillment channel for the item. Possible values:  * Amazon - Fulfilled by Amazon. * Merchant - Fulfilled by the seller. | [default to null]
**ships_domestically** | **String** | Indicates whether the marketplace specified in the request and the location that the item ships from are in the same country. Possible values: True, False, or Unknown. | [default to null]
**shipping_time** | [***::models::ShippingTimeType**](ShippingTimeType.md) | (0-2 days, 3-7 days, 8-13 days, or 14 or more days) – Indicates the maximum time within which the item will likely be shipped once an order has been placed. | [default to null]
**seller_positive_feedback_rating** | **String** | (98-100%, 95-97%, 90-94%, 80-89%, 70-79%, Less than 70%, or Just launched ) – Indicates the percentage of feedback ratings that were positive over the past 12 months. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


