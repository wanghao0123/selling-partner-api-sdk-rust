/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FinancialEvents : Contains all information related to a financial event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialEvents {
  /// A list of shipment events.
  #[serde(rename = "ShipmentEventList")]
  shipment_event_list: Option<::models::ShipmentEventList>,
  /// A list of refund events.
  #[serde(rename = "RefundEventList")]
  refund_event_list: Option<::models::ShipmentEventList>,
  /// A list of guarantee claim events.
  #[serde(rename = "GuaranteeClaimEventList")]
  guarantee_claim_event_list: Option<::models::ShipmentEventList>,
  /// A list of chargeback events.
  #[serde(rename = "ChargebackEventList")]
  chargeback_event_list: Option<::models::ShipmentEventList>,
  #[serde(rename = "PayWithAmazonEventList")]
  pay_with_amazon_event_list: Option<::models::PayWithAmazonEventList>,
  #[serde(rename = "ServiceProviderCreditEventList")]
  service_provider_credit_event_list: Option<::models::SolutionProviderCreditEventList>,
  #[serde(rename = "RetrochargeEventList")]
  retrocharge_event_list: Option<::models::RetrochargeEventList>,
  #[serde(rename = "RentalTransactionEventList")]
  rental_transaction_event_list: Option<::models::RentalTransactionEventList>,
  #[serde(rename = "ProductAdsPaymentEventList")]
  product_ads_payment_event_list: Option<::models::ProductAdsPaymentEventList>,
  #[serde(rename = "ServiceFeeEventList")]
  service_fee_event_list: Option<::models::ServiceFeeEventList>,
  #[serde(rename = "SellerDealPaymentEventList")]
  seller_deal_payment_event_list: Option<::models::SellerDealPaymentEventList>,
  #[serde(rename = "DebtRecoveryEventList")]
  debt_recovery_event_list: Option<::models::DebtRecoveryEventList>,
  #[serde(rename = "LoanServicingEventList")]
  loan_servicing_event_list: Option<::models::LoanServicingEventList>,
  #[serde(rename = "AdjustmentEventList")]
  adjustment_event_list: Option<::models::AdjustmentEventList>,
  #[serde(rename = "SAFETReimbursementEventList")]
  safet_reimbursement_event_list: Option<::models::SafetReimbursementEventList>,
  #[serde(rename = "SellerReviewEnrollmentPaymentEventList")]
  seller_review_enrollment_payment_event_list: Option<::models::SellerReviewEnrollmentPaymentEventList>,
  #[serde(rename = "FBALiquidationEventList")]
  fba_liquidation_event_list: Option<::models::FbaLiquidationEventList>,
  #[serde(rename = "CouponPaymentEventList")]
  coupon_payment_event_list: Option<::models::CouponPaymentEventList>,
  #[serde(rename = "ImagingServicesFeeEventList")]
  imaging_services_fee_event_list: Option<::models::ImagingServicesFeeEventList>,
  #[serde(rename = "NetworkComminglingTransactionEventList")]
  network_commingling_transaction_event_list: Option<::models::NetworkComminglingTransactionEventList>,
  #[serde(rename = "AffordabilityExpenseEventList")]
  affordability_expense_event_list: Option<::models::AffordabilityExpenseEventList>,
  #[serde(rename = "AffordabilityExpenseReversalEventList")]
  affordability_expense_reversal_event_list: Option<::models::AffordabilityExpenseEventList>
}

impl FinancialEvents {
  /// Contains all information related to a financial event.
  pub fn new() -> FinancialEvents {
    FinancialEvents {
      shipment_event_list: None,
      refund_event_list: None,
      guarantee_claim_event_list: None,
      chargeback_event_list: None,
      pay_with_amazon_event_list: None,
      service_provider_credit_event_list: None,
      retrocharge_event_list: None,
      rental_transaction_event_list: None,
      product_ads_payment_event_list: None,
      service_fee_event_list: None,
      seller_deal_payment_event_list: None,
      debt_recovery_event_list: None,
      loan_servicing_event_list: None,
      adjustment_event_list: None,
      safet_reimbursement_event_list: None,
      seller_review_enrollment_payment_event_list: None,
      fba_liquidation_event_list: None,
      coupon_payment_event_list: None,
      imaging_services_fee_event_list: None,
      network_commingling_transaction_event_list: None,
      affordability_expense_event_list: None,
      affordability_expense_reversal_event_list: None
    }
  }

  pub fn set_shipment_event_list(&mut self, shipment_event_list: ::models::ShipmentEventList) {
    self.shipment_event_list = Some(shipment_event_list);
  }

  pub fn with_shipment_event_list(mut self, shipment_event_list: ::models::ShipmentEventList) -> FinancialEvents {
    self.shipment_event_list = Some(shipment_event_list);
    self
  }

  pub fn shipment_event_list(&self) -> Option<&::models::ShipmentEventList> {
    self.shipment_event_list.as_ref()
  }

  pub fn reset_shipment_event_list(&mut self) {
    self.shipment_event_list = None;
  }

  pub fn set_refund_event_list(&mut self, refund_event_list: ::models::ShipmentEventList) {
    self.refund_event_list = Some(refund_event_list);
  }

  pub fn with_refund_event_list(mut self, refund_event_list: ::models::ShipmentEventList) -> FinancialEvents {
    self.refund_event_list = Some(refund_event_list);
    self
  }

  pub fn refund_event_list(&self) -> Option<&::models::ShipmentEventList> {
    self.refund_event_list.as_ref()
  }

  pub fn reset_refund_event_list(&mut self) {
    self.refund_event_list = None;
  }

  pub fn set_guarantee_claim_event_list(&mut self, guarantee_claim_event_list: ::models::ShipmentEventList) {
    self.guarantee_claim_event_list = Some(guarantee_claim_event_list);
  }

  pub fn with_guarantee_claim_event_list(mut self, guarantee_claim_event_list: ::models::ShipmentEventList) -> FinancialEvents {
    self.guarantee_claim_event_list = Some(guarantee_claim_event_list);
    self
  }

  pub fn guarantee_claim_event_list(&self) -> Option<&::models::ShipmentEventList> {
    self.guarantee_claim_event_list.as_ref()
  }

  pub fn reset_guarantee_claim_event_list(&mut self) {
    self.guarantee_claim_event_list = None;
  }

  pub fn set_chargeback_event_list(&mut self, chargeback_event_list: ::models::ShipmentEventList) {
    self.chargeback_event_list = Some(chargeback_event_list);
  }

  pub fn with_chargeback_event_list(mut self, chargeback_event_list: ::models::ShipmentEventList) -> FinancialEvents {
    self.chargeback_event_list = Some(chargeback_event_list);
    self
  }

  pub fn chargeback_event_list(&self) -> Option<&::models::ShipmentEventList> {
    self.chargeback_event_list.as_ref()
  }

  pub fn reset_chargeback_event_list(&mut self) {
    self.chargeback_event_list = None;
  }

  pub fn set_pay_with_amazon_event_list(&mut self, pay_with_amazon_event_list: ::models::PayWithAmazonEventList) {
    self.pay_with_amazon_event_list = Some(pay_with_amazon_event_list);
  }

  pub fn with_pay_with_amazon_event_list(mut self, pay_with_amazon_event_list: ::models::PayWithAmazonEventList) -> FinancialEvents {
    self.pay_with_amazon_event_list = Some(pay_with_amazon_event_list);
    self
  }

  pub fn pay_with_amazon_event_list(&self) -> Option<&::models::PayWithAmazonEventList> {
    self.pay_with_amazon_event_list.as_ref()
  }

  pub fn reset_pay_with_amazon_event_list(&mut self) {
    self.pay_with_amazon_event_list = None;
  }

  pub fn set_service_provider_credit_event_list(&mut self, service_provider_credit_event_list: ::models::SolutionProviderCreditEventList) {
    self.service_provider_credit_event_list = Some(service_provider_credit_event_list);
  }

  pub fn with_service_provider_credit_event_list(mut self, service_provider_credit_event_list: ::models::SolutionProviderCreditEventList) -> FinancialEvents {
    self.service_provider_credit_event_list = Some(service_provider_credit_event_list);
    self
  }

  pub fn service_provider_credit_event_list(&self) -> Option<&::models::SolutionProviderCreditEventList> {
    self.service_provider_credit_event_list.as_ref()
  }

  pub fn reset_service_provider_credit_event_list(&mut self) {
    self.service_provider_credit_event_list = None;
  }

  pub fn set_retrocharge_event_list(&mut self, retrocharge_event_list: ::models::RetrochargeEventList) {
    self.retrocharge_event_list = Some(retrocharge_event_list);
  }

  pub fn with_retrocharge_event_list(mut self, retrocharge_event_list: ::models::RetrochargeEventList) -> FinancialEvents {
    self.retrocharge_event_list = Some(retrocharge_event_list);
    self
  }

  pub fn retrocharge_event_list(&self) -> Option<&::models::RetrochargeEventList> {
    self.retrocharge_event_list.as_ref()
  }

  pub fn reset_retrocharge_event_list(&mut self) {
    self.retrocharge_event_list = None;
  }

  pub fn set_rental_transaction_event_list(&mut self, rental_transaction_event_list: ::models::RentalTransactionEventList) {
    self.rental_transaction_event_list = Some(rental_transaction_event_list);
  }

  pub fn with_rental_transaction_event_list(mut self, rental_transaction_event_list: ::models::RentalTransactionEventList) -> FinancialEvents {
    self.rental_transaction_event_list = Some(rental_transaction_event_list);
    self
  }

  pub fn rental_transaction_event_list(&self) -> Option<&::models::RentalTransactionEventList> {
    self.rental_transaction_event_list.as_ref()
  }

  pub fn reset_rental_transaction_event_list(&mut self) {
    self.rental_transaction_event_list = None;
  }

  pub fn set_product_ads_payment_event_list(&mut self, product_ads_payment_event_list: ::models::ProductAdsPaymentEventList) {
    self.product_ads_payment_event_list = Some(product_ads_payment_event_list);
  }

  pub fn with_product_ads_payment_event_list(mut self, product_ads_payment_event_list: ::models::ProductAdsPaymentEventList) -> FinancialEvents {
    self.product_ads_payment_event_list = Some(product_ads_payment_event_list);
    self
  }

  pub fn product_ads_payment_event_list(&self) -> Option<&::models::ProductAdsPaymentEventList> {
    self.product_ads_payment_event_list.as_ref()
  }

  pub fn reset_product_ads_payment_event_list(&mut self) {
    self.product_ads_payment_event_list = None;
  }

  pub fn set_service_fee_event_list(&mut self, service_fee_event_list: ::models::ServiceFeeEventList) {
    self.service_fee_event_list = Some(service_fee_event_list);
  }

  pub fn with_service_fee_event_list(mut self, service_fee_event_list: ::models::ServiceFeeEventList) -> FinancialEvents {
    self.service_fee_event_list = Some(service_fee_event_list);
    self
  }

  pub fn service_fee_event_list(&self) -> Option<&::models::ServiceFeeEventList> {
    self.service_fee_event_list.as_ref()
  }

  pub fn reset_service_fee_event_list(&mut self) {
    self.service_fee_event_list = None;
  }

  pub fn set_seller_deal_payment_event_list(&mut self, seller_deal_payment_event_list: ::models::SellerDealPaymentEventList) {
    self.seller_deal_payment_event_list = Some(seller_deal_payment_event_list);
  }

  pub fn with_seller_deal_payment_event_list(mut self, seller_deal_payment_event_list: ::models::SellerDealPaymentEventList) -> FinancialEvents {
    self.seller_deal_payment_event_list = Some(seller_deal_payment_event_list);
    self
  }

  pub fn seller_deal_payment_event_list(&self) -> Option<&::models::SellerDealPaymentEventList> {
    self.seller_deal_payment_event_list.as_ref()
  }

  pub fn reset_seller_deal_payment_event_list(&mut self) {
    self.seller_deal_payment_event_list = None;
  }

  pub fn set_debt_recovery_event_list(&mut self, debt_recovery_event_list: ::models::DebtRecoveryEventList) {
    self.debt_recovery_event_list = Some(debt_recovery_event_list);
  }

  pub fn with_debt_recovery_event_list(mut self, debt_recovery_event_list: ::models::DebtRecoveryEventList) -> FinancialEvents {
    self.debt_recovery_event_list = Some(debt_recovery_event_list);
    self
  }

  pub fn debt_recovery_event_list(&self) -> Option<&::models::DebtRecoveryEventList> {
    self.debt_recovery_event_list.as_ref()
  }

  pub fn reset_debt_recovery_event_list(&mut self) {
    self.debt_recovery_event_list = None;
  }

  pub fn set_loan_servicing_event_list(&mut self, loan_servicing_event_list: ::models::LoanServicingEventList) {
    self.loan_servicing_event_list = Some(loan_servicing_event_list);
  }

  pub fn with_loan_servicing_event_list(mut self, loan_servicing_event_list: ::models::LoanServicingEventList) -> FinancialEvents {
    self.loan_servicing_event_list = Some(loan_servicing_event_list);
    self
  }

  pub fn loan_servicing_event_list(&self) -> Option<&::models::LoanServicingEventList> {
    self.loan_servicing_event_list.as_ref()
  }

  pub fn reset_loan_servicing_event_list(&mut self) {
    self.loan_servicing_event_list = None;
  }

  pub fn set_adjustment_event_list(&mut self, adjustment_event_list: ::models::AdjustmentEventList) {
    self.adjustment_event_list = Some(adjustment_event_list);
  }

  pub fn with_adjustment_event_list(mut self, adjustment_event_list: ::models::AdjustmentEventList) -> FinancialEvents {
    self.adjustment_event_list = Some(adjustment_event_list);
    self
  }

  pub fn adjustment_event_list(&self) -> Option<&::models::AdjustmentEventList> {
    self.adjustment_event_list.as_ref()
  }

  pub fn reset_adjustment_event_list(&mut self) {
    self.adjustment_event_list = None;
  }

  pub fn set_safet_reimbursement_event_list(&mut self, safet_reimbursement_event_list: ::models::SafetReimbursementEventList) {
    self.safet_reimbursement_event_list = Some(safet_reimbursement_event_list);
  }

  pub fn with_safet_reimbursement_event_list(mut self, safet_reimbursement_event_list: ::models::SafetReimbursementEventList) -> FinancialEvents {
    self.safet_reimbursement_event_list = Some(safet_reimbursement_event_list);
    self
  }

  pub fn safet_reimbursement_event_list(&self) -> Option<&::models::SafetReimbursementEventList> {
    self.safet_reimbursement_event_list.as_ref()
  }

  pub fn reset_safet_reimbursement_event_list(&mut self) {
    self.safet_reimbursement_event_list = None;
  }

  pub fn set_seller_review_enrollment_payment_event_list(&mut self, seller_review_enrollment_payment_event_list: ::models::SellerReviewEnrollmentPaymentEventList) {
    self.seller_review_enrollment_payment_event_list = Some(seller_review_enrollment_payment_event_list);
  }

  pub fn with_seller_review_enrollment_payment_event_list(mut self, seller_review_enrollment_payment_event_list: ::models::SellerReviewEnrollmentPaymentEventList) -> FinancialEvents {
    self.seller_review_enrollment_payment_event_list = Some(seller_review_enrollment_payment_event_list);
    self
  }

  pub fn seller_review_enrollment_payment_event_list(&self) -> Option<&::models::SellerReviewEnrollmentPaymentEventList> {
    self.seller_review_enrollment_payment_event_list.as_ref()
  }

  pub fn reset_seller_review_enrollment_payment_event_list(&mut self) {
    self.seller_review_enrollment_payment_event_list = None;
  }

  pub fn set_fba_liquidation_event_list(&mut self, fba_liquidation_event_list: ::models::FbaLiquidationEventList) {
    self.fba_liquidation_event_list = Some(fba_liquidation_event_list);
  }

  pub fn with_fba_liquidation_event_list(mut self, fba_liquidation_event_list: ::models::FbaLiquidationEventList) -> FinancialEvents {
    self.fba_liquidation_event_list = Some(fba_liquidation_event_list);
    self
  }

  pub fn fba_liquidation_event_list(&self) -> Option<&::models::FbaLiquidationEventList> {
    self.fba_liquidation_event_list.as_ref()
  }

  pub fn reset_fba_liquidation_event_list(&mut self) {
    self.fba_liquidation_event_list = None;
  }

  pub fn set_coupon_payment_event_list(&mut self, coupon_payment_event_list: ::models::CouponPaymentEventList) {
    self.coupon_payment_event_list = Some(coupon_payment_event_list);
  }

  pub fn with_coupon_payment_event_list(mut self, coupon_payment_event_list: ::models::CouponPaymentEventList) -> FinancialEvents {
    self.coupon_payment_event_list = Some(coupon_payment_event_list);
    self
  }

  pub fn coupon_payment_event_list(&self) -> Option<&::models::CouponPaymentEventList> {
    self.coupon_payment_event_list.as_ref()
  }

  pub fn reset_coupon_payment_event_list(&mut self) {
    self.coupon_payment_event_list = None;
  }

  pub fn set_imaging_services_fee_event_list(&mut self, imaging_services_fee_event_list: ::models::ImagingServicesFeeEventList) {
    self.imaging_services_fee_event_list = Some(imaging_services_fee_event_list);
  }

  pub fn with_imaging_services_fee_event_list(mut self, imaging_services_fee_event_list: ::models::ImagingServicesFeeEventList) -> FinancialEvents {
    self.imaging_services_fee_event_list = Some(imaging_services_fee_event_list);
    self
  }

  pub fn imaging_services_fee_event_list(&self) -> Option<&::models::ImagingServicesFeeEventList> {
    self.imaging_services_fee_event_list.as_ref()
  }

  pub fn reset_imaging_services_fee_event_list(&mut self) {
    self.imaging_services_fee_event_list = None;
  }

  pub fn set_network_commingling_transaction_event_list(&mut self, network_commingling_transaction_event_list: ::models::NetworkComminglingTransactionEventList) {
    self.network_commingling_transaction_event_list = Some(network_commingling_transaction_event_list);
  }

  pub fn with_network_commingling_transaction_event_list(mut self, network_commingling_transaction_event_list: ::models::NetworkComminglingTransactionEventList) -> FinancialEvents {
    self.network_commingling_transaction_event_list = Some(network_commingling_transaction_event_list);
    self
  }

  pub fn network_commingling_transaction_event_list(&self) -> Option<&::models::NetworkComminglingTransactionEventList> {
    self.network_commingling_transaction_event_list.as_ref()
  }

  pub fn reset_network_commingling_transaction_event_list(&mut self) {
    self.network_commingling_transaction_event_list = None;
  }

  pub fn set_affordability_expense_event_list(&mut self, affordability_expense_event_list: ::models::AffordabilityExpenseEventList) {
    self.affordability_expense_event_list = Some(affordability_expense_event_list);
  }

  pub fn with_affordability_expense_event_list(mut self, affordability_expense_event_list: ::models::AffordabilityExpenseEventList) -> FinancialEvents {
    self.affordability_expense_event_list = Some(affordability_expense_event_list);
    self
  }

  pub fn affordability_expense_event_list(&self) -> Option<&::models::AffordabilityExpenseEventList> {
    self.affordability_expense_event_list.as_ref()
  }

  pub fn reset_affordability_expense_event_list(&mut self) {
    self.affordability_expense_event_list = None;
  }

  pub fn set_affordability_expense_reversal_event_list(&mut self, affordability_expense_reversal_event_list: ::models::AffordabilityExpenseEventList) {
    self.affordability_expense_reversal_event_list = Some(affordability_expense_reversal_event_list);
  }

  pub fn with_affordability_expense_reversal_event_list(mut self, affordability_expense_reversal_event_list: ::models::AffordabilityExpenseEventList) -> FinancialEvents {
    self.affordability_expense_reversal_event_list = Some(affordability_expense_reversal_event_list);
    self
  }

  pub fn affordability_expense_reversal_event_list(&self) -> Option<&::models::AffordabilityExpenseEventList> {
    self.affordability_expense_reversal_event_list.as_ref()
  }

  pub fn reset_affordability_expense_reversal_event_list(&mut self) {
    self.affordability_expense_reversal_event_list = None;
  }

}



