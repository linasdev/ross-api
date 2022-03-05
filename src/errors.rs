use rocket::response::Responder;

use ross_configurator::ross_configurator::ConfiguratorError;
use ross_protocol::protocol::ProtocolError;

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(status = 500)]
    CommunicationError(()),
}

impl From<ConfiguratorError> for ApiError {
    fn from(_error: ConfiguratorError) -> ApiError {
        ApiError::CommunicationError(())
    }
}

impl From<ProtocolError> for ApiError {
    fn from(_error: ProtocolError) -> ApiError {
        ApiError::CommunicationError(())
    }
}
