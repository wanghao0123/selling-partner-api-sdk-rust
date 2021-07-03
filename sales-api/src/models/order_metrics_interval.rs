/* 
 * Selling Partner API for Sales
 *
 * The Selling Partner API for Sales provides APIs related to sales performance.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OrderMetricsInterval : Contains order metrics.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderMetricsInterval {
  /// The interval of time based on requested granularity (ex. Hour, Day, etc.) If this is the first or the last interval from the list, it might contain incomplete data if the requested interval doesn't align with the requested granularity (ex. request interval 2018-09-01T02:00:00Z--2018-09-04T19:00:00Z and granularity day will result in Sept 1st UTC day and Sept 4th UTC days having partial data).
  #[serde(rename = "interval")]
  interval: String,
  /// The number of units in orders based on the specified filters.
  #[serde(rename = "unitCount")]
  unit_count: i32,
  /// The number of order items based on the specified filters.
  #[serde(rename = "orderItemCount")]
  order_item_count: i32,
  /// The number of orders based on the specified filters.
  #[serde(rename = "orderCount")]
  order_count: i32,
  /// The average price for an item based on the specified filters. Formula is totalSales/unitCount.
  #[serde(rename = "averageUnitPrice")]
  average_unit_price: ::models::Money,
  /// The total ordered product sales for all orders based on the specified filters.
  #[serde(rename = "totalSales")]
  total_sales: ::models::Money
}

impl OrderMetricsInterval {
  /// Contains order metrics.
  pub fn new(interval: String, unit_count: i32, order_item_count: i32, order_count: i32, average_unit_price: ::models::Money, total_sales: ::models::Money) -> OrderMetricsInterval {
    OrderMetricsInterval {
      interval: interval,
      unit_count: unit_count,
      order_item_count: order_item_count,
      order_count: order_count,
      average_unit_price: average_unit_price,
      total_sales: total_sales
    }
  }

  pub fn set_interval(&mut self, interval: String) {
    self.interval = interval;
  }

  pub fn with_interval(mut self, interval: String) -> OrderMetricsInterval {
    self.interval = interval;
    self
  }

  pub fn interval(&self) -> &String {
    &self.interval
  }


  pub fn set_unit_count(&mut self, unit_count: i32) {
    self.unit_count = unit_count;
  }

  pub fn with_unit_count(mut self, unit_count: i32) -> OrderMetricsInterval {
    self.unit_count = unit_count;
    self
  }

  pub fn unit_count(&self) -> &i32 {
    &self.unit_count
  }


  pub fn set_order_item_count(&mut self, order_item_count: i32) {
    self.order_item_count = order_item_count;
  }

  pub fn with_order_item_count(mut self, order_item_count: i32) -> OrderMetricsInterval {
    self.order_item_count = order_item_count;
    self
  }

  pub fn order_item_count(&self) -> &i32 {
    &self.order_item_count
  }


  pub fn set_order_count(&mut self, order_count: i32) {
    self.order_count = order_count;
  }

  pub fn with_order_count(mut self, order_count: i32) -> OrderMetricsInterval {
    self.order_count = order_count;
    self
  }

  pub fn order_count(&self) -> &i32 {
    &self.order_count
  }


  pub fn set_average_unit_price(&mut self, average_unit_price: ::models::Money) {
    self.average_unit_price = average_unit_price;
  }

  pub fn with_average_unit_price(mut self, average_unit_price: ::models::Money) -> OrderMetricsInterval {
    self.average_unit_price = average_unit_price;
    self
  }

  pub fn average_unit_price(&self) -> &::models::Money {
    &self.average_unit_price
  }


  pub fn set_total_sales(&mut self, total_sales: ::models::Money) {
    self.total_sales = total_sales;
  }

  pub fn with_total_sales(mut self, total_sales: ::models::Money) -> OrderMetricsInterval {
    self.total_sales = total_sales;
    self
  }

  pub fn total_sales(&self) -> &::models::Money {
    &self.total_sales
  }


}



