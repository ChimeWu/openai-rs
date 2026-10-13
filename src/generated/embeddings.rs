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
impl CreateEmbeddingRequest {
    pub fn builder() -> builder::CreateEmbeddingRequest {
        Default::default()
    }
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
impl CreateEmbeddingResponse {
    pub fn builder() -> builder::CreateEmbeddingResponse {
        Default::default()
    }
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
impl CreateEmbeddingResponseUsage {
    pub fn builder() -> builder::CreateEmbeddingResponseUsage {
        Default::default()
    }
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
impl Embedding {
    pub fn builder() -> builder::Embedding {
        Default::default()
    }
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
impl EmbeddingsApiSchema {
    pub fn builder() -> builder::EmbeddingsApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateEmbeddingRequest {
        dimensions: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        encoding_format: ::std::result::Result<
            ::std::option::Option<super::CreateEmbeddingRequestEncodingFormat>,
            ::std::string::String,
        >,
        input: ::std::result::Result<super::CreateEmbeddingRequestInput, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        user: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEmbeddingRequest {
        fn default() -> Self {
            Self {
                dimensions: Ok(Default::default()),
                encoding_format: Ok(Default::default()),
                input: Err("no value supplied for input".to_string()),
                model: Err("no value supplied for model".to_string()),
                user: Ok(Default::default()),
            }
        }
    }
    impl CreateEmbeddingRequest {
        pub fn dimensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dimensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dimensions: {e}"));
            self
        }
        pub fn encoding_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEmbeddingRequestEncodingFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.encoding_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for encoding_format: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEmbeddingRequestInput>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEmbeddingRequest> for super::CreateEmbeddingRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEmbeddingRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dimensions: value.dimensions?,
                encoding_format: value.encoding_format?,
                input: value.input?,
                model: value.model?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEmbeddingRequest> for CreateEmbeddingRequest {
        fn from(value: super::CreateEmbeddingRequest) -> Self {
            Self {
                dimensions: Ok(value.dimensions),
                encoding_format: Ok(value.encoding_format),
                input: Ok(value.input),
                model: Ok(value.model),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEmbeddingResponse {
        data: ::std::result::Result<::std::vec::Vec<super::Embedding>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::CreateEmbeddingResponseObject, ::std::string::String>,
        usage: ::std::result::Result<super::CreateEmbeddingResponseUsage, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEmbeddingResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl CreateEmbeddingResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Embedding>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEmbeddingResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEmbeddingResponseUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEmbeddingResponse> for super::CreateEmbeddingResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEmbeddingResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                model: value.model?,
                object: value.object?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEmbeddingResponse> for CreateEmbeddingResponse {
        fn from(value: super::CreateEmbeddingResponse) -> Self {
            Self {
                data: Ok(value.data),
                model: Ok(value.model),
                object: Ok(value.object),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEmbeddingResponseUsage {
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEmbeddingResponseUsage {
        fn default() -> Self {
            Self {
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl CreateEmbeddingResponseUsage {
        pub fn prompt_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_tokens: {e}"));
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
    impl ::std::convert::TryFrom<CreateEmbeddingResponseUsage> for super::CreateEmbeddingResponseUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEmbeddingResponseUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt_tokens: value.prompt_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEmbeddingResponseUsage> for CreateEmbeddingResponseUsage {
        fn from(value: super::CreateEmbeddingResponseUsage) -> Self {
            Self {
                prompt_tokens: Ok(value.prompt_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Embedding {
        embedding: ::std::result::Result<super::EmbeddingEmbedding, ::std::string::String>,
        index: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::EmbeddingObject, ::std::string::String>,
    }
    impl ::std::default::Default for Embedding {
        fn default() -> Self {
            Self {
                embedding: Err("no value supplied for embedding".to_string()),
                index: Err("no value supplied for index".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl Embedding {
        pub fn embedding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EmbeddingEmbedding>,
            T::Error: ::std::fmt::Display,
        {
            self.embedding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for embedding: {e}"));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EmbeddingObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Embedding> for super::Embedding {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Embedding,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                embedding: value.embedding?,
                index: value.index?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::Embedding> for Embedding {
        fn from(value: super::Embedding) -> Self {
            Self {
                embedding: Ok(value.embedding),
                index: Ok(value.index),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmbeddingsApiSchema {
        create_embedding_request:
            ::std::result::Result<super::CreateEmbeddingRequest, ::std::string::String>,
        create_embedding_response:
            ::std::result::Result<super::CreateEmbeddingResponse, ::std::string::String>,
    }
    impl ::std::default::Default for EmbeddingsApiSchema {
        fn default() -> Self {
            Self {
                create_embedding_request: Err(
                    "no value supplied for create_embedding_request".to_string()
                ),
                create_embedding_response: Err(
                    "no value supplied for create_embedding_response".to_string()
                ),
            }
        }
    }
    impl EmbeddingsApiSchema {
        pub fn create_embedding_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEmbeddingRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_embedding_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_embedding_request: {e}")
            });
            self
        }
        pub fn create_embedding_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEmbeddingResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_embedding_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_embedding_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<EmbeddingsApiSchema> for super::EmbeddingsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmbeddingsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_embedding_request: value.create_embedding_request?,
                create_embedding_response: value.create_embedding_response?,
            })
        }
    }
    impl ::std::convert::From<super::EmbeddingsApiSchema> for EmbeddingsApiSchema {
        fn from(value: super::EmbeddingsApiSchema) -> Self {
            Self {
                create_embedding_request: Ok(value.create_embedding_request),
                create_embedding_response: Ok(value.create_embedding_response),
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
