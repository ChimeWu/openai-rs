#[doc = "`ChatApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatApiSchema {
    #[serde(rename = "CreateChatCompletionRequest")]
    pub create_chat_completion_request: CreateChatCompletionRequest,
    #[serde(rename = "CreateChatCompletionResponse")]
    pub create_chat_completion_response: CreateChatCompletionResponse,
    #[serde(rename = "CreateChatCompletionStreamResponse")]
    pub create_chat_completion_stream_response: CreateChatCompletionStreamResponse,
    #[serde(rename = "DeleteChatCompletionResponse")]
    pub delete_chat_completion_response: ChatCompletionDeleted,
    #[serde(rename = "GetChatCompletionMessagesParams")]
    pub get_chat_completion_messages_params: GetChatCompletionMessagesParams,
    #[serde(rename = "GetChatCompletionMessagesResponse")]
    pub get_chat_completion_messages_response: ChatCompletionMessageList,
    #[serde(rename = "GetChatCompletionResponse")]
    pub get_chat_completion_response: CreateChatCompletionResponse,
    #[serde(rename = "ListChatCompletionsParams")]
    pub list_chat_completions_params: ListChatCompletionsParams,
    #[serde(rename = "ListChatCompletionsResponse")]
    pub list_chat_completions_response: ChatCompletionList,
    #[serde(rename = "UpdateChatCompletionRequest")]
    pub update_chat_completion_request: UpdateChatCompletionRequest,
    #[serde(rename = "UpdateChatCompletionResponse")]
    pub update_chat_completion_response: CreateChatCompletionResponse,
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionAllowedTools {
    #[doc = "Constrains the tools available to the model to a pre-defined set.\n\n`auto` allows the model to pick from among the allowed tools and generate a\nmessage.\n\n`required` requires the model to call one or more of the allowed tools.\n"]
    pub mode: ChatCompletionAllowedToolsMode,
    #[doc = "A list of tool definitions that the model should be allowed to call.\n\nFor the Chat Completions API, the list of tool definitions might look like:\n```json\n[\n  { \"type\": \"function\", \"function\": { \"name\": \"get_weather\" } },\n  { \"type\": \"function\", \"function\": { \"name\": \"get_time\" } }\n]\n```\n"]
    pub tools: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionAllowedToolsChoice {
    pub allowed_tools: ChatCompletionAllowedTools,
    #[doc = "Allowed tool configuration type. Always `allowed_tools`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionAllowedToolsChoiceType,
}
#[doc = "Allowed tool configuration type. Always `allowed_tools`."]
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
pub enum ChatCompletionAllowedToolsChoiceType {
    #[serde(rename = "allowed_tools")]
    AllowedTools,
}
impl ::std::fmt::Display for ChatCompletionAllowedToolsChoiceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllowedTools => f.write_str("allowed_tools"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionAllowedToolsChoiceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowed_tools" => Ok(Self::AllowedTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionAllowedToolsChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionAllowedToolsChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n\n`auto` allows the model to pick from among the allowed tools and generate a\nmessage.\n\n`required` requires the model to call one or more of the allowed tools.\n"]
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
pub enum ChatCompletionAllowedToolsMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ChatCompletionAllowedToolsMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionAllowedToolsMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionAllowedToolsMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionAllowedToolsMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionDeleted`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionDeleted {
    #[doc = "Whether the chat completion was deleted."]
    pub deleted: bool,
    #[doc = "The ID of the chat completion that was deleted."]
    pub id: ::std::string::String,
    #[doc = "The type of object being deleted."]
    pub object: ChatCompletionDeletedObject,
}
#[doc = "The type of object being deleted."]
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
pub enum ChatCompletionDeletedObject {
    #[serde(rename = "chat.completion.deleted")]
    ChatCompletionDeleted,
}
impl ::std::fmt::Display for ChatCompletionDeletedObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatCompletionDeleted => f.write_str("chat.completion.deleted"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionDeletedObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chat.completion.deleted" => Ok(Self::ChatCompletionDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionDeletedObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionDeletedObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionFunctionCallOption {
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
#[doc = "`ChatCompletionFunctions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionFunctions {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<FunctionParameters>,
}
#[doc = "An object representing a list of Chat Completions.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionList {
    #[doc = "An array of chat completion objects.\n"]
    pub data: ::std::vec::Vec<CreateChatCompletionResponse>,
    #[doc = "The identifier of the first chat completion in the data array."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates whether there are more Chat Completions available."]
    pub has_more: bool,
    #[doc = "The identifier of the last chat completion in the data array."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: ChatCompletionListObject,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
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
pub enum ChatCompletionListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ChatCompletionListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A call to a custom tool created by the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageCustomToolCall {
    pub custom: ChatCompletionMessageCustomToolCallCustom,
    #[doc = "The ID of the tool call."]
    pub id: ::std::string::String,
    #[doc = "The type of the tool. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageCustomToolCallType,
}
#[doc = "The custom tool that the model called."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageCustomToolCallCustom {
    #[doc = "The input for the custom tool call generated by the model."]
    pub input: ::std::string::String,
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
}
#[doc = "The type of the tool. Always `custom`."]
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
pub enum ChatCompletionMessageCustomToolCallType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for ChatCompletionMessageCustomToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageCustomToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageCustomToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionMessageCustomToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An object representing a list of chat completion messages.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageList {
    #[doc = "An array of chat completion message objects.\n"]
    pub data: ::std::vec::Vec<ChatCompletionMessageListDataItem>,
    #[doc = "The identifier of the first chat message in the data array."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates whether there are more chat messages available."]
    pub has_more: bool,
    #[doc = "The identifier of the last chat message in the data array."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: ChatCompletionMessageListObject,
}
#[doc = "`ChatCompletionMessageListDataItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content_parts:
        ::std::option::Option<::std::vec::Vec<ChatCompletionMessageListDataItemContentPartsItem>>,
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub role: ChatCompletionMessageListDataItemRole,
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<ChatCompletionMessageListDataItemContentPartsItemFile>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<ChatCompletionMessageListDataItemContentPartsItemImageUrl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio:
        ::std::option::Option<ChatCompletionMessageListDataItemContentPartsItemInputAudio>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageListDataItemContentPartsItemType,
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemFile`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItemFile {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_data: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemImageUrl`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItemImageUrl {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItemInputAudio {
    pub data: ::std::string::String,
    pub format: ::std::string::String,
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemType`"]
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
pub enum ChatCompletionMessageListDataItemContentPartsItemType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image_url")]
    ImageUrl,
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for ChatCompletionMessageListDataItemContentPartsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::ImageUrl => f.write_str("image_url"),
            Self::InputAudio => f.write_str("input_audio"),
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageListDataItemContentPartsItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image_url" => Ok(Self::ImageUrl),
            "input_audio" => Ok(Self::InputAudio),
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageListDataItemContentPartsItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionMessageListDataItemContentPartsItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionMessageListDataItemRole`"]
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
pub enum ChatCompletionMessageListDataItemRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for ChatCompletionMessageListDataItemRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::Tool => f.write_str("tool"),
            Self::System => f.write_str("system"),
            Self::Function => f.write_str("function"),
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageListDataItemRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "tool" => Ok(Self::Tool),
            "system" => Ok(Self::System),
            "function" => Ok(Self::Function),
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageListDataItemRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionMessageListDataItemRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
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
pub enum ChatCompletionMessageListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ChatCompletionMessageListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionMessageListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A call to a function tool created by the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageToolCall {
    pub function: ChatCompletionMessageToolCallFunction,
    #[doc = "The ID of the tool call."]
    pub id: ::std::string::String,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageToolCallType,
}
#[doc = "`ChatCompletionMessageToolCallChunk`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageToolCallChunk {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function: ::std::option::Option<ChatCompletionMessageToolCallChunkFunction>,
    #[doc = "The ID of the tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    pub index: i64,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ChatCompletionMessageToolCallChunkType>,
}
#[doc = "`ChatCompletionMessageToolCallChunkFunction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionMessageToolCallChunkFunction {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub arguments: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
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
pub enum ChatCompletionMessageToolCallChunkType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionMessageToolCallChunkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageToolCallChunkType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageToolCallChunkType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionMessageToolCallChunkType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The function that the model called."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageToolCallFunction {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
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
pub enum ChatCompletionMessageToolCallType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionMessageToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionMessageToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionMessageToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionMessageToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The tool calls generated by the model, such as function calls."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ChatCompletionMessageToolCalls(pub ::std::vec::Vec<ChatCompletionMessageToolCallsItem>);
impl ::std::ops::Deref for ChatCompletionMessageToolCalls {
    type Target = ::std::vec::Vec<ChatCompletionMessageToolCallsItem>;
    fn deref(&self) -> &::std::vec::Vec<ChatCompletionMessageToolCallsItem> {
        &self.0
    }
}
impl ::std::convert::From<ChatCompletionMessageToolCalls>
    for ::std::vec::Vec<ChatCompletionMessageToolCallsItem>
{
    fn from(value: ChatCompletionMessageToolCalls) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionMessageToolCallsItem>>
    for ChatCompletionMessageToolCalls
{
    fn from(value: ::std::vec::Vec<ChatCompletionMessageToolCallsItem>) -> Self {
        Self(value)
    }
}
#[doc = "`ChatCompletionMessageToolCallsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionMessageToolCallsItem {
    ToolCall(ChatCompletionMessageToolCall),
    CustomToolCall(ChatCompletionMessageCustomToolCall),
}
impl ::std::convert::From<ChatCompletionMessageToolCall> for ChatCompletionMessageToolCallsItem {
    fn from(value: ChatCompletionMessageToolCall) -> Self {
        Self::ToolCall(value)
    }
}
impl ::std::convert::From<ChatCompletionMessageCustomToolCall>
    for ChatCompletionMessageToolCallsItem
{
    fn from(value: ChatCompletionMessageCustomToolCall) -> Self {
        Self::CustomToolCall(value)
    }
}
#[doc = "Moderation results or errors for the request input and generated output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionModeration {
    #[doc = "Moderation for the request input."]
    pub input: ChatCompletionModerationInput,
    #[doc = "Moderation for the generated output."]
    pub output: ChatCompletionModerationOutput,
}
#[doc = "An error produced while attempting moderation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionModerationError {
    #[doc = "The error code."]
    pub code: ::std::string::String,
    #[doc = "The error message."]
    pub message: ::std::string::String,
    #[doc = "The object type, which is always `error`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionModerationErrorType,
}
#[doc = "The object type, which is always `error`."]
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
pub enum ChatCompletionModerationErrorType {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for ChatCompletionModerationErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionModerationErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionModerationErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionModerationErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Moderation for the request input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionModerationInput {
    Results(ChatCompletionModerationResults),
    Error(ChatCompletionModerationError),
}
impl ::std::convert::From<ChatCompletionModerationResults> for ChatCompletionModerationInput {
    fn from(value: ChatCompletionModerationResults) -> Self {
        Self::Results(value)
    }
}
impl ::std::convert::From<ChatCompletionModerationError> for ChatCompletionModerationInput {
    fn from(value: ChatCompletionModerationError) -> Self {
        Self::Error(value)
    }
}
#[doc = "Moderation for the generated output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionModerationOutput {
    Results(ChatCompletionModerationResults),
    Error(ChatCompletionModerationError),
}
impl ::std::convert::From<ChatCompletionModerationResults> for ChatCompletionModerationOutput {
    fn from(value: ChatCompletionModerationResults) -> Self {
        Self::Results(value)
    }
}
impl ::std::convert::From<ChatCompletionModerationError> for ChatCompletionModerationOutput {
    fn from(value: ChatCompletionModerationError) -> Self {
        Self::Error(value)
    }
}
#[doc = "Successful moderation results for the request input or generated output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionModerationResults {
    #[doc = "The moderation model used to generate the results."]
    pub model: ::std::string::String,
    #[doc = "A list of moderation results."]
    pub results: ::std::vec::Vec<ModerationResultBody>,
    #[doc = "The object type, which is always `moderation_results`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionModerationResultsType,
}
#[doc = "The object type, which is always `moderation_results`."]
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
pub enum ChatCompletionModerationResultsType {
    #[serde(rename = "moderation_results")]
    ModerationResults,
}
impl ::std::fmt::Display for ChatCompletionModerationResultsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ModerationResults => f.write_str("moderation_results"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionModerationResultsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "moderation_results" => Ok(Self::ModerationResults),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionModerationResultsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionModerationResultsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific function."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoice {
    pub function: ChatCompletionNamedToolChoiceFunction,
    #[doc = "For function calling, the type is always `function`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceType,
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific custom tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoiceCustom {
    pub custom: ChatCompletionNamedToolChoiceCustomCustom,
    #[doc = "For custom tool calling, the type is always `custom`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceCustomType,
}
#[doc = "`ChatCompletionNamedToolChoiceCustomCustom`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoiceCustomCustom {
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
}
#[doc = "For custom tool calling, the type is always `custom`."]
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
pub enum ChatCompletionNamedToolChoiceCustomType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for ChatCompletionNamedToolChoiceCustomType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionNamedToolChoiceCustomType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionNamedToolChoiceCustomType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionNamedToolChoiceCustomType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionNamedToolChoiceFunction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
#[doc = "For function calling, the type is always `function`."]
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
pub enum ChatCompletionNamedToolChoiceType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionNamedToolChoiceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionNamedToolChoiceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionNamedToolChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionNamedToolChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Messages sent by the model in response to user messages.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestAssistantMessage {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<ChatCompletionRequestAssistantMessageAudio>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<ChatCompletionRequestAssistantMessageContent>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call: ::std::option::Option<ChatCompletionRequestAssistantMessageFunctionCall>,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refusal: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the messages author, in this case `assistant`."]
    pub role: ChatCompletionRequestAssistantMessageRole,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_calls: ::std::option::Option<ChatCompletionMessageToolCalls>,
}
#[doc = "Data about a previous audio response from the model.\n[Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageAudio {
    #[doc = "Unique identifier for a previous audio response from the model.\n"]
    pub id: ::std::string::String,
}
#[doc = "The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestAssistantMessageContentPart>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestAssistantMessageContentPart>>
    for ChatCompletionRequestAssistantMessageContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestAssistantMessageContentPart>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "`ChatCompletionRequestAssistantMessageContentPart`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Refusal(ChatCompletionRequestMessageContentPartRefusal),
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestAssistantMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartRefusal>
    for ChatCompletionRequestAssistantMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartRefusal) -> Self {
        Self::Refusal(value)
    }
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
#[doc = "The role of the messages author, in this case `assistant`."]
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
pub enum ChatCompletionRequestAssistantMessageRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for ChatCompletionRequestAssistantMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestAssistantMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestAssistantMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestAssistantMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, `developer` messages\nreplace the previous `system` messages.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestDeveloperMessage {
    #[doc = "The contents of the developer message."]
    pub content: ChatCompletionRequestDeveloperMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the messages author, in this case `developer`."]
    pub role: ChatCompletionRequestDeveloperMessageRole,
}
#[doc = "The contents of the developer message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestMessageContentPartText>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestMessageContentPartText>>
    for ChatCompletionRequestDeveloperMessageContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "The role of the messages author, in this case `developer`."]
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
pub enum ChatCompletionRequestDeveloperMessageRole {
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for ChatCompletionRequestDeveloperMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestDeveloperMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestDeveloperMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestDeveloperMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionRequestFunctionMessage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestFunctionMessage {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
    #[doc = "The role of the messages author, in this case `function`."]
    pub role: ChatCompletionRequestFunctionMessageRole,
}
#[doc = "The role of the messages author, in this case `function`."]
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
pub enum ChatCompletionRequestFunctionMessageRole {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionRequestFunctionMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestFunctionMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestFunctionMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestFunctionMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionRequestMessage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessage {
    DeveloperMessage(ChatCompletionRequestDeveloperMessage),
    SystemMessage(ChatCompletionRequestSystemMessage),
    UserMessage(ChatCompletionRequestUserMessage),
    AssistantMessage(ChatCompletionRequestAssistantMessage),
    ToolMessage(ChatCompletionRequestToolMessage),
    FunctionMessage(ChatCompletionRequestFunctionMessage),
}
impl ::std::convert::From<ChatCompletionRequestDeveloperMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestDeveloperMessage) -> Self {
        Self::DeveloperMessage(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestSystemMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestSystemMessage) -> Self {
        Self::SystemMessage(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestUserMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestUserMessage) -> Self {
        Self::UserMessage(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestAssistantMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestAssistantMessage) -> Self {
        Self::AssistantMessage(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestToolMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestToolMessage) -> Self {
        Self::ToolMessage(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestFunctionMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestFunctionMessage) -> Self {
        Self::FunctionMessage(value)
    }
}
#[doc = "Learn about [audio inputs](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudio {
    pub input_audio: ChatCompletionRequestMessageContentPartAudioInputAudio,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The type of the content part. Always `input_audio`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartAudioType,
}
#[doc = "`ChatCompletionRequestMessageContentPartAudioInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
    #[doc = "Base64 encoded audio data."]
    pub data: ::std::string::String,
    #[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
    pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}
#[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
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
pub enum ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wav => f.write_str("wav"),
            Self::Mp3 => f.write_str("mp3"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wav" => Ok(Self::Wav),
            "mp3" => Ok(Self::Mp3),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for ChatCompletionRequestMessageContentPartAudioInputAudioFormat
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartAudioInputAudioFormat
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the content part. Always `input_audio`."]
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
pub enum ChatCompletionRequestMessageContentPartAudioType {
    #[serde(rename = "input_audio")]
    InputAudio,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartAudioType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudio => f.write_str("input_audio"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartAudioType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio" => Ok(Self::InputAudio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartAudioType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartAudioType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Learn about [file inputs](https://developers.openai.com/api/docs/guides/text) for text generation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFile {
    pub file: ChatCompletionRequestMessageContentPartFileFile,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The type of the content part. Always `file`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartFileType,
}
#[doc = "`ChatCompletionRequestMessageContentPartFileFile`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartFileFile {
    #[doc = "The base64 encoded file data, used when passing the file to the model\nas a string.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_data: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of an uploaded file to use as input.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the file, used when passing the file to the model as a\nstring.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
}
#[doc = "The type of the content part. Always `file`."]
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
pub enum ChatCompletionRequestMessageContentPartFileType {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartFileType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartFileType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartFileType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartFileType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Learn about [image inputs](https://developers.openai.com/api/docs/guides/images-vision).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartImage {
    pub image_url: ChatCompletionRequestMessageContentPartImageImageUrl,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The type of the content part."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartImageType,
}
#[doc = "`ChatCompletionRequestMessageContentPartImageImageUrl`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
    #[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](https://developers.openai.com/api/docs/guides/images-vision#choose-an-image-detail-level)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<ChatCompletionRequestMessageContentPartImageImageUrlDetail>,
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    pub url: ::std::string::String,
}
#[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](https://developers.openai.com/api/docs/guides/images-vision#choose-an-image-detail-level)."]
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
pub enum ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartImageImageUrlDetail
{
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
pub enum ChatCompletionRequestMessageContentPartImageType {
    #[serde(rename = "image_url")]
    ImageUrl,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartImageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageUrl => f.write_str("image_url"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartImageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_url" => Ok(Self::ImageUrl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartImageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartImageType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionRequestMessageContentPartRefusal`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    #[doc = "The refusal message generated by the model."]
    pub refusal: ::std::string::String,
    #[doc = "The type of the content part."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartRefusalType,
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
pub enum ChatCompletionRequestMessageContentPartRefusalType {
    #[serde(rename = "refusal")]
    Refusal,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartRefusalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Refusal => f.write_str("refusal"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartRefusalType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "refusal" => Ok(Self::Refusal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartRefusalType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartRefusalType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Learn about [text inputs](https://developers.openai.com/api/docs/guides/text).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartText {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The text content."]
    pub text: ::std::string::String,
    #[doc = "The type of the content part."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartTextType,
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
pub enum ChatCompletionRequestMessageContentPartTextType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for ChatCompletionRequestMessageContentPartTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestMessageContentPartTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestMessageContentPartTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionRequestMessageContentPartTextType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, use `developer` messages\nfor this purpose instead.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestSystemMessage {
    #[doc = "The contents of the system message."]
    pub content: ChatCompletionRequestSystemMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the messages author, in this case `system`."]
    pub role: ChatCompletionRequestSystemMessageRole,
}
#[doc = "The contents of the system message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestSystemMessageContentPart>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestSystemMessageContentPart>>
    for ChatCompletionRequestSystemMessageContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestSystemMessageContentPart>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "`ChatCompletionRequestSystemMessageContentPart`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ChatCompletionRequestSystemMessageContentPart(
    pub ChatCompletionRequestMessageContentPartText,
);
impl ::std::ops::Deref for ChatCompletionRequestSystemMessageContentPart {
    type Target = ChatCompletionRequestMessageContentPartText;
    fn deref(&self) -> &ChatCompletionRequestMessageContentPartText {
        &self.0
    }
}
impl ::std::convert::From<ChatCompletionRequestSystemMessageContentPart>
    for ChatCompletionRequestMessageContentPartText
{
    fn from(value: ChatCompletionRequestSystemMessageContentPart) -> Self {
        value.0
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestSystemMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        Self(value)
    }
}
#[doc = "The role of the messages author, in this case `system`."]
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
pub enum ChatCompletionRequestSystemMessageRole {
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for ChatCompletionRequestSystemMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestSystemMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestSystemMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestSystemMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionRequestToolMessage`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestToolMessage {
    #[doc = "The contents of the tool message."]
    pub content: ChatCompletionRequestToolMessageContent,
    #[doc = "The role of the messages author, in this case `tool`."]
    pub role: ChatCompletionRequestToolMessageRole,
    #[doc = "Tool call that this message is responding to."]
    pub tool_call_id: ::std::string::String,
}
#[doc = "The contents of the tool message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestToolMessageContentPart>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestToolMessageContentPart>>
    for ChatCompletionRequestToolMessageContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestToolMessageContentPart>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "`ChatCompletionRequestToolMessageContentPart`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ChatCompletionRequestToolMessageContentPart(
    pub ChatCompletionRequestMessageContentPartText,
);
impl ::std::ops::Deref for ChatCompletionRequestToolMessageContentPart {
    type Target = ChatCompletionRequestMessageContentPartText;
    fn deref(&self) -> &ChatCompletionRequestMessageContentPartText {
        &self.0
    }
}
impl ::std::convert::From<ChatCompletionRequestToolMessageContentPart>
    for ChatCompletionRequestMessageContentPartText
{
    fn from(value: ChatCompletionRequestToolMessageContentPart) -> Self {
        value.0
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestToolMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        Self(value)
    }
}
#[doc = "The role of the messages author, in this case `tool`."]
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
pub enum ChatCompletionRequestToolMessageRole {
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for ChatCompletionRequestToolMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestToolMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestToolMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestToolMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Messages sent by an end user, containing prompts or additional context\ninformation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestUserMessage {
    #[doc = "The contents of the user message.\n"]
    pub content: ChatCompletionRequestUserMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the messages author, in this case `user`."]
    pub role: ChatCompletionRequestUserMessageRole,
}
#[doc = "The contents of the user message.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestUserMessageContentPart>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestUserMessageContentPart>>
    for ChatCompletionRequestUserMessageContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestUserMessageContentPart>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "`ChatCompletionRequestUserMessageContentPart`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Image(ChatCompletionRequestMessageContentPartImage),
    Audio(ChatCompletionRequestMessageContentPartAudio),
    File(ChatCompletionRequestMessageContentPartFile),
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartImage>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartImage) -> Self {
        Self::Image(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartAudio>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartAudio) -> Self {
        Self::Audio(value)
    }
}
impl ::std::convert::From<ChatCompletionRequestMessageContentPartFile>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartFile) -> Self {
        Self::File(value)
    }
}
#[doc = "The role of the messages author, in this case `user`."]
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
pub enum ChatCompletionRequestUserMessageRole {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for ChatCompletionRequestUserMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionRequestUserMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionRequestUserMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionRequestUserMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A chat completion message generated by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessage {
    #[doc = "Annotations for the message, when applicable, as when using the\n[web search tool](https://developers.openai.com/api/docs/guides/tools-web-search).\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub annotations: ::std::vec::Vec<ChatCompletionResponseMessageAnnotationsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<ChatCompletionResponseMessageAudio>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content: ::std::option::Option<::std::string::String>,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call: ::std::option::Option<ChatCompletionResponseMessageFunctionCall>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refusal: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the author of this message."]
    pub role: ChatCompletionResponseMessageRole,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_calls: ::std::option::Option<ChatCompletionMessageToolCalls>,
}
#[doc = "A URL citation when using web search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageAnnotationsItem {
    #[doc = "The type of the URL citation. Always `url_citation`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseMessageAnnotationsItemType,
    pub url_citation: ChatCompletionResponseMessageAnnotationsItemUrlCitation,
}
#[doc = "The type of the URL citation. Always `url_citation`."]
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
pub enum ChatCompletionResponseMessageAnnotationsItemType {
    #[serde(rename = "url_citation")]
    UrlCitation,
}
impl ::std::fmt::Display for ChatCompletionResponseMessageAnnotationsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UrlCitation => f.write_str("url_citation"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionResponseMessageAnnotationsItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url_citation" => Ok(Self::UrlCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionResponseMessageAnnotationsItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ChatCompletionResponseMessageAnnotationsItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A URL citation when using web search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageAnnotationsItemUrlCitation {
    #[doc = "The index of the last character of the URL citation in the message."]
    pub end_index: i64,
    #[doc = "The index of the first character of the URL citation in the message."]
    pub start_index: i64,
    #[doc = "The title of the web resource."]
    pub title: ::std::string::String,
    #[doc = "The URL of the web resource."]
    pub url: ::std::string::String,
}
#[doc = "If the audio output modality is requested, this object contains data\nabout the audio response from the model. [Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageAudio {
    #[doc = "Base64 encoded audio bytes generated by the model, in the format\nspecified in the request.\n"]
    pub data: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) for when this audio response will\nno longer be accessible on the server for use in multi-turn\nconversations.\n"]
    pub expires_at: i64,
    #[doc = "Unique identifier for this audio response."]
    pub id: ::std::string::String,
    #[doc = "Transcript of the audio generated by the model."]
    pub transcript: ::std::string::String,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
#[doc = "The role of the author of this message."]
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
pub enum ChatCompletionResponseMessageRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for ChatCompletionResponseMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionResponseMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionResponseMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionResponseMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionStreamOptions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ChatCompletionStreamOptions(pub ::std::option::Option<ChatCompletionStreamOptionsValue>);
impl ::std::ops::Deref for ChatCompletionStreamOptions {
    type Target = ::std::option::Option<ChatCompletionStreamOptionsValue>;
    fn deref(&self) -> &::std::option::Option<ChatCompletionStreamOptionsValue> {
        &self.0
    }
}
impl ::std::convert::From<ChatCompletionStreamOptions>
    for ::std::option::Option<ChatCompletionStreamOptionsValue>
{
    fn from(value: ChatCompletionStreamOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ChatCompletionStreamOptionsValue>>
    for ChatCompletionStreamOptions
{
    fn from(value: ::std::option::Option<ChatCompletionStreamOptionsValue>) -> Self {
        Self(value)
    }
}
#[doc = "Options for streaming response. Only set this when you set `stream: true`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionStreamOptionsValue {
    #[doc = "When true, stream obfuscation will be enabled. Stream obfuscation adds\nrandom characters to an `obfuscation` field on streaming delta events to\nnormalize payload sizes as a mitigation to certain side-channel attacks.\nThese obfuscation fields are included by default, but add a small amount\nof overhead to the data stream. You can set `include_obfuscation` to\nfalse to optimize for bandwidth if you trust the network links between\nyour application and the OpenAI API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_obfuscation: ::std::option::Option<bool>,
    #[doc = "If set, an additional chunk will be streamed before the `data: [DONE]`\nmessage. The `usage` field on this chunk shows the token usage statistics\nfor the entire request, and the `choices` field will always be an empty\narray.\n\nAll other chunks will also include a `usage` field, but with a null\nvalue. **NOTE:** If the stream is interrupted, you may not receive the\nfinal usage chunk which contains the total token usage for the request.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_usage: ::std::option::Option<bool>,
}
#[doc = "A chat completion delta generated by streamed model responses.\nStreamed audio can arrive in partial updates with an ID, base64 data, or\ntranscript text. The final audio update contains only its expiry timestamp.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionStreamResponseDelta {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<ChatCompletionStreamResponseDeltaAudio>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call: ::std::option::Option<ChatCompletionStreamResponseDeltaFunctionCall>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refusal: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the author of this message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<ChatCompletionStreamResponseDeltaRole>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_calls: ::std::vec::Vec<ChatCompletionMessageToolCallChunk>,
}
#[doc = "A partial audio response. Audio chunks may carry an ID, base64 data, or\ntranscript text; the final audio update contains only its expiry timestamp.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionStreamResponseDeltaAudio {
    #[doc = "Base64 encoded audio bytes generated by the model, in the format\nspecified in the request.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::std::string::String>,
    #[doc = "The Unix timestamp (in seconds) for when this audio response will no\nlonger be accessible on the server for use in multi-turn conversations.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "Unique identifier for this audio response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Transcript text from this audio chunk."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transcript: ::std::option::Option<::std::string::String>,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatCompletionStreamResponseDeltaFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub arguments: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
#[doc = "The role of the author of this message."]
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
pub enum ChatCompletionStreamResponseDeltaRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for ChatCompletionStreamResponseDeltaRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Developer => f.write_str("developer"),
            Self::System => f.write_str("system"),
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionStreamResponseDeltaRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "developer" => Ok(Self::Developer),
            "system" => Ok(Self::System),
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionStreamResponseDeltaRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionStreamResponseDeltaRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatCompletionTokenLogprob`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionTokenLogprob {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub bytes: ::std::option::Option<::std::vec::Vec<i64>>,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    pub logprob: f64,
    #[doc = "The token."]
    pub token: ::std::string::String,
    #[doc = "List of the most likely tokens and their log probability, at this token position. The number of entries may be fewer than the requested `top_logprobs`."]
    pub top_logprobs: ::std::vec::Vec<ChatCompletionTokenLogprobTopLogprobsItem>,
}
#[doc = "`ChatCompletionTokenLogprobTopLogprobsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionTokenLogprobTopLogprobsItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub bytes: ::std::option::Option<::std::vec::Vec<i64>>,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    pub logprob: f64,
    #[doc = "The token."]
    pub token: ::std::string::String,
}
#[doc = "A function tool that can be used to generate a response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionTool {
    pub function: FunctionObject,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tool and instead generates a message.\n`auto` means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools.\nSpecifying a particular tool via `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n\n`none` is the default when no tools are present. `auto` is the default if tools are present.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    ToolChoiceMode(ToolChoiceMode),
    ChatCompletionAllowedToolsChoice(ChatCompletionAllowedToolsChoice),
    ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
    ChatCompletionNamedToolChoiceCustom(ChatCompletionNamedToolChoiceCustom),
}
impl ::std::convert::From<ToolChoiceMode> for ChatCompletionToolChoiceOption {
    fn from(value: ToolChoiceMode) -> Self {
        Self::ToolChoiceMode(value)
    }
}
impl ::std::convert::From<ChatCompletionAllowedToolsChoice> for ChatCompletionToolChoiceOption {
    fn from(value: ChatCompletionAllowedToolsChoice) -> Self {
        Self::ChatCompletionAllowedToolsChoice(value)
    }
}
impl ::std::convert::From<ChatCompletionNamedToolChoice> for ChatCompletionToolChoiceOption {
    fn from(value: ChatCompletionNamedToolChoice) -> Self {
        Self::ChatCompletionNamedToolChoice(value)
    }
}
impl ::std::convert::From<ChatCompletionNamedToolChoiceCustom> for ChatCompletionToolChoiceOption {
    fn from(value: ChatCompletionNamedToolChoiceCustom) -> Self {
        Self::ChatCompletionNamedToolChoiceCustom(value)
    }
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
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
pub enum ChatCompletionToolType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Usage statistics for the completion request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompletionUsage {
    #[doc = "Number of tokens in the generated completion."]
    pub completion_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completion_tokens_details: ::std::option::Option<CompletionUsageCompletionTokensDetails>,
    #[doc = "Number of tokens in the prompt."]
    pub prompt_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_tokens_details: ::std::option::Option<CompletionUsagePromptTokensDetails>,
    #[doc = "Total number of tokens used in the request (prompt + completion)."]
    pub total_tokens: i64,
}
#[doc = "Breakdown of tokens used in a completion."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CompletionUsageCompletionTokensDetails {
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that appeared in the completion.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub accepted_prediction_tokens: ::std::option::Option<i64>,
    #[doc = "Audio input tokens generated by the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "Tokens generated by the model for reasoning."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_tokens: ::std::option::Option<i64>,
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that did not appear in the completion. However, like\nreasoning tokens, these tokens are still counted in the total\ncompletion tokens for purposes of billing, output, and context window\nlimits.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rejected_prediction_tokens: ::std::option::Option<i64>,
    #[doc = "Text output tokens generated by the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "Breakdown of tokens used in the prompt."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CompletionUsagePromptTokensDetails {
    #[doc = "Audio input tokens present in the prompt."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio_tokens: ::std::option::Option<i64>,
    #[doc = "The unadjusted number of prompt tokens written to cache."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_write_tokens: ::std::option::Option<i64>,
    #[doc = "Cached tokens present in the prompt."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cached_tokens: ::std::option::Option<i64>,
    #[doc = "Image input tokens present in the prompt."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_tokens: ::std::option::Option<i64>,
    #[doc = "Text input tokens present in the prompt."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text_tokens: ::std::option::Option<i64>,
}
#[doc = "`CreateChatCompletionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionRequest {
    #[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<CreateChatCompletionRequestAudio>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\ntheir existing frequency in the text so far, decreasing the model's\nlikelihood to repeat the same line verbatim.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub frequency_penalty: ::std::option::Option<f64>,
    #[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call: ::std::option::Option<CreateChatCompletionRequestFunctionCall>,
    #[doc = "Deprecated in favor of `tools`.\n\nA list of functions the model may generate JSON inputs for.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub functions: ::std::vec::Vec<ChatCompletionFunctions>,
    #[doc = "Modify the likelihood of specified tokens appearing in the completion.\n\nAccepts a JSON object that maps tokens (specified by their token ID in the\ntokenizer) to an associated bias value from -100 to 100. Mathematically,\nthe bias is added to the logits generated by the model prior to sampling.\nThe exact effect will vary per model, but values between -1 and 1 should\ndecrease or increase likelihood of selection; values like -100 or 100\nshould result in a ban or exclusive selection of the relevant token.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logit_bias: ::std::option::Option<::std::collections::HashMap<::std::string::String, i64>>,
    #[doc = "Whether to return log probabilities of the output tokens or not. If true,\nreturns the log probabilities of each output token returned in the\n`content` of `message`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<bool>,
    #[doc = "An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](https://developers.openai.com/api/docs/guides/reasoning).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[doc = "The maximum number of [tokens](https://platform.openai.com/tokenizer) that can be generated in the\nchat completion. This value can be used to control\n[costs](https://openai.com/api/pricing/) for text generated via API.\n\nThis value is now deprecated in favor of `max_completion_tokens`, and is\nnot compatible with [o-series models](https://developers.openai.com/api/docs/guides/reasoning).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tokens: ::std::option::Option<i64>,
    #[doc = "A list of messages comprising the conversation so far. Depending on the\n[model](https://developers.openai.com/api/docs/models) you use, different message types (modalities) are\nsupported, like [text](https://developers.openai.com/api/docs/guides/text),\n[images](https://developers.openai.com/api/docs/guides/images-vision), and [audio](https://developers.openai.com/api/docs/guides/audio).\n"]
    pub messages: ::std::vec::Vec<ChatCompletionRequestMessage>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub modalities: ::std::option::Option<ResponseModalities>,
    #[doc = "Model ID used to generate the response, like `gpt-6-astra` or `o3`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models)\nto browse and compare available models.\n"]
    pub model: ModelIdsShared,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ModerationParam>,
    #[doc = "How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<ParallelToolCalls>,
    #[doc = "Configuration for a [Predicted Output](https://developers.openai.com/api/docs/guides/predicted-outputs),\nwhich can greatly improve response times when large parts of the model\nresponse are known ahead of time. This is most common when you are\nregenerating a file with only minor changes to most of the content.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prediction: ::std::option::Option<PredictionContent>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\nwhether they appear in the text so far, increasing the model's likelihood\nto talk about new topics.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub presence_penalty: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_options: ::std::option::Option<PromptCacheOptionsParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention:
        ::std::option::Option<CreateChatCompletionRequestPromptCacheRetention>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
    #[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateChatCompletionRequestResponseFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<CreateChatCompletionRequestSafetyIdentifier>,
    #[doc = "This feature is in Beta.\nIf specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.\nDeterminism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stop: ::std::option::Option<StopConfiguration>,
    #[doc = "Whether or not to store the output of this chat completion request for\nuse in our [model distillation](https://developers.openai.com/api/docs/guides/supervised-fine-tuning#distilling-from-a-larger-model) or\n[evals](https://developers.openai.com/api/docs/guides/evals) products.\n\nSupports text and image inputs. Note: image inputs over 8MB will be dropped.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<bool>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\nSee the [Streaming section below](https://developers.openai.com/api/reference/resources/chat/subresources/completions/streaming-events)\nfor more information, along with the [streaming responses](https://developers.openai.com/api/docs/guides/streaming-responses)\nguide for more information on how to handle the streaming events.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream_options: ::std::option::Option<ChatCompletionStreamOptions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<ChatCompletionToolChoiceOption>,
    #[doc = "A list of tools the model may call. You can provide either\n[custom tools](https://developers.openai.com/api/docs/guides/function-calling#custom-tools) or\n[function tools](https://developers.openai.com/api/docs/guides/function-calling).\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<CreateChatCompletionRequestToolsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[doc = "This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead to maintain caching optimizations.\nA stable identifier for your end-users.\nUsed to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verbosity: ::std::option::Option<Verbosity>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_search_options: ::std::option::Option<WebSearch>,
}
#[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionRequestAudio {
    #[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
    pub format: CreateChatCompletionRequestAudioFormat,
    #[doc = "The voice the model uses to respond. Supported built-in voices are\n`alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`,\n`sage`, `shimmer`, `marin`, and `cedar`. You may also provide a\ncustom voice object with an `id`, for example `{ \"id\": \"voice_1234\" }`.\nCustom voices must be created from audio samples.\n"]
    pub voice: VoiceIdsOrCustomVoice,
}
#[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
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
pub enum CreateChatCompletionRequestAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "pcm16")]
    Pcm16,
}
impl ::std::fmt::Display for CreateChatCompletionRequestAudioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wav => f.write_str("wav"),
            Self::Aac => f.write_str("aac"),
            Self::Mp3 => f.write_str("mp3"),
            Self::Flac => f.write_str("flac"),
            Self::Opus => f.write_str("opus"),
            Self::Pcm16 => f.write_str("pcm16"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionRequestAudioFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wav" => Ok(Self::Wav),
            "aac" => Ok(Self::Aac),
            "mp3" => Ok(Self::Mp3),
            "flac" => Ok(Self::Flac),
            "opus" => Ok(Self::Opus),
            "pcm16" => Ok(Self::Pcm16),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionRequestAudioFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateChatCompletionRequestAudioFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestFunctionCall {
    String(CreateChatCompletionRequestFunctionCallString),
    ChatCompletionFunctionCallOption(ChatCompletionFunctionCallOption),
}
impl ::std::convert::From<CreateChatCompletionRequestFunctionCallString>
    for CreateChatCompletionRequestFunctionCall
{
    fn from(value: CreateChatCompletionRequestFunctionCallString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<ChatCompletionFunctionCallOption>
    for CreateChatCompletionRequestFunctionCall
{
    fn from(value: ChatCompletionFunctionCallOption) -> Self {
        Self::ChatCompletionFunctionCallOption(value)
    }
}
#[doc = "`none` means the model will not call a function and instead generates a message. `auto` means the model can pick between generating a message or calling a function.\n"]
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
pub enum CreateChatCompletionRequestFunctionCallString {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateChatCompletionRequestFunctionCallString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionRequestFunctionCallString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionRequestFunctionCallString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateChatCompletionRequestFunctionCallString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Deprecated. Use `prompt_cache_options.ttl` instead.\n\nThe retention policy for the prompt cache. Set to `24h` to enable extended prompt caching, which keeps cached prefixes active for longer, up to a maximum of 24 hours. [Learn more](https://developers.openai.com/api/docs/guides/prompt-caching#prompt-cache-retention).\nThis field expresses a maximum retention policy, while\n`prompt_cache_options.ttl` expresses a minimum cache lifetime. The two\nfields are independent and do not interact.\nFor `gpt-5.5`, `gpt-5.5-pro`, and future models, only `24h` is supported.\n\nFor older models that support both `in_memory` and `24h`, the default depends on your organization's data retention policy:\n  - Organizations without ZDR enabled default to `24h`.\n  - Organizations with ZDR enabled default to `in_memory` when `prompt_cache_retention` is not specified.\n"]
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
pub enum CreateChatCompletionRequestPromptCacheRetention {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for CreateChatCompletionRequestPromptCacheRetention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionRequestPromptCacheRetention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionRequestPromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateChatCompletionRequestPromptCacheRetention
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestResponseFormat {
    Text(ResponseFormatText),
    JsonSchema(ResponseFormatJsonSchema),
    JsonObject(ResponseFormatJsonObject),
}
impl ::std::convert::From<ResponseFormatText> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<ResponseFormatJsonSchema> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatJsonSchema) -> Self {
        Self::JsonSchema(value)
    }
}
impl ::std::convert::From<ResponseFormatJsonObject> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatJsonObject) -> Self {
        Self::JsonObject(value)
    }
}
#[doc = "A stable identifier used to help detect users of your application that may be violating OpenAI's usage policies.\nThe IDs should be a string that uniquely identifies each user, with a maximum length of 128 characters. We recommend hashing their username or email address, in order to avoid sending us any identifying information. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateChatCompletionRequestSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for CreateChatCompletionRequestSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateChatCompletionRequestSafetyIdentifier> for ::std::string::String {
    fn from(value: CreateChatCompletionRequestSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateChatCompletionRequestSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionRequestSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateChatCompletionRequestSafetyIdentifier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateChatCompletionRequestSafetyIdentifier {
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
#[doc = "`CreateChatCompletionRequestToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestToolsItem {
    ChatCompletionTool(ChatCompletionTool),
    CustomToolChatCompletions(CustomToolChatCompletions),
}
impl ::std::convert::From<ChatCompletionTool> for CreateChatCompletionRequestToolsItem {
    fn from(value: ChatCompletionTool) -> Self {
        Self::ChatCompletionTool(value)
    }
}
impl ::std::convert::From<CustomToolChatCompletions> for CreateChatCompletionRequestToolsItem {
    fn from(value: CustomToolChatCompletions) -> Self {
        Self::CustomToolChatCompletions(value)
    }
}
#[doc = "Represents a chat completion response returned by model, based on the provided input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionResponse {
    #[doc = "A list of chat completion choices. Can be more than one if `n` is greater than 1."]
    pub choices: ::std::vec::Vec<CreateChatCompletionResponseChoicesItem>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created."]
    pub created: i64,
    #[doc = "A unique identifier for the chat completion."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[doc = "The model used for the chat completion."]
    pub model: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ChatCompletionModeration>,
    #[doc = "The object type, which is always `chat.completion`."]
    pub object: CreateChatCompletionResponseObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\n\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CompletionUsage>,
}
#[doc = "`CreateChatCompletionResponseChoicesItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionResponseChoicesItem {
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\nRead the [Model Spec](https://model-spec.openai.com/2025-12-18.html) for more.\n"]
    pub finish_reason: CreateChatCompletionResponseChoicesItemFinishReason,
    #[doc = "The index of the choice in the list of choices."]
    pub index: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub logprobs: ::std::option::Option<CreateChatCompletionResponseChoicesItemLogprobs>,
    pub message: ChatCompletionResponseMessage,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\nRead the [Model Spec](https://model-spec.openai.com/2025-12-18.html) for more.\n"]
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
pub enum CreateChatCompletionResponseChoicesItemFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for CreateChatCompletionResponseChoicesItemFinishReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stop => f.write_str("stop"),
            Self::Length => f.write_str("length"),
            Self::ToolCalls => f.write_str("tool_calls"),
            Self::ContentFilter => f.write_str("content_filter"),
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionResponseChoicesItemFinishReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop" => Ok(Self::Stop),
            "length" => Ok(Self::Length),
            "tool_calls" => Ok(Self::ToolCalls),
            "content_filter" => Ok(Self::ContentFilter),
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionResponseChoicesItemFinishReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateChatCompletionResponseChoicesItemFinishReason
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Log probability information for the choice."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionResponseChoicesItemLogprobs {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content: ::std::option::Option<::std::vec::Vec<ChatCompletionTokenLogprob>>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub refusal: ::std::option::Option<::std::vec::Vec<ChatCompletionTokenLogprob>>,
}
#[doc = "The object type, which is always `chat.completion`."]
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
pub enum CreateChatCompletionResponseObject {
    #[serde(rename = "chat.completion")]
    ChatCompletion,
}
impl ::std::fmt::Display for CreateChatCompletionResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatCompletion => f.write_str("chat.completion"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chat.completion" => Ok(Self::ChatCompletion),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateChatCompletionResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a streamed chunk of a chat completion response returned\nby the model, based on the provided input.\n[Learn more](https://developers.openai.com/api/docs/guides/streaming-responses).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionStreamResponse {
    #[doc = "A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the\nlast chunk if you set `stream_options: {\"include_usage\": true}`.\n"]
    pub choices: ::std::vec::Vec<CreateChatCompletionStreamResponseChoicesItem>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp."]
    pub created: i64,
    #[doc = "A unique identifier for the chat completion. Each chunk has the same ID."]
    pub id: ::std::string::String,
    #[doc = "The model to generate the completion."]
    pub model: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ChatCompletionModeration>,
    #[doc = "An obfuscation string added to normalize the size of streamed chunks as a\nmitigation to certain side-channel attacks. The field is included by\ndefault and omitted when `stream_options.include_obfuscation` is `false`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub obfuscation: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `chat.completion.chunk`."]
    pub object: CreateChatCompletionStreamResponseObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTier>,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_fingerprint: ::std::option::Option<::std::string::String>,
    #[doc = "An optional field that will only be present when you set\n`stream_options: {\"include_usage\": true}` in your request. When present, it\ncontains a null value **except for the last chunk** which contains the\ntoken usage statistics for the entire request.\n\n**NOTE:** If the stream is interrupted or cancelled, you may not\nreceive the final usage chunk which contains the total token usage for\nthe request.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CompletionUsage>,
}
#[doc = "`CreateChatCompletionStreamResponseChoicesItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionStreamResponseChoicesItem {
    pub delta: ChatCompletionStreamResponseDelta,
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\nOmitted from the final audio update containing only `delta.audio.expires_at`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub finish_reason:
        ::std::option::Option<CreateChatCompletionStreamResponseChoicesItemFinishReason>,
    #[doc = "The index of the choice in the list of choices."]
    pub index: i64,
    #[doc = "Log probability information for the choice."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<CreateChatCompletionStreamResponseChoicesItemLogprobs>,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\nOmitted from the final audio update containing only `delta.audio.expires_at`.\n"]
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
pub enum CreateChatCompletionStreamResponseChoicesItemFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for CreateChatCompletionStreamResponseChoicesItemFinishReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stop => f.write_str("stop"),
            Self::Length => f.write_str("length"),
            Self::ToolCalls => f.write_str("tool_calls"),
            Self::ContentFilter => f.write_str("content_filter"),
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionStreamResponseChoicesItemFinishReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop" => Ok(Self::Stop),
            "length" => Ok(Self::Length),
            "tool_calls" => Ok(Self::ToolCalls),
            "content_filter" => Ok(Self::ContentFilter),
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionStreamResponseChoicesItemFinishReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateChatCompletionStreamResponseChoicesItemFinishReason
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Log probability information for the choice."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionStreamResponseChoicesItemLogprobs {
    #[doc = "A list of message content tokens with log probability information."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub content: ::std::option::Option<::std::vec::Vec<ChatCompletionTokenLogprob>>,
    #[doc = "A list of message refusal tokens with log probability information."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub refusal: ::std::option::Option<::std::vec::Vec<ChatCompletionTokenLogprob>>,
}
#[doc = "The object type, which is always `chat.completion.chunk`."]
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
pub enum CreateChatCompletionStreamResponseObject {
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}
impl ::std::fmt::Display for CreateChatCompletionStreamResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatCompletionChunk => f.write_str("chat.completion.chunk"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionStreamResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chat.completion.chunk" => Ok(Self::ChatCompletionChunk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionStreamResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateChatCompletionStreamResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateModelResponseProperties`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateModelResponseProperties {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_options: ::std::option::Option<PromptCacheOptionsParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention:
        ::std::option::Option<CreateModelResponsePropertiesPromptCacheRetention>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<CreateModelResponsePropertiesSafetyIdentifier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[doc = "This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead to maintain caching optimizations.\nA stable identifier for your end-users.\nUsed to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "Deprecated. Use `prompt_cache_options.ttl` instead.\n\nThe retention policy for the prompt cache. Set to `24h` to enable extended prompt caching, which keeps cached prefixes active for longer, up to a maximum of 24 hours. [Learn more](https://developers.openai.com/api/docs/guides/prompt-caching#prompt-cache-retention).\nThis field expresses a maximum retention policy, while\n`prompt_cache_options.ttl` expresses a minimum cache lifetime. The two\nfields are independent and do not interact.\nFor `gpt-5.5`, `gpt-5.5-pro`, and future models, only `24h` is supported.\n\nFor older models that support both `in_memory` and `24h`, the default depends on your organization's data retention policy:\n  - Organizations without ZDR enabled default to `24h`.\n  - Organizations with ZDR enabled default to `in_memory` when `prompt_cache_retention` is not specified.\n"]
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
pub enum CreateModelResponsePropertiesPromptCacheRetention {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for CreateModelResponsePropertiesPromptCacheRetention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for CreateModelResponsePropertiesPromptCacheRetention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateModelResponsePropertiesPromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModelResponsePropertiesPromptCacheRetention
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A stable identifier used to help detect users of your application that may be violating OpenAI's usage policies.\nThe IDs should be a string that uniquely identifies each user, with a maximum length of 128 characters. We recommend hashing their username or email address, in order to avoid sending us any identifying information. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateModelResponsePropertiesSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for CreateModelResponsePropertiesSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateModelResponsePropertiesSafetyIdentifier> for ::std::string::String {
    fn from(value: CreateModelResponsePropertiesSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateModelResponsePropertiesSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateModelResponsePropertiesSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateModelResponsePropertiesSafetyIdentifier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateModelResponsePropertiesSafetyIdentifier {
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
#[doc = "A custom tool that processes input using a specified format.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolChatCompletions {
    pub custom: CustomToolProperties,
    #[doc = "The type of the custom tool. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: CustomToolChatCompletionsType,
}
#[doc = "The type of the custom tool. Always `custom`."]
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
pub enum CustomToolChatCompletionsType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for CustomToolChatCompletionsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for CustomToolChatCompletionsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolChatCompletionsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolChatCompletionsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Properties of the custom tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolProperties {
    #[doc = "Optional description of the custom tool, used to provide more context.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The input format for the custom tool. Default is unconstrained text.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<CustomToolPropertiesFormat>,
    #[doc = "The name of the custom tool, used to identify it in tool calls."]
    pub name: ::std::string::String,
}
#[doc = "The input format for the custom tool. Default is unconstrained text.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", content = "grammar")]
pub enum CustomToolPropertiesFormat {
    #[serde(rename = "text")]
    Text,
    #[doc = "Grammar format\n\nA grammar defined by the user."]
    #[serde(rename = "grammar")]
    Grammar {
        #[doc = "The grammar definition."]
        definition: ::std::string::String,
        #[doc = "The syntax of the grammar definition. One of `lark` or `regex`."]
        syntax: GrammarFormatSyntax,
    },
}
#[doc = "`FunctionObject`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionObject {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<FunctionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
}
#[doc = "The parameters the functions accepts, described as a JSON Schema object. See the [guide](https://developers.openai.com/api/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.\n\nOmitting `parameters` defines a function with an empty parameter list."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct FunctionParameters(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FunctionParameters {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FunctionParameters>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FunctionParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FunctionParameters
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`GetChatCompletionMessagesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct GetChatCompletionMessagesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<GetChatCompletionMessagesParamsOrder>,
}
#[doc = "`GetChatCompletionMessagesParamsOrder`"]
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
pub enum GetChatCompletionMessagesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for GetChatCompletionMessagesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for GetChatCompletionMessagesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetChatCompletionMessagesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetChatCompletionMessagesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The syntax of the grammar definition. One of `lark` or `regex`."]
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
pub enum GrammarFormatSyntax {
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "regex")]
    Regex,
}
impl ::std::fmt::Display for GrammarFormatSyntax {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Lark => f.write_str("lark"),
            Self::Regex => f.write_str("regex"),
        }
    }
}
impl ::std::str::FromStr for GrammarFormatSyntax {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "lark" => Ok(Self::Lark),
            "regex" => Ok(Self::Regex),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GrammarFormatSyntax {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GrammarFormatSyntax {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Structured Outputs configuration options, including a JSON Schema.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct JsonSchema {
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub schema: ::std::option::Option<ResponseFormatJsonSchemaSchema>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
}
#[doc = "`ListChatCompletionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListChatCompletionsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListChatCompletionsParamsOrder>,
}
#[doc = "`ListChatCompletionsParamsOrder`"]
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
pub enum ListChatCompletionsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListChatCompletionsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListChatCompletionsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListChatCompletionsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListChatCompletionsParamsOrder {
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
#[doc = "`ModelIdsShared`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct ModelIdsShared(pub ::std::string::String);
impl ::std::ops::Deref for ModelIdsShared {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelIdsShared> for ::std::string::String {
    fn from(value: ModelIdsShared) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ModelIdsShared {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ModelIdsShared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ModelIdsShared {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "`ModelResponseProperties`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModelResponseProperties {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention: ::std::option::Option<ModelResponsePropertiesPromptCacheRetention>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<ModelResponsePropertiesSafetyIdentifier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[doc = "This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead to maintain caching optimizations.\nA stable identifier for your end-users.\nUsed to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "Deprecated. Use `prompt_cache_options.ttl` instead.\n\nThe retention policy for the prompt cache. Set to `24h` to enable extended prompt caching, which keeps cached prefixes active for longer, up to a maximum of 24 hours. [Learn more](https://developers.openai.com/api/docs/guides/prompt-caching#prompt-cache-retention).\nThis field expresses a maximum retention policy, while\n`prompt_cache_options.ttl` expresses a minimum cache lifetime. The two\nfields are independent and do not interact.\nFor `gpt-5.5`, `gpt-5.5-pro`, and future models, only `24h` is supported.\n\nFor older models that support both `in_memory` and `24h`, the default depends on your organization's data retention policy:\n  - Organizations without ZDR enabled default to `24h`.\n  - Organizations with ZDR enabled default to `in_memory` when `prompt_cache_retention` is not specified.\n"]
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
pub enum ModelResponsePropertiesPromptCacheRetention {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for ModelResponsePropertiesPromptCacheRetention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for ModelResponsePropertiesPromptCacheRetention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelResponsePropertiesPromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ModelResponsePropertiesPromptCacheRetention
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A stable identifier used to help detect users of your application that may be violating OpenAI's usage policies.\nThe IDs should be a string that uniquely identifies each user, with a maximum length of 128 characters. We recommend hashing their username or email address, in order to avoid sending us any identifying information. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ModelResponsePropertiesSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for ModelResponsePropertiesSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelResponsePropertiesSafetyIdentifier> for ::std::string::String {
    fn from(value: ModelResponsePropertiesSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ModelResponsePropertiesSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ModelResponsePropertiesSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelResponsePropertiesSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ModelResponsePropertiesSafetyIdentifier {
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
#[doc = "The moderation policy for the response input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationConfigParam {
    pub mode: ModerationMode,
}
#[doc = "`ModerationInputType`"]
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
pub enum ModerationInputType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for ModerationInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for ModerationInputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModerationInputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModerationInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ModerationMode`"]
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
pub enum ModerationMode {
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "block")]
    Block,
}
impl ::std::fmt::Display for ModerationMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => f.write_str("score"),
            Self::Block => f.write_str("block"),
        }
    }
}
impl ::std::str::FromStr for ModerationMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score" => Ok(Self::Score),
            "block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModerationMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModerationMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for running moderation on the input and output of this response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationParam {
    #[doc = "The moderation model to use for moderated completions, e.g. 'omni-moderation-latest'."]
    pub model: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<ModerationPolicyParam>,
}
#[doc = "The policy to apply to moderated response input and output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModerationPolicyParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<ModerationConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<ModerationConfigParam>,
}
#[doc = "A moderation result produced for the response input or output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationResultBody {
    #[doc = "A dictionary of moderation categories to booleans, True if the input is flagged under this category."]
    pub categories: ::std::collections::HashMap<::std::string::String, bool>,
    #[doc = "Which modalities of input are reflected by the score for each category."]
    pub category_applied_input_types:
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<ModerationInputType>>,
    #[doc = "A dictionary of moderation categories to scores."]
    pub category_scores: ::std::collections::HashMap<::std::string::String, f64>,
    #[doc = "A boolean indicating whether the content was flagged by any category."]
    pub flagged: bool,
    #[doc = "The moderation model that produced this result."]
    pub model: ::std::string::String,
    #[doc = "The object type, which was always `moderation_result` for successful moderation results."]
    #[serde(rename = "type")]
    pub type_: ModerationResultBodyType,
}
#[doc = "The object type, which was always `moderation_result` for successful moderation results."]
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
pub enum ModerationResultBodyType {
    #[serde(rename = "moderation_result")]
    ModerationResult,
}
impl ::std::fmt::Display for ModerationResultBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ModerationResult => f.write_str("moderation_result"),
        }
    }
}
impl ::std::str::FromStr for ModerationResultBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "moderation_result" => Ok(Self::ModerationResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModerationResultBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModerationResultBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Whether to enable [parallel function calling](https://developers.openai.com/api/docs/guides/function-calling#parallel-function-calling) during tool use."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ParallelToolCalls(pub bool);
impl ::std::ops::Deref for ParallelToolCalls {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<ParallelToolCalls> for bool {
    fn from(value: ParallelToolCalls) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for ParallelToolCalls {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ParallelToolCalls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ParallelToolCalls {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ParallelToolCalls {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ParallelToolCalls {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
#[doc = "Static predicted output content, such as the content of a text file that is\nbeing regenerated.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PredictionContent {
    #[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
    pub content: PredictionContentContent,
    #[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
    #[serde(rename = "type")]
    pub type_: PredictionContentType,
}
#[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PredictionContentContent {
    TextContent(::std::string::String),
    ArrayOfContentParts(::std::vec::Vec<ChatCompletionRequestMessageContentPartText>),
}
impl ::std::convert::From<::std::vec::Vec<ChatCompletionRequestMessageContentPartText>>
    for PredictionContentContent
{
    fn from(value: ::std::vec::Vec<ChatCompletionRequestMessageContentPartText>) -> Self {
        Self::ArrayOfContentParts(value)
    }
}
#[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
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
pub enum PredictionContentType {
    #[serde(rename = "content")]
    Content,
}
impl ::std::fmt::Display for PredictionContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Content => f.write_str("content"),
        }
    }
}
impl ::std::str::FromStr for PredictionContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "content" => Ok(Self::Content),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PredictionContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PredictionContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheBreakpointParam {
    #[doc = "The breakpoint mode. Always `explicit`."]
    pub mode: PromptCacheBreakpointParamMode,
}
#[doc = "The breakpoint mode. Always `explicit`."]
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
pub enum PromptCacheBreakpointParamMode {
    #[serde(rename = "explicit")]
    Explicit,
}
impl ::std::fmt::Display for PromptCacheBreakpointParamMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Explicit => f.write_str("explicit"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheBreakpointParamMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "explicit" => Ok(Self::Explicit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheBreakpointParamMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheBreakpointParamMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PromptCacheModeEnum`"]
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
pub enum PromptCacheModeEnum {
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "explicit")]
    Explicit,
}
impl ::std::fmt::Display for PromptCacheModeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Implicit => f.write_str("implicit"),
            Self::Explicit => f.write_str("explicit"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheModeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "implicit" => Ok(Self::Implicit),
            "explicit" => Ok(Self::Explicit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Options for prompt caching. Supported for `gpt-5.6` and later models. By default, OpenAI automatically chooses one implicit cache breakpoint. You can add explicit breakpoints to content blocks with `prompt_cache_breakpoint`. Each request can write up to four breakpoints. For cache matching, OpenAI considers up to the latest 80 breakpoints in the conversation, without a content-block lookback limit. Set `mode` to `explicit` to disable the implicit breakpoint. The `ttl` defaults to `30m`, which is currently the only supported value. See the [prompt caching guide](https://developers.openai.com/api/docs/guides/prompt-caching) for current details."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PromptCacheOptionsParam {
    #[doc = "Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`. With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request does not use prompt caching."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<PromptCacheModeEnum>,
    #[doc = "The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries for longer."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<PromptCacheTtlEnum>,
}
#[doc = "`PromptCacheTtlEnum`"]
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
pub enum PromptCacheTtlEnum {
    #[serde(rename = "30m")]
    X30m,
}
impl ::std::fmt::Display for PromptCacheTtlEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X30m => f.write_str("30m"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheTtlEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "30m" => Ok(Self::X30m),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheTtlEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheTtlEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReasoningEffort`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ReasoningEffort(pub ::std::option::Option<ReasoningEffortValue>);
impl ::std::ops::Deref for ReasoningEffort {
    type Target = ::std::option::Option<ReasoningEffortValue>;
    fn deref(&self) -> &::std::option::Option<ReasoningEffortValue> {
        &self.0
    }
}
impl ::std::convert::From<ReasoningEffort> for ::std::option::Option<ReasoningEffortValue> {
    fn from(value: ReasoningEffort) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ReasoningEffortValue>> for ReasoningEffort {
    fn from(value: ::std::option::Option<ReasoningEffortValue>) -> Self {
        Self(value)
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
#[doc = "JSON object response format. An older method of generating JSON responses.\nUsing `json_schema` is recommended for models that support it. Note that the\nmodel will not generate JSON without a system or user message instructing it\nto do so.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFormatJsonObject {
    #[doc = "The type of response format being defined. Always `json_object`."]
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonObjectType,
}
#[doc = "The type of response format being defined. Always `json_object`."]
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
pub enum ResponseFormatJsonObjectType {
    #[serde(rename = "json_object")]
    JsonObject,
}
impl ::std::fmt::Display for ResponseFormatJsonObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonObject => f.write_str("json_object"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatJsonObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json_object" => Ok(Self::JsonObject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatJsonObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatJsonObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFormatJsonSchema {
    pub json_schema: JsonSchema,
    #[doc = "The type of response format being defined. Always `json_schema`."]
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonSchemaType,
}
#[doc = "The schema for the response format, described as a JSON Schema object.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponseFormatJsonSchemaSchema(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ResponseFormatJsonSchemaSchema {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ResponseFormatJsonSchemaSchema>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ResponseFormatJsonSchemaSchema) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ResponseFormatJsonSchemaSchema
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "The type of response format being defined. Always `json_schema`."]
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
pub enum ResponseFormatJsonSchemaType {
    #[serde(rename = "json_schema")]
    JsonSchema,
}
impl ::std::fmt::Display for ResponseFormatJsonSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonSchema => f.write_str("json_schema"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatJsonSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json_schema" => Ok(Self::JsonSchema),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Default response format. Used to generate text responses.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFormatText {
    #[doc = "The type of response format being defined. Always `text`."]
    #[serde(rename = "type")]
    pub type_: ResponseFormatTextType,
}
#[doc = "The type of response format being defined. Always `text`."]
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
pub enum ResponseFormatTextType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for ResponseFormatTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponseModalities`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponseModalities(pub ::std::option::Option<ResponseModalitiesValue>);
impl ::std::ops::Deref for ResponseModalities {
    type Target = ::std::option::Option<ResponseModalitiesValue>;
    fn deref(&self) -> &::std::option::Option<ResponseModalitiesValue> {
        &self.0
    }
}
impl ::std::convert::From<ResponseModalities> for ::std::option::Option<ResponseModalitiesValue> {
    fn from(value: ResponseModalities) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ResponseModalitiesValue>> for ResponseModalities {
    fn from(value: ::std::option::Option<ResponseModalitiesValue>) -> Self {
        Self(value)
    }
}
#[doc = "Output types that you would like the model to generate.\nMost models are capable of generating text, which is the default:\n\n`[\"text\"]`\n\nThe `gpt-4o-audio-preview` model can also be used to\n[generate audio](https://developers.openai.com/api/docs/guides/audio). To request that this model generate\nboth text and audio responses, you can use:\n\n`[\"text\", \"audio\"]`\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponseModalitiesValue(pub ::std::vec::Vec<ResponseModalitiesValueItem>);
impl ::std::ops::Deref for ResponseModalitiesValue {
    type Target = ::std::vec::Vec<ResponseModalitiesValueItem>;
    fn deref(&self) -> &::std::vec::Vec<ResponseModalitiesValueItem> {
        &self.0
    }
}
impl ::std::convert::From<ResponseModalitiesValue>
    for ::std::vec::Vec<ResponseModalitiesValueItem>
{
    fn from(value: ResponseModalitiesValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ResponseModalitiesValueItem>>
    for ResponseModalitiesValue
{
    fn from(value: ::std::vec::Vec<ResponseModalitiesValueItem>) -> Self {
        Self(value)
    }
}
#[doc = "`ResponseModalitiesValueItem`"]
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
pub enum ResponseModalitiesValueItem {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
impl ::std::fmt::Display for ResponseModalitiesValueItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Audio => f.write_str("audio"),
        }
    }
}
impl ::std::str::FromStr for ResponseModalitiesValueItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseModalitiesValueItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseModalitiesValueItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ServiceTier`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ServiceTier(pub ::std::option::Option<ServiceTierValue>);
impl ::std::ops::Deref for ServiceTier {
    type Target = ::std::option::Option<ServiceTierValue>;
    fn deref(&self) -> &::std::option::Option<ServiceTierValue> {
        &self.0
    }
}
impl ::std::convert::From<ServiceTier> for ::std::option::Option<ServiceTierValue> {
    fn from(value: ServiceTier) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ServiceTierValue>> for ServiceTier {
    fn from(value: ::std::option::Option<ServiceTierValue>) -> Self {
        Self(value)
    }
}
#[doc = "Specifies the processing type used for serving the request.\n  - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.\n  - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.\n  - If set to '[flex](https://developers.openai.com/api/docs/guides/flex-processing)', then the request will be processed with the Flex Processing service tier.\n  - To opt-in to [Fast mode](https://developers.openai.com/api/docs/guides/fast-mode) at the request level, include the `service_tier=fast` or `service_tier=priority` parameter for Responses or Chat Completions. The response will show `service_tier=priority` regardless of if you specify `service_tier=fast` or `priority` in your request.\n  - When not set, the default behavior is 'auto'.\n\n  When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.\n"]
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
pub enum ServiceTierValue {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "flex")]
    Flex,
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "fast")]
    Fast,
}
impl ::std::fmt::Display for ServiceTierValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::Flex => f.write_str("flex"),
            Self::Scale => f.write_str("scale"),
            Self::Priority => f.write_str("priority"),
            Self::Fast => f.write_str("fast"),
        }
    }
}
impl ::std::str::FromStr for ServiceTierValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "flex" => Ok(Self::Flex),
            "scale" => Ok(Self::Scale),
            "priority" => Ok(Self::Priority),
            "fast" => Ok(Self::Fast),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ServiceTierValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ServiceTierValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Not supported with latest reasoning models `o3` and `o4-mini`.\n\nUp to 4 sequences where the API will stop generating further tokens. The\nreturned text will not contain the stop sequence.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum StopConfiguration {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
    Null,
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for StopConfiguration {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.\n"]
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
pub enum ToolChoiceMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ToolChoiceMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceMode {
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
impl ::std::convert::TryFrom<&str> for ToolChoiceMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateChatCompletionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateChatCompletionRequest {
    pub metadata: Metadata,
}
#[doc = "`Verbosity`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Verbosity(pub ::std::option::Option<VerbosityValue>);
impl ::std::ops::Deref for Verbosity {
    type Target = ::std::option::Option<VerbosityValue>;
    fn deref(&self) -> &::std::option::Option<VerbosityValue> {
        &self.0
    }
}
impl ::std::convert::From<Verbosity> for ::std::option::Option<VerbosityValue> {
    fn from(value: Verbosity) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<VerbosityValue>> for Verbosity {
    fn from(value: ::std::option::Option<VerbosityValue>) -> Self {
        Self(value)
    }
}
#[doc = "Constrains the verbosity of the model's response. Lower values will result in\nmore concise responses, while higher values will result in more verbose responses.\nCurrently supported values are `low`, `medium`, and `high`. The default is\n`medium`.\n"]
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
pub enum VerbosityValue {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for VerbosityValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for VerbosityValue {
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
impl ::std::convert::TryFrom<&str> for VerbosityValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerbosityValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A built-in voice name or a custom voice reference.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum VoiceIdsOrCustomVoice {
    VoiceIdsShared(VoiceIdsShared),
    Object {
        #[doc = "The custom voice ID, e.g. `voice_1234`."]
        id: ::std::string::String,
    },
}
impl ::std::convert::From<VoiceIdsShared> for VoiceIdsOrCustomVoice {
    fn from(value: VoiceIdsShared) -> Self {
        Self::VoiceIdsShared(value)
    }
}
#[doc = "`VoiceIdsShared`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct VoiceIdsShared(pub ::std::string::String);
impl ::std::ops::Deref for VoiceIdsShared {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceIdsShared> for ::std::string::String {
    fn from(value: VoiceIdsShared) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for VoiceIdsShared {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for VoiceIdsShared {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for VoiceIdsShared {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "This tool searches the web for relevant results to use in a response.\nLearn more about the [web search tool](https://developers.openai.com/api/docs/guides/tools-web-search).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct WebSearch {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub search_context_size: ::std::option::Option<WebSearchContextSize>,
    #[doc = "Approximate location parameters for the search.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_location: ::std::option::Option<WebSearchUserLocation>,
}
#[doc = "High level guidance for the amount of context window space to use for the \nsearch. One of `low`, `medium`, or `high`. `medium` is the default.\n"]
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
pub enum WebSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for WebSearchContextSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for WebSearchContextSize {
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
impl ::std::convert::TryFrom<&str> for WebSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Approximate location parameters for the search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct WebSearchLocation {
    #[doc = "Free text input for the city of the user, e.g. `San Francisco`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[doc = "The two-letter \n[ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,\ne.g. `US`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[doc = "Free text input for the region of the user, e.g. `California`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
    #[doc = "The [IANA timezone](https://timeapi.io/documentation/iana-timezones) \nof the user, e.g. `America/Los_Angeles`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
#[doc = "Approximate location parameters for the search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchUserLocation {
    pub approximate: WebSearchLocation,
    #[doc = "The type of location approximation. Always `approximate`.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchUserLocationType,
}
#[doc = "The type of location approximation. Always `approximate`.\n"]
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
pub enum WebSearchUserLocationType {
    #[serde(rename = "approximate")]
    Approximate,
}
impl ::std::fmt::Display for WebSearchUserLocationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Approximate => f.write_str("approximate"),
        }
    }
}
impl ::std::str::FromStr for WebSearchUserLocationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "approximate" => Ok(Self::Approximate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchUserLocationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchUserLocationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
