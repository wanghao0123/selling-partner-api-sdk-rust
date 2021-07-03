/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SetAppointmentResponse : Response schema for add or reschedule appointment operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetAppointmentResponse {
  /// New appointment id generated during add or reschedule appointment operation.
  #[serde(rename = "appointmentId")]
  appointment_id: Option<::models::AppointmentId>,
  /// Warnings generated during add or reschedule appointment operation.
  #[serde(rename = "warnings")]
  warnings: Option<::models::WarningList>,
  /// Errors occurred during during add or reschedule appointment operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl SetAppointmentResponse {
  /// Response schema for add or reschedule appointment operation.
  pub fn new() -> SetAppointmentResponse {
    SetAppointmentResponse {
      appointment_id: None,
      warnings: None,
      errors: None
    }
  }

  pub fn set_appointment_id(&mut self, appointment_id: ::models::AppointmentId) {
    self.appointment_id = Some(appointment_id);
  }

  pub fn with_appointment_id(mut self, appointment_id: ::models::AppointmentId) -> SetAppointmentResponse {
    self.appointment_id = Some(appointment_id);
    self
  }

  pub fn appointment_id(&self) -> Option<&::models::AppointmentId> {
    self.appointment_id.as_ref()
  }

  pub fn reset_appointment_id(&mut self) {
    self.appointment_id = None;
  }

  pub fn set_warnings(&mut self, warnings: ::models::WarningList) {
    self.warnings = Some(warnings);
  }

  pub fn with_warnings(mut self, warnings: ::models::WarningList) -> SetAppointmentResponse {
    self.warnings = Some(warnings);
    self
  }

  pub fn warnings(&self) -> Option<&::models::WarningList> {
    self.warnings.as_ref()
  }

  pub fn reset_warnings(&mut self) {
    self.warnings = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> SetAppointmentResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&::models::ErrorList> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

}



