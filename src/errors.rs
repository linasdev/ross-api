use rocket::response::Responder;

use ross_configurator::ross_configurator::ConfiguratorError;

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
