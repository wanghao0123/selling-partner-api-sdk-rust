mod create_upload_destination_response;
pub use self::create_upload_destination_response::CreateUploadDestinationResponse;
mod error;
pub use self::error::Error;
mod error_list;
pub use self::error_list::ErrorList;
mod upload_destination;
pub use self::upload_destination::UploadDestination;

// TODO(farcaller): sort out files
pub struct File;
