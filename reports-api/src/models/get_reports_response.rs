/* 
 * Selling Partner API for Reports
 *
 * The Selling Partner API for Reports lets you retrieve and manage a variety of reports that can help selling partners manage their businesses.
 *
 * OpenAPI spec version: 2021-06-30
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetReportsResponse : The response for the getReports operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetReportsResponse {
  /// The reports.
  #[serde(rename = "reports")]
  reports: ::models::ReportList,
  /// Returned when the number of results exceeds pageSize. To get the next page of results, call getReports with this token as the only parameter.
  #[serde(rename = "nextToken")]
  next_token: Option<String>
}

impl GetReportsResponse {
  /// The response for the getReports operation.
  pub fn new(reports: ::models::ReportList) -> GetReportsResponse {
    GetReportsResponse {
      reports: reports,
      next_token: None
    }
  }

  pub fn set_reports(&mut self, reports: ::models::ReportList) {
    self.reports = reports;
  }

  pub fn with_reports(mut self, reports: ::models::ReportList) -> GetReportsResponse {
    self.reports = reports;
    self
  }

  pub fn reports(&self) -> &::models::ReportList {
    &self.reports
  }


  pub fn set_next_token(&mut self, next_token: String) {
    self.next_token = Some(next_token);
  }

  pub fn with_next_token(mut self, next_token: String) -> GetReportsResponse {
    self.next_token = Some(next_token);
    self
  }

  pub fn next_token(&self) -> Option<&String> {
    self.next_token.as_ref()
  }

  pub fn reset_next_token(&mut self) {
    self.next_token = None;
  }

}



