mod address;
pub use self::address::Address;
mod container;
pub use self::container::Container;
mod customer_invoice;
pub use self::customer_invoice::CustomerInvoice;
mod customer_invoice_list;
pub use self::customer_invoice_list::CustomerInvoiceList;
mod decimal;
pub use self::decimal::Decimal;
mod dimensions;
pub use self::dimensions::Dimensions;
mod error;
pub use self::error::Error;
mod error_list;
pub use self::error_list::ErrorList;
mod get_customer_invoice_response;
pub use self::get_customer_invoice_response::GetCustomerInvoiceResponse;
mod get_customer_invoices_response;
pub use self::get_customer_invoices_response::GetCustomerInvoicesResponse;
mod get_packing_slip_list_response;
pub use self::get_packing_slip_list_response::GetPackingSlipListResponse;
mod get_packing_slip_response;
pub use self::get_packing_slip_response::GetPackingSlipResponse;
mod get_shipping_label_list_response;
pub use self::get_shipping_label_list_response::GetShippingLabelListResponse;
mod get_shipping_label_response;
pub use self::get_shipping_label_response::GetShippingLabelResponse;
mod item;
pub use self::item::Item;
mod item_quantity;
pub use self::item_quantity::ItemQuantity;
mod label_data;
pub use self::label_data::LabelData;
mod package;
pub use self::package::Package;
mod packed_item;
pub use self::packed_item::PackedItem;
mod packing_slip;
pub use self::packing_slip::PackingSlip;
mod packing_slip_list;
pub use self::packing_slip_list::PackingSlipList;
mod pagination;
pub use self::pagination::Pagination;
mod party_identification;
pub use self::party_identification::PartyIdentification;
mod shipment_confirmation;
pub use self::shipment_confirmation::ShipmentConfirmation;
mod shipment_details;
pub use self::shipment_details::ShipmentDetails;
mod shipment_status_update;
pub use self::shipment_status_update::ShipmentStatusUpdate;
mod shipping_label;
pub use self::shipping_label::ShippingLabel;
mod shipping_label_list;
pub use self::shipping_label_list::ShippingLabelList;
mod shipping_label_request;
pub use self::shipping_label_request::ShippingLabelRequest;
mod status_update_details;
pub use self::status_update_details::StatusUpdateDetails;
mod submit_shipment_confirmations_request;
pub use self::submit_shipment_confirmations_request::SubmitShipmentConfirmationsRequest;
mod submit_shipment_confirmations_response;
pub use self::submit_shipment_confirmations_response::SubmitShipmentConfirmationsResponse;
mod submit_shipment_status_updates_request;
pub use self::submit_shipment_status_updates_request::SubmitShipmentStatusUpdatesRequest;
mod submit_shipment_status_updates_response;
pub use self::submit_shipment_status_updates_response::SubmitShipmentStatusUpdatesResponse;
mod submit_shipping_labels_request;
pub use self::submit_shipping_labels_request::SubmitShippingLabelsRequest;
mod submit_shipping_labels_response;
pub use self::submit_shipping_labels_response::SubmitShippingLabelsResponse;
mod tax_registration_details;
pub use self::tax_registration_details::TaxRegistrationDetails;
mod transaction_reference;
pub use self::transaction_reference::TransactionReference;
mod weight;
pub use self::weight::Weight;

// TODO(farcaller): sort out files
pub struct File;
