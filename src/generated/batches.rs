#[doc = "`Batch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Batch {
    #[doc = "The Unix timestamp (in seconds) for when the batch was cancelled."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cancelled_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch started cancelling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cancelling_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch was completed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "The time frame within which the batch should be processed."]
    pub completion_window: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) for when the batch was created."]
    pub created_at: i64,
    #[doc = "The OpenAI API endpoint used by the batch."]
    pub endpoint: ::std::string::String,
    #[doc = "The ID of the file containing the outputs of requests with errors."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub errors: ::std::option::Option<BatchErrors>,
    #[doc = "The Unix timestamp (in seconds) for when the batch expired."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch will expire."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch failed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub failed_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch started finalizing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub finalizing_at: ::std::option::Option<i64>,
    pub id: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) for when the batch started processing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub in_progress_at: ::std::option::Option<i64>,
    #[doc = "The ID of the input file for the batch."]
    pub input_file_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[doc = "Model ID used to process the batch, like `gpt-6-astra`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model\nguide](https://developers.openai.com/api/docs/models) to browse and compare available models.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `batch`."]
    pub object: BatchObject,
    #[doc = "The ID of the file containing the outputs of successfully executed requests."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_counts: ::std::option::Option<BatchRequestCounts>,
    #[doc = "The current status of the batch."]
    pub status: BatchStatus,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<BatchUsage>,
}
#[doc = "`BatchApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchApiSchema {
    #[serde(rename = "CancelBatchResponse")]
    pub cancel_batch_response: Batch,
    #[serde(rename = "CreateBatchRequest")]
    pub create_batch_request: CreateBatchRequest,
    #[serde(rename = "CreateBatchResponse")]
    pub create_batch_response: Batch,
    #[serde(rename = "ListBatchesParams")]
    pub list_batches_params: ListBatchesParams,
    #[serde(rename = "ListBatchesResponse")]
    pub list_batches_response: ListBatchesResponse,
    #[serde(rename = "RetrieveBatchResponse")]
    pub retrieve_batch_response: Batch,
}
#[doc = "`BatchError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct BatchError {
    #[doc = "An error code identifying the error type."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub line: ::std::option::Option<i64>,
    #[doc = "A human-readable message providing more details about the error."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub param: ::std::option::Option<::std::string::String>,
}
#[doc = "`BatchErrors`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct BatchErrors {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub data: ::std::vec::Vec<BatchError>,
    #[doc = "The object type, which is always `list`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
}
#[doc = "The expiration policy for the output and/or error file that are generated for a batch."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchFileExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`. Note that the anchor is the file creation time, not the time the batch is created."]
    pub anchor: BatchFileExpirationAfterAnchor,
    #[doc = "The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days)."]
    pub seconds: i64,
}
#[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`. Note that the anchor is the file creation time, not the time the batch is created."]
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
pub enum BatchFileExpirationAfterAnchor {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl ::std::fmt::Display for BatchFileExpirationAfterAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
        }
    }
}
impl ::std::str::FromStr for BatchFileExpirationAfterAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BatchFileExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BatchFileExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `batch`."]
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
pub enum BatchObject {
    #[serde(rename = "batch")]
    Batch,
}
impl ::std::fmt::Display for BatchObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Batch => f.write_str("batch"),
        }
    }
}
impl ::std::str::FromStr for BatchObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "batch" => Ok(Self::Batch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BatchObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BatchObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The request counts for different statuses within the batch."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchRequestCounts {
    #[doc = "Number of requests that have been completed successfully."]
    pub completed: i64,
    #[doc = "Number of requests that have failed."]
    pub failed: i64,
    #[doc = "Total number of requests in the batch."]
    pub total: i64,
}
#[doc = "The current status of the batch."]
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
pub enum BatchStatus {
    #[serde(rename = "validating")]
    Validating,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finalizing")]
    Finalizing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Validating => f.write_str("validating"),
            Self::Failed => f.write_str("failed"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Finalizing => f.write_str("finalizing"),
            Self::Completed => f.write_str("completed"),
            Self::Expired => f.write_str("expired"),
            Self::Cancelling => f.write_str("cancelling"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for BatchStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "validating" => Ok(Self::Validating),
            "failed" => Ok(Self::Failed),
            "in_progress" => Ok(Self::InProgress),
            "finalizing" => Ok(Self::Finalizing),
            "completed" => Ok(Self::Completed),
            "expired" => Ok(Self::Expired),
            "cancelling" => Ok(Self::Cancelling),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BatchStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BatchStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents token usage details including input tokens, output tokens, a\nbreakdown of output tokens, and the total tokens used. Only populated on\nbatches created after September 7, 2025.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchUsage {
    #[doc = "The number of input tokens."]
    pub input_tokens: i64,
    pub input_tokens_details: BatchUsageInputTokensDetails,
    #[doc = "The number of output tokens."]
    pub output_tokens: i64,
    pub output_tokens_details: BatchUsageOutputTokensDetails,
    #[doc = "The total number of tokens used."]
    pub total_tokens: i64,
}
#[doc = "A detailed breakdown of the input tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchUsageInputTokensDetails {
    #[doc = "The number of tokens that were retrieved from the cache. [More on\nprompt caching](https://developers.openai.com/api/docs/guides/prompt-caching).\n"]
    pub cached_tokens: i64,
}
#[doc = "A detailed breakdown of the output tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchUsageOutputTokensDetails {
    #[doc = "The number of reasoning tokens."]
    pub reasoning_tokens: i64,
}
#[doc = "`CreateBatchRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateBatchRequest {
    #[doc = "The time frame within which the batch should be processed. Currently only `24h` is supported."]
    pub completion_window: CreateBatchRequestCompletionWindow,
    #[doc = "The endpoint to be used for all requests in the batch. Currently `/v1/responses`, `/v1/chat/completions`, `/v1/embeddings`, `/v1/completions`, `/v1/moderations`, `/v1/images/generations`, `/v1/images/edits`, and `/v1/videos` are supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000 embedding inputs across all requests in the batch."]
    pub endpoint: CreateBatchRequestEndpoint,
    #[doc = "The ID of an uploaded file that contains requests for the new batch.\n\nSee [upload file](https://developers.openai.com/api/reference/resources/files/methods/create) for how to upload a file.\n\nYour input file must be formatted as a [JSONL file](https://developers.openai.com/api/docs/guides/batch#1-prepare-your-batch-file), and must be uploaded with the purpose `batch`. The file can contain up to 50,000 requests, and can be up to 200 MB in size.\n"]
    pub input_file_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_expires_after: ::std::option::Option<BatchFileExpirationAfter>,
}
#[doc = "The time frame within which the batch should be processed. Currently only `24h` is supported."]
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
pub enum CreateBatchRequestCompletionWindow {
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for CreateBatchRequestCompletionWindow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for CreateBatchRequestCompletionWindow {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateBatchRequestCompletionWindow {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateBatchRequestCompletionWindow {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The endpoint to be used for all requests in the batch. Currently `/v1/responses`, `/v1/chat/completions`, `/v1/embeddings`, `/v1/completions`, `/v1/moderations`, `/v1/images/generations`, `/v1/images/edits`, and `/v1/videos` are supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000 embedding inputs across all requests in the batch."]
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
pub enum CreateBatchRequestEndpoint {
    #[serde(rename = "/v1/responses")]
    V1Responses,
    #[serde(rename = "/v1/chat/completions")]
    V1ChatCompletions,
    #[serde(rename = "/v1/embeddings")]
    V1Embeddings,
    #[serde(rename = "/v1/completions")]
    V1Completions,
    #[serde(rename = "/v1/moderations")]
    V1Moderations,
    #[serde(rename = "/v1/images/generations")]
    V1ImagesGenerations,
    #[serde(rename = "/v1/images/edits")]
    V1ImagesEdits,
    #[serde(rename = "/v1/videos")]
    V1Videos,
}
impl ::std::fmt::Display for CreateBatchRequestEndpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V1Responses => f.write_str("/v1/responses"),
            Self::V1ChatCompletions => f.write_str("/v1/chat/completions"),
            Self::V1Embeddings => f.write_str("/v1/embeddings"),
            Self::V1Completions => f.write_str("/v1/completions"),
            Self::V1Moderations => f.write_str("/v1/moderations"),
            Self::V1ImagesGenerations => f.write_str("/v1/images/generations"),
            Self::V1ImagesEdits => f.write_str("/v1/images/edits"),
            Self::V1Videos => f.write_str("/v1/videos"),
        }
    }
}
impl ::std::str::FromStr for CreateBatchRequestEndpoint {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "/v1/responses" => Ok(Self::V1Responses),
            "/v1/chat/completions" => Ok(Self::V1ChatCompletions),
            "/v1/embeddings" => Ok(Self::V1Embeddings),
            "/v1/completions" => Ok(Self::V1Completions),
            "/v1/moderations" => Ok(Self::V1Moderations),
            "/v1/images/generations" => Ok(Self::V1ImagesGenerations),
            "/v1/images/edits" => Ok(Self::V1ImagesEdits),
            "/v1/videos" => Ok(Self::V1Videos),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateBatchRequestEndpoint {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateBatchRequestEndpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListBatchesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListBatchesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
#[doc = "`ListBatchesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListBatchesResponse {
    pub data: ::std::vec::Vec<Batch>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListBatchesResponseObject,
}
#[doc = "`ListBatchesResponseObject`"]
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
pub enum ListBatchesResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListBatchesResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListBatchesResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListBatchesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListBatchesResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Metadata`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Metadata(pub ::std::option::Option<MetadataValue>);
impl ::std::ops::Deref for Metadata {
    type Target = ::std::option::Option<MetadataValue>;
    fn deref(&self) -> &::std::option::Option<MetadataValue> {
        &self.0
    }
}
impl ::std::convert::From<Metadata> for ::std::option::Option<MetadataValue> {
    fn from(value: Metadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<MetadataValue>> for Metadata {
    fn from(value: ::std::option::Option<MetadataValue>) -> Self {
        Self(value)
    }
}
#[doc = "Set of 16 key-value pairs that can be attached to an object. This can be\nuseful for storing additional information about the object in a structured\nformat, and querying for objects via API or the dashboard.\n\nKeys are strings with a maximum length of 64 characters. Values are strings\nwith a maximum length of 512 characters.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct MetadataValue(
    pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
);
impl ::std::ops::Deref for MetadataValue {
    type Target = ::std::collections::HashMap<::std::string::String, ::std::string::String>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<MetadataValue>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String>
{
    fn from(value: MetadataValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, ::std::string::String>>
    for MetadataValue
{
    fn from(
        value: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self(value)
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
