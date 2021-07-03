# SwaggerClient::Summary

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_offer_count** | **Integer** | The number of unique offers contained in NumberOfOffers. | 
**number_of_offers** | [**NumberOfOffers**](NumberOfOffers.md) | A list that contains the total number of offers for the item for the given conditions and fulfillment channels. | [optional] 
**lowest_prices** | [**LowestPrices**](LowestPrices.md) | A list of the lowest prices for the item. | [optional] 
**buy_box_prices** | [**BuyBoxPrices**](BuyBoxPrices.md) | A list of item prices. | [optional] 
**list_price** | [**MoneyType**](MoneyType.md) | The list price of the item as suggested by the manufacturer. | [optional] 
**suggested_lower_price_plus_shipping** | [**MoneyType**](MoneyType.md) | The suggested lower price of the item, including shipping and Amazon Points. The suggested lower price is based on a range of factors, including historical selling prices, recent Buy Box-eligible prices, and input from customers for your products. | [optional] 
**buy_box_eligible_offers** | [**BuyBoxEligibleOffers**](BuyBoxEligibleOffers.md) | A list that contains the total number of offers that are eligible for the Buy Box for the given conditions and fulfillment channels. | [optional] 
**offers_available_time** | **DateTime** | When the status is ActiveButTooSoonForProcessing, this is the time when the offers will be available for processing. | [optional] 


