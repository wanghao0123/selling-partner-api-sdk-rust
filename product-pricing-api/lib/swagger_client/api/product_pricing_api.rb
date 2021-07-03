=begin
#Selling Partner API for Pricing

#The Selling Partner API for Pricing helps you programmatically retrieve product pricing and offer information for Amazon Marketplace products.

OpenAPI spec version: v0

Generated by: https://github.com/swagger-api/swagger-codegen.git
Swagger Codegen version: 2.4.21

=end

require 'uri'

module SwaggerClient
  class ProductPricingApi
    attr_accessor :api_client

    def initialize(api_client = ApiClient.default)
      @api_client = api_client
    end
    # Returns competitive pricing information for a seller's offer listings based on seller SKU or ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_type Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter. Possible values: Asin, Sku.
    # @param [Hash] opts the optional parameters
    # @option opts [Array<String>] :asins A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    # @option opts [Array<String>] :skus A list of up to twenty seller SKU values used to identify items in the given marketplace.
    # @option opts [String] :customer_type Indicates whether to request pricing information from the point of view of Consumer or Business buyers. Default is Consumer.
    # @return [GetPricingResponse]
    def get_competitive_pricing(marketplace_id, item_type, opts = {})
      data, _status_code, _headers = get_competitive_pricing_with_http_info(marketplace_id, item_type, opts)
      data
    end

    # Returns competitive pricing information for a seller&#39;s offer listings based on seller SKU or ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \&quot;Usage Plans and Rate Limits\&quot; in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_type Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter. Possible values: Asin, Sku.
    # @param [Hash] opts the optional parameters
    # @option opts [Array<String>] :asins A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    # @option opts [Array<String>] :skus A list of up to twenty seller SKU values used to identify items in the given marketplace.
    # @option opts [String] :customer_type Indicates whether to request pricing information from the point of view of Consumer or Business buyers. Default is Consumer.
    # @return [Array<(GetPricingResponse, Fixnum, Hash)>] GetPricingResponse data, response status code and response headers
    def get_competitive_pricing_with_http_info(marketplace_id, item_type, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: ProductPricingApi.get_competitive_pricing ...'
      end
      # verify the required parameter 'marketplace_id' is set
      if @api_client.config.client_side_validation && marketplace_id.nil?
        fail ArgumentError, "Missing the required parameter 'marketplace_id' when calling ProductPricingApi.get_competitive_pricing"
      end
      # verify the required parameter 'item_type' is set
      if @api_client.config.client_side_validation && item_type.nil?
        fail ArgumentError, "Missing the required parameter 'item_type' when calling ProductPricingApi.get_competitive_pricing"
      end
      # verify enum value
      if @api_client.config.client_side_validation && !['Asin', 'Sku'].include?(item_type)
        fail ArgumentError, "invalid value for 'item_type', must be one of Asin, Sku"
      end
      if @api_client.config.client_side_validation && !opts[:'asins'].nil? && opts[:'asins'].length > 20
        fail ArgumentError, 'invalid value for "opts[:"asins"]" when calling ProductPricingApi.get_competitive_pricing, number of items must be less than or equal to 20.'
      end

      if @api_client.config.client_side_validation && !opts[:'skus'].nil? && opts[:'skus'].length > 20
        fail ArgumentError, 'invalid value for "opts[:"skus"]" when calling ProductPricingApi.get_competitive_pricing, number of items must be less than or equal to 20.'
      end

      if @api_client.config.client_side_validation && opts[:'customer_type'] && !['Consumer', 'Business'].include?(opts[:'customer_type'])
        fail ArgumentError, 'invalid value for "customer_type", must be one of Consumer, Business'
      end
      # resource path
      local_var_path = '/products/pricing/v0/competitivePrice'

      # query parameters
      query_params = {}
      query_params[:'MarketplaceId'] = marketplace_id
      query_params[:'ItemType'] = item_type
      query_params[:'Asins'] = @api_client.build_collection_param(opts[:'asins'], :multi) if !opts[:'asins'].nil?
      query_params[:'Skus'] = @api_client.build_collection_param(opts[:'skus'], :multi) if !opts[:'skus'].nil?
      query_params[:'CustomerType'] = opts[:'customer_type'] if !opts[:'customer_type'].nil?

      # header parameters
      header_params = {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      header_params['Content-Type'] = @api_client.select_header_content_type(['application/json'])

      # form parameters
      form_params = {}

      # http body (model)
      post_body = nil
      auth_names = []
      data, status_code, headers = @api_client.call_api(:GET, local_var_path,
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => 'GetPricingResponse')
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: ProductPricingApi#get_competitive_pricing\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end
    # Returns the lowest priced offers for a single item based on ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_condition Filters the offer listings to be considered based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @param asin The Amazon Standard Identification Number (ASIN) of the item.
    # @param [Hash] opts the optional parameters
    # @option opts [String] :customer_type Indicates whether to request Consumer or Business offers. Default is Consumer.
    # @return [GetOffersResponse]
    def get_item_offers(marketplace_id, item_condition, asin, opts = {})
      data, _status_code, _headers = get_item_offers_with_http_info(marketplace_id, item_condition, asin, opts)
      data
    end

    # Returns the lowest priced offers for a single item based on ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \&quot;Usage Plans and Rate Limits\&quot; in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_condition Filters the offer listings to be considered based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @param asin The Amazon Standard Identification Number (ASIN) of the item.
    # @param [Hash] opts the optional parameters
    # @option opts [String] :customer_type Indicates whether to request Consumer or Business offers. Default is Consumer.
    # @return [Array<(GetOffersResponse, Fixnum, Hash)>] GetOffersResponse data, response status code and response headers
    def get_item_offers_with_http_info(marketplace_id, item_condition, asin, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: ProductPricingApi.get_item_offers ...'
      end
      # verify the required parameter 'marketplace_id' is set
      if @api_client.config.client_side_validation && marketplace_id.nil?
        fail ArgumentError, "Missing the required parameter 'marketplace_id' when calling ProductPricingApi.get_item_offers"
      end
      # verify the required parameter 'item_condition' is set
      if @api_client.config.client_side_validation && item_condition.nil?
        fail ArgumentError, "Missing the required parameter 'item_condition' when calling ProductPricingApi.get_item_offers"
      end
      # verify enum value
      if @api_client.config.client_side_validation && !['New', 'Used', 'Collectible', 'Refurbished', 'Club'].include?(item_condition)
        fail ArgumentError, "invalid value for 'item_condition', must be one of New, Used, Collectible, Refurbished, Club"
      end
      # verify the required parameter 'asin' is set
      if @api_client.config.client_side_validation && asin.nil?
        fail ArgumentError, "Missing the required parameter 'asin' when calling ProductPricingApi.get_item_offers"
      end
      if @api_client.config.client_side_validation && opts[:'customer_type'] && !['Consumer', 'Business'].include?(opts[:'customer_type'])
        fail ArgumentError, 'invalid value for "customer_type", must be one of Consumer, Business'
      end
      # resource path
      local_var_path = '/products/pricing/v0/items/{Asin}/offers'.sub('{' + 'Asin' + '}', asin.to_s)

      # query parameters
      query_params = {}
      query_params[:'MarketplaceId'] = marketplace_id
      query_params[:'ItemCondition'] = item_condition
      query_params[:'CustomerType'] = opts[:'customer_type'] if !opts[:'customer_type'].nil?

      # header parameters
      header_params = {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      header_params['Content-Type'] = @api_client.select_header_content_type(['application/json'])

      # form parameters
      form_params = {}

      # http body (model)
      post_body = nil
      auth_names = []
      data, status_code, headers = @api_client.call_api(:GET, local_var_path,
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => 'GetOffersResponse')
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: ProductPricingApi#get_item_offers\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end
    # Returns the lowest priced offers for a single SKU listing.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_condition Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @param seller_sku Identifies an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit.
    # @param [Hash] opts the optional parameters
    # @option opts [String] :customer_type Indicates whether to request Consumer or Business offers. Default is Consumer.
    # @return [GetOffersResponse]
    def get_listing_offers(marketplace_id, item_condition, seller_sku, opts = {})
      data, _status_code, _headers = get_listing_offers_with_http_info(marketplace_id, item_condition, seller_sku, opts)
      data
    end

    # Returns the lowest priced offers for a single SKU listing.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \&quot;Usage Plans and Rate Limits\&quot; in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_condition Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @param seller_sku Identifies an item in the given marketplace. SellerSKU is qualified by the seller&#39;s SellerId, which is included with every operation that you submit.
    # @param [Hash] opts the optional parameters
    # @option opts [String] :customer_type Indicates whether to request Consumer or Business offers. Default is Consumer.
    # @return [Array<(GetOffersResponse, Fixnum, Hash)>] GetOffersResponse data, response status code and response headers
    def get_listing_offers_with_http_info(marketplace_id, item_condition, seller_sku, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: ProductPricingApi.get_listing_offers ...'
      end
      # verify the required parameter 'marketplace_id' is set
      if @api_client.config.client_side_validation && marketplace_id.nil?
        fail ArgumentError, "Missing the required parameter 'marketplace_id' when calling ProductPricingApi.get_listing_offers"
      end
      # verify the required parameter 'item_condition' is set
      if @api_client.config.client_side_validation && item_condition.nil?
        fail ArgumentError, "Missing the required parameter 'item_condition' when calling ProductPricingApi.get_listing_offers"
      end
      # verify enum value
      if @api_client.config.client_side_validation && !['New', 'Used', 'Collectible', 'Refurbished', 'Club'].include?(item_condition)
        fail ArgumentError, "invalid value for 'item_condition', must be one of New, Used, Collectible, Refurbished, Club"
      end
      # verify the required parameter 'seller_sku' is set
      if @api_client.config.client_side_validation && seller_sku.nil?
        fail ArgumentError, "Missing the required parameter 'seller_sku' when calling ProductPricingApi.get_listing_offers"
      end
      if @api_client.config.client_side_validation && opts[:'customer_type'] && !['Consumer', 'Business'].include?(opts[:'customer_type'])
        fail ArgumentError, 'invalid value for "customer_type", must be one of Consumer, Business'
      end
      # resource path
      local_var_path = '/products/pricing/v0/listings/{SellerSKU}/offers'.sub('{' + 'SellerSKU' + '}', seller_sku.to_s)

      # query parameters
      query_params = {}
      query_params[:'MarketplaceId'] = marketplace_id
      query_params[:'ItemCondition'] = item_condition
      query_params[:'CustomerType'] = opts[:'customer_type'] if !opts[:'customer_type'].nil?

      # header parameters
      header_params = {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      header_params['Content-Type'] = @api_client.select_header_content_type(['application/json'])

      # form parameters
      form_params = {}

      # http body (model)
      post_body = nil
      auth_names = []
      data, status_code, headers = @api_client.call_api(:GET, local_var_path,
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => 'GetOffersResponse')
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: ProductPricingApi#get_listing_offers\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end
    # Returns pricing information for a seller's offer listings based on seller SKU or ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \"Usage Plans and Rate Limits\" in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_type Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter.
    # @param [Hash] opts the optional parameters
    # @option opts [Array<String>] :asins A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    # @option opts [Array<String>] :skus A list of up to twenty seller SKU values used to identify items in the given marketplace.
    # @option opts [String] :item_condition Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @option opts [String] :offer_type Indicates whether to request pricing information for the seller&#39;s B2C or B2B offers. Default is B2C.
    # @return [GetPricingResponse]
    def get_pricing(marketplace_id, item_type, opts = {})
      data, _status_code, _headers = get_pricing_with_http_info(marketplace_id, item_type, opts)
      data
    end

    # Returns pricing information for a seller&#39;s offer listings based on seller SKU or ASIN.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 10 | 20 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see \&quot;Usage Plans and Rate Limits\&quot; in the Selling Partner API documentation.
    # @param marketplace_id A marketplace identifier. Specifies the marketplace for which prices are returned.
    # @param item_type Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter.
    # @param [Hash] opts the optional parameters
    # @option opts [Array<String>] :asins A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    # @option opts [Array<String>] :skus A list of up to twenty seller SKU values used to identify items in the given marketplace.
    # @option opts [String] :item_condition Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club.
    # @option opts [String] :offer_type Indicates whether to request pricing information for the seller&#39;s B2C or B2B offers. Default is B2C.
    # @return [Array<(GetPricingResponse, Fixnum, Hash)>] GetPricingResponse data, response status code and response headers
    def get_pricing_with_http_info(marketplace_id, item_type, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: ProductPricingApi.get_pricing ...'
      end
      # verify the required parameter 'marketplace_id' is set
      if @api_client.config.client_side_validation && marketplace_id.nil?
        fail ArgumentError, "Missing the required parameter 'marketplace_id' when calling ProductPricingApi.get_pricing"
      end
      # verify the required parameter 'item_type' is set
      if @api_client.config.client_side_validation && item_type.nil?
        fail ArgumentError, "Missing the required parameter 'item_type' when calling ProductPricingApi.get_pricing"
      end
      # verify enum value
      if @api_client.config.client_side_validation && !['Asin', 'Sku'].include?(item_type)
        fail ArgumentError, "invalid value for 'item_type', must be one of Asin, Sku"
      end
      if @api_client.config.client_side_validation && !opts[:'asins'].nil? && opts[:'asins'].length > 20
        fail ArgumentError, 'invalid value for "opts[:"asins"]" when calling ProductPricingApi.get_pricing, number of items must be less than or equal to 20.'
      end

      if @api_client.config.client_side_validation && !opts[:'skus'].nil? && opts[:'skus'].length > 20
        fail ArgumentError, 'invalid value for "opts[:"skus"]" when calling ProductPricingApi.get_pricing, number of items must be less than or equal to 20.'
      end

      if @api_client.config.client_side_validation && opts[:'item_condition'] && !['New', 'Used', 'Collectible', 'Refurbished', 'Club'].include?(opts[:'item_condition'])
        fail ArgumentError, 'invalid value for "item_condition", must be one of New, Used, Collectible, Refurbished, Club'
      end
      if @api_client.config.client_side_validation && opts[:'offer_type'] && !['B2C', 'B2B'].include?(opts[:'offer_type'])
        fail ArgumentError, 'invalid value for "offer_type", must be one of B2C, B2B'
      end
      # resource path
      local_var_path = '/products/pricing/v0/price'

      # query parameters
      query_params = {}
      query_params[:'MarketplaceId'] = marketplace_id
      query_params[:'ItemType'] = item_type
      query_params[:'Asins'] = @api_client.build_collection_param(opts[:'asins'], :multi) if !opts[:'asins'].nil?
      query_params[:'Skus'] = @api_client.build_collection_param(opts[:'skus'], :multi) if !opts[:'skus'].nil?
      query_params[:'ItemCondition'] = opts[:'item_condition'] if !opts[:'item_condition'].nil?
      query_params[:'OfferType'] = opts[:'offer_type'] if !opts[:'offer_type'].nil?

      # header parameters
      header_params = {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      header_params['Content-Type'] = @api_client.select_header_content_type(['application/json'])

      # form parameters
      form_params = {}

      # http body (model)
      post_body = nil
      auth_names = []
      data, status_code, headers = @api_client.call_api(:GET, local_var_path,
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => 'GetPricingResponse')
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: ProductPricingApi#get_pricing\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end
  end
end
