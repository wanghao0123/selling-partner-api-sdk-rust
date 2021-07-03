=begin
#Selling Partner API for Pricing

#The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.

OpenAPI spec version: v0

Generated by: https://github.com/swagger-api/swagger-codegen.git
Swagger Codegen version: 2.4.21

=end

require 'date'

module SwaggerClient
  class FulfillmentChannelType
    
    AMAZON = 'Amazon'.freeze
    MERCHANT = 'Merchant'.freeze

    # Builds the enum from string
    # @param [String] The enum value in the form of the string
    # @return [String] The enum value
    def build_from_hash(value)
      constantValues = FulfillmentChannelType.constants.select { |c| FulfillmentChannelType::const_get(c) == value }
      raise "Invalid ENUM value #{value} for class #FulfillmentChannelType" if constantValues.empty?
      value
    end
  end
end
