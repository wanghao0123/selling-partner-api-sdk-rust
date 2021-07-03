mod create_product_review_and_seller_feedback_solicitation_response;
pub use self::create_product_review_and_seller_feedback_solicitation_response::CreateProductReviewAndSellerFeedbackSolicitationResponse;
mod error;
pub use self::error::Error;
mod error_list;
pub use self::error_list::ErrorList;
mod get_schema_response;
pub use self::get_schema_response::GetSchemaResponse;
mod get_solicitation_action_response;
pub use self::get_solicitation_action_response::GetSolicitationActionResponse;
mod get_solicitation_actions_for_order_response;
pub use self::get_solicitation_actions_for_order_response::GetSolicitationActionsForOrderResponse;
mod link_object;
pub use self::link_object::LinkObject;
mod schema;
pub use self::schema::Schema;
mod solicitations_action;
pub use self::solicitations_action::SolicitationsAction;

// TODO(farcaller): sort out files
pub struct File;
