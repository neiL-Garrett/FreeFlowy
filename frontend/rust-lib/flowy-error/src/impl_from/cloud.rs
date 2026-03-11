use crate::{ErrorCode, FlowyError};
use client_api::error::{AppResponseError, ErrorCode as AppErrorCode};

impl From<AppResponseError> for FlowyError {
  fn from(error: AppResponseError) -> Self {
    let code = match error.code {
      AppErrorCode::Ok => ErrorCode::Internal,
      AppErrorCode::Unhandled => ErrorCode::Internal,
      AppErrorCode::RecordNotFound => ErrorCode::RecordNotFound,
      AppErrorCode::RecordAlreadyExists => ErrorCode::RecordAlreadyExists,
      AppErrorCode::InvalidEmail => ErrorCode::EmailFormatInvalid,
      AppErrorCode::InvalidPassword => ErrorCode::PasswordFormatInvalid,
      AppErrorCode::OAuthError => ErrorCode::UserUnauthorized,
      AppErrorCode::MissingPayload => ErrorCode::MissingPayload,
      AppErrorCode::OpenError => ErrorCode::Internal,
      AppErrorCode::InvalidUrl => ErrorCode::InvalidURL,
      AppErrorCode::InvalidRequest => ErrorCode::InvalidRequest,
      AppErrorCode::InvalidOAuthProvider => ErrorCode::InvalidAuthConfig,
      AppErrorCode::NotLoggedIn => ErrorCode::UserUnauthorized,
      AppErrorCode::NotEnoughPermissions => ErrorCode::NotEnoughPermissions,
      AppErrorCode::NetworkError => ErrorCode::NetworkError,
      AppErrorCode::RequestTimeout => ErrorCode::RequestTimeout,
      AppErrorCode::PayloadTooLarge => ErrorCode::PayloadTooLarge,
      AppErrorCode::UserUnAuthorized => ErrorCode::UserUnauthorized,
      AppErrorCode::WorkspaceLimitExceeded
      | AppErrorCode::WorkspaceMemberLimitExceeded
      | AppErrorCode::AIResponseLimitExceeded
      | AppErrorCode::AIImageResponseLimitExceeded
      | AppErrorCode::AIMaxRequired
      | AppErrorCode::FileStorageLimitExceeded
      | AppErrorCode::SingleUploadLimitExceeded
      | AppErrorCode::CustomNamespaceDisabled
      | AppErrorCode::FreePlanGuestLimitExceeded
      | AppErrorCode::PaidPlanGuestLimitExceeded => ErrorCode::NotEnoughPermissions,
      AppErrorCode::CustomNamespaceDisallowed => ErrorCode::CustomNamespaceNotAllowed,
      AppErrorCode::PublishNamespaceAlreadyTaken => ErrorCode::CustomNamespaceAlreadyTaken,
      AppErrorCode::CustomNamespaceTooShort => ErrorCode::CustomNamespaceTooShort,
      AppErrorCode::CustomNamespaceTooLong => ErrorCode::CustomNamespaceTooLong,
      AppErrorCode::CustomNamespaceReserved => ErrorCode::CustomNamespaceReserved,
      AppErrorCode::PublishNameAlreadyExists => ErrorCode::PublishNameAlreadyExists,
      AppErrorCode::PublishNameInvalidCharacter => ErrorCode::PublishNameInvalidCharacter,
      AppErrorCode::PublishNameTooLong => ErrorCode::PublishNameTooLong,
      AppErrorCode::CustomNamespaceInvalidCharacter => ErrorCode::CustomNamespaceInvalidCharacter,
      AppErrorCode::AIServiceUnavailable => ErrorCode::AIServiceUnavailable,
      AppErrorCode::InvalidGuest => ErrorCode::InvalidGuest,
      _ => ErrorCode::Internal,
    };

    let message = match error.code {
      AppErrorCode::WorkspaceLimitExceeded
      | AppErrorCode::WorkspaceMemberLimitExceeded
      | AppErrorCode::AIResponseLimitExceeded
      | AppErrorCode::AIImageResponseLimitExceeded
      | AppErrorCode::AIMaxRequired
      | AppErrorCode::FileStorageLimitExceeded
      | AppErrorCode::SingleUploadLimitExceeded
      | AppErrorCode::CustomNamespaceDisabled
      | AppErrorCode::FreePlanGuestLimitExceeded
      | AppErrorCode::PaidPlanGuestLimitExceeded => {
        "This action is unavailable on the current server.".to_string()
      },
      _ => error.message.to_string(),
    };

    FlowyError::new(code, message)
  }
}
