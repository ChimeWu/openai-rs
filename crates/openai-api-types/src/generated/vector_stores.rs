pub use crate::common::{ComparisonFilter, CompoundFilter, Error, ErrorResponse, Metadata, MisalignmentErrorDetailsResource, VectorStoreFileAttributes, MisalignmentErrorType, MisalignmentSteer};
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AutoChunkingStrategyRequestParam {
    #[doc = "Always `auto`."]
    #[serde(rename = "type")]
    pub type_: AutoChunkingStrategyRequestParamType,
}
impl AutoChunkingStrategyRequestParam {
    pub fn builder() -> builder::AutoChunkingStrategyRequestParam {
        Default::default()
    }
}
#[doc = "Always `auto`."]
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
pub enum AutoChunkingStrategyRequestParamType {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for AutoChunkingStrategyRequestParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for AutoChunkingStrategyRequestParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AutoChunkingStrategyRequestParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AutoChunkingStrategyRequestParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CancelVectorStoreFileBatchParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CancelVectorStoreFileBatchParams {
    pub batch_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl CancelVectorStoreFileBatchParams {
    pub fn builder() -> builder::CancelVectorStoreFileBatchParams {
        Default::default()
    }
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChunkingStrategyRequestParam {
    AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
    StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}
impl ::std::convert::From<AutoChunkingStrategyRequestParam> for ChunkingStrategyRequestParam {
    fn from(value: AutoChunkingStrategyRequestParam) -> Self {
        Self::AutoChunkingStrategyRequestParam(value)
    }
}
impl ::std::convert::From<StaticChunkingStrategyRequestParam> for ChunkingStrategyRequestParam {
    fn from(value: StaticChunkingStrategyRequestParam) -> Self {
        Self::StaticChunkingStrategyRequestParam(value)
    }
}
#[doc = "`CreateVectorStoreFileBatchParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVectorStoreFileBatchParams {
    pub vector_store_id: ::std::string::String,
}
impl CreateVectorStoreFileBatchParams {
    pub fn builder() -> builder::CreateVectorStoreFileBatchParams {
        Default::default()
    }
}
#[doc = "`CreateVectorStoreFileBatchRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum CreateVectorStoreFileBatchRequest {
    Variant0 {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        attributes: ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        chunking_strategy: ::std::option::Option<ChunkingStrategyRequestParam>,
        #[doc = "A list of [File](https://developers.openai.com/api/reference/resources/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files. If `attributes` or `chunking_strategy` are provided, they will be applied to all files in the batch. The maximum batch size is 2000 files. This endpoint is recommended for multi-file ingestion and helps reduce per-vector-store write request pressure. Mutually exclusive with `files`."]
        file_ids: ::std::vec::Vec<::std::string::String>,
    },
    Variant1 {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        attributes: ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        chunking_strategy: ::std::option::Option<ChunkingStrategyRequestParam>,
        #[doc = "A list of objects that each include a `file_id` plus optional `attributes` or `chunking_strategy`. Use this when you need to override metadata for specific files. The global `attributes` or `chunking_strategy` will be ignored and must be specified for each file. The maximum batch size is 2000 files. This endpoint is recommended for multi-file ingestion and helps reduce per-vector-store write request pressure. Mutually exclusive with `file_ids`."]
        files: ::std::vec::Vec<CreateVectorStoreFileRequest>,
    },
}
#[doc = "`CreateVectorStoreFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVectorStoreFileParams {
    pub vector_store_id: ::std::string::String,
}
impl CreateVectorStoreFileParams {
    pub fn builder() -> builder::CreateVectorStoreFileParams {
        Default::default()
    }
}
#[doc = "`CreateVectorStoreFileRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVectorStoreFileRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<ChunkingStrategyRequestParam>,
    #[doc = "A [File](https://developers.openai.com/api/reference/resources/files) ID that the vector store should use. Useful for tools like `file_search` that can access files. For multi-file ingestion, we recommend [`file_batches`](https://developers.openai.com/api/reference/resources/vector_stores/subresources/file_batches/methods/create) to minimize per-vector-store write requests."]
    pub file_id: ::std::string::String,
}
impl CreateVectorStoreFileRequest {
    pub fn builder() -> builder::CreateVectorStoreFileRequest {
        Default::default()
    }
}
#[doc = "`CreateVectorStoreRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVectorStoreRequest {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<CreateVectorStoreRequestChunkingStrategy>,
    #[doc = "A description for the vector store. Can be used to describe the vector store's purpose."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<VectorStoreExpirationAfter>,
    #[doc = "A list of [File](https://developers.openai.com/api/reference/resources/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The name of the vector store."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl CreateVectorStoreRequest {
    pub fn builder() -> builder::CreateVectorStoreRequest {
        Default::default()
    }
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateVectorStoreRequestChunkingStrategy {
    AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
    StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}
impl ::std::convert::From<AutoChunkingStrategyRequestParam>
    for CreateVectorStoreRequestChunkingStrategy
{
    fn from(value: AutoChunkingStrategyRequestParam) -> Self {
        Self::AutoChunkingStrategyRequestParam(value)
    }
}
impl ::std::convert::From<StaticChunkingStrategyRequestParam>
    for CreateVectorStoreRequestChunkingStrategy
{
    fn from(value: StaticChunkingStrategyRequestParam) -> Self {
        Self::StaticChunkingStrategyRequestParam(value)
    }
}
#[doc = "`DeleteVectorStoreFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVectorStoreFileParams {
    pub file_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl DeleteVectorStoreFileParams {
    pub fn builder() -> builder::DeleteVectorStoreFileParams {
        Default::default()
    }
}
#[doc = "`DeleteVectorStoreFileResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVectorStoreFileResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteVectorStoreFileResponseObject,
}
impl DeleteVectorStoreFileResponse {
    pub fn builder() -> builder::DeleteVectorStoreFileResponse {
        Default::default()
    }
}
#[doc = "`DeleteVectorStoreFileResponseObject`"]
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
pub enum DeleteVectorStoreFileResponseObject {
    #[serde(rename = "vector_store.file.deleted")]
    VectorStoreFileDeleted,
}
impl ::std::fmt::Display for DeleteVectorStoreFileResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreFileDeleted => f.write_str("vector_store.file.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteVectorStoreFileResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.file.deleted" => Ok(Self::VectorStoreFileDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteVectorStoreFileResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteVectorStoreFileResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteVectorStoreParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVectorStoreParams {
    pub vector_store_id: ::std::string::String,
}
impl DeleteVectorStoreParams {
    pub fn builder() -> builder::DeleteVectorStoreParams {
        Default::default()
    }
}
#[doc = "`DeleteVectorStoreResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVectorStoreResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteVectorStoreResponseObject,
}
impl DeleteVectorStoreResponse {
    pub fn builder() -> builder::DeleteVectorStoreResponse {
        Default::default()
    }
}
#[doc = "`DeleteVectorStoreResponseObject`"]
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
pub enum DeleteVectorStoreResponseObject {
    #[serde(rename = "vector_store.deleted")]
    VectorStoreDeleted,
}
impl ::std::fmt::Display for DeleteVectorStoreResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreDeleted => f.write_str("vector_store.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteVectorStoreResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.deleted" => Ok(Self::VectorStoreDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteVectorStoreResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteVectorStoreResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetVectorStoreFileBatchParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVectorStoreFileBatchParams {
    pub batch_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl GetVectorStoreFileBatchParams {
    pub fn builder() -> builder::GetVectorStoreFileBatchParams {
        Default::default()
    }
}
#[doc = "`GetVectorStoreFileParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVectorStoreFileParams {
    pub file_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl GetVectorStoreFileParams {
    pub fn builder() -> builder::GetVectorStoreFileParams {
        Default::default()
    }
}
#[doc = "`GetVectorStoreParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVectorStoreParams {
    pub vector_store_id: ::std::string::String,
}
impl GetVectorStoreParams {
    pub fn builder() -> builder::GetVectorStoreParams {
        Default::default()
    }
}
#[doc = "`ListFilesInVectorStoreBatchParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListFilesInVectorStoreBatchParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub batch_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: ::std::option::Option<ListFilesInVectorStoreBatchParamsFilter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListFilesInVectorStoreBatchParamsOrder>,
    pub vector_store_id: ::std::string::String,
}
impl ListFilesInVectorStoreBatchParams {
    pub fn builder() -> builder::ListFilesInVectorStoreBatchParams {
        Default::default()
    }
}
#[doc = "`ListFilesInVectorStoreBatchParamsFilter`"]
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
pub enum ListFilesInVectorStoreBatchParamsFilter {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for ListFilesInVectorStoreBatchParamsFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for ListFilesInVectorStoreBatchParamsFilter {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFilesInVectorStoreBatchParamsFilter {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListFilesInVectorStoreBatchParamsFilter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFilesInVectorStoreBatchParamsOrder`"]
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
pub enum ListFilesInVectorStoreBatchParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListFilesInVectorStoreBatchParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListFilesInVectorStoreBatchParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFilesInVectorStoreBatchParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListFilesInVectorStoreBatchParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListVectorStoreFilesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListVectorStoreFilesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: ::std::option::Option<ListVectorStoreFilesParamsFilter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListVectorStoreFilesParamsOrder>,
    pub vector_store_id: ::std::string::String,
}
impl ListVectorStoreFilesParams {
    pub fn builder() -> builder::ListVectorStoreFilesParams {
        Default::default()
    }
}
#[doc = "`ListVectorStoreFilesParamsFilter`"]
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
pub enum ListVectorStoreFilesParamsFilter {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for ListVectorStoreFilesParamsFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for ListVectorStoreFilesParamsFilter {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListVectorStoreFilesParamsFilter {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVectorStoreFilesParamsFilter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListVectorStoreFilesParamsOrder`"]
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
pub enum ListVectorStoreFilesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListVectorStoreFilesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListVectorStoreFilesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListVectorStoreFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVectorStoreFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListVectorStoreFilesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListVectorStoreFilesResponse {
    pub data: ::std::vec::Vec<VectorStoreFileObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListVectorStoreFilesResponse {
    pub fn builder() -> builder::ListVectorStoreFilesResponse {
        Default::default()
    }
}
#[doc = "`ListVectorStoresParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListVectorStoresParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListVectorStoresParamsOrder>,
}
impl ListVectorStoresParams {
    pub fn builder() -> builder::ListVectorStoresParams {
        Default::default()
    }
}
#[doc = "`ListVectorStoresParamsOrder`"]
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
pub enum ListVectorStoresParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListVectorStoresParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListVectorStoresParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListVectorStoresParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVectorStoresParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListVectorStoresResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListVectorStoresResponse {
    pub data: ::std::vec::Vec<VectorStoreObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListVectorStoresResponse {
    pub fn builder() -> builder::ListVectorStoresResponse {
        Default::default()
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
#[doc = "`ModifyVectorStoreParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyVectorStoreParams {
    pub vector_store_id: ::std::string::String,
}
impl ModifyVectorStoreParams {
    pub fn builder() -> builder::ModifyVectorStoreParams {
        Default::default()
    }
}
#[doc = "This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OtherChunkingStrategyResponseParam {
    #[doc = "Always `other`."]
    #[serde(rename = "type")]
    pub type_: OtherChunkingStrategyResponseParamType,
}
impl OtherChunkingStrategyResponseParam {
    pub fn builder() -> builder::OtherChunkingStrategyResponseParam {
        Default::default()
    }
}
#[doc = "Always `other`."]
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
pub enum OtherChunkingStrategyResponseParamType {
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for OtherChunkingStrategyResponseParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for OtherChunkingStrategyResponseParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OtherChunkingStrategyResponseParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OtherChunkingStrategyResponseParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveVectorStoreFileContentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveVectorStoreFileContentParams {
    pub file_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl RetrieveVectorStoreFileContentParams {
    pub fn builder() -> builder::RetrieveVectorStoreFileContentParams {
        Default::default()
    }
}
#[doc = "`SearchVectorStoreParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SearchVectorStoreParams {
    pub vector_store_id: ::std::string::String,
}
impl SearchVectorStoreParams {
    pub fn builder() -> builder::SearchVectorStoreParams {
        Default::default()
    }
}
#[doc = "`StaticChunkingStrategy`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StaticChunkingStrategy {
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    pub chunk_overlap_tokens: i64,
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    pub max_chunk_size_tokens: i64,
}
impl StaticChunkingStrategy {
    pub fn builder() -> builder::StaticChunkingStrategy {
        Default::default()
    }
}
#[doc = "Customize your own chunking strategy by setting chunk size and chunk overlap."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StaticChunkingStrategyRequestParam {
    #[serde(rename = "static")]
    pub static_: StaticChunkingStrategy,
    #[doc = "Always `static`."]
    #[serde(rename = "type")]
    pub type_: StaticChunkingStrategyRequestParamType,
}
impl StaticChunkingStrategyRequestParam {
    pub fn builder() -> builder::StaticChunkingStrategyRequestParam {
        Default::default()
    }
}
#[doc = "Always `static`."]
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
pub enum StaticChunkingStrategyRequestParamType {
    #[serde(rename = "static")]
    Static,
}
impl ::std::fmt::Display for StaticChunkingStrategyRequestParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Static => f.write_str("static"),
        }
    }
}
impl ::std::str::FromStr for StaticChunkingStrategyRequestParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "static" => Ok(Self::Static),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StaticChunkingStrategyRequestParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StaticChunkingStrategyRequestParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StaticChunkingStrategyResponseParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct StaticChunkingStrategyResponseParam {
    #[serde(rename = "static")]
    pub static_: StaticChunkingStrategy,
    #[doc = "Always `static`."]
    #[serde(rename = "type")]
    pub type_: StaticChunkingStrategyResponseParamType,
}
impl StaticChunkingStrategyResponseParam {
    pub fn builder() -> builder::StaticChunkingStrategyResponseParam {
        Default::default()
    }
}
#[doc = "Always `static`."]
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
pub enum StaticChunkingStrategyResponseParamType {
    #[serde(rename = "static")]
    Static,
}
impl ::std::fmt::Display for StaticChunkingStrategyResponseParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Static => f.write_str("static"),
        }
    }
}
impl ::std::str::FromStr for StaticChunkingStrategyResponseParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "static" => Ok(Self::Static),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StaticChunkingStrategyResponseParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StaticChunkingStrategyResponseParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateVectorStoreFileAttributesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateVectorStoreFileAttributesParams {
    pub file_id: ::std::string::String,
    pub vector_store_id: ::std::string::String,
}
impl UpdateVectorStoreFileAttributesParams {
    pub fn builder() -> builder::UpdateVectorStoreFileAttributesParams {
        Default::default()
    }
}
#[doc = "`UpdateVectorStoreFileAttributesRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpdateVectorStoreFileAttributesRequest {
    pub attributes: ::openai_api_types::common::VectorStoreFileAttributes,
}
impl UpdateVectorStoreFileAttributesRequest {
    pub fn builder() -> builder::UpdateVectorStoreFileAttributesRequest {
        Default::default()
    }
}
#[doc = "`UpdateVectorStoreRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpdateVectorStoreRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<VectorStoreExpirationAfter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The name of the vector store."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl UpdateVectorStoreRequest {
    pub fn builder() -> builder::UpdateVectorStoreRequest {
        Default::default()
    }
}
#[doc = "The expiration policy for a vector store."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
    pub anchor: VectorStoreExpirationAfterAnchor,
    #[doc = "The number of days after the anchor time that the vector store will expire."]
    pub days: ::std::num::NonZeroU64,
}
impl VectorStoreExpirationAfter {
    pub fn builder() -> builder::VectorStoreExpirationAfter {
        Default::default()
    }
}
#[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
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
pub enum VectorStoreExpirationAfterAnchor {
    #[serde(rename = "last_active_at")]
    LastActiveAt,
}
impl ::std::fmt::Display for VectorStoreExpirationAfterAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LastActiveAt => f.write_str("last_active_at"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreExpirationAfterAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "last_active_at" => Ok(Self::LastActiveAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreExpirationAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Set of 16 key-value pairs that can be attached to an object. This can be\nuseful for storing additional information about the object in a structured\nformat, and querying for objects via API or the dashboard. Keys are strings\nwith a maximum length of 64 characters. Values are strings with a maximum\nlength of 512 characters, booleans, or numbers.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct VectorStoreFileAttributesValue(
    pub  ::std::collections::HashMap<
        VectorStoreFileAttributesValueKey,
        VectorStoreFileAttributesValueValue,
    >,
);
impl ::std::ops::Deref for VectorStoreFileAttributesValue {
    type Target = ::std::collections::HashMap<
        VectorStoreFileAttributesValueKey,
        VectorStoreFileAttributesValueValue,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<
        VectorStoreFileAttributesValueKey,
        VectorStoreFileAttributesValueValue,
    > {
        &self.0
    }
}
impl ::std::convert::From<VectorStoreFileAttributesValue>
    for ::std::collections::HashMap<
        VectorStoreFileAttributesValueKey,
        VectorStoreFileAttributesValueValue,
    >
{
    fn from(value: VectorStoreFileAttributesValue) -> Self {
        value.0
    }
}
impl
    ::std::convert::From<
        ::std::collections::HashMap<
            VectorStoreFileAttributesValueKey,
            VectorStoreFileAttributesValueValue,
        >,
    > for VectorStoreFileAttributesValue
{
    fn from(
        value: ::std::collections::HashMap<
            VectorStoreFileAttributesValueKey,
            VectorStoreFileAttributesValueValue,
        >,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`VectorStoreFileAttributesValueKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VectorStoreFileAttributesValueKey(::std::string::String);
impl ::std::ops::Deref for VectorStoreFileAttributesValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VectorStoreFileAttributesValueKey> for ::std::string::String {
    fn from(value: VectorStoreFileAttributesValueKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VectorStoreFileAttributesValueKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileAttributesValueKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileAttributesValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VectorStoreFileAttributesValueKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`VectorStoreFileAttributesValueValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorStoreFileAttributesValueValue {
    String(VectorStoreFileAttributesValueValueString),
    Number(f64),
    Boolean(bool),
}
impl ::std::fmt::Display for VectorStoreFileAttributesValueValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileAttributesValueValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileAttributesValueValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileAttributesValueValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<VectorStoreFileAttributesValueValueString>
    for VectorStoreFileAttributesValueValue
{
    fn from(value: VectorStoreFileAttributesValueValueString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for VectorStoreFileAttributesValueValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<bool> for VectorStoreFileAttributesValueValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`VectorStoreFileAttributesValueValueString`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VectorStoreFileAttributesValueValueString(::std::string::String);
impl ::std::ops::Deref for VectorStoreFileAttributesValueValueString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VectorStoreFileAttributesValueValueString> for ::std::string::String {
    fn from(value: VectorStoreFileAttributesValueValueString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VectorStoreFileAttributesValueValueString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileAttributesValueValueString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileAttributesValueValueString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VectorStoreFileAttributesValueValueString {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A batch of files attached to a vector store."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreFileBatchObject {
    #[doc = "The Unix timestamp (in seconds) for when the vector store files batch was created."]
    pub created_at: i64,
    pub file_counts: VectorStoreFileBatchObjectFileCounts,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `vector_store.file_batch`."]
    pub object: VectorStoreFileBatchObjectObject,
    #[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
    pub status: VectorStoreFileBatchObjectStatus,
    #[doc = "The ID of the [vector store](https://developers.openai.com/api/reference/resources/vector_stores) that the [File](https://developers.openai.com/api/reference/resources/files) is attached to."]
    pub vector_store_id: ::std::string::String,
}
impl VectorStoreFileBatchObject {
    pub fn builder() -> builder::VectorStoreFileBatchObject {
        Default::default()
    }
}
#[doc = "`VectorStoreFileBatchObjectFileCounts`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreFileBatchObjectFileCounts {
    #[doc = "The number of files that where cancelled."]
    pub cancelled: i64,
    #[doc = "The number of files that have been processed."]
    pub completed: i64,
    #[doc = "The number of files that have failed to process."]
    pub failed: i64,
    #[doc = "The number of files that are currently being processed."]
    pub in_progress: i64,
    #[doc = "The total number of files."]
    pub total: i64,
}
impl VectorStoreFileBatchObjectFileCounts {
    pub fn builder() -> builder::VectorStoreFileBatchObjectFileCounts {
        Default::default()
    }
}
#[doc = "The object type, which is always `vector_store.file_batch`."]
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
pub enum VectorStoreFileBatchObjectObject {
    #[serde(rename = "vector_store.file_batch")]
    VectorStoreFileBatch,
}
impl ::std::fmt::Display for VectorStoreFileBatchObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreFileBatch => f.write_str("vector_store.file_batch"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileBatchObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.file_batch" => Ok(Self::VectorStoreFileBatch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileBatchObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileBatchObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
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
pub enum VectorStoreFileBatchObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for VectorStoreFileBatchObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileBatchObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileBatchObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileBatchObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents the parsed content of a vector store file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreFileContentResponse {
    #[doc = "Parsed content of the file."]
    pub data: ::std::vec::Vec<VectorStoreFileContentResponseDataItem>,
    #[doc = "Indicates if there are more content pages to fetch."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next_page: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `vector_store.file_content.page`"]
    pub object: VectorStoreFileContentResponseObject,
}
impl VectorStoreFileContentResponse {
    pub fn builder() -> builder::VectorStoreFileContentResponse {
        Default::default()
    }
}
#[doc = "`VectorStoreFileContentResponseDataItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct VectorStoreFileContentResponseDataItem {
    #[doc = "The text content"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The content type (currently only `\"text\"`)"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl VectorStoreFileContentResponseDataItem {
    pub fn builder() -> builder::VectorStoreFileContentResponseDataItem {
        Default::default()
    }
}
#[doc = "The object type, which is always `vector_store.file_content.page`"]
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
pub enum VectorStoreFileContentResponseObject {
    #[serde(rename = "vector_store.file_content.page")]
    VectorStoreFileContentPage,
}
impl ::std::fmt::Display for VectorStoreFileContentResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreFileContentPage => f.write_str("vector_store.file_content.page"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileContentResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.file_content.page" => Ok(Self::VectorStoreFileContentPage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileContentResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileContentResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A list of files attached to a vector store."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreFileObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
    #[doc = "The strategy used to chunk the file."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<VectorStoreFileObjectChunkingStrategy>,
    #[doc = "The Unix timestamp (in seconds) for when the vector store file was created."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_error: ::std::option::Option<VectorStoreFileObjectLastError>,
    #[doc = "The object type, which is always `vector_store.file`."]
    pub object: VectorStoreFileObjectObject,
    #[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
    pub status: VectorStoreFileObjectStatus,
    #[doc = "The total vector store usage in bytes. Note that this may be different from the original file size."]
    pub usage_bytes: i64,
    #[doc = "The ID of the [vector store](https://developers.openai.com/api/reference/resources/vector_stores) that the [File](https://developers.openai.com/api/reference/resources/files) is attached to."]
    pub vector_store_id: ::std::string::String,
}
impl VectorStoreFileObject {
    pub fn builder() -> builder::VectorStoreFileObject {
        Default::default()
    }
}
#[doc = "The strategy used to chunk the file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorStoreFileObjectChunkingStrategy {
    StaticChunkingStrategyResponseParam(StaticChunkingStrategyResponseParam),
    OtherChunkingStrategyResponseParam(OtherChunkingStrategyResponseParam),
}
impl ::std::convert::From<StaticChunkingStrategyResponseParam>
    for VectorStoreFileObjectChunkingStrategy
{
    fn from(value: StaticChunkingStrategyResponseParam) -> Self {
        Self::StaticChunkingStrategyResponseParam(value)
    }
}
impl ::std::convert::From<OtherChunkingStrategyResponseParam>
    for VectorStoreFileObjectChunkingStrategy
{
    fn from(value: OtherChunkingStrategyResponseParam) -> Self {
        Self::OtherChunkingStrategyResponseParam(value)
    }
}
#[doc = "The last error associated with this vector store file. Will be `null` if there are no errors."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreFileObjectLastError {
    #[doc = "One of `server_error`, `unsupported_file`, or `invalid_file`."]
    pub code: VectorStoreFileObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: ::std::string::String,
}
impl VectorStoreFileObjectLastError {
    pub fn builder() -> builder::VectorStoreFileObjectLastError {
        Default::default()
    }
}
#[doc = "One of `server_error`, `unsupported_file`, or `invalid_file`."]
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
pub enum VectorStoreFileObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "unsupported_file")]
    UnsupportedFile,
    #[serde(rename = "invalid_file")]
    InvalidFile,
}
impl ::std::fmt::Display for VectorStoreFileObjectLastErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerError => f.write_str("server_error"),
            Self::UnsupportedFile => f.write_str("unsupported_file"),
            Self::InvalidFile => f.write_str("invalid_file"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileObjectLastErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_error" => Ok(Self::ServerError),
            "unsupported_file" => Ok(Self::UnsupportedFile),
            "invalid_file" => Ok(Self::InvalidFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `vector_store.file`."]
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
pub enum VectorStoreFileObjectObject {
    #[serde(rename = "vector_store.file")]
    VectorStoreFile,
}
impl ::std::fmt::Display for VectorStoreFileObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreFile => f.write_str("vector_store.file"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.file" => Ok(Self::VectorStoreFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
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
pub enum VectorStoreFileObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for VectorStoreFileObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreFileObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreFileObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreFileObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A vector store is a collection of processed files can be used by the `file_search` tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreObject {
    #[doc = "The Unix timestamp (in seconds) for when the vector store was created."]
    pub created_at: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<VectorStoreExpirationAfter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    pub file_counts: VectorStoreObjectFileCounts,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_active_at: ::std::option::Option<i64>,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The name of the vector store."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `vector_store`."]
    pub object: VectorStoreObjectObject,
    #[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
    pub status: VectorStoreObjectStatus,
    #[doc = "The total number of bytes used by the files in the vector store."]
    pub usage_bytes: i64,
}
impl VectorStoreObject {
    pub fn builder() -> builder::VectorStoreObject {
        Default::default()
    }
}
#[doc = "`VectorStoreObjectFileCounts`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoreObjectFileCounts {
    #[doc = "The number of files that were cancelled."]
    pub cancelled: i64,
    #[doc = "The number of files that have been successfully processed."]
    pub completed: i64,
    #[doc = "The number of files that have failed to process."]
    pub failed: i64,
    #[doc = "The number of files that are currently being processed."]
    pub in_progress: i64,
    #[doc = "The total number of files."]
    pub total: i64,
}
impl VectorStoreObjectFileCounts {
    pub fn builder() -> builder::VectorStoreObjectFileCounts {
        Default::default()
    }
}
#[doc = "The object type, which is always `vector_store`."]
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
pub enum VectorStoreObjectObject {
    #[serde(rename = "vector_store")]
    VectorStore,
}
impl ::std::fmt::Display for VectorStoreObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStore => f.write_str("vector_store"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store" => Ok(Self::VectorStore),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
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
pub enum VectorStoreObjectStatus {
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for VectorStoreObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Expired => f.write_str("expired"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "expired" => Ok(Self::Expired),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VectorStoreSearchRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VectorStoreSearchRequest {
    #[doc = "A filter to apply based on file attributes."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filters: ::std::option::Option<VectorStoreSearchRequestFilters>,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_num_results: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "A query string for a search"]
    pub query: VectorStoreSearchRequestQuery,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranking_options: ::std::option::Option<VectorStoreSearchRequestRankingOptions>,
    #[doc = "Whether to rewrite the natural language query for vector search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rewrite_query: ::std::option::Option<bool>,
}
impl VectorStoreSearchRequest {
    pub fn builder() -> builder::VectorStoreSearchRequest {
        Default::default()
    }
}
#[doc = "A filter to apply based on file attributes."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
    Variant0(::openai_api_types::common::ComparisonFilter),
    Variant1(::openai_api_types::common::CompoundFilter),
}
impl ::std::convert::From<::openai_api_types::common::ComparisonFilter>
    for VectorStoreSearchRequestFilters
{
    fn from(value: ::openai_api_types::common::ComparisonFilter) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::CompoundFilter>
    for VectorStoreSearchRequestFilters
{
    fn from(value: ::openai_api_types::common::CompoundFilter) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "A query string for a search"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestQuery {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for VectorStoreSearchRequestQuery
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "Ranking options for search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VectorStoreSearchRequestRankingOptions {
    #[doc = "Enable re-ranking; set to `none` to disable, which can help reduce latency."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranker: ::std::option::Option<VectorStoreSearchRequestRankingOptionsRanker>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_threshold: ::std::option::Option<f64>,
}
impl VectorStoreSearchRequestRankingOptions {
    pub fn builder() -> builder::VectorStoreSearchRequestRankingOptions {
        Default::default()
    }
}
#[doc = "Enable re-ranking; set to `none` to disable, which can help reduce latency."]
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
pub enum VectorStoreSearchRequestRankingOptionsRanker {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}
impl ::std::fmt::Display for VectorStoreSearchRequestRankingOptionsRanker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Default20241115 => f.write_str("default-2024-11-15"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreSearchRequestRankingOptionsRanker {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "default-2024-11-15" => Ok(Self::Default20241115),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreSearchRequestRankingOptionsRanker {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VectorStoreSearchRequestRankingOptionsRanker
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VectorStoreSearchResultContentObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VectorStoreSearchResultContentObject {
    #[doc = "The text content returned from search."]
    pub text: ::std::string::String,
    #[doc = "The type of content."]
    #[serde(rename = "type")]
    pub type_: VectorStoreSearchResultContentObjectType,
}
impl VectorStoreSearchResultContentObject {
    pub fn builder() -> builder::VectorStoreSearchResultContentObject {
        Default::default()
    }
}
#[doc = "The type of content."]
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
pub enum VectorStoreSearchResultContentObjectType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for VectorStoreSearchResultContentObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreSearchResultContentObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreSearchResultContentObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreSearchResultContentObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VectorStoreSearchResultItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VectorStoreSearchResultItem {
    pub attributes: ::openai_api_types::common::VectorStoreFileAttributes,
    #[doc = "Content chunks from the file."]
    pub content: ::std::vec::Vec<VectorStoreSearchResultContentObject>,
    #[doc = "The ID of the vector store file."]
    pub file_id: ::std::string::String,
    #[doc = "The name of the vector store file."]
    pub filename: ::std::string::String,
    #[doc = "The similarity score for the result."]
    pub score: f64,
}
impl VectorStoreSearchResultItem {
    pub fn builder() -> builder::VectorStoreSearchResultItem {
        Default::default()
    }
}
#[doc = "`VectorStoreSearchResultsPage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VectorStoreSearchResultsPage {
    #[doc = "The list of search result items."]
    pub data: ::std::vec::Vec<VectorStoreSearchResultItem>,
    #[doc = "Indicates if there are more results to fetch."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next_page: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `vector_store.search_results.page`"]
    pub object: VectorStoreSearchResultsPageObject,
    pub search_query: ::std::vec::Vec<::std::string::String>,
}
impl VectorStoreSearchResultsPage {
    pub fn builder() -> builder::VectorStoreSearchResultsPage {
        Default::default()
    }
}
#[doc = "The object type, which is always `vector_store.search_results.page`"]
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
pub enum VectorStoreSearchResultsPageObject {
    #[serde(rename = "vector_store.search_results.page")]
    VectorStoreSearchResultsPage,
}
impl ::std::fmt::Display for VectorStoreSearchResultsPageObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VectorStoreSearchResultsPage => f.write_str("vector_store.search_results.page"),
        }
    }
}
impl ::std::str::FromStr for VectorStoreSearchResultsPageObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vector_store.search_results.page" => Ok(Self::VectorStoreSearchResultsPage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VectorStoreSearchResultsPageObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VectorStoreSearchResultsPageObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VectorStoresApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VectorStoresApiSchema {
    #[serde(rename = "CancelVectorStoreFileBatchError400")]
    pub cancel_vector_store_file_batch_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelVectorStoreFileBatchError404")]
    pub cancel_vector_store_file_batch_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelVectorStoreFileBatchError409")]
    pub cancel_vector_store_file_batch_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelVectorStoreFileBatchError429")]
    pub cancel_vector_store_file_batch_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelVectorStoreFileBatchParams")]
    pub cancel_vector_store_file_batch_params: CancelVectorStoreFileBatchParams,
    #[serde(rename = "CancelVectorStoreFileBatchResponse")]
    pub cancel_vector_store_file_batch_response: VectorStoreFileBatchObject,
    #[serde(rename = "CreateVectorStoreError400")]
    pub create_vector_store_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreError404")]
    pub create_vector_store_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreError429")]
    pub create_vector_store_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchError400")]
    pub create_vector_store_file_batch_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchError404")]
    pub create_vector_store_file_batch_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchError409")]
    pub create_vector_store_file_batch_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchError429")]
    pub create_vector_store_file_batch_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchError500")]
    pub create_vector_store_file_batch_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileBatchParams")]
    pub create_vector_store_file_batch_params: CreateVectorStoreFileBatchParams,
    #[serde(rename = "CreateVectorStoreFileBatchRequest")]
    pub create_vector_store_file_batch_request: CreateVectorStoreFileBatchRequest,
    #[serde(rename = "CreateVectorStoreFileBatchResponse")]
    pub create_vector_store_file_batch_response: VectorStoreFileBatchObject,
    #[serde(rename = "CreateVectorStoreFileError400")]
    pub create_vector_store_file_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileError404")]
    pub create_vector_store_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileError409")]
    pub create_vector_store_file_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileError429")]
    pub create_vector_store_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileError500")]
    pub create_vector_store_file_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVectorStoreFileParams")]
    pub create_vector_store_file_params: CreateVectorStoreFileParams,
    #[serde(rename = "CreateVectorStoreFileRequest")]
    pub create_vector_store_file_request: CreateVectorStoreFileRequest,
    #[serde(rename = "CreateVectorStoreFileResponse")]
    pub create_vector_store_file_response: VectorStoreFileObject,
    #[serde(rename = "CreateVectorStoreRequest")]
    pub create_vector_store_request: CreateVectorStoreRequest,
    #[serde(rename = "CreateVectorStoreResponse")]
    pub create_vector_store_response: VectorStoreObject,
    #[serde(rename = "DeleteVectorStoreError404")]
    pub delete_vector_store_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreError409")]
    pub delete_vector_store_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreError429")]
    pub delete_vector_store_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileError400")]
    pub delete_vector_store_file_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileError404")]
    pub delete_vector_store_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileError409")]
    pub delete_vector_store_file_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileError429")]
    pub delete_vector_store_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileError500")]
    pub delete_vector_store_file_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVectorStoreFileParams")]
    pub delete_vector_store_file_params: DeleteVectorStoreFileParams,
    #[serde(rename = "DeleteVectorStoreFileResponse")]
    pub delete_vector_store_file_response: DeleteVectorStoreFileResponse,
    #[serde(rename = "DeleteVectorStoreParams")]
    pub delete_vector_store_params: DeleteVectorStoreParams,
    #[serde(rename = "DeleteVectorStoreResponse")]
    pub delete_vector_store_response: DeleteVectorStoreResponse,
    #[serde(rename = "GetVectorStoreError404")]
    pub get_vector_store_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreError429")]
    pub get_vector_store_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileBatchError400")]
    pub get_vector_store_file_batch_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileBatchError404")]
    pub get_vector_store_file_batch_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileBatchError429")]
    pub get_vector_store_file_batch_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileBatchParams")]
    pub get_vector_store_file_batch_params: GetVectorStoreFileBatchParams,
    #[serde(rename = "GetVectorStoreFileBatchResponse")]
    pub get_vector_store_file_batch_response: VectorStoreFileBatchObject,
    #[serde(rename = "GetVectorStoreFileError404")]
    pub get_vector_store_file_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileError429")]
    pub get_vector_store_file_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileError500")]
    pub get_vector_store_file_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVectorStoreFileParams")]
    pub get_vector_store_file_params: GetVectorStoreFileParams,
    #[serde(rename = "GetVectorStoreFileResponse")]
    pub get_vector_store_file_response: VectorStoreFileObject,
    #[serde(rename = "GetVectorStoreParams")]
    pub get_vector_store_params: GetVectorStoreParams,
    #[serde(rename = "GetVectorStoreResponse")]
    pub get_vector_store_response: VectorStoreObject,
    #[serde(rename = "ListFilesInVectorStoreBatchError400")]
    pub list_files_in_vector_store_batch_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesInVectorStoreBatchError404")]
    pub list_files_in_vector_store_batch_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesInVectorStoreBatchError429")]
    pub list_files_in_vector_store_batch_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListFilesInVectorStoreBatchParams")]
    pub list_files_in_vector_store_batch_params: ListFilesInVectorStoreBatchParams,
    #[serde(rename = "ListFilesInVectorStoreBatchResponse")]
    pub list_files_in_vector_store_batch_response: ListVectorStoreFilesResponse,
    #[serde(rename = "ListVectorStoreFilesError400")]
    pub list_vector_store_files_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVectorStoreFilesError404")]
    pub list_vector_store_files_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVectorStoreFilesError429")]
    pub list_vector_store_files_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVectorStoreFilesParams")]
    pub list_vector_store_files_params: ListVectorStoreFilesParams,
    #[serde(rename = "ListVectorStoreFilesResponse")]
    pub list_vector_store_files_response: ListVectorStoreFilesResponse,
    #[serde(rename = "ListVectorStoresError429")]
    pub list_vector_stores_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVectorStoresError503")]
    pub list_vector_stores_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVectorStoresParams")]
    pub list_vector_stores_params: ListVectorStoresParams,
    #[serde(rename = "ListVectorStoresResponse")]
    pub list_vector_stores_response: ListVectorStoresResponse,
    #[serde(rename = "ModifyVectorStoreError400")]
    pub modify_vector_store_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyVectorStoreError403")]
    pub modify_vector_store_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyVectorStoreError404")]
    pub modify_vector_store_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyVectorStoreError429")]
    pub modify_vector_store_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyVectorStoreParams")]
    pub modify_vector_store_params: ModifyVectorStoreParams,
    #[serde(rename = "ModifyVectorStoreRequest")]
    pub modify_vector_store_request: UpdateVectorStoreRequest,
    #[serde(rename = "ModifyVectorStoreResponse")]
    pub modify_vector_store_response: VectorStoreObject,
    #[serde(rename = "RetrieveVectorStoreFileContentError400")]
    pub retrieve_vector_store_file_content_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVectorStoreFileContentError404")]
    pub retrieve_vector_store_file_content_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVectorStoreFileContentError429")]
    pub retrieve_vector_store_file_content_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVectorStoreFileContentError500")]
    pub retrieve_vector_store_file_content_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVectorStoreFileContentParams")]
    pub retrieve_vector_store_file_content_params: RetrieveVectorStoreFileContentParams,
    #[serde(rename = "RetrieveVectorStoreFileContentResponse")]
    pub retrieve_vector_store_file_content_response: VectorStoreFileContentResponse,
    #[serde(rename = "SearchVectorStoreError400")]
    pub search_vector_store_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "SearchVectorStoreError404")]
    pub search_vector_store_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "SearchVectorStoreError429")]
    pub search_vector_store_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "SearchVectorStoreParams")]
    pub search_vector_store_params: SearchVectorStoreParams,
    #[serde(rename = "SearchVectorStoreRequest")]
    pub search_vector_store_request: VectorStoreSearchRequest,
    #[serde(rename = "SearchVectorStoreResponse")]
    pub search_vector_store_response: VectorStoreSearchResultsPage,
    #[serde(rename = "UpdateVectorStoreFileAttributesError400")]
    pub update_vector_store_file_attributes_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVectorStoreFileAttributesError404")]
    pub update_vector_store_file_attributes_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVectorStoreFileAttributesError409")]
    pub update_vector_store_file_attributes_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVectorStoreFileAttributesError429")]
    pub update_vector_store_file_attributes_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVectorStoreFileAttributesError500")]
    pub update_vector_store_file_attributes_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateVectorStoreFileAttributesParams")]
    pub update_vector_store_file_attributes_params: UpdateVectorStoreFileAttributesParams,
    #[serde(rename = "UpdateVectorStoreFileAttributesRequest")]
    pub update_vector_store_file_attributes_request: UpdateVectorStoreFileAttributesRequest,
    #[serde(rename = "UpdateVectorStoreFileAttributesResponse")]
    pub update_vector_store_file_attributes_response: VectorStoreFileObject,
}
impl VectorStoresApiSchema {
    pub fn builder() -> builder::VectorStoresApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AutoChunkingStrategyRequestParam {
        type_: ::std::result::Result<
            super::AutoChunkingStrategyRequestParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AutoChunkingStrategyRequestParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AutoChunkingStrategyRequestParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AutoChunkingStrategyRequestParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AutoChunkingStrategyRequestParam>
        for super::AutoChunkingStrategyRequestParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AutoChunkingStrategyRequestParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AutoChunkingStrategyRequestParam>
        for AutoChunkingStrategyRequestParam
    {
        fn from(value: super::AutoChunkingStrategyRequestParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelVectorStoreFileBatchParams {
        batch_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelVectorStoreFileBatchParams {
        fn default() -> Self {
            Self {
                batch_id: Err("no value supplied for batch_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl CancelVectorStoreFileBatchParams {
        pub fn batch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelVectorStoreFileBatchParams>
        for super::CancelVectorStoreFileBatchParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelVectorStoreFileBatchParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_id: value.batch_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelVectorStoreFileBatchParams>
        for CancelVectorStoreFileBatchParams
    {
        fn from(value: super::CancelVectorStoreFileBatchParams) -> Self {
            Self {
                batch_id: Ok(value.batch_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVectorStoreFileBatchParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVectorStoreFileBatchParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl CreateVectorStoreFileBatchParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVectorStoreFileBatchParams>
        for super::CreateVectorStoreFileBatchParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVectorStoreFileBatchParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVectorStoreFileBatchParams>
        for CreateVectorStoreFileBatchParams
    {
        fn from(value: super::CreateVectorStoreFileBatchParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVectorStoreFileParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVectorStoreFileParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl CreateVectorStoreFileParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVectorStoreFileParams> for super::CreateVectorStoreFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVectorStoreFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVectorStoreFileParams> for CreateVectorStoreFileParams {
        fn from(value: super::CreateVectorStoreFileParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVectorStoreFileRequest {
        attributes: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
            ::std::string::String,
        >,
        chunking_strategy: ::std::result::Result<
            ::std::option::Option<super::ChunkingStrategyRequestParam>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVectorStoreFileRequest {
        fn default() -> Self {
            Self {
                attributes: Ok(Default::default()),
                chunking_strategy: Ok(Default::default()),
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl CreateVectorStoreFileRequest {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {e}"));
            self
        }
        pub fn chunking_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChunkingStrategyRequestParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
            self
        }
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVectorStoreFileRequest> for super::CreateVectorStoreFileRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVectorStoreFileRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attributes: value.attributes?,
                chunking_strategy: value.chunking_strategy?,
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVectorStoreFileRequest> for CreateVectorStoreFileRequest {
        fn from(value: super::CreateVectorStoreFileRequest) -> Self {
            Self {
                attributes: Ok(value.attributes),
                chunking_strategy: Ok(value.chunking_strategy),
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVectorStoreRequest {
        chunking_strategy: ::std::result::Result<
            ::std::option::Option<super::CreateVectorStoreRequestChunkingStrategy>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        expires_after: ::std::result::Result<
            ::std::option::Option<super::VectorStoreExpirationAfter>,
            ::std::string::String,
        >,
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVectorStoreRequest {
        fn default() -> Self {
            Self {
                chunking_strategy: Ok(Default::default()),
                description: Ok(Default::default()),
                expires_after: Ok(Default::default()),
                file_ids: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl CreateVectorStoreRequest {
        pub fn chunking_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateVectorStoreRequestChunkingStrategy>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VectorStoreExpirationAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
            self
        }
        pub fn file_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_ids: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::openai_api_types::common::Metadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVectorStoreRequest> for super::CreateVectorStoreRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVectorStoreRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chunking_strategy: value.chunking_strategy?,
                description: value.description?,
                expires_after: value.expires_after?,
                file_ids: value.file_ids?,
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVectorStoreRequest> for CreateVectorStoreRequest {
        fn from(value: super::CreateVectorStoreRequest) -> Self {
            Self {
                chunking_strategy: Ok(value.chunking_strategy),
                description: Ok(value.description),
                expires_after: Ok(value.expires_after),
                file_ids: Ok(value.file_ids),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVectorStoreFileParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVectorStoreFileParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl DeleteVectorStoreFileParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVectorStoreFileParams> for super::DeleteVectorStoreFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVectorStoreFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVectorStoreFileParams> for DeleteVectorStoreFileParams {
        fn from(value: super::DeleteVectorStoreFileParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVectorStoreFileResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::DeleteVectorStoreFileResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteVectorStoreFileResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteVectorStoreFileResponse {
        pub fn deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreFileResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVectorStoreFileResponse>
        for super::DeleteVectorStoreFileResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVectorStoreFileResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVectorStoreFileResponse> for DeleteVectorStoreFileResponse {
        fn from(value: super::DeleteVectorStoreFileResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVectorStoreParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVectorStoreParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl DeleteVectorStoreParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVectorStoreParams> for super::DeleteVectorStoreParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVectorStoreParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVectorStoreParams> for DeleteVectorStoreParams {
        fn from(value: super::DeleteVectorStoreParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVectorStoreResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::DeleteVectorStoreResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVectorStoreResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteVectorStoreResponse {
        pub fn deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVectorStoreResponse> for super::DeleteVectorStoreResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVectorStoreResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVectorStoreResponse> for DeleteVectorStoreResponse {
        fn from(value: super::DeleteVectorStoreResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVectorStoreFileBatchParams {
        batch_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVectorStoreFileBatchParams {
        fn default() -> Self {
            Self {
                batch_id: Err("no value supplied for batch_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl GetVectorStoreFileBatchParams {
        pub fn batch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVectorStoreFileBatchParams>
        for super::GetVectorStoreFileBatchParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVectorStoreFileBatchParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_id: value.batch_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVectorStoreFileBatchParams> for GetVectorStoreFileBatchParams {
        fn from(value: super::GetVectorStoreFileBatchParams) -> Self {
            Self {
                batch_id: Ok(value.batch_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVectorStoreFileParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVectorStoreFileParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl GetVectorStoreFileParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVectorStoreFileParams> for super::GetVectorStoreFileParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVectorStoreFileParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVectorStoreFileParams> for GetVectorStoreFileParams {
        fn from(value: super::GetVectorStoreFileParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVectorStoreParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVectorStoreParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl GetVectorStoreParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVectorStoreParams> for super::GetVectorStoreParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVectorStoreParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVectorStoreParams> for GetVectorStoreParams {
        fn from(value: super::GetVectorStoreParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFilesInVectorStoreBatchParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        batch_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        filter: ::std::result::Result<
            ::std::option::Option<super::ListFilesInVectorStoreBatchParamsFilter>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListFilesInVectorStoreBatchParamsOrder>,
            ::std::string::String,
        >,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListFilesInVectorStoreBatchParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                batch_id: Err("no value supplied for batch_id".to_string()),
                before: Ok(Default::default()),
                filter: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl ListFilesInVectorStoreBatchParams {
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
        pub fn batch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_id: {e}"));
            self
        }
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {e}"));
            self
        }
        pub fn filter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListFilesInVectorStoreBatchParamsFilter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.filter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter: {e}"));
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListFilesInVectorStoreBatchParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFilesInVectorStoreBatchParams>
        for super::ListFilesInVectorStoreBatchParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFilesInVectorStoreBatchParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                batch_id: value.batch_id?,
                before: value.before?,
                filter: value.filter?,
                limit: value.limit?,
                order: value.order?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListFilesInVectorStoreBatchParams>
        for ListFilesInVectorStoreBatchParams
    {
        fn from(value: super::ListFilesInVectorStoreBatchParams) -> Self {
            Self {
                after: Ok(value.after),
                batch_id: Ok(value.batch_id),
                before: Ok(value.before),
                filter: Ok(value.filter),
                limit: Ok(value.limit),
                order: Ok(value.order),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVectorStoreFilesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        filter: ::std::result::Result<
            ::std::option::Option<super::ListVectorStoreFilesParamsFilter>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListVectorStoreFilesParamsOrder>,
            ::std::string::String,
        >,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListVectorStoreFilesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                filter: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl ListVectorStoreFilesParams {
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
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {e}"));
            self
        }
        pub fn filter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListVectorStoreFilesParamsFilter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.filter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter: {e}"));
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListVectorStoreFilesParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVectorStoreFilesParams> for super::ListVectorStoreFilesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVectorStoreFilesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                filter: value.filter?,
                limit: value.limit?,
                order: value.order?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListVectorStoreFilesParams> for ListVectorStoreFilesParams {
        fn from(value: super::ListVectorStoreFilesParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                filter: Ok(value.filter),
                limit: Ok(value.limit),
                order: Ok(value.order),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVectorStoreFilesResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::VectorStoreFileObject>,
            ::std::string::String,
        >,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListVectorStoreFilesResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Err("no value supplied for first_id".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Err("no value supplied for last_id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListVectorStoreFilesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VectorStoreFileObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn first_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.last_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVectorStoreFilesResponse> for super::ListVectorStoreFilesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVectorStoreFilesResponse,
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
    impl ::std::convert::From<super::ListVectorStoreFilesResponse> for ListVectorStoreFilesResponse {
        fn from(value: super::ListVectorStoreFilesResponse) -> Self {
            Self {
                data: Ok(value.data),
                first_id: Ok(value.first_id),
                has_more: Ok(value.has_more),
                last_id: Ok(value.last_id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVectorStoresParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListVectorStoresParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListVectorStoresParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListVectorStoresParams {
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
        pub fn before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for before: {e}"));
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListVectorStoresParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVectorStoresParams> for super::ListVectorStoresParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVectorStoresParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListVectorStoresParams> for ListVectorStoresParams {
        fn from(value: super::ListVectorStoresParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVectorStoresResponse {
        data:
            ::std::result::Result<::std::vec::Vec<super::VectorStoreObject>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListVectorStoresResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Err("no value supplied for first_id".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Err("no value supplied for last_id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListVectorStoresResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VectorStoreObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn first_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.last_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVectorStoresResponse> for super::ListVectorStoresResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVectorStoresResponse,
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
    impl ::std::convert::From<super::ListVectorStoresResponse> for ListVectorStoresResponse {
        fn from(value: super::ListVectorStoresResponse) -> Self {
            Self {
                data: Ok(value.data),
                first_id: Ok(value.first_id),
                has_more: Ok(value.has_more),
                last_id: Ok(value.last_id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyVectorStoreParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyVectorStoreParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl ModifyVectorStoreParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyVectorStoreParams> for super::ModifyVectorStoreParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyVectorStoreParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyVectorStoreParams> for ModifyVectorStoreParams {
        fn from(value: super::ModifyVectorStoreParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OtherChunkingStrategyResponseParam {
        type_: ::std::result::Result<
            super::OtherChunkingStrategyResponseParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OtherChunkingStrategyResponseParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OtherChunkingStrategyResponseParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OtherChunkingStrategyResponseParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OtherChunkingStrategyResponseParam>
        for super::OtherChunkingStrategyResponseParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OtherChunkingStrategyResponseParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OtherChunkingStrategyResponseParam>
        for OtherChunkingStrategyResponseParam
    {
        fn from(value: super::OtherChunkingStrategyResponseParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveVectorStoreFileContentParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveVectorStoreFileContentParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl RetrieveVectorStoreFileContentParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveVectorStoreFileContentParams>
        for super::RetrieveVectorStoreFileContentParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveVectorStoreFileContentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveVectorStoreFileContentParams>
        for RetrieveVectorStoreFileContentParams
    {
        fn from(value: super::RetrieveVectorStoreFileContentParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SearchVectorStoreParams {
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SearchVectorStoreParams {
        fn default() -> Self {
            Self {
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl SearchVectorStoreParams {
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SearchVectorStoreParams> for super::SearchVectorStoreParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SearchVectorStoreParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::SearchVectorStoreParams> for SearchVectorStoreParams {
        fn from(value: super::SearchVectorStoreParams) -> Self {
            Self {
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StaticChunkingStrategy {
        chunk_overlap_tokens: ::std::result::Result<i64, ::std::string::String>,
        max_chunk_size_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for StaticChunkingStrategy {
        fn default() -> Self {
            Self {
                chunk_overlap_tokens: Err("no value supplied for chunk_overlap_tokens".to_string()),
                max_chunk_size_tokens: Err(
                    "no value supplied for max_chunk_size_tokens".to_string()
                ),
            }
        }
    }
    impl StaticChunkingStrategy {
        pub fn chunk_overlap_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.chunk_overlap_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for chunk_overlap_tokens: {e}")
            });
            self
        }
        pub fn max_chunk_size_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_chunk_size_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_chunk_size_tokens: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<StaticChunkingStrategy> for super::StaticChunkingStrategy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StaticChunkingStrategy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chunk_overlap_tokens: value.chunk_overlap_tokens?,
                max_chunk_size_tokens: value.max_chunk_size_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::StaticChunkingStrategy> for StaticChunkingStrategy {
        fn from(value: super::StaticChunkingStrategy) -> Self {
            Self {
                chunk_overlap_tokens: Ok(value.chunk_overlap_tokens),
                max_chunk_size_tokens: Ok(value.max_chunk_size_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StaticChunkingStrategyRequestParam {
        static_: ::std::result::Result<super::StaticChunkingStrategy, ::std::string::String>,
        type_: ::std::result::Result<
            super::StaticChunkingStrategyRequestParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StaticChunkingStrategyRequestParam {
        fn default() -> Self {
            Self {
                static_: Err("no value supplied for static_".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StaticChunkingStrategyRequestParam {
        pub fn static_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaticChunkingStrategy>,
            T::Error: ::std::fmt::Display,
        {
            self.static_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for static_: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaticChunkingStrategyRequestParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StaticChunkingStrategyRequestParam>
        for super::StaticChunkingStrategyRequestParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StaticChunkingStrategyRequestParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                static_: value.static_?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StaticChunkingStrategyRequestParam>
        for StaticChunkingStrategyRequestParam
    {
        fn from(value: super::StaticChunkingStrategyRequestParam) -> Self {
            Self {
                static_: Ok(value.static_),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StaticChunkingStrategyResponseParam {
        static_: ::std::result::Result<super::StaticChunkingStrategy, ::std::string::String>,
        type_: ::std::result::Result<
            super::StaticChunkingStrategyResponseParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StaticChunkingStrategyResponseParam {
        fn default() -> Self {
            Self {
                static_: Err("no value supplied for static_".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl StaticChunkingStrategyResponseParam {
        pub fn static_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaticChunkingStrategy>,
            T::Error: ::std::fmt::Display,
        {
            self.static_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for static_: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaticChunkingStrategyResponseParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StaticChunkingStrategyResponseParam>
        for super::StaticChunkingStrategyResponseParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StaticChunkingStrategyResponseParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                static_: value.static_?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::StaticChunkingStrategyResponseParam>
        for StaticChunkingStrategyResponseParam
    {
        fn from(value: super::StaticChunkingStrategyResponseParam) -> Self {
            Self {
                static_: Ok(value.static_),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVectorStoreFileAttributesParams {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateVectorStoreFileAttributesParams {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl UpdateVectorStoreFileAttributesParams {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVectorStoreFileAttributesParams>
        for super::UpdateVectorStoreFileAttributesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVectorStoreFileAttributesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateVectorStoreFileAttributesParams>
        for UpdateVectorStoreFileAttributesParams
    {
        fn from(value: super::UpdateVectorStoreFileAttributesParams) -> Self {
            Self {
                file_id: Ok(value.file_id),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVectorStoreFileAttributesRequest {
        attributes: ::std::result::Result<
            ::openai_api_types::common::VectorStoreFileAttributes,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateVectorStoreFileAttributesRequest {
        fn default() -> Self {
            Self {
                attributes: Err("no value supplied for attributes".to_string()),
            }
        }
    }
    impl UpdateVectorStoreFileAttributesRequest {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::VectorStoreFileAttributes>,
            T::Error: ::std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVectorStoreFileAttributesRequest>
        for super::UpdateVectorStoreFileAttributesRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVectorStoreFileAttributesRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attributes: value.attributes?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateVectorStoreFileAttributesRequest>
        for UpdateVectorStoreFileAttributesRequest
    {
        fn from(value: super::UpdateVectorStoreFileAttributesRequest) -> Self {
            Self {
                attributes: Ok(value.attributes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVectorStoreRequest {
        expires_after: ::std::result::Result<
            ::std::option::Option<super::VectorStoreExpirationAfter>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateVectorStoreRequest {
        fn default() -> Self {
            Self {
                expires_after: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl UpdateVectorStoreRequest {
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VectorStoreExpirationAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::openai_api_types::common::Metadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVectorStoreRequest> for super::UpdateVectorStoreRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVectorStoreRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_after: value.expires_after?,
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateVectorStoreRequest> for UpdateVectorStoreRequest {
        fn from(value: super::UpdateVectorStoreRequest) -> Self {
            Self {
                expires_after: Ok(value.expires_after),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreExpirationAfter {
        anchor:
            ::std::result::Result<super::VectorStoreExpirationAfterAnchor, ::std::string::String>,
        days: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreExpirationAfter {
        fn default() -> Self {
            Self {
                anchor: Err("no value supplied for anchor".to_string()),
                days: Err("no value supplied for days".to_string()),
            }
        }
    }
    impl VectorStoreExpirationAfter {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreExpirationAfterAnchor>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
            self
        }
        pub fn days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.days = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for days: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreExpirationAfter> for super::VectorStoreExpirationAfter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreExpirationAfter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                days: value.days?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreExpirationAfter> for VectorStoreExpirationAfter {
        fn from(value: super::VectorStoreExpirationAfter) -> Self {
            Self {
                anchor: Ok(value.anchor),
                days: Ok(value.days),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileBatchObject {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        file_counts: ::std::result::Result<
            super::VectorStoreFileBatchObjectFileCounts,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::VectorStoreFileBatchObjectObject, ::std::string::String>,
        status:
            ::std::result::Result<super::VectorStoreFileBatchObjectStatus, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreFileBatchObject {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                file_counts: Err("no value supplied for file_counts".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                status: Err("no value supplied for status".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl VectorStoreFileBatchObject {
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
        pub fn file_counts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObjectFileCounts>,
            T::Error: ::std::fmt::Display,
        {
            self.file_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_counts: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreFileBatchObject> for super::VectorStoreFileBatchObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileBatchObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                file_counts: value.file_counts?,
                id: value.id?,
                object: value.object?,
                status: value.status?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileBatchObject> for VectorStoreFileBatchObject {
        fn from(value: super::VectorStoreFileBatchObject) -> Self {
            Self {
                created_at: Ok(value.created_at),
                file_counts: Ok(value.file_counts),
                id: Ok(value.id),
                object: Ok(value.object),
                status: Ok(value.status),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileBatchObjectFileCounts {
        cancelled: ::std::result::Result<i64, ::std::string::String>,
        completed: ::std::result::Result<i64, ::std::string::String>,
        failed: ::std::result::Result<i64, ::std::string::String>,
        in_progress: ::std::result::Result<i64, ::std::string::String>,
        total: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreFileBatchObjectFileCounts {
        fn default() -> Self {
            Self {
                cancelled: Err("no value supplied for cancelled".to_string()),
                completed: Err("no value supplied for completed".to_string()),
                failed: Err("no value supplied for failed".to_string()),
                in_progress: Err("no value supplied for in_progress".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl VectorStoreFileBatchObjectFileCounts {
        pub fn cancelled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cancelled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cancelled: {e}"));
            self
        }
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
        pub fn in_progress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.in_progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_progress: {e}"));
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
    impl ::std::convert::TryFrom<VectorStoreFileBatchObjectFileCounts>
        for super::VectorStoreFileBatchObjectFileCounts
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileBatchObjectFileCounts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancelled: value.cancelled?,
                completed: value.completed?,
                failed: value.failed?,
                in_progress: value.in_progress?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileBatchObjectFileCounts>
        for VectorStoreFileBatchObjectFileCounts
    {
        fn from(value: super::VectorStoreFileBatchObjectFileCounts) -> Self {
            Self {
                cancelled: Ok(value.cancelled),
                completed: Ok(value.completed),
                failed: Ok(value.failed),
                in_progress: Ok(value.in_progress),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileContentResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::VectorStoreFileContentResponseDataItem>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next_page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::VectorStoreFileContentResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VectorStoreFileContentResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next_page: Err("no value supplied for next_page".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl VectorStoreFileContentResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::VectorStoreFileContentResponseDataItem>,
            >,
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
            T: ::std::convert::TryInto<super::VectorStoreFileContentResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreFileContentResponse>
        for super::VectorStoreFileContentResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileContentResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next_page: value.next_page?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileContentResponse>
        for VectorStoreFileContentResponse
    {
        fn from(value: super::VectorStoreFileContentResponse) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next_page: Ok(value.next_page),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileContentResponseDataItem {
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VectorStoreFileContentResponseDataItem {
        fn default() -> Self {
            Self {
                text: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl VectorStoreFileContentResponseDataItem {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreFileContentResponseDataItem>
        for super::VectorStoreFileContentResponseDataItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileContentResponseDataItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileContentResponseDataItem>
        for VectorStoreFileContentResponseDataItem
    {
        fn from(value: super::VectorStoreFileContentResponseDataItem) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileObject {
        attributes: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
            ::std::string::String,
        >,
        chunking_strategy: ::std::result::Result<
            ::std::option::Option<super::VectorStoreFileObjectChunkingStrategy>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_error: ::std::result::Result<
            ::std::option::Option<super::VectorStoreFileObjectLastError>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::VectorStoreFileObjectObject, ::std::string::String>,
        status: ::std::result::Result<super::VectorStoreFileObjectStatus, ::std::string::String>,
        usage_bytes: ::std::result::Result<i64, ::std::string::String>,
        vector_store_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreFileObject {
        fn default() -> Self {
            Self {
                attributes: Ok(Default::default()),
                chunking_strategy: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                last_error: Err("no value supplied for last_error".to_string()),
                object: Err("no value supplied for object".to_string()),
                status: Err("no value supplied for status".to_string()),
                usage_bytes: Err("no value supplied for usage_bytes".to_string()),
                vector_store_id: Err("no value supplied for vector_store_id".to_string()),
            }
        }
    }
    impl VectorStoreFileObject {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::VectorStoreFileAttributes>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {e}"));
            self
        }
        pub fn chunking_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VectorStoreFileObjectChunkingStrategy>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
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
        pub fn last_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VectorStoreFileObjectLastError>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_error: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
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
        pub fn vector_store_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vector_store_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vector_store_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreFileObject> for super::VectorStoreFileObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attributes: value.attributes?,
                chunking_strategy: value.chunking_strategy?,
                created_at: value.created_at?,
                id: value.id?,
                last_error: value.last_error?,
                object: value.object?,
                status: value.status?,
                usage_bytes: value.usage_bytes?,
                vector_store_id: value.vector_store_id?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileObject> for VectorStoreFileObject {
        fn from(value: super::VectorStoreFileObject) -> Self {
            Self {
                attributes: Ok(value.attributes),
                chunking_strategy: Ok(value.chunking_strategy),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                last_error: Ok(value.last_error),
                object: Ok(value.object),
                status: Ok(value.status),
                usage_bytes: Ok(value.usage_bytes),
                vector_store_id: Ok(value.vector_store_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreFileObjectLastError {
        code:
            ::std::result::Result<super::VectorStoreFileObjectLastErrorCode, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreFileObjectLastError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl VectorStoreFileObjectLastError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObjectLastErrorCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreFileObjectLastError>
        for super::VectorStoreFileObjectLastError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreFileObjectLastError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreFileObjectLastError>
        for VectorStoreFileObjectLastError
    {
        fn from(value: super::VectorStoreFileObjectLastError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreObject {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_after: ::std::result::Result<
            ::std::option::Option<super::VectorStoreExpirationAfter>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        file_counts:
            ::std::result::Result<super::VectorStoreObjectFileCounts, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_active_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::VectorStoreObjectObject, ::std::string::String>,
        status: ::std::result::Result<super::VectorStoreObjectStatus, ::std::string::String>,
        usage_bytes: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreObject {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_after: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                file_counts: Err("no value supplied for file_counts".to_string()),
                id: Err("no value supplied for id".to_string()),
                last_active_at: Err("no value supplied for last_active_at".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                status: Err("no value supplied for status".to_string()),
                usage_bytes: Err("no value supplied for usage_bytes".to_string()),
            }
        }
    }
    impl VectorStoreObject {
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
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VectorStoreExpirationAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
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
        pub fn file_counts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObjectFileCounts>,
            T::Error: ::std::fmt::Display,
        {
            self.file_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_counts: {e}"));
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
        pub fn last_active_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_active_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_active_at: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Metadata>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
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
    impl ::std::convert::TryFrom<VectorStoreObject> for super::VectorStoreObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_after: value.expires_after?,
                expires_at: value.expires_at?,
                file_counts: value.file_counts?,
                id: value.id?,
                last_active_at: value.last_active_at?,
                metadata: value.metadata?,
                name: value.name?,
                object: value.object?,
                status: value.status?,
                usage_bytes: value.usage_bytes?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreObject> for VectorStoreObject {
        fn from(value: super::VectorStoreObject) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_after: Ok(value.expires_after),
                expires_at: Ok(value.expires_at),
                file_counts: Ok(value.file_counts),
                id: Ok(value.id),
                last_active_at: Ok(value.last_active_at),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                object: Ok(value.object),
                status: Ok(value.status),
                usage_bytes: Ok(value.usage_bytes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreObjectFileCounts {
        cancelled: ::std::result::Result<i64, ::std::string::String>,
        completed: ::std::result::Result<i64, ::std::string::String>,
        failed: ::std::result::Result<i64, ::std::string::String>,
        in_progress: ::std::result::Result<i64, ::std::string::String>,
        total: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreObjectFileCounts {
        fn default() -> Self {
            Self {
                cancelled: Err("no value supplied for cancelled".to_string()),
                completed: Err("no value supplied for completed".to_string()),
                failed: Err("no value supplied for failed".to_string()),
                in_progress: Err("no value supplied for in_progress".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl VectorStoreObjectFileCounts {
        pub fn cancelled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cancelled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cancelled: {e}"));
            self
        }
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
        pub fn in_progress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.in_progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_progress: {e}"));
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
    impl ::std::convert::TryFrom<VectorStoreObjectFileCounts> for super::VectorStoreObjectFileCounts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreObjectFileCounts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancelled: value.cancelled?,
                completed: value.completed?,
                failed: value.failed?,
                in_progress: value.in_progress?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreObjectFileCounts> for VectorStoreObjectFileCounts {
        fn from(value: super::VectorStoreObjectFileCounts) -> Self {
            Self {
                cancelled: Ok(value.cancelled),
                completed: Ok(value.completed),
                failed: Ok(value.failed),
                in_progress: Ok(value.in_progress),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreSearchRequest {
        filters: ::std::result::Result<
            ::std::option::Option<super::VectorStoreSearchRequestFilters>,
            ::std::string::String,
        >,
        max_num_results: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        query: ::std::result::Result<super::VectorStoreSearchRequestQuery, ::std::string::String>,
        ranking_options: ::std::result::Result<
            ::std::option::Option<super::VectorStoreSearchRequestRankingOptions>,
            ::std::string::String,
        >,
        rewrite_query: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreSearchRequest {
        fn default() -> Self {
            Self {
                filters: Ok(Default::default()),
                max_num_results: Ok(Default::default()),
                query: Err("no value supplied for query".to_string()),
                ranking_options: Ok(Default::default()),
                rewrite_query: Ok(Default::default()),
            }
        }
    }
    impl VectorStoreSearchRequest {
        pub fn filters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VectorStoreSearchRequestFilters>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.filters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filters: {e}"));
            self
        }
        pub fn max_num_results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_num_results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_num_results: {e}"));
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreSearchRequestQuery>,
            T::Error: ::std::fmt::Display,
        {
            self.query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for query: {e}"));
            self
        }
        pub fn ranking_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VectorStoreSearchRequestRankingOptions>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ranking_options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranking_options: {e}"));
            self
        }
        pub fn rewrite_query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.rewrite_query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rewrite_query: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreSearchRequest> for super::VectorStoreSearchRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreSearchRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                filters: value.filters?,
                max_num_results: value.max_num_results?,
                query: value.query?,
                ranking_options: value.ranking_options?,
                rewrite_query: value.rewrite_query?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreSearchRequest> for VectorStoreSearchRequest {
        fn from(value: super::VectorStoreSearchRequest) -> Self {
            Self {
                filters: Ok(value.filters),
                max_num_results: Ok(value.max_num_results),
                query: Ok(value.query),
                ranking_options: Ok(value.ranking_options),
                rewrite_query: Ok(value.rewrite_query),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreSearchRequestRankingOptions {
        ranker: ::std::result::Result<
            ::std::option::Option<super::VectorStoreSearchRequestRankingOptionsRanker>,
            ::std::string::String,
        >,
        score_threshold: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreSearchRequestRankingOptions {
        fn default() -> Self {
            Self {
                ranker: Ok(Default::default()),
                score_threshold: Ok(Default::default()),
            }
        }
    }
    impl VectorStoreSearchRequestRankingOptions {
        pub fn ranker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VectorStoreSearchRequestRankingOptionsRanker>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ranker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranker: {e}"));
            self
        }
        pub fn score_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.score_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreSearchRequestRankingOptions>
        for super::VectorStoreSearchRequestRankingOptions
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreSearchRequestRankingOptions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ranker: value.ranker?,
                score_threshold: value.score_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreSearchRequestRankingOptions>
        for VectorStoreSearchRequestRankingOptions
    {
        fn from(value: super::VectorStoreSearchRequestRankingOptions) -> Self {
            Self {
                ranker: Ok(value.ranker),
                score_threshold: Ok(value.score_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreSearchResultContentObject {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::VectorStoreSearchResultContentObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VectorStoreSearchResultContentObject {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VectorStoreSearchResultContentObject {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreSearchResultContentObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreSearchResultContentObject>
        for super::VectorStoreSearchResultContentObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreSearchResultContentObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreSearchResultContentObject>
        for VectorStoreSearchResultContentObject
    {
        fn from(value: super::VectorStoreSearchResultContentObject) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreSearchResultItem {
        attributes: ::std::result::Result<
            ::openai_api_types::common::VectorStoreFileAttributes,
            ::std::string::String,
        >,
        content: ::std::result::Result<
            ::std::vec::Vec<super::VectorStoreSearchResultContentObject>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        score: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreSearchResultItem {
        fn default() -> Self {
            Self {
                attributes: Err("no value supplied for attributes".to_string()),
                content: Err("no value supplied for content".to_string()),
                file_id: Err("no value supplied for file_id".to_string()),
                filename: Err("no value supplied for filename".to_string()),
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl VectorStoreSearchResultItem {
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::VectorStoreFileAttributes>,
            T::Error: ::std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attributes: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::VectorStoreSearchResultContentObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {e}"));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreSearchResultItem> for super::VectorStoreSearchResultItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreSearchResultItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attributes: value.attributes?,
                content: value.content?,
                file_id: value.file_id?,
                filename: value.filename?,
                score: value.score?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreSearchResultItem> for VectorStoreSearchResultItem {
        fn from(value: super::VectorStoreSearchResultItem) -> Self {
            Self {
                attributes: Ok(value.attributes),
                content: Ok(value.content),
                file_id: Ok(value.file_id),
                filename: Ok(value.filename),
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoreSearchResultsPage {
        data: ::std::result::Result<
            ::std::vec::Vec<super::VectorStoreSearchResultItem>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next_page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object:
            ::std::result::Result<super::VectorStoreSearchResultsPageObject, ::std::string::String>,
        search_query:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoreSearchResultsPage {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next_page: Err("no value supplied for next_page".to_string()),
                object: Err("no value supplied for object".to_string()),
                search_query: Err("no value supplied for search_query".to_string()),
            }
        }
    }
    impl VectorStoreSearchResultsPage {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VectorStoreSearchResultItem>>,
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
            T: ::std::convert::TryInto<super::VectorStoreSearchResultsPageObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn search_query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.search_query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for search_query: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoreSearchResultsPage> for super::VectorStoreSearchResultsPage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoreSearchResultsPage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next_page: value.next_page?,
                object: value.object?,
                search_query: value.search_query?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoreSearchResultsPage> for VectorStoreSearchResultsPage {
        fn from(value: super::VectorStoreSearchResultsPage) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next_page: Ok(value.next_page),
                object: Ok(value.object),
                search_query: Ok(value.search_query),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VectorStoresApiSchema {
        cancel_vector_store_file_batch_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_vector_store_file_batch_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_vector_store_file_batch_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_vector_store_file_batch_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_vector_store_file_batch_params:
            ::std::result::Result<super::CancelVectorStoreFileBatchParams, ::std::string::String>,
        cancel_vector_store_file_batch_response:
            ::std::result::Result<super::VectorStoreFileBatchObject, ::std::string::String>,
        create_vector_store_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_batch_params:
            ::std::result::Result<super::CreateVectorStoreFileBatchParams, ::std::string::String>,
        create_vector_store_file_batch_request:
            ::std::result::Result<super::CreateVectorStoreFileBatchRequest, ::std::string::String>,
        create_vector_store_file_batch_response:
            ::std::result::Result<super::VectorStoreFileBatchObject, ::std::string::String>,
        create_vector_store_file_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_vector_store_file_params:
            ::std::result::Result<super::CreateVectorStoreFileParams, ::std::string::String>,
        create_vector_store_file_request:
            ::std::result::Result<super::CreateVectorStoreFileRequest, ::std::string::String>,
        create_vector_store_file_response:
            ::std::result::Result<super::VectorStoreFileObject, ::std::string::String>,
        create_vector_store_request:
            ::std::result::Result<super::CreateVectorStoreRequest, ::std::string::String>,
        create_vector_store_response:
            ::std::result::Result<super::VectorStoreObject, ::std::string::String>,
        delete_vector_store_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_vector_store_file_params:
            ::std::result::Result<super::DeleteVectorStoreFileParams, ::std::string::String>,
        delete_vector_store_file_response:
            ::std::result::Result<super::DeleteVectorStoreFileResponse, ::std::string::String>,
        delete_vector_store_params:
            ::std::result::Result<super::DeleteVectorStoreParams, ::std::string::String>,
        delete_vector_store_response:
            ::std::result::Result<super::DeleteVectorStoreResponse, ::std::string::String>,
        get_vector_store_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_batch_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_batch_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_batch_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_batch_params:
            ::std::result::Result<super::GetVectorStoreFileBatchParams, ::std::string::String>,
        get_vector_store_file_batch_response:
            ::std::result::Result<super::VectorStoreFileBatchObject, ::std::string::String>,
        get_vector_store_file_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_vector_store_file_params:
            ::std::result::Result<super::GetVectorStoreFileParams, ::std::string::String>,
        get_vector_store_file_response:
            ::std::result::Result<super::VectorStoreFileObject, ::std::string::String>,
        get_vector_store_params:
            ::std::result::Result<super::GetVectorStoreParams, ::std::string::String>,
        get_vector_store_response:
            ::std::result::Result<super::VectorStoreObject, ::std::string::String>,
        list_files_in_vector_store_batch_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_in_vector_store_batch_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_in_vector_store_batch_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_files_in_vector_store_batch_params:
            ::std::result::Result<super::ListFilesInVectorStoreBatchParams, ::std::string::String>,
        list_files_in_vector_store_batch_response:
            ::std::result::Result<super::ListVectorStoreFilesResponse, ::std::string::String>,
        list_vector_store_files_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_vector_store_files_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_vector_store_files_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_vector_store_files_params:
            ::std::result::Result<super::ListVectorStoreFilesParams, ::std::string::String>,
        list_vector_store_files_response:
            ::std::result::Result<super::ListVectorStoreFilesResponse, ::std::string::String>,
        list_vector_stores_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_vector_stores_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_vector_stores_params:
            ::std::result::Result<super::ListVectorStoresParams, ::std::string::String>,
        list_vector_stores_response:
            ::std::result::Result<super::ListVectorStoresResponse, ::std::string::String>,
        modify_vector_store_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_vector_store_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_vector_store_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_vector_store_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_vector_store_params:
            ::std::result::Result<super::ModifyVectorStoreParams, ::std::string::String>,
        modify_vector_store_request:
            ::std::result::Result<super::UpdateVectorStoreRequest, ::std::string::String>,
        modify_vector_store_response:
            ::std::result::Result<super::VectorStoreObject, ::std::string::String>,
        retrieve_vector_store_file_content_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_vector_store_file_content_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_vector_store_file_content_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_vector_store_file_content_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_vector_store_file_content_params: ::std::result::Result<
            super::RetrieveVectorStoreFileContentParams,
            ::std::string::String,
        >,
        retrieve_vector_store_file_content_response:
            ::std::result::Result<super::VectorStoreFileContentResponse, ::std::string::String>,
        search_vector_store_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        search_vector_store_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        search_vector_store_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        search_vector_store_params:
            ::std::result::Result<super::SearchVectorStoreParams, ::std::string::String>,
        search_vector_store_request:
            ::std::result::Result<super::VectorStoreSearchRequest, ::std::string::String>,
        search_vector_store_response:
            ::std::result::Result<super::VectorStoreSearchResultsPage, ::std::string::String>,
        update_vector_store_file_attributes_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_vector_store_file_attributes_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_vector_store_file_attributes_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_vector_store_file_attributes_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_vector_store_file_attributes_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_vector_store_file_attributes_params: ::std::result::Result<
            super::UpdateVectorStoreFileAttributesParams,
            ::std::string::String,
        >,
        update_vector_store_file_attributes_request: ::std::result::Result<
            super::UpdateVectorStoreFileAttributesRequest,
            ::std::string::String,
        >,
        update_vector_store_file_attributes_response:
            ::std::result::Result<super::VectorStoreFileObject, ::std::string::String>,
    }
    impl ::std::default::Default for VectorStoresApiSchema {
        fn default() -> Self {
            Self {
                cancel_vector_store_file_batch_error400: Err(
                    "no value supplied for cancel_vector_store_file_batch_error400".to_string(),
                ),
                cancel_vector_store_file_batch_error404: Err(
                    "no value supplied for cancel_vector_store_file_batch_error404".to_string(),
                ),
                cancel_vector_store_file_batch_error409: Err(
                    "no value supplied for cancel_vector_store_file_batch_error409".to_string(),
                ),
                cancel_vector_store_file_batch_error429: Err(
                    "no value supplied for cancel_vector_store_file_batch_error429".to_string(),
                ),
                cancel_vector_store_file_batch_params: Err(
                    "no value supplied for cancel_vector_store_file_batch_params".to_string(),
                ),
                cancel_vector_store_file_batch_response: Err(
                    "no value supplied for cancel_vector_store_file_batch_response".to_string(),
                ),
                create_vector_store_error400: Err(
                    "no value supplied for create_vector_store_error400".to_string(),
                ),
                create_vector_store_error404: Err(
                    "no value supplied for create_vector_store_error404".to_string(),
                ),
                create_vector_store_error429: Err(
                    "no value supplied for create_vector_store_error429".to_string(),
                ),
                create_vector_store_file_batch_error400: Err(
                    "no value supplied for create_vector_store_file_batch_error400".to_string(),
                ),
                create_vector_store_file_batch_error404: Err(
                    "no value supplied for create_vector_store_file_batch_error404".to_string(),
                ),
                create_vector_store_file_batch_error409: Err(
                    "no value supplied for create_vector_store_file_batch_error409".to_string(),
                ),
                create_vector_store_file_batch_error429: Err(
                    "no value supplied for create_vector_store_file_batch_error429".to_string(),
                ),
                create_vector_store_file_batch_error500: Err(
                    "no value supplied for create_vector_store_file_batch_error500".to_string(),
                ),
                create_vector_store_file_batch_params: Err(
                    "no value supplied for create_vector_store_file_batch_params".to_string(),
                ),
                create_vector_store_file_batch_request: Err(
                    "no value supplied for create_vector_store_file_batch_request".to_string(),
                ),
                create_vector_store_file_batch_response: Err(
                    "no value supplied for create_vector_store_file_batch_response".to_string(),
                ),
                create_vector_store_file_error400: Err(
                    "no value supplied for create_vector_store_file_error400".to_string(),
                ),
                create_vector_store_file_error404: Err(
                    "no value supplied for create_vector_store_file_error404".to_string(),
                ),
                create_vector_store_file_error409: Err(
                    "no value supplied for create_vector_store_file_error409".to_string(),
                ),
                create_vector_store_file_error429: Err(
                    "no value supplied for create_vector_store_file_error429".to_string(),
                ),
                create_vector_store_file_error500: Err(
                    "no value supplied for create_vector_store_file_error500".to_string(),
                ),
                create_vector_store_file_params: Err(
                    "no value supplied for create_vector_store_file_params".to_string(),
                ),
                create_vector_store_file_request: Err(
                    "no value supplied for create_vector_store_file_request".to_string(),
                ),
                create_vector_store_file_response: Err(
                    "no value supplied for create_vector_store_file_response".to_string(),
                ),
                create_vector_store_request: Err(
                    "no value supplied for create_vector_store_request".to_string(),
                ),
                create_vector_store_response: Err(
                    "no value supplied for create_vector_store_response".to_string(),
                ),
                delete_vector_store_error404: Err(
                    "no value supplied for delete_vector_store_error404".to_string(),
                ),
                delete_vector_store_error409: Err(
                    "no value supplied for delete_vector_store_error409".to_string(),
                ),
                delete_vector_store_error429: Err(
                    "no value supplied for delete_vector_store_error429".to_string(),
                ),
                delete_vector_store_file_error400: Err(
                    "no value supplied for delete_vector_store_file_error400".to_string(),
                ),
                delete_vector_store_file_error404: Err(
                    "no value supplied for delete_vector_store_file_error404".to_string(),
                ),
                delete_vector_store_file_error409: Err(
                    "no value supplied for delete_vector_store_file_error409".to_string(),
                ),
                delete_vector_store_file_error429: Err(
                    "no value supplied for delete_vector_store_file_error429".to_string(),
                ),
                delete_vector_store_file_error500: Err(
                    "no value supplied for delete_vector_store_file_error500".to_string(),
                ),
                delete_vector_store_file_params: Err(
                    "no value supplied for delete_vector_store_file_params".to_string(),
                ),
                delete_vector_store_file_response: Err(
                    "no value supplied for delete_vector_store_file_response".to_string(),
                ),
                delete_vector_store_params: Err(
                    "no value supplied for delete_vector_store_params".to_string()
                ),
                delete_vector_store_response: Err(
                    "no value supplied for delete_vector_store_response".to_string(),
                ),
                get_vector_store_error404: Err(
                    "no value supplied for get_vector_store_error404".to_string()
                ),
                get_vector_store_error429: Err(
                    "no value supplied for get_vector_store_error429".to_string()
                ),
                get_vector_store_file_batch_error400: Err(
                    "no value supplied for get_vector_store_file_batch_error400".to_string(),
                ),
                get_vector_store_file_batch_error404: Err(
                    "no value supplied for get_vector_store_file_batch_error404".to_string(),
                ),
                get_vector_store_file_batch_error429: Err(
                    "no value supplied for get_vector_store_file_batch_error429".to_string(),
                ),
                get_vector_store_file_batch_params: Err(
                    "no value supplied for get_vector_store_file_batch_params".to_string(),
                ),
                get_vector_store_file_batch_response: Err(
                    "no value supplied for get_vector_store_file_batch_response".to_string(),
                ),
                get_vector_store_file_error404: Err(
                    "no value supplied for get_vector_store_file_error404".to_string(),
                ),
                get_vector_store_file_error429: Err(
                    "no value supplied for get_vector_store_file_error429".to_string(),
                ),
                get_vector_store_file_error500: Err(
                    "no value supplied for get_vector_store_file_error500".to_string(),
                ),
                get_vector_store_file_params: Err(
                    "no value supplied for get_vector_store_file_params".to_string(),
                ),
                get_vector_store_file_response: Err(
                    "no value supplied for get_vector_store_file_response".to_string(),
                ),
                get_vector_store_params: Err(
                    "no value supplied for get_vector_store_params".to_string()
                ),
                get_vector_store_response: Err(
                    "no value supplied for get_vector_store_response".to_string()
                ),
                list_files_in_vector_store_batch_error400: Err(
                    "no value supplied for list_files_in_vector_store_batch_error400".to_string(),
                ),
                list_files_in_vector_store_batch_error404: Err(
                    "no value supplied for list_files_in_vector_store_batch_error404".to_string(),
                ),
                list_files_in_vector_store_batch_error429: Err(
                    "no value supplied for list_files_in_vector_store_batch_error429".to_string(),
                ),
                list_files_in_vector_store_batch_params: Err(
                    "no value supplied for list_files_in_vector_store_batch_params".to_string(),
                ),
                list_files_in_vector_store_batch_response: Err(
                    "no value supplied for list_files_in_vector_store_batch_response".to_string(),
                ),
                list_vector_store_files_error400: Err(
                    "no value supplied for list_vector_store_files_error400".to_string(),
                ),
                list_vector_store_files_error404: Err(
                    "no value supplied for list_vector_store_files_error404".to_string(),
                ),
                list_vector_store_files_error429: Err(
                    "no value supplied for list_vector_store_files_error429".to_string(),
                ),
                list_vector_store_files_params: Err(
                    "no value supplied for list_vector_store_files_params".to_string(),
                ),
                list_vector_store_files_response: Err(
                    "no value supplied for list_vector_store_files_response".to_string(),
                ),
                list_vector_stores_error429: Err(
                    "no value supplied for list_vector_stores_error429".to_string(),
                ),
                list_vector_stores_error503: Err(
                    "no value supplied for list_vector_stores_error503".to_string(),
                ),
                list_vector_stores_params: Err(
                    "no value supplied for list_vector_stores_params".to_string()
                ),
                list_vector_stores_response: Err(
                    "no value supplied for list_vector_stores_response".to_string(),
                ),
                modify_vector_store_error400: Err(
                    "no value supplied for modify_vector_store_error400".to_string(),
                ),
                modify_vector_store_error403: Err(
                    "no value supplied for modify_vector_store_error403".to_string(),
                ),
                modify_vector_store_error404: Err(
                    "no value supplied for modify_vector_store_error404".to_string(),
                ),
                modify_vector_store_error429: Err(
                    "no value supplied for modify_vector_store_error429".to_string(),
                ),
                modify_vector_store_params: Err(
                    "no value supplied for modify_vector_store_params".to_string()
                ),
                modify_vector_store_request: Err(
                    "no value supplied for modify_vector_store_request".to_string(),
                ),
                modify_vector_store_response: Err(
                    "no value supplied for modify_vector_store_response".to_string(),
                ),
                retrieve_vector_store_file_content_error400: Err(
                    "no value supplied for retrieve_vector_store_file_content_error400".to_string(),
                ),
                retrieve_vector_store_file_content_error404: Err(
                    "no value supplied for retrieve_vector_store_file_content_error404".to_string(),
                ),
                retrieve_vector_store_file_content_error429: Err(
                    "no value supplied for retrieve_vector_store_file_content_error429".to_string(),
                ),
                retrieve_vector_store_file_content_error500: Err(
                    "no value supplied for retrieve_vector_store_file_content_error500".to_string(),
                ),
                retrieve_vector_store_file_content_params: Err(
                    "no value supplied for retrieve_vector_store_file_content_params".to_string(),
                ),
                retrieve_vector_store_file_content_response: Err(
                    "no value supplied for retrieve_vector_store_file_content_response".to_string(),
                ),
                search_vector_store_error400: Err(
                    "no value supplied for search_vector_store_error400".to_string(),
                ),
                search_vector_store_error404: Err(
                    "no value supplied for search_vector_store_error404".to_string(),
                ),
                search_vector_store_error429: Err(
                    "no value supplied for search_vector_store_error429".to_string(),
                ),
                search_vector_store_params: Err(
                    "no value supplied for search_vector_store_params".to_string()
                ),
                search_vector_store_request: Err(
                    "no value supplied for search_vector_store_request".to_string(),
                ),
                search_vector_store_response: Err(
                    "no value supplied for search_vector_store_response".to_string(),
                ),
                update_vector_store_file_attributes_error400: Err(
                    "no value supplied for update_vector_store_file_attributes_error400"
                        .to_string(),
                ),
                update_vector_store_file_attributes_error404: Err(
                    "no value supplied for update_vector_store_file_attributes_error404"
                        .to_string(),
                ),
                update_vector_store_file_attributes_error409: Err(
                    "no value supplied for update_vector_store_file_attributes_error409"
                        .to_string(),
                ),
                update_vector_store_file_attributes_error429: Err(
                    "no value supplied for update_vector_store_file_attributes_error429"
                        .to_string(),
                ),
                update_vector_store_file_attributes_error500: Err(
                    "no value supplied for update_vector_store_file_attributes_error500"
                        .to_string(),
                ),
                update_vector_store_file_attributes_params: Err(
                    "no value supplied for update_vector_store_file_attributes_params".to_string(),
                ),
                update_vector_store_file_attributes_request: Err(
                    "no value supplied for update_vector_store_file_attributes_request".to_string(),
                ),
                update_vector_store_file_attributes_response: Err(
                    "no value supplied for update_vector_store_file_attributes_response"
                        .to_string(),
                ),
            }
        }
    }
    impl VectorStoresApiSchema {
        pub fn cancel_vector_store_file_batch_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_error400: {e}")) ;
            self
        }
        pub fn cancel_vector_store_file_batch_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_error404: {e}")) ;
            self
        }
        pub fn cancel_vector_store_file_batch_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_error409 = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_error409: {e}")) ;
            self
        }
        pub fn cancel_vector_store_file_batch_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_error429: {e}")) ;
            self
        }
        pub fn cancel_vector_store_file_batch_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelVectorStoreFileBatchParams>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_params: {e}")) ;
            self
        }
        pub fn cancel_vector_store_file_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObject>,
            T::Error: ::std::fmt::Display,
        {
            self . cancel_vector_store_file_batch_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for cancel_vector_store_file_batch_response: {e}")) ;
            self
        }
        pub fn create_vector_store_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_error400: {e}")
            });
            self
        }
        pub fn create_vector_store_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_error404: {e}")
            });
            self
        }
        pub fn create_vector_store_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_error429: {e}")
            });
            self
        }
        pub fn create_vector_store_file_batch_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_error400: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_error404: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_error409 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_error409: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_error429: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_error500: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVectorStoreFileBatchParams>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_params: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVectorStoreFileBatchRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_request: {e}")) ;
            self
        }
        pub fn create_vector_store_file_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObject>,
            T::Error: ::std::fmt::Display,
        {
            self . create_vector_store_file_batch_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_vector_store_file_batch_response: {e}")) ;
            self
        }
        pub fn create_vector_store_file_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_error400: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_error404: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_file_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_error409 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_error409: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_error429: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_file_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_error500 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_error500: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVectorStoreFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_file_params: {e}")
            });
            self
        }
        pub fn create_vector_store_file_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVectorStoreFileRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_file_request: {e}")
            });
            self
        }
        pub fn create_vector_store_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_file_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_vector_store_file_response: {e}"
                )
            });
            self
        }
        pub fn create_vector_store_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVectorStoreRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_request: {e}")
            });
            self
        }
        pub fn create_vector_store_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vector_store_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vector_store_response: {e}")
            });
            self
        }
        pub fn delete_vector_store_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_error404: {e}")
            });
            self
        }
        pub fn delete_vector_store_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_error409: {e}")
            });
            self
        }
        pub fn delete_vector_store_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_error429: {e}")
            });
            self
        }
        pub fn delete_vector_store_file_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_error400: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_error404: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_file_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_error409 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_error409: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_error429: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_file_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_error500 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_error500: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_file_params: {e}")
            });
            self
        }
        pub fn delete_vector_store_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreFileResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_file_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_vector_store_file_response: {e}"
                )
            });
            self
        }
        pub fn delete_vector_store_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_params: {e}")
            });
            self
        }
        pub fn delete_vector_store_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVectorStoreResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vector_store_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vector_store_response: {e}")
            });
            self
        }
        pub fn get_vector_store_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_error404: {e}")
            });
            self
        }
        pub fn get_vector_store_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_error429: {e}")
            });
            self
        }
        pub fn get_vector_store_file_batch_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_batch_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_vector_store_file_batch_error400: {e}"
                )
            });
            self
        }
        pub fn get_vector_store_file_batch_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_batch_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_vector_store_file_batch_error404: {e}"
                )
            });
            self
        }
        pub fn get_vector_store_file_batch_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_batch_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_vector_store_file_batch_error429: {e}"
                )
            });
            self
        }
        pub fn get_vector_store_file_batch_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVectorStoreFileBatchParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_batch_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_vector_store_file_batch_params: {e}"
                )
            });
            self
        }
        pub fn get_vector_store_file_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileBatchObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_batch_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_vector_store_file_batch_response: {e}"
                )
            });
            self
        }
        pub fn get_vector_store_file_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_file_error404: {e}")
            });
            self
        }
        pub fn get_vector_store_file_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_file_error429: {e}")
            });
            self
        }
        pub fn get_vector_store_file_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_file_error500: {e}")
            });
            self
        }
        pub fn get_vector_store_file_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVectorStoreFileParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_file_params: {e}")
            });
            self
        }
        pub fn get_vector_store_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_file_response: {e}")
            });
            self
        }
        pub fn get_vector_store_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVectorStoreParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_params: {e}")
            });
            self
        }
        pub fn get_vector_store_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_vector_store_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_vector_store_response: {e}")
            });
            self
        }
        pub fn list_files_in_vector_store_batch_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_files_in_vector_store_batch_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_files_in_vector_store_batch_error400: {e}")) ;
            self
        }
        pub fn list_files_in_vector_store_batch_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_files_in_vector_store_batch_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_files_in_vector_store_batch_error404: {e}")) ;
            self
        }
        pub fn list_files_in_vector_store_batch_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_files_in_vector_store_batch_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_files_in_vector_store_batch_error429: {e}")) ;
            self
        }
        pub fn list_files_in_vector_store_batch_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFilesInVectorStoreBatchParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_files_in_vector_store_batch_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_files_in_vector_store_batch_params: {e}")) ;
            self
        }
        pub fn list_files_in_vector_store_batch_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVectorStoreFilesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_files_in_vector_store_batch_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_files_in_vector_store_batch_response: {e}")) ;
            self
        }
        pub fn list_vector_store_files_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_store_files_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_store_files_error400: {e}")
            });
            self
        }
        pub fn list_vector_store_files_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_store_files_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_store_files_error404: {e}")
            });
            self
        }
        pub fn list_vector_store_files_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_store_files_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_store_files_error429: {e}")
            });
            self
        }
        pub fn list_vector_store_files_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVectorStoreFilesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_store_files_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_store_files_params: {e}")
            });
            self
        }
        pub fn list_vector_store_files_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVectorStoreFilesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_store_files_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_store_files_response: {e}")
            });
            self
        }
        pub fn list_vector_stores_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_stores_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_stores_error429: {e}")
            });
            self
        }
        pub fn list_vector_stores_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_stores_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_stores_error503: {e}")
            });
            self
        }
        pub fn list_vector_stores_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVectorStoresParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_stores_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_stores_params: {e}")
            });
            self
        }
        pub fn list_vector_stores_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVectorStoresResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vector_stores_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vector_stores_response: {e}")
            });
            self
        }
        pub fn modify_vector_store_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_error400: {e}")
            });
            self
        }
        pub fn modify_vector_store_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_error403: {e}")
            });
            self
        }
        pub fn modify_vector_store_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_error404: {e}")
            });
            self
        }
        pub fn modify_vector_store_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_error429: {e}")
            });
            self
        }
        pub fn modify_vector_store_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyVectorStoreParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_params: {e}")
            });
            self
        }
        pub fn modify_vector_store_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVectorStoreRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_request: {e}")
            });
            self
        }
        pub fn modify_vector_store_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreObject>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_vector_store_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_vector_store_response: {e}")
            });
            self
        }
        pub fn retrieve_vector_store_file_content_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_error400: {e}")) ;
            self
        }
        pub fn retrieve_vector_store_file_content_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_error404: {e}")) ;
            self
        }
        pub fn retrieve_vector_store_file_content_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_error429: {e}")) ;
            self
        }
        pub fn retrieve_vector_store_file_content_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_error500: {e}")) ;
            self
        }
        pub fn retrieve_vector_store_file_content_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVectorStoreFileContentParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_params: {e}")) ;
            self
        }
        pub fn retrieve_vector_store_file_content_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileContentResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_vector_store_file_content_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_vector_store_file_content_response: {e}")) ;
            self
        }
        pub fn search_vector_store_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_error400: {e}")
            });
            self
        }
        pub fn search_vector_store_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_error404: {e}")
            });
            self
        }
        pub fn search_vector_store_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_error429: {e}")
            });
            self
        }
        pub fn search_vector_store_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchVectorStoreParams>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_params: {e}")
            });
            self
        }
        pub fn search_vector_store_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreSearchRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_request: {e}")
            });
            self
        }
        pub fn search_vector_store_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreSearchResultsPage>,
            T::Error: ::std::fmt::Display,
        {
            self.search_vector_store_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_vector_store_response: {e}")
            });
            self
        }
        pub fn update_vector_store_file_attributes_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_error400: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_error404: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_error409 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_error409: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_error429: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_error500 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_error500: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVectorStoreFileAttributesParams>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_params: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVectorStoreFileAttributesRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_request: {e}")) ;
            self
        }
        pub fn update_vector_store_file_attributes_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VectorStoreFileObject>,
            T::Error: ::std::fmt::Display,
        {
            self . update_vector_store_file_attributes_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_vector_store_file_attributes_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<VectorStoresApiSchema> for super::VectorStoresApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VectorStoresApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_vector_store_file_batch_error400: value
                    .cancel_vector_store_file_batch_error400?,
                cancel_vector_store_file_batch_error404: value
                    .cancel_vector_store_file_batch_error404?,
                cancel_vector_store_file_batch_error409: value
                    .cancel_vector_store_file_batch_error409?,
                cancel_vector_store_file_batch_error429: value
                    .cancel_vector_store_file_batch_error429?,
                cancel_vector_store_file_batch_params: value
                    .cancel_vector_store_file_batch_params?,
                cancel_vector_store_file_batch_response: value
                    .cancel_vector_store_file_batch_response?,
                create_vector_store_error400: value.create_vector_store_error400?,
                create_vector_store_error404: value.create_vector_store_error404?,
                create_vector_store_error429: value.create_vector_store_error429?,
                create_vector_store_file_batch_error400: value
                    .create_vector_store_file_batch_error400?,
                create_vector_store_file_batch_error404: value
                    .create_vector_store_file_batch_error404?,
                create_vector_store_file_batch_error409: value
                    .create_vector_store_file_batch_error409?,
                create_vector_store_file_batch_error429: value
                    .create_vector_store_file_batch_error429?,
                create_vector_store_file_batch_error500: value
                    .create_vector_store_file_batch_error500?,
                create_vector_store_file_batch_params: value
                    .create_vector_store_file_batch_params?,
                create_vector_store_file_batch_request: value
                    .create_vector_store_file_batch_request?,
                create_vector_store_file_batch_response: value
                    .create_vector_store_file_batch_response?,
                create_vector_store_file_error400: value.create_vector_store_file_error400?,
                create_vector_store_file_error404: value.create_vector_store_file_error404?,
                create_vector_store_file_error409: value.create_vector_store_file_error409?,
                create_vector_store_file_error429: value.create_vector_store_file_error429?,
                create_vector_store_file_error500: value.create_vector_store_file_error500?,
                create_vector_store_file_params: value.create_vector_store_file_params?,
                create_vector_store_file_request: value.create_vector_store_file_request?,
                create_vector_store_file_response: value.create_vector_store_file_response?,
                create_vector_store_request: value.create_vector_store_request?,
                create_vector_store_response: value.create_vector_store_response?,
                delete_vector_store_error404: value.delete_vector_store_error404?,
                delete_vector_store_error409: value.delete_vector_store_error409?,
                delete_vector_store_error429: value.delete_vector_store_error429?,
                delete_vector_store_file_error400: value.delete_vector_store_file_error400?,
                delete_vector_store_file_error404: value.delete_vector_store_file_error404?,
                delete_vector_store_file_error409: value.delete_vector_store_file_error409?,
                delete_vector_store_file_error429: value.delete_vector_store_file_error429?,
                delete_vector_store_file_error500: value.delete_vector_store_file_error500?,
                delete_vector_store_file_params: value.delete_vector_store_file_params?,
                delete_vector_store_file_response: value.delete_vector_store_file_response?,
                delete_vector_store_params: value.delete_vector_store_params?,
                delete_vector_store_response: value.delete_vector_store_response?,
                get_vector_store_error404: value.get_vector_store_error404?,
                get_vector_store_error429: value.get_vector_store_error429?,
                get_vector_store_file_batch_error400: value.get_vector_store_file_batch_error400?,
                get_vector_store_file_batch_error404: value.get_vector_store_file_batch_error404?,
                get_vector_store_file_batch_error429: value.get_vector_store_file_batch_error429?,
                get_vector_store_file_batch_params: value.get_vector_store_file_batch_params?,
                get_vector_store_file_batch_response: value.get_vector_store_file_batch_response?,
                get_vector_store_file_error404: value.get_vector_store_file_error404?,
                get_vector_store_file_error429: value.get_vector_store_file_error429?,
                get_vector_store_file_error500: value.get_vector_store_file_error500?,
                get_vector_store_file_params: value.get_vector_store_file_params?,
                get_vector_store_file_response: value.get_vector_store_file_response?,
                get_vector_store_params: value.get_vector_store_params?,
                get_vector_store_response: value.get_vector_store_response?,
                list_files_in_vector_store_batch_error400: value
                    .list_files_in_vector_store_batch_error400?,
                list_files_in_vector_store_batch_error404: value
                    .list_files_in_vector_store_batch_error404?,
                list_files_in_vector_store_batch_error429: value
                    .list_files_in_vector_store_batch_error429?,
                list_files_in_vector_store_batch_params: value
                    .list_files_in_vector_store_batch_params?,
                list_files_in_vector_store_batch_response: value
                    .list_files_in_vector_store_batch_response?,
                list_vector_store_files_error400: value.list_vector_store_files_error400?,
                list_vector_store_files_error404: value.list_vector_store_files_error404?,
                list_vector_store_files_error429: value.list_vector_store_files_error429?,
                list_vector_store_files_params: value.list_vector_store_files_params?,
                list_vector_store_files_response: value.list_vector_store_files_response?,
                list_vector_stores_error429: value.list_vector_stores_error429?,
                list_vector_stores_error503: value.list_vector_stores_error503?,
                list_vector_stores_params: value.list_vector_stores_params?,
                list_vector_stores_response: value.list_vector_stores_response?,
                modify_vector_store_error400: value.modify_vector_store_error400?,
                modify_vector_store_error403: value.modify_vector_store_error403?,
                modify_vector_store_error404: value.modify_vector_store_error404?,
                modify_vector_store_error429: value.modify_vector_store_error429?,
                modify_vector_store_params: value.modify_vector_store_params?,
                modify_vector_store_request: value.modify_vector_store_request?,
                modify_vector_store_response: value.modify_vector_store_response?,
                retrieve_vector_store_file_content_error400: value
                    .retrieve_vector_store_file_content_error400?,
                retrieve_vector_store_file_content_error404: value
                    .retrieve_vector_store_file_content_error404?,
                retrieve_vector_store_file_content_error429: value
                    .retrieve_vector_store_file_content_error429?,
                retrieve_vector_store_file_content_error500: value
                    .retrieve_vector_store_file_content_error500?,
                retrieve_vector_store_file_content_params: value
                    .retrieve_vector_store_file_content_params?,
                retrieve_vector_store_file_content_response: value
                    .retrieve_vector_store_file_content_response?,
                search_vector_store_error400: value.search_vector_store_error400?,
                search_vector_store_error404: value.search_vector_store_error404?,
                search_vector_store_error429: value.search_vector_store_error429?,
                search_vector_store_params: value.search_vector_store_params?,
                search_vector_store_request: value.search_vector_store_request?,
                search_vector_store_response: value.search_vector_store_response?,
                update_vector_store_file_attributes_error400: value
                    .update_vector_store_file_attributes_error400?,
                update_vector_store_file_attributes_error404: value
                    .update_vector_store_file_attributes_error404?,
                update_vector_store_file_attributes_error409: value
                    .update_vector_store_file_attributes_error409?,
                update_vector_store_file_attributes_error429: value
                    .update_vector_store_file_attributes_error429?,
                update_vector_store_file_attributes_error500: value
                    .update_vector_store_file_attributes_error500?,
                update_vector_store_file_attributes_params: value
                    .update_vector_store_file_attributes_params?,
                update_vector_store_file_attributes_request: value
                    .update_vector_store_file_attributes_request?,
                update_vector_store_file_attributes_response: value
                    .update_vector_store_file_attributes_response?,
            })
        }
    }
    impl ::std::convert::From<super::VectorStoresApiSchema> for VectorStoresApiSchema {
        fn from(value: super::VectorStoresApiSchema) -> Self {
            Self {
                cancel_vector_store_file_batch_error400: Ok(
                    value.cancel_vector_store_file_batch_error400
                ),
                cancel_vector_store_file_batch_error404: Ok(
                    value.cancel_vector_store_file_batch_error404
                ),
                cancel_vector_store_file_batch_error409: Ok(
                    value.cancel_vector_store_file_batch_error409
                ),
                cancel_vector_store_file_batch_error429: Ok(
                    value.cancel_vector_store_file_batch_error429
                ),
                cancel_vector_store_file_batch_params: Ok(
                    value.cancel_vector_store_file_batch_params
                ),
                cancel_vector_store_file_batch_response: Ok(
                    value.cancel_vector_store_file_batch_response
                ),
                create_vector_store_error400: Ok(value.create_vector_store_error400),
                create_vector_store_error404: Ok(value.create_vector_store_error404),
                create_vector_store_error429: Ok(value.create_vector_store_error429),
                create_vector_store_file_batch_error400: Ok(
                    value.create_vector_store_file_batch_error400
                ),
                create_vector_store_file_batch_error404: Ok(
                    value.create_vector_store_file_batch_error404
                ),
                create_vector_store_file_batch_error409: Ok(
                    value.create_vector_store_file_batch_error409
                ),
                create_vector_store_file_batch_error429: Ok(
                    value.create_vector_store_file_batch_error429
                ),
                create_vector_store_file_batch_error500: Ok(
                    value.create_vector_store_file_batch_error500
                ),
                create_vector_store_file_batch_params: Ok(
                    value.create_vector_store_file_batch_params
                ),
                create_vector_store_file_batch_request: Ok(
                    value.create_vector_store_file_batch_request
                ),
                create_vector_store_file_batch_response: Ok(
                    value.create_vector_store_file_batch_response
                ),
                create_vector_store_file_error400: Ok(value.create_vector_store_file_error400),
                create_vector_store_file_error404: Ok(value.create_vector_store_file_error404),
                create_vector_store_file_error409: Ok(value.create_vector_store_file_error409),
                create_vector_store_file_error429: Ok(value.create_vector_store_file_error429),
                create_vector_store_file_error500: Ok(value.create_vector_store_file_error500),
                create_vector_store_file_params: Ok(value.create_vector_store_file_params),
                create_vector_store_file_request: Ok(value.create_vector_store_file_request),
                create_vector_store_file_response: Ok(value.create_vector_store_file_response),
                create_vector_store_request: Ok(value.create_vector_store_request),
                create_vector_store_response: Ok(value.create_vector_store_response),
                delete_vector_store_error404: Ok(value.delete_vector_store_error404),
                delete_vector_store_error409: Ok(value.delete_vector_store_error409),
                delete_vector_store_error429: Ok(value.delete_vector_store_error429),
                delete_vector_store_file_error400: Ok(value.delete_vector_store_file_error400),
                delete_vector_store_file_error404: Ok(value.delete_vector_store_file_error404),
                delete_vector_store_file_error409: Ok(value.delete_vector_store_file_error409),
                delete_vector_store_file_error429: Ok(value.delete_vector_store_file_error429),
                delete_vector_store_file_error500: Ok(value.delete_vector_store_file_error500),
                delete_vector_store_file_params: Ok(value.delete_vector_store_file_params),
                delete_vector_store_file_response: Ok(value.delete_vector_store_file_response),
                delete_vector_store_params: Ok(value.delete_vector_store_params),
                delete_vector_store_response: Ok(value.delete_vector_store_response),
                get_vector_store_error404: Ok(value.get_vector_store_error404),
                get_vector_store_error429: Ok(value.get_vector_store_error429),
                get_vector_store_file_batch_error400: Ok(value.get_vector_store_file_batch_error400),
                get_vector_store_file_batch_error404: Ok(value.get_vector_store_file_batch_error404),
                get_vector_store_file_batch_error429: Ok(value.get_vector_store_file_batch_error429),
                get_vector_store_file_batch_params: Ok(value.get_vector_store_file_batch_params),
                get_vector_store_file_batch_response: Ok(value.get_vector_store_file_batch_response),
                get_vector_store_file_error404: Ok(value.get_vector_store_file_error404),
                get_vector_store_file_error429: Ok(value.get_vector_store_file_error429),
                get_vector_store_file_error500: Ok(value.get_vector_store_file_error500),
                get_vector_store_file_params: Ok(value.get_vector_store_file_params),
                get_vector_store_file_response: Ok(value.get_vector_store_file_response),
                get_vector_store_params: Ok(value.get_vector_store_params),
                get_vector_store_response: Ok(value.get_vector_store_response),
                list_files_in_vector_store_batch_error400: Ok(
                    value.list_files_in_vector_store_batch_error400
                ),
                list_files_in_vector_store_batch_error404: Ok(
                    value.list_files_in_vector_store_batch_error404
                ),
                list_files_in_vector_store_batch_error429: Ok(
                    value.list_files_in_vector_store_batch_error429
                ),
                list_files_in_vector_store_batch_params: Ok(
                    value.list_files_in_vector_store_batch_params
                ),
                list_files_in_vector_store_batch_response: Ok(
                    value.list_files_in_vector_store_batch_response
                ),
                list_vector_store_files_error400: Ok(value.list_vector_store_files_error400),
                list_vector_store_files_error404: Ok(value.list_vector_store_files_error404),
                list_vector_store_files_error429: Ok(value.list_vector_store_files_error429),
                list_vector_store_files_params: Ok(value.list_vector_store_files_params),
                list_vector_store_files_response: Ok(value.list_vector_store_files_response),
                list_vector_stores_error429: Ok(value.list_vector_stores_error429),
                list_vector_stores_error503: Ok(value.list_vector_stores_error503),
                list_vector_stores_params: Ok(value.list_vector_stores_params),
                list_vector_stores_response: Ok(value.list_vector_stores_response),
                modify_vector_store_error400: Ok(value.modify_vector_store_error400),
                modify_vector_store_error403: Ok(value.modify_vector_store_error403),
                modify_vector_store_error404: Ok(value.modify_vector_store_error404),
                modify_vector_store_error429: Ok(value.modify_vector_store_error429),
                modify_vector_store_params: Ok(value.modify_vector_store_params),
                modify_vector_store_request: Ok(value.modify_vector_store_request),
                modify_vector_store_response: Ok(value.modify_vector_store_response),
                retrieve_vector_store_file_content_error400: Ok(
                    value.retrieve_vector_store_file_content_error400
                ),
                retrieve_vector_store_file_content_error404: Ok(
                    value.retrieve_vector_store_file_content_error404
                ),
                retrieve_vector_store_file_content_error429: Ok(
                    value.retrieve_vector_store_file_content_error429
                ),
                retrieve_vector_store_file_content_error500: Ok(
                    value.retrieve_vector_store_file_content_error500
                ),
                retrieve_vector_store_file_content_params: Ok(
                    value.retrieve_vector_store_file_content_params
                ),
                retrieve_vector_store_file_content_response: Ok(
                    value.retrieve_vector_store_file_content_response
                ),
                search_vector_store_error400: Ok(value.search_vector_store_error400),
                search_vector_store_error404: Ok(value.search_vector_store_error404),
                search_vector_store_error429: Ok(value.search_vector_store_error429),
                search_vector_store_params: Ok(value.search_vector_store_params),
                search_vector_store_request: Ok(value.search_vector_store_request),
                search_vector_store_response: Ok(value.search_vector_store_response),
                update_vector_store_file_attributes_error400: Ok(
                    value.update_vector_store_file_attributes_error400
                ),
                update_vector_store_file_attributes_error404: Ok(
                    value.update_vector_store_file_attributes_error404
                ),
                update_vector_store_file_attributes_error409: Ok(
                    value.update_vector_store_file_attributes_error409
                ),
                update_vector_store_file_attributes_error429: Ok(
                    value.update_vector_store_file_attributes_error429
                ),
                update_vector_store_file_attributes_error500: Ok(
                    value.update_vector_store_file_attributes_error500
                ),
                update_vector_store_file_attributes_params: Ok(
                    value.update_vector_store_file_attributes_params
                ),
                update_vector_store_file_attributes_request: Ok(
                    value.update_vector_store_file_attributes_request
                ),
                update_vector_store_file_attributes_response: Ok(
                    value.update_vector_store_file_attributes_response
                ),
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
