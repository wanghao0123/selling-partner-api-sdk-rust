=begin
#Selling Partner API for Pricing

#The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.

OpenAPI spec version: v0

Generated by: https://github.com/swagger-api/swagger-codegen.git
Swagger Codegen version: 2.4.21

=end

# Common files
require 'swagger_client/api_client'
require 'swagger_client/api_error'
require 'swagger_client/version'
require 'swagger_client/configuration'

# Models
require 'swagger_client/models/asin_identifier'
require 'swagger_client/models/attribute_set_list'
require 'swagger_client/models/buy_box_eligible_offers'
require 'swagger_client/models/buy_box_price_type'
require 'swagger_client/models/buy_box_prices'
require 'swagger_client/models/competitive_price_list'
require 'swagger_client/models/competitive_price_type'
require 'swagger_client/models/competitive_pricing_type'
require 'swagger_client/models/condition_type'
require 'swagger_client/models/detailed_shipping_time_type'
require 'swagger_client/models/error'
require 'swagger_client/models/error_list'
require 'swagger_client/models/fulfillment_channel_type'
require 'swagger_client/models/get_offers_response'
require 'swagger_client/models/get_offers_result'
require 'swagger_client/models/get_pricing_response'
require 'swagger_client/models/identifier_type'
require 'swagger_client/models/item_identifier'
require 'swagger_client/models/lowest_price_type'
require 'swagger_client/models/lowest_prices'
require 'swagger_client/models/money_type'
require 'swagger_client/models/number_of_offer_listings_list'
require 'swagger_client/models/number_of_offers'
require 'swagger_client/models/offer_count_type'
require 'swagger_client/models/offer_customer_type'
require 'swagger_client/models/offer_detail'
require 'swagger_client/models/offer_detail_list'
require 'swagger_client/models/offer_listing_count_type'
require 'swagger_client/models/offer_type'
require 'swagger_client/models/offers_list'
require 'swagger_client/models/points'
require 'swagger_client/models/price'
require 'swagger_client/models/price_list'
require 'swagger_client/models/price_type'
require 'swagger_client/models/product'
require 'swagger_client/models/quantity_discount_price_type'
require 'swagger_client/models/quantity_discount_type'
require 'swagger_client/models/relationship_list'
require 'swagger_client/models/sales_rank_list'
require 'swagger_client/models/sales_rank_type'
require 'swagger_client/models/seller_feedback_type'
require 'swagger_client/models/seller_sku_identifier'
require 'swagger_client/models/ships_from_type'
require 'swagger_client/models/summary'

# APIs
require 'swagger_client/api/product_pricing_api'

module SwaggerClient
  class << self
    # Customize default settings for the SDK using block.
    #   SwaggerClient.configure do |config|
    #     config.username = "xxx"
    #     config.password = "xxx"
    #   end
    # If no block given, return the default Configuration object.
    def configure
      if block_given?
        yield(Configuration.default)
      else
        Configuration.default
      end
    end
  end
end
