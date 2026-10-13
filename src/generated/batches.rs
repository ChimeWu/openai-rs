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
impl Batch {
    pub fn builder() -> builder::Batch {
        Default::default()
    }
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
impl BatchApiSchema {
    pub fn builder() -> builder::BatchApiSchema {
        Default::default()
    }
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
impl BatchError {
    pub fn builder() -> builder::BatchError {
        Default::default()
    }
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
impl BatchErrors {
    pub fn builder() -> builder::BatchErrors {
        Default::default()
    }
}
#[doc = "The expiration policy for the output and/or error file that are generated for a batch."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchFileExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `created_at`. Note that the anchor is the file creation time, not the time the batch is created."]
    pub anchor: BatchFileExpirationAfterAnchor,
    #[doc = "The number of seconds after the anchor time that the file will expire. Must be between 3600 (1 hour) and 2592000 (30 days)."]
    pub seconds: i64,
}
impl BatchFileExpirationAfter {
    pub fn builder() -> builder::BatchFileExpirationAfter {
        Default::default()
    }
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
impl BatchRequestCounts {
    pub fn builder() -> builder::BatchRequestCounts {
        Default::default()
    }
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
impl BatchUsage {
    pub fn builder() -> builder::BatchUsage {
        Default::default()
    }
}
#[doc = "A detailed breakdown of the input tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchUsageInputTokensDetails {
    #[doc = "The number of tokens that were retrieved from the cache. [More on\nprompt caching](https://developers.openai.com/api/docs/guides/prompt-caching).\n"]
    pub cached_tokens: i64,
}
impl BatchUsageInputTokensDetails {
    pub fn builder() -> builder::BatchUsageInputTokensDetails {
        Default::default()
    }
}
#[doc = "A detailed breakdown of the output tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BatchUsageOutputTokensDetails {
    #[doc = "The number of reasoning tokens."]
    pub reasoning_tokens: i64,
}
impl BatchUsageOutputTokensDetails {
    pub fn builder() -> builder::BatchUsageOutputTokensDetails {
        Default::default()
    }
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
impl CreateBatchRequest {
    pub fn builder() -> builder::CreateBatchRequest {
        Default::default()
    }
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
impl ListBatchesParams {
    pub fn builder() -> builder::ListBatchesParams {
        Default::default()
    }
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
impl ListBatchesResponse {
    pub fn builder() -> builder::ListBatchesResponse {
        Default::default()
    }
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Batch {
        cancelled_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        cancelling_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        completed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        completion_window: ::std::result::Result<::std::string::String, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        error_file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        errors:
            ::std::result::Result<::std::option::Option<super::BatchErrors>, ::std::string::String>,
        expired_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        failed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        finalizing_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        in_progress_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::BatchObject, ::std::string::String>,
        output_file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        request_counts: ::std::result::Result<
            ::std::option::Option<super::BatchRequestCounts>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::BatchStatus, ::std::string::String>,
        usage:
            ::std::result::Result<::std::option::Option<super::BatchUsage>, ::std::string::String>,
    }
    impl ::std::default::Default for Batch {
        fn default() -> Self {
            Self {
                cancelled_at: Ok(Default::default()),
                cancelling_at: Ok(Default::default()),
                completed_at: Ok(Default::default()),
                completion_window: Err("no value supplied for completion_window".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                endpoint: Err("no value supplied for endpoint".to_string()),
                error_file_id: Ok(Default::default()),
                errors: Ok(Default::default()),
                expired_at: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                failed_at: Ok(Default::default()),
                finalizing_at: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                in_progress_at: Ok(Default::default()),
                input_file_id: Err("no value supplied for input_file_id".to_string()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                output_file_id: Ok(Default::default()),
                request_counts: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl Batch {
        pub fn cancelled_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cancelled_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cancelled_at: {e}"));
            self
        }
        pub fn cancelling_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cancelling_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cancelling_at: {e}"));
            self
        }
        pub fn completed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.completed_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed_at: {e}"));
            self
        }
        pub fn completion_window<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.completion_window = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion_window: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn endpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.endpoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endpoint: {e}"));
            self
        }
        pub fn error_file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_file_id: {e}"));
            self
        }
        pub fn errors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BatchErrors>>,
            T::Error: ::std::fmt::Display,
        {
            self.errors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for errors: {e}"));
            self
        }
        pub fn expired_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.expired_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expired_at: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn failed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.failed_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for failed_at: {e}"));
            self
        }
        pub fn finalizing_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.finalizing_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finalizing_at: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn in_progress_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.in_progress_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_progress_at: {e}"));
            self
        }
        pub fn input_file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.input_file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_file_id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Metadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BatchObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn output_file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_file_id: {e}"));
            self
        }
        pub fn request_counts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BatchRequestCounts>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_counts: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BatchStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BatchUsage>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Batch> for super::Batch {
        type Error = super::error::ConversionError;
        fn try_from(value: Batch) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancelled_at: value.cancelled_at?,
                cancelling_at: value.cancelling_at?,
                completed_at: value.completed_at?,
                completion_window: value.completion_window?,
                created_at: value.created_at?,
                endpoint: value.endpoint?,
                error_file_id: value.error_file_id?,
                errors: value.errors?,
                expired_at: value.expired_at?,
                expires_at: value.expires_at?,
                failed_at: value.failed_at?,
                finalizing_at: value.finalizing_at?,
                id: value.id?,
                in_progress_at: value.in_progress_at?,
                input_file_id: value.input_file_id?,
                metadata: value.metadata?,
                model: value.model?,
                object: value.object?,
                output_file_id: value.output_file_id?,
                request_counts: value.request_counts?,
                status: value.status?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::Batch> for Batch {
        fn from(value: super::Batch) -> Self {
            Self {
                cancelled_at: Ok(value.cancelled_at),
                cancelling_at: Ok(value.cancelling_at),
                completed_at: Ok(value.completed_at),
                completion_window: Ok(value.completion_window),
                created_at: Ok(value.created_at),
                endpoint: Ok(value.endpoint),
                error_file_id: Ok(value.error_file_id),
                errors: Ok(value.errors),
                expired_at: Ok(value.expired_at),
                expires_at: Ok(value.expires_at),
                failed_at: Ok(value.failed_at),
                finalizing_at: Ok(value.finalizing_at),
                id: Ok(value.id),
                in_progress_at: Ok(value.in_progress_at),
                input_file_id: Ok(value.input_file_id),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                object: Ok(value.object),
                output_file_id: Ok(value.output_file_id),
                request_counts: Ok(value.request_counts),
                status: Ok(value.status),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchApiSchema {
        cancel_batch_response: ::std::result::Result<super::Batch, ::std::string::String>,
        create_batch_request:
            ::std::result::Result<super::CreateBatchRequest, ::std::string::String>,
        create_batch_response: ::std::result::Result<super::Batch, ::std::string::String>,
        list_batches_params: ::std::result::Result<super::ListBatchesParams, ::std::string::String>,
        list_batches_response:
            ::std::result::Result<super::ListBatchesResponse, ::std::string::String>,
        retrieve_batch_response: ::std::result::Result<super::Batch, ::std::string::String>,
    }
    impl ::std::default::Default for BatchApiSchema {
        fn default() -> Self {
            Self {
                cancel_batch_response: Err(
                    "no value supplied for cancel_batch_response".to_string()
                ),
                create_batch_request: Err("no value supplied for create_batch_request".to_string()),
                create_batch_response: Err(
                    "no value supplied for create_batch_response".to_string()
                ),
                list_batches_params: Err("no value supplied for list_batches_params".to_string()),
                list_batches_response: Err(
                    "no value supplied for list_batches_response".to_string()
                ),
                retrieve_batch_response: Err(
                    "no value supplied for retrieve_batch_response".to_string()
                ),
            }
        }
    }
    impl BatchApiSchema {
        pub fn cancel_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Batch>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_batch_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_batch_response: {e}")
            });
            self
        }
        pub fn create_batch_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateBatchRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_batch_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_batch_request: {e}")
            });
            self
        }
        pub fn create_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Batch>,
            T::Error: ::std::fmt::Display,
        {
            self.create_batch_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_batch_response: {e}")
            });
            self
        }
        pub fn list_batches_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListBatchesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_batches_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_batches_params: {e}")
            });
            self
        }
        pub fn list_batches_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListBatchesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_batches_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_batches_response: {e}")
            });
            self
        }
        pub fn retrieve_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Batch>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_batch_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_batch_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<BatchApiSchema> for super::BatchApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_batch_response: value.cancel_batch_response?,
                create_batch_request: value.create_batch_request?,
                create_batch_response: value.create_batch_response?,
                list_batches_params: value.list_batches_params?,
                list_batches_response: value.list_batches_response?,
                retrieve_batch_response: value.retrieve_batch_response?,
            })
        }
    }
    impl ::std::convert::From<super::BatchApiSchema> for BatchApiSchema {
        fn from(value: super::BatchApiSchema) -> Self {
            Self {
                cancel_batch_response: Ok(value.cancel_batch_response),
                create_batch_request: Ok(value.create_batch_request),
                create_batch_response: Ok(value.create_batch_response),
                list_batches_params: Ok(value.list_batches_params),
                list_batches_response: Ok(value.list_batches_response),
                retrieve_batch_response: Ok(value.retrieve_batch_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchError {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        line: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        param: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BatchError {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                line: Ok(Default::default()),
                message: Ok(Default::default()),
                param: Ok(Default::default()),
            }
        }
    }
    impl BatchError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn param<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.param = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for param: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchError> for super::BatchError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                line: value.line?,
                message: value.message?,
                param: value.param?,
            })
        }
    }
    impl ::std::convert::From<super::BatchError> for BatchError {
        fn from(value: super::BatchError) -> Self {
            Self {
                code: Ok(value.code),
                line: Ok(value.line),
                message: Ok(value.message),
                param: Ok(value.param),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchErrors {
        data: ::std::result::Result<::std::vec::Vec<super::BatchError>, ::std::string::String>,
        object: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BatchErrors {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                object: Ok(Default::default()),
            }
        }
    }
    impl BatchErrors {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BatchError>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchErrors> for super::BatchErrors {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchErrors,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::BatchErrors> for BatchErrors {
        fn from(value: super::BatchErrors) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchFileExpirationAfter {
        anchor: ::std::result::Result<super::BatchFileExpirationAfterAnchor, ::std::string::String>,
        seconds: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for BatchFileExpirationAfter {
        fn default() -> Self {
            Self {
                anchor: Err("no value supplied for anchor".to_string()),
                seconds: Err("no value supplied for seconds".to_string()),
            }
        }
    }
    impl BatchFileExpirationAfter {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BatchFileExpirationAfterAnchor>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
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
    }
    impl ::std::convert::TryFrom<BatchFileExpirationAfter> for super::BatchFileExpirationAfter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchFileExpirationAfter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                seconds: value.seconds?,
            })
        }
    }
    impl ::std::convert::From<super::BatchFileExpirationAfter> for BatchFileExpirationAfter {
        fn from(value: super::BatchFileExpirationAfter) -> Self {
            Self {
                anchor: Ok(value.anchor),
                seconds: Ok(value.seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchRequestCounts {
        completed: ::std::result::Result<i64, ::std::string::String>,
        failed: ::std::result::Result<i64, ::std::string::String>,
        total: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for BatchRequestCounts {
        fn default() -> Self {
            Self {
                completed: Err("no value supplied for completed".to_string()),
                failed: Err("no value supplied for failed".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl BatchRequestCounts {
        pub fn completed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.completed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed: {e}"));
            self
        }
        pub fn failed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.failed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for failed: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchRequestCounts> for super::BatchRequestCounts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchRequestCounts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completed: value.completed?,
                failed: value.failed?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::BatchRequestCounts> for BatchRequestCounts {
        fn from(value: super::BatchRequestCounts) -> Self {
            Self {
                completed: Ok(value.completed),
                failed: Ok(value.failed),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchUsage {
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        input_tokens_details:
            ::std::result::Result<super::BatchUsageInputTokensDetails, ::std::string::String>,
        output_tokens: ::std::result::Result<i64, ::std::string::String>,
        output_tokens_details:
            ::std::result::Result<super::BatchUsageOutputTokensDetails, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for BatchUsage {
        fn default() -> Self {
            Self {
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                input_tokens_details: Err("no value supplied for input_tokens_details".to_string()),
                output_tokens: Err("no value supplied for output_tokens".to_string()),
                output_tokens_details: Err(
                    "no value supplied for output_tokens_details".to_string()
                ),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl BatchUsage {
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
        pub fn input_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BatchUsageInputTokensDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_tokens_details: {e}")
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
        pub fn output_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BatchUsageOutputTokensDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_tokens_details: {e}")
            });
            self
        }
        pub fn total_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.total_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchUsage> for super::BatchUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_tokens: value.input_tokens?,
                input_tokens_details: value.input_tokens_details?,
                output_tokens: value.output_tokens?,
                output_tokens_details: value.output_tokens_details?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::BatchUsage> for BatchUsage {
        fn from(value: super::BatchUsage) -> Self {
            Self {
                input_tokens: Ok(value.input_tokens),
                input_tokens_details: Ok(value.input_tokens_details),
                output_tokens: Ok(value.output_tokens),
                output_tokens_details: Ok(value.output_tokens_details),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchUsageInputTokensDetails {
        cached_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for BatchUsageInputTokensDetails {
        fn default() -> Self {
            Self {
                cached_tokens: Err("no value supplied for cached_tokens".to_string()),
            }
        }
    }
    impl BatchUsageInputTokensDetails {
        pub fn cached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cached_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cached_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchUsageInputTokensDetails> for super::BatchUsageInputTokensDetails {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchUsageInputTokensDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cached_tokens: value.cached_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::BatchUsageInputTokensDetails> for BatchUsageInputTokensDetails {
        fn from(value: super::BatchUsageInputTokensDetails) -> Self {
            Self {
                cached_tokens: Ok(value.cached_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BatchUsageOutputTokensDetails {
        reasoning_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for BatchUsageOutputTokensDetails {
        fn default() -> Self {
            Self {
                reasoning_tokens: Err("no value supplied for reasoning_tokens".to_string()),
            }
        }
    }
    impl BatchUsageOutputTokensDetails {
        pub fn reasoning_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BatchUsageOutputTokensDetails>
        for super::BatchUsageOutputTokensDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BatchUsageOutputTokensDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reasoning_tokens: value.reasoning_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::BatchUsageOutputTokensDetails> for BatchUsageOutputTokensDetails {
        fn from(value: super::BatchUsageOutputTokensDetails) -> Self {
            Self {
                reasoning_tokens: Ok(value.reasoning_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateBatchRequest {
        completion_window:
            ::std::result::Result<super::CreateBatchRequestCompletionWindow, ::std::string::String>,
        endpoint: ::std::result::Result<super::CreateBatchRequestEndpoint, ::std::string::String>,
        input_file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        output_expires_after: ::std::result::Result<
            ::std::option::Option<super::BatchFileExpirationAfter>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateBatchRequest {
        fn default() -> Self {
            Self {
                completion_window: Err("no value supplied for completion_window".to_string()),
                endpoint: Err("no value supplied for endpoint".to_string()),
                input_file_id: Err("no value supplied for input_file_id".to_string()),
                metadata: Ok(Default::default()),
                output_expires_after: Ok(Default::default()),
            }
        }
    }
    impl CreateBatchRequest {
        pub fn completion_window<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateBatchRequestCompletionWindow>,
            T::Error: ::std::fmt::Display,
        {
            self.completion_window = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion_window: {e}"));
            self
        }
        pub fn endpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateBatchRequestEndpoint>,
            T::Error: ::std::fmt::Display,
        {
            self.endpoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for endpoint: {e}"));
            self
        }
        pub fn input_file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.input_file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_file_id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Metadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn output_expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BatchFileExpirationAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_expires_after = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_expires_after: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateBatchRequest> for super::CreateBatchRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateBatchRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion_window: value.completion_window?,
                endpoint: value.endpoint?,
                input_file_id: value.input_file_id?,
                metadata: value.metadata?,
                output_expires_after: value.output_expires_after?,
            })
        }
    }
    impl ::std::convert::From<super::CreateBatchRequest> for CreateBatchRequest {
        fn from(value: super::CreateBatchRequest) -> Self {
            Self {
                completion_window: Ok(value.completion_window),
                endpoint: Ok(value.endpoint),
                input_file_id: Ok(value.input_file_id),
                metadata: Ok(value.metadata),
                output_expires_after: Ok(value.output_expires_after),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListBatchesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListBatchesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListBatchesParams {
        pub fn after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for after: {e}"));
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
    }
    impl ::std::convert::TryFrom<ListBatchesParams> for super::ListBatchesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListBatchesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListBatchesParams> for ListBatchesParams {
        fn from(value: super::ListBatchesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListBatchesResponse {
        data: ::std::result::Result<::std::vec::Vec<super::Batch>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ListBatchesResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ListBatchesResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Ok(Default::default()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListBatchesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Batch>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn first_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_id: {e}"));
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
        pub fn last_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListBatchesResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListBatchesResponse> for super::ListBatchesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListBatchesResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                first_id: value.first_id?,
                has_more: value.has_more?,
                last_id: value.last_id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ListBatchesResponse> for ListBatchesResponse {
        fn from(value: super::ListBatchesResponse) -> Self {
            Self {
                data: Ok(value.data),
                first_id: Ok(value.first_id),
                has_more: Ok(value.has_more),
                last_id: Ok(value.last_id),
                object: Ok(value.object),
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
