use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct DataResponse<T: Clone + Default> {
    data: T,
}

impl<T: Clone + Default> DataResponse<T> {
    pub fn builder() -> DataResponseBuilder<T> {
        DataResponseBuilder::<T>::default()
    }
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct ErrorResponse {
    error: String,
    error_description: Option<String>,
    state: Option<String>,
}

macro_rules! static_resp {
    ($name:ident, $error:expr) => {
        paste::item! {
            #[allow(non_snake_case, missing_docs)]
            pub fn $name() -> ErrorResponseBuilder {
                let mut builder = ErrorResponseBuilder::default();

                builder.with_error($error);

                builder
            }

            #[allow(non_snake_case, missing_docs)]
            pub fn [< Empty $name >] () -> ErrorResponse {
                ErrorResponseBuilder::default()
                    .with_error($error)
                    .build()
                    .unwrap()
            }
        }
    };
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        ErrorResponseBuilder::default()
    }

    static_resp!(Conflict, "conflict");
    static_resp!(ContentTooLarge, "content_too_large");
    static_resp!(InvalidGrant, "invalid_grant");
    static_resp!(InvalidRequest, "invalid_request");
    static_resp!(InvalidSignature, "invalid_signature");
    static_resp!(InvalidToken, "invalid_token");
    static_resp!(NotFound, "not_found");
    static_resp!(ServerError, "server_error");
    static_resp!(Impossible, "impossible");
}
