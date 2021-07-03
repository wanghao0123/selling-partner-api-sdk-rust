mod create_feed_document_response;
pub use self::create_feed_document_response::CreateFeedDocumentResponse;
mod create_feed_document_specification;
pub use self::create_feed_document_specification::CreateFeedDocumentSpecification;
mod create_feed_response;
pub use self::create_feed_response::CreateFeedResponse;
mod create_feed_specification;
pub use self::create_feed_specification::CreateFeedSpecification;
mod error;
pub use self::error::Error;
mod error_list;
pub use self::error_list::ErrorList;
mod feed;
pub use self::feed::Feed;
mod feed_document;
pub use self::feed_document::FeedDocument;
mod feed_list;
pub use self::feed_list::FeedList;
mod feed_options;
pub use self::feed_options::FeedOptions;
mod get_feeds_response;
pub use self::get_feeds_response::GetFeedsResponse;

// TODO(farcaller): sort out files
pub struct File;
