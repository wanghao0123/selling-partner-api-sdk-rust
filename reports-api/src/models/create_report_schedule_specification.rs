/* 
 * Selling Partner API for Reports
 *
 * The Selling Partner API for Reports lets you retrieve and manage a variety of reports that can help selling partners manage their businesses.
 *
 * OpenAPI spec version: 2021-06-30
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReportScheduleSpecification {
  /// The report type.
  #[serde(rename = "reportType")]
  report_type: String,
  /// A list of marketplace identifiers for the report schedule.
  #[serde(rename = "marketplaceIds")]
  marketplace_ids: Vec<String>,
  #[serde(rename = "reportOptions")]
  report_options: Option<::models::ReportOptions>,
  /// One of a set of predefined ISO 8601 periods that specifies how often a report should be created.
  #[serde(rename = "period")]
  period: String,
  /// The date and time when the schedule will create its next report, in ISO 8601 date time format.
  #[serde(rename = "nextReportCreationTime")]
  next_report_creation_time: Option<String>
}

impl CreateReportScheduleSpecification {
  pub fn new(report_type: String, marketplace_ids: Vec<String>, period: String) -> CreateReportScheduleSpecification {
    CreateReportScheduleSpecification {
      report_type: report_type,
      marketplace_ids: marketplace_ids,
      report_options: None,
      period: period,
      next_report_creation_time: None
    }
  }

  pub fn set_report_type(&mut self, report_type: String) {
    self.report_type = report_type;
  }

  pub fn with_report_type(mut self, report_type: String) -> CreateReportScheduleSpecification {
    self.report_type = report_type;
    self
  }

  pub fn report_type(&self) -> &String {
    &self.report_type
  }


  pub fn set_marketplace_ids(&mut self, marketplace_ids: Vec<String>) {
    self.marketplace_ids = marketplace_ids;
  }

  pub fn with_marketplace_ids(mut self, marketplace_ids: Vec<String>) -> CreateReportScheduleSpecification {
    self.marketplace_ids = marketplace_ids;
    self
  }

  pub fn marketplace_ids(&self) -> &Vec<String> {
    &self.marketplace_ids
  }


  pub fn set_report_options(&mut self, report_options: ::models::ReportOptions) {
    self.report_options = Some(report_options);
  }

  pub fn with_report_options(mut self, report_options: ::models::ReportOptions) -> CreateReportScheduleSpecification {
    self.report_options = Some(report_options);
    self
  }

  pub fn report_options(&self) -> Option<&::models::ReportOptions> {
    self.report_options.as_ref()
  }

  pub fn reset_report_options(&mut self) {
    self.report_options = None;
  }

  pub fn set_period(&mut self, period: String) {
    self.period = period;
  }

  pub fn with_period(mut self, period: String) -> CreateReportScheduleSpecification {
    self.period = period;
    self
  }

  pub fn period(&self) -> &String {
    &self.period
  }


  pub fn set_next_report_creation_time(&mut self, next_report_creation_time: String) {
    self.next_report_creation_time = Some(next_report_creation_time);
  }

  pub fn with_next_report_creation_time(mut self, next_report_creation_time: String) -> CreateReportScheduleSpecification {
    self.next_report_creation_time = Some(next_report_creation_time);
    self
  }

  pub fn next_report_creation_time(&self) -> Option<&String> {
    self.next_report_creation_time.as_ref()
  }

  pub fn reset_next_report_creation_time(&mut self) {
    self.next_report_creation_time = None;
  }

}



