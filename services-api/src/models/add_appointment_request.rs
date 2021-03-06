/* 
 * Selling Partner API for Services
 *
 * With the Services API, you can build applications that help service providers get and modify their service orders.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AddAppointmentRequest : Input for add appointment operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAppointmentRequest {
  /// Input appointment time details.
  #[serde(rename = "appointmentTime")]
  appointment_time: ::models::AppointmentTimeInput
}

impl AddAppointmentRequest {
  /// Input for add appointment operation.
  pub fn new(appointment_time: ::models::AppointmentTimeInput) -> AddAppointmentRequest {
    AddAppointmentRequest {
      appointment_time: appointment_time
    }
  }

  pub fn set_appointment_time(&mut self, appointment_time: ::models::AppointmentTimeInput) {
    self.appointment_time = appointment_time;
  }

  pub fn with_appointment_time(mut self, appointment_time: ::models::AppointmentTimeInput) -> AddAppointmentRequest {
    self.appointment_time = appointment_time;
    self
  }

  pub fn appointment_time(&self) -> &::models::AppointmentTimeInput {
    &self.appointment_time
  }


}



