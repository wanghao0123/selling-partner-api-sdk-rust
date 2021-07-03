# Order

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | [default to null]
**seller_order_id** | **String** | A seller-defined order identifier. | [optional] [default to null]
**purchase_date** | **String** | The date when the order was created. | [default to null]
**last_update_date** | **String** | The date when the order was last updated.  Note: LastUpdateDate is returned with an incorrect date for orders that were last updated before 2009-04-01. | [default to null]
**order_status** | **String** | The current order status. | [default to null]
**fulfillment_channel** | **String** | Whether the order was fulfilled by Amazon (AFN) or by the seller (MFN). | [optional] [default to null]
**sales_channel** | **String** | The sales channel of the first item in the order. | [optional] [default to null]
**order_channel** | **String** | The order channel of the first item in the order. | [optional] [default to null]
**ship_service_level** | **String** | The shipment service level of the order. | [optional] [default to null]
**order_total** | [***::models::Money**](Money.md) | The total charge for this order. | [optional] [default to null]
**number_of_items_shipped** | **i32** | The number of items shipped. | [optional] [default to null]
**number_of_items_unshipped** | **i32** | The number of items unshipped. | [optional] [default to null]
**payment_execution_detail** | [***::models::PaymentExecutionDetailItemList**](PaymentExecutionDetailItemList.md) | Information about sub-payment methods for a Cash On Delivery (COD) order.  Note: For a COD order that is paid for using one sub-payment method, one PaymentExecutionDetailItem object is returned, with PaymentExecutionDetailItem/PaymentMethod &#x3D; COD. For a COD order that is paid for using multiple sub-payment methods, two or more PaymentExecutionDetailItem objects are returned. | [optional] [default to null]
**payment_method** | **String** | The payment method for the order. This property is limited to Cash On Delivery (COD) and Convenience Store (CVS) payment methods. Unless you need the specific COD payment information provided by the PaymentExecutionDetailItem object, we recommend using the PaymentMethodDetails property to get payment method information. | [optional] [default to null]
**payment_method_details** | [***::models::PaymentMethodDetailItemList**](PaymentMethodDetailItemList.md) | A list of payment methods for the order. | [optional] [default to null]
**marketplace_id** | **String** | The identifier for the marketplace where the order was placed. | [optional] [default to null]
**shipment_service_level_category** | **String** | The shipment service level category of the order.  Possible values: Expedited, FreeEconomy, NextDay, SameDay, SecondDay, Scheduled, Standard. | [optional] [default to null]
**easy_ship_shipment_status** | **String** | The status of the Amazon Easy Ship order. This property is included only for Amazon Easy Ship orders.  Possible values: PendingPickUp, LabelCanceled, PickedUp, OutForDelivery, Damaged, Delivered, RejectedByBuyer, Undeliverable, ReturnedToSeller, ReturningToSeller. | [optional] [default to null]
**cba_displayable_shipping_label** | **String** | Custom ship label for Checkout by Amazon (CBA). | [optional] [default to null]
**order_type** | **String** | The type of the order. | [optional] [default to null]
**earliest_ship_date** | **String** | The start of the time period within which you have committed to ship the order. In ISO 8601 date time format. Returned only for seller-fulfilled orders.  Note: EarliestShipDate might not be returned for orders placed before February 1, 2013. | [optional] [default to null]
**latest_ship_date** | **String** | The end of the time period within which you have committed to ship the order. In ISO 8601 date time format. Returned only for seller-fulfilled orders.  Note: LatestShipDate might not be returned for orders placed before February 1, 2013. | [optional] [default to null]
**earliest_delivery_date** | **String** | The start of the time period within which you have committed to fulfill the order. In ISO 8601 date time format. Returned only for seller-fulfilled orders. | [optional] [default to null]
**latest_delivery_date** | **String** | The end of the time period within which you have committed to fulfill the order. In ISO 8601 date time format. Returned only for seller-fulfilled orders that do not have a PendingAvailability, Pending, or Canceled status. | [optional] [default to null]
**is_business_order** | **bool** | When true, the order is an Amazon Business order. An Amazon Business order is an order where the buyer is a Verified Business Buyer. | [optional] [default to null]
**is_prime** | **bool** | When true, the order is a seller-fulfilled Amazon Prime order. | [optional] [default to null]
**is_premium_order** | **bool** | When true, the order has a Premium Shipping Service Level Agreement. For more information about Premium Shipping orders, see \&quot;Premium Shipping Options\&quot; in the Seller Central Help for your marketplace. | [optional] [default to null]
**is_global_express_enabled** | **bool** | When true, the order is a GlobalExpress order. | [optional] [default to null]
**replaced_order_id** | **String** | The order ID value for the order that is being replaced. Returned only if IsReplacementOrder &#x3D; true. | [optional] [default to null]
**is_replacement_order** | **bool** | When true, this is a replacement order. | [optional] [default to null]
**promise_response_due_date** | **String** | Indicates the date by which the seller must respond to the buyer with an estimated ship date. Returned only for Sourcing on Demand orders. | [optional] [default to null]
**is_estimated_ship_date_set** | **bool** | When true, the estimated ship date is set for the order. Returned only for Sourcing on Demand orders. | [optional] [default to null]
**is_sold_by_ab** | **bool** | When true, the item within this order was bought and re-sold by Amazon Business EU SARL (ABEU). By buying and instantly re-selling your items, ABEU becomes the seller of record, making your inventory available for sale to customers who would not otherwise purchase from a third-party seller. | [optional] [default to null]
**default_ship_from_location_address** | [***::models::Address**](Address.md) | The recommended location for the seller to ship the items from. It is calculated at checkout. The seller may or may not choose to ship from this location. | [optional] [default to null]
**fulfillment_instruction** | [***::models::FulfillmentInstruction**](FulfillmentInstruction.md) | Contains the instructions about the fulfillment like where should it be fulfilled from. | [optional] [default to null]
**is_ispu** | **bool** | When true, this order is marked to be picked up from a store rather than delivered. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


