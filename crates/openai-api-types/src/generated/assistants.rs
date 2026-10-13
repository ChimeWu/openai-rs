pub use crate::common::{Error, ErrorResponse, FunctionObject, FunctionParameters, Metadata, MisalignmentErrorDetailsResource, ParallelToolCalls, ReasoningEffort, ResponseFormatJsonObject, ResponseFormatJsonSchema, ResponseFormatJsonSchemaSchema, ResponseFormatText, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`ArrayOfContentPartsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ArrayOfContentPartsItem {
    ContentImageFileObject(MessageContentImageFileObject),
    ContentImageUrlObject(MessageContentImageUrlObject),
    RequestContentTextObject(MessageRequestContentTextObject),
}
impl ::std::convert::From<MessageContentImageFileObject> for ArrayOfContentPartsItem {
    fn from(value: MessageContentImageFileObject) -> Self {
        Self::ContentImageFileObject(value)
    }
}
impl ::std::convert::From<MessageContentImageUrlObject> for ArrayOfContentPartsItem {
    fn from(value: MessageContentImageUrlObject) -> Self {
        Self::ContentImageUrlObject(value)
    }
}
impl ::std::convert::From<MessageRequestContentTextObject> for ArrayOfContentPartsItem {
    fn from(value: MessageRequestContentTextObject) -> Self {
        Self::RequestContentTextObject(value)
    }
}
#[doc = "Represents an `assistant` that can call the model and use tools."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantObject {
    #[doc = "The Unix timestamp (in seconds) for when the assistant was created."]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<AssistantObjectDescription>,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub instructions: ::std::option::Option<AssistantObjectInstructions>,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
    pub model: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<AssistantObjectName>,
    #[doc = "The object type, which is always `assistant`."]
    pub object: AssistantObjectObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<AssistantObjectToolResources>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    pub tools: ::std::vec::Vec<AssistantObjectToolsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
impl AssistantObject {
    pub fn builder() -> builder::AssistantObject {
        Default::default()
    }
}
#[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AssistantObjectDescription(::std::string::String);
impl ::std::ops::Deref for AssistantObjectDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AssistantObjectDescription> for ::std::string::String {
    fn from(value: AssistantObjectDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AssistantObjectDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AssistantObjectDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantObjectDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AssistantObjectDescription {
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
#[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AssistantObjectInstructions(::std::string::String);
impl ::std::ops::Deref for AssistantObjectInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AssistantObjectInstructions> for ::std::string::String {
    fn from(value: AssistantObjectInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AssistantObjectInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256000usize {
            return Err("longer than 256000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AssistantObjectInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantObjectInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AssistantObjectInstructions {
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
#[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AssistantObjectName(::std::string::String);
impl ::std::ops::Deref for AssistantObjectName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AssistantObjectName> for ::std::string::String {
    fn from(value: AssistantObjectName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AssistantObjectName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AssistantObjectName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantObjectName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AssistantObjectName {
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
#[doc = "The object type, which is always `assistant`."]
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
pub enum AssistantObjectObject {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for AssistantObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for AssistantObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AssistantObjectToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<AssistantObjectToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<AssistantObjectToolResourcesFileSearch>,
}
impl AssistantObjectToolResources {
    pub fn builder() -> builder::AssistantObjectToolResources {
        Default::default()
    }
}
#[doc = "`AssistantObjectToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AssistantObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl AssistantObjectToolResourcesCodeInterpreter {
    pub fn builder() -> builder::AssistantObjectToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`AssistantObjectToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AssistantObjectToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl AssistantObjectToolResourcesFileSearch {
    pub fn builder() -> builder::AssistantObjectToolResourcesFileSearch {
        Default::default()
    }
}
#[doc = "`AssistantObjectToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssistantObjectToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for AssistantObjectToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for AssistantObjectToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for AssistantObjectToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "Represents an event emitted when streaming a Run.\n\nEach event in a server-sent events stream has an `event` and `data` property:\n\n```\nevent: thread.created\ndata: {\"id\": \"thread_123\", \"object\": \"thread\", ...}\n```\n\nWe emit events whenever a new object is created, transitions to a new state, or is being\nstreamed in parts (deltas). For example, we emit `thread.run.created` when a new run\nis created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses\nto create a message during a run, we emit a `thread.message.created event`, a\n`thread.message.in_progress` event, many `thread.message.delta` events, and finally a\n`thread.message.completed` event.\n\nWe may add additional events over time, so we recommend handling unknown events gracefully\nin your code. See the [Assistants API quickstart](https://developers.openai.com/api/docs/assistants/migration) to learn how to\nintegrate the Assistants API with streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssistantStreamEvent {
    ThreadStreamEvent(ThreadStreamEvent),
    RunStreamEvent(RunStreamEvent),
    RunStepStreamEvent(RunStepStreamEvent),
    MessageStreamEvent(MessageStreamEvent),
    ErrorEvent(ErrorEvent),
    DoneEvent(DoneEvent),
}
impl ::std::convert::From<ThreadStreamEvent> for AssistantStreamEvent {
    fn from(value: ThreadStreamEvent) -> Self {
        Self::ThreadStreamEvent(value)
    }
}
impl ::std::convert::From<RunStreamEvent> for AssistantStreamEvent {
    fn from(value: RunStreamEvent) -> Self {
        Self::RunStreamEvent(value)
    }
}
impl ::std::convert::From<RunStepStreamEvent> for AssistantStreamEvent {
    fn from(value: RunStepStreamEvent) -> Self {
        Self::RunStepStreamEvent(value)
    }
}
impl ::std::convert::From<MessageStreamEvent> for AssistantStreamEvent {
    fn from(value: MessageStreamEvent) -> Self {
        Self::MessageStreamEvent(value)
    }
}
impl ::std::convert::From<ErrorEvent> for AssistantStreamEvent {
    fn from(value: ErrorEvent) -> Self {
        Self::ErrorEvent(value)
    }
}
impl ::std::convert::From<DoneEvent> for AssistantStreamEvent {
    fn from(value: DoneEvent) -> Self {
        Self::DoneEvent(value)
    }
}
#[doc = "`AssistantSupportedModels`"]
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
pub enum AssistantSupportedModels {
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-mini")]
    Gpt5Mini,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,
    #[serde(rename = "gpt-5-2025-08-07")]
    Gpt520250807,
    #[serde(rename = "gpt-5-mini-2025-08-07")]
    Gpt5Mini20250807,
    #[serde(rename = "gpt-5-nano-2025-08-07")]
    Gpt5Nano20250807,
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt41Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano,
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt41Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt41Nano20250414,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3-mini-2025-01-31")]
    O3Mini20250131,
    #[serde(rename = "o1")]
    O1,
    #[serde(rename = "o1-2024-12-17")]
    O120241217,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt45Preview,
    #[serde(rename = "gpt-4.5-preview-2025-02-27")]
    Gpt45Preview20250227,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt35Turbo16k0613,
}
impl ::std::fmt::Display for AssistantSupportedModels {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gpt5 => f.write_str("gpt-5"),
            Self::Gpt5Mini => f.write_str("gpt-5-mini"),
            Self::Gpt5Nano => f.write_str("gpt-5-nano"),
            Self::Gpt520250807 => f.write_str("gpt-5-2025-08-07"),
            Self::Gpt5Mini20250807 => f.write_str("gpt-5-mini-2025-08-07"),
            Self::Gpt5Nano20250807 => f.write_str("gpt-5-nano-2025-08-07"),
            Self::Gpt41 => f.write_str("gpt-4.1"),
            Self::Gpt41Mini => f.write_str("gpt-4.1-mini"),
            Self::Gpt41Nano => f.write_str("gpt-4.1-nano"),
            Self::Gpt4120250414 => f.write_str("gpt-4.1-2025-04-14"),
            Self::Gpt41Mini20250414 => f.write_str("gpt-4.1-mini-2025-04-14"),
            Self::Gpt41Nano20250414 => f.write_str("gpt-4.1-nano-2025-04-14"),
            Self::O3Mini => f.write_str("o3-mini"),
            Self::O3Mini20250131 => f.write_str("o3-mini-2025-01-31"),
            Self::O1 => f.write_str("o1"),
            Self::O120241217 => f.write_str("o1-2024-12-17"),
            Self::Gpt4o => f.write_str("gpt-4o"),
            Self::Gpt4o20241120 => f.write_str("gpt-4o-2024-11-20"),
            Self::Gpt4o20240806 => f.write_str("gpt-4o-2024-08-06"),
            Self::Gpt4o20240513 => f.write_str("gpt-4o-2024-05-13"),
            Self::Gpt4oMini => f.write_str("gpt-4o-mini"),
            Self::Gpt4oMini20240718 => f.write_str("gpt-4o-mini-2024-07-18"),
            Self::Gpt45Preview => f.write_str("gpt-4.5-preview"),
            Self::Gpt45Preview20250227 => f.write_str("gpt-4.5-preview-2025-02-27"),
            Self::Gpt4Turbo => f.write_str("gpt-4-turbo"),
            Self::Gpt4Turbo20240409 => f.write_str("gpt-4-turbo-2024-04-09"),
            Self::Gpt40125Preview => f.write_str("gpt-4-0125-preview"),
            Self::Gpt4TurboPreview => f.write_str("gpt-4-turbo-preview"),
            Self::Gpt41106Preview => f.write_str("gpt-4-1106-preview"),
            Self::Gpt4VisionPreview => f.write_str("gpt-4-vision-preview"),
            Self::Gpt4 => f.write_str("gpt-4"),
            Self::Gpt40314 => f.write_str("gpt-4-0314"),
            Self::Gpt40613 => f.write_str("gpt-4-0613"),
            Self::Gpt432k => f.write_str("gpt-4-32k"),
            Self::Gpt432k0314 => f.write_str("gpt-4-32k-0314"),
            Self::Gpt432k0613 => f.write_str("gpt-4-32k-0613"),
            Self::Gpt35Turbo => f.write_str("gpt-3.5-turbo"),
            Self::Gpt35Turbo16k => f.write_str("gpt-3.5-turbo-16k"),
            Self::Gpt35Turbo0613 => f.write_str("gpt-3.5-turbo-0613"),
            Self::Gpt35Turbo1106 => f.write_str("gpt-3.5-turbo-1106"),
            Self::Gpt35Turbo0125 => f.write_str("gpt-3.5-turbo-0125"),
            Self::Gpt35Turbo16k0613 => f.write_str("gpt-3.5-turbo-16k-0613"),
        }
    }
}
impl ::std::str::FromStr for AssistantSupportedModels {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gpt-5" => Ok(Self::Gpt5),
            "gpt-5-mini" => Ok(Self::Gpt5Mini),
            "gpt-5-nano" => Ok(Self::Gpt5Nano),
            "gpt-5-2025-08-07" => Ok(Self::Gpt520250807),
            "gpt-5-mini-2025-08-07" => Ok(Self::Gpt5Mini20250807),
            "gpt-5-nano-2025-08-07" => Ok(Self::Gpt5Nano20250807),
            "gpt-4.1" => Ok(Self::Gpt41),
            "gpt-4.1-mini" => Ok(Self::Gpt41Mini),
            "gpt-4.1-nano" => Ok(Self::Gpt41Nano),
            "gpt-4.1-2025-04-14" => Ok(Self::Gpt4120250414),
            "gpt-4.1-mini-2025-04-14" => Ok(Self::Gpt41Mini20250414),
            "gpt-4.1-nano-2025-04-14" => Ok(Self::Gpt41Nano20250414),
            "o3-mini" => Ok(Self::O3Mini),
            "o3-mini-2025-01-31" => Ok(Self::O3Mini20250131),
            "o1" => Ok(Self::O1),
            "o1-2024-12-17" => Ok(Self::O120241217),
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4o-2024-11-20" => Ok(Self::Gpt4o20241120),
            "gpt-4o-2024-08-06" => Ok(Self::Gpt4o20240806),
            "gpt-4o-2024-05-13" => Ok(Self::Gpt4o20240513),
            "gpt-4o-mini" => Ok(Self::Gpt4oMini),
            "gpt-4o-mini-2024-07-18" => Ok(Self::Gpt4oMini20240718),
            "gpt-4.5-preview" => Ok(Self::Gpt45Preview),
            "gpt-4.5-preview-2025-02-27" => Ok(Self::Gpt45Preview20250227),
            "gpt-4-turbo" => Ok(Self::Gpt4Turbo),
            "gpt-4-turbo-2024-04-09" => Ok(Self::Gpt4Turbo20240409),
            "gpt-4-0125-preview" => Ok(Self::Gpt40125Preview),
            "gpt-4-turbo-preview" => Ok(Self::Gpt4TurboPreview),
            "gpt-4-1106-preview" => Ok(Self::Gpt41106Preview),
            "gpt-4-vision-preview" => Ok(Self::Gpt4VisionPreview),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-0314" => Ok(Self::Gpt40314),
            "gpt-4-0613" => Ok(Self::Gpt40613),
            "gpt-4-32k" => Ok(Self::Gpt432k),
            "gpt-4-32k-0314" => Ok(Self::Gpt432k0314),
            "gpt-4-32k-0613" => Ok(Self::Gpt432k0613),
            "gpt-3.5-turbo" => Ok(Self::Gpt35Turbo),
            "gpt-3.5-turbo-16k" => Ok(Self::Gpt35Turbo16k),
            "gpt-3.5-turbo-0613" => Ok(Self::Gpt35Turbo0613),
            "gpt-3.5-turbo-1106" => Ok(Self::Gpt35Turbo1106),
            "gpt-3.5-turbo-0125" => Ok(Self::Gpt35Turbo0125),
            "gpt-3.5-turbo-16k-0613" => Ok(Self::Gpt35Turbo16k0613),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantSupportedModels {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AssistantToolsCode`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantToolsCode {
    #[doc = "The type of tool being defined: `code_interpreter`"]
    #[serde(rename = "type")]
    pub type_: AssistantToolsCodeType,
}
impl AssistantToolsCode {
    pub fn builder() -> builder::AssistantToolsCode {
        Default::default()
    }
}
#[doc = "The type of tool being defined: `code_interpreter`"]
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
pub enum AssistantToolsCodeType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for AssistantToolsCodeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for AssistantToolsCodeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantToolsCodeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantToolsCodeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AssistantToolsFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantToolsFileSearch {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<AssistantToolsFileSearchFileSearch>,
    #[doc = "The type of tool being defined: `file_search`"]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFileSearchType,
}
impl AssistantToolsFileSearch {
    pub fn builder() -> builder::AssistantToolsFileSearch {
        Default::default()
    }
}
#[doc = "Overrides for the file search tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AssistantToolsFileSearchFileSearch {
    #[doc = "The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.\n\nNote that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](https://developers.openai.com/api/docs/guides/tools-file-search#retrieval-customization) for more information.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_num_results: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranking_options: ::std::option::Option<FileSearchRankingOptions>,
}
impl AssistantToolsFileSearchFileSearch {
    pub fn builder() -> builder::AssistantToolsFileSearchFileSearch {
        Default::default()
    }
}
#[doc = "The type of tool being defined: `file_search`"]
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
pub enum AssistantToolsFileSearchType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for AssistantToolsFileSearchType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for AssistantToolsFileSearchType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantToolsFileSearchType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantToolsFileSearchType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AssistantToolsFileSearchTypeOnly`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantToolsFileSearchTypeOnly {
    #[doc = "The type of tool being defined: `file_search`"]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFileSearchTypeOnlyType,
}
impl AssistantToolsFileSearchTypeOnly {
    pub fn builder() -> builder::AssistantToolsFileSearchTypeOnly {
        Default::default()
    }
}
#[doc = "The type of tool being defined: `file_search`"]
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
pub enum AssistantToolsFileSearchTypeOnlyType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for AssistantToolsFileSearchTypeOnlyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for AssistantToolsFileSearchTypeOnlyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantToolsFileSearchTypeOnlyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantToolsFileSearchTypeOnlyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AssistantToolsFunction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantToolsFunction {
    pub function: ::openai_api_types::common::FunctionObject,
    #[doc = "The type of tool being defined: `function`"]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFunctionType,
}
impl AssistantToolsFunction {
    pub fn builder() -> builder::AssistantToolsFunction {
        Default::default()
    }
}
#[doc = "The type of tool being defined: `function`"]
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
pub enum AssistantToolsFunctionType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for AssistantToolsFunctionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for AssistantToolsFunctionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantToolsFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantToolsFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Specifies the format that the model must output. Compatible with [GPT-4o](https://developers.openai.com/api/docs/models/gpt-4o), [GPT-4 Turbo](https://developers.openai.com/api/docs/models/gpt-4-turbo), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables JSON mode, which ensures the message the model generates is valid JSON.\n\n**Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly \"stuck\" request. Also note that the message content may be partially cut off if `finish_reason=\"length\"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
    Variant0(AssistantsApiResponseFormatOptionVariant0),
    Variant1(::openai_api_types::common::ResponseFormatText),
    Variant2(::openai_api_types::common::ResponseFormatJsonObject),
    Variant3(::openai_api_types::common::ResponseFormatJsonSchema),
}
impl ::std::convert::From<AssistantsApiResponseFormatOptionVariant0>
    for AssistantsApiResponseFormatOption
{
    fn from(value: AssistantsApiResponseFormatOptionVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatText>
    for AssistantsApiResponseFormatOption
{
    fn from(value: ::openai_api_types::common::ResponseFormatText) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatJsonObject>
    for AssistantsApiResponseFormatOption
{
    fn from(value: ::openai_api_types::common::ResponseFormatJsonObject) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatJsonSchema>
    for AssistantsApiResponseFormatOption
{
    fn from(value: ::openai_api_types::common::ResponseFormatJsonSchema) -> Self {
        Self::Variant3(value)
    }
}
#[doc = "`auto` is the default value\n"]
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
pub enum AssistantsApiResponseFormatOptionVariant0 {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for AssistantsApiResponseFormatOptionVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for AssistantsApiResponseFormatOptionVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantsApiResponseFormatOptionVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantsApiResponseFormatOptionVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AssistantsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantsApiSchema {
    #[serde(rename = "CancelRunError429")]
    pub cancel_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelRunParams")]
    pub cancel_run_params: CancelRunParams,
    #[serde(rename = "CancelRunResponse")]
    pub cancel_run_response: RunObject,
    #[serde(rename = "CreateAssistantError429")]
    pub create_assistant_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateAssistantRequest")]
    pub create_assistant_request: CreateAssistantRequest,
    #[serde(rename = "CreateAssistantResponse")]
    pub create_assistant_response: AssistantObject,
    #[serde(rename = "CreateMessageError429")]
    pub create_message_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateMessageParams")]
    pub create_message_params: CreateMessageParams,
    #[serde(rename = "CreateMessageRequest")]
    pub create_message_request: CreateMessageRequest,
    #[serde(rename = "CreateMessageResponse")]
    pub create_message_response: MessageObject,
    #[serde(rename = "CreateRunError429")]
    pub create_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateRunParams")]
    pub create_run_params: CreateRunParams,
    #[serde(rename = "CreateRunRequest")]
    pub create_run_request: CreateRunRequest,
    #[serde(rename = "CreateRunResponse")]
    pub create_run_response: RunObject,
    #[serde(rename = "CreateRunStreamResponse")]
    pub create_run_stream_response: AssistantStreamEvent,
    #[serde(rename = "CreateThreadAndRunError429")]
    pub create_thread_and_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateThreadAndRunRequest")]
    pub create_thread_and_run_request: CreateThreadAndRunRequest,
    #[serde(rename = "CreateThreadAndRunResponse")]
    pub create_thread_and_run_response: RunObject,
    #[serde(rename = "CreateThreadAndRunStreamResponse")]
    pub create_thread_and_run_stream_response: AssistantStreamEvent,
    #[serde(rename = "CreateThreadError429")]
    pub create_thread_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateThreadRequest")]
    pub create_thread_request: CreateThreadRequest,
    #[serde(rename = "CreateThreadResponse")]
    pub create_thread_response: ThreadObject,
    #[serde(rename = "DeleteAssistantError429")]
    pub delete_assistant_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteAssistantParams")]
    pub delete_assistant_params: DeleteAssistantParams,
    #[serde(rename = "DeleteAssistantResponse")]
    pub delete_assistant_response: DeleteAssistantResponse,
    #[serde(rename = "DeleteMessageError429")]
    pub delete_message_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteMessageParams")]
    pub delete_message_params: DeleteMessageParams,
    #[serde(rename = "DeleteMessageResponse")]
    pub delete_message_response: DeleteMessageResponse,
    #[serde(rename = "DeleteThreadError429")]
    pub delete_thread_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteThreadParams")]
    pub delete_thread_params: DeleteThreadParams,
    #[serde(rename = "DeleteThreadResponse")]
    pub delete_thread_response: DeleteThreadResponse,
    #[serde(rename = "GetAssistantError429")]
    pub get_assistant_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetAssistantParams")]
    pub get_assistant_params: GetAssistantParams,
    #[serde(rename = "GetAssistantResponse")]
    pub get_assistant_response: AssistantObject,
    #[serde(rename = "GetMessageError429")]
    pub get_message_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetMessageParams")]
    pub get_message_params: GetMessageParams,
    #[serde(rename = "GetMessageResponse")]
    pub get_message_response: MessageObject,
    #[serde(rename = "GetRunError429")]
    pub get_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetRunParams")]
    pub get_run_params: GetRunParams,
    #[serde(rename = "GetRunResponse")]
    pub get_run_response: RunObject,
    #[serde(rename = "GetRunStepError429")]
    pub get_run_step_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetRunStepParams")]
    pub get_run_step_params: GetRunStepParams,
    #[serde(rename = "GetRunStepResponse")]
    pub get_run_step_response: RunStepObject,
    #[serde(rename = "GetThreadError429")]
    pub get_thread_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetThreadParams")]
    pub get_thread_params: GetThreadParams,
    #[serde(rename = "GetThreadResponse")]
    pub get_thread_response: ThreadObject,
    #[serde(rename = "ListAssistantsError429")]
    pub list_assistants_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListAssistantsParams")]
    pub list_assistants_params: ListAssistantsParams,
    #[serde(rename = "ListAssistantsResponse")]
    pub list_assistants_response: ListAssistantsResponse,
    #[serde(rename = "ListMessagesError429")]
    pub list_messages_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListMessagesParams")]
    pub list_messages_params: ListMessagesParams,
    #[serde(rename = "ListMessagesResponse")]
    pub list_messages_response: ListMessagesResponse,
    #[serde(rename = "ListRunStepsError429")]
    pub list_run_steps_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListRunStepsParams")]
    pub list_run_steps_params: ListRunStepsParams,
    #[serde(rename = "ListRunStepsResponse")]
    pub list_run_steps_response: ListRunStepsResponse,
    #[serde(rename = "ListRunsError429")]
    pub list_runs_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListRunsParams")]
    pub list_runs_params: ListRunsParams,
    #[serde(rename = "ListRunsResponse")]
    pub list_runs_response: ListRunsResponse,
    #[serde(rename = "ModifyAssistantError429")]
    pub modify_assistant_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyAssistantParams")]
    pub modify_assistant_params: ModifyAssistantParams,
    #[serde(rename = "ModifyAssistantRequest")]
    pub modify_assistant_request: ModifyAssistantRequest,
    #[serde(rename = "ModifyAssistantResponse")]
    pub modify_assistant_response: AssistantObject,
    #[serde(rename = "ModifyMessageError429")]
    pub modify_message_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyMessageParams")]
    pub modify_message_params: ModifyMessageParams,
    #[serde(rename = "ModifyMessageRequest")]
    pub modify_message_request: ModifyMessageRequest,
    #[serde(rename = "ModifyMessageResponse")]
    pub modify_message_response: MessageObject,
    #[serde(rename = "ModifyRunError429")]
    pub modify_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyRunParams")]
    pub modify_run_params: ModifyRunParams,
    #[serde(rename = "ModifyRunRequest")]
    pub modify_run_request: ModifyRunRequest,
    #[serde(rename = "ModifyRunResponse")]
    pub modify_run_response: RunObject,
    #[serde(rename = "ModifyThreadError429")]
    pub modify_thread_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyThreadParams")]
    pub modify_thread_params: ModifyThreadParams,
    #[serde(rename = "ModifyThreadRequest")]
    pub modify_thread_request: ModifyThreadRequest,
    #[serde(rename = "ModifyThreadResponse")]
    pub modify_thread_response: ThreadObject,
    #[serde(rename = "SubmitToolOuputsToRunError429")]
    pub submit_tool_ouputs_to_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "SubmitToolOuputsToRunParams")]
    pub submit_tool_ouputs_to_run_params: SubmitToolOuputsToRunParams,
    #[serde(rename = "SubmitToolOuputsToRunRequest")]
    pub submit_tool_ouputs_to_run_request: SubmitToolOutputsRunRequest,
    #[serde(rename = "SubmitToolOuputsToRunResponse")]
    pub submit_tool_ouputs_to_run_response: RunObject,
    #[serde(rename = "SubmitToolOuputsToRunStreamResponse")]
    pub submit_tool_ouputs_to_run_stream_response: AssistantStreamEvent,
}
impl AssistantsApiSchema {
    pub fn builder() -> builder::AssistantsApiSchema {
        Default::default()
    }
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tools and instead generates a message.\n`auto` is the default value and means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools before responding to the user.\nSpecifying a particular tool like `{\"type\": \"file_search\"}` or `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
    String(AssistantsApiToolChoiceOptionString),
    AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}
impl ::std::convert::From<AssistantsApiToolChoiceOptionString> for AssistantsApiToolChoiceOption {
    fn from(value: AssistantsApiToolChoiceOptionString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<AssistantsNamedToolChoice> for AssistantsApiToolChoiceOption {
    fn from(value: AssistantsNamedToolChoice) -> Self {
        Self::AssistantsNamedToolChoice(value)
    }
}
#[doc = "`none` means the model will not call any tools and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools before responding to the user.\n"]
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
pub enum AssistantsApiToolChoiceOptionString {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for AssistantsApiToolChoiceOptionString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for AssistantsApiToolChoiceOptionString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantsApiToolChoiceOptionString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantsApiToolChoiceOptionString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantsNamedToolChoice {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function: ::std::option::Option<AssistantsNamedToolChoiceFunction>,
    #[doc = "The type of the tool. If type is `function`, the function name must be set"]
    #[serde(rename = "type")]
    pub type_: AssistantsNamedToolChoiceType,
}
impl AssistantsNamedToolChoice {
    pub fn builder() -> builder::AssistantsNamedToolChoice {
        Default::default()
    }
}
#[doc = "`AssistantsNamedToolChoiceFunction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantsNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
impl AssistantsNamedToolChoiceFunction {
    pub fn builder() -> builder::AssistantsNamedToolChoiceFunction {
        Default::default()
    }
}
#[doc = "The type of the tool. If type is `function`, the function name must be set"]
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
pub enum AssistantsNamedToolChoiceType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for AssistantsNamedToolChoiceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
            Self::CodeInterpreter => f.write_str("code_interpreter"),
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for AssistantsNamedToolChoiceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            "code_interpreter" => Ok(Self::CodeInterpreter),
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantsNamedToolChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantsNamedToolChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CancelRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CancelRunParams {
    pub run_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl CancelRunParams {
    pub fn builder() -> builder::CancelRunParams {
        Default::default()
    }
}
#[doc = "`CreateAssistantRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateAssistantRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<CreateAssistantRequestDescription>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<CreateAssistantRequestInstructions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
    pub model: CreateAssistantRequestModel,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<CreateAssistantRequestName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<CreateAssistantRequestToolResources>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<CreateAssistantRequestToolsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
impl CreateAssistantRequest {
    pub fn builder() -> builder::CreateAssistantRequest {
        Default::default()
    }
}
#[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateAssistantRequestDescription(::std::string::String);
impl ::std::ops::Deref for CreateAssistantRequestDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateAssistantRequestDescription> for ::std::string::String {
    fn from(value: CreateAssistantRequestDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateAssistantRequestDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateAssistantRequestDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateAssistantRequestDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateAssistantRequestDescription {
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
#[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateAssistantRequestInstructions(::std::string::String);
impl ::std::ops::Deref for CreateAssistantRequestInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateAssistantRequestInstructions> for ::std::string::String {
    fn from(value: CreateAssistantRequestInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateAssistantRequestInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256000usize {
            return Err("longer than 256000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateAssistantRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateAssistantRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateAssistantRequestInstructions {
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
#[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateAssistantRequestModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<AssistantSupportedModels>,
}
impl CreateAssistantRequestModel {
    pub fn builder() -> builder::CreateAssistantRequestModel {
        Default::default()
    }
}
#[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateAssistantRequestName(::std::string::String);
impl ::std::ops::Deref for CreateAssistantRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateAssistantRequestName> for ::std::string::String {
    fn from(value: CreateAssistantRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateAssistantRequestName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateAssistantRequestName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateAssistantRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateAssistantRequestName {
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
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateAssistantRequestToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<CreateAssistantRequestToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<CreateAssistantRequestToolResourcesFileSearch>,
}
impl CreateAssistantRequestToolResources {
    pub fn builder() -> builder::CreateAssistantRequestToolResources {
        Default::default()
    }
}
#[doc = "`CreateAssistantRequestToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl CreateAssistantRequestToolResourcesCodeInterpreter {
    pub fn builder() -> builder::CreateAssistantRequestToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`CreateAssistantRequestToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateAssistantRequestToolResourcesFileSearch {
    Variant0 {
        #[doc = "The [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
        vector_store_ids: ::std::vec::Vec<::std::string::String>,
    },
    Variant1 {
        #[doc = "A helper to create a [vector store](https://developers.openai.com/api/reference/resources/vector_stores) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
        vector_stores:
            ::std::vec::Vec<CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem>,
    },
}
#[doc = "`CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<
        CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy,
    >,
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs to add to the vector store. For vector stores created before Nov 2025, there can be a maximum of 10,000 files in a vector store. For vector stores created starting in Nov 2025, the limit is 100,000,000 files.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
}
impl CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem {
    pub fn builder(
    ) -> builder::CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem {
        Default::default()
    }
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "static", deny_unknown_fields)]
pub enum CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy {
    #[serde(rename = "auto")]
    Auto,
    #[doc = "Static Chunking Strategy"]
    #[serde(rename = "static")]
    Static {
        #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
        chunk_overlap_tokens: i64,
        #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
        max_chunk_size_tokens: i64,
    },
}
#[doc = "`CreateAssistantRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateAssistantRequestToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for CreateAssistantRequestToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for CreateAssistantRequestToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for CreateAssistantRequestToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "`CreateMessageParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateMessageParams {
    pub thread_id: ::std::string::String,
}
impl CreateMessageParams {
    pub fn builder() -> builder::CreateMessageParams {
        Default::default()
    }
}
#[doc = "`CreateMessageRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateMessageRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attachments: ::std::option::Option<::std::vec::Vec<CreateMessageRequestAttachmentsItem>>,
    pub content: CreateMessageRequestContent,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
    pub role: CreateMessageRequestRole,
}
impl CreateMessageRequest {
    pub fn builder() -> builder::CreateMessageRequest {
        Default::default()
    }
}
#[doc = "`CreateMessageRequestAttachmentsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateMessageRequestAttachmentsItem {
    #[doc = "The ID of the file to attach to the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The tools to add this file to."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<CreateMessageRequestAttachmentsItemToolsItem>,
}
impl CreateMessageRequestAttachmentsItem {
    pub fn builder() -> builder::CreateMessageRequestAttachmentsItem {
        Default::default()
    }
}
#[doc = "`CreateMessageRequestAttachmentsItemToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateMessageRequestAttachmentsItemToolsItem {
    Code(AssistantToolsCode),
    FileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}
impl ::std::convert::From<AssistantToolsCode> for CreateMessageRequestAttachmentsItemToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearchTypeOnly>
    for CreateMessageRequestAttachmentsItemToolsItem
{
    fn from(value: AssistantToolsFileSearchTypeOnly) -> Self {
        Self::FileSearchTypeOnly(value)
    }
}
#[doc = "`CreateMessageRequestContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateMessageRequestContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ArrayOfContentPartsItem>),
}
impl ::std::convert::From<::std::vec::Vec<ArrayOfContentPartsItem>>
    for CreateMessageRequestContent
{
    fn from(value: ::std::vec::Vec<ArrayOfContentPartsItem>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
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
pub enum CreateMessageRequestRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for CreateMessageRequestRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for CreateMessageRequestRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMessageRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMessageRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateRunParams {
    #[serde(
        rename = "include[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include: ::std::vec::Vec<CreateRunParamsIncludeItem>,
    pub thread_id: ::std::string::String,
}
impl CreateRunParams {
    pub fn builder() -> builder::CreateRunParams {
        Default::default()
    }
}
#[doc = "`CreateRunParamsIncludeItem`"]
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
pub enum CreateRunParamsIncludeItem {
    #[serde(rename = "step_details.tool_calls[*].file_search.results[*].content")]
    StepDetailsToolCallsFileSearchResultsContent,
}
impl ::std::fmt::Display for CreateRunParamsIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StepDetailsToolCallsFileSearchResultsContent => {
                f.write_str("step_details.tool_calls[*].file_search.results[*].content")
            }
        }
    }
}
impl ::std::str::FromStr for CreateRunParamsIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "step_details.tool_calls[*].file_search.results[*].content" => {
                Ok(Self::StepDetailsToolCallsFileSearchResultsContent)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateRunParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateRunParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateRunRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateRunRequest {
    #[doc = "Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_instructions: ::std::option::Option<::std::string::String>,
    #[doc = "Adds additional messages to the thread before creating the run."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_messages: ::std::option::Option<::std::vec::Vec<CreateMessageRequest>>,
    #[doc = "The ID of the [assistant](https://developers.openai.com/api/docs/assistants/migration) to use to execute this run."]
    pub assistant_id: ::std::string::String,
    #[doc = "Overrides the [instructions](https://developers.openai.com/api/docs/assistants/migration) of the assistant. This is useful for modifying the behavior on a per-run basis."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_prompt_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The ID of the [Model](https://developers.openai.com/api/reference/resources/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<CreateRunRequestModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<AssistantsApiToolChoiceOption>,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<::std::vec::Vec<CreateRunRequestToolsItem>>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation_strategy: ::std::option::Option<TruncationObject>,
}
impl CreateRunRequest {
    pub fn builder() -> builder::CreateRunRequest {
        Default::default()
    }
}
#[doc = "The ID of the [Model](https://developers.openai.com/api/reference/resources/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateRunRequestModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<AssistantSupportedModels>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
impl CreateRunRequestModel {
    pub fn builder() -> builder::CreateRunRequestModel {
        Default::default()
    }
}
#[doc = "`CreateRunRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateRunRequestToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for CreateRunRequestToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for CreateRunRequestToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for CreateRunRequestToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "`CreateThreadAndRunRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateThreadAndRunRequest {
    #[doc = "The ID of the [assistant](https://developers.openai.com/api/docs/assistants/migration) to use to execute this run."]
    pub assistant_id: ::std::string::String,
    #[doc = "Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_prompt_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The ID of the [Model](https://developers.openai.com/api/reference/resources/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<CreateThreadAndRunRequestModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub thread: ::std::option::Option<CreateThreadRequest>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<AssistantsApiToolChoiceOption>,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<CreateThreadAndRunRequestToolResources>,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<::std::vec::Vec<CreateThreadAndRunRequestToolsItem>>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation_strategy: ::std::option::Option<TruncationObject>,
}
impl CreateThreadAndRunRequest {
    pub fn builder() -> builder::CreateThreadAndRunRequest {
        Default::default()
    }
}
#[doc = "The ID of the [Model](https://developers.openai.com/api/reference/resources/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadAndRunRequestModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<CreateThreadAndRunRequestModelSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
impl CreateThreadAndRunRequestModel {
    pub fn builder() -> builder::CreateThreadAndRunRequestModel {
        Default::default()
    }
}
#[doc = "`CreateThreadAndRunRequestModelSubtype1`"]
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
pub enum CreateThreadAndRunRequestModelSubtype1 {
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-mini")]
    Gpt5Mini,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,
    #[serde(rename = "gpt-5-2025-08-07")]
    Gpt520250807,
    #[serde(rename = "gpt-5-mini-2025-08-07")]
    Gpt5Mini20250807,
    #[serde(rename = "gpt-5-nano-2025-08-07")]
    Gpt5Nano20250807,
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt41Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano,
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt41Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt41Nano20250414,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt45Preview,
    #[serde(rename = "gpt-4.5-preview-2025-02-27")]
    Gpt45Preview20250227,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt35Turbo16k0613,
}
impl ::std::fmt::Display for CreateThreadAndRunRequestModelSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gpt5 => f.write_str("gpt-5"),
            Self::Gpt5Mini => f.write_str("gpt-5-mini"),
            Self::Gpt5Nano => f.write_str("gpt-5-nano"),
            Self::Gpt520250807 => f.write_str("gpt-5-2025-08-07"),
            Self::Gpt5Mini20250807 => f.write_str("gpt-5-mini-2025-08-07"),
            Self::Gpt5Nano20250807 => f.write_str("gpt-5-nano-2025-08-07"),
            Self::Gpt41 => f.write_str("gpt-4.1"),
            Self::Gpt41Mini => f.write_str("gpt-4.1-mini"),
            Self::Gpt41Nano => f.write_str("gpt-4.1-nano"),
            Self::Gpt4120250414 => f.write_str("gpt-4.1-2025-04-14"),
            Self::Gpt41Mini20250414 => f.write_str("gpt-4.1-mini-2025-04-14"),
            Self::Gpt41Nano20250414 => f.write_str("gpt-4.1-nano-2025-04-14"),
            Self::Gpt4o => f.write_str("gpt-4o"),
            Self::Gpt4o20241120 => f.write_str("gpt-4o-2024-11-20"),
            Self::Gpt4o20240806 => f.write_str("gpt-4o-2024-08-06"),
            Self::Gpt4o20240513 => f.write_str("gpt-4o-2024-05-13"),
            Self::Gpt4oMini => f.write_str("gpt-4o-mini"),
            Self::Gpt4oMini20240718 => f.write_str("gpt-4o-mini-2024-07-18"),
            Self::Gpt45Preview => f.write_str("gpt-4.5-preview"),
            Self::Gpt45Preview20250227 => f.write_str("gpt-4.5-preview-2025-02-27"),
            Self::Gpt4Turbo => f.write_str("gpt-4-turbo"),
            Self::Gpt4Turbo20240409 => f.write_str("gpt-4-turbo-2024-04-09"),
            Self::Gpt40125Preview => f.write_str("gpt-4-0125-preview"),
            Self::Gpt4TurboPreview => f.write_str("gpt-4-turbo-preview"),
            Self::Gpt41106Preview => f.write_str("gpt-4-1106-preview"),
            Self::Gpt4VisionPreview => f.write_str("gpt-4-vision-preview"),
            Self::Gpt4 => f.write_str("gpt-4"),
            Self::Gpt40314 => f.write_str("gpt-4-0314"),
            Self::Gpt40613 => f.write_str("gpt-4-0613"),
            Self::Gpt432k => f.write_str("gpt-4-32k"),
            Self::Gpt432k0314 => f.write_str("gpt-4-32k-0314"),
            Self::Gpt432k0613 => f.write_str("gpt-4-32k-0613"),
            Self::Gpt35Turbo => f.write_str("gpt-3.5-turbo"),
            Self::Gpt35Turbo16k => f.write_str("gpt-3.5-turbo-16k"),
            Self::Gpt35Turbo0613 => f.write_str("gpt-3.5-turbo-0613"),
            Self::Gpt35Turbo1106 => f.write_str("gpt-3.5-turbo-1106"),
            Self::Gpt35Turbo0125 => f.write_str("gpt-3.5-turbo-0125"),
            Self::Gpt35Turbo16k0613 => f.write_str("gpt-3.5-turbo-16k-0613"),
        }
    }
}
impl ::std::str::FromStr for CreateThreadAndRunRequestModelSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gpt-5" => Ok(Self::Gpt5),
            "gpt-5-mini" => Ok(Self::Gpt5Mini),
            "gpt-5-nano" => Ok(Self::Gpt5Nano),
            "gpt-5-2025-08-07" => Ok(Self::Gpt520250807),
            "gpt-5-mini-2025-08-07" => Ok(Self::Gpt5Mini20250807),
            "gpt-5-nano-2025-08-07" => Ok(Self::Gpt5Nano20250807),
            "gpt-4.1" => Ok(Self::Gpt41),
            "gpt-4.1-mini" => Ok(Self::Gpt41Mini),
            "gpt-4.1-nano" => Ok(Self::Gpt41Nano),
            "gpt-4.1-2025-04-14" => Ok(Self::Gpt4120250414),
            "gpt-4.1-mini-2025-04-14" => Ok(Self::Gpt41Mini20250414),
            "gpt-4.1-nano-2025-04-14" => Ok(Self::Gpt41Nano20250414),
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4o-2024-11-20" => Ok(Self::Gpt4o20241120),
            "gpt-4o-2024-08-06" => Ok(Self::Gpt4o20240806),
            "gpt-4o-2024-05-13" => Ok(Self::Gpt4o20240513),
            "gpt-4o-mini" => Ok(Self::Gpt4oMini),
            "gpt-4o-mini-2024-07-18" => Ok(Self::Gpt4oMini20240718),
            "gpt-4.5-preview" => Ok(Self::Gpt45Preview),
            "gpt-4.5-preview-2025-02-27" => Ok(Self::Gpt45Preview20250227),
            "gpt-4-turbo" => Ok(Self::Gpt4Turbo),
            "gpt-4-turbo-2024-04-09" => Ok(Self::Gpt4Turbo20240409),
            "gpt-4-0125-preview" => Ok(Self::Gpt40125Preview),
            "gpt-4-turbo-preview" => Ok(Self::Gpt4TurboPreview),
            "gpt-4-1106-preview" => Ok(Self::Gpt41106Preview),
            "gpt-4-vision-preview" => Ok(Self::Gpt4VisionPreview),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-0314" => Ok(Self::Gpt40314),
            "gpt-4-0613" => Ok(Self::Gpt40613),
            "gpt-4-32k" => Ok(Self::Gpt432k),
            "gpt-4-32k-0314" => Ok(Self::Gpt432k0314),
            "gpt-4-32k-0613" => Ok(Self::Gpt432k0613),
            "gpt-3.5-turbo" => Ok(Self::Gpt35Turbo),
            "gpt-3.5-turbo-16k" => Ok(Self::Gpt35Turbo16k),
            "gpt-3.5-turbo-0613" => Ok(Self::Gpt35Turbo0613),
            "gpt-3.5-turbo-1106" => Ok(Self::Gpt35Turbo1106),
            "gpt-3.5-turbo-0125" => Ok(Self::Gpt35Turbo0125),
            "gpt-3.5-turbo-16k-0613" => Ok(Self::Gpt35Turbo16k0613),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateThreadAndRunRequestModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateThreadAndRunRequestModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadAndRunRequestToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter:
        ::std::option::Option<CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<CreateThreadAndRunRequestToolResourcesFileSearch>,
}
impl CreateThreadAndRunRequestToolResources {
    pub fn builder() -> builder::CreateThreadAndRunRequestToolResources {
        Default::default()
    }
}
#[doc = "`CreateThreadAndRunRequestToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl CreateThreadAndRunRequestToolResourcesCodeInterpreter {
    pub fn builder() -> builder::CreateThreadAndRunRequestToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`CreateThreadAndRunRequestToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl CreateThreadAndRunRequestToolResourcesFileSearch {
    pub fn builder() -> builder::CreateThreadAndRunRequestToolResourcesFileSearch {
        Default::default()
    }
}
#[doc = "`CreateThreadAndRunRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateThreadAndRunRequestToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for CreateThreadAndRunRequestToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for CreateThreadAndRunRequestToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for CreateThreadAndRunRequestToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "Options to create a new thread. If no thread is provided when running a\nrequest, an empty thread will be created.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateThreadRequest {
    #[doc = "A list of [messages](https://developers.openai.com/api/docs/assistants/migration) to start the thread with."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub messages: ::std::vec::Vec<CreateMessageRequest>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<CreateThreadRequestToolResources>,
}
impl CreateThreadRequest {
    pub fn builder() -> builder::CreateThreadRequest {
        Default::default()
    }
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadRequestToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<CreateThreadRequestToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<CreateThreadRequestToolResourcesFileSearch>,
}
impl CreateThreadRequestToolResources {
    pub fn builder() -> builder::CreateThreadRequestToolResources {
        Default::default()
    }
}
#[doc = "`CreateThreadRequestToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl CreateThreadRequestToolResourcesCodeInterpreter {
    pub fn builder() -> builder::CreateThreadRequestToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`CreateThreadRequestToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateThreadRequestToolResourcesFileSearch {
    Variant0 {
        #[doc = "The [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
        vector_store_ids: ::std::vec::Vec<::std::string::String>,
    },
    Variant1 {
        #[doc = "A helper to create a [vector store](https://developers.openai.com/api/reference/resources/vector_stores) with file_ids and attach it to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
        vector_stores:
            ::std::vec::Vec<CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem>,
    },
}
#[doc = "`CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chunking_strategy: ::std::option::Option<
        CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy,
    >,
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs to add to the vector store. For vector stores created before Nov 2025, there can be a maximum of 10,000 files in a vector store. For vector stores created starting in Nov 2025, the limit is 100,000,000 files.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
}
impl CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem {
    pub fn builder() -> builder::CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        Default::default()
    }
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "static", deny_unknown_fields)]
pub enum CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy {
    #[serde(rename = "auto")]
    Auto,
    #[doc = "Static Chunking Strategy"]
    #[serde(rename = "static")]
    Static {
        #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
        chunk_overlap_tokens: i64,
        #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
        max_chunk_size_tokens: i64,
    },
}
#[doc = "`DeleteAssistantParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteAssistantParams {
    pub assistant_id: ::std::string::String,
}
impl DeleteAssistantParams {
    pub fn builder() -> builder::DeleteAssistantParams {
        Default::default()
    }
}
#[doc = "`DeleteAssistantResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteAssistantResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteAssistantResponseObject,
}
impl DeleteAssistantResponse {
    pub fn builder() -> builder::DeleteAssistantResponse {
        Default::default()
    }
}
#[doc = "`DeleteAssistantResponseObject`"]
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
pub enum DeleteAssistantResponseObject {
    #[serde(rename = "assistant.deleted")]
    AssistantDeleted,
}
impl ::std::fmt::Display for DeleteAssistantResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AssistantDeleted => f.write_str("assistant.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteAssistantResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant.deleted" => Ok(Self::AssistantDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteAssistantResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteAssistantResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteMessageParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteMessageParams {
    pub message_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl DeleteMessageParams {
    pub fn builder() -> builder::DeleteMessageParams {
        Default::default()
    }
}
#[doc = "`DeleteMessageResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteMessageResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteMessageResponseObject,
}
impl DeleteMessageResponse {
    pub fn builder() -> builder::DeleteMessageResponse {
        Default::default()
    }
}
#[doc = "`DeleteMessageResponseObject`"]
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
pub enum DeleteMessageResponseObject {
    #[serde(rename = "thread.message.deleted")]
    ThreadMessageDeleted,
}
impl ::std::fmt::Display for DeleteMessageResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadMessageDeleted => f.write_str("thread.message.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteMessageResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.message.deleted" => Ok(Self::ThreadMessageDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteMessageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteMessageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteThreadParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteThreadParams {
    pub thread_id: ::std::string::String,
}
impl DeleteThreadParams {
    pub fn builder() -> builder::DeleteThreadParams {
        Default::default()
    }
}
#[doc = "`DeleteThreadResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteThreadResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteThreadResponseObject,
}
impl DeleteThreadResponse {
    pub fn builder() -> builder::DeleteThreadResponse {
        Default::default()
    }
}
#[doc = "`DeleteThreadResponseObject`"]
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
pub enum DeleteThreadResponseObject {
    #[serde(rename = "thread.deleted")]
    ThreadDeleted,
}
impl ::std::fmt::Display for DeleteThreadResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadDeleted => f.write_str("thread.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteThreadResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.deleted" => Ok(Self::ThreadDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteThreadResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteThreadResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Occurs when a stream ends."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DoneEvent {
    pub data: DoneEventData,
    pub event: DoneEventEvent,
}
impl DoneEvent {
    pub fn builder() -> builder::DoneEvent {
        Default::default()
    }
}
#[doc = "`DoneEventData`"]
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
pub enum DoneEventData {
    #[serde(rename = "[DONE]")]
    Done,
}
impl ::std::fmt::Display for DoneEventData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Done => f.write_str("[DONE]"),
        }
    }
}
impl ::std::str::FromStr for DoneEventData {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "[DONE]" => Ok(Self::Done),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DoneEventData {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DoneEventData {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DoneEventEvent`"]
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
pub enum DoneEventEvent {
    #[serde(rename = "done")]
    Done,
}
impl ::std::fmt::Display for DoneEventEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Done => f.write_str("done"),
        }
    }
}
impl ::std::str::FromStr for DoneEventEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "done" => Ok(Self::Done),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DoneEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DoneEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Occurs when an [error](https://developers.openai.com/api/docs/guides/error-codes#api-errors) occurs. This can happen due to an internal server error or a timeout."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ErrorEvent {
    pub data: ::openai_api_types::common::Error,
    pub event: ErrorEventEvent,
}
impl ErrorEvent {
    pub fn builder() -> builder::ErrorEvent {
        Default::default()
    }
}
#[doc = "`ErrorEventEvent`"]
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
pub enum ErrorEventEvent {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for ErrorEventEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ErrorEventEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ErrorEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ErrorEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The ranker to use for the file search. If not specified will use the `auto` ranker."]
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
pub enum FileSearchRanker {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default_2024_08_21")]
    Default20240821,
}
impl ::std::fmt::Display for FileSearchRanker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default20240821 => f.write_str("default_2024_08_21"),
        }
    }
}
impl ::std::str::FromStr for FileSearchRanker {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default_2024_08_21" => Ok(Self::Default20240821),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileSearchRanker {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileSearchRanker {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.\n\nSee the [file search tool documentation](https://developers.openai.com/api/docs/guides/tools-file-search#retrieval-customization) for more information.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileSearchRankingOptions {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranker: ::std::option::Option<FileSearchRanker>,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    pub score_threshold: f64,
}
impl FileSearchRankingOptions {
    pub fn builder() -> builder::FileSearchRankingOptions {
        Default::default()
    }
}
#[doc = "`GetAssistantParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetAssistantParams {
    pub assistant_id: ::std::string::String,
}
impl GetAssistantParams {
    pub fn builder() -> builder::GetAssistantParams {
        Default::default()
    }
}
#[doc = "`GetMessageParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetMessageParams {
    pub message_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl GetMessageParams {
    pub fn builder() -> builder::GetMessageParams {
        Default::default()
    }
}
#[doc = "`GetRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetRunParams {
    pub run_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl GetRunParams {
    pub fn builder() -> builder::GetRunParams {
        Default::default()
    }
}
#[doc = "`GetRunStepParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetRunStepParams {
    #[serde(
        rename = "include[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include: ::std::vec::Vec<GetRunStepParamsIncludeItem>,
    pub run_id: ::std::string::String,
    pub step_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl GetRunStepParams {
    pub fn builder() -> builder::GetRunStepParams {
        Default::default()
    }
}
#[doc = "`GetRunStepParamsIncludeItem`"]
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
pub enum GetRunStepParamsIncludeItem {
    #[serde(rename = "step_details.tool_calls[*].file_search.results[*].content")]
    StepDetailsToolCallsFileSearchResultsContent,
}
impl ::std::fmt::Display for GetRunStepParamsIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StepDetailsToolCallsFileSearchResultsContent => {
                f.write_str("step_details.tool_calls[*].file_search.results[*].content")
            }
        }
    }
}
impl ::std::str::FromStr for GetRunStepParamsIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "step_details.tool_calls[*].file_search.results[*].content" => {
                Ok(Self::StepDetailsToolCallsFileSearchResultsContent)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetRunStepParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetRunStepParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetThreadParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetThreadParams {
    pub thread_id: ::std::string::String,
}
impl GetThreadParams {
    pub fn builder() -> builder::GetThreadParams {
        Default::default()
    }
}
#[doc = "`ListAssistantsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListAssistantsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListAssistantsParamsOrder>,
}
impl ListAssistantsParams {
    pub fn builder() -> builder::ListAssistantsParams {
        Default::default()
    }
}
#[doc = "`ListAssistantsParamsOrder`"]
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
pub enum ListAssistantsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListAssistantsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListAssistantsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListAssistantsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListAssistantsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListAssistantsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListAssistantsResponse {
    pub data: ::std::vec::Vec<AssistantObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListAssistantsResponse {
    pub fn builder() -> builder::ListAssistantsResponse {
        Default::default()
    }
}
#[doc = "`ListMessagesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListMessagesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListMessagesParamsOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub run_id: ::std::option::Option<::std::string::String>,
    pub thread_id: ::std::string::String,
}
impl ListMessagesParams {
    pub fn builder() -> builder::ListMessagesParams {
        Default::default()
    }
}
#[doc = "`ListMessagesParamsOrder`"]
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
pub enum ListMessagesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListMessagesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListMessagesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListMessagesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListMessagesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListMessagesResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListMessagesResponse {
    pub data: ::std::vec::Vec<MessageObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListMessagesResponse {
    pub fn builder() -> builder::ListMessagesResponse {
        Default::default()
    }
}
#[doc = "`ListRunStepsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListRunStepsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "include[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub include: ::std::vec::Vec<ListRunStepsParamsIncludeItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListRunStepsParamsOrder>,
    pub run_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl ListRunStepsParams {
    pub fn builder() -> builder::ListRunStepsParams {
        Default::default()
    }
}
#[doc = "`ListRunStepsParamsIncludeItem`"]
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
pub enum ListRunStepsParamsIncludeItem {
    #[serde(rename = "step_details.tool_calls[*].file_search.results[*].content")]
    StepDetailsToolCallsFileSearchResultsContent,
}
impl ::std::fmt::Display for ListRunStepsParamsIncludeItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StepDetailsToolCallsFileSearchResultsContent => {
                f.write_str("step_details.tool_calls[*].file_search.results[*].content")
            }
        }
    }
}
impl ::std::str::FromStr for ListRunStepsParamsIncludeItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "step_details.tool_calls[*].file_search.results[*].content" => {
                Ok(Self::StepDetailsToolCallsFileSearchResultsContent)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListRunStepsParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListRunStepsParamsIncludeItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListRunStepsParamsOrder`"]
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
pub enum ListRunStepsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListRunStepsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListRunStepsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListRunStepsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListRunStepsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListRunStepsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListRunStepsResponse {
    pub data: ::std::vec::Vec<RunStepObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListRunStepsResponse {
    pub fn builder() -> builder::ListRunStepsResponse {
        Default::default()
    }
}
#[doc = "`ListRunsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListRunsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListRunsParamsOrder>,
    pub thread_id: ::std::string::String,
}
impl ListRunsParams {
    pub fn builder() -> builder::ListRunsParams {
        Default::default()
    }
}
#[doc = "`ListRunsParamsOrder`"]
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
pub enum ListRunsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListRunsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListRunsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListRunsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListRunsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListRunsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListRunsResponse {
    pub data: ::std::vec::Vec<RunObject>,
    pub first_id: ::std::string::String,
    pub has_more: bool,
    pub last_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl ListRunsResponse {
    pub fn builder() -> builder::ListRunsResponse {
        Default::default()
    }
}
#[doc = "References an image [File](https://developers.openai.com/api/reference/resources/files) in the content of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentImageFileObject {
    pub image_file: MessageContentImageFileObjectImageFile,
    #[doc = "Always `image_file`."]
    #[serde(rename = "type")]
    pub type_: MessageContentImageFileObjectType,
}
impl MessageContentImageFileObject {
    pub fn builder() -> builder::MessageContentImageFileObject {
        Default::default()
    }
}
#[doc = "`MessageContentImageFileObjectImageFile`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentImageFileObjectImageFile {
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<MessageContentImageFileObjectImageFileDetail>,
    #[doc = "The [File](https://developers.openai.com/api/reference/resources/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    pub file_id: ::std::string::String,
}
impl MessageContentImageFileObjectImageFile {
    pub fn builder() -> builder::MessageContentImageFileObjectImageFile {
        Default::default()
    }
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
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
pub enum MessageContentImageFileObjectImageFileDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for MessageContentImageFileObjectImageFileDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for MessageContentImageFileObjectImageFileDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentImageFileObjectImageFileDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageContentImageFileObjectImageFileDetail
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Always `image_file`."]
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
pub enum MessageContentImageFileObjectType {
    #[serde(rename = "image_file")]
    ImageFile,
}
impl ::std::fmt::Display for MessageContentImageFileObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageFile => f.write_str("image_file"),
        }
    }
}
impl ::std::str::FromStr for MessageContentImageFileObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_file" => Ok(Self::ImageFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentImageFileObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageContentImageFileObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "References an image URL in the content of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentImageUrlObject {
    pub image_url: MessageContentImageUrlObjectImageUrl,
    #[doc = "The type of the content part."]
    #[serde(rename = "type")]
    pub type_: MessageContentImageUrlObjectType,
}
impl MessageContentImageUrlObject {
    pub fn builder() -> builder::MessageContentImageUrlObject {
        Default::default()
    }
}
#[doc = "`MessageContentImageUrlObjectImageUrl`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentImageUrlObjectImageUrl {
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<MessageContentImageUrlObjectImageUrlDetail>,
    #[doc = "The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    pub url: ::std::string::String,
}
impl MessageContentImageUrlObjectImageUrl {
    pub fn builder() -> builder::MessageContentImageUrlObjectImageUrl {
        Default::default()
    }
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
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
pub enum MessageContentImageUrlObjectImageUrlDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for MessageContentImageUrlObjectImageUrlDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for MessageContentImageUrlObjectImageUrlDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentImageUrlObjectImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageContentImageUrlObjectImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the content part."]
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
pub enum MessageContentImageUrlObjectType {
    #[serde(rename = "image_url")]
    ImageUrl,
}
impl ::std::fmt::Display for MessageContentImageUrlObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageUrl => f.write_str("image_url"),
        }
    }
}
impl ::std::str::FromStr for MessageContentImageUrlObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_url" => Ok(Self::ImageUrl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentImageUrlObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageContentImageUrlObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The refusal content generated by the assistant."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentRefusalObject {
    pub refusal: ::std::string::String,
    #[doc = "Always `refusal`."]
    #[serde(rename = "type")]
    pub type_: MessageContentRefusalObjectType,
}
impl MessageContentRefusalObject {
    pub fn builder() -> builder::MessageContentRefusalObject {
        Default::default()
    }
}
#[doc = "Always `refusal`."]
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
pub enum MessageContentRefusalObjectType {
    #[serde(rename = "refusal")]
    Refusal,
}
impl ::std::fmt::Display for MessageContentRefusalObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Refusal => f.write_str("refusal"),
        }
    }
}
impl ::std::str::FromStr for MessageContentRefusalObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "refusal" => Ok(Self::Refusal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentRefusalObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageContentRefusalObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextAnnotationsFileCitationObject {
    pub end_index: u64,
    pub file_citation: MessageContentTextAnnotationsFileCitationObjectFileCitation,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: ::std::string::String,
    #[doc = "Always `file_citation`."]
    #[serde(rename = "type")]
    pub type_: MessageContentTextAnnotationsFileCitationObjectType,
}
impl MessageContentTextAnnotationsFileCitationObject {
    pub fn builder() -> builder::MessageContentTextAnnotationsFileCitationObject {
        Default::default()
    }
}
#[doc = "`MessageContentTextAnnotationsFileCitationObjectFileCitation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    pub file_id: ::std::string::String,
}
impl MessageContentTextAnnotationsFileCitationObjectFileCitation {
    pub fn builder() -> builder::MessageContentTextAnnotationsFileCitationObjectFileCitation {
        Default::default()
    }
}
#[doc = "Always `file_citation`."]
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
pub enum MessageContentTextAnnotationsFileCitationObjectType {
    #[serde(rename = "file_citation")]
    FileCitation,
}
impl ::std::fmt::Display for MessageContentTextAnnotationsFileCitationObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileCitation => f.write_str("file_citation"),
        }
    }
}
impl ::std::str::FromStr for MessageContentTextAnnotationsFileCitationObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_citation" => Ok(Self::FileCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentTextAnnotationsFileCitationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageContentTextAnnotationsFileCitationObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextAnnotationsFilePathObject {
    pub end_index: u64,
    pub file_path: MessageContentTextAnnotationsFilePathObjectFilePath,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: ::std::string::String,
    #[doc = "Always `file_path`."]
    #[serde(rename = "type")]
    pub type_: MessageContentTextAnnotationsFilePathObjectType,
}
impl MessageContentTextAnnotationsFilePathObject {
    pub fn builder() -> builder::MessageContentTextAnnotationsFilePathObject {
        Default::default()
    }
}
#[doc = "`MessageContentTextAnnotationsFilePathObjectFilePath`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    pub file_id: ::std::string::String,
}
impl MessageContentTextAnnotationsFilePathObjectFilePath {
    pub fn builder() -> builder::MessageContentTextAnnotationsFilePathObjectFilePath {
        Default::default()
    }
}
#[doc = "Always `file_path`."]
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
pub enum MessageContentTextAnnotationsFilePathObjectType {
    #[serde(rename = "file_path")]
    FilePath,
}
impl ::std::fmt::Display for MessageContentTextAnnotationsFilePathObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FilePath => f.write_str("file_path"),
        }
    }
}
impl ::std::str::FromStr for MessageContentTextAnnotationsFilePathObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_path" => Ok(Self::FilePath),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentTextAnnotationsFilePathObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageContentTextAnnotationsFilePathObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The text content that is part of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextObject {
    pub text: MessageContentTextObjectText,
    #[doc = "Always `text`."]
    #[serde(rename = "type")]
    pub type_: MessageContentTextObjectType,
}
impl MessageContentTextObject {
    pub fn builder() -> builder::MessageContentTextObject {
        Default::default()
    }
}
#[doc = "`MessageContentTextObjectText`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageContentTextObjectText {
    pub annotations: ::std::vec::Vec<MessageContentTextObjectTextAnnotationsItem>,
    #[doc = "The data that makes up the text."]
    pub value: ::std::string::String,
}
impl MessageContentTextObjectText {
    pub fn builder() -> builder::MessageContentTextObjectText {
        Default::default()
    }
}
#[doc = "`MessageContentTextObjectTextAnnotationsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageContentTextObjectTextAnnotationsItem {
    CitationObject(MessageContentTextAnnotationsFileCitationObject),
    PathObject(MessageContentTextAnnotationsFilePathObject),
}
impl ::std::convert::From<MessageContentTextAnnotationsFileCitationObject>
    for MessageContentTextObjectTextAnnotationsItem
{
    fn from(value: MessageContentTextAnnotationsFileCitationObject) -> Self {
        Self::CitationObject(value)
    }
}
impl ::std::convert::From<MessageContentTextAnnotationsFilePathObject>
    for MessageContentTextObjectTextAnnotationsItem
{
    fn from(value: MessageContentTextAnnotationsFilePathObject) -> Self {
        Self::PathObject(value)
    }
}
#[doc = "Always `text`."]
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
pub enum MessageContentTextObjectType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for MessageContentTextObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for MessageContentTextObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "References an image [File](https://developers.openai.com/api/reference/resources/files) in the content of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentImageFileObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_file: ::std::option::Option<MessageDeltaContentImageFileObjectImageFile>,
    #[doc = "The index of the content part in the message."]
    pub index: i64,
    #[doc = "Always `image_file`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentImageFileObjectType,
}
impl MessageDeltaContentImageFileObject {
    pub fn builder() -> builder::MessageDeltaContentImageFileObject {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentImageFileObjectImageFile`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaContentImageFileObjectImageFile {
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<MessageDeltaContentImageFileObjectImageFileDetail>,
    #[doc = "The [File](https://developers.openai.com/api/reference/resources/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
}
impl MessageDeltaContentImageFileObjectImageFile {
    pub fn builder() -> builder::MessageDeltaContentImageFileObjectImageFile {
        Default::default()
    }
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
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
pub enum MessageDeltaContentImageFileObjectImageFileDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for MessageDeltaContentImageFileObjectImageFileDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentImageFileObjectImageFileDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentImageFileObjectImageFileDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageDeltaContentImageFileObjectImageFileDetail
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Always `image_file`."]
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
pub enum MessageDeltaContentImageFileObjectType {
    #[serde(rename = "image_file")]
    ImageFile,
}
impl ::std::fmt::Display for MessageDeltaContentImageFileObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageFile => f.write_str("image_file"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentImageFileObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_file" => Ok(Self::ImageFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentImageFileObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaContentImageFileObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "References an image URL in the content of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentImageUrlObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<MessageDeltaContentImageUrlObjectImageUrl>,
    #[doc = "The index of the content part in the message."]
    pub index: i64,
    #[doc = "Always `image_url`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentImageUrlObjectType,
}
impl MessageDeltaContentImageUrlObject {
    pub fn builder() -> builder::MessageDeltaContentImageUrlObject {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentImageUrlObjectImageUrl`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaContentImageUrlObjectImageUrl {
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<MessageDeltaContentImageUrlObjectImageUrlDetail>,
    #[doc = "The URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl MessageDeltaContentImageUrlObjectImageUrl {
    pub fn builder() -> builder::MessageDeltaContentImageUrlObjectImageUrl {
        Default::default()
    }
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
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
pub enum MessageDeltaContentImageUrlObjectImageUrlDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for MessageDeltaContentImageUrlObjectImageUrlDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentImageUrlObjectImageUrlDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentImageUrlObjectImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageDeltaContentImageUrlObjectImageUrlDetail
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Always `image_url`."]
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
pub enum MessageDeltaContentImageUrlObjectType {
    #[serde(rename = "image_url")]
    ImageUrl,
}
impl ::std::fmt::Display for MessageDeltaContentImageUrlObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageUrl => f.write_str("image_url"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentImageUrlObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_url" => Ok(Self::ImageUrl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentImageUrlObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaContentImageUrlObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The refusal content that is part of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentRefusalObject {
    #[doc = "The index of the refusal part in the message."]
    pub index: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refusal: ::std::option::Option<::std::string::String>,
    #[doc = "Always `refusal`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentRefusalObjectType,
}
impl MessageDeltaContentRefusalObject {
    pub fn builder() -> builder::MessageDeltaContentRefusalObject {
        Default::default()
    }
}
#[doc = "Always `refusal`."]
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
pub enum MessageDeltaContentRefusalObjectType {
    #[serde(rename = "refusal")]
    Refusal,
}
impl ::std::fmt::Display for MessageDeltaContentRefusalObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Refusal => f.write_str("refusal"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentRefusalObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "refusal" => Ok(Self::Refusal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentRefusalObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaContentRefusalObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_index: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_citation:
        ::std::option::Option<MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>,
    #[doc = "The index of the annotation in the text content part."]
    pub index: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_index: ::std::option::Option<u64>,
    #[doc = "The text in the message content that needs to be replaced."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "Always `file_citation`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextAnnotationsFileCitationObjectType,
}
impl MessageDeltaContentTextAnnotationsFileCitationObject {
    pub fn builder() -> builder::MessageDeltaContentTextAnnotationsFileCitationObject {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The specific quote in the file."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quote: ::std::option::Option<::std::string::String>,
}
impl MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
    pub fn builder() -> builder::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
        Default::default()
    }
}
#[doc = "Always `file_citation`."]
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
pub enum MessageDeltaContentTextAnnotationsFileCitationObjectType {
    #[serde(rename = "file_citation")]
    FileCitation,
}
impl ::std::fmt::Display for MessageDeltaContentTextAnnotationsFileCitationObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileCitation => f.write_str("file_citation"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentTextAnnotationsFileCitationObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_citation" => Ok(Self::FileCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentTextAnnotationsFileCitationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageDeltaContentTextAnnotationsFileCitationObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_index: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_path: ::std::option::Option<MessageDeltaContentTextAnnotationsFilePathObjectFilePath>,
    #[doc = "The index of the annotation in the text content part."]
    pub index: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_index: ::std::option::Option<u64>,
    #[doc = "The text in the message content that needs to be replaced."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "Always `file_path`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextAnnotationsFilePathObjectType,
}
impl MessageDeltaContentTextAnnotationsFilePathObject {
    pub fn builder() -> builder::MessageDeltaContentTextAnnotationsFilePathObject {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentTextAnnotationsFilePathObjectFilePath`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
}
impl MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    pub fn builder() -> builder::MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
        Default::default()
    }
}
#[doc = "Always `file_path`."]
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
pub enum MessageDeltaContentTextAnnotationsFilePathObjectType {
    #[serde(rename = "file_path")]
    FilePath,
}
impl ::std::fmt::Display for MessageDeltaContentTextAnnotationsFilePathObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FilePath => f.write_str("file_path"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentTextAnnotationsFilePathObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_path" => Ok(Self::FilePath),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentTextAnnotationsFilePathObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MessageDeltaContentTextAnnotationsFilePathObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The text content that is part of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaContentTextObject {
    #[doc = "The index of the content part in the message."]
    pub index: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<MessageDeltaContentTextObjectText>,
    #[doc = "Always `text`."]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextObjectType,
}
impl MessageDeltaContentTextObject {
    pub fn builder() -> builder::MessageDeltaContentTextObject {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentTextObjectText`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaContentTextObjectText {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub annotations: ::std::vec::Vec<MessageDeltaContentTextObjectTextAnnotationsItem>,
    #[doc = "The data that makes up the text."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl MessageDeltaContentTextObjectText {
    pub fn builder() -> builder::MessageDeltaContentTextObjectText {
        Default::default()
    }
}
#[doc = "`MessageDeltaContentTextObjectTextAnnotationsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageDeltaContentTextObjectTextAnnotationsItem {
    CitationObject(MessageDeltaContentTextAnnotationsFileCitationObject),
    PathObject(MessageDeltaContentTextAnnotationsFilePathObject),
}
impl ::std::convert::From<MessageDeltaContentTextAnnotationsFileCitationObject>
    for MessageDeltaContentTextObjectTextAnnotationsItem
{
    fn from(value: MessageDeltaContentTextAnnotationsFileCitationObject) -> Self {
        Self::CitationObject(value)
    }
}
impl ::std::convert::From<MessageDeltaContentTextAnnotationsFilePathObject>
    for MessageDeltaContentTextObjectTextAnnotationsItem
{
    fn from(value: MessageDeltaContentTextAnnotationsFilePathObject) -> Self {
        Self::PathObject(value)
    }
}
#[doc = "Always `text`."]
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
pub enum MessageDeltaContentTextObjectType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for MessageDeltaContentTextObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaContentTextObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a message delta i.e. any changed fields on a message during streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageDeltaObject {
    pub delta: MessageDeltaObjectDelta,
    #[doc = "The identifier of the message, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `thread.message.delta`."]
    pub object: MessageDeltaObjectObject,
}
impl MessageDeltaObject {
    pub fn builder() -> builder::MessageDeltaObject {
        Default::default()
    }
}
#[doc = "The delta containing the fields that have changed on the Message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageDeltaObjectDelta {
    #[doc = "The content of the message in array of text and/or images."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub content: ::std::vec::Vec<MessageDeltaObjectDeltaContentItem>,
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<MessageDeltaObjectDeltaRole>,
}
impl MessageDeltaObjectDelta {
    pub fn builder() -> builder::MessageDeltaObjectDelta {
        Default::default()
    }
}
#[doc = "`MessageDeltaObjectDeltaContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageDeltaObjectDeltaContentItem {
    ImageFileObject(MessageDeltaContentImageFileObject),
    TextObject(MessageDeltaContentTextObject),
    RefusalObject(MessageDeltaContentRefusalObject),
    ImageUrlObject(MessageDeltaContentImageUrlObject),
}
impl ::std::convert::From<MessageDeltaContentImageFileObject>
    for MessageDeltaObjectDeltaContentItem
{
    fn from(value: MessageDeltaContentImageFileObject) -> Self {
        Self::ImageFileObject(value)
    }
}
impl ::std::convert::From<MessageDeltaContentTextObject> for MessageDeltaObjectDeltaContentItem {
    fn from(value: MessageDeltaContentTextObject) -> Self {
        Self::TextObject(value)
    }
}
impl ::std::convert::From<MessageDeltaContentRefusalObject> for MessageDeltaObjectDeltaContentItem {
    fn from(value: MessageDeltaContentRefusalObject) -> Self {
        Self::RefusalObject(value)
    }
}
impl ::std::convert::From<MessageDeltaContentImageUrlObject>
    for MessageDeltaObjectDeltaContentItem
{
    fn from(value: MessageDeltaContentImageUrlObject) -> Self {
        Self::ImageUrlObject(value)
    }
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
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
pub enum MessageDeltaObjectDeltaRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for MessageDeltaObjectDeltaRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaObjectDeltaRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaObjectDeltaRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaObjectDeltaRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `thread.message.delta`."]
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
pub enum MessageDeltaObjectObject {
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta,
}
impl ::std::fmt::Display for MessageDeltaObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadMessageDelta => f.write_str("thread.message.delta"),
        }
    }
}
impl ::std::str::FromStr for MessageDeltaObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.message.delta" => Ok(Self::ThreadMessageDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageDeltaObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageDeltaObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a message within a [thread](https://developers.openai.com/api/docs/assistants/migration)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageObject {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub assistant_id: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub attachments: ::std::option::Option<::std::vec::Vec<MessageObjectAttachmentsItem>>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "The content of the message in array of text and/or images."]
    pub content: ::std::vec::Vec<MessageObjectContentItem>,
    #[doc = "The Unix timestamp (in seconds) for when the message was created."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub incomplete_at: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub incomplete_details: ::std::option::Option<MessageObjectIncompleteDetails>,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The object type, which is always `thread.message`."]
    pub object: MessageObjectObject,
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    pub role: MessageObjectRole,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub run_id: ::std::option::Option<::std::string::String>,
    #[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
    pub status: MessageObjectStatus,
    #[doc = "The [thread](https://developers.openai.com/api/docs/assistants/migration) ID that this message belongs to."]
    pub thread_id: ::std::string::String,
}
impl MessageObject {
    pub fn builder() -> builder::MessageObject {
        Default::default()
    }
}
#[doc = "`MessageObjectAttachmentsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MessageObjectAttachmentsItem {
    #[doc = "The ID of the file to attach to the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The tools to add this file to."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<MessageObjectAttachmentsItemToolsItem>,
}
impl MessageObjectAttachmentsItem {
    pub fn builder() -> builder::MessageObjectAttachmentsItem {
        Default::default()
    }
}
#[doc = "`MessageObjectAttachmentsItemToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageObjectAttachmentsItemToolsItem {
    Code(AssistantToolsCode),
    FileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}
impl ::std::convert::From<AssistantToolsCode> for MessageObjectAttachmentsItemToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearchTypeOnly>
    for MessageObjectAttachmentsItemToolsItem
{
    fn from(value: AssistantToolsFileSearchTypeOnly) -> Self {
        Self::FileSearchTypeOnly(value)
    }
}
#[doc = "`MessageObjectContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageObjectContentItem {
    ImageFileObject(MessageContentImageFileObject),
    ImageUrlObject(MessageContentImageUrlObject),
    TextObject(MessageContentTextObject),
    RefusalObject(MessageContentRefusalObject),
}
impl ::std::convert::From<MessageContentImageFileObject> for MessageObjectContentItem {
    fn from(value: MessageContentImageFileObject) -> Self {
        Self::ImageFileObject(value)
    }
}
impl ::std::convert::From<MessageContentImageUrlObject> for MessageObjectContentItem {
    fn from(value: MessageContentImageUrlObject) -> Self {
        Self::ImageUrlObject(value)
    }
}
impl ::std::convert::From<MessageContentTextObject> for MessageObjectContentItem {
    fn from(value: MessageContentTextObject) -> Self {
        Self::TextObject(value)
    }
}
impl ::std::convert::From<MessageContentRefusalObject> for MessageObjectContentItem {
    fn from(value: MessageContentRefusalObject) -> Self {
        Self::RefusalObject(value)
    }
}
#[doc = "On an incomplete message, details about why the message is incomplete."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageObjectIncompleteDetails {
    #[doc = "The reason the message is incomplete."]
    pub reason: MessageObjectIncompleteDetailsReason,
}
impl MessageObjectIncompleteDetails {
    pub fn builder() -> builder::MessageObjectIncompleteDetails {
        Default::default()
    }
}
#[doc = "The reason the message is incomplete."]
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
pub enum MessageObjectIncompleteDetailsReason {
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "run_cancelled")]
    RunCancelled,
    #[serde(rename = "run_expired")]
    RunExpired,
    #[serde(rename = "run_failed")]
    RunFailed,
}
impl ::std::fmt::Display for MessageObjectIncompleteDetailsReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContentFilter => f.write_str("content_filter"),
            Self::MaxTokens => f.write_str("max_tokens"),
            Self::RunCancelled => f.write_str("run_cancelled"),
            Self::RunExpired => f.write_str("run_expired"),
            Self::RunFailed => f.write_str("run_failed"),
        }
    }
}
impl ::std::str::FromStr for MessageObjectIncompleteDetailsReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "content_filter" => Ok(Self::ContentFilter),
            "max_tokens" => Ok(Self::MaxTokens),
            "run_cancelled" => Ok(Self::RunCancelled),
            "run_expired" => Ok(Self::RunExpired),
            "run_failed" => Ok(Self::RunFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageObjectIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageObjectIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `thread.message`."]
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
pub enum MessageObjectObject {
    #[serde(rename = "thread.message")]
    ThreadMessage,
}
impl ::std::fmt::Display for MessageObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadMessage => f.write_str("thread.message"),
        }
    }
}
impl ::std::str::FromStr for MessageObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.message" => Ok(Self::ThreadMessage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
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
pub enum MessageObjectRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for MessageObjectRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for MessageObjectRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageObjectRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageObjectRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
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
pub enum MessageObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for MessageObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for MessageObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "incomplete" => Ok(Self::Incomplete),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The text content that is part of a message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MessageRequestContentTextObject {
    #[doc = "Text content to be sent to the model"]
    pub text: ::std::string::String,
    #[doc = "Always `text`."]
    #[serde(rename = "type")]
    pub type_: MessageRequestContentTextObjectType,
}
impl MessageRequestContentTextObject {
    pub fn builder() -> builder::MessageRequestContentTextObject {
        Default::default()
    }
}
#[doc = "Always `text`."]
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
pub enum MessageRequestContentTextObjectType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for MessageRequestContentTextObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for MessageRequestContentTextObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageRequestContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageRequestContentTextObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MessageStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "event", content = "data")]
pub enum MessageStreamEvent {
    #[doc = "Occurs when a [message](https://developers.openai.com/api/docs/assistants/migration) is created."]
    #[serde(rename = "thread.message.created")]
    ThreadMessageCreated(MessageObject),
    #[doc = "Occurs when a [message](https://developers.openai.com/api/docs/assistants/migration) moves to an `in_progress` state."]
    #[serde(rename = "thread.message.in_progress")]
    ThreadMessageInProgress(MessageObject),
    #[doc = "Occurs when parts of a [Message](https://developers.openai.com/api/docs/assistants/migration) are being streamed."]
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta(MessageDeltaObject),
    #[doc = "Occurs when a [message](https://developers.openai.com/api/docs/assistants/migration) is completed."]
    #[serde(rename = "thread.message.completed")]
    ThreadMessageCompleted(MessageObject),
    #[doc = "Occurs when a [message](https://developers.openai.com/api/docs/assistants/migration) ends before it is completed."]
    #[serde(rename = "thread.message.incomplete")]
    ThreadMessageIncomplete(MessageObject),
}
impl ::std::convert::From<MessageDeltaObject> for MessageStreamEvent {
    fn from(value: MessageDeltaObject) -> Self {
        Self::ThreadMessageDelta(value)
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
#[doc = "`ModifyAssistantParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyAssistantParams {
    pub assistant_id: ::std::string::String,
}
impl ModifyAssistantParams {
    pub fn builder() -> builder::ModifyAssistantParams {
        Default::default()
    }
}
#[doc = "`ModifyAssistantRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModifyAssistantRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<ModifyAssistantRequestDescription>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<ModifyAssistantRequestInstructions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ModifyAssistantRequestModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<ModifyAssistantRequestName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<ModifyAssistantRequestToolResources>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<ModifyAssistantRequestToolsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
impl ModifyAssistantRequest {
    pub fn builder() -> builder::ModifyAssistantRequest {
        Default::default()
    }
}
#[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ModifyAssistantRequestDescription(::std::string::String);
impl ::std::ops::Deref for ModifyAssistantRequestDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModifyAssistantRequestDescription> for ::std::string::String {
    fn from(value: ModifyAssistantRequestDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ModifyAssistantRequestDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModifyAssistantRequestDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModifyAssistantRequestDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModifyAssistantRequestDescription {
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
#[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ModifyAssistantRequestInstructions(::std::string::String);
impl ::std::ops::Deref for ModifyAssistantRequestInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModifyAssistantRequestInstructions> for ::std::string::String {
    fn from(value: ModifyAssistantRequestInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ModifyAssistantRequestInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256000usize {
            return Err("longer than 256000 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModifyAssistantRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModifyAssistantRequestInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModifyAssistantRequestInstructions {
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
#[doc = "ID of the model to use. You can use the [List models](https://developers.openai.com/api/reference/resources/models/methods/list) API to see all of your available models, or see our [Model overview](https://developers.openai.com/api/docs/models) for descriptions of them.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyAssistantRequestModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<AssistantSupportedModels>,
}
impl ModifyAssistantRequestModel {
    pub fn builder() -> builder::ModifyAssistantRequestModel {
        Default::default()
    }
}
#[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ModifyAssistantRequestName(::std::string::String);
impl ::std::ops::Deref for ModifyAssistantRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModifyAssistantRequestName> for ::std::string::String {
    fn from(value: ModifyAssistantRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ModifyAssistantRequestName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModifyAssistantRequestName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModifyAssistantRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModifyAssistantRequestName {
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
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyAssistantRequestToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<ModifyAssistantRequestToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<ModifyAssistantRequestToolResourcesFileSearch>,
}
impl ModifyAssistantRequestToolResources {
    pub fn builder() -> builder::ModifyAssistantRequestToolResources {
        Default::default()
    }
}
#[doc = "`ModifyAssistantRequestToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "Overrides the list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl ModifyAssistantRequestToolResourcesCodeInterpreter {
    pub fn builder() -> builder::ModifyAssistantRequestToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`ModifyAssistantRequestToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyAssistantRequestToolResourcesFileSearch {
    #[doc = "Overrides the [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl ModifyAssistantRequestToolResourcesFileSearch {
    pub fn builder() -> builder::ModifyAssistantRequestToolResourcesFileSearch {
        Default::default()
    }
}
#[doc = "`ModifyAssistantRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ModifyAssistantRequestToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for ModifyAssistantRequestToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for ModifyAssistantRequestToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for ModifyAssistantRequestToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "`ModifyMessageParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyMessageParams {
    pub message_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl ModifyMessageParams {
    pub fn builder() -> builder::ModifyMessageParams {
        Default::default()
    }
}
#[doc = "`ModifyMessageRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModifyMessageRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
}
impl ModifyMessageRequest {
    pub fn builder() -> builder::ModifyMessageRequest {
        Default::default()
    }
}
#[doc = "`ModifyRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyRunParams {
    pub run_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl ModifyRunParams {
    pub fn builder() -> builder::ModifyRunParams {
        Default::default()
    }
}
#[doc = "`ModifyRunRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModifyRunRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
}
impl ModifyRunRequest {
    pub fn builder() -> builder::ModifyRunRequest {
        Default::default()
    }
}
#[doc = "`ModifyThreadParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyThreadParams {
    pub thread_id: ::std::string::String,
}
impl ModifyThreadParams {
    pub fn builder() -> builder::ModifyThreadParams {
        Default::default()
    }
}
#[doc = "`ModifyThreadRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModifyThreadRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_resources: ::std::option::Option<ModifyThreadRequestToolResources>,
}
impl ModifyThreadRequest {
    pub fn builder() -> builder::ModifyThreadRequest {
        Default::default()
    }
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyThreadRequestToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<ModifyThreadRequestToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<ModifyThreadRequestToolResourcesFileSearch>,
}
impl ModifyThreadRequestToolResources {
    pub fn builder() -> builder::ModifyThreadRequestToolResources {
        Default::default()
    }
}
#[doc = "`ModifyThreadRequestToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl ModifyThreadRequestToolResourcesCodeInterpreter {
    pub fn builder() -> builder::ModifyThreadRequestToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`ModifyThreadRequestToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModifyThreadRequestToolResourcesFileSearch {
    #[doc = "The [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl ModifyThreadRequestToolResourcesFileSearch {
    pub fn builder() -> builder::ModifyThreadRequestToolResourcesFileSearch {
        Default::default()
    }
}
#[doc = "Constrains effort on reasoning for reasoning models. Currently supported\nvalues are `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`.\nReducing reasoning effort can result in faster responses and fewer tokens\nused on reasoning in a response. Not all reasoning models support every\nvalue. See the\n[reasoning guide](https://developers.openai.com/api/docs/guides/reasoning)\nfor model-specific support.\n"]
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
pub enum ReasoningEffortValue {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
}
impl ::std::fmt::Display for ReasoningEffortValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
        }
    }
}
impl ::std::str::FromStr for ReasoningEffortValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningEffortValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningEffortValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunCompletionUsage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct RunCompletionUsage(pub ::std::option::Option<RunCompletionUsageValue>);
impl ::std::ops::Deref for RunCompletionUsage {
    type Target = ::std::option::Option<RunCompletionUsageValue>;
    fn deref(&self) -> &::std::option::Option<RunCompletionUsageValue> {
        &self.0
    }
}
impl ::std::convert::From<RunCompletionUsage> for ::std::option::Option<RunCompletionUsageValue> {
    fn from(value: RunCompletionUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<RunCompletionUsageValue>> for RunCompletionUsage {
    fn from(value: ::std::option::Option<RunCompletionUsageValue>) -> Self {
        Self(value)
    }
}
#[doc = "Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunCompletionUsageValue {
    #[doc = "Number of completion tokens used over the course of the run."]
    pub completion_tokens: i64,
    #[doc = "Number of prompt tokens used over the course of the run."]
    pub prompt_tokens: i64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    pub total_tokens: i64,
}
impl RunCompletionUsageValue {
    pub fn builder() -> builder::RunCompletionUsageValue {
        Default::default()
    }
}
#[doc = "Represents an execution run on a [thread](https://developers.openai.com/api/docs/assistants/migration)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunObject {
    #[doc = "The ID of the [assistant](https://developers.openai.com/api/docs/assistants/migration) used for execution of this run."]
    pub assistant_id: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) for when the run was cancelled."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub cancelled_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the run was completed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the run was created."]
    pub created_at: i64,
    #[doc = "The Unix timestamp (in seconds) for when the run will expire."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the run failed."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failed_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub incomplete_details: ::std::option::Option<RunObjectIncompleteDetails>,
    #[doc = "The instructions that the [assistant](https://developers.openai.com/api/docs/assistants/migration) used for this run."]
    pub instructions: ::std::string::String,
    #[doc = "The last error associated with this run. Will be `null` if there are no errors."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_error: ::std::option::Option<RunObjectLastError>,
    #[doc = "The maximum number of completion tokens specified to have been used over the course of the run.\n"]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[doc = "The maximum number of prompt tokens specified to have been used over the course of the run.\n"]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_prompt_tokens: ::std::option::Option<i64>,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The model that the [assistant](https://developers.openai.com/api/docs/assistants/migration) used for this run."]
    pub model: ::std::string::String,
    #[doc = "The object type, which is always `thread.run`."]
    pub object: RunObjectObject,
    pub parallel_tool_calls: ::openai_api_types::common::ParallelToolCalls,
    #[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub required_action: ::std::option::Option<RunObjectRequiredAction>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub response_format: ::std::option::Option<AssistantsApiResponseFormatOption>,
    #[doc = "The Unix timestamp (in seconds) for when the run was started."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub started_at: ::std::option::Option<i64>,
    #[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
    pub status: RunObjectStatus,
    #[doc = "The sampling temperature used for this run. If not set, defaults to 1."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "The ID of the [thread](https://developers.openai.com/api/docs/assistants/migration) that was executed on as a part of this run."]
    pub thread_id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub tool_choice: ::std::option::Option<AssistantsApiToolChoiceOption>,
    #[doc = "The list of tools that the [assistant](https://developers.openai.com/api/docs/assistants/migration) used for this run."]
    pub tools: ::std::vec::Vec<RunObjectToolsItem>,
    #[doc = "The nucleus sampling value used for this run. If not set, defaults to 1."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub truncation_strategy: ::std::option::Option<TruncationObject>,
    pub usage: RunCompletionUsage,
}
impl RunObject {
    pub fn builder() -> builder::RunObject {
        Default::default()
    }
}
#[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunObjectIncompleteDetails {
    #[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<RunObjectIncompleteDetailsReason>,
}
impl RunObjectIncompleteDetails {
    pub fn builder() -> builder::RunObjectIncompleteDetails {
        Default::default()
    }
}
#[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
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
pub enum RunObjectIncompleteDetailsReason {
    #[serde(rename = "max_completion_tokens")]
    MaxCompletionTokens,
    #[serde(rename = "max_prompt_tokens")]
    MaxPromptTokens,
}
impl ::std::fmt::Display for RunObjectIncompleteDetailsReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MaxCompletionTokens => f.write_str("max_completion_tokens"),
            Self::MaxPromptTokens => f.write_str("max_prompt_tokens"),
        }
    }
}
impl ::std::str::FromStr for RunObjectIncompleteDetailsReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "max_completion_tokens" => Ok(Self::MaxCompletionTokens),
            "max_prompt_tokens" => Ok(Self::MaxPromptTokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunObjectIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunObjectIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The last error associated with this run. Will be `null` if there are no errors."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunObjectLastError {
    #[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
    pub code: RunObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: ::std::string::String,
}
impl RunObjectLastError {
    pub fn builder() -> builder::RunObjectLastError {
        Default::default()
    }
}
#[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
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
pub enum RunObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
}
impl ::std::fmt::Display for RunObjectLastErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerError => f.write_str("server_error"),
            Self::RateLimitExceeded => f.write_str("rate_limit_exceeded"),
            Self::InvalidPrompt => f.write_str("invalid_prompt"),
        }
    }
}
impl ::std::str::FromStr for RunObjectLastErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_error" => Ok(Self::ServerError),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            "invalid_prompt" => Ok(Self::InvalidPrompt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `thread.run`."]
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
pub enum RunObjectObject {
    #[serde(rename = "thread.run")]
    ThreadRun,
}
impl ::std::fmt::Display for RunObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadRun => f.write_str("thread.run"),
        }
    }
}
impl ::std::str::FromStr for RunObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.run" => Ok(Self::ThreadRun),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunObjectRequiredAction {
    pub submit_tool_outputs: RunObjectRequiredActionSubmitToolOutputs,
    #[doc = "For now, this is always `submit_tool_outputs`."]
    #[serde(rename = "type")]
    pub type_: RunObjectRequiredActionType,
}
impl RunObjectRequiredAction {
    pub fn builder() -> builder::RunObjectRequiredAction {
        Default::default()
    }
}
#[doc = "Details on the tool outputs needed for this run to continue."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunObjectRequiredActionSubmitToolOutputs {
    #[doc = "A list of the relevant tool calls."]
    pub tool_calls: ::std::vec::Vec<RunToolCallObject>,
}
impl RunObjectRequiredActionSubmitToolOutputs {
    pub fn builder() -> builder::RunObjectRequiredActionSubmitToolOutputs {
        Default::default()
    }
}
#[doc = "For now, this is always `submit_tool_outputs`."]
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
pub enum RunObjectRequiredActionType {
    #[serde(rename = "submit_tool_outputs")]
    SubmitToolOutputs,
}
impl ::std::fmt::Display for RunObjectRequiredActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SubmitToolOutputs => f.write_str("submit_tool_outputs"),
        }
    }
}
impl ::std::str::FromStr for RunObjectRequiredActionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "submit_tool_outputs" => Ok(Self::SubmitToolOutputs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunObjectRequiredActionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunObjectRequiredActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
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
pub enum RunObjectStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for RunObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::InProgress => f.write_str("in_progress"),
            Self::RequiresAction => f.write_str("requires_action"),
            Self::Cancelling => f.write_str("cancelling"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for RunObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "in_progress" => Ok(Self::InProgress),
            "requires_action" => Ok(Self::RequiresAction),
            "cancelling" => Ok(Self::Cancelling),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunObjectToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunObjectToolsItem {
    Code(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
impl ::std::convert::From<AssistantToolsCode> for RunObjectToolsItem {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}
impl ::std::convert::From<AssistantToolsFileSearch> for RunObjectToolsItem {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}
impl ::std::convert::From<AssistantToolsFunction> for RunObjectToolsItem {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}
#[doc = "`RunStepCompletionUsage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct RunStepCompletionUsage(pub ::std::option::Option<RunStepCompletionUsageValue>);
impl ::std::ops::Deref for RunStepCompletionUsage {
    type Target = ::std::option::Option<RunStepCompletionUsageValue>;
    fn deref(&self) -> &::std::option::Option<RunStepCompletionUsageValue> {
        &self.0
    }
}
impl ::std::convert::From<RunStepCompletionUsage>
    for ::std::option::Option<RunStepCompletionUsageValue>
{
    fn from(value: RunStepCompletionUsage) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<RunStepCompletionUsageValue>>
    for RunStepCompletionUsage
{
    fn from(value: ::std::option::Option<RunStepCompletionUsageValue>) -> Self {
        Self(value)
    }
}
#[doc = "Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepCompletionUsageValue {
    #[doc = "Number of completion tokens used over the course of the run step."]
    pub completion_tokens: i64,
    #[doc = "Number of prompt tokens used over the course of the run step."]
    pub prompt_tokens: i64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    pub total_tokens: i64,
}
impl RunStepCompletionUsageValue {
    pub fn builder() -> builder::RunStepCompletionUsageValue {
        Default::default()
    }
}
#[doc = "Represents a run step delta i.e. any changed fields on a run step during streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaObject {
    pub delta: RunStepDeltaObjectDelta,
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `thread.run.step.delta`."]
    pub object: RunStepDeltaObjectObject,
}
impl RunStepDeltaObject {
    pub fn builder() -> builder::RunStepDeltaObject {
        Default::default()
    }
}
#[doc = "The delta containing the fields that have changed on the run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDeltaObjectDelta {
    #[doc = "The details of the run step."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub step_details: ::std::option::Option<RunStepDeltaObjectDeltaStepDetails>,
}
impl RunStepDeltaObjectDelta {
    pub fn builder() -> builder::RunStepDeltaObjectDelta {
        Default::default()
    }
}
#[doc = "The details of the run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepDeltaObjectDeltaStepDetails {
    MessageCreationObject(RunStepDeltaStepDetailsMessageCreationObject),
    ToolCallsObject(RunStepDeltaStepDetailsToolCallsObject),
}
impl ::std::convert::From<RunStepDeltaStepDetailsMessageCreationObject>
    for RunStepDeltaObjectDeltaStepDetails
{
    fn from(value: RunStepDeltaStepDetailsMessageCreationObject) -> Self {
        Self::MessageCreationObject(value)
    }
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsObject>
    for RunStepDeltaObjectDeltaStepDetails
{
    fn from(value: RunStepDeltaStepDetailsToolCallsObject) -> Self {
        Self::ToolCallsObject(value)
    }
}
#[doc = "The object type, which is always `thread.run.step.delta`."]
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
pub enum RunStepDeltaObjectObject {
    #[serde(rename = "thread.run.step.delta")]
    ThreadRunStepDelta,
}
impl ::std::fmt::Display for RunStepDeltaObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadRunStepDelta => f.write_str("thread.run.step.delta"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.run.step.delta" => Ok(Self::ThreadRunStepDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDeltaObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the message creation by the run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message_creation:
        ::std::option::Option<RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>,
    #[doc = "Always `message_creation`."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsMessageCreationObjectType,
}
impl RunStepDeltaStepDetailsMessageCreationObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsMessageCreationObject {
        Default::default()
    }
}
#[doc = "`RunStepDeltaStepDetailsMessageCreationObjectMessageCreation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message_id: ::std::option::Option<::std::string::String>,
}
impl RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    pub fn builder() -> builder::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
        Default::default()
    }
}
#[doc = "Always `message_creation`."]
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
pub enum RunStepDeltaStepDetailsMessageCreationObjectType {
    #[serde(rename = "message_creation")]
    MessageCreation,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsMessageCreationObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MessageCreation => f.write_str("message_creation"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsMessageCreationObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message_creation" => Ok(Self::MessageCreation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsMessageCreationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsMessageCreationObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter:
        ::std::option::Option<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>,
    #[doc = "The ID of the tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: i64,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeObjectType,
}
impl RunStepDeltaStepDetailsToolCallsCodeObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsCodeObject {
        Default::default()
    }
}
#[doc = "The Code Interpreter tool call definition."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<::std::string::String>,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub outputs:
        ::std::vec::Vec<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem>,
}
impl RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
        Default::default()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem {
    LogsObject(RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject),
    ImageObject(RunStepDeltaStepDetailsToolCallsCodeOutputImageObject),
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject>
    for RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem
{
    fn from(value: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject) -> Self {
        Self::LogsObject(value)
    }
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsCodeOutputImageObject>
    for RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem
{
    fn from(value: RunStepDeltaStepDetailsToolCallsCodeOutputImageObject) -> Self {
        Self::ImageObject(value)
    }
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
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
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsCodeObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsCodeObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsCodeObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsToolCallsCodeObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsCodeOutputImageObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>,
    #[doc = "The index of the output in the outputs array."]
    pub index: i64,
    #[doc = "Always `image`."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
}
impl RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
        Default::default()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](https://developers.openai.com/api/reference/resources/files) ID of the image."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
}
impl RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
        Default::default()
    }
}
#[doc = "Always `image`."]
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
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "The index of the output in the outputs array."]
    pub index: i64,
    #[doc = "The text output from the Code Interpreter tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logs: ::std::option::Option<::std::string::String>,
    #[doc = "Always `logs`."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
}
impl RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
        Default::default()
    }
}
#[doc = "Always `logs`."]
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
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    #[serde(rename = "logs")]
    Logs,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logs => f.write_str("logs"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logs" => Ok(Self::Logs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsFileSearchObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    #[doc = "For now, this is always going to be an empty object."]
    pub file_search: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The ID of the tool call object."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: i64,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
}
impl RunStepDeltaStepDetailsToolCallsFileSearchObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsFileSearchObject {
        Default::default()
    }
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
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
pub enum RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsToolCallsFileSearchObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsFunctionObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function: ::std::option::Option<RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>,
    #[doc = "The ID of the tool call object."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: i64,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsFunctionObjectType,
}
impl RunStepDeltaStepDetailsToolCallsFunctionObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsFunctionObject {
        Default::default()
    }
}
#[doc = "The definition of the function that was called."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The arguments passed to the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub arguments: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
}
impl RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
        Default::default()
    }
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
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
pub enum RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDeltaStepDetailsToolCallsFunctionObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_calls: ::std::vec::Vec<RunStepDeltaStepDetailsToolCallsObjectToolCallsItem>,
    #[doc = "Always `tool_calls`."]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsObjectType,
}
impl RunStepDeltaStepDetailsToolCallsObject {
    pub fn builder() -> builder::RunStepDeltaStepDetailsToolCallsObject {
        Default::default()
    }
}
#[doc = "`RunStepDeltaStepDetailsToolCallsObjectToolCallsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsObjectToolCallsItem {
    CodeObject(RunStepDeltaStepDetailsToolCallsCodeObject),
    FileSearchObject(RunStepDeltaStepDetailsToolCallsFileSearchObject),
    FunctionObject(RunStepDeltaStepDetailsToolCallsFunctionObject),
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsCodeObject>
    for RunStepDeltaStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDeltaStepDetailsToolCallsCodeObject) -> Self {
        Self::CodeObject(value)
    }
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsFileSearchObject>
    for RunStepDeltaStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDeltaStepDetailsToolCallsFileSearchObject) -> Self {
        Self::FileSearchObject(value)
    }
}
impl ::std::convert::From<RunStepDeltaStepDetailsToolCallsFunctionObject>
    for RunStepDeltaStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDeltaStepDetailsToolCallsFunctionObject) -> Self {
        Self::FunctionObject(value)
    }
}
#[doc = "Always `tool_calls`."]
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
pub enum RunStepDeltaStepDetailsToolCallsObjectType {
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
impl ::std::fmt::Display for RunStepDeltaStepDetailsToolCallsObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolCalls => f.write_str("tool_calls"),
        }
    }
}
impl ::std::str::FromStr for RunStepDeltaStepDetailsToolCallsObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_calls" => Ok(Self::ToolCalls),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDeltaStepDetailsToolCallsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDeltaStepDetailsToolCallsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the message creation by the run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsMessageCreationObject {
    pub message_creation: RunStepDetailsMessageCreationObjectMessageCreation,
    #[doc = "Always `message_creation`."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsMessageCreationObjectType,
}
impl RunStepDetailsMessageCreationObject {
    pub fn builder() -> builder::RunStepDetailsMessageCreationObject {
        Default::default()
    }
}
#[doc = "`RunStepDetailsMessageCreationObjectMessageCreation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    pub message_id: ::std::string::String,
}
impl RunStepDetailsMessageCreationObjectMessageCreation {
    pub fn builder() -> builder::RunStepDetailsMessageCreationObjectMessageCreation {
        Default::default()
    }
}
#[doc = "Always `message_creation`."]
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
pub enum RunStepDetailsMessageCreationObjectType {
    #[serde(rename = "message_creation")]
    MessageCreation,
}
impl ::std::fmt::Display for RunStepDetailsMessageCreationObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MessageCreation => f.write_str("message_creation"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsMessageCreationObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message_creation" => Ok(Self::MessageCreation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsMessageCreationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDetailsMessageCreationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsCodeObject {
    pub code_interpreter: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
    #[doc = "The ID of the tool call."]
    pub id: ::std::string::String,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeObjectType,
}
impl RunStepDetailsToolCallsCodeObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsCodeObject {
        Default::default()
    }
}
#[doc = "The Code Interpreter tool call definition."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    pub input: ::std::string::String,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    pub outputs: ::std::vec::Vec<RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem>,
}
impl RunStepDetailsToolCallsCodeObjectCodeInterpreter {
    pub fn builder() -> builder::RunStepDetailsToolCallsCodeObjectCodeInterpreter {
        Default::default()
    }
}
#[doc = "`RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem {
    LogsObject(RunStepDetailsToolCallsCodeOutputLogsObject),
    ImageObject(RunStepDetailsToolCallsCodeOutputImageObject),
}
impl ::std::convert::From<RunStepDetailsToolCallsCodeOutputLogsObject>
    for RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem
{
    fn from(value: RunStepDetailsToolCallsCodeOutputLogsObject) -> Self {
        Self::LogsObject(value)
    }
}
impl ::std::convert::From<RunStepDetailsToolCallsCodeOutputImageObject>
    for RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem
{
    fn from(value: RunStepDetailsToolCallsCodeOutputImageObject) -> Self {
        Self::ImageObject(value)
    }
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
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
pub enum RunStepDetailsToolCallsCodeObjectType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsCodeObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsCodeObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsCodeObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDetailsToolCallsCodeObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDetailsToolCallsCodeOutputImageObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    pub image: RunStepDetailsToolCallsCodeOutputImageObjectImage,
    #[doc = "Always `image`."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeOutputImageObjectType,
}
impl RunStepDetailsToolCallsCodeOutputImageObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsCodeOutputImageObject {
        Default::default()
    }
}
#[doc = "`RunStepDetailsToolCallsCodeOutputImageObjectImage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](https://developers.openai.com/api/reference/resources/files) ID of the image."]
    pub file_id: ::std::string::String,
}
impl RunStepDetailsToolCallsCodeOutputImageObjectImage {
    pub fn builder() -> builder::RunStepDetailsToolCallsCodeOutputImageObjectImage {
        Default::default()
    }
}
#[doc = "Always `image`."]
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
pub enum RunStepDetailsToolCallsCodeOutputImageObjectType {
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsCodeOutputImageObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsCodeOutputImageObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsCodeOutputImageObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDetailsToolCallsCodeOutputImageObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "The text output from the Code Interpreter tool call."]
    pub logs: ::std::string::String,
    #[doc = "Always `logs`."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeOutputLogsObjectType,
}
impl RunStepDetailsToolCallsCodeOutputLogsObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsCodeOutputLogsObject {
        Default::default()
    }
}
#[doc = "Always `logs`."]
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
pub enum RunStepDetailsToolCallsCodeOutputLogsObjectType {
    #[serde(rename = "logs")]
    Logs,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsCodeOutputLogsObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logs => f.write_str("logs"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsCodeOutputLogsObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logs" => Ok(Self::Logs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsCodeOutputLogsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDetailsToolCallsCodeOutputLogsObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDetailsToolCallsFileSearchObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchObject {
    pub file_search: RunStepDetailsToolCallsFileSearchObjectFileSearch,
    #[doc = "The ID of the tool call object."]
    pub id: ::std::string::String,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsFileSearchObjectType,
}
impl RunStepDetailsToolCallsFileSearchObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsFileSearchObject {
        Default::default()
    }
}
#[doc = "For now, this is always going to be an empty object."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranking_options:
        ::std::option::Option<RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
    #[doc = "The results of the file search."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub results: ::std::vec::Vec<RunStepDetailsToolCallsFileSearchResultObject>,
}
impl RunStepDetailsToolCallsFileSearchObjectFileSearch {
    pub fn builder() -> builder::RunStepDetailsToolCallsFileSearchObjectFileSearch {
        Default::default()
    }
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
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
pub enum RunStepDetailsToolCallsFileSearchObjectType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsFileSearchObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsFileSearchObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsFileSearchObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDetailsToolCallsFileSearchObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The ranking options for the file search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    pub ranker: FileSearchRanker,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    pub score_threshold: f64,
}
impl RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsFileSearchRankingOptionsObject {
        Default::default()
    }
}
#[doc = "A result instance of the file search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    #[doc = "The content of the result that was found. The content is only included if requested via the include query parameter."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub content: ::std::vec::Vec<RunStepDetailsToolCallsFileSearchResultObjectContentItem>,
    #[doc = "The ID of the file that result was found in."]
    pub file_id: ::std::string::String,
    #[doc = "The name of the file that result was found in."]
    pub file_name: ::std::string::String,
    #[doc = "The score of the result. All values must be a floating point number between 0 and 1."]
    pub score: f64,
}
impl RunStepDetailsToolCallsFileSearchResultObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsFileSearchResultObject {
        Default::default()
    }
}
#[doc = "`RunStepDetailsToolCallsFileSearchResultObjectContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RunStepDetailsToolCallsFileSearchResultObjectContentItem {
    #[doc = "The text content of the file."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the content."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<RunStepDetailsToolCallsFileSearchResultObjectContentItemType>,
}
impl RunStepDetailsToolCallsFileSearchResultObjectContentItem {
    pub fn builder() -> builder::RunStepDetailsToolCallsFileSearchResultObjectContentItem {
        Default::default()
    }
}
#[doc = "The type of the content."]
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
pub enum RunStepDetailsToolCallsFileSearchResultObjectContentItemType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsFileSearchResultObjectContentItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsFileSearchResultObjectContentItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for RunStepDetailsToolCallsFileSearchResultObjectContentItemType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RunStepDetailsToolCallsFileSearchResultObjectContentItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepDetailsToolCallsFunctionObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsFunctionObject {
    pub function: RunStepDetailsToolCallsFunctionObjectFunction,
    #[doc = "The ID of the tool call object."]
    pub id: ::std::string::String,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsFunctionObjectType,
}
impl RunStepDetailsToolCallsFunctionObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsFunctionObject {
        Default::default()
    }
}
#[doc = "The definition of the function that was called."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The arguments passed to the function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function."]
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub output: ::std::option::Option<::std::string::String>,
}
impl RunStepDetailsToolCallsFunctionObjectFunction {
    pub fn builder() -> builder::RunStepDetailsToolCallsFunctionObjectFunction {
        Default::default()
    }
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
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
pub enum RunStepDetailsToolCallsFunctionObjectType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsFunctionObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsFunctionObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsFunctionObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDetailsToolCallsFunctionObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of the tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepDetailsToolCallsObject {
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    pub tool_calls: ::std::vec::Vec<RunStepDetailsToolCallsObjectToolCallsItem>,
    #[doc = "Always `tool_calls`."]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsObjectType,
}
impl RunStepDetailsToolCallsObject {
    pub fn builder() -> builder::RunStepDetailsToolCallsObject {
        Default::default()
    }
}
#[doc = "`RunStepDetailsToolCallsObjectToolCallsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsObjectToolCallsItem {
    CodeObject(RunStepDetailsToolCallsCodeObject),
    FileSearchObject(RunStepDetailsToolCallsFileSearchObject),
    FunctionObject(RunStepDetailsToolCallsFunctionObject),
}
impl ::std::convert::From<RunStepDetailsToolCallsCodeObject>
    for RunStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDetailsToolCallsCodeObject) -> Self {
        Self::CodeObject(value)
    }
}
impl ::std::convert::From<RunStepDetailsToolCallsFileSearchObject>
    for RunStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDetailsToolCallsFileSearchObject) -> Self {
        Self::FileSearchObject(value)
    }
}
impl ::std::convert::From<RunStepDetailsToolCallsFunctionObject>
    for RunStepDetailsToolCallsObjectToolCallsItem
{
    fn from(value: RunStepDetailsToolCallsFunctionObject) -> Self {
        Self::FunctionObject(value)
    }
}
#[doc = "Always `tool_calls`."]
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
pub enum RunStepDetailsToolCallsObjectType {
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
impl ::std::fmt::Display for RunStepDetailsToolCallsObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolCalls => f.write_str("tool_calls"),
        }
    }
}
impl ::std::str::FromStr for RunStepDetailsToolCallsObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_calls" => Ok(Self::ToolCalls),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepDetailsToolCallsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepDetailsToolCallsObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a step in execution of a run.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepObject {
    #[doc = "The ID of the [assistant](https://developers.openai.com/api/docs/assistants/migration) associated with the run step."]
    pub assistant_id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub cancelled_at: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) for when the run step was created."]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expired_at: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failed_at: ::std::option::Option<i64>,
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_error: ::std::option::Option<RunStepObjectLastError>,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The object type, which is always `thread.run.step`."]
    pub object: RunStepObjectObject,
    #[doc = "The ID of the [run](https://developers.openai.com/api/docs/assistants/migration) that this run step is a part of."]
    pub run_id: ::std::string::String,
    #[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
    pub status: RunStepObjectStatus,
    #[doc = "The details of the run step."]
    pub step_details: RunStepObjectStepDetails,
    #[doc = "The ID of the [thread](https://developers.openai.com/api/docs/assistants/migration) that was run."]
    pub thread_id: ::std::string::String,
    #[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
    #[serde(rename = "type")]
    pub type_: RunStepObjectType,
    pub usage: RunStepCompletionUsage,
}
impl RunStepObject {
    pub fn builder() -> builder::RunStepObject {
        Default::default()
    }
}
#[doc = "The last error associated with this run step. Will be `null` if there are no errors."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunStepObjectLastError {
    #[doc = "One of `server_error` or `rate_limit_exceeded`."]
    pub code: RunStepObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: ::std::string::String,
}
impl RunStepObjectLastError {
    pub fn builder() -> builder::RunStepObjectLastError {
        Default::default()
    }
}
#[doc = "One of `server_error` or `rate_limit_exceeded`."]
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
pub enum RunStepObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
}
impl ::std::fmt::Display for RunStepObjectLastErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerError => f.write_str("server_error"),
            Self::RateLimitExceeded => f.write_str("rate_limit_exceeded"),
        }
    }
}
impl ::std::str::FromStr for RunStepObjectLastErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_error" => Ok(Self::ServerError),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepObjectLastErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `thread.run.step`."]
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
pub enum RunStepObjectObject {
    #[serde(rename = "thread.run.step")]
    ThreadRunStep,
}
impl ::std::fmt::Display for RunStepObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThreadRunStep => f.write_str("thread.run.step"),
        }
    }
}
impl ::std::str::FromStr for RunStepObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread.run.step" => Ok(Self::ThreadRunStep),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
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
pub enum RunStepObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for RunStepObjectStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Failed => f.write_str("failed"),
            Self::Completed => f.write_str("completed"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for RunStepObjectStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "cancelled" => Ok(Self::Cancelled),
            "failed" => Ok(Self::Failed),
            "completed" => Ok(Self::Completed),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepObjectStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The details of the run step."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RunStepObjectStepDetails {
    MessageCreationObject(RunStepDetailsMessageCreationObject),
    ToolCallsObject(RunStepDetailsToolCallsObject),
}
impl ::std::convert::From<RunStepDetailsMessageCreationObject> for RunStepObjectStepDetails {
    fn from(value: RunStepDetailsMessageCreationObject) -> Self {
        Self::MessageCreationObject(value)
    }
}
impl ::std::convert::From<RunStepDetailsToolCallsObject> for RunStepObjectStepDetails {
    fn from(value: RunStepDetailsToolCallsObject) -> Self {
        Self::ToolCallsObject(value)
    }
}
#[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
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
pub enum RunStepObjectType {
    #[serde(rename = "message_creation")]
    MessageCreation,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
impl ::std::fmt::Display for RunStepObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MessageCreation => f.write_str("message_creation"),
            Self::ToolCalls => f.write_str("tool_calls"),
        }
    }
}
impl ::std::str::FromStr for RunStepObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message_creation" => Ok(Self::MessageCreation),
            "tool_calls" => Ok(Self::ToolCalls),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunStepObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunStepObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RunStepStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "event", content = "data")]
pub enum RunStepStreamEvent {
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) is created."]
    #[serde(rename = "thread.run.step.created")]
    ThreadRunStepCreated(RunStepObject),
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) moves to an `in_progress` state."]
    #[serde(rename = "thread.run.step.in_progress")]
    ThreadRunStepInProgress(RunStepObject),
    #[doc = "Occurs when parts of a [run step](https://developers.openai.com/api/docs/assistants/migration) are being streamed."]
    #[serde(rename = "thread.run.step.delta")]
    ThreadRunStepDelta(RunStepDeltaObject),
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) is completed."]
    #[serde(rename = "thread.run.step.completed")]
    ThreadRunStepCompleted(RunStepObject),
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) fails."]
    #[serde(rename = "thread.run.step.failed")]
    ThreadRunStepFailed(RunStepObject),
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) is cancelled."]
    #[serde(rename = "thread.run.step.cancelled")]
    ThreadRunStepCancelled(RunStepObject),
    #[doc = "Occurs when a [run step](https://developers.openai.com/api/docs/assistants/migration) expires."]
    #[serde(rename = "thread.run.step.expired")]
    ThreadRunStepExpired(RunStepObject),
}
impl ::std::convert::From<RunStepDeltaObject> for RunStepStreamEvent {
    fn from(value: RunStepDeltaObject) -> Self {
        Self::ThreadRunStepDelta(value)
    }
}
#[doc = "`RunStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "event", content = "data")]
pub enum RunStreamEvent {
    #[doc = "Occurs when a new [run](https://developers.openai.com/api/docs/assistants/migration) is created."]
    #[serde(rename = "thread.run.created")]
    ThreadRunCreated(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) moves to a `queued` status."]
    #[serde(rename = "thread.run.queued")]
    ThreadRunQueued(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) moves to an `in_progress` status."]
    #[serde(rename = "thread.run.in_progress")]
    ThreadRunInProgress(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) moves to a `requires_action` status."]
    #[serde(rename = "thread.run.requires_action")]
    ThreadRunRequiresAction(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) is completed."]
    #[serde(rename = "thread.run.completed")]
    ThreadRunCompleted(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) ends with status `incomplete`."]
    #[serde(rename = "thread.run.incomplete")]
    ThreadRunIncomplete(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) fails."]
    #[serde(rename = "thread.run.failed")]
    ThreadRunFailed(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) moves to a `cancelling` status."]
    #[serde(rename = "thread.run.cancelling")]
    ThreadRunCancelling(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) is cancelled."]
    #[serde(rename = "thread.run.cancelled")]
    ThreadRunCancelled(RunObject),
    #[doc = "Occurs when a [run](https://developers.openai.com/api/docs/assistants/migration) expires."]
    #[serde(rename = "thread.run.expired")]
    ThreadRunExpired(RunObject),
}
#[doc = "Tool call objects"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunToolCallObject {
    pub function: RunToolCallObjectFunction,
    #[doc = "The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](https://developers.openai.com/api/docs/assistants/migration) endpoint."]
    pub id: ::std::string::String,
    #[doc = "The type of tool call the output is required for. For now, this is always `function`."]
    #[serde(rename = "type")]
    pub type_: RunToolCallObjectType,
}
impl RunToolCallObject {
    pub fn builder() -> builder::RunToolCallObject {
        Default::default()
    }
}
#[doc = "The function definition."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RunToolCallObjectFunction {
    #[doc = "The arguments that the model expects you to pass to the function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function."]
    pub name: ::std::string::String,
}
impl RunToolCallObjectFunction {
    pub fn builder() -> builder::RunToolCallObjectFunction {
        Default::default()
    }
}
#[doc = "The type of tool call the output is required for. For now, this is always `function`."]
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
pub enum RunToolCallObjectType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for RunToolCallObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for RunToolCallObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RunToolCallObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RunToolCallObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SubmitToolOuputsToRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SubmitToolOuputsToRunParams {
    pub run_id: ::std::string::String,
    pub thread_id: ::std::string::String,
}
impl SubmitToolOuputsToRunParams {
    pub fn builder() -> builder::SubmitToolOuputsToRunParams {
        Default::default()
    }
}
#[doc = "`SubmitToolOutputsRunRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SubmitToolOutputsRunRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "A list of tools for which the outputs are being submitted."]
    pub tool_outputs: ::std::vec::Vec<SubmitToolOutputsRunRequestToolOutputsItem>,
}
impl SubmitToolOutputsRunRequest {
    pub fn builder() -> builder::SubmitToolOutputsRunRequest {
        Default::default()
    }
}
#[doc = "`SubmitToolOutputsRunRequestToolOutputsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct SubmitToolOutputsRunRequestToolOutputsItem {
    #[doc = "The output of the tool call to be submitted to continue the run."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the tool call in the `required_action` object within the run object the output is being submitted for."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_call_id: ::std::option::Option<::std::string::String>,
}
impl SubmitToolOutputsRunRequestToolOutputsItem {
    pub fn builder() -> builder::SubmitToolOutputsRunRequestToolOutputsItem {
        Default::default()
    }
}
#[doc = "Represents a thread that contains [messages](https://developers.openai.com/api/docs/assistants/migration)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ThreadObject {
    #[doc = "The Unix timestamp (in seconds) for when the thread was created."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The object type, which is always `thread`."]
    pub object: ThreadObjectObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub tool_resources: ::std::option::Option<ThreadObjectToolResources>,
}
impl ThreadObject {
    pub fn builder() -> builder::ThreadObject {
        Default::default()
    }
}
#[doc = "The object type, which is always `thread`."]
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
pub enum ThreadObjectObject {
    #[serde(rename = "thread")]
    Thread,
}
impl ::std::fmt::Display for ThreadObjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Thread => f.write_str("thread"),
        }
    }
}
impl ::std::str::FromStr for ThreadObjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "thread" => Ok(Self::Thread),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadObjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ThreadObjectToolResources {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code_interpreter: ::std::option::Option<ThreadObjectToolResourcesCodeInterpreter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_search: ::std::option::Option<ThreadObjectToolResourcesFileSearch>,
}
impl ThreadObjectToolResources {
    pub fn builder() -> builder::ThreadObjectToolResources {
        Default::default()
    }
}
#[doc = "`ThreadObjectToolResourcesCodeInterpreter`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ThreadObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](https://developers.openai.com/api/reference/resources/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
}
impl ThreadObjectToolResourcesCodeInterpreter {
    pub fn builder() -> builder::ThreadObjectToolResourcesCodeInterpreter {
        Default::default()
    }
}
#[doc = "`ThreadObjectToolResourcesFileSearch`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ThreadObjectToolResourcesFileSearch {
    #[doc = "The [vector store](https://developers.openai.com/api/reference/resources/vector_stores) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
impl ThreadObjectToolResourcesFileSearch {
    pub fn builder() -> builder::ThreadObjectToolResourcesFileSearch {
        Default::default()
    }
}
#[doc = "`ThreadStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "event")]
pub enum ThreadStreamEvent {
    #[doc = "Occurs when a new [thread](https://developers.openai.com/api/docs/assistants/migration) is created."]
    #[serde(rename = "thread.created")]
    ThreadCreated {
        data: ThreadObject,
        #[doc = "Whether to enable input audio transcription."]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        enabled: ::std::option::Option<bool>,
    },
}
#[doc = "Controls for how a thread will be truncated prior to the run. Use this to control the initial context window of the run."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TruncationObject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_messages: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
    #[serde(rename = "type")]
    pub type_: TruncationObjectType,
}
impl TruncationObject {
    pub fn builder() -> builder::TruncationObject {
        Default::default()
    }
}
#[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
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
pub enum TruncationObjectType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "last_messages")]
    LastMessages,
}
impl ::std::fmt::Display for TruncationObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::LastMessages => f.write_str("last_messages"),
        }
    }
}
impl ::std::str::FromStr for TruncationObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "last_messages" => Ok(Self::LastMessages),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TruncationObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TruncationObjectType {
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
    pub struct AssistantObject {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectDescription>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectInstructions>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectName>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::AssistantObjectObject, ::std::string::String>,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectToolResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::AssistantObjectToolsItem>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantObject {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                instructions: Err("no value supplied for instructions".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                model: Err("no value supplied for model".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                response_format: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
                tools: Err("no value supplied for tools".to_string()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl AssistantObject {
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
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantObjectDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
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
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantObjectInstructions>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
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
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantObjectName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantObjectToolResources>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AssistantObjectToolsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantObject> for super::AssistantObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                description: value.description?,
                id: value.id?,
                instructions: value.instructions?,
                metadata: value.metadata?,
                model: value.model?,
                name: value.name?,
                object: value.object?,
                response_format: value.response_format?,
                temperature: value.temperature?,
                tool_resources: value.tool_resources?,
                tools: value.tools?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantObject> for AssistantObject {
        fn from(value: super::AssistantObject) -> Self {
            Self {
                created_at: Ok(value.created_at),
                description: Ok(value.description),
                id: Ok(value.id),
                instructions: Ok(value.instructions),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                name: Ok(value.name),
                object: Ok(value.object),
                response_format: Ok(value.response_format),
                temperature: Ok(value.temperature),
                tool_resources: Ok(value.tool_resources),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantObjectToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::AssistantObjectToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AssistantObjectToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl AssistantObjectToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantObjectToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantObjectToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantObjectToolResources> for super::AssistantObjectToolResources {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantObjectToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantObjectToolResources> for AssistantObjectToolResources {
        fn from(value: super::AssistantObjectToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantObjectToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantObjectToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl AssistantObjectToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<AssistantObjectToolResourcesCodeInterpreter>
        for super::AssistantObjectToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantObjectToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantObjectToolResourcesCodeInterpreter>
        for AssistantObjectToolResourcesCodeInterpreter
    {
        fn from(value: super::AssistantObjectToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantObjectToolResourcesFileSearch {
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantObjectToolResourcesFileSearch {
        fn default() -> Self {
            Self {
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl AssistantObjectToolResourcesFileSearch {
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
    impl ::std::convert::TryFrom<AssistantObjectToolResourcesFileSearch>
        for super::AssistantObjectToolResourcesFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantObjectToolResourcesFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantObjectToolResourcesFileSearch>
        for AssistantObjectToolResourcesFileSearch
    {
        fn from(value: super::AssistantObjectToolResourcesFileSearch) -> Self {
            Self {
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantToolsCode {
        type_: ::std::result::Result<super::AssistantToolsCodeType, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantToolsCode {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantToolsCode {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantToolsCodeType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantToolsCode> for super::AssistantToolsCode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantToolsCode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantToolsCode> for AssistantToolsCode {
        fn from(value: super::AssistantToolsCode) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantToolsFileSearch {
        file_search: ::std::result::Result<
            ::std::option::Option<super::AssistantToolsFileSearchFileSearch>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AssistantToolsFileSearchType, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantToolsFileSearch {
        fn default() -> Self {
            Self {
                file_search: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantToolsFileSearch {
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantToolsFileSearchFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantToolsFileSearchType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantToolsFileSearch> for super::AssistantToolsFileSearch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantToolsFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_search: value.file_search?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantToolsFileSearch> for AssistantToolsFileSearch {
        fn from(value: super::AssistantToolsFileSearch) -> Self {
            Self {
                file_search: Ok(value.file_search),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantToolsFileSearchFileSearch {
        max_num_results: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        ranking_options: ::std::result::Result<
            ::std::option::Option<super::FileSearchRankingOptions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AssistantToolsFileSearchFileSearch {
        fn default() -> Self {
            Self {
                max_num_results: Ok(Default::default()),
                ranking_options: Ok(Default::default()),
            }
        }
    }
    impl AssistantToolsFileSearchFileSearch {
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
        pub fn ranking_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileSearchRankingOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.ranking_options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranking_options: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantToolsFileSearchFileSearch>
        for super::AssistantToolsFileSearchFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantToolsFileSearchFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_num_results: value.max_num_results?,
                ranking_options: value.ranking_options?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantToolsFileSearchFileSearch>
        for AssistantToolsFileSearchFileSearch
    {
        fn from(value: super::AssistantToolsFileSearchFileSearch) -> Self {
            Self {
                max_num_results: Ok(value.max_num_results),
                ranking_options: Ok(value.ranking_options),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantToolsFileSearchTypeOnly {
        type_: ::std::result::Result<
            super::AssistantToolsFileSearchTypeOnlyType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AssistantToolsFileSearchTypeOnly {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantToolsFileSearchTypeOnly {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantToolsFileSearchTypeOnlyType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantToolsFileSearchTypeOnly>
        for super::AssistantToolsFileSearchTypeOnly
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantToolsFileSearchTypeOnly,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantToolsFileSearchTypeOnly>
        for AssistantToolsFileSearchTypeOnly
    {
        fn from(value: super::AssistantToolsFileSearchTypeOnly) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantToolsFunction {
        function: ::std::result::Result<
            ::openai_api_types::common::FunctionObject,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AssistantToolsFunctionType, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantToolsFunction {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantToolsFunction {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::FunctionObject>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantToolsFunctionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantToolsFunction> for super::AssistantToolsFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantToolsFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantToolsFunction> for AssistantToolsFunction {
        fn from(value: super::AssistantToolsFunction) -> Self {
            Self {
                function: Ok(value.function),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantsApiSchema {
        cancel_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_run_params: ::std::result::Result<super::CancelRunParams, ::std::string::String>,
        cancel_run_response: ::std::result::Result<super::RunObject, ::std::string::String>,
        create_assistant_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_assistant_request:
            ::std::result::Result<super::CreateAssistantRequest, ::std::string::String>,
        create_assistant_response:
            ::std::result::Result<super::AssistantObject, ::std::string::String>,
        create_message_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_message_params:
            ::std::result::Result<super::CreateMessageParams, ::std::string::String>,
        create_message_request:
            ::std::result::Result<super::CreateMessageRequest, ::std::string::String>,
        create_message_response: ::std::result::Result<super::MessageObject, ::std::string::String>,
        create_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_run_params: ::std::result::Result<super::CreateRunParams, ::std::string::String>,
        create_run_request: ::std::result::Result<super::CreateRunRequest, ::std::string::String>,
        create_run_response: ::std::result::Result<super::RunObject, ::std::string::String>,
        create_run_stream_response:
            ::std::result::Result<super::AssistantStreamEvent, ::std::string::String>,
        create_thread_and_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_thread_and_run_request:
            ::std::result::Result<super::CreateThreadAndRunRequest, ::std::string::String>,
        create_thread_and_run_response:
            ::std::result::Result<super::RunObject, ::std::string::String>,
        create_thread_and_run_stream_response:
            ::std::result::Result<super::AssistantStreamEvent, ::std::string::String>,
        create_thread_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_thread_request:
            ::std::result::Result<super::CreateThreadRequest, ::std::string::String>,
        create_thread_response: ::std::result::Result<super::ThreadObject, ::std::string::String>,
        delete_assistant_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_assistant_params:
            ::std::result::Result<super::DeleteAssistantParams, ::std::string::String>,
        delete_assistant_response:
            ::std::result::Result<super::DeleteAssistantResponse, ::std::string::String>,
        delete_message_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_message_params:
            ::std::result::Result<super::DeleteMessageParams, ::std::string::String>,
        delete_message_response:
            ::std::result::Result<super::DeleteMessageResponse, ::std::string::String>,
        delete_thread_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_thread_params:
            ::std::result::Result<super::DeleteThreadParams, ::std::string::String>,
        delete_thread_response:
            ::std::result::Result<super::DeleteThreadResponse, ::std::string::String>,
        get_assistant_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_assistant_params:
            ::std::result::Result<super::GetAssistantParams, ::std::string::String>,
        get_assistant_response:
            ::std::result::Result<super::AssistantObject, ::std::string::String>,
        get_message_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_message_params: ::std::result::Result<super::GetMessageParams, ::std::string::String>,
        get_message_response: ::std::result::Result<super::MessageObject, ::std::string::String>,
        get_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_run_params: ::std::result::Result<super::GetRunParams, ::std::string::String>,
        get_run_response: ::std::result::Result<super::RunObject, ::std::string::String>,
        get_run_step_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_run_step_params: ::std::result::Result<super::GetRunStepParams, ::std::string::String>,
        get_run_step_response: ::std::result::Result<super::RunStepObject, ::std::string::String>,
        get_thread_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_thread_params: ::std::result::Result<super::GetThreadParams, ::std::string::String>,
        get_thread_response: ::std::result::Result<super::ThreadObject, ::std::string::String>,
        list_assistants_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_assistants_params:
            ::std::result::Result<super::ListAssistantsParams, ::std::string::String>,
        list_assistants_response:
            ::std::result::Result<super::ListAssistantsResponse, ::std::string::String>,
        list_messages_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_messages_params:
            ::std::result::Result<super::ListMessagesParams, ::std::string::String>,
        list_messages_response:
            ::std::result::Result<super::ListMessagesResponse, ::std::string::String>,
        list_run_steps_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_run_steps_params:
            ::std::result::Result<super::ListRunStepsParams, ::std::string::String>,
        list_run_steps_response:
            ::std::result::Result<super::ListRunStepsResponse, ::std::string::String>,
        list_runs_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_runs_params: ::std::result::Result<super::ListRunsParams, ::std::string::String>,
        list_runs_response: ::std::result::Result<super::ListRunsResponse, ::std::string::String>,
        modify_assistant_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_assistant_params:
            ::std::result::Result<super::ModifyAssistantParams, ::std::string::String>,
        modify_assistant_request:
            ::std::result::Result<super::ModifyAssistantRequest, ::std::string::String>,
        modify_assistant_response:
            ::std::result::Result<super::AssistantObject, ::std::string::String>,
        modify_message_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_message_params:
            ::std::result::Result<super::ModifyMessageParams, ::std::string::String>,
        modify_message_request:
            ::std::result::Result<super::ModifyMessageRequest, ::std::string::String>,
        modify_message_response: ::std::result::Result<super::MessageObject, ::std::string::String>,
        modify_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_run_params: ::std::result::Result<super::ModifyRunParams, ::std::string::String>,
        modify_run_request: ::std::result::Result<super::ModifyRunRequest, ::std::string::String>,
        modify_run_response: ::std::result::Result<super::RunObject, ::std::string::String>,
        modify_thread_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_thread_params:
            ::std::result::Result<super::ModifyThreadParams, ::std::string::String>,
        modify_thread_request:
            ::std::result::Result<super::ModifyThreadRequest, ::std::string::String>,
        modify_thread_response: ::std::result::Result<super::ThreadObject, ::std::string::String>,
        submit_tool_ouputs_to_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        submit_tool_ouputs_to_run_params:
            ::std::result::Result<super::SubmitToolOuputsToRunParams, ::std::string::String>,
        submit_tool_ouputs_to_run_request:
            ::std::result::Result<super::SubmitToolOutputsRunRequest, ::std::string::String>,
        submit_tool_ouputs_to_run_response:
            ::std::result::Result<super::RunObject, ::std::string::String>,
        submit_tool_ouputs_to_run_stream_response:
            ::std::result::Result<super::AssistantStreamEvent, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantsApiSchema {
        fn default() -> Self {
            Self {
                cancel_run_error429: Err("no value supplied for cancel_run_error429".to_string()),
                cancel_run_params: Err("no value supplied for cancel_run_params".to_string()),
                cancel_run_response: Err("no value supplied for cancel_run_response".to_string()),
                create_assistant_error429: Err(
                    "no value supplied for create_assistant_error429".to_string()
                ),
                create_assistant_request: Err(
                    "no value supplied for create_assistant_request".to_string()
                ),
                create_assistant_response: Err(
                    "no value supplied for create_assistant_response".to_string()
                ),
                create_message_error429: Err(
                    "no value supplied for create_message_error429".to_string()
                ),
                create_message_params: Err(
                    "no value supplied for create_message_params".to_string()
                ),
                create_message_request: Err(
                    "no value supplied for create_message_request".to_string()
                ),
                create_message_response: Err(
                    "no value supplied for create_message_response".to_string()
                ),
                create_run_error429: Err("no value supplied for create_run_error429".to_string()),
                create_run_params: Err("no value supplied for create_run_params".to_string()),
                create_run_request: Err("no value supplied for create_run_request".to_string()),
                create_run_response: Err("no value supplied for create_run_response".to_string()),
                create_run_stream_response: Err(
                    "no value supplied for create_run_stream_response".to_string()
                ),
                create_thread_and_run_error429: Err(
                    "no value supplied for create_thread_and_run_error429".to_string(),
                ),
                create_thread_and_run_request: Err(
                    "no value supplied for create_thread_and_run_request".to_string(),
                ),
                create_thread_and_run_response: Err(
                    "no value supplied for create_thread_and_run_response".to_string(),
                ),
                create_thread_and_run_stream_response: Err(
                    "no value supplied for create_thread_and_run_stream_response".to_string(),
                ),
                create_thread_error429: Err(
                    "no value supplied for create_thread_error429".to_string()
                ),
                create_thread_request: Err(
                    "no value supplied for create_thread_request".to_string()
                ),
                create_thread_response: Err(
                    "no value supplied for create_thread_response".to_string()
                ),
                delete_assistant_error429: Err(
                    "no value supplied for delete_assistant_error429".to_string()
                ),
                delete_assistant_params: Err(
                    "no value supplied for delete_assistant_params".to_string()
                ),
                delete_assistant_response: Err(
                    "no value supplied for delete_assistant_response".to_string()
                ),
                delete_message_error429: Err(
                    "no value supplied for delete_message_error429".to_string()
                ),
                delete_message_params: Err(
                    "no value supplied for delete_message_params".to_string()
                ),
                delete_message_response: Err(
                    "no value supplied for delete_message_response".to_string()
                ),
                delete_thread_error429: Err(
                    "no value supplied for delete_thread_error429".to_string()
                ),
                delete_thread_params: Err("no value supplied for delete_thread_params".to_string()),
                delete_thread_response: Err(
                    "no value supplied for delete_thread_response".to_string()
                ),
                get_assistant_error429: Err(
                    "no value supplied for get_assistant_error429".to_string()
                ),
                get_assistant_params: Err("no value supplied for get_assistant_params".to_string()),
                get_assistant_response: Err(
                    "no value supplied for get_assistant_response".to_string()
                ),
                get_message_error429: Err("no value supplied for get_message_error429".to_string()),
                get_message_params: Err("no value supplied for get_message_params".to_string()),
                get_message_response: Err("no value supplied for get_message_response".to_string()),
                get_run_error429: Err("no value supplied for get_run_error429".to_string()),
                get_run_params: Err("no value supplied for get_run_params".to_string()),
                get_run_response: Err("no value supplied for get_run_response".to_string()),
                get_run_step_error429: Err(
                    "no value supplied for get_run_step_error429".to_string()
                ),
                get_run_step_params: Err("no value supplied for get_run_step_params".to_string()),
                get_run_step_response: Err(
                    "no value supplied for get_run_step_response".to_string()
                ),
                get_thread_error429: Err("no value supplied for get_thread_error429".to_string()),
                get_thread_params: Err("no value supplied for get_thread_params".to_string()),
                get_thread_response: Err("no value supplied for get_thread_response".to_string()),
                list_assistants_error429: Err(
                    "no value supplied for list_assistants_error429".to_string()
                ),
                list_assistants_params: Err(
                    "no value supplied for list_assistants_params".to_string()
                ),
                list_assistants_response: Err(
                    "no value supplied for list_assistants_response".to_string()
                ),
                list_messages_error429: Err(
                    "no value supplied for list_messages_error429".to_string()
                ),
                list_messages_params: Err("no value supplied for list_messages_params".to_string()),
                list_messages_response: Err(
                    "no value supplied for list_messages_response".to_string()
                ),
                list_run_steps_error429: Err(
                    "no value supplied for list_run_steps_error429".to_string()
                ),
                list_run_steps_params: Err(
                    "no value supplied for list_run_steps_params".to_string()
                ),
                list_run_steps_response: Err(
                    "no value supplied for list_run_steps_response".to_string()
                ),
                list_runs_error429: Err("no value supplied for list_runs_error429".to_string()),
                list_runs_params: Err("no value supplied for list_runs_params".to_string()),
                list_runs_response: Err("no value supplied for list_runs_response".to_string()),
                modify_assistant_error429: Err(
                    "no value supplied for modify_assistant_error429".to_string()
                ),
                modify_assistant_params: Err(
                    "no value supplied for modify_assistant_params".to_string()
                ),
                modify_assistant_request: Err(
                    "no value supplied for modify_assistant_request".to_string()
                ),
                modify_assistant_response: Err(
                    "no value supplied for modify_assistant_response".to_string()
                ),
                modify_message_error429: Err(
                    "no value supplied for modify_message_error429".to_string()
                ),
                modify_message_params: Err(
                    "no value supplied for modify_message_params".to_string()
                ),
                modify_message_request: Err(
                    "no value supplied for modify_message_request".to_string()
                ),
                modify_message_response: Err(
                    "no value supplied for modify_message_response".to_string()
                ),
                modify_run_error429: Err("no value supplied for modify_run_error429".to_string()),
                modify_run_params: Err("no value supplied for modify_run_params".to_string()),
                modify_run_request: Err("no value supplied for modify_run_request".to_string()),
                modify_run_response: Err("no value supplied for modify_run_response".to_string()),
                modify_thread_error429: Err(
                    "no value supplied for modify_thread_error429".to_string()
                ),
                modify_thread_params: Err("no value supplied for modify_thread_params".to_string()),
                modify_thread_request: Err(
                    "no value supplied for modify_thread_request".to_string()
                ),
                modify_thread_response: Err(
                    "no value supplied for modify_thread_response".to_string()
                ),
                submit_tool_ouputs_to_run_error429: Err(
                    "no value supplied for submit_tool_ouputs_to_run_error429".to_string(),
                ),
                submit_tool_ouputs_to_run_params: Err(
                    "no value supplied for submit_tool_ouputs_to_run_params".to_string(),
                ),
                submit_tool_ouputs_to_run_request: Err(
                    "no value supplied for submit_tool_ouputs_to_run_request".to_string(),
                ),
                submit_tool_ouputs_to_run_response: Err(
                    "no value supplied for submit_tool_ouputs_to_run_response".to_string(),
                ),
                submit_tool_ouputs_to_run_stream_response: Err(
                    "no value supplied for submit_tool_ouputs_to_run_stream_response".to_string(),
                ),
            }
        }
    }
    impl AssistantsApiSchema {
        pub fn cancel_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_run_error429: {e}")
            });
            self
        }
        pub fn cancel_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_run_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cancel_run_params: {e}"));
            self
        }
        pub fn cancel_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_run_response: {e}")
            });
            self
        }
        pub fn create_assistant_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_assistant_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_assistant_error429: {e}")
            });
            self
        }
        pub fn create_assistant_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateAssistantRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_assistant_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_assistant_request: {e}")
            });
            self
        }
        pub fn create_assistant_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_assistant_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_assistant_response: {e}")
            });
            self
        }
        pub fn create_message_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_message_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_message_error429: {e}")
            });
            self
        }
        pub fn create_message_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_message_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_message_params: {e}")
            });
            self
        }
        pub fn create_message_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_message_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_message_request: {e}")
            });
            self
        }
        pub fn create_message_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_message_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_message_response: {e}")
            });
            self
        }
        pub fn create_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_run_error429: {e}")
            });
            self
        }
        pub fn create_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_run_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for create_run_params: {e}"));
            self
        }
        pub fn create_run_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateRunRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_run_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_run_request: {e}")
            });
            self
        }
        pub fn create_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_run_response: {e}")
            });
            self
        }
        pub fn create_run_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantStreamEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.create_run_stream_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_run_stream_response: {e}")
            });
            self
        }
        pub fn create_thread_and_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_and_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_and_run_error429: {e}")
            });
            self
        }
        pub fn create_thread_and_run_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateThreadAndRunRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_and_run_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_and_run_request: {e}")
            });
            self
        }
        pub fn create_thread_and_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_and_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_and_run_response: {e}")
            });
            self
        }
        pub fn create_thread_and_run_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantStreamEvent>,
            T::Error: ::std::fmt::Display,
        {
            self . create_thread_and_run_stream_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_thread_and_run_stream_response: {e}")) ;
            self
        }
        pub fn create_thread_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_error429: {e}")
            });
            self
        }
        pub fn create_thread_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateThreadRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_request: {e}")
            });
            self
        }
        pub fn create_thread_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadObject>,
            T::Error: ::std::fmt::Display,
        {
            self.create_thread_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_thread_response: {e}")
            });
            self
        }
        pub fn delete_assistant_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_assistant_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_assistant_error429: {e}")
            });
            self
        }
        pub fn delete_assistant_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteAssistantParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_assistant_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_assistant_params: {e}")
            });
            self
        }
        pub fn delete_assistant_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteAssistantResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_assistant_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_assistant_response: {e}")
            });
            self
        }
        pub fn delete_message_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_message_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_message_error429: {e}")
            });
            self
        }
        pub fn delete_message_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteMessageParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_message_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_message_params: {e}")
            });
            self
        }
        pub fn delete_message_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteMessageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_message_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_message_response: {e}")
            });
            self
        }
        pub fn delete_thread_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_error429: {e}")
            });
            self
        }
        pub fn delete_thread_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteThreadParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_params: {e}")
            });
            self
        }
        pub fn delete_thread_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteThreadResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_response: {e}")
            });
            self
        }
        pub fn get_assistant_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_assistant_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_assistant_error429: {e}")
            });
            self
        }
        pub fn get_assistant_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetAssistantParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_assistant_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_assistant_params: {e}")
            });
            self
        }
        pub fn get_assistant_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_assistant_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_assistant_response: {e}")
            });
            self
        }
        pub fn get_message_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_message_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_message_error429: {e}")
            });
            self
        }
        pub fn get_message_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetMessageParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_message_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_message_params: {e}")
            });
            self
        }
        pub fn get_message_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_message_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_message_response: {e}")
            });
            self
        }
        pub fn get_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_error429 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_run_error429: {e}"));
            self
        }
        pub fn get_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_run_params: {e}"));
            self
        }
        pub fn get_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_response = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_run_response: {e}"));
            self
        }
        pub fn get_run_step_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_step_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_run_step_error429: {e}")
            });
            self
        }
        pub fn get_run_step_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetRunStepParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_step_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_run_step_params: {e}")
            });
            self
        }
        pub fn get_run_step_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_run_step_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_run_step_response: {e}")
            });
            self
        }
        pub fn get_thread_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_error429: {e}")
            });
            self
        }
        pub fn get_thread_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetThreadParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_thread_params: {e}"));
            self
        }
        pub fn get_thread_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadObject>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_response: {e}")
            });
            self
        }
        pub fn list_assistants_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_assistants_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_assistants_error429: {e}")
            });
            self
        }
        pub fn list_assistants_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListAssistantsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_assistants_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_assistants_params: {e}")
            });
            self
        }
        pub fn list_assistants_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListAssistantsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_assistants_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_assistants_response: {e}")
            });
            self
        }
        pub fn list_messages_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_messages_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_messages_error429: {e}")
            });
            self
        }
        pub fn list_messages_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListMessagesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_messages_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_messages_params: {e}")
            });
            self
        }
        pub fn list_messages_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListMessagesResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_messages_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_messages_response: {e}")
            });
            self
        }
        pub fn list_run_steps_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_run_steps_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_run_steps_error429: {e}")
            });
            self
        }
        pub fn list_run_steps_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListRunStepsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_run_steps_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_run_steps_params: {e}")
            });
            self
        }
        pub fn list_run_steps_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListRunStepsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_run_steps_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_run_steps_response: {e}")
            });
            self
        }
        pub fn list_runs_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_runs_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_runs_error429: {e}")
            });
            self
        }
        pub fn list_runs_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListRunsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_runs_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_runs_params: {e}"));
            self
        }
        pub fn list_runs_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListRunsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_runs_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_runs_response: {e}")
            });
            self
        }
        pub fn modify_assistant_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_assistant_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_assistant_error429: {e}")
            });
            self
        }
        pub fn modify_assistant_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyAssistantParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_assistant_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_assistant_params: {e}")
            });
            self
        }
        pub fn modify_assistant_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyAssistantRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_assistant_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_assistant_request: {e}")
            });
            self
        }
        pub fn modify_assistant_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantObject>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_assistant_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_assistant_response: {e}")
            });
            self
        }
        pub fn modify_message_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_message_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_message_error429: {e}")
            });
            self
        }
        pub fn modify_message_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyMessageParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_message_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_message_params: {e}")
            });
            self
        }
        pub fn modify_message_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyMessageRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_message_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_message_request: {e}")
            });
            self
        }
        pub fn modify_message_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObject>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_message_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_message_response: {e}")
            });
            self
        }
        pub fn modify_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_run_error429: {e}")
            });
            self
        }
        pub fn modify_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_run_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modify_run_params: {e}"));
            self
        }
        pub fn modify_run_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyRunRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_run_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_run_request: {e}")
            });
            self
        }
        pub fn modify_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_run_response: {e}")
            });
            self
        }
        pub fn modify_thread_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_thread_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_thread_error429: {e}")
            });
            self
        }
        pub fn modify_thread_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyThreadParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_thread_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_thread_params: {e}")
            });
            self
        }
        pub fn modify_thread_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyThreadRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_thread_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_thread_request: {e}")
            });
            self
        }
        pub fn modify_thread_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadObject>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_thread_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_thread_response: {e}")
            });
            self
        }
        pub fn submit_tool_ouputs_to_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.submit_tool_ouputs_to_run_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for submit_tool_ouputs_to_run_error429: {e}"
                )
            });
            self
        }
        pub fn submit_tool_ouputs_to_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubmitToolOuputsToRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.submit_tool_ouputs_to_run_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for submit_tool_ouputs_to_run_params: {e}")
            });
            self
        }
        pub fn submit_tool_ouputs_to_run_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubmitToolOutputsRunRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.submit_tool_ouputs_to_run_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for submit_tool_ouputs_to_run_request: {e}"
                )
            });
            self
        }
        pub fn submit_tool_ouputs_to_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.submit_tool_ouputs_to_run_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for submit_tool_ouputs_to_run_response: {e}"
                )
            });
            self
        }
        pub fn submit_tool_ouputs_to_run_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantStreamEvent>,
            T::Error: ::std::fmt::Display,
        {
            self . submit_tool_ouputs_to_run_stream_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for submit_tool_ouputs_to_run_stream_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantsApiSchema> for super::AssistantsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_run_error429: value.cancel_run_error429?,
                cancel_run_params: value.cancel_run_params?,
                cancel_run_response: value.cancel_run_response?,
                create_assistant_error429: value.create_assistant_error429?,
                create_assistant_request: value.create_assistant_request?,
                create_assistant_response: value.create_assistant_response?,
                create_message_error429: value.create_message_error429?,
                create_message_params: value.create_message_params?,
                create_message_request: value.create_message_request?,
                create_message_response: value.create_message_response?,
                create_run_error429: value.create_run_error429?,
                create_run_params: value.create_run_params?,
                create_run_request: value.create_run_request?,
                create_run_response: value.create_run_response?,
                create_run_stream_response: value.create_run_stream_response?,
                create_thread_and_run_error429: value.create_thread_and_run_error429?,
                create_thread_and_run_request: value.create_thread_and_run_request?,
                create_thread_and_run_response: value.create_thread_and_run_response?,
                create_thread_and_run_stream_response: value
                    .create_thread_and_run_stream_response?,
                create_thread_error429: value.create_thread_error429?,
                create_thread_request: value.create_thread_request?,
                create_thread_response: value.create_thread_response?,
                delete_assistant_error429: value.delete_assistant_error429?,
                delete_assistant_params: value.delete_assistant_params?,
                delete_assistant_response: value.delete_assistant_response?,
                delete_message_error429: value.delete_message_error429?,
                delete_message_params: value.delete_message_params?,
                delete_message_response: value.delete_message_response?,
                delete_thread_error429: value.delete_thread_error429?,
                delete_thread_params: value.delete_thread_params?,
                delete_thread_response: value.delete_thread_response?,
                get_assistant_error429: value.get_assistant_error429?,
                get_assistant_params: value.get_assistant_params?,
                get_assistant_response: value.get_assistant_response?,
                get_message_error429: value.get_message_error429?,
                get_message_params: value.get_message_params?,
                get_message_response: value.get_message_response?,
                get_run_error429: value.get_run_error429?,
                get_run_params: value.get_run_params?,
                get_run_response: value.get_run_response?,
                get_run_step_error429: value.get_run_step_error429?,
                get_run_step_params: value.get_run_step_params?,
                get_run_step_response: value.get_run_step_response?,
                get_thread_error429: value.get_thread_error429?,
                get_thread_params: value.get_thread_params?,
                get_thread_response: value.get_thread_response?,
                list_assistants_error429: value.list_assistants_error429?,
                list_assistants_params: value.list_assistants_params?,
                list_assistants_response: value.list_assistants_response?,
                list_messages_error429: value.list_messages_error429?,
                list_messages_params: value.list_messages_params?,
                list_messages_response: value.list_messages_response?,
                list_run_steps_error429: value.list_run_steps_error429?,
                list_run_steps_params: value.list_run_steps_params?,
                list_run_steps_response: value.list_run_steps_response?,
                list_runs_error429: value.list_runs_error429?,
                list_runs_params: value.list_runs_params?,
                list_runs_response: value.list_runs_response?,
                modify_assistant_error429: value.modify_assistant_error429?,
                modify_assistant_params: value.modify_assistant_params?,
                modify_assistant_request: value.modify_assistant_request?,
                modify_assistant_response: value.modify_assistant_response?,
                modify_message_error429: value.modify_message_error429?,
                modify_message_params: value.modify_message_params?,
                modify_message_request: value.modify_message_request?,
                modify_message_response: value.modify_message_response?,
                modify_run_error429: value.modify_run_error429?,
                modify_run_params: value.modify_run_params?,
                modify_run_request: value.modify_run_request?,
                modify_run_response: value.modify_run_response?,
                modify_thread_error429: value.modify_thread_error429?,
                modify_thread_params: value.modify_thread_params?,
                modify_thread_request: value.modify_thread_request?,
                modify_thread_response: value.modify_thread_response?,
                submit_tool_ouputs_to_run_error429: value.submit_tool_ouputs_to_run_error429?,
                submit_tool_ouputs_to_run_params: value.submit_tool_ouputs_to_run_params?,
                submit_tool_ouputs_to_run_request: value.submit_tool_ouputs_to_run_request?,
                submit_tool_ouputs_to_run_response: value.submit_tool_ouputs_to_run_response?,
                submit_tool_ouputs_to_run_stream_response: value
                    .submit_tool_ouputs_to_run_stream_response?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantsApiSchema> for AssistantsApiSchema {
        fn from(value: super::AssistantsApiSchema) -> Self {
            Self {
                cancel_run_error429: Ok(value.cancel_run_error429),
                cancel_run_params: Ok(value.cancel_run_params),
                cancel_run_response: Ok(value.cancel_run_response),
                create_assistant_error429: Ok(value.create_assistant_error429),
                create_assistant_request: Ok(value.create_assistant_request),
                create_assistant_response: Ok(value.create_assistant_response),
                create_message_error429: Ok(value.create_message_error429),
                create_message_params: Ok(value.create_message_params),
                create_message_request: Ok(value.create_message_request),
                create_message_response: Ok(value.create_message_response),
                create_run_error429: Ok(value.create_run_error429),
                create_run_params: Ok(value.create_run_params),
                create_run_request: Ok(value.create_run_request),
                create_run_response: Ok(value.create_run_response),
                create_run_stream_response: Ok(value.create_run_stream_response),
                create_thread_and_run_error429: Ok(value.create_thread_and_run_error429),
                create_thread_and_run_request: Ok(value.create_thread_and_run_request),
                create_thread_and_run_response: Ok(value.create_thread_and_run_response),
                create_thread_and_run_stream_response: Ok(
                    value.create_thread_and_run_stream_response
                ),
                create_thread_error429: Ok(value.create_thread_error429),
                create_thread_request: Ok(value.create_thread_request),
                create_thread_response: Ok(value.create_thread_response),
                delete_assistant_error429: Ok(value.delete_assistant_error429),
                delete_assistant_params: Ok(value.delete_assistant_params),
                delete_assistant_response: Ok(value.delete_assistant_response),
                delete_message_error429: Ok(value.delete_message_error429),
                delete_message_params: Ok(value.delete_message_params),
                delete_message_response: Ok(value.delete_message_response),
                delete_thread_error429: Ok(value.delete_thread_error429),
                delete_thread_params: Ok(value.delete_thread_params),
                delete_thread_response: Ok(value.delete_thread_response),
                get_assistant_error429: Ok(value.get_assistant_error429),
                get_assistant_params: Ok(value.get_assistant_params),
                get_assistant_response: Ok(value.get_assistant_response),
                get_message_error429: Ok(value.get_message_error429),
                get_message_params: Ok(value.get_message_params),
                get_message_response: Ok(value.get_message_response),
                get_run_error429: Ok(value.get_run_error429),
                get_run_params: Ok(value.get_run_params),
                get_run_response: Ok(value.get_run_response),
                get_run_step_error429: Ok(value.get_run_step_error429),
                get_run_step_params: Ok(value.get_run_step_params),
                get_run_step_response: Ok(value.get_run_step_response),
                get_thread_error429: Ok(value.get_thread_error429),
                get_thread_params: Ok(value.get_thread_params),
                get_thread_response: Ok(value.get_thread_response),
                list_assistants_error429: Ok(value.list_assistants_error429),
                list_assistants_params: Ok(value.list_assistants_params),
                list_assistants_response: Ok(value.list_assistants_response),
                list_messages_error429: Ok(value.list_messages_error429),
                list_messages_params: Ok(value.list_messages_params),
                list_messages_response: Ok(value.list_messages_response),
                list_run_steps_error429: Ok(value.list_run_steps_error429),
                list_run_steps_params: Ok(value.list_run_steps_params),
                list_run_steps_response: Ok(value.list_run_steps_response),
                list_runs_error429: Ok(value.list_runs_error429),
                list_runs_params: Ok(value.list_runs_params),
                list_runs_response: Ok(value.list_runs_response),
                modify_assistant_error429: Ok(value.modify_assistant_error429),
                modify_assistant_params: Ok(value.modify_assistant_params),
                modify_assistant_request: Ok(value.modify_assistant_request),
                modify_assistant_response: Ok(value.modify_assistant_response),
                modify_message_error429: Ok(value.modify_message_error429),
                modify_message_params: Ok(value.modify_message_params),
                modify_message_request: Ok(value.modify_message_request),
                modify_message_response: Ok(value.modify_message_response),
                modify_run_error429: Ok(value.modify_run_error429),
                modify_run_params: Ok(value.modify_run_params),
                modify_run_request: Ok(value.modify_run_request),
                modify_run_response: Ok(value.modify_run_response),
                modify_thread_error429: Ok(value.modify_thread_error429),
                modify_thread_params: Ok(value.modify_thread_params),
                modify_thread_request: Ok(value.modify_thread_request),
                modify_thread_response: Ok(value.modify_thread_response),
                submit_tool_ouputs_to_run_error429: Ok(value.submit_tool_ouputs_to_run_error429),
                submit_tool_ouputs_to_run_params: Ok(value.submit_tool_ouputs_to_run_params),
                submit_tool_ouputs_to_run_request: Ok(value.submit_tool_ouputs_to_run_request),
                submit_tool_ouputs_to_run_response: Ok(value.submit_tool_ouputs_to_run_response),
                submit_tool_ouputs_to_run_stream_response: Ok(
                    value.submit_tool_ouputs_to_run_stream_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantsNamedToolChoice {
        function: ::std::result::Result<
            ::std::option::Option<super::AssistantsNamedToolChoiceFunction>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AssistantsNamedToolChoiceType, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantsNamedToolChoice {
        fn default() -> Self {
            Self {
                function: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantsNamedToolChoice {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsNamedToolChoiceFunction>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantsNamedToolChoiceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantsNamedToolChoice> for super::AssistantsNamedToolChoice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantsNamedToolChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantsNamedToolChoice> for AssistantsNamedToolChoice {
        fn from(value: super::AssistantsNamedToolChoice) -> Self {
            Self {
                function: Ok(value.function),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantsNamedToolChoiceFunction {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantsNamedToolChoiceFunction {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl AssistantsNamedToolChoiceFunction {
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
    }
    impl ::std::convert::TryFrom<AssistantsNamedToolChoiceFunction>
        for super::AssistantsNamedToolChoiceFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantsNamedToolChoiceFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::AssistantsNamedToolChoiceFunction>
        for AssistantsNamedToolChoiceFunction
    {
        fn from(value: super::AssistantsNamedToolChoiceFunction) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelRunParams {
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelRunParams {
        fn default() -> Self {
            Self {
                run_id: Err("no value supplied for run_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl CancelRunParams {
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelRunParams> for super::CancelRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelRunParams> for CancelRunParams {
        fn from(value: super::CancelRunParams) -> Self {
            Self {
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAssistantRequest {
        description: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestDescription>,
            ::std::string::String,
        >,
        instructions: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestInstructions>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::CreateAssistantRequestModel, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestName>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestToolResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::CreateAssistantRequestToolsItem>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateAssistantRequest {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                instructions: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                name: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                response_format: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl CreateAssistantRequest {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateAssistantRequestDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateAssistantRequestInstructions>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateAssistantRequestModel>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateAssistantRequestName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn reasoning_effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_effort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_effort: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateAssistantRequestToolResources>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateAssistantRequestToolsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAssistantRequest> for super::CreateAssistantRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAssistantRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                instructions: value.instructions?,
                metadata: value.metadata?,
                model: value.model?,
                name: value.name?,
                reasoning_effort: value.reasoning_effort?,
                response_format: value.response_format?,
                temperature: value.temperature?,
                tool_resources: value.tool_resources?,
                tools: value.tools?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAssistantRequest> for CreateAssistantRequest {
        fn from(value: super::CreateAssistantRequest) -> Self {
            Self {
                description: Ok(value.description),
                instructions: Ok(value.instructions),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                name: Ok(value.name),
                reasoning_effort: Ok(value.reasoning_effort),
                response_format: Ok(value.response_format),
                temperature: Ok(value.temperature),
                tool_resources: Ok(value.tool_resources),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAssistantRequestModel {
        subtype_0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::AssistantSupportedModels>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateAssistantRequestModel {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl CreateAssistantRequestModel {
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
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantSupportedModels>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAssistantRequestModel> for super::CreateAssistantRequestModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAssistantRequestModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAssistantRequestModel> for CreateAssistantRequestModel {
        fn from(value: super::CreateAssistantRequestModel) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAssistantRequestToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::CreateAssistantRequestToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateAssistantRequestToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl CreateAssistantRequestToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateAssistantRequestToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateAssistantRequestToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateAssistantRequestToolResources>
        for super::CreateAssistantRequestToolResources
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAssistantRequestToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAssistantRequestToolResources>
        for CreateAssistantRequestToolResources
    {
        fn from(value: super::CreateAssistantRequestToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateAssistantRequestToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl CreateAssistantRequestToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<CreateAssistantRequestToolResourcesCodeInterpreter>
        for super::CreateAssistantRequestToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAssistantRequestToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CreateAssistantRequestToolResourcesCodeInterpreter>
        for CreateAssistantRequestToolResourcesCodeInterpreter
    {
        fn from(value: super::CreateAssistantRequestToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem { chunking_strategy : :: std :: result :: Result < :: std :: option :: Option < super :: CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy > , :: std :: string :: String > , file_ids : :: std :: result :: Result < :: std :: vec :: Vec < :: std :: string :: String > , :: std :: string :: String > , metadata : :: std :: result :: Result < :: std :: option :: Option < :: openai_api_types :: common :: Metadata > , :: std :: string :: String > , }
    impl ::std::default::Default
        for CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        fn default() -> Self {
            Self {
                chunking_strategy: Ok(Default::default()),
                file_ids: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem {
        pub fn chunking_strategy < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: option :: Option < super :: CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy > > , T :: Error : :: std :: fmt :: Display ,{
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
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
    }
    impl
        ::std::convert::TryFrom<
            CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem,
        > for super::CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chunking_strategy: value.chunking_strategy?,
                file_ids: value.file_ids?,
                metadata: value.metadata?,
            })
        }
    }
    impl
        ::std::convert::From<
            super::CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem,
        > for CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        fn from(
            value: super::CreateAssistantRequestToolResourcesFileSearchVariant1VectorStoresItem,
        ) -> Self {
            Self {
                chunking_strategy: Ok(value.chunking_strategy),
                file_ids: Ok(value.file_ids),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl CreateMessageParams {
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageParams> for super::CreateMessageParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageParams> for CreateMessageParams {
        fn from(value: super::CreateMessageParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequest {
        attachments: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CreateMessageRequestAttachmentsItem>>,
            ::std::string::String,
        >,
        content: ::std::result::Result<super::CreateMessageRequestContent, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        role: ::std::result::Result<super::CreateMessageRequestRole, ::std::string::String>,
    }
    impl ::std::default::Default for CreateMessageRequest {
        fn default() -> Self {
            Self {
                attachments: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                metadata: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl CreateMessageRequest {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CreateMessageRequestAttachmentsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageRequestContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMessageRequestRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequest> for super::CreateMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                content: value.content?,
                metadata: value.metadata?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequest> for CreateMessageRequest {
        fn from(value: super::CreateMessageRequest) -> Self {
            Self {
                attachments: Ok(value.attachments),
                content: Ok(value.content),
                metadata: Ok(value.metadata),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMessageRequestAttachmentsItem {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::CreateMessageRequestAttachmentsItemToolsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMessageRequestAttachmentsItem {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl CreateMessageRequestAttachmentsItem {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateMessageRequestAttachmentsItemToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMessageRequestAttachmentsItem>
        for super::CreateMessageRequestAttachmentsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMessageRequestAttachmentsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMessageRequestAttachmentsItem>
        for CreateMessageRequestAttachmentsItem
    {
        fn from(value: super::CreateMessageRequestAttachmentsItem) -> Self {
            Self {
                file_id: Ok(value.file_id),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateRunParams {
        include: ::std::result::Result<
            ::std::vec::Vec<super::CreateRunParamsIncludeItem>,
            ::std::string::String,
        >,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateRunParams {
        fn default() -> Self {
            Self {
                include: Ok(Default::default()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl CreateRunParams {
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateRunParamsIncludeItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateRunParams> for super::CreateRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include: value.include?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateRunParams> for CreateRunParams {
        fn from(value: super::CreateRunParams) -> Self {
            Self {
                include: Ok(value.include),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateRunRequest {
        additional_instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        additional_messages: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CreateMessageRequest>>,
            ::std::string::String,
        >,
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_prompt_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::CreateRunRequestModel>,
            ::std::string::String,
        >,
        parallel_tool_calls: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        stream: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiToolChoiceOption>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CreateRunRequestToolsItem>>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        truncation_strategy: ::std::result::Result<
            ::std::option::Option<super::TruncationObject>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateRunRequest {
        fn default() -> Self {
            Self {
                additional_instructions: Ok(Default::default()),
                additional_messages: Ok(Default::default()),
                assistant_id: Err("no value supplied for assistant_id".to_string()),
                instructions: Ok(Default::default()),
                max_completion_tokens: Ok(Default::default()),
                max_prompt_tokens: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                parallel_tool_calls: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                response_format: Ok(Default::default()),
                stream: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
                truncation_strategy: Ok(Default::default()),
            }
        }
    }
    impl CreateRunRequest {
        pub fn additional_instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_instructions = value.try_into().map_err(|e| {
                format!("error converting supplied value for additional_instructions: {e}")
            });
            self
        }
        pub fn additional_messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CreateMessageRequest>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.additional_messages = value.try_into().map_err(|e| {
                format!("error converting supplied value for additional_messages: {e}")
            });
            self
        }
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn max_completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_completion_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_completion_tokens: {e}")
            });
            self
        }
        pub fn max_prompt_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_prompt_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_prompt_tokens: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateRunRequestModel>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn reasoning_effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_effort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_effort: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn stream<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantsApiToolChoiceOption>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_choice: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CreateRunRequestToolsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
        pub fn truncation_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TruncationObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.truncation_strategy = value.try_into().map_err(|e| {
                format!("error converting supplied value for truncation_strategy: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateRunRequest> for super::CreateRunRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateRunRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_instructions: value.additional_instructions?,
                additional_messages: value.additional_messages?,
                assistant_id: value.assistant_id?,
                instructions: value.instructions?,
                max_completion_tokens: value.max_completion_tokens?,
                max_prompt_tokens: value.max_prompt_tokens?,
                metadata: value.metadata?,
                model: value.model?,
                parallel_tool_calls: value.parallel_tool_calls?,
                reasoning_effort: value.reasoning_effort?,
                response_format: value.response_format?,
                stream: value.stream?,
                temperature: value.temperature?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
                top_p: value.top_p?,
                truncation_strategy: value.truncation_strategy?,
            })
        }
    }
    impl ::std::convert::From<super::CreateRunRequest> for CreateRunRequest {
        fn from(value: super::CreateRunRequest) -> Self {
            Self {
                additional_instructions: Ok(value.additional_instructions),
                additional_messages: Ok(value.additional_messages),
                assistant_id: Ok(value.assistant_id),
                instructions: Ok(value.instructions),
                max_completion_tokens: Ok(value.max_completion_tokens),
                max_prompt_tokens: Ok(value.max_prompt_tokens),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                reasoning_effort: Ok(value.reasoning_effort),
                response_format: Ok(value.response_format),
                stream: Ok(value.stream),
                temperature: Ok(value.temperature),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
                truncation_strategy: Ok(value.truncation_strategy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateRunRequestModel {
        subtype_0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::AssistantSupportedModels>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<::std::option::Option<()>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateRunRequestModel {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl CreateRunRequestModel {
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
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantSupportedModels>>,
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
    impl ::std::convert::TryFrom<CreateRunRequestModel> for super::CreateRunRequestModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateRunRequestModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::CreateRunRequestModel> for CreateRunRequestModel {
        fn from(value: super::CreateRunRequestModel) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadAndRunRequest {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_prompt_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::CreateThreadAndRunRequestModel>,
            ::std::string::String,
        >,
        parallel_tool_calls: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        stream: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        thread: ::std::result::Result<
            ::std::option::Option<super::CreateThreadRequest>,
            ::std::string::String,
        >,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiToolChoiceOption>,
            ::std::string::String,
        >,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::CreateThreadAndRunRequestToolResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CreateThreadAndRunRequestToolsItem>>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        truncation_strategy: ::std::result::Result<
            ::std::option::Option<super::TruncationObject>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateThreadAndRunRequest {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
                instructions: Ok(Default::default()),
                max_completion_tokens: Ok(Default::default()),
                max_prompt_tokens: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                parallel_tool_calls: Ok(Default::default()),
                response_format: Ok(Default::default()),
                stream: Ok(Default::default()),
                temperature: Ok(Default::default()),
                thread: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
                truncation_strategy: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadAndRunRequest {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn max_completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_completion_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_completion_tokens: {e}")
            });
            self
        }
        pub fn max_prompt_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_prompt_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_prompt_tokens: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadAndRunRequestModel>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ParallelToolCalls>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn stream<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn thread<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateThreadRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.thread = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantsApiToolChoiceOption>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_choice: {e}"));
            self
        }
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadAndRunRequestToolResources>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::CreateThreadAndRunRequestToolsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
        pub fn truncation_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TruncationObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.truncation_strategy = value.try_into().map_err(|e| {
                format!("error converting supplied value for truncation_strategy: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateThreadAndRunRequest> for super::CreateThreadAndRunRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadAndRunRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
                instructions: value.instructions?,
                max_completion_tokens: value.max_completion_tokens?,
                max_prompt_tokens: value.max_prompt_tokens?,
                metadata: value.metadata?,
                model: value.model?,
                parallel_tool_calls: value.parallel_tool_calls?,
                response_format: value.response_format?,
                stream: value.stream?,
                temperature: value.temperature?,
                thread: value.thread?,
                tool_choice: value.tool_choice?,
                tool_resources: value.tool_resources?,
                tools: value.tools?,
                top_p: value.top_p?,
                truncation_strategy: value.truncation_strategy?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadAndRunRequest> for CreateThreadAndRunRequest {
        fn from(value: super::CreateThreadAndRunRequest) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
                instructions: Ok(value.instructions),
                max_completion_tokens: Ok(value.max_completion_tokens),
                max_prompt_tokens: Ok(value.max_prompt_tokens),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                response_format: Ok(value.response_format),
                stream: Ok(value.stream),
                temperature: Ok(value.temperature),
                thread: Ok(value.thread),
                tool_choice: Ok(value.tool_choice),
                tool_resources: Ok(value.tool_resources),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
                truncation_strategy: Ok(value.truncation_strategy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadAndRunRequestModel {
        subtype_0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::CreateThreadAndRunRequestModelSubtype1>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<::std::option::Option<()>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateThreadAndRunRequestModel {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadAndRunRequestModel {
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
                ::std::option::Option<super::CreateThreadAndRunRequestModelSubtype1>,
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
    impl ::std::convert::TryFrom<CreateThreadAndRunRequestModel>
        for super::CreateThreadAndRunRequestModel
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadAndRunRequestModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadAndRunRequestModel>
        for CreateThreadAndRunRequestModel
    {
        fn from(value: super::CreateThreadAndRunRequestModel) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadAndRunRequestToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::CreateThreadAndRunRequestToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateThreadAndRunRequestToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadAndRunRequestToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadAndRunRequestToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateThreadAndRunRequestToolResources>
        for super::CreateThreadAndRunRequestToolResources
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadAndRunRequestToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadAndRunRequestToolResources>
        for CreateThreadAndRunRequestToolResources
    {
        fn from(value: super::CreateThreadAndRunRequestToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateThreadAndRunRequestToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadAndRunRequestToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<CreateThreadAndRunRequestToolResourcesCodeInterpreter>
        for super::CreateThreadAndRunRequestToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadAndRunRequestToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadAndRunRequestToolResourcesCodeInterpreter>
        for CreateThreadAndRunRequestToolResourcesCodeInterpreter
    {
        fn from(value: super::CreateThreadAndRunRequestToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateThreadAndRunRequestToolResourcesFileSearch {
        fn default() -> Self {
            Self {
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadAndRunRequestToolResourcesFileSearch {
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
    impl ::std::convert::TryFrom<CreateThreadAndRunRequestToolResourcesFileSearch>
        for super::CreateThreadAndRunRequestToolResourcesFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadAndRunRequestToolResourcesFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadAndRunRequestToolResourcesFileSearch>
        for CreateThreadAndRunRequestToolResourcesFileSearch
    {
        fn from(value: super::CreateThreadAndRunRequestToolResourcesFileSearch) -> Self {
            Self {
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadRequest {
        messages: ::std::result::Result<
            ::std::vec::Vec<super::CreateMessageRequest>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::CreateThreadRequestToolResources>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateThreadRequest {
        fn default() -> Self {
            Self {
                messages: Ok(Default::default()),
                metadata: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadRequest {
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateMessageRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
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
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadRequestToolResources>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateThreadRequest> for super::CreateThreadRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                messages: value.messages?,
                metadata: value.metadata?,
                tool_resources: value.tool_resources?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadRequest> for CreateThreadRequest {
        fn from(value: super::CreateThreadRequest) -> Self {
            Self {
                messages: Ok(value.messages),
                metadata: Ok(value.metadata),
                tool_resources: Ok(value.tool_resources),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadRequestToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::CreateThreadRequestToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::CreateThreadRequestToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateThreadRequestToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadRequestToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadRequestToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateThreadRequestToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateThreadRequestToolResources>
        for super::CreateThreadRequestToolResources
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadRequestToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadRequestToolResources>
        for CreateThreadRequestToolResources
    {
        fn from(value: super::CreateThreadRequestToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadRequestToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateThreadRequestToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadRequestToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<CreateThreadRequestToolResourcesCodeInterpreter>
        for super::CreateThreadRequestToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadRequestToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CreateThreadRequestToolResourcesCodeInterpreter>
        for CreateThreadRequestToolResourcesCodeInterpreter
    {
        fn from(value: super::CreateThreadRequestToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem { chunking_strategy : :: std :: result :: Result < :: std :: option :: Option < super :: CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy > , :: std :: string :: String > , file_ids : :: std :: result :: Result < :: std :: vec :: Vec < :: std :: string :: String > , :: std :: string :: String > , metadata : :: std :: result :: Result < :: std :: option :: Option < :: openai_api_types :: common :: Metadata > , :: std :: string :: String > , }
    impl ::std::default::Default
        for CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        fn default() -> Self {
            Self {
                chunking_strategy: Ok(Default::default()),
                file_ids: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem {
        pub fn chunking_strategy < T > (mut self , value : T) -> Self where T : :: std :: convert :: TryInto < :: std :: option :: Option < super :: CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItemChunkingStrategy > > , T :: Error : :: std :: fmt :: Display ,{
            self.chunking_strategy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chunking_strategy: {e}"));
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
    }
    impl ::std::convert::TryFrom<CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem>
        for super::CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chunking_strategy: value.chunking_strategy?,
                file_ids: value.file_ids?,
                metadata: value.metadata?,
            })
        }
    }
    impl
        ::std::convert::From<
            super::CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem,
        > for CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem
    {
        fn from(
            value: super::CreateThreadRequestToolResourcesFileSearchVariant1VectorStoresItem,
        ) -> Self {
            Self {
                chunking_strategy: Ok(value.chunking_strategy),
                file_ids: Ok(value.file_ids),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteAssistantParams {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteAssistantParams {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
            }
        }
    }
    impl DeleteAssistantParams {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteAssistantParams> for super::DeleteAssistantParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteAssistantParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteAssistantParams> for DeleteAssistantParams {
        fn from(value: super::DeleteAssistantParams) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteAssistantResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeleteAssistantResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteAssistantResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteAssistantResponse {
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
            T: ::std::convert::TryInto<super::DeleteAssistantResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteAssistantResponse> for super::DeleteAssistantResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteAssistantResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteAssistantResponse> for DeleteAssistantResponse {
        fn from(value: super::DeleteAssistantResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteMessageParams {
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteMessageParams {
        fn default() -> Self {
            Self {
                message_id: Err("no value supplied for message_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl DeleteMessageParams {
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteMessageParams> for super::DeleteMessageParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteMessageParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_id: value.message_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteMessageParams> for DeleteMessageParams {
        fn from(value: super::DeleteMessageParams) -> Self {
            Self {
                message_id: Ok(value.message_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteMessageResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeleteMessageResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteMessageResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteMessageResponse {
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
            T: ::std::convert::TryInto<super::DeleteMessageResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteMessageResponse> for super::DeleteMessageResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteMessageResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteMessageResponse> for DeleteMessageResponse {
        fn from(value: super::DeleteMessageResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteThreadParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteThreadParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl DeleteThreadParams {
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteThreadParams> for super::DeleteThreadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteThreadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteThreadParams> for DeleteThreadParams {
        fn from(value: super::DeleteThreadParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteThreadResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeleteThreadResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteThreadResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteThreadResponse {
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
            T: ::std::convert::TryInto<super::DeleteThreadResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteThreadResponse> for super::DeleteThreadResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteThreadResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteThreadResponse> for DeleteThreadResponse {
        fn from(value: super::DeleteThreadResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DoneEvent {
        data: ::std::result::Result<super::DoneEventData, ::std::string::String>,
        event: ::std::result::Result<super::DoneEventEvent, ::std::string::String>,
    }
    impl ::std::default::Default for DoneEvent {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                event: Err("no value supplied for event".to_string()),
            }
        }
    }
    impl DoneEvent {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DoneEventData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DoneEventEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DoneEvent> for super::DoneEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DoneEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                event: value.event?,
            })
        }
    }
    impl ::std::convert::From<super::DoneEvent> for DoneEvent {
        fn from(value: super::DoneEvent) -> Self {
            Self {
                data: Ok(value.data),
                event: Ok(value.event),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ErrorEvent {
        data: ::std::result::Result<::openai_api_types::common::Error, ::std::string::String>,
        event: ::std::result::Result<super::ErrorEventEvent, ::std::string::String>,
    }
    impl ::std::default::Default for ErrorEvent {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                event: Err("no value supplied for event".to_string()),
            }
        }
    }
    impl ErrorEvent {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::Error>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ErrorEventEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ErrorEvent> for super::ErrorEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ErrorEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                event: value.event?,
            })
        }
    }
    impl ::std::convert::From<super::ErrorEvent> for ErrorEvent {
        fn from(value: super::ErrorEvent) -> Self {
            Self {
                data: Ok(value.data),
                event: Ok(value.event),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileSearchRankingOptions {
        ranker: ::std::result::Result<
            ::std::option::Option<super::FileSearchRanker>,
            ::std::string::String,
        >,
        score_threshold: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for FileSearchRankingOptions {
        fn default() -> Self {
            Self {
                ranker: Ok(Default::default()),
                score_threshold: Err("no value supplied for score_threshold".to_string()),
            }
        }
    }
    impl FileSearchRankingOptions {
        pub fn ranker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileSearchRanker>>,
            T::Error: ::std::fmt::Display,
        {
            self.ranker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranker: {e}"));
            self
        }
        pub fn score_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FileSearchRankingOptions> for super::FileSearchRankingOptions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileSearchRankingOptions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ranker: value.ranker?,
                score_threshold: value.score_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::FileSearchRankingOptions> for FileSearchRankingOptions {
        fn from(value: super::FileSearchRankingOptions) -> Self {
            Self {
                ranker: Ok(value.ranker),
                score_threshold: Ok(value.score_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetAssistantParams {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetAssistantParams {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
            }
        }
    }
    impl GetAssistantParams {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetAssistantParams> for super::GetAssistantParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetAssistantParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetAssistantParams> for GetAssistantParams {
        fn from(value: super::GetAssistantParams) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetMessageParams {
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetMessageParams {
        fn default() -> Self {
            Self {
                message_id: Err("no value supplied for message_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl GetMessageParams {
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetMessageParams> for super::GetMessageParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetMessageParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_id: value.message_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetMessageParams> for GetMessageParams {
        fn from(value: super::GetMessageParams) -> Self {
            Self {
                message_id: Ok(value.message_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetRunParams {
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetRunParams {
        fn default() -> Self {
            Self {
                run_id: Err("no value supplied for run_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl GetRunParams {
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetRunParams> for super::GetRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetRunParams> for GetRunParams {
        fn from(value: super::GetRunParams) -> Self {
            Self {
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetRunStepParams {
        include: ::std::result::Result<
            ::std::vec::Vec<super::GetRunStepParamsIncludeItem>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        step_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetRunStepParams {
        fn default() -> Self {
            Self {
                include: Ok(Default::default()),
                run_id: Err("no value supplied for run_id".to_string()),
                step_id: Err("no value supplied for step_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl GetRunStepParams {
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GetRunStepParamsIncludeItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn step_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.step_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetRunStepParams> for super::GetRunStepParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetRunStepParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include: value.include?,
                run_id: value.run_id?,
                step_id: value.step_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetRunStepParams> for GetRunStepParams {
        fn from(value: super::GetRunStepParams) -> Self {
            Self {
                include: Ok(value.include),
                run_id: Ok(value.run_id),
                step_id: Ok(value.step_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetThreadParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetThreadParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl GetThreadParams {
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetThreadParams> for super::GetThreadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetThreadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetThreadParams> for GetThreadParams {
        fn from(value: super::GetThreadParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListAssistantsParams {
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
            ::std::option::Option<super::ListAssistantsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListAssistantsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListAssistantsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListAssistantsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListAssistantsParams> for super::ListAssistantsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListAssistantsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListAssistantsParams> for ListAssistantsParams {
        fn from(value: super::ListAssistantsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListAssistantsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::AssistantObject>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListAssistantsResponse {
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
    impl ListAssistantsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AssistantObject>>,
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
    impl ::std::convert::TryFrom<ListAssistantsResponse> for super::ListAssistantsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListAssistantsResponse,
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
    impl ::std::convert::From<super::ListAssistantsResponse> for ListAssistantsResponse {
        fn from(value: super::ListAssistantsResponse) -> Self {
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
    pub struct ListMessagesParams {
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
            ::std::option::Option<super::ListMessagesParamsOrder>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListMessagesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                run_id: Ok(Default::default()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ListMessagesParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListMessagesParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListMessagesParams> for super::ListMessagesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListMessagesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListMessagesParams> for ListMessagesParams {
        fn from(value: super::ListMessagesParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListMessagesResponse {
        data: ::std::result::Result<::std::vec::Vec<super::MessageObject>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListMessagesResponse {
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
    impl ListMessagesResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MessageObject>>,
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
    impl ::std::convert::TryFrom<ListMessagesResponse> for super::ListMessagesResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListMessagesResponse,
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
    impl ::std::convert::From<super::ListMessagesResponse> for ListMessagesResponse {
        fn from(value: super::ListMessagesResponse) -> Self {
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
    pub struct ListRunStepsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        include: ::std::result::Result<
            ::std::vec::Vec<super::ListRunStepsParamsIncludeItem>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListRunStepsParamsOrder>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListRunStepsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                include: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                run_id: Err("no value supplied for run_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ListRunStepsParams {
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
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ListRunStepsParamsIncludeItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListRunStepsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRunStepsParams> for super::ListRunStepsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRunStepsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                include: value.include?,
                limit: value.limit?,
                order: value.order?,
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListRunStepsParams> for ListRunStepsParams {
        fn from(value: super::ListRunStepsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                include: Ok(value.include),
                limit: Ok(value.limit),
                order: Ok(value.order),
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRunStepsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::RunStepObject>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListRunStepsResponse {
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
    impl ListRunStepsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RunStepObject>>,
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
    impl ::std::convert::TryFrom<ListRunStepsResponse> for super::ListRunStepsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRunStepsResponse,
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
    impl ::std::convert::From<super::ListRunStepsResponse> for ListRunStepsResponse {
        fn from(value: super::ListRunStepsResponse) -> Self {
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
    pub struct ListRunsParams {
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
            ::std::option::Option<super::ListRunsParamsOrder>,
            ::std::string::String,
        >,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListRunsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ListRunsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListRunsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListRunsParams> for super::ListRunsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRunsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListRunsParams> for ListRunsParams {
        fn from(value: super::ListRunsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListRunsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::RunObject>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListRunsResponse {
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
    impl ListRunsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RunObject>>,
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
    impl ::std::convert::TryFrom<ListRunsResponse> for super::ListRunsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListRunsResponse,
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
    impl ::std::convert::From<super::ListRunsResponse> for ListRunsResponse {
        fn from(value: super::ListRunsResponse) -> Self {
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
    pub struct MessageContentImageFileObject {
        image_file: ::std::result::Result<
            super::MessageContentImageFileObjectImageFile,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::MessageContentImageFileObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentImageFileObject {
        fn default() -> Self {
            Self {
                image_file: Err("no value supplied for image_file".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentImageFileObject {
        pub fn image_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentImageFileObjectImageFile>,
            T::Error: ::std::fmt::Display,
        {
            self.image_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_file: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentImageFileObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentImageFileObject>
        for super::MessageContentImageFileObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentImageFileObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_file: value.image_file?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentImageFileObject> for MessageContentImageFileObject {
        fn from(value: super::MessageContentImageFileObject) -> Self {
            Self {
                image_file: Ok(value.image_file),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentImageFileObjectImageFile {
        detail: ::std::result::Result<
            ::std::option::Option<super::MessageContentImageFileObjectImageFileDetail>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentImageFileObjectImageFile {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl MessageContentImageFileObjectImageFile {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageContentImageFileObjectImageFileDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
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
    impl ::std::convert::TryFrom<MessageContentImageFileObjectImageFile>
        for super::MessageContentImageFileObjectImageFile
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentImageFileObjectImageFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentImageFileObjectImageFile>
        for MessageContentImageFileObjectImageFile
    {
        fn from(value: super::MessageContentImageFileObjectImageFile) -> Self {
            Self {
                detail: Ok(value.detail),
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentImageUrlObject {
        image_url: ::std::result::Result<
            super::MessageContentImageUrlObjectImageUrl,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::MessageContentImageUrlObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentImageUrlObject {
        fn default() -> Self {
            Self {
                image_url: Err("no value supplied for image_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentImageUrlObject {
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentImageUrlObjectImageUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentImageUrlObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentImageUrlObject> for super::MessageContentImageUrlObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentImageUrlObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_url: value.image_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentImageUrlObject> for MessageContentImageUrlObject {
        fn from(value: super::MessageContentImageUrlObject) -> Self {
            Self {
                image_url: Ok(value.image_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentImageUrlObjectImageUrl {
        detail: ::std::result::Result<
            ::std::option::Option<super::MessageContentImageUrlObjectImageUrlDetail>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentImageUrlObjectImageUrl {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl MessageContentImageUrlObjectImageUrl {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageContentImageUrlObjectImageUrlDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentImageUrlObjectImageUrl>
        for super::MessageContentImageUrlObjectImageUrl
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentImageUrlObjectImageUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentImageUrlObjectImageUrl>
        for MessageContentImageUrlObjectImageUrl
    {
        fn from(value: super::MessageContentImageUrlObjectImageUrl) -> Self {
            Self {
                detail: Ok(value.detail),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentRefusalObject {
        refusal: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::MessageContentRefusalObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentRefusalObject {
        fn default() -> Self {
            Self {
                refusal: Err("no value supplied for refusal".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentRefusalObject {
        pub fn refusal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.refusal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refusal: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentRefusalObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentRefusalObject> for super::MessageContentRefusalObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentRefusalObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                refusal: value.refusal?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentRefusalObject> for MessageContentRefusalObject {
        fn from(value: super::MessageContentRefusalObject) -> Self {
            Self {
                refusal: Ok(value.refusal),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextAnnotationsFileCitationObject {
        end_index: ::std::result::Result<u64, ::std::string::String>,
        file_citation: ::std::result::Result<
            super::MessageContentTextAnnotationsFileCitationObjectFileCitation,
            ::std::string::String,
        >,
        start_index: ::std::result::Result<u64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::MessageContentTextAnnotationsFileCitationObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageContentTextAnnotationsFileCitationObject {
        fn default() -> Self {
            Self {
                end_index: Err("no value supplied for end_index".to_string()),
                file_citation: Err("no value supplied for file_citation".to_string()),
                start_index: Err("no value supplied for start_index".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentTextAnnotationsFileCitationObject {
        pub fn end_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_index: {e}"));
            self
        }
        pub fn file_citation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::MessageContentTextAnnotationsFileCitationObjectFileCitation,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_citation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_citation: {e}"));
            self
        }
        pub fn start_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_index: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<super::MessageContentTextAnnotationsFileCitationObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentTextAnnotationsFileCitationObject>
        for super::MessageContentTextAnnotationsFileCitationObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextAnnotationsFileCitationObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_index: value.end_index?,
                file_citation: value.file_citation?,
                start_index: value.start_index?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextAnnotationsFileCitationObject>
        for MessageContentTextAnnotationsFileCitationObject
    {
        fn from(value: super::MessageContentTextAnnotationsFileCitationObject) -> Self {
            Self {
                end_index: Ok(value.end_index),
                file_citation: Ok(value.file_citation),
                start_index: Ok(value.start_index),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentTextAnnotationsFileCitationObjectFileCitation {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl MessageContentTextAnnotationsFileCitationObjectFileCitation {
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
    impl ::std::convert::TryFrom<MessageContentTextAnnotationsFileCitationObjectFileCitation>
        for super::MessageContentTextAnnotationsFileCitationObjectFileCitation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextAnnotationsFileCitationObjectFileCitation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextAnnotationsFileCitationObjectFileCitation>
        for MessageContentTextAnnotationsFileCitationObjectFileCitation
    {
        fn from(value: super::MessageContentTextAnnotationsFileCitationObjectFileCitation) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextAnnotationsFilePathObject {
        end_index: ::std::result::Result<u64, ::std::string::String>,
        file_path: ::std::result::Result<
            super::MessageContentTextAnnotationsFilePathObjectFilePath,
            ::std::string::String,
        >,
        start_index: ::std::result::Result<u64, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::MessageContentTextAnnotationsFilePathObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageContentTextAnnotationsFilePathObject {
        fn default() -> Self {
            Self {
                end_index: Err("no value supplied for end_index".to_string()),
                file_path: Err("no value supplied for file_path".to_string()),
                start_index: Err("no value supplied for start_index".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentTextAnnotationsFilePathObject {
        pub fn end_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_index: {e}"));
            self
        }
        pub fn file_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentTextAnnotationsFilePathObjectFilePath>,
            T::Error: ::std::fmt::Display,
        {
            self.file_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_path: {e}"));
            self
        }
        pub fn start_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_index: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<super::MessageContentTextAnnotationsFilePathObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentTextAnnotationsFilePathObject>
        for super::MessageContentTextAnnotationsFilePathObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextAnnotationsFilePathObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_index: value.end_index?,
                file_path: value.file_path?,
                start_index: value.start_index?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextAnnotationsFilePathObject>
        for MessageContentTextAnnotationsFilePathObject
    {
        fn from(value: super::MessageContentTextAnnotationsFilePathObject) -> Self {
            Self {
                end_index: Ok(value.end_index),
                file_path: Ok(value.file_path),
                start_index: Ok(value.start_index),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentTextAnnotationsFilePathObjectFilePath {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl MessageContentTextAnnotationsFilePathObjectFilePath {
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
    impl ::std::convert::TryFrom<MessageContentTextAnnotationsFilePathObjectFilePath>
        for super::MessageContentTextAnnotationsFilePathObjectFilePath
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextAnnotationsFilePathObjectFilePath,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextAnnotationsFilePathObjectFilePath>
        for MessageContentTextAnnotationsFilePathObjectFilePath
    {
        fn from(value: super::MessageContentTextAnnotationsFilePathObjectFilePath) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextObject {
        text: ::std::result::Result<super::MessageContentTextObjectText, ::std::string::String>,
        type_: ::std::result::Result<super::MessageContentTextObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentTextObject {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageContentTextObject {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentTextObjectText>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageContentTextObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentTextObject> for super::MessageContentTextObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextObject> for MessageContentTextObject {
        fn from(value: super::MessageContentTextObject) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageContentTextObjectText {
        annotations: ::std::result::Result<
            ::std::vec::Vec<super::MessageContentTextObjectTextAnnotationsItem>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageContentTextObjectText {
        fn default() -> Self {
            Self {
                annotations: Err("no value supplied for annotations".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl MessageContentTextObjectText {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MessageContentTextObjectTextAnnotationsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageContentTextObjectText> for super::MessageContentTextObjectText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageContentTextObjectText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MessageContentTextObjectText> for MessageContentTextObjectText {
        fn from(value: super::MessageContentTextObjectText) -> Self {
            Self {
                annotations: Ok(value.annotations),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentImageFileObject {
        image_file: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentImageFileObjectImageFile>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::MessageDeltaContentImageFileObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentImageFileObject {
        fn default() -> Self {
            Self {
                image_file: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentImageFileObject {
        pub fn image_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageDeltaContentImageFileObjectImageFile>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_file: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageDeltaContentImageFileObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentImageFileObject>
        for super::MessageDeltaContentImageFileObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentImageFileObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_file: value.image_file?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentImageFileObject>
        for MessageDeltaContentImageFileObject
    {
        fn from(value: super::MessageDeltaContentImageFileObject) -> Self {
            Self {
                image_file: Ok(value.image_file),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentImageFileObjectImageFile {
        detail: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentImageFileObjectImageFileDetail>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentImageFileObjectImageFile {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                file_id: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaContentImageFileObjectImageFile {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageDeltaContentImageFileObjectImageFileDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentImageFileObjectImageFile>
        for super::MessageDeltaContentImageFileObjectImageFile
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentImageFileObjectImageFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentImageFileObjectImageFile>
        for MessageDeltaContentImageFileObjectImageFile
    {
        fn from(value: super::MessageDeltaContentImageFileObjectImageFile) -> Self {
            Self {
                detail: Ok(value.detail),
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentImageUrlObject {
        image_url: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentImageUrlObjectImageUrl>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::MessageDeltaContentImageUrlObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentImageUrlObject {
        fn default() -> Self {
            Self {
                image_url: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentImageUrlObject {
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageDeltaContentImageUrlObjectImageUrl>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageDeltaContentImageUrlObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentImageUrlObject>
        for super::MessageDeltaContentImageUrlObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentImageUrlObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_url: value.image_url?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentImageUrlObject>
        for MessageDeltaContentImageUrlObject
    {
        fn from(value: super::MessageDeltaContentImageUrlObject) -> Self {
            Self {
                image_url: Ok(value.image_url),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentImageUrlObjectImageUrl {
        detail: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentImageUrlObjectImageUrlDetail>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentImageUrlObjectImageUrl {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaContentImageUrlObjectImageUrl {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageDeltaContentImageUrlObjectImageUrlDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentImageUrlObjectImageUrl>
        for super::MessageDeltaContentImageUrlObjectImageUrl
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentImageUrlObjectImageUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentImageUrlObjectImageUrl>
        for MessageDeltaContentImageUrlObjectImageUrl
    {
        fn from(value: super::MessageDeltaContentImageUrlObjectImageUrl) -> Self {
            Self {
                detail: Ok(value.detail),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentRefusalObject {
        index: ::std::result::Result<i64, ::std::string::String>,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::MessageDeltaContentRefusalObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentRefusalObject {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                refusal: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentRefusalObject {
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
        pub fn refusal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refusal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refusal: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageDeltaContentRefusalObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentRefusalObject>
        for super::MessageDeltaContentRefusalObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentRefusalObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                refusal: value.refusal?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentRefusalObject>
        for MessageDeltaContentRefusalObject
    {
        fn from(value: super::MessageDeltaContentRefusalObject) -> Self {
            Self {
                index: Ok(value.index),
                refusal: Ok(value.refusal),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
        end_index: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        file_citation: ::std::result::Result<
            ::std::option::Option<
                super::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
            >,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        start_index: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::MessageDeltaContentTextAnnotationsFileCitationObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentTextAnnotationsFileCitationObject {
        fn default() -> Self {
            Self {
                end_index: Ok(Default::default()),
                file_citation: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                start_index: Ok(Default::default()),
                text: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentTextAnnotationsFileCitationObject {
        pub fn end_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_index: {e}"));
            self
        }
        pub fn file_citation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_citation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_citation: {e}"));
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
        pub fn start_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_index: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<
                super::MessageDeltaContentTextAnnotationsFileCitationObjectType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextAnnotationsFileCitationObject>
        for super::MessageDeltaContentTextAnnotationsFileCitationObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextAnnotationsFileCitationObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_index: value.end_index?,
                file_citation: value.file_citation?,
                index: value.index?,
                start_index: value.start_index?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentTextAnnotationsFileCitationObject>
        for MessageDeltaContentTextAnnotationsFileCitationObject
    {
        fn from(value: super::MessageDeltaContentTextAnnotationsFileCitationObject) -> Self {
            Self {
                end_index: Ok(value.end_index),
                file_citation: Ok(value.file_citation),
                index: Ok(value.index),
                start_index: Ok(value.start_index),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        quote: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
                quote: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn quote<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.quote = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quote: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>
        for super::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                quote: value.quote?,
            })
        }
    }
    impl
        ::std::convert::From<
            super::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
        > for MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation
    {
        fn from(
            value: super::MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
        ) -> Self {
            Self {
                file_id: Ok(value.file_id),
                quote: Ok(value.quote),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextAnnotationsFilePathObject {
        end_index: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        file_path: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentTextAnnotationsFilePathObjectFilePath>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        start_index: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::MessageDeltaContentTextAnnotationsFilePathObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentTextAnnotationsFilePathObject {
        fn default() -> Self {
            Self {
                end_index: Ok(Default::default()),
                file_path: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                start_index: Ok(Default::default()),
                text: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentTextAnnotationsFilePathObject {
        pub fn end_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_index: {e}"));
            self
        }
        pub fn file_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::MessageDeltaContentTextAnnotationsFilePathObjectFilePath,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_path: {e}"));
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
        pub fn start_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_index: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<super::MessageDeltaContentTextAnnotationsFilePathObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextAnnotationsFilePathObject>
        for super::MessageDeltaContentTextAnnotationsFilePathObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextAnnotationsFilePathObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_index: value.end_index?,
                file_path: value.file_path?,
                index: value.index?,
                start_index: value.start_index?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentTextAnnotationsFilePathObject>
        for MessageDeltaContentTextAnnotationsFilePathObject
    {
        fn from(value: super::MessageDeltaContentTextAnnotationsFilePathObject) -> Self {
            Self {
                end_index: Ok(value.end_index),
                file_path: Ok(value.file_path),
                index: Ok(value.index),
                start_index: Ok(value.start_index),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextAnnotationsFilePathObjectFilePath>
        for super::MessageDeltaContentTextAnnotationsFilePathObjectFilePath
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextAnnotationsFilePathObjectFilePath,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentTextAnnotationsFilePathObjectFilePath>
        for MessageDeltaContentTextAnnotationsFilePathObjectFilePath
    {
        fn from(value: super::MessageDeltaContentTextAnnotationsFilePathObjectFilePath) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextObject {
        index: ::std::result::Result<i64, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaContentTextObjectText>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::MessageDeltaContentTextObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for MessageDeltaContentTextObject {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                text: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageDeltaContentTextObject {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageDeltaContentTextObjectText>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageDeltaContentTextObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextObject>
        for super::MessageDeltaContentTextObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentTextObject> for MessageDeltaContentTextObject {
        fn from(value: super::MessageDeltaContentTextObject) -> Self {
            Self {
                index: Ok(value.index),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaContentTextObjectText {
        annotations: ::std::result::Result<
            ::std::vec::Vec<super::MessageDeltaContentTextObjectTextAnnotationsItem>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaContentTextObjectText {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaContentTextObjectText {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MessageDeltaContentTextObjectTextAnnotationsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaContentTextObjectText>
        for super::MessageDeltaContentTextObjectText
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaContentTextObjectText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaContentTextObjectText>
        for MessageDeltaContentTextObjectText
    {
        fn from(value: super::MessageDeltaContentTextObjectText) -> Self {
            Self {
                annotations: Ok(value.annotations),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaObject {
        delta: ::std::result::Result<super::MessageDeltaObjectDelta, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::MessageDeltaObjectObject, ::std::string::String>,
    }
    impl ::std::default::Default for MessageDeltaObject {
        fn default() -> Self {
            Self {
                delta: Err("no value supplied for delta".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl MessageDeltaObject {
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageDeltaObjectDelta>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
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
            T: ::std::convert::TryInto<super::MessageDeltaObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaObject> for super::MessageDeltaObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delta: value.delta?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaObject> for MessageDeltaObject {
        fn from(value: super::MessageDeltaObject) -> Self {
            Self {
                delta: Ok(value.delta),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageDeltaObjectDelta {
        content: ::std::result::Result<
            ::std::vec::Vec<super::MessageDeltaObjectDeltaContentItem>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<super::MessageDeltaObjectDeltaRole>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageDeltaObjectDelta {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                role: Ok(Default::default()),
            }
        }
    }
    impl MessageDeltaObjectDelta {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MessageDeltaObjectDeltaContentItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MessageDeltaObjectDeltaRole>>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageDeltaObjectDelta> for super::MessageDeltaObjectDelta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageDeltaObjectDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::MessageDeltaObjectDelta> for MessageDeltaObjectDelta {
        fn from(value: super::MessageDeltaObjectDelta) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageObject {
        assistant_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        attachments: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::MessageObjectAttachmentsItem>>,
            ::std::string::String,
        >,
        completed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        content: ::std::result::Result<
            ::std::vec::Vec<super::MessageObjectContentItem>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        incomplete_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        incomplete_details: ::std::result::Result<
            ::std::option::Option<super::MessageObjectIncompleteDetails>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        object: ::std::result::Result<super::MessageObjectObject, ::std::string::String>,
        role: ::std::result::Result<super::MessageObjectRole, ::std::string::String>,
        run_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::MessageObjectStatus, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MessageObject {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
                attachments: Err("no value supplied for attachments".to_string()),
                completed_at: Err("no value supplied for completed_at".to_string()),
                content: Err("no value supplied for content".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                incomplete_at: Err("no value supplied for incomplete_at".to_string()),
                incomplete_details: Err("no value supplied for incomplete_details".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                object: Err("no value supplied for object".to_string()),
                role: Err("no value supplied for role".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
                status: Err("no value supplied for status".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl MessageObject {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::MessageObjectAttachmentsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
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
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MessageObjectContentItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn incomplete_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.incomplete_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incomplete_at: {e}"));
            self
        }
        pub fn incomplete_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::MessageObjectIncompleteDetails>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.incomplete_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for incomplete_details: {e}")
            });
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObjectRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageObject> for super::MessageObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
                attachments: value.attachments?,
                completed_at: value.completed_at?,
                content: value.content?,
                created_at: value.created_at?,
                id: value.id?,
                incomplete_at: value.incomplete_at?,
                incomplete_details: value.incomplete_details?,
                metadata: value.metadata?,
                object: value.object?,
                role: value.role?,
                run_id: value.run_id?,
                status: value.status?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::MessageObject> for MessageObject {
        fn from(value: super::MessageObject) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
                attachments: Ok(value.attachments),
                completed_at: Ok(value.completed_at),
                content: Ok(value.content),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                incomplete_at: Ok(value.incomplete_at),
                incomplete_details: Ok(value.incomplete_details),
                metadata: Ok(value.metadata),
                object: Ok(value.object),
                role: Ok(value.role),
                run_id: Ok(value.run_id),
                status: Ok(value.status),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageObjectAttachmentsItem {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::MessageObjectAttachmentsItemToolsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageObjectAttachmentsItem {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl MessageObjectAttachmentsItem {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::MessageObjectAttachmentsItemToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageObjectAttachmentsItem> for super::MessageObjectAttachmentsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageObjectAttachmentsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::MessageObjectAttachmentsItem> for MessageObjectAttachmentsItem {
        fn from(value: super::MessageObjectAttachmentsItem) -> Self {
            Self {
                file_id: Ok(value.file_id),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageObjectIncompleteDetails {
        reason: ::std::result::Result<
            super::MessageObjectIncompleteDetailsReason,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageObjectIncompleteDetails {
        fn default() -> Self {
            Self {
                reason: Err("no value supplied for reason".to_string()),
            }
        }
    }
    impl MessageObjectIncompleteDetails {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageObjectIncompleteDetailsReason>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageObjectIncompleteDetails>
        for super::MessageObjectIncompleteDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageObjectIncompleteDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::MessageObjectIncompleteDetails>
        for MessageObjectIncompleteDetails
    {
        fn from(value: super::MessageObjectIncompleteDetails) -> Self {
            Self {
                reason: Ok(value.reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageRequestContentTextObject {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::MessageRequestContentTextObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageRequestContentTextObject {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl MessageRequestContentTextObject {
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
            T: ::std::convert::TryInto<super::MessageRequestContentTextObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageRequestContentTextObject>
        for super::MessageRequestContentTextObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageRequestContentTextObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MessageRequestContentTextObject>
        for MessageRequestContentTextObject
    {
        fn from(value: super::MessageRequestContentTextObject) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantParams {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyAssistantParams {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
            }
        }
    }
    impl ModifyAssistantParams {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyAssistantParams> for super::ModifyAssistantParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantParams> for ModifyAssistantParams {
        fn from(value: super::ModifyAssistantParams) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantRequest {
        description: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestDescription>,
            ::std::string::String,
        >,
        instructions: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestInstructions>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestModel>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestName>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestToolResources>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::ModifyAssistantRequestToolsItem>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyAssistantRequest {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                instructions: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                name: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                response_format: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl ModifyAssistantRequest {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyAssistantRequestDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyAssistantRequestInstructions>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModifyAssistantRequestModel>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModifyAssistantRequestName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn reasoning_effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_effort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_effort: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyAssistantRequestToolResources>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ModifyAssistantRequestToolsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyAssistantRequest> for super::ModifyAssistantRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                instructions: value.instructions?,
                metadata: value.metadata?,
                model: value.model?,
                name: value.name?,
                reasoning_effort: value.reasoning_effort?,
                response_format: value.response_format?,
                temperature: value.temperature?,
                tool_resources: value.tool_resources?,
                tools: value.tools?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantRequest> for ModifyAssistantRequest {
        fn from(value: super::ModifyAssistantRequest) -> Self {
            Self {
                description: Ok(value.description),
                instructions: Ok(value.instructions),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                name: Ok(value.name),
                reasoning_effort: Ok(value.reasoning_effort),
                response_format: Ok(value.response_format),
                temperature: Ok(value.temperature),
                tool_resources: Ok(value.tool_resources),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantRequestModel {
        subtype_0: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::AssistantSupportedModels>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyAssistantRequestModel {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ModifyAssistantRequestModel {
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
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantSupportedModels>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyAssistantRequestModel> for super::ModifyAssistantRequestModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantRequestModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantRequestModel> for ModifyAssistantRequestModel {
        fn from(value: super::ModifyAssistantRequestModel) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantRequestToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::ModifyAssistantRequestToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyAssistantRequestToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl ModifyAssistantRequestToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyAssistantRequestToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyAssistantRequestToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyAssistantRequestToolResources>
        for super::ModifyAssistantRequestToolResources
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantRequestToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantRequestToolResources>
        for ModifyAssistantRequestToolResources
    {
        fn from(value: super::ModifyAssistantRequestToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyAssistantRequestToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl ModifyAssistantRequestToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<ModifyAssistantRequestToolResourcesCodeInterpreter>
        for super::ModifyAssistantRequestToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantRequestToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantRequestToolResourcesCodeInterpreter>
        for ModifyAssistantRequestToolResourcesCodeInterpreter
    {
        fn from(value: super::ModifyAssistantRequestToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyAssistantRequestToolResourcesFileSearch {
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyAssistantRequestToolResourcesFileSearch {
        fn default() -> Self {
            Self {
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl ModifyAssistantRequestToolResourcesFileSearch {
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
    impl ::std::convert::TryFrom<ModifyAssistantRequestToolResourcesFileSearch>
        for super::ModifyAssistantRequestToolResourcesFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyAssistantRequestToolResourcesFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyAssistantRequestToolResourcesFileSearch>
        for ModifyAssistantRequestToolResourcesFileSearch
    {
        fn from(value: super::ModifyAssistantRequestToolResourcesFileSearch) -> Self {
            Self {
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyMessageParams {
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyMessageParams {
        fn default() -> Self {
            Self {
                message_id: Err("no value supplied for message_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ModifyMessageParams {
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyMessageParams> for super::ModifyMessageParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyMessageParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_id: value.message_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyMessageParams> for ModifyMessageParams {
        fn from(value: super::ModifyMessageParams) -> Self {
            Self {
                message_id: Ok(value.message_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyMessageRequest {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyMessageRequest {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
            }
        }
    }
    impl ModifyMessageRequest {
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
    }
    impl ::std::convert::TryFrom<ModifyMessageRequest> for super::ModifyMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyMessageRequest> for ModifyMessageRequest {
        fn from(value: super::ModifyMessageRequest) -> Self {
            Self {
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyRunParams {
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyRunParams {
        fn default() -> Self {
            Self {
                run_id: Err("no value supplied for run_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ModifyRunParams {
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyRunParams> for super::ModifyRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyRunParams> for ModifyRunParams {
        fn from(value: super::ModifyRunParams) -> Self {
            Self {
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyRunRequest {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyRunRequest {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
            }
        }
    }
    impl ModifyRunRequest {
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
    }
    impl ::std::convert::TryFrom<ModifyRunRequest> for super::ModifyRunRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyRunRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyRunRequest> for ModifyRunRequest {
        fn from(value: super::ModifyRunRequest) -> Self {
            Self {
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyThreadParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyThreadParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ModifyThreadParams {
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyThreadParams> for super::ModifyThreadParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyThreadParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyThreadParams> for ModifyThreadParams {
        fn from(value: super::ModifyThreadParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyThreadRequest {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::ModifyThreadRequestToolResources>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyThreadRequest {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                tool_resources: Ok(Default::default()),
            }
        }
    }
    impl ModifyThreadRequest {
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
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyThreadRequestToolResources>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyThreadRequest> for super::ModifyThreadRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyThreadRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                tool_resources: value.tool_resources?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyThreadRequest> for ModifyThreadRequest {
        fn from(value: super::ModifyThreadRequest) -> Self {
            Self {
                metadata: Ok(value.metadata),
                tool_resources: Ok(value.tool_resources),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyThreadRequestToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::ModifyThreadRequestToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::ModifyThreadRequestToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModifyThreadRequestToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl ModifyThreadRequestToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyThreadRequestToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModifyThreadRequestToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyThreadRequestToolResources>
        for super::ModifyThreadRequestToolResources
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyThreadRequestToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyThreadRequestToolResources>
        for ModifyThreadRequestToolResources
    {
        fn from(value: super::ModifyThreadRequestToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyThreadRequestToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl ModifyThreadRequestToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<ModifyThreadRequestToolResourcesCodeInterpreter>
        for super::ModifyThreadRequestToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyThreadRequestToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyThreadRequestToolResourcesCodeInterpreter>
        for ModifyThreadRequestToolResourcesCodeInterpreter
    {
        fn from(value: super::ModifyThreadRequestToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyThreadRequestToolResourcesFileSearch {
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyThreadRequestToolResourcesFileSearch {
        fn default() -> Self {
            Self {
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl ModifyThreadRequestToolResourcesFileSearch {
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
    impl ::std::convert::TryFrom<ModifyThreadRequestToolResourcesFileSearch>
        for super::ModifyThreadRequestToolResourcesFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyThreadRequestToolResourcesFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyThreadRequestToolResourcesFileSearch>
        for ModifyThreadRequestToolResourcesFileSearch
    {
        fn from(value: super::ModifyThreadRequestToolResourcesFileSearch) -> Self {
            Self {
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunCompletionUsageValue {
        completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for RunCompletionUsageValue {
        fn default() -> Self {
            Self {
                completion_tokens: Err("no value supplied for completion_tokens".to_string()),
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl RunCompletionUsageValue {
        pub fn completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.completion_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion_tokens: {e}"));
            self
        }
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
    impl ::std::convert::TryFrom<RunCompletionUsageValue> for super::RunCompletionUsageValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunCompletionUsageValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion_tokens: value.completion_tokens?,
                prompt_tokens: value.prompt_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::RunCompletionUsageValue> for RunCompletionUsageValue {
        fn from(value: super::RunCompletionUsageValue) -> Self {
            Self {
                completion_tokens: Ok(value.completion_tokens),
                prompt_tokens: Ok(value.prompt_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunObject {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        cancelled_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        completed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        failed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        incomplete_details: ::std::result::Result<
            ::std::option::Option<super::RunObjectIncompleteDetails>,
            ::std::string::String,
        >,
        instructions: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_error: ::std::result::Result<
            ::std::option::Option<super::RunObjectLastError>,
            ::std::string::String,
        >,
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_prompt_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::RunObjectObject, ::std::string::String>,
        parallel_tool_calls: ::std::result::Result<
            ::openai_api_types::common::ParallelToolCalls,
            ::std::string::String,
        >,
        required_action: ::std::result::Result<
            ::std::option::Option<super::RunObjectRequiredAction>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            ::std::string::String,
        >,
        started_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        status: ::std::result::Result<super::RunObjectStatus, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::AssistantsApiToolChoiceOption>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::RunObjectToolsItem>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        truncation_strategy: ::std::result::Result<
            ::std::option::Option<super::TruncationObject>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<super::RunCompletionUsage, ::std::string::String>,
    }
    impl ::std::default::Default for RunObject {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
                cancelled_at: Err("no value supplied for cancelled_at".to_string()),
                completed_at: Err("no value supplied for completed_at".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                failed_at: Err("no value supplied for failed_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                incomplete_details: Err("no value supplied for incomplete_details".to_string()),
                instructions: Err("no value supplied for instructions".to_string()),
                last_error: Err("no value supplied for last_error".to_string()),
                max_completion_tokens: Err(
                    "no value supplied for max_completion_tokens".to_string()
                ),
                max_prompt_tokens: Err("no value supplied for max_prompt_tokens".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
                parallel_tool_calls: Err("no value supplied for parallel_tool_calls".to_string()),
                required_action: Err("no value supplied for required_action".to_string()),
                response_format: Err("no value supplied for response_format".to_string()),
                started_at: Err("no value supplied for started_at".to_string()),
                status: Err("no value supplied for status".to_string()),
                temperature: Ok(Default::default()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                tool_choice: Err("no value supplied for tool_choice".to_string()),
                tools: Err("no value supplied for tools".to_string()),
                top_p: Ok(Default::default()),
                truncation_strategy: Err("no value supplied for truncation_strategy".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl RunObject {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
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
        pub fn incomplete_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RunObjectIncompleteDetails>>,
            T::Error: ::std::fmt::Display,
        {
            self.incomplete_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for incomplete_details: {e}")
            });
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn last_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RunObjectLastError>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_error: {e}"));
            self
        }
        pub fn max_completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_completion_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_completion_tokens: {e}")
            });
            self
        }
        pub fn max_prompt_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_prompt_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_prompt_tokens: {e}"));
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
            T: ::std::convert::TryInto<super::RunObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ParallelToolCalls>,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn required_action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RunObjectRequiredAction>>,
            T::Error: ::std::fmt::Display,
        {
            self.required_action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required_action: {e}"));
            self
        }
        pub fn response_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AssistantsApiResponseFormatOption>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn started_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.started_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started_at: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AssistantsApiToolChoiceOption>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_choice: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RunObjectToolsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
        pub fn truncation_strategy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TruncationObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.truncation_strategy = value.try_into().map_err(|e| {
                format!("error converting supplied value for truncation_strategy: {e}")
            });
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunCompletionUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunObject> for super::RunObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
                cancelled_at: value.cancelled_at?,
                completed_at: value.completed_at?,
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                failed_at: value.failed_at?,
                id: value.id?,
                incomplete_details: value.incomplete_details?,
                instructions: value.instructions?,
                last_error: value.last_error?,
                max_completion_tokens: value.max_completion_tokens?,
                max_prompt_tokens: value.max_prompt_tokens?,
                metadata: value.metadata?,
                model: value.model?,
                object: value.object?,
                parallel_tool_calls: value.parallel_tool_calls?,
                required_action: value.required_action?,
                response_format: value.response_format?,
                started_at: value.started_at?,
                status: value.status?,
                temperature: value.temperature?,
                thread_id: value.thread_id?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
                top_p: value.top_p?,
                truncation_strategy: value.truncation_strategy?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::RunObject> for RunObject {
        fn from(value: super::RunObject) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
                cancelled_at: Ok(value.cancelled_at),
                completed_at: Ok(value.completed_at),
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                failed_at: Ok(value.failed_at),
                id: Ok(value.id),
                incomplete_details: Ok(value.incomplete_details),
                instructions: Ok(value.instructions),
                last_error: Ok(value.last_error),
                max_completion_tokens: Ok(value.max_completion_tokens),
                max_prompt_tokens: Ok(value.max_prompt_tokens),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                object: Ok(value.object),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                required_action: Ok(value.required_action),
                response_format: Ok(value.response_format),
                started_at: Ok(value.started_at),
                status: Ok(value.status),
                temperature: Ok(value.temperature),
                thread_id: Ok(value.thread_id),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
                truncation_strategy: Ok(value.truncation_strategy),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunObjectIncompleteDetails {
        reason: ::std::result::Result<
            ::std::option::Option<super::RunObjectIncompleteDetailsReason>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunObjectIncompleteDetails {
        fn default() -> Self {
            Self {
                reason: Ok(Default::default()),
            }
        }
    }
    impl RunObjectIncompleteDetails {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RunObjectIncompleteDetailsReason>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunObjectIncompleteDetails> for super::RunObjectIncompleteDetails {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunObjectIncompleteDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::RunObjectIncompleteDetails> for RunObjectIncompleteDetails {
        fn from(value: super::RunObjectIncompleteDetails) -> Self {
            Self {
                reason: Ok(value.reason),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunObjectLastError {
        code: ::std::result::Result<super::RunObjectLastErrorCode, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RunObjectLastError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl RunObjectLastError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObjectLastErrorCode>,
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
    impl ::std::convert::TryFrom<RunObjectLastError> for super::RunObjectLastError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunObjectLastError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::RunObjectLastError> for RunObjectLastError {
        fn from(value: super::RunObjectLastError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunObjectRequiredAction {
        submit_tool_outputs: ::std::result::Result<
            super::RunObjectRequiredActionSubmitToolOutputs,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::RunObjectRequiredActionType, ::std::string::String>,
    }
    impl ::std::default::Default for RunObjectRequiredAction {
        fn default() -> Self {
            Self {
                submit_tool_outputs: Err("no value supplied for submit_tool_outputs".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunObjectRequiredAction {
        pub fn submit_tool_outputs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObjectRequiredActionSubmitToolOutputs>,
            T::Error: ::std::fmt::Display,
        {
            self.submit_tool_outputs = value.try_into().map_err(|e| {
                format!("error converting supplied value for submit_tool_outputs: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunObjectRequiredActionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunObjectRequiredAction> for super::RunObjectRequiredAction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunObjectRequiredAction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                submit_tool_outputs: value.submit_tool_outputs?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunObjectRequiredAction> for RunObjectRequiredAction {
        fn from(value: super::RunObjectRequiredAction) -> Self {
            Self {
                submit_tool_outputs: Ok(value.submit_tool_outputs),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunObjectRequiredActionSubmitToolOutputs {
        tool_calls:
            ::std::result::Result<::std::vec::Vec<super::RunToolCallObject>, ::std::string::String>,
    }
    impl ::std::default::Default for RunObjectRequiredActionSubmitToolOutputs {
        fn default() -> Self {
            Self {
                tool_calls: Err("no value supplied for tool_calls".to_string()),
            }
        }
    }
    impl RunObjectRequiredActionSubmitToolOutputs {
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RunToolCallObject>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunObjectRequiredActionSubmitToolOutputs>
        for super::RunObjectRequiredActionSubmitToolOutputs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunObjectRequiredActionSubmitToolOutputs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tool_calls: value.tool_calls?,
            })
        }
    }
    impl ::std::convert::From<super::RunObjectRequiredActionSubmitToolOutputs>
        for RunObjectRequiredActionSubmitToolOutputs
    {
        fn from(value: super::RunObjectRequiredActionSubmitToolOutputs) -> Self {
            Self {
                tool_calls: Ok(value.tool_calls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepCompletionUsageValue {
        completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepCompletionUsageValue {
        fn default() -> Self {
            Self {
                completion_tokens: Err("no value supplied for completion_tokens".to_string()),
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl RunStepCompletionUsageValue {
        pub fn completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.completion_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completion_tokens: {e}"));
            self
        }
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
    impl ::std::convert::TryFrom<RunStepCompletionUsageValue> for super::RunStepCompletionUsageValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepCompletionUsageValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion_tokens: value.completion_tokens?,
                prompt_tokens: value.prompt_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepCompletionUsageValue> for RunStepCompletionUsageValue {
        fn from(value: super::RunStepCompletionUsageValue) -> Self {
            Self {
                completion_tokens: Ok(value.completion_tokens),
                prompt_tokens: Ok(value.prompt_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaObject {
        delta: ::std::result::Result<super::RunStepDeltaObjectDelta, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::RunStepDeltaObjectObject, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDeltaObject {
        fn default() -> Self {
            Self {
                delta: Err("no value supplied for delta".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl RunStepDeltaObject {
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaObjectDelta>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
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
            T: ::std::convert::TryInto<super::RunStepDeltaObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaObject> for super::RunStepDeltaObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delta: value.delta?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaObject> for RunStepDeltaObject {
        fn from(value: super::RunStepDeltaObject) -> Self {
            Self {
                delta: Ok(value.delta),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaObjectDelta {
        step_details: ::std::result::Result<
            ::std::option::Option<super::RunStepDeltaObjectDeltaStepDetails>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaObjectDelta {
        fn default() -> Self {
            Self {
                step_details: Ok(Default::default()),
            }
        }
    }
    impl RunStepDeltaObjectDelta {
        pub fn step_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RunStepDeltaObjectDeltaStepDetails>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.step_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_details: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaObjectDelta> for super::RunStepDeltaObjectDelta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaObjectDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                step_details: value.step_details?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaObjectDelta> for RunStepDeltaObjectDelta {
        fn from(value: super::RunStepDeltaObjectDelta) -> Self {
            Self {
                step_details: Ok(value.step_details),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsMessageCreationObject {
        message_creation: ::std::result::Result<
            ::std::option::Option<
                super::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation,
            >,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsMessageCreationObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsMessageCreationObject {
        fn default() -> Self {
            Self {
                message_creation: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsMessageCreationObject {
        pub fn message_creation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_creation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_creation: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaStepDetailsMessageCreationObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsMessageCreationObject>
        for super::RunStepDeltaStepDetailsMessageCreationObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsMessageCreationObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_creation: value.message_creation?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsMessageCreationObject>
        for RunStepDeltaStepDetailsMessageCreationObject
    {
        fn from(value: super::RunStepDeltaStepDetailsMessageCreationObject) -> Self {
            Self {
                message_creation: Ok(value.message_creation),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
        message_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
        fn default() -> Self {
            Self {
                message_id: Ok(Default::default()),
            }
        }
    }
    impl RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>
        for super::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsMessageCreationObjectMessageCreation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_id: value.message_id?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>
        for RunStepDeltaStepDetailsMessageCreationObjectMessageCreation
    {
        fn from(value: super::RunStepDeltaStepDetailsMessageCreationObjectMessageCreation) -> Self {
            Self {
                message_id: Ok(value.message_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsCodeObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsCodeObject {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                id: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsCodeObject {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaStepDetailsToolCallsCodeObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsCodeObject>
        for super::RunStepDeltaStepDetailsToolCallsCodeObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsCodeObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                id: value.id?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsCodeObject>
        for RunStepDeltaStepDetailsToolCallsCodeObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsCodeObject) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                id: Ok(value.id),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
        input: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        outputs: ::std::result::Result<
            ::std::vec::Vec<
                super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
        fn default() -> Self {
            Self {
                input: Ok(Default::default()),
                outputs: Ok(Default::default()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn outputs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<
                    super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.outputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outputs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>
        for super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                outputs: value.outputs?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>
        for RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter) -> Self {
            Self {
                input: Ok(value.input),
                outputs: Ok(value.outputs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
        image: ::std::result::Result<
            ::std::option::Option<
                super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage,
            >,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
        fn default() -> Self {
            Self {
                image: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsCodeOutputImageObject>
        for super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsCodeOutputImageObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObject>
        for RunStepDeltaStepDetailsToolCallsCodeOutputImageObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObject) -> Self {
            Self {
                image: Ok(value.image),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>
        for super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>
        for RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
        index: ::std::result::Result<i64, ::std::string::String>,
        logs: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
        fn default() -> Self {
            Self {
                index: Err("no value supplied for index".to_string()),
                logs: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
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
        pub fn logs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.logs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logs: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject>
        for super::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                index: value.index?,
                logs: value.logs?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject>
        for RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject) -> Self {
            Self {
                index: Ok(value.index),
                logs: Ok(value.logs),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
        file_search: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsFileSearchObject {
        fn default() -> Self {
            Self {
                file_search: Err("no value supplied for file_search".to_string()),
                id: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsFileSearchObject {
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaStepDetailsToolCallsFileSearchObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsFileSearchObject>
        for super::RunStepDeltaStepDetailsToolCallsFileSearchObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsFileSearchObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_search: value.file_search?,
                id: value.id?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsFileSearchObject>
        for RunStepDeltaStepDetailsToolCallsFileSearchObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsFileSearchObject) -> Self {
            Self {
                file_search: Ok(value.file_search),
                id: Ok(value.id),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
        function: ::std::result::Result<
            ::std::option::Option<super::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsFunctionObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsFunctionObject {
        fn default() -> Self {
            Self {
                function: Ok(Default::default()),
                id: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsFunctionObject {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaStepDetailsToolCallsFunctionObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsFunctionObject>
        for super::RunStepDeltaStepDetailsToolCallsFunctionObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsFunctionObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                id: value.id?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsFunctionObject>
        for RunStepDeltaStepDetailsToolCallsFunctionObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsFunctionObject) -> Self {
            Self {
                function: Ok(value.function),
                id: Ok(value.id),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
        arguments: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        output: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                name: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
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
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>
        for super::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsFunctionObjectFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>
        for RunStepDeltaStepDetailsToolCallsFunctionObjectFunction
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsFunctionObjectFunction) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDeltaStepDetailsToolCallsObject {
        tool_calls: ::std::result::Result<
            ::std::vec::Vec<super::RunStepDeltaStepDetailsToolCallsObjectToolCallsItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RunStepDeltaStepDetailsToolCallsObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDeltaStepDetailsToolCallsObject {
        fn default() -> Self {
            Self {
                tool_calls: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDeltaStepDetailsToolCallsObject {
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::RunStepDeltaStepDetailsToolCallsObjectToolCallsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDeltaStepDetailsToolCallsObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDeltaStepDetailsToolCallsObject>
        for super::RunStepDeltaStepDetailsToolCallsObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDeltaStepDetailsToolCallsObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tool_calls: value.tool_calls?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDeltaStepDetailsToolCallsObject>
        for RunStepDeltaStepDetailsToolCallsObject
    {
        fn from(value: super::RunStepDeltaStepDetailsToolCallsObject) -> Self {
            Self {
                tool_calls: Ok(value.tool_calls),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsMessageCreationObject {
        message_creation: ::std::result::Result<
            super::RunStepDetailsMessageCreationObjectMessageCreation,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RunStepDetailsMessageCreationObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsMessageCreationObject {
        fn default() -> Self {
            Self {
                message_creation: Err("no value supplied for message_creation".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsMessageCreationObject {
        pub fn message_creation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsMessageCreationObjectMessageCreation>,
            T::Error: ::std::fmt::Display,
        {
            self.message_creation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_creation: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsMessageCreationObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsMessageCreationObject>
        for super::RunStepDetailsMessageCreationObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsMessageCreationObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_creation: value.message_creation?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsMessageCreationObject>
        for RunStepDetailsMessageCreationObject
    {
        fn from(value: super::RunStepDetailsMessageCreationObject) -> Self {
            Self {
                message_creation: Ok(value.message_creation),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsMessageCreationObjectMessageCreation {
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDetailsMessageCreationObjectMessageCreation {
        fn default() -> Self {
            Self {
                message_id: Err("no value supplied for message_id".to_string()),
            }
        }
    }
    impl RunStepDetailsMessageCreationObjectMessageCreation {
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsMessageCreationObjectMessageCreation>
        for super::RunStepDetailsMessageCreationObjectMessageCreation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsMessageCreationObjectMessageCreation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_id: value.message_id?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsMessageCreationObjectMessageCreation>
        for RunStepDetailsMessageCreationObjectMessageCreation
    {
        fn from(value: super::RunStepDetailsMessageCreationObjectMessageCreation) -> Self {
            Self {
                message_id: Ok(value.message_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsCodeObject {
        code_interpreter: ::std::result::Result<
            super::RunStepDetailsToolCallsCodeObjectCodeInterpreter,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDetailsToolCallsCodeObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsCodeObject {
        fn default() -> Self {
            Self {
                code_interpreter: Err("no value supplied for code_interpreter".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsCodeObject {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsCodeObjectCodeInterpreter>,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsCodeObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsCodeObject>
        for super::RunStepDetailsToolCallsCodeObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsCodeObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsCodeObject>
        for RunStepDetailsToolCallsCodeObject
    {
        fn from(value: super::RunStepDetailsToolCallsCodeObject) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
        input: ::std::result::Result<::std::string::String, ::std::string::String>,
        outputs: ::std::result::Result<
            ::std::vec::Vec<super::RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsCodeObjectCodeInterpreter {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                outputs: Err("no value supplied for outputs".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsCodeObjectCodeInterpreter {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn outputs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.outputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outputs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsCodeObjectCodeInterpreter>
        for super::RunStepDetailsToolCallsCodeObjectCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                outputs: value.outputs?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsCodeObjectCodeInterpreter>
        for RunStepDetailsToolCallsCodeObjectCodeInterpreter
    {
        fn from(value: super::RunStepDetailsToolCallsCodeObjectCodeInterpreter) -> Self {
            Self {
                input: Ok(value.input),
                outputs: Ok(value.outputs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsCodeOutputImageObject {
        image: ::std::result::Result<
            super::RunStepDetailsToolCallsCodeOutputImageObjectImage,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RunStepDetailsToolCallsCodeOutputImageObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsCodeOutputImageObject {
        fn default() -> Self {
            Self {
                image: Err("no value supplied for image".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsCodeOutputImageObject {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsCodeOutputImageObjectImage>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsCodeOutputImageObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsCodeOutputImageObject>
        for super::RunStepDetailsToolCallsCodeOutputImageObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsCodeOutputImageObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsCodeOutputImageObject>
        for RunStepDetailsToolCallsCodeOutputImageObject
    {
        fn from(value: super::RunStepDetailsToolCallsCodeOutputImageObject) -> Self {
            Self {
                image: Ok(value.image),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsCodeOutputImageObjectImage {
        fn default() -> Self {
            Self {
                file_id: Err("no value supplied for file_id".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsCodeOutputImageObjectImage {
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
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsCodeOutputImageObjectImage>
        for super::RunStepDetailsToolCallsCodeOutputImageObjectImage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsCodeOutputImageObjectImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsCodeOutputImageObjectImage>
        for RunStepDetailsToolCallsCodeOutputImageObjectImage
    {
        fn from(value: super::RunStepDetailsToolCallsCodeOutputImageObjectImage) -> Self {
            Self {
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
        logs: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDetailsToolCallsCodeOutputLogsObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsCodeOutputLogsObject {
        fn default() -> Self {
            Self {
                logs: Err("no value supplied for logs".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsCodeOutputLogsObject {
        pub fn logs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.logs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logs: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsCodeOutputLogsObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsCodeOutputLogsObject>
        for super::RunStepDetailsToolCallsCodeOutputLogsObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsCodeOutputLogsObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                logs: value.logs?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsCodeOutputLogsObject>
        for RunStepDetailsToolCallsCodeOutputLogsObject
    {
        fn from(value: super::RunStepDetailsToolCallsCodeOutputLogsObject) -> Self {
            Self {
                logs: Ok(value.logs),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFileSearchObject {
        file_search: ::std::result::Result<
            super::RunStepDetailsToolCallsFileSearchObjectFileSearch,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDetailsToolCallsFileSearchObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFileSearchObject {
        fn default() -> Self {
            Self {
                file_search: Err("no value supplied for file_search".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsFileSearchObject {
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsFileSearchObjectFileSearch>,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsFileSearchObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFileSearchObject>
        for super::RunStepDetailsToolCallsFileSearchObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFileSearchObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_search: value.file_search?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFileSearchObject>
        for RunStepDetailsToolCallsFileSearchObject
    {
        fn from(value: super::RunStepDetailsToolCallsFileSearchObject) -> Self {
            Self {
                file_search: Ok(value.file_search),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
        ranking_options: ::std::result::Result<
            ::std::option::Option<super::RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
            ::std::string::String,
        >,
        results: ::std::result::Result<
            ::std::vec::Vec<super::RunStepDetailsToolCallsFileSearchResultObject>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFileSearchObjectFileSearch {
        fn default() -> Self {
            Self {
                ranking_options: Ok(Default::default()),
                results: Ok(Default::default()),
            }
        }
    }
    impl RunStepDetailsToolCallsFileSearchObjectFileSearch {
        pub fn ranking_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ranking_options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranking_options: {e}"));
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::RunStepDetailsToolCallsFileSearchResultObject>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFileSearchObjectFileSearch>
        for super::RunStepDetailsToolCallsFileSearchObjectFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFileSearchObjectFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ranking_options: value.ranking_options?,
                results: value.results?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFileSearchObjectFileSearch>
        for RunStepDetailsToolCallsFileSearchObjectFileSearch
    {
        fn from(value: super::RunStepDetailsToolCallsFileSearchObjectFileSearch) -> Self {
            Self {
                ranking_options: Ok(value.ranking_options),
                results: Ok(value.results),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
        ranker: ::std::result::Result<super::FileSearchRanker, ::std::string::String>,
        score_threshold: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFileSearchRankingOptionsObject {
        fn default() -> Self {
            Self {
                ranker: Err("no value supplied for ranker".to_string()),
                score_threshold: Err("no value supplied for score_threshold".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsFileSearchRankingOptionsObject {
        pub fn ranker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileSearchRanker>,
            T::Error: ::std::fmt::Display,
        {
            self.ranker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranker: {e}"));
            self
        }
        pub fn score_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFileSearchRankingOptionsObject>
        for super::RunStepDetailsToolCallsFileSearchRankingOptionsObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFileSearchRankingOptionsObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ranker: value.ranker?,
                score_threshold: value.score_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFileSearchRankingOptionsObject>
        for RunStepDetailsToolCallsFileSearchRankingOptionsObject
    {
        fn from(value: super::RunStepDetailsToolCallsFileSearchRankingOptionsObject) -> Self {
            Self {
                ranker: Ok(value.ranker),
                score_threshold: Ok(value.score_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFileSearchResultObject {
        content: ::std::result::Result<
            ::std::vec::Vec<super::RunStepDetailsToolCallsFileSearchResultObjectContentItem>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        file_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        score: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFileSearchResultObject {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                file_id: Err("no value supplied for file_id".to_string()),
                file_name: Err("no value supplied for file_name".to_string()),
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsFileSearchResultObject {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::RunStepDetailsToolCallsFileSearchResultObjectContentItem>,
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
        pub fn file_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.file_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_name: {e}"));
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
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFileSearchResultObject>
        for super::RunStepDetailsToolCallsFileSearchResultObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFileSearchResultObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                file_id: value.file_id?,
                file_name: value.file_name?,
                score: value.score?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFileSearchResultObject>
        for RunStepDetailsToolCallsFileSearchResultObject
    {
        fn from(value: super::RunStepDetailsToolCallsFileSearchResultObject) -> Self {
            Self {
                content: Ok(value.content),
                file_id: Ok(value.file_id),
                file_name: Ok(value.file_name),
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFileSearchResultObjectContentItem {
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<
                super::RunStepDetailsToolCallsFileSearchResultObjectContentItemType,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFileSearchResultObjectContentItem {
        fn default() -> Self {
            Self {
                text: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl RunStepDetailsToolCallsFileSearchResultObjectContentItem {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RunStepDetailsToolCallsFileSearchResultObjectContentItemType,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFileSearchResultObjectContentItem>
        for super::RunStepDetailsToolCallsFileSearchResultObjectContentItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFileSearchResultObjectContentItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFileSearchResultObjectContentItem>
        for RunStepDetailsToolCallsFileSearchResultObjectContentItem
    {
        fn from(value: super::RunStepDetailsToolCallsFileSearchResultObjectContentItem) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFunctionObject {
        function: ::std::result::Result<
            super::RunStepDetailsToolCallsFunctionObjectFunction,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::RunStepDetailsToolCallsFunctionObjectType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFunctionObject {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsFunctionObject {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsFunctionObjectFunction>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsFunctionObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFunctionObject>
        for super::RunStepDetailsToolCallsFunctionObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFunctionObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFunctionObject>
        for RunStepDetailsToolCallsFunctionObject
    {
        fn from(value: super::RunStepDetailsToolCallsFunctionObject) -> Self {
            Self {
                function: Ok(value.function),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsFunctionObjectFunction {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        output: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsFunctionObjectFunction {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                name: Err("no value supplied for name".to_string()),
                output: Err("no value supplied for output".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsFunctionObjectFunction {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
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
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsFunctionObjectFunction>
        for super::RunStepDetailsToolCallsFunctionObjectFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsFunctionObjectFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsFunctionObjectFunction>
        for RunStepDetailsToolCallsFunctionObjectFunction
    {
        fn from(value: super::RunStepDetailsToolCallsFunctionObjectFunction) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepDetailsToolCallsObject {
        tool_calls: ::std::result::Result<
            ::std::vec::Vec<super::RunStepDetailsToolCallsObjectToolCallsItem>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::RunStepDetailsToolCallsObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepDetailsToolCallsObject {
        fn default() -> Self {
            Self {
                tool_calls: Err("no value supplied for tool_calls".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunStepDetailsToolCallsObject {
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::RunStepDetailsToolCallsObjectToolCallsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepDetailsToolCallsObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepDetailsToolCallsObject>
        for super::RunStepDetailsToolCallsObject
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepDetailsToolCallsObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tool_calls: value.tool_calls?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepDetailsToolCallsObject> for RunStepDetailsToolCallsObject {
        fn from(value: super::RunStepDetailsToolCallsObject) -> Self {
            Self {
                tool_calls: Ok(value.tool_calls),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepObject {
        assistant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        cancelled_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        completed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expired_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        failed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_error: ::std::result::Result<
            ::std::option::Option<super::RunStepObjectLastError>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        object: ::std::result::Result<super::RunStepObjectObject, ::std::string::String>,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<super::RunStepObjectStatus, ::std::string::String>,
        step_details: ::std::result::Result<super::RunStepObjectStepDetails, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::RunStepObjectType, ::std::string::String>,
        usage: ::std::result::Result<super::RunStepCompletionUsage, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepObject {
        fn default() -> Self {
            Self {
                assistant_id: Err("no value supplied for assistant_id".to_string()),
                cancelled_at: Err("no value supplied for cancelled_at".to_string()),
                completed_at: Err("no value supplied for completed_at".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                expired_at: Err("no value supplied for expired_at".to_string()),
                failed_at: Err("no value supplied for failed_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                last_error: Err("no value supplied for last_error".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                object: Err("no value supplied for object".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
                status: Err("no value supplied for status".to_string()),
                step_details: Err("no value supplied for step_details".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl RunStepObject {
        pub fn assistant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.assistant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assistant_id: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<::std::option::Option<super::RunStepObjectLastError>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_error: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObjectStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn step_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObjectStepDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.step_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_details: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepCompletionUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunStepObject> for super::RunStepObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assistant_id: value.assistant_id?,
                cancelled_at: value.cancelled_at?,
                completed_at: value.completed_at?,
                created_at: value.created_at?,
                expired_at: value.expired_at?,
                failed_at: value.failed_at?,
                id: value.id?,
                last_error: value.last_error?,
                metadata: value.metadata?,
                object: value.object?,
                run_id: value.run_id?,
                status: value.status?,
                step_details: value.step_details?,
                thread_id: value.thread_id?,
                type_: value.type_?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepObject> for RunStepObject {
        fn from(value: super::RunStepObject) -> Self {
            Self {
                assistant_id: Ok(value.assistant_id),
                cancelled_at: Ok(value.cancelled_at),
                completed_at: Ok(value.completed_at),
                created_at: Ok(value.created_at),
                expired_at: Ok(value.expired_at),
                failed_at: Ok(value.failed_at),
                id: Ok(value.id),
                last_error: Ok(value.last_error),
                metadata: Ok(value.metadata),
                object: Ok(value.object),
                run_id: Ok(value.run_id),
                status: Ok(value.status),
                step_details: Ok(value.step_details),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunStepObjectLastError {
        code: ::std::result::Result<super::RunStepObjectLastErrorCode, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RunStepObjectLastError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl RunStepObjectLastError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunStepObjectLastErrorCode>,
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
    impl ::std::convert::TryFrom<RunStepObjectLastError> for super::RunStepObjectLastError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunStepObjectLastError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::RunStepObjectLastError> for RunStepObjectLastError {
        fn from(value: super::RunStepObjectLastError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunToolCallObject {
        function: ::std::result::Result<super::RunToolCallObjectFunction, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::RunToolCallObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for RunToolCallObject {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RunToolCallObject {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunToolCallObjectFunction>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RunToolCallObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RunToolCallObject> for super::RunToolCallObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunToolCallObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RunToolCallObject> for RunToolCallObject {
        fn from(value: super::RunToolCallObject) -> Self {
            Self {
                function: Ok(value.function),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RunToolCallObjectFunction {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RunToolCallObjectFunction {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl RunToolCallObjectFunction {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {e}"));
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
    }
    impl ::std::convert::TryFrom<RunToolCallObjectFunction> for super::RunToolCallObjectFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RunToolCallObjectFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::RunToolCallObjectFunction> for RunToolCallObjectFunction {
        fn from(value: super::RunToolCallObjectFunction) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubmitToolOuputsToRunParams {
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SubmitToolOuputsToRunParams {
        fn default() -> Self {
            Self {
                run_id: Err("no value supplied for run_id".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl SubmitToolOuputsToRunParams {
        pub fn run_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.run_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for run_id: {e}"));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubmitToolOuputsToRunParams> for super::SubmitToolOuputsToRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubmitToolOuputsToRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                run_id: value.run_id?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::SubmitToolOuputsToRunParams> for SubmitToolOuputsToRunParams {
        fn from(value: super::SubmitToolOuputsToRunParams) -> Self {
            Self {
                run_id: Ok(value.run_id),
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubmitToolOutputsRunRequest {
        stream: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        tool_outputs: ::std::result::Result<
            ::std::vec::Vec<super::SubmitToolOutputsRunRequestToolOutputsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubmitToolOutputsRunRequest {
        fn default() -> Self {
            Self {
                stream: Ok(Default::default()),
                tool_outputs: Err("no value supplied for tool_outputs".to_string()),
            }
        }
    }
    impl SubmitToolOutputsRunRequest {
        pub fn stream<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream: {e}"));
            self
        }
        pub fn tool_outputs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::SubmitToolOutputsRunRequestToolOutputsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_outputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_outputs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubmitToolOutputsRunRequest> for super::SubmitToolOutputsRunRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubmitToolOutputsRunRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                stream: value.stream?,
                tool_outputs: value.tool_outputs?,
            })
        }
    }
    impl ::std::convert::From<super::SubmitToolOutputsRunRequest> for SubmitToolOutputsRunRequest {
        fn from(value: super::SubmitToolOutputsRunRequest) -> Self {
            Self {
                stream: Ok(value.stream),
                tool_outputs: Ok(value.tool_outputs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubmitToolOutputsRunRequestToolOutputsItem {
        output: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tool_call_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubmitToolOutputsRunRequestToolOutputsItem {
        fn default() -> Self {
            Self {
                output: Ok(Default::default()),
                tool_call_id: Ok(Default::default()),
            }
        }
    }
    impl SubmitToolOutputsRunRequestToolOutputsItem {
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
        pub fn tool_call_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_call_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_call_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubmitToolOutputsRunRequestToolOutputsItem>
        for super::SubmitToolOutputsRunRequestToolOutputsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubmitToolOutputsRunRequestToolOutputsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                output: value.output?,
                tool_call_id: value.tool_call_id?,
            })
        }
    }
    impl ::std::convert::From<super::SubmitToolOutputsRunRequestToolOutputsItem>
        for SubmitToolOutputsRunRequestToolOutputsItem
    {
        fn from(value: super::SubmitToolOutputsRunRequestToolOutputsItem) -> Self {
            Self {
                output: Ok(value.output),
                tool_call_id: Ok(value.tool_call_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadObject {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        object: ::std::result::Result<super::ThreadObjectObject, ::std::string::String>,
        tool_resources: ::std::result::Result<
            ::std::option::Option<super::ThreadObjectToolResources>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ThreadObject {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                object: Err("no value supplied for object".to_string()),
                tool_resources: Err("no value supplied for tool_resources".to_string()),
            }
        }
    }
    impl ThreadObject {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadObjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn tool_resources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ThreadObjectToolResources>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_resources: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadObject> for super::ThreadObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                metadata: value.metadata?,
                object: value.object?,
                tool_resources: value.tool_resources?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadObject> for ThreadObject {
        fn from(value: super::ThreadObject) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                object: Ok(value.object),
                tool_resources: Ok(value.tool_resources),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadObjectToolResources {
        code_interpreter: ::std::result::Result<
            ::std::option::Option<super::ThreadObjectToolResourcesCodeInterpreter>,
            ::std::string::String,
        >,
        file_search: ::std::result::Result<
            ::std::option::Option<super::ThreadObjectToolResourcesFileSearch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ThreadObjectToolResources {
        fn default() -> Self {
            Self {
                code_interpreter: Ok(Default::default()),
                file_search: Ok(Default::default()),
            }
        }
    }
    impl ThreadObjectToolResources {
        pub fn code_interpreter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ThreadObjectToolResourcesCodeInterpreter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.code_interpreter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_interpreter: {e}"));
            self
        }
        pub fn file_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ThreadObjectToolResourcesFileSearch>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_search = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_search: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadObjectToolResources> for super::ThreadObjectToolResources {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadObjectToolResources,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_interpreter: value.code_interpreter?,
                file_search: value.file_search?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadObjectToolResources> for ThreadObjectToolResources {
        fn from(value: super::ThreadObjectToolResources) -> Self {
            Self {
                code_interpreter: Ok(value.code_interpreter),
                file_search: Ok(value.file_search),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadObjectToolResourcesCodeInterpreter {
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadObjectToolResourcesCodeInterpreter {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
            }
        }
    }
    impl ThreadObjectToolResourcesCodeInterpreter {
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
    }
    impl ::std::convert::TryFrom<ThreadObjectToolResourcesCodeInterpreter>
        for super::ThreadObjectToolResourcesCodeInterpreter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadObjectToolResourcesCodeInterpreter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadObjectToolResourcesCodeInterpreter>
        for ThreadObjectToolResourcesCodeInterpreter
    {
        fn from(value: super::ThreadObjectToolResourcesCodeInterpreter) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadObjectToolResourcesFileSearch {
        vector_store_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadObjectToolResourcesFileSearch {
        fn default() -> Self {
            Self {
                vector_store_ids: Ok(Default::default()),
            }
        }
    }
    impl ThreadObjectToolResourcesFileSearch {
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
    impl ::std::convert::TryFrom<ThreadObjectToolResourcesFileSearch>
        for super::ThreadObjectToolResourcesFileSearch
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadObjectToolResourcesFileSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vector_store_ids: value.vector_store_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadObjectToolResourcesFileSearch>
        for ThreadObjectToolResourcesFileSearch
    {
        fn from(value: super::ThreadObjectToolResourcesFileSearch) -> Self {
            Self {
                vector_store_ids: Ok(value.vector_store_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TruncationObject {
        last_messages: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::TruncationObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for TruncationObject {
        fn default() -> Self {
            Self {
                last_messages: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TruncationObject {
        pub fn last_messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_messages: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TruncationObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TruncationObject> for super::TruncationObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TruncationObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                last_messages: value.last_messages?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TruncationObject> for TruncationObject {
        fn from(value: super::TruncationObject) -> Self {
            Self {
                last_messages: Ok(value.last_messages),
                type_: Ok(value.type_),
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
