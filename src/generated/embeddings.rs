#[doc = "`CreateEmbeddingRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateEmbeddingRequest {
    #[doc = "The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dimensions: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding_format: ::std::option::Option<CreateEmbeddingRequestEncodingFormat>,
    #[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for all embedding models), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. In addition to the per-input token limit, all embedding models enforce a maximum of 300,000 tokens summed across all inputs in a single request.\n"]
    pub input: CreateEmbeddingRequestInput,
    #[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
    pub model: ::std::string::String,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
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
pub enum CreateEmbeddingRequestEncodingFormat {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}
impl ::std::fmt::Display for CreateEmbeddingRequestEncodingFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Float => f.write_str("float"),
            Self::Base64 => f.write_str("base64"),
        }
    }
}
impl ::std::str::FromStr for CreateEmbeddingRequestEncodingFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "float" => Ok(Self::Float),
            "base64" => Ok(Self::Base64),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEmbeddingRequestEncodingFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEmbeddingRequestEncodingFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for all embedding models), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. In addition to the per-input token limit, all embedding models enforce a maximum of 300,000 tokens summed across all inputs in a single request.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
    Variant0(::std::string::String),
    Variant1(::std::vec::Vec<::std::string::String>),
    Variant2(::std::vec::Vec<i64>),
    Variant3(::std::vec::Vec<::std::vec::Vec<i64>>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CreateEmbeddingRequestInput {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<i64>> for CreateEmbeddingRequestInput {
    fn from(value: ::std::vec::Vec<i64>) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::vec::Vec<i64>>> for CreateEmbeddingRequestInput {
    fn from(value: ::std::vec::Vec<::std::vec::Vec<i64>>) -> Self {
        Self::Variant3(value)
    }
}
#[doc = "`CreateEmbeddingResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEmbeddingResponse {
    #[doc = "The list of embeddings generated by the model."]
    pub data: ::std::vec::Vec<Embedding>,
    #[doc = "The name of the model used to generate the embedding."]
    pub model: ::std::string::String,
    #[doc = "The object type, which is always \"list\"."]
    pub object: CreateEmbeddingResponseObject,
    pub usage: CreateEmbeddingResponseUsage,
}
#[doc = "The object type, which is always \"list\"."]
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
pub enum CreateEmbeddingResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for CreateEmbeddingResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for CreateEmbeddingResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEmbeddingResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEmbeddingResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The usage information for the request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEmbeddingResponseUsage {
    #[doc = "The number of tokens used by the prompt."]
    pub prompt_tokens: i64,
    #[doc = "The total number of tokens used by the request."]
    pub total_tokens: i64,
}
#[doc = "Represents an embedding vector returned by embedding endpoint.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Embedding {
    #[doc = "The embedding vector, returned as a list of floats when `encoding_format` is `float` (the default), or as a base64-encoded string when `encoding_format` is `base64`. The length of the vector depends on the model as listed in the [embedding guide](https://developers.openai.com/api/docs/guides/embeddings).\n"]
    pub embedding: EmbeddingEmbedding,
    #[doc = "The index of the embedding in the list of embeddings."]
    pub index: i64,
    #[doc = "The object type, which is always \"embedding\"."]
    pub object: EmbeddingObject,
}
#[doc = "The embedding vector, returned as a list of floats when `encoding_format` is `float` (the default), or as a base64-encoded string when `encoding_format` is `base64`. The length of the vector depends on the model as listed in the [embedding guide](https://developers.openai.com/api/docs/guides/embeddings).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EmbeddingEmbedding {
    Array(::std::vec::Vec<f32>),
    String(::std::string::String),
}
impl ::std::convert::From<::std::vec::Vec<f32>> for EmbeddingEmbedding {
    fn from(value: ::std::vec::Vec<f32>) -> Self {
        Self::Array(value)
    }
}
#[doc = "The object type, which is always \"embedding\"."]
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
pub enum EmbeddingObject {
    #[serde(rename = "embedding")]
    Embedding,
}
impl ::std::fmt::Display for EmbeddingObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Embedding => f.write_str("embedding"),
        }
    }
}
impl ::std::str::FromStr for EmbeddingObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "embedding" => Ok(Self::Embedding),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EmbeddingObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EmbeddingObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EmbeddingsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EmbeddingsApiSchema {
    #[serde(rename = "CreateEmbeddingRequest")]
    pub create_embedding_request: CreateEmbeddingRequest,
    #[serde(rename = "CreateEmbeddingResponse")]
    pub create_embedding_response: CreateEmbeddingResponse,
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
