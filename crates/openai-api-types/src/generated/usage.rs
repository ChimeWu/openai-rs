pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "The aggregated costs details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CostsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<CostsResultAmount>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "When grouped by `api_source`, `agents_api` identifies attributed Agents API activity and `unlabeled` includes all records without published source attribution, including historical and unknown origins. Unlabeled does not imply direct API usage. Without source grouping, this field is null."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_source: ::std::option::Option<CostsResultApiSource>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub line_item: ::std::option::Option<::std::string::String>,
    pub object: CostsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quantity: ::std::option::Option<f64>,
    #[doc = "The unit of the `quantity` value. If no single supported unit applies to the result, this field is `null`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quantity_unit: ::std::option::Option<CostsResultQuantityUnit>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl CostsResult {
    pub fn builder() -> builder::CostsResult {
        Default::default()
    }
}
#[doc = "The monetary value in its associated currency."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CostsResultAmount {
    #[doc = "Lowercase ISO-4217 currency e.g. \"usd\""]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    #[doc = "The numeric value of the cost."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl CostsResultAmount {
    pub fn builder() -> builder::CostsResultAmount {
        Default::default()
    }
}
#[doc = "`CostsResultApiSource`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CostsResultApiSource {
    #[serde(rename = "agents_api")]
    AgentsApi,
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ::std::fmt::Display for CostsResultApiSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentsApi => f.write_str("agents_api"),
            Self::Unlabeled => f.write_str("unlabeled"),
        }
    }
}
impl ::std::str::FromStr for CostsResultApiSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agents_api" => Ok(Self::AgentsApi),
            "unlabeled" => Ok(Self::Unlabeled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CostsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CostsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CostsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CostsResultObject {
    #[serde(rename = "organization.costs.result")]
    OrganizationCostsResult,
}
impl ::std::fmt::Display for CostsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationCostsResult => f.write_str("organization.costs.result"),
        }
    }
}
impl ::std::str::FromStr for CostsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.costs.result" => Ok(Self::OrganizationCostsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CostsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CostsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The unit of the `quantity` value. If no single supported unit applies to the result, this field is `null`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CostsResultQuantityUnit {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<CostsResultQuantityUnitSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
impl CostsResultQuantityUnit {
    pub fn builder() -> builder::CostsResultQuantityUnit {
        Default::default()
    }
}
#[doc = "`CostsResultQuantityUnitSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CostsResultQuantityUnitSubtype1 {
    #[serde(rename = "tokens")]
    Tokens,
    #[serde(rename = "1000_tokens")]
    X1000Tokens,
    #[serde(rename = "duration_seconds")]
    DurationSeconds,
    #[serde(rename = "duration_minutes")]
    DurationMinutes,
    #[serde(rename = "duration_hours")]
    DurationHours,
    #[serde(rename = "gibibyte_hours")]
    GibibyteHours,
    #[serde(rename = "images")]
    Images,
    #[serde(rename = "characters")]
    Characters,
}
impl ::std::fmt::Display for CostsResultQuantityUnitSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tokens => f.write_str("tokens"),
            Self::X1000Tokens => f.write_str("1000_tokens"),
            Self::DurationSeconds => f.write_str("duration_seconds"),
            Self::DurationMinutes => f.write_str("duration_minutes"),
            Self::DurationHours => f.write_str("duration_hours"),
            Self::GibibyteHours => f.write_str("gibibyte_hours"),
            Self::Images => f.write_str("images"),
            Self::Characters => f.write_str("characters"),
        }
    }
}
impl ::std::str::FromStr for CostsResultQuantityUnitSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tokens" => Ok(Self::Tokens),
            "1000_tokens" => Ok(Self::X1000Tokens),
            "duration_seconds" => Ok(Self::DurationSeconds),
            "duration_minutes" => Ok(Self::DurationMinutes),
            "duration_hours" => Ok(Self::DurationHours),
            "gibibyte_hours" => Ok(Self::GibibyteHours),
            "images" => Ok(Self::Images),
            "characters" => Ok(Self::Characters),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CostsResultQuantityUnitSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CostsResultQuantityUnitSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageApiSchema {
    #[serde(rename = "UsageAudioSpeechesError400")]
    pub usage_audio_speeches_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageAudioSpeechesError403")]
    pub usage_audio_speeches_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageAudioSpeechesError429")]
    pub usage_audio_speeches_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageAudioSpeechesParams")]
    pub usage_audio_speeches_params: UsageAudioSpeechesParams,
    #[serde(rename = "UsageAudioSpeechesResponse")]
    pub usage_audio_speeches_response: UsageResponse,
    #[serde(rename = "UsageAudioTranscriptionsError400")]
    pub usage_audio_transcriptions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageAudioTranscriptionsError403")]
    pub usage_audio_transcriptions_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageAudioTranscriptionsError429")]
    pub usage_audio_transcriptions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageAudioTranscriptionsParams")]
    pub usage_audio_transcriptions_params: UsageAudioTranscriptionsParams,
    #[serde(rename = "UsageAudioTranscriptionsResponse")]
    pub usage_audio_transcriptions_response: UsageResponse,
    #[serde(rename = "UsageCodeInterpreterSessionsError400")]
    pub usage_code_interpreter_sessions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCodeInterpreterSessionsError403")]
    pub usage_code_interpreter_sessions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageCodeInterpreterSessionsError429")]
    pub usage_code_interpreter_sessions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCodeInterpreterSessionsParams")]
    pub usage_code_interpreter_sessions_params: UsageCodeInterpreterSessionsParams,
    #[serde(rename = "UsageCodeInterpreterSessionsResponse")]
    pub usage_code_interpreter_sessions_response: UsageResponse,
    #[serde(rename = "UsageCompletionsError400")]
    pub usage_completions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCompletionsError403")]
    pub usage_completions_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageCompletionsError429")]
    pub usage_completions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCompletionsParams")]
    pub usage_completions_params: UsageCompletionsParams,
    #[serde(rename = "UsageCompletionsResponse")]
    pub usage_completions_response: UsageResponse,
    #[serde(rename = "UsageCostsError400")]
    pub usage_costs_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCostsError403")]
    pub usage_costs_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageCostsError429")]
    pub usage_costs_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageCostsParams")]
    pub usage_costs_params: UsageCostsParams,
    #[serde(rename = "UsageCostsResponse")]
    pub usage_costs_response: UsageResponse,
    #[serde(rename = "UsageEmbeddingsError400")]
    pub usage_embeddings_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageEmbeddingsError403")]
    pub usage_embeddings_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageEmbeddingsError429")]
    pub usage_embeddings_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageEmbeddingsParams")]
    pub usage_embeddings_params: UsageEmbeddingsParams,
    #[serde(rename = "UsageEmbeddingsResponse")]
    pub usage_embeddings_response: UsageResponse,
    #[serde(rename = "UsageFileSearchCallsError400")]
    pub usage_file_search_calls_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageFileSearchCallsError403")]
    pub usage_file_search_calls_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageFileSearchCallsError429")]
    pub usage_file_search_calls_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageFileSearchCallsParams")]
    pub usage_file_search_calls_params: UsageFileSearchCallsParams,
    #[serde(rename = "UsageFileSearchCallsResponse")]
    pub usage_file_search_calls_response: UsageResponse,
    #[serde(rename = "UsageImagesError400")]
    pub usage_images_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageImagesError403")]
    pub usage_images_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageImagesError429")]
    pub usage_images_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageImagesParams")]
    pub usage_images_params: UsageImagesParams,
    #[serde(rename = "UsageImagesResponse")]
    pub usage_images_response: UsageResponse,
    #[serde(rename = "UsageModerationsError400")]
    pub usage_moderations_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageModerationsError403")]
    pub usage_moderations_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageModerationsError429")]
    pub usage_moderations_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageModerationsParams")]
    pub usage_moderations_params: UsageModerationsParams,
    #[serde(rename = "UsageModerationsResponse")]
    pub usage_moderations_response: UsageResponse,
    #[serde(rename = "UsageVectorStoresError400")]
    pub usage_vector_stores_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageVectorStoresError403")]
    pub usage_vector_stores_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageVectorStoresError429")]
    pub usage_vector_stores_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageVectorStoresParams")]
    pub usage_vector_stores_params: UsageVectorStoresParams,
    #[serde(rename = "UsageVectorStoresResponse")]
    pub usage_vector_stores_response: UsageResponse,
    #[serde(rename = "UsageWebSearchCallsError400")]
    pub usage_web_search_calls_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageWebSearchCallsError403")]
    pub usage_web_search_calls_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UsageWebSearchCallsError429")]
    pub usage_web_search_calls_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UsageWebSearchCallsParams")]
    pub usage_web_search_calls_params: UsageWebSearchCallsParams,
    #[serde(rename = "UsageWebSearchCallsResponse")]
    pub usage_web_search_calls_response: UsageResponse,
}
impl UsageApiSchema {
    pub fn builder() -> builder::UsageApiSchema {
        Default::default()
    }
}
#[doc = "`UsageAudioSpeechesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageAudioSpeechesParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageAudioSpeechesParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageAudioSpeechesParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageAudioSpeechesParams {
    pub fn builder() -> builder::UsageAudioSpeechesParams {
        Default::default()
    }
}
#[doc = "`UsageAudioSpeechesParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioSpeechesParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageAudioSpeechesParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageAudioSpeechesParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioSpeechesParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioSpeechesParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageAudioSpeechesParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioSpeechesParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
}
impl ::std::fmt::Display for UsageAudioSpeechesParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
        }
    }
}
impl ::std::str::FromStr for UsageAudioSpeechesParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioSpeechesParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioSpeechesParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated audio speeches usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageAudioSpeechesResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The number of characters processed."]
    pub characters: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageAudioSpeechesResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageAudioSpeechesResult {
    pub fn builder() -> builder::UsageAudioSpeechesResult {
        Default::default()
    }
}
#[doc = "`UsageAudioSpeechesResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioSpeechesResultObject {
    #[serde(rename = "organization.usage.audio_speeches.result")]
    OrganizationUsageAudioSpeechesResult,
}
impl ::std::fmt::Display for UsageAudioSpeechesResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageAudioSpeechesResult => {
                f.write_str("organization.usage.audio_speeches.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageAudioSpeechesResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.audio_speeches.result" => {
                Ok(Self::OrganizationUsageAudioSpeechesResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioSpeechesResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioSpeechesResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageAudioTranscriptionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageAudioTranscriptionsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageAudioTranscriptionsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageAudioTranscriptionsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageAudioTranscriptionsParams {
    pub fn builder() -> builder::UsageAudioTranscriptionsParams {
        Default::default()
    }
}
#[doc = "`UsageAudioTranscriptionsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioTranscriptionsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageAudioTranscriptionsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageAudioTranscriptionsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioTranscriptionsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioTranscriptionsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageAudioTranscriptionsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioTranscriptionsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
}
impl ::std::fmt::Display for UsageAudioTranscriptionsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
        }
    }
}
impl ::std::str::FromStr for UsageAudioTranscriptionsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioTranscriptionsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioTranscriptionsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated audio transcriptions usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageAudioTranscriptionsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageAudioTranscriptionsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[doc = "The number of seconds processed."]
    pub seconds: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageAudioTranscriptionsResult {
    pub fn builder() -> builder::UsageAudioTranscriptionsResult {
        Default::default()
    }
}
#[doc = "`UsageAudioTranscriptionsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageAudioTranscriptionsResultObject {
    #[serde(rename = "organization.usage.audio_transcriptions.result")]
    OrganizationUsageAudioTranscriptionsResult,
}
impl ::std::fmt::Display for UsageAudioTranscriptionsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageAudioTranscriptionsResult => {
                f.write_str("organization.usage.audio_transcriptions.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageAudioTranscriptionsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.audio_transcriptions.result" => {
                Ok(Self::OrganizationUsageAudioTranscriptionsResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageAudioTranscriptionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageAudioTranscriptionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCodeInterpreterSessionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageCodeInterpreterSessionsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageCodeInterpreterSessionsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageCodeInterpreterSessionsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
}
impl UsageCodeInterpreterSessionsParams {
    pub fn builder() -> builder::UsageCodeInterpreterSessionsParams {
        Default::default()
    }
}
#[doc = "`UsageCodeInterpreterSessionsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCodeInterpreterSessionsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageCodeInterpreterSessionsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageCodeInterpreterSessionsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCodeInterpreterSessionsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for UsageCodeInterpreterSessionsParamsBucketWidth
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCodeInterpreterSessionsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCodeInterpreterSessionsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
}
impl ::std::fmt::Display for UsageCodeInterpreterSessionsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
        }
    }
}
impl ::std::str::FromStr for UsageCodeInterpreterSessionsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCodeInterpreterSessionsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for UsageCodeInterpreterSessionsParamsGroupByItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated code interpreter sessions usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageCodeInterpreterSessionsResult {
    #[doc = "The number of code interpreter sessions."]
    pub num_sessions: i64,
    pub object: UsageCodeInterpreterSessionsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
}
impl UsageCodeInterpreterSessionsResult {
    pub fn builder() -> builder::UsageCodeInterpreterSessionsResult {
        Default::default()
    }
}
#[doc = "`UsageCodeInterpreterSessionsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCodeInterpreterSessionsResultObject {
    #[serde(rename = "organization.usage.code_interpreter_sessions.result")]
    OrganizationUsageCodeInterpreterSessionsResult,
}
impl ::std::fmt::Display for UsageCodeInterpreterSessionsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageCodeInterpreterSessionsResult => {
                f.write_str("organization.usage.code_interpreter_sessions.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageCodeInterpreterSessionsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.code_interpreter_sessions.result" => {
                Ok(Self::OrganizationUsageCodeInterpreterSessionsResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCodeInterpreterSessionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCodeInterpreterSessionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCompletionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageCompletionsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageCompletionsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageCompletionsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageCompletionsParams {
    pub fn builder() -> builder::UsageCompletionsParams {
        Default::default()
    }
}
#[doc = "`UsageCompletionsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCompletionsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageCompletionsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageCompletionsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCompletionsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCompletionsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCompletionsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCompletionsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "service_tier")]
    ServiceTier,
    #[serde(rename = "api_source")]
    ApiSource,
}
impl ::std::fmt::Display for UsageCompletionsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
            Self::Batch => f.write_str("batch"),
            Self::ServiceTier => f.write_str("service_tier"),
            Self::ApiSource => f.write_str("api_source"),
        }
    }
}
impl ::std::str::FromStr for UsageCompletionsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            "batch" => Ok(Self::Batch),
            "service_tier" => Ok(Self::ServiceTier),
            "api_source" => Ok(Self::ApiSource),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCompletionsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCompletionsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated completions usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageCompletionsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "When grouped by `api_source`, `agents_api` identifies attributed Agents API activity and `unlabeled` includes all records without published source attribution, including historical and unknown origins. Unlabeled does not imply direct API usage. Without source grouping, this field is null."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_source: ::std::option::Option<UsageCompletionsResultApiSource>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch: ::std::option::Option<bool>,
    #[doc = "The aggregated number of uncached audio input tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of input tokens written to the cache with a 12-hour retention period."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cache_write_12h_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of input tokens written to the cache with a 30-minute retention period."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cache_write_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of cached audio input tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_audio_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of cached image input tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_image_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of cached text input tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_text_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of cached input tokens used across text, audio, and image inputs. For customers subscribed to Scale Tier, this includes Scale Tier tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of uncached image input tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_image_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of uncached text input tokens used, excluding cache-write tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_text_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of input tokens used, including cached and cache-write tokens. This includes text, audio, and image tokens. For customers subscribed to Scale Tier, this includes Scale Tier tokens."]
    pub input_tokens: i64,
    #[doc = "The aggregated number of uncached input tokens used across text, audio, and image inputs, excluding cache-write tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_uncached_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageCompletionsResultObject,
    #[doc = "The aggregated number of audio output tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_audio_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of image output tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_image_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of text output tokens used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_text_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of output tokens used across text, audio, and image outputs. For customers subscribed to Scale Tier, this includes Scale Tier tokens."]
    pub output_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageCompletionsResult {
    pub fn builder() -> builder::UsageCompletionsResult {
        Default::default()
    }
}
#[doc = "`UsageCompletionsResultApiSource`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCompletionsResultApiSource {
    #[serde(rename = "agents_api")]
    AgentsApi,
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ::std::fmt::Display for UsageCompletionsResultApiSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentsApi => f.write_str("agents_api"),
            Self::Unlabeled => f.write_str("unlabeled"),
        }
    }
}
impl ::std::str::FromStr for UsageCompletionsResultApiSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agents_api" => Ok(Self::AgentsApi),
            "unlabeled" => Ok(Self::Unlabeled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCompletionsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCompletionsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCompletionsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCompletionsResultObject {
    #[serde(rename = "organization.usage.completions.result")]
    OrganizationUsageCompletionsResult,
}
impl ::std::fmt::Display for UsageCompletionsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageCompletionsResult => {
                f.write_str("organization.usage.completions.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageCompletionsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.completions.result" => Ok(Self::OrganizationUsageCompletionsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCompletionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCompletionsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCostsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageCostsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageCostsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageCostsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub line_items: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
}
impl UsageCostsParams {
    pub fn builder() -> builder::UsageCostsParams {
        Default::default()
    }
}
#[doc = "`UsageCostsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCostsParamsBucketWidth {
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageCostsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageCostsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCostsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCostsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageCostsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageCostsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "line_item")]
    LineItem,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "api_source")]
    ApiSource,
}
impl ::std::fmt::Display for UsageCostsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::LineItem => f.write_str("line_item"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::ApiSource => f.write_str("api_source"),
        }
    }
}
impl ::std::str::FromStr for UsageCostsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "line_item" => Ok(Self::LineItem),
            "api_key_id" => Ok(Self::ApiKeyId),
            "api_source" => Ok(Self::ApiSource),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageCostsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageCostsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageEmbeddingsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageEmbeddingsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageEmbeddingsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageEmbeddingsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageEmbeddingsParams {
    pub fn builder() -> builder::UsageEmbeddingsParams {
        Default::default()
    }
}
#[doc = "`UsageEmbeddingsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageEmbeddingsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageEmbeddingsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageEmbeddingsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageEmbeddingsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageEmbeddingsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageEmbeddingsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageEmbeddingsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
}
impl ::std::fmt::Display for UsageEmbeddingsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
        }
    }
}
impl ::std::str::FromStr for UsageEmbeddingsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageEmbeddingsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageEmbeddingsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated embeddings usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageEmbeddingsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The aggregated number of input tokens used."]
    pub input_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageEmbeddingsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageEmbeddingsResult {
    pub fn builder() -> builder::UsageEmbeddingsResult {
        Default::default()
    }
}
#[doc = "`UsageEmbeddingsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageEmbeddingsResultObject {
    #[serde(rename = "organization.usage.embeddings.result")]
    OrganizationUsageEmbeddingsResult,
}
impl ::std::fmt::Display for UsageEmbeddingsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageEmbeddingsResult => {
                f.write_str("organization.usage.embeddings.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageEmbeddingsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.embeddings.result" => Ok(Self::OrganizationUsageEmbeddingsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageEmbeddingsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageEmbeddingsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageFileSearchCallsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageFileSearchCallsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageFileSearchCallsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageFileSearchCallsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageFileSearchCallsParams {
    pub fn builder() -> builder::UsageFileSearchCallsParams {
        Default::default()
    }
}
#[doc = "`UsageFileSearchCallsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageFileSearchCallsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageFileSearchCallsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageFileSearchCallsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageFileSearchCallsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageFileSearchCallsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageFileSearchCallsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageFileSearchCallsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "vector_store_id")]
    VectorStoreId,
}
impl ::std::fmt::Display for UsageFileSearchCallsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::VectorStoreId => f.write_str("vector_store_id"),
        }
    }
}
impl ::std::str::FromStr for UsageFileSearchCallsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "vector_store_id" => Ok(Self::VectorStoreId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageFileSearchCallsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageFileSearchCallsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated file search calls usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageFileSearchCallsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The count of file search calls."]
    pub num_requests: i64,
    pub object: UsageFileSearchCallsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vector_store_id: ::std::option::Option<::std::string::String>,
}
impl UsageFileSearchCallsResult {
    pub fn builder() -> builder::UsageFileSearchCallsResult {
        Default::default()
    }
}
#[doc = "`UsageFileSearchCallsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageFileSearchCallsResultObject {
    #[serde(rename = "organization.usage.file_searches.result")]
    OrganizationUsageFileSearchesResult,
}
impl ::std::fmt::Display for UsageFileSearchCallsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageFileSearchesResult => {
                f.write_str("organization.usage.file_searches.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageFileSearchCallsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.file_searches.result" => {
                Ok(Self::OrganizationUsageFileSearchesResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageFileSearchCallsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageFileSearchCallsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageImagesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageImagesParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageImagesParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageImagesParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sizes: ::std::vec::Vec<UsageImagesParamsSizesItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sources: ::std::vec::Vec<UsageImagesParamsSourcesItem>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageImagesParams {
    pub fn builder() -> builder::UsageImagesParams {
        Default::default()
    }
}
#[doc = "`UsageImagesParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageImagesParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageImagesParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageImagesParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageImagesParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageImagesParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageImagesParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageImagesParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "source")]
    Source,
}
impl ::std::fmt::Display for UsageImagesParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
            Self::Size => f.write_str("size"),
            Self::Source => f.write_str("source"),
        }
    }
}
impl ::std::str::FromStr for UsageImagesParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            "size" => Ok(Self::Size),
            "source" => Ok(Self::Source),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageImagesParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageImagesParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageImagesParamsSizesItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageImagesParamsSizesItem {
    #[serde(rename = "256x256")]
    X256x256,
    #[serde(rename = "512x512")]
    X512x512,
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1792x1792")]
    X1792x1792,
    #[serde(rename = "1024x1792")]
    X1024x1792,
}
impl ::std::fmt::Display for UsageImagesParamsSizesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X256x256 => f.write_str("256x256"),
            Self::X512x512 => f.write_str("512x512"),
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1792x1792 => f.write_str("1792x1792"),
            Self::X1024x1792 => f.write_str("1024x1792"),
        }
    }
}
impl ::std::str::FromStr for UsageImagesParamsSizesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "256x256" => Ok(Self::X256x256),
            "512x512" => Ok(Self::X512x512),
            "1024x1024" => Ok(Self::X1024x1024),
            "1792x1792" => Ok(Self::X1792x1792),
            "1024x1792" => Ok(Self::X1024x1792),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageImagesParamsSizesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageImagesParamsSizesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageImagesParamsSourcesItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageImagesParamsSourcesItem {
    #[serde(rename = "image.generation")]
    ImageGeneration,
    #[serde(rename = "image.edit")]
    ImageEdit,
    #[serde(rename = "image.variation")]
    ImageVariation,
}
impl ::std::fmt::Display for UsageImagesParamsSourcesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGeneration => f.write_str("image.generation"),
            Self::ImageEdit => f.write_str("image.edit"),
            Self::ImageVariation => f.write_str("image.variation"),
        }
    }
}
impl ::std::str::FromStr for UsageImagesParamsSourcesItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image.generation" => Ok(Self::ImageGeneration),
            "image.edit" => Ok(Self::ImageEdit),
            "image.variation" => Ok(Self::ImageVariation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageImagesParamsSourcesItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageImagesParamsSourcesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated images usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageImagesResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The number of images processed."]
    pub images: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageImagesResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageImagesResult {
    pub fn builder() -> builder::UsageImagesResult {
        Default::default()
    }
}
#[doc = "`UsageImagesResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageImagesResultObject {
    #[serde(rename = "organization.usage.images.result")]
    OrganizationUsageImagesResult,
}
impl ::std::fmt::Display for UsageImagesResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageImagesResult => f.write_str("organization.usage.images.result"),
        }
    }
}
impl ::std::str::FromStr for UsageImagesResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.images.result" => Ok(Self::OrganizationUsageImagesResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageImagesResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageImagesResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageModerationsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageModerationsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageModerationsParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageModerationsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageModerationsParams {
    pub fn builder() -> builder::UsageModerationsParams {
        Default::default()
    }
}
#[doc = "`UsageModerationsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageModerationsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageModerationsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageModerationsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageModerationsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageModerationsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageModerationsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageModerationsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
}
impl ::std::fmt::Display for UsageModerationsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
        }
    }
}
impl ::std::str::FromStr for UsageModerationsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageModerationsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageModerationsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated moderations usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageModerationsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The aggregated number of input tokens used."]
    pub input_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: UsageModerationsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageModerationsResult {
    pub fn builder() -> builder::UsageModerationsResult {
        Default::default()
    }
}
#[doc = "`UsageModerationsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageModerationsResultObject {
    #[serde(rename = "organization.usage.moderations.result")]
    OrganizationUsageModerationsResult,
}
impl ::std::fmt::Display for UsageModerationsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageModerationsResult => {
                f.write_str("organization.usage.moderations.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageModerationsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.moderations.result" => Ok(Self::OrganizationUsageModerationsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageModerationsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageModerationsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageResponse {
    pub data: ::std::vec::Vec<UsageTimeBucket>,
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next_page: ::std::option::Option<::std::string::String>,
    pub object: UsageResponseObject,
}
impl UsageResponse {
    pub fn builder() -> builder::UsageResponse {
        Default::default()
    }
}
#[doc = "`UsageResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageResponseObject {
    #[serde(rename = "page")]
    Page,
}
impl ::std::fmt::Display for UsageResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Page => f.write_str("page"),
        }
    }
}
impl ::std::str::FromStr for UsageResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "page" => Ok(Self::Page),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageTimeBucket`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageTimeBucket {
    pub end_time: i64,
    pub object: UsageTimeBucketObject,
    pub results: ::std::vec::Vec<UsageTimeBucketResultsItem>,
    pub start_time: i64,
}
impl UsageTimeBucket {
    pub fn builder() -> builder::UsageTimeBucket {
        Default::default()
    }
}
#[doc = "`UsageTimeBucketObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageTimeBucketObject {
    #[serde(rename = "bucket")]
    Bucket,
}
impl ::std::fmt::Display for UsageTimeBucketObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bucket => f.write_str("bucket"),
        }
    }
}
impl ::std::str::FromStr for UsageTimeBucketObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bucket" => Ok(Self::Bucket),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageTimeBucketObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageTimeBucketObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageTimeBucketResultsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum UsageTimeBucketResultsItem {
    UsageCompletionsResult(UsageCompletionsResult),
    UsageEmbeddingsResult(UsageEmbeddingsResult),
    UsageModerationsResult(UsageModerationsResult),
    UsageImagesResult(UsageImagesResult),
    UsageAudioSpeechesResult(UsageAudioSpeechesResult),
    UsageAudioTranscriptionsResult(UsageAudioTranscriptionsResult),
    UsageVectorStoresResult(UsageVectorStoresResult),
    UsageCodeInterpreterSessionsResult(UsageCodeInterpreterSessionsResult),
    UsageFileSearchCallsResult(UsageFileSearchCallsResult),
    UsageWebSearchCallsResult(UsageWebSearchCallsResult),
    CostsResult(CostsResult),
}
impl ::std::convert::From<UsageCompletionsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageCompletionsResult) -> Self {
        Self::UsageCompletionsResult(value)
    }
}
impl ::std::convert::From<UsageEmbeddingsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageEmbeddingsResult) -> Self {
        Self::UsageEmbeddingsResult(value)
    }
}
impl ::std::convert::From<UsageModerationsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageModerationsResult) -> Self {
        Self::UsageModerationsResult(value)
    }
}
impl ::std::convert::From<UsageImagesResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageImagesResult) -> Self {
        Self::UsageImagesResult(value)
    }
}
impl ::std::convert::From<UsageAudioSpeechesResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageAudioSpeechesResult) -> Self {
        Self::UsageAudioSpeechesResult(value)
    }
}
impl ::std::convert::From<UsageAudioTranscriptionsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageAudioTranscriptionsResult) -> Self {
        Self::UsageAudioTranscriptionsResult(value)
    }
}
impl ::std::convert::From<UsageVectorStoresResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageVectorStoresResult) -> Self {
        Self::UsageVectorStoresResult(value)
    }
}
impl ::std::convert::From<UsageCodeInterpreterSessionsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageCodeInterpreterSessionsResult) -> Self {
        Self::UsageCodeInterpreterSessionsResult(value)
    }
}
impl ::std::convert::From<UsageFileSearchCallsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageFileSearchCallsResult) -> Self {
        Self::UsageFileSearchCallsResult(value)
    }
}
impl ::std::convert::From<UsageWebSearchCallsResult> for UsageTimeBucketResultsItem {
    fn from(value: UsageWebSearchCallsResult) -> Self {
        Self::UsageWebSearchCallsResult(value)
    }
}
impl ::std::convert::From<CostsResult> for UsageTimeBucketResultsItem {
    fn from(value: CostsResult) -> Self {
        Self::CostsResult(value)
    }
}
#[doc = "`UsageVectorStoresParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageVectorStoresParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageVectorStoresParamsBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageVectorStoresParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
}
impl UsageVectorStoresParams {
    pub fn builder() -> builder::UsageVectorStoresParams {
        Default::default()
    }
}
#[doc = "`UsageVectorStoresParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageVectorStoresParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageVectorStoresParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageVectorStoresParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageVectorStoresParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageVectorStoresParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageVectorStoresParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageVectorStoresParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
}
impl ::std::fmt::Display for UsageVectorStoresParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
        }
    }
}
impl ::std::str::FromStr for UsageVectorStoresParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageVectorStoresParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageVectorStoresParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated vector stores usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageVectorStoresResult {
    pub object: UsageVectorStoresResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[doc = "The vector stores usage in bytes."]
    pub usage_bytes: i64,
}
impl UsageVectorStoresResult {
    pub fn builder() -> builder::UsageVectorStoresResult {
        Default::default()
    }
}
#[doc = "`UsageVectorStoresResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageVectorStoresResultObject {
    #[serde(rename = "organization.usage.vector_stores.result")]
    OrganizationUsageVectorStoresResult,
}
impl ::std::fmt::Display for UsageVectorStoresResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageVectorStoresResult => {
                f.write_str("organization.usage.vector_stores.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageVectorStoresResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.vector_stores.result" => {
                Ok(Self::OrganizationUsageVectorStoresResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageVectorStoresResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageVectorStoresResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageWebSearchCallsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageWebSearchCallsParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub api_key_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<UsageWebSearchCallsParamsBucketWidth>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub context_levels: ::std::vec::Vec<UsageWebSearchCallsParamsContextLevelsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub group_by: ::std::vec::Vec<UsageWebSearchCallsParamsGroupByItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub models: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    pub start_time: i64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub user_ids: ::std::vec::Vec<::std::string::String>,
}
impl UsageWebSearchCallsParams {
    pub fn builder() -> builder::UsageWebSearchCallsParams {
        Default::default()
    }
}
#[doc = "`UsageWebSearchCallsParamsBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageWebSearchCallsParamsBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for UsageWebSearchCallsParamsBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for UsageWebSearchCallsParamsBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageWebSearchCallsParamsBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageWebSearchCallsParamsContextLevelsItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageWebSearchCallsParamsContextLevelsItem {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for UsageWebSearchCallsParamsContextLevelsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for UsageWebSearchCallsParamsContextLevelsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsParamsContextLevelsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageWebSearchCallsParamsContextLevelsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageWebSearchCallsParamsGroupByItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageWebSearchCallsParamsGroupByItem {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "context_level")]
    ContextLevel,
    #[serde(rename = "api_source")]
    ApiSource,
}
impl ::std::fmt::Display for UsageWebSearchCallsParamsGroupByItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
            Self::ContextLevel => f.write_str("context_level"),
            Self::ApiSource => f.write_str("api_source"),
        }
    }
}
impl ::std::str::FromStr for UsageWebSearchCallsParamsGroupByItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            "context_level" => Ok(Self::ContextLevel),
            "api_source" => Ok(Self::ApiSource),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageWebSearchCallsParamsGroupByItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The aggregated web search calls usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsageWebSearchCallsResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "When grouped by `api_source`, `agents_api` identifies attributed Agents API activity and `unlabeled` includes all records without published source attribution, including historical and unknown origins. Unlabeled does not imply direct API usage. Without source grouping, this field is null."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_source: ::std::option::Option<UsageWebSearchCallsResultApiSource>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context_level: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of model requests."]
    pub num_model_requests: i64,
    #[doc = "The count of web search calls."]
    pub num_requests: i64,
    pub object: UsageWebSearchCallsResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl UsageWebSearchCallsResult {
    pub fn builder() -> builder::UsageWebSearchCallsResult {
        Default::default()
    }
}
#[doc = "`UsageWebSearchCallsResultApiSource`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageWebSearchCallsResultApiSource {
    #[serde(rename = "agents_api")]
    AgentsApi,
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ::std::fmt::Display for UsageWebSearchCallsResultApiSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentsApi => f.write_str("agents_api"),
            Self::Unlabeled => f.write_str("unlabeled"),
        }
    }
}
impl ::std::str::FromStr for UsageWebSearchCallsResultApiSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agents_api" => Ok(Self::AgentsApi),
            "unlabeled" => Ok(Self::Unlabeled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageWebSearchCallsResultApiSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UsageWebSearchCallsResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UsageWebSearchCallsResultObject {
    #[serde(rename = "organization.usage.web_searches.result")]
    OrganizationUsageWebSearchesResult,
}
impl ::std::fmt::Display for UsageWebSearchCallsResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageWebSearchesResult => {
                f.write_str("organization.usage.web_searches.result")
            }
        }
    }
}
impl ::std::str::FromStr for UsageWebSearchCallsResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.web_searches.result" => {
                Ok(Self::OrganizationUsageWebSearchesResult)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UsageWebSearchCallsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsageWebSearchCallsResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CostsResult {
        amount: ::std::result::Result<
            ::std::option::Option<super::CostsResultAmount>,
            ::std::string::String,
        >,
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        api_source: ::std::result::Result<
            ::std::option::Option<super::CostsResultApiSource>,
            ::std::string::String,
        >,
        line_item: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::CostsResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        quantity_unit: ::std::result::Result<
            ::std::option::Option<super::CostsResultQuantityUnit>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CostsResult {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                api_key_id: Ok(Default::default()),
                api_source: Ok(Default::default()),
                line_item: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                quantity: Ok(Default::default()),
                quantity_unit: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl CostsResult {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CostsResultAmount>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amount: {e}"));
            self
        }
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn api_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CostsResultApiSource>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_source: {e}"));
            self
        }
        pub fn line_item<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_item: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CostsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn quantity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.quantity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quantity: {e}"));
            self
        }
        pub fn quantity_unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CostsResultQuantityUnit>>,
            T::Error: ::std::fmt::Display,
        {
            self.quantity_unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quantity_unit: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CostsResult> for super::CostsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CostsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                api_key_id: value.api_key_id?,
                api_source: value.api_source?,
                line_item: value.line_item?,
                object: value.object?,
                project_id: value.project_id?,
                quantity: value.quantity?,
                quantity_unit: value.quantity_unit?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::CostsResult> for CostsResult {
        fn from(value: super::CostsResult) -> Self {
            Self {
                amount: Ok(value.amount),
                api_key_id: Ok(value.api_key_id),
                api_source: Ok(value.api_source),
                line_item: Ok(value.line_item),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                quantity: Ok(value.quantity),
                quantity_unit: Ok(value.quantity_unit),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CostsResultAmount {
        currency: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CostsResultAmount {
        fn default() -> Self {
            Self {
                currency: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl CostsResultAmount {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CostsResultAmount> for super::CostsResultAmount {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CostsResultAmount,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::CostsResultAmount> for CostsResultAmount {
        fn from(value: super::CostsResultAmount) -> Self {
            Self {
                currency: Ok(value.currency),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CostsResultQuantityUnit {
        subtype_0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::CostsResultQuantityUnitSubtype1>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<::std::option::Option<()>, ::std::string::String>,
    }
    impl ::std::default::Default for CostsResultQuantityUnit {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl CostsResultQuantityUnit {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CostsResultQuantityUnitSubtype1>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<()>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CostsResultQuantityUnit> for super::CostsResultQuantityUnit {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CostsResultQuantityUnit,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::CostsResultQuantityUnit> for CostsResultQuantityUnit {
        fn from(value: super::CostsResultQuantityUnit) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageApiSchema {
        usage_audio_speeches_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_audio_speeches_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_audio_speeches_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_audio_speeches_params:
            ::std::result::Result<super::UsageAudioSpeechesParams, ::std::string::String>,
        usage_audio_speeches_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_audio_transcriptions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_audio_transcriptions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_audio_transcriptions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_audio_transcriptions_params:
            ::std::result::Result<super::UsageAudioTranscriptionsParams, ::std::string::String>,
        usage_audio_transcriptions_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_code_interpreter_sessions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_code_interpreter_sessions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_code_interpreter_sessions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_code_interpreter_sessions_params:
            ::std::result::Result<super::UsageCodeInterpreterSessionsParams, ::std::string::String>,
        usage_code_interpreter_sessions_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_completions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_completions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_completions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_completions_params:
            ::std::result::Result<super::UsageCompletionsParams, ::std::string::String>,
        usage_completions_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_costs_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_costs_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_costs_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_costs_params: ::std::result::Result<super::UsageCostsParams, ::std::string::String>,
        usage_costs_response: ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_embeddings_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_embeddings_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_embeddings_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_embeddings_params:
            ::std::result::Result<super::UsageEmbeddingsParams, ::std::string::String>,
        usage_embeddings_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_file_search_calls_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_file_search_calls_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_file_search_calls_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_file_search_calls_params:
            ::std::result::Result<super::UsageFileSearchCallsParams, ::std::string::String>,
        usage_file_search_calls_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_images_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_images_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_images_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_images_params: ::std::result::Result<super::UsageImagesParams, ::std::string::String>,
        usage_images_response: ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_moderations_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_moderations_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_moderations_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_moderations_params:
            ::std::result::Result<super::UsageModerationsParams, ::std::string::String>,
        usage_moderations_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_vector_stores_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_vector_stores_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_vector_stores_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_vector_stores_params:
            ::std::result::Result<super::UsageVectorStoresParams, ::std::string::String>,
        usage_vector_stores_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
        usage_web_search_calls_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_web_search_calls_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        usage_web_search_calls_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        usage_web_search_calls_params:
            ::std::result::Result<super::UsageWebSearchCallsParams, ::std::string::String>,
        usage_web_search_calls_response:
            ::std::result::Result<super::UsageResponse, ::std::string::String>,
    }
    impl ::std::default::Default for UsageApiSchema {
        fn default() -> Self {
            Self {
                usage_audio_speeches_error400: Err(
                    "no value supplied for usage_audio_speeches_error400".to_string(),
                ),
                usage_audio_speeches_error403: Err(
                    "no value supplied for usage_audio_speeches_error403".to_string(),
                ),
                usage_audio_speeches_error429: Err(
                    "no value supplied for usage_audio_speeches_error429".to_string(),
                ),
                usage_audio_speeches_params: Err(
                    "no value supplied for usage_audio_speeches_params".to_string(),
                ),
                usage_audio_speeches_response: Err(
                    "no value supplied for usage_audio_speeches_response".to_string(),
                ),
                usage_audio_transcriptions_error400: Err(
                    "no value supplied for usage_audio_transcriptions_error400".to_string(),
                ),
                usage_audio_transcriptions_error403: Err(
                    "no value supplied for usage_audio_transcriptions_error403".to_string(),
                ),
                usage_audio_transcriptions_error429: Err(
                    "no value supplied for usage_audio_transcriptions_error429".to_string(),
                ),
                usage_audio_transcriptions_params: Err(
                    "no value supplied for usage_audio_transcriptions_params".to_string(),
                ),
                usage_audio_transcriptions_response: Err(
                    "no value supplied for usage_audio_transcriptions_response".to_string(),
                ),
                usage_code_interpreter_sessions_error400: Err(
                    "no value supplied for usage_code_interpreter_sessions_error400".to_string(),
                ),
                usage_code_interpreter_sessions_error403: Err(
                    "no value supplied for usage_code_interpreter_sessions_error403".to_string(),
                ),
                usage_code_interpreter_sessions_error429: Err(
                    "no value supplied for usage_code_interpreter_sessions_error429".to_string(),
                ),
                usage_code_interpreter_sessions_params: Err(
                    "no value supplied for usage_code_interpreter_sessions_params".to_string(),
                ),
                usage_code_interpreter_sessions_response: Err(
                    "no value supplied for usage_code_interpreter_sessions_response".to_string(),
                ),
                usage_completions_error400: Err(
                    "no value supplied for usage_completions_error400".to_string()
                ),
                usage_completions_error403: Err(
                    "no value supplied for usage_completions_error403".to_string()
                ),
                usage_completions_error429: Err(
                    "no value supplied for usage_completions_error429".to_string()
                ),
                usage_completions_params: Err(
                    "no value supplied for usage_completions_params".to_string()
                ),
                usage_completions_response: Err(
                    "no value supplied for usage_completions_response".to_string()
                ),
                usage_costs_error400: Err("no value supplied for usage_costs_error400".to_string()),
                usage_costs_error403: Err("no value supplied for usage_costs_error403".to_string()),
                usage_costs_error429: Err("no value supplied for usage_costs_error429".to_string()),
                usage_costs_params: Err("no value supplied for usage_costs_params".to_string()),
                usage_costs_response: Err("no value supplied for usage_costs_response".to_string()),
                usage_embeddings_error400: Err(
                    "no value supplied for usage_embeddings_error400".to_string()
                ),
                usage_embeddings_error403: Err(
                    "no value supplied for usage_embeddings_error403".to_string()
                ),
                usage_embeddings_error429: Err(
                    "no value supplied for usage_embeddings_error429".to_string()
                ),
                usage_embeddings_params: Err(
                    "no value supplied for usage_embeddings_params".to_string()
                ),
                usage_embeddings_response: Err(
                    "no value supplied for usage_embeddings_response".to_string()
                ),
                usage_file_search_calls_error400: Err(
                    "no value supplied for usage_file_search_calls_error400".to_string(),
                ),
                usage_file_search_calls_error403: Err(
                    "no value supplied for usage_file_search_calls_error403".to_string(),
                ),
                usage_file_search_calls_error429: Err(
                    "no value supplied for usage_file_search_calls_error429".to_string(),
                ),
                usage_file_search_calls_params: Err(
                    "no value supplied for usage_file_search_calls_params".to_string(),
                ),
                usage_file_search_calls_response: Err(
                    "no value supplied for usage_file_search_calls_response".to_string(),
                ),
                usage_images_error400: Err(
                    "no value supplied for usage_images_error400".to_string()
                ),
                usage_images_error403: Err(
                    "no value supplied for usage_images_error403".to_string()
                ),
                usage_images_error429: Err(
                    "no value supplied for usage_images_error429".to_string()
                ),
                usage_images_params: Err("no value supplied for usage_images_params".to_string()),
                usage_images_response: Err(
                    "no value supplied for usage_images_response".to_string()
                ),
                usage_moderations_error400: Err(
                    "no value supplied for usage_moderations_error400".to_string()
                ),
                usage_moderations_error403: Err(
                    "no value supplied for usage_moderations_error403".to_string()
                ),
                usage_moderations_error429: Err(
                    "no value supplied for usage_moderations_error429".to_string()
                ),
                usage_moderations_params: Err(
                    "no value supplied for usage_moderations_params".to_string()
                ),
                usage_moderations_response: Err(
                    "no value supplied for usage_moderations_response".to_string()
                ),
                usage_vector_stores_error400: Err(
                    "no value supplied for usage_vector_stores_error400".to_string(),
                ),
                usage_vector_stores_error403: Err(
                    "no value supplied for usage_vector_stores_error403".to_string(),
                ),
                usage_vector_stores_error429: Err(
                    "no value supplied for usage_vector_stores_error429".to_string(),
                ),
                usage_vector_stores_params: Err(
                    "no value supplied for usage_vector_stores_params".to_string()
                ),
                usage_vector_stores_response: Err(
                    "no value supplied for usage_vector_stores_response".to_string(),
                ),
                usage_web_search_calls_error400: Err(
                    "no value supplied for usage_web_search_calls_error400".to_string(),
                ),
                usage_web_search_calls_error403: Err(
                    "no value supplied for usage_web_search_calls_error403".to_string(),
                ),
                usage_web_search_calls_error429: Err(
                    "no value supplied for usage_web_search_calls_error429".to_string(),
                ),
                usage_web_search_calls_params: Err(
                    "no value supplied for usage_web_search_calls_params".to_string(),
                ),
                usage_web_search_calls_response: Err(
                    "no value supplied for usage_web_search_calls_response".to_string(),
                ),
            }
        }
    }
    impl UsageApiSchema {
        pub fn usage_audio_speeches_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_speeches_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_audio_speeches_error400: {e}")
            });
            self
        }
        pub fn usage_audio_speeches_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_speeches_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_audio_speeches_error403: {e}")
            });
            self
        }
        pub fn usage_audio_speeches_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_speeches_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_audio_speeches_error429: {e}")
            });
            self
        }
        pub fn usage_audio_speeches_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageAudioSpeechesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_speeches_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_audio_speeches_params: {e}")
            });
            self
        }
        pub fn usage_audio_speeches_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_speeches_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_audio_speeches_response: {e}")
            });
            self
        }
        pub fn usage_audio_transcriptions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_transcriptions_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for usage_audio_transcriptions_error400: {e}"
                )
            });
            self
        }
        pub fn usage_audio_transcriptions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_transcriptions_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for usage_audio_transcriptions_error403: {e}"
                )
            });
            self
        }
        pub fn usage_audio_transcriptions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_transcriptions_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for usage_audio_transcriptions_error429: {e}"
                )
            });
            self
        }
        pub fn usage_audio_transcriptions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageAudioTranscriptionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_transcriptions_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for usage_audio_transcriptions_params: {e}"
                )
            });
            self
        }
        pub fn usage_audio_transcriptions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_audio_transcriptions_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for usage_audio_transcriptions_response: {e}"
                )
            });
            self
        }
        pub fn usage_code_interpreter_sessions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . usage_code_interpreter_sessions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for usage_code_interpreter_sessions_error400: {e}")) ;
            self
        }
        pub fn usage_code_interpreter_sessions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . usage_code_interpreter_sessions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for usage_code_interpreter_sessions_error403: {e}")) ;
            self
        }
        pub fn usage_code_interpreter_sessions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . usage_code_interpreter_sessions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for usage_code_interpreter_sessions_error429: {e}")) ;
            self
        }
        pub fn usage_code_interpreter_sessions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageCodeInterpreterSessionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . usage_code_interpreter_sessions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for usage_code_interpreter_sessions_params: {e}")) ;
            self
        }
        pub fn usage_code_interpreter_sessions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . usage_code_interpreter_sessions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for usage_code_interpreter_sessions_response: {e}")) ;
            self
        }
        pub fn usage_completions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_completions_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_completions_error400: {e}")
            });
            self
        }
        pub fn usage_completions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_completions_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_completions_error403: {e}")
            });
            self
        }
        pub fn usage_completions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_completions_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_completions_error429: {e}")
            });
            self
        }
        pub fn usage_completions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageCompletionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_completions_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_completions_params: {e}")
            });
            self
        }
        pub fn usage_completions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_completions_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_completions_response: {e}")
            });
            self
        }
        pub fn usage_costs_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_costs_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_costs_error400: {e}")
            });
            self
        }
        pub fn usage_costs_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_costs_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_costs_error403: {e}")
            });
            self
        }
        pub fn usage_costs_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_costs_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_costs_error429: {e}")
            });
            self
        }
        pub fn usage_costs_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageCostsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_costs_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_costs_params: {e}")
            });
            self
        }
        pub fn usage_costs_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_costs_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_costs_response: {e}")
            });
            self
        }
        pub fn usage_embeddings_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_embeddings_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_embeddings_error400: {e}")
            });
            self
        }
        pub fn usage_embeddings_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_embeddings_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_embeddings_error403: {e}")
            });
            self
        }
        pub fn usage_embeddings_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_embeddings_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_embeddings_error429: {e}")
            });
            self
        }
        pub fn usage_embeddings_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageEmbeddingsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_embeddings_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_embeddings_params: {e}")
            });
            self
        }
        pub fn usage_embeddings_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_embeddings_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_embeddings_response: {e}")
            });
            self
        }
        pub fn usage_file_search_calls_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_file_search_calls_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_file_search_calls_error400: {e}")
            });
            self
        }
        pub fn usage_file_search_calls_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_file_search_calls_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_file_search_calls_error403: {e}")
            });
            self
        }
        pub fn usage_file_search_calls_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_file_search_calls_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_file_search_calls_error429: {e}")
            });
            self
        }
        pub fn usage_file_search_calls_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageFileSearchCallsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_file_search_calls_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_file_search_calls_params: {e}")
            });
            self
        }
        pub fn usage_file_search_calls_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_file_search_calls_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_file_search_calls_response: {e}")
            });
            self
        }
        pub fn usage_images_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_images_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_images_error400: {e}")
            });
            self
        }
        pub fn usage_images_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_images_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_images_error403: {e}")
            });
            self
        }
        pub fn usage_images_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_images_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_images_error429: {e}")
            });
            self
        }
        pub fn usage_images_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageImagesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_images_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_images_params: {e}")
            });
            self
        }
        pub fn usage_images_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_images_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_images_response: {e}")
            });
            self
        }
        pub fn usage_moderations_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_moderations_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_moderations_error400: {e}")
            });
            self
        }
        pub fn usage_moderations_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_moderations_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_moderations_error403: {e}")
            });
            self
        }
        pub fn usage_moderations_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_moderations_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_moderations_error429: {e}")
            });
            self
        }
        pub fn usage_moderations_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageModerationsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_moderations_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_moderations_params: {e}")
            });
            self
        }
        pub fn usage_moderations_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_moderations_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_moderations_response: {e}")
            });
            self
        }
        pub fn usage_vector_stores_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_vector_stores_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_vector_stores_error400: {e}")
            });
            self
        }
        pub fn usage_vector_stores_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_vector_stores_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_vector_stores_error403: {e}")
            });
            self
        }
        pub fn usage_vector_stores_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_vector_stores_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_vector_stores_error429: {e}")
            });
            self
        }
        pub fn usage_vector_stores_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageVectorStoresParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_vector_stores_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_vector_stores_params: {e}")
            });
            self
        }
        pub fn usage_vector_stores_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_vector_stores_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_vector_stores_response: {e}")
            });
            self
        }
        pub fn usage_web_search_calls_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_web_search_calls_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_web_search_calls_error400: {e}")
            });
            self
        }
        pub fn usage_web_search_calls_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_web_search_calls_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_web_search_calls_error403: {e}")
            });
            self
        }
        pub fn usage_web_search_calls_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_web_search_calls_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_web_search_calls_error429: {e}")
            });
            self
        }
        pub fn usage_web_search_calls_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageWebSearchCallsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_web_search_calls_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_web_search_calls_params: {e}")
            });
            self
        }
        pub fn usage_web_search_calls_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_web_search_calls_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_web_search_calls_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UsageApiSchema> for super::UsageApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                usage_audio_speeches_error400: value.usage_audio_speeches_error400?,
                usage_audio_speeches_error403: value.usage_audio_speeches_error403?,
                usage_audio_speeches_error429: value.usage_audio_speeches_error429?,
                usage_audio_speeches_params: value.usage_audio_speeches_params?,
                usage_audio_speeches_response: value.usage_audio_speeches_response?,
                usage_audio_transcriptions_error400: value.usage_audio_transcriptions_error400?,
                usage_audio_transcriptions_error403: value.usage_audio_transcriptions_error403?,
                usage_audio_transcriptions_error429: value.usage_audio_transcriptions_error429?,
                usage_audio_transcriptions_params: value.usage_audio_transcriptions_params?,
                usage_audio_transcriptions_response: value.usage_audio_transcriptions_response?,
                usage_code_interpreter_sessions_error400: value
                    .usage_code_interpreter_sessions_error400?,
                usage_code_interpreter_sessions_error403: value
                    .usage_code_interpreter_sessions_error403?,
                usage_code_interpreter_sessions_error429: value
                    .usage_code_interpreter_sessions_error429?,
                usage_code_interpreter_sessions_params: value
                    .usage_code_interpreter_sessions_params?,
                usage_code_interpreter_sessions_response: value
                    .usage_code_interpreter_sessions_response?,
                usage_completions_error400: value.usage_completions_error400?,
                usage_completions_error403: value.usage_completions_error403?,
                usage_completions_error429: value.usage_completions_error429?,
                usage_completions_params: value.usage_completions_params?,
                usage_completions_response: value.usage_completions_response?,
                usage_costs_error400: value.usage_costs_error400?,
                usage_costs_error403: value.usage_costs_error403?,
                usage_costs_error429: value.usage_costs_error429?,
                usage_costs_params: value.usage_costs_params?,
                usage_costs_response: value.usage_costs_response?,
                usage_embeddings_error400: value.usage_embeddings_error400?,
                usage_embeddings_error403: value.usage_embeddings_error403?,
                usage_embeddings_error429: value.usage_embeddings_error429?,
                usage_embeddings_params: value.usage_embeddings_params?,
                usage_embeddings_response: value.usage_embeddings_response?,
                usage_file_search_calls_error400: value.usage_file_search_calls_error400?,
                usage_file_search_calls_error403: value.usage_file_search_calls_error403?,
                usage_file_search_calls_error429: value.usage_file_search_calls_error429?,
                usage_file_search_calls_params: value.usage_file_search_calls_params?,
                usage_file_search_calls_response: value.usage_file_search_calls_response?,
                usage_images_error400: value.usage_images_error400?,
                usage_images_error403: value.usage_images_error403?,
                usage_images_error429: value.usage_images_error429?,
                usage_images_params: value.usage_images_params?,
                usage_images_response: value.usage_images_response?,
                usage_moderations_error400: value.usage_moderations_error400?,
                usage_moderations_error403: value.usage_moderations_error403?,
                usage_moderations_error429: value.usage_moderations_error429?,
                usage_moderations_params: value.usage_moderations_params?,
                usage_moderations_response: value.usage_moderations_response?,
                usage_vector_stores_error400: value.usage_vector_stores_error400?,
                usage_vector_stores_error403: value.usage_vector_stores_error403?,
                usage_vector_stores_error429: value.usage_vector_stores_error429?,
                usage_vector_stores_params: value.usage_vector_stores_params?,
                usage_vector_stores_response: value.usage_vector_stores_response?,
                usage_web_search_calls_error400: value.usage_web_search_calls_error400?,
                usage_web_search_calls_error403: value.usage_web_search_calls_error403?,
                usage_web_search_calls_error429: value.usage_web_search_calls_error429?,
                usage_web_search_calls_params: value.usage_web_search_calls_params?,
                usage_web_search_calls_response: value.usage_web_search_calls_response?,
            })
        }
    }
    impl ::std::convert::From<super::UsageApiSchema> for UsageApiSchema {
        fn from(value: super::UsageApiSchema) -> Self {
            Self {
                usage_audio_speeches_error400: Ok(value.usage_audio_speeches_error400),
                usage_audio_speeches_error403: Ok(value.usage_audio_speeches_error403),
                usage_audio_speeches_error429: Ok(value.usage_audio_speeches_error429),
                usage_audio_speeches_params: Ok(value.usage_audio_speeches_params),
                usage_audio_speeches_response: Ok(value.usage_audio_speeches_response),
                usage_audio_transcriptions_error400: Ok(value.usage_audio_transcriptions_error400),
                usage_audio_transcriptions_error403: Ok(value.usage_audio_transcriptions_error403),
                usage_audio_transcriptions_error429: Ok(value.usage_audio_transcriptions_error429),
                usage_audio_transcriptions_params: Ok(value.usage_audio_transcriptions_params),
                usage_audio_transcriptions_response: Ok(value.usage_audio_transcriptions_response),
                usage_code_interpreter_sessions_error400: Ok(
                    value.usage_code_interpreter_sessions_error400
                ),
                usage_code_interpreter_sessions_error403: Ok(
                    value.usage_code_interpreter_sessions_error403
                ),
                usage_code_interpreter_sessions_error429: Ok(
                    value.usage_code_interpreter_sessions_error429
                ),
                usage_code_interpreter_sessions_params: Ok(
                    value.usage_code_interpreter_sessions_params
                ),
                usage_code_interpreter_sessions_response: Ok(
                    value.usage_code_interpreter_sessions_response
                ),
                usage_completions_error400: Ok(value.usage_completions_error400),
                usage_completions_error403: Ok(value.usage_completions_error403),
                usage_completions_error429: Ok(value.usage_completions_error429),
                usage_completions_params: Ok(value.usage_completions_params),
                usage_completions_response: Ok(value.usage_completions_response),
                usage_costs_error400: Ok(value.usage_costs_error400),
                usage_costs_error403: Ok(value.usage_costs_error403),
                usage_costs_error429: Ok(value.usage_costs_error429),
                usage_costs_params: Ok(value.usage_costs_params),
                usage_costs_response: Ok(value.usage_costs_response),
                usage_embeddings_error400: Ok(value.usage_embeddings_error400),
                usage_embeddings_error403: Ok(value.usage_embeddings_error403),
                usage_embeddings_error429: Ok(value.usage_embeddings_error429),
                usage_embeddings_params: Ok(value.usage_embeddings_params),
                usage_embeddings_response: Ok(value.usage_embeddings_response),
                usage_file_search_calls_error400: Ok(value.usage_file_search_calls_error400),
                usage_file_search_calls_error403: Ok(value.usage_file_search_calls_error403),
                usage_file_search_calls_error429: Ok(value.usage_file_search_calls_error429),
                usage_file_search_calls_params: Ok(value.usage_file_search_calls_params),
                usage_file_search_calls_response: Ok(value.usage_file_search_calls_response),
                usage_images_error400: Ok(value.usage_images_error400),
                usage_images_error403: Ok(value.usage_images_error403),
                usage_images_error429: Ok(value.usage_images_error429),
                usage_images_params: Ok(value.usage_images_params),
                usage_images_response: Ok(value.usage_images_response),
                usage_moderations_error400: Ok(value.usage_moderations_error400),
                usage_moderations_error403: Ok(value.usage_moderations_error403),
                usage_moderations_error429: Ok(value.usage_moderations_error429),
                usage_moderations_params: Ok(value.usage_moderations_params),
                usage_moderations_response: Ok(value.usage_moderations_response),
                usage_vector_stores_error400: Ok(value.usage_vector_stores_error400),
                usage_vector_stores_error403: Ok(value.usage_vector_stores_error403),
                usage_vector_stores_error429: Ok(value.usage_vector_stores_error429),
                usage_vector_stores_params: Ok(value.usage_vector_stores_params),
                usage_vector_stores_response: Ok(value.usage_vector_stores_response),
                usage_web_search_calls_error400: Ok(value.usage_web_search_calls_error400),
                usage_web_search_calls_error403: Ok(value.usage_web_search_calls_error403),
                usage_web_search_calls_error429: Ok(value.usage_web_search_calls_error429),
                usage_web_search_calls_params: Ok(value.usage_web_search_calls_params),
                usage_web_search_calls_response: Ok(value.usage_web_search_calls_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageAudioSpeechesParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageAudioSpeechesParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageAudioSpeechesParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageAudioSpeechesParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageAudioSpeechesParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageAudioSpeechesParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageAudioSpeechesParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageAudioSpeechesParams> for super::UsageAudioSpeechesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageAudioSpeechesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageAudioSpeechesParams> for UsageAudioSpeechesParams {
        fn from(value: super::UsageAudioSpeechesParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageAudioSpeechesResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        characters: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageAudioSpeechesResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageAudioSpeechesResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                characters: Err("no value supplied for characters".to_string()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageAudioSpeechesResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn characters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.characters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for characters: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageAudioSpeechesResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageAudioSpeechesResult> for super::UsageAudioSpeechesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageAudioSpeechesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                characters: value.characters?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageAudioSpeechesResult> for UsageAudioSpeechesResult {
        fn from(value: super::UsageAudioSpeechesResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                characters: Ok(value.characters),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageAudioTranscriptionsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageAudioTranscriptionsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageAudioTranscriptionsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageAudioTranscriptionsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageAudioTranscriptionsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageAudioTranscriptionsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::UsageAudioTranscriptionsParamsGroupByItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageAudioTranscriptionsParams>
        for super::UsageAudioTranscriptionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageAudioTranscriptionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageAudioTranscriptionsParams>
        for UsageAudioTranscriptionsParams
    {
        fn from(value: super::UsageAudioTranscriptionsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageAudioTranscriptionsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<
            super::UsageAudioTranscriptionsResultObject,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        seconds: ::std::result::Result<i64, ::std::string::String>,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageAudioTranscriptionsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                seconds: Err("no value supplied for seconds".to_string()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageAudioTranscriptionsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageAudioTranscriptionsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageAudioTranscriptionsResult>
        for super::UsageAudioTranscriptionsResult
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageAudioTranscriptionsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                project_id: value.project_id?,
                seconds: value.seconds?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageAudioTranscriptionsResult>
        for UsageAudioTranscriptionsResult
    {
        fn from(value: super::UsageAudioTranscriptionsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                seconds: Ok(value.seconds),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageCodeInterpreterSessionsParams {
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageCodeInterpreterSessionsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageCodeInterpreterSessionsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for UsageCodeInterpreterSessionsParams {
        fn default() -> Self {
            Self {
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl UsageCodeInterpreterSessionsParams {
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageCodeInterpreterSessionsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::UsageCodeInterpreterSessionsParamsGroupByItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageCodeInterpreterSessionsParams>
        for super::UsageCodeInterpreterSessionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageCodeInterpreterSessionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::UsageCodeInterpreterSessionsParams>
        for UsageCodeInterpreterSessionsParams
    {
        fn from(value: super::UsageCodeInterpreterSessionsParams) -> Self {
            Self {
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageCodeInterpreterSessionsResult {
        num_sessions: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<
            super::UsageCodeInterpreterSessionsResultObject,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageCodeInterpreterSessionsResult {
        fn default() -> Self {
            Self {
                num_sessions: Err("no value supplied for num_sessions".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
            }
        }
    }
    impl UsageCodeInterpreterSessionsResult {
        pub fn num_sessions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_sessions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_sessions: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageCodeInterpreterSessionsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageCodeInterpreterSessionsResult>
        for super::UsageCodeInterpreterSessionsResult
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageCodeInterpreterSessionsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                num_sessions: value.num_sessions?,
                object: value.object?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageCodeInterpreterSessionsResult>
        for UsageCodeInterpreterSessionsResult
    {
        fn from(value: super::UsageCodeInterpreterSessionsResult) -> Self {
            Self {
                num_sessions: Ok(value.num_sessions),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageCompletionsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        batch: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageCompletionsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageCompletionsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageCompletionsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                batch: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageCompletionsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn batch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageCompletionsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageCompletionsParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageCompletionsParams> for super::UsageCompletionsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageCompletionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                batch: value.batch?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageCompletionsParams> for UsageCompletionsParams {
        fn from(value: super::UsageCompletionsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                batch: Ok(value.batch),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageCompletionsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        api_source: ::std::result::Result<
            ::std::option::Option<super::UsageCompletionsResultApiSource>,
            ::std::string::String,
        >,
        batch: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        input_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cache_write_12h_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cache_write_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_text_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_text_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        input_uncached_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageCompletionsResultObject, ::std::string::String>,
        output_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_text_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_tokens: ::std::result::Result<i64, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        service_tier: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageCompletionsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                api_source: Ok(Default::default()),
                batch: Ok(Default::default()),
                input_audio_tokens: Ok(Default::default()),
                input_cache_write_12h_tokens: Ok(Default::default()),
                input_cache_write_tokens: Ok(Default::default()),
                input_cached_audio_tokens: Ok(Default::default()),
                input_cached_image_tokens: Ok(Default::default()),
                input_cached_text_tokens: Ok(Default::default()),
                input_cached_tokens: Ok(Default::default()),
                input_image_tokens: Ok(Default::default()),
                input_text_tokens: Ok(Default::default()),
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                input_uncached_tokens: Ok(Default::default()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                output_audio_tokens: Ok(Default::default()),
                output_image_tokens: Ok(Default::default()),
                output_text_tokens: Ok(Default::default()),
                output_tokens: Err("no value supplied for output_tokens".to_string()),
                project_id: Ok(Default::default()),
                service_tier: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageCompletionsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn api_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageCompletionsResultApiSource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.api_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_source: {e}"));
            self
        }
        pub fn batch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch: {e}"));
            self
        }
        pub fn input_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_audio_tokens: {e}")
            });
            self
        }
        pub fn input_cache_write_12h_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cache_write_12h_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cache_write_12h_tokens: {e}")
            });
            self
        }
        pub fn input_cache_write_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cache_write_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cache_write_tokens: {e}")
            });
            self
        }
        pub fn input_cached_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_audio_tokens: {e}")
            });
            self
        }
        pub fn input_cached_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_image_tokens: {e}")
            });
            self
        }
        pub fn input_cached_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_text_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_text_tokens: {e}")
            });
            self
        }
        pub fn input_cached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_tokens: {e}")
            });
            self
        }
        pub fn input_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_image_tokens: {e}")
            });
            self
        }
        pub fn input_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_text_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_text_tokens: {e}"));
            self
        }
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn input_uncached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_uncached_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_uncached_tokens: {e}")
            });
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageCompletionsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn output_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_audio_tokens: {e}")
            });
            self
        }
        pub fn output_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_image_tokens: {e}")
            });
            self
        }
        pub fn output_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_text_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_text_tokens: {e}")
            });
            self
        }
        pub fn output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_tokens: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageCompletionsResult> for super::UsageCompletionsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageCompletionsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                api_source: value.api_source?,
                batch: value.batch?,
                input_audio_tokens: value.input_audio_tokens?,
                input_cache_write_12h_tokens: value.input_cache_write_12h_tokens?,
                input_cache_write_tokens: value.input_cache_write_tokens?,
                input_cached_audio_tokens: value.input_cached_audio_tokens?,
                input_cached_image_tokens: value.input_cached_image_tokens?,
                input_cached_text_tokens: value.input_cached_text_tokens?,
                input_cached_tokens: value.input_cached_tokens?,
                input_image_tokens: value.input_image_tokens?,
                input_text_tokens: value.input_text_tokens?,
                input_tokens: value.input_tokens?,
                input_uncached_tokens: value.input_uncached_tokens?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                output_audio_tokens: value.output_audio_tokens?,
                output_image_tokens: value.output_image_tokens?,
                output_text_tokens: value.output_text_tokens?,
                output_tokens: value.output_tokens?,
                project_id: value.project_id?,
                service_tier: value.service_tier?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageCompletionsResult> for UsageCompletionsResult {
        fn from(value: super::UsageCompletionsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                api_source: Ok(value.api_source),
                batch: Ok(value.batch),
                input_audio_tokens: Ok(value.input_audio_tokens),
                input_cache_write_12h_tokens: Ok(value.input_cache_write_12h_tokens),
                input_cache_write_tokens: Ok(value.input_cache_write_tokens),
                input_cached_audio_tokens: Ok(value.input_cached_audio_tokens),
                input_cached_image_tokens: Ok(value.input_cached_image_tokens),
                input_cached_text_tokens: Ok(value.input_cached_text_tokens),
                input_cached_tokens: Ok(value.input_cached_tokens),
                input_image_tokens: Ok(value.input_image_tokens),
                input_text_tokens: Ok(value.input_text_tokens),
                input_tokens: Ok(value.input_tokens),
                input_uncached_tokens: Ok(value.input_uncached_tokens),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                output_audio_tokens: Ok(value.output_audio_tokens),
                output_image_tokens: Ok(value.output_image_tokens),
                output_text_tokens: Ok(value.output_text_tokens),
                output_tokens: Ok(value.output_tokens),
                project_id: Ok(value.project_id),
                service_tier: Ok(value.service_tier),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageCostsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageCostsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageCostsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        line_items:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for UsageCostsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                line_items: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl UsageCostsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UsageCostsParamsBucketWidth>>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageCostsParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn line_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_items: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageCostsParams> for super::UsageCostsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageCostsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                line_items: value.line_items?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::UsageCostsParams> for UsageCostsParams {
        fn from(value: super::UsageCostsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                line_items: Ok(value.line_items),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageEmbeddingsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageEmbeddingsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageEmbeddingsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageEmbeddingsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageEmbeddingsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageEmbeddingsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageEmbeddingsParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageEmbeddingsParams> for super::UsageEmbeddingsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageEmbeddingsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageEmbeddingsParams> for UsageEmbeddingsParams {
        fn from(value: super::UsageEmbeddingsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageEmbeddingsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageEmbeddingsResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageEmbeddingsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageEmbeddingsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageEmbeddingsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageEmbeddingsResult> for super::UsageEmbeddingsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageEmbeddingsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                input_tokens: value.input_tokens?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageEmbeddingsResult> for UsageEmbeddingsResult {
        fn from(value: super::UsageEmbeddingsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                input_tokens: Ok(value.input_tokens),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageFileSearchCallsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageFileSearchCallsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageFileSearchCallsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageFileSearchCallsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageFileSearchCallsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageFileSearchCallsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::UsageFileSearchCallsParamsGroupByItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
        pub fn vector_store_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageFileSearchCallsParams> for super::UsageFileSearchCallsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageFileSearchCallsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageFileSearchCallsParams> for UsageFileSearchCallsParams {
        fn from(value: super::UsageFileSearchCallsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageFileSearchCallsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_requests: ::std::result::Result<i64, ::std::string::String>,
        object:
            ::std::result::Result<super::UsageFileSearchCallsResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        vector_store_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageFileSearchCallsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                num_requests: Err("no value supplied for num_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                user_id: Ok(Default::default()),
                vector_store_id: Ok(Default::default()),
            }
        }
    }
    impl UsageFileSearchCallsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn num_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_requests = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_requests: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageFileSearchCallsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageFileSearchCallsResult> for super::UsageFileSearchCallsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageFileSearchCallsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                num_requests: value.num_requests?,
                object: value.object?,
                project_id: value.project_id?,
                user_id: value.user_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageFileSearchCallsResult> for UsageFileSearchCallsResult {
        fn from(value: super::UsageFileSearchCallsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                num_requests: Ok(value.num_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageImagesParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageImagesParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageImagesParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        sizes: ::std::result::Result<
            ::std::vec::Vec<super::UsageImagesParamsSizesItem>,
            ::std::string::String,
        >,
        sources: ::std::result::Result<
            ::std::vec::Vec<super::UsageImagesParamsSourcesItem>,
            ::std::string::String,
        >,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageImagesParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                sizes: Ok(Default::default()),
                sources: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageImagesParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UsageImagesParamsBucketWidth>>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageImagesParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn sizes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageImagesParamsSizesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sizes: {e}"));
            self
        }
        pub fn sources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageImagesParamsSourcesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.sources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sources: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageImagesParams> for super::UsageImagesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageImagesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                sizes: value.sizes?,
                sources: value.sources?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageImagesParams> for UsageImagesParams {
        fn from(value: super::UsageImagesParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                sizes: Ok(value.sizes),
                sources: Ok(value.sources),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageImagesResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        images: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageImagesResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        size: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageImagesResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                images: Err("no value supplied for images".to_string()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                size: Ok(Default::default()),
                source: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageImagesResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageImagesResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageImagesResult> for super::UsageImagesResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageImagesResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                images: value.images?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                project_id: value.project_id?,
                size: value.size?,
                source: value.source?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageImagesResult> for UsageImagesResult {
        fn from(value: super::UsageImagesResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                images: Ok(value.images),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                size: Ok(value.size),
                source: Ok(value.source),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageModerationsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageModerationsParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageModerationsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageModerationsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageModerationsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageModerationsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageModerationsParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageModerationsParams> for super::UsageModerationsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageModerationsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageModerationsParams> for UsageModerationsParams {
        fn from(value: super::UsageModerationsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageModerationsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageModerationsResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageModerationsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageModerationsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageModerationsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageModerationsResult> for super::UsageModerationsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageModerationsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                input_tokens: value.input_tokens?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageModerationsResult> for UsageModerationsResult {
        fn from(value: super::UsageModerationsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                input_tokens: Ok(value.input_tokens),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageResponse {
        data: ::std::result::Result<::std::vec::Vec<super::UsageTimeBucket>, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next_page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::UsageResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for UsageResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next_page: Err("no value supplied for next_page".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl UsageResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageTimeBucket>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn has_more<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.has_more = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for has_more: {e}"));
            self
        }
        pub fn next_page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_page: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageResponse> for super::UsageResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next_page: value.next_page?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::UsageResponse> for UsageResponse {
        fn from(value: super::UsageResponse) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next_page: Ok(value.next_page),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageTimeBucket {
        end_time: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::UsageTimeBucketObject, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::UsageTimeBucketResultsItem>,
            ::std::string::String,
        >,
        start_time: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for UsageTimeBucket {
        fn default() -> Self {
            Self {
                end_time: Err("no value supplied for end_time".to_string()),
                object: Err("no value supplied for object".to_string()),
                results: Err("no value supplied for results".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl UsageTimeBucket {
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageTimeBucketObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageTimeBucketResultsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageTimeBucket> for super::UsageTimeBucket {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageTimeBucket,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_time: value.end_time?,
                object: value.object?,
                results: value.results?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::UsageTimeBucket> for UsageTimeBucket {
        fn from(value: super::UsageTimeBucket) -> Self {
            Self {
                end_time: Ok(value.end_time),
                object: Ok(value.object),
                results: Ok(value.results),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageVectorStoresParams {
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageVectorStoresParamsBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageVectorStoresParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for UsageVectorStoresParams {
        fn default() -> Self {
            Self {
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl UsageVectorStoresParams {
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageVectorStoresParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UsageVectorStoresParamsGroupByItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageVectorStoresParams> for super::UsageVectorStoresParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageVectorStoresParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::UsageVectorStoresParams> for UsageVectorStoresParams {
        fn from(value: super::UsageVectorStoresParams) -> Self {
            Self {
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageVectorStoresResult {
        object: ::std::result::Result<super::UsageVectorStoresResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        usage_bytes: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for UsageVectorStoresResult {
        fn default() -> Self {
            Self {
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                usage_bytes: Err("no value supplied for usage_bytes".to_string()),
            }
        }
    }
    impl UsageVectorStoresResult {
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageVectorStoresResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn usage_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage_bytes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageVectorStoresResult> for super::UsageVectorStoresResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageVectorStoresResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                object: value.object?,
                project_id: value.project_id?,
                usage_bytes: value.usage_bytes?,
            })
        }
    }
    impl ::std::convert::From<super::UsageVectorStoresResult> for UsageVectorStoresResult {
        fn from(value: super::UsageVectorStoresResult) -> Self {
            Self {
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                usage_bytes: Ok(value.usage_bytes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageWebSearchCallsParams {
        api_key_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::UsageWebSearchCallsParamsBucketWidth>,
            ::std::string::String,
        >,
        context_levels: ::std::result::Result<
            ::std::vec::Vec<super::UsageWebSearchCallsParamsContextLevelsItem>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::vec::Vec<super::UsageWebSearchCallsParamsGroupByItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for UsageWebSearchCallsParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                context_levels: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl UsageWebSearchCallsParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageWebSearchCallsParamsBucketWidth>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn context_levels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::UsageWebSearchCallsParamsContextLevelsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.context_levels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_levels: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::UsageWebSearchCallsParamsGroupByItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
            self
        }
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageWebSearchCallsParams> for super::UsageWebSearchCallsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageWebSearchCallsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                bucket_width: value.bucket_width?,
                context_levels: value.context_levels?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::UsageWebSearchCallsParams> for UsageWebSearchCallsParams {
        fn from(value: super::UsageWebSearchCallsParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                bucket_width: Ok(value.bucket_width),
                context_levels: Ok(value.context_levels),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsageWebSearchCallsResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        api_source: ::std::result::Result<
            ::std::option::Option<super::UsageWebSearchCallsResultApiSource>,
            ::std::string::String,
        >,
        context_level: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        num_requests: ::std::result::Result<i64, ::std::string::String>,
        object:
            ::std::result::Result<super::UsageWebSearchCallsResultObject, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UsageWebSearchCallsResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                api_source: Ok(Default::default()),
                context_level: Ok(Default::default()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                num_requests: Err("no value supplied for num_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl UsageWebSearchCallsResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn api_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UsageWebSearchCallsResultApiSource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.api_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_source: {e}"));
            self
        }
        pub fn context_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.context_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_level: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn num_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_requests = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_requests: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UsageWebSearchCallsResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UsageWebSearchCallsResult> for super::UsageWebSearchCallsResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsageWebSearchCallsResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                api_source: value.api_source?,
                context_level: value.context_level?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                num_requests: value.num_requests?,
                object: value.object?,
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UsageWebSearchCallsResult> for UsageWebSearchCallsResult {
        fn from(value: super::UsageWebSearchCallsResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                api_source: Ok(value.api_source),
                context_level: Ok(value.context_level),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                num_requests: Ok(value.num_requests),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
