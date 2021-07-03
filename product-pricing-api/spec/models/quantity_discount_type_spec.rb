=begin
#Selling Partner API for Pricing

#The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.

OpenAPI spec version: v0

Generated by: https://github.com/swagger-api/swagger-codegen.git
Swagger Codegen version: 2.4.21

=end

require 'spec_helper'
require 'json'
require 'date'

# Unit tests for SwaggerClient::QuantityDiscountType
# Automatically generated by swagger-codegen (github.com/swagger-api/swagger-codegen)
# Please update as you see appropriate
describe 'QuantityDiscountType' do
  before do
    # run before each test
    @instance = SwaggerClient::QuantityDiscountType.new
  end

  after do
    # run after each test
  end

  describe 'test an instance of QuantityDiscountType' do
    it 'should create an instance of QuantityDiscountType' do
      expect(@instance).to be_instance_of(SwaggerClient::QuantityDiscountType)
    end
  end
end
