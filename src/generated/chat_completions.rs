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
impl ChatApiSchema {
    pub fn builder() -> builder::ChatApiSchema {
        Default::default()
    }
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionAllowedTools {
    #[doc = "Constrains the tools available to the model to a pre-defined set.\n\n`auto` allows the model to pick from among the allowed tools and generate a\nmessage.\n\n`required` requires the model to call one or more of the allowed tools.\n"]
    pub mode: ChatCompletionAllowedToolsMode,
    #[doc = "A list of tool definitions that the model should be allowed to call.\n\nFor the Chat Completions API, the list of tool definitions might look like:\n```json\n[\n  { \"type\": \"function\", \"function\": { \"name\": \"get_weather\" } },\n  { \"type\": \"function\", \"function\": { \"name\": \"get_time\" } }\n]\n```\n"]
    pub tools: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
}
impl ChatCompletionAllowedTools {
    pub fn builder() -> builder::ChatCompletionAllowedTools {
        Default::default()
    }
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionAllowedToolsChoice {
    pub allowed_tools: ChatCompletionAllowedTools,
    #[doc = "Allowed tool configuration type. Always `allowed_tools`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionAllowedToolsChoiceType,
}
impl ChatCompletionAllowedToolsChoice {
    pub fn builder() -> builder::ChatCompletionAllowedToolsChoice {
        Default::default()
    }
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
impl ChatCompletionDeleted {
    pub fn builder() -> builder::ChatCompletionDeleted {
        Default::default()
    }
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
impl ChatCompletionFunctionCallOption {
    pub fn builder() -> builder::ChatCompletionFunctionCallOption {
        Default::default()
    }
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
impl ChatCompletionFunctions {
    pub fn builder() -> builder::ChatCompletionFunctions {
        Default::default()
    }
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
impl ChatCompletionList {
    pub fn builder() -> builder::ChatCompletionList {
        Default::default()
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
impl ChatCompletionMessageCustomToolCall {
    pub fn builder() -> builder::ChatCompletionMessageCustomToolCall {
        Default::default()
    }
}
#[doc = "The custom tool that the model called."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageCustomToolCallCustom {
    #[doc = "The input for the custom tool call generated by the model."]
    pub input: ::std::string::String,
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
}
impl ChatCompletionMessageCustomToolCallCustom {
    pub fn builder() -> builder::ChatCompletionMessageCustomToolCallCustom {
        Default::default()
    }
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
impl ChatCompletionMessageList {
    pub fn builder() -> builder::ChatCompletionMessageList {
        Default::default()
    }
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
impl ChatCompletionMessageListDataItem {
    pub fn builder() -> builder::ChatCompletionMessageListDataItem {
        Default::default()
    }
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
impl ChatCompletionMessageListDataItemContentPartsItem {
    pub fn builder() -> builder::ChatCompletionMessageListDataItemContentPartsItem {
        Default::default()
    }
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
impl ChatCompletionMessageListDataItemContentPartsItemFile {
    pub fn builder() -> builder::ChatCompletionMessageListDataItemContentPartsItemFile {
        Default::default()
    }
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemImageUrl`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItemImageUrl {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
}
impl ChatCompletionMessageListDataItemContentPartsItemImageUrl {
    pub fn builder() -> builder::ChatCompletionMessageListDataItemContentPartsItemImageUrl {
        Default::default()
    }
}
#[doc = "`ChatCompletionMessageListDataItemContentPartsItemInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionMessageListDataItemContentPartsItemInputAudio {
    pub data: ::std::string::String,
    pub format: ::std::string::String,
}
impl ChatCompletionMessageListDataItemContentPartsItemInputAudio {
    pub fn builder() -> builder::ChatCompletionMessageListDataItemContentPartsItemInputAudio {
        Default::default()
    }
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
impl ChatCompletionMessageToolCall {
    pub fn builder() -> builder::ChatCompletionMessageToolCall {
        Default::default()
    }
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
impl ChatCompletionMessageToolCallChunk {
    pub fn builder() -> builder::ChatCompletionMessageToolCallChunk {
        Default::default()
    }
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
impl ChatCompletionMessageToolCallChunkFunction {
    pub fn builder() -> builder::ChatCompletionMessageToolCallChunkFunction {
        Default::default()
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
impl ChatCompletionMessageToolCallFunction {
    pub fn builder() -> builder::ChatCompletionMessageToolCallFunction {
        Default::default()
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
impl ChatCompletionModeration {
    pub fn builder() -> builder::ChatCompletionModeration {
        Default::default()
    }
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
impl ChatCompletionModerationError {
    pub fn builder() -> builder::ChatCompletionModerationError {
        Default::default()
    }
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
impl ChatCompletionModerationResults {
    pub fn builder() -> builder::ChatCompletionModerationResults {
        Default::default()
    }
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
impl ChatCompletionNamedToolChoice {
    pub fn builder() -> builder::ChatCompletionNamedToolChoice {
        Default::default()
    }
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific custom tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoiceCustom {
    pub custom: ChatCompletionNamedToolChoiceCustomCustom,
    #[doc = "For custom tool calling, the type is always `custom`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceCustomType,
}
impl ChatCompletionNamedToolChoiceCustom {
    pub fn builder() -> builder::ChatCompletionNamedToolChoiceCustom {
        Default::default()
    }
}
#[doc = "`ChatCompletionNamedToolChoiceCustomCustom`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionNamedToolChoiceCustomCustom {
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
}
impl ChatCompletionNamedToolChoiceCustomCustom {
    pub fn builder() -> builder::ChatCompletionNamedToolChoiceCustomCustom {
        Default::default()
    }
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
impl ChatCompletionNamedToolChoiceFunction {
    pub fn builder() -> builder::ChatCompletionNamedToolChoiceFunction {
        Default::default()
    }
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
impl ChatCompletionRequestAssistantMessage {
    pub fn builder() -> builder::ChatCompletionRequestAssistantMessage {
        Default::default()
    }
}
#[doc = "Data about a previous audio response from the model.\n[Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestAssistantMessageAudio {
    #[doc = "Unique identifier for a previous audio response from the model.\n"]
    pub id: ::std::string::String,
}
impl ChatCompletionRequestAssistantMessageAudio {
    pub fn builder() -> builder::ChatCompletionRequestAssistantMessageAudio {
        Default::default()
    }
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
impl ChatCompletionRequestAssistantMessageFunctionCall {
    pub fn builder() -> builder::ChatCompletionRequestAssistantMessageFunctionCall {
        Default::default()
    }
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
impl ChatCompletionRequestDeveloperMessage {
    pub fn builder() -> builder::ChatCompletionRequestDeveloperMessage {
        Default::default()
    }
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
impl ChatCompletionRequestFunctionMessage {
    pub fn builder() -> builder::ChatCompletionRequestFunctionMessage {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartAudio {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartAudio {
        Default::default()
    }
}
#[doc = "`ChatCompletionRequestMessageContentPartAudioInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
    #[doc = "Base64 encoded audio data."]
    pub data: ::std::string::String,
    #[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
    pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}
impl ChatCompletionRequestMessageContentPartAudioInputAudio {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartAudioInputAudio {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartFile {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartFile {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartFileFile {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartFileFile {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartImage {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartImage {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartImageImageUrl {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartImageImageUrl {
        Default::default()
    }
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
impl ChatCompletionRequestMessageContentPartRefusal {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartRefusal {
        Default::default()
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
impl ChatCompletionRequestMessageContentPartText {
    pub fn builder() -> builder::ChatCompletionRequestMessageContentPartText {
        Default::default()
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
impl ChatCompletionRequestSystemMessage {
    pub fn builder() -> builder::ChatCompletionRequestSystemMessage {
        Default::default()
    }
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
impl ChatCompletionRequestToolMessage {
    pub fn builder() -> builder::ChatCompletionRequestToolMessage {
        Default::default()
    }
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
impl ChatCompletionRequestUserMessage {
    pub fn builder() -> builder::ChatCompletionRequestUserMessage {
        Default::default()
    }
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
impl ChatCompletionResponseMessage {
    pub fn builder() -> builder::ChatCompletionResponseMessage {
        Default::default()
    }
}
#[doc = "A URL citation when using web search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageAnnotationsItem {
    #[doc = "The type of the URL citation. Always `url_citation`."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseMessageAnnotationsItemType,
    pub url_citation: ChatCompletionResponseMessageAnnotationsItemUrlCitation,
}
impl ChatCompletionResponseMessageAnnotationsItem {
    pub fn builder() -> builder::ChatCompletionResponseMessageAnnotationsItem {
        Default::default()
    }
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
impl ChatCompletionResponseMessageAnnotationsItemUrlCitation {
    pub fn builder() -> builder::ChatCompletionResponseMessageAnnotationsItemUrlCitation {
        Default::default()
    }
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
impl ChatCompletionResponseMessageAudio {
    pub fn builder() -> builder::ChatCompletionResponseMessageAudio {
        Default::default()
    }
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionResponseMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: ::std::string::String,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
}
impl ChatCompletionResponseMessageFunctionCall {
    pub fn builder() -> builder::ChatCompletionResponseMessageFunctionCall {
        Default::default()
    }
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
impl ChatCompletionStreamOptionsValue {
    pub fn builder() -> builder::ChatCompletionStreamOptionsValue {
        Default::default()
    }
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
impl ChatCompletionStreamResponseDelta {
    pub fn builder() -> builder::ChatCompletionStreamResponseDelta {
        Default::default()
    }
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
impl ChatCompletionStreamResponseDeltaAudio {
    pub fn builder() -> builder::ChatCompletionStreamResponseDeltaAudio {
        Default::default()
    }
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
impl ChatCompletionStreamResponseDeltaFunctionCall {
    pub fn builder() -> builder::ChatCompletionStreamResponseDeltaFunctionCall {
        Default::default()
    }
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
impl ChatCompletionTokenLogprob {
    pub fn builder() -> builder::ChatCompletionTokenLogprob {
        Default::default()
    }
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
impl ChatCompletionTokenLogprobTopLogprobsItem {
    pub fn builder() -> builder::ChatCompletionTokenLogprobTopLogprobsItem {
        Default::default()
    }
}
#[doc = "A function tool that can be used to generate a response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatCompletionTool {
    pub function: FunctionObject,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
}
impl ChatCompletionTool {
    pub fn builder() -> builder::ChatCompletionTool {
        Default::default()
    }
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
impl CompletionUsage {
    pub fn builder() -> builder::CompletionUsage {
        Default::default()
    }
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
impl CompletionUsageCompletionTokensDetails {
    pub fn builder() -> builder::CompletionUsageCompletionTokensDetails {
        Default::default()
    }
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
impl CompletionUsagePromptTokensDetails {
    pub fn builder() -> builder::CompletionUsagePromptTokensDetails {
        Default::default()
    }
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
impl CreateChatCompletionRequest {
    pub fn builder() -> builder::CreateChatCompletionRequest {
        Default::default()
    }
}
#[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](https://developers.openai.com/api/docs/guides/audio).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatCompletionRequestAudio {
    #[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
    pub format: CreateChatCompletionRequestAudioFormat,
    #[doc = "The voice the model uses to respond. Supported built-in voices are\n`alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`,\n`sage`, `shimmer`, `marin`, and `cedar`. You may also provide a\ncustom voice object with an `id`, for example `{ \"id\": \"voice_1234\" }`.\nCustom voices must be created from audio samples.\n"]
    pub voice: VoiceIdsOrCustomVoice,
}
impl CreateChatCompletionRequestAudio {
    pub fn builder() -> builder::CreateChatCompletionRequestAudio {
        Default::default()
    }
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
impl CreateChatCompletionResponse {
    pub fn builder() -> builder::CreateChatCompletionResponse {
        Default::default()
    }
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
impl CreateChatCompletionResponseChoicesItem {
    pub fn builder() -> builder::CreateChatCompletionResponseChoicesItem {
        Default::default()
    }
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
impl CreateChatCompletionResponseChoicesItemLogprobs {
    pub fn builder() -> builder::CreateChatCompletionResponseChoicesItemLogprobs {
        Default::default()
    }
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
impl CreateChatCompletionStreamResponse {
    pub fn builder() -> builder::CreateChatCompletionStreamResponse {
        Default::default()
    }
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
impl CreateChatCompletionStreamResponseChoicesItem {
    pub fn builder() -> builder::CreateChatCompletionStreamResponseChoicesItem {
        Default::default()
    }
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
impl CreateChatCompletionStreamResponseChoicesItemLogprobs {
    pub fn builder() -> builder::CreateChatCompletionStreamResponseChoicesItemLogprobs {
        Default::default()
    }
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
impl CreateModelResponseProperties {
    pub fn builder() -> builder::CreateModelResponseProperties {
        Default::default()
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
impl CustomToolChatCompletions {
    pub fn builder() -> builder::CustomToolChatCompletions {
        Default::default()
    }
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
impl CustomToolProperties {
    pub fn builder() -> builder::CustomToolProperties {
        Default::default()
    }
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
impl FunctionObject {
    pub fn builder() -> builder::FunctionObject {
        Default::default()
    }
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
impl GetChatCompletionMessagesParams {
    pub fn builder() -> builder::GetChatCompletionMessagesParams {
        Default::default()
    }
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
impl JsonSchema {
    pub fn builder() -> builder::JsonSchema {
        Default::default()
    }
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
impl ListChatCompletionsParams {
    pub fn builder() -> builder::ListChatCompletionsParams {
        Default::default()
    }
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
impl ModelResponseProperties {
    pub fn builder() -> builder::ModelResponseProperties {
        Default::default()
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
impl ModerationConfigParam {
    pub fn builder() -> builder::ModerationConfigParam {
        Default::default()
    }
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
impl ModerationParam {
    pub fn builder() -> builder::ModerationParam {
        Default::default()
    }
}
#[doc = "The policy to apply to moderated response input and output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModerationPolicyParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<ModerationConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<ModerationConfigParam>,
}
impl ModerationPolicyParam {
    pub fn builder() -> builder::ModerationPolicyParam {
        Default::default()
    }
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
impl ModerationResultBody {
    pub fn builder() -> builder::ModerationResultBody {
        Default::default()
    }
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
impl PredictionContent {
    pub fn builder() -> builder::PredictionContent {
        Default::default()
    }
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
impl PromptCacheBreakpointParam {
    pub fn builder() -> builder::PromptCacheBreakpointParam {
        Default::default()
    }
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
impl PromptCacheOptionsParam {
    pub fn builder() -> builder::PromptCacheOptionsParam {
        Default::default()
    }
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
impl ResponseFormatJsonObject {
    pub fn builder() -> builder::ResponseFormatJsonObject {
        Default::default()
    }
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
impl ResponseFormatJsonSchema {
    pub fn builder() -> builder::ResponseFormatJsonSchema {
        Default::default()
    }
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
impl ResponseFormatText {
    pub fn builder() -> builder::ResponseFormatText {
        Default::default()
    }
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
impl UpdateChatCompletionRequest {
    pub fn builder() -> builder::UpdateChatCompletionRequest {
        Default::default()
    }
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
impl WebSearch {
    pub fn builder() -> builder::WebSearch {
        Default::default()
    }
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
impl WebSearchLocation {
    pub fn builder() -> builder::WebSearchLocation {
        Default::default()
    }
}
#[doc = "Approximate location parameters for the search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchUserLocation {
    pub approximate: WebSearchLocation,
    #[doc = "The type of location approximation. Always `approximate`.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchUserLocationType,
}
impl WebSearchUserLocation {
    pub fn builder() -> builder::WebSearchUserLocation {
        Default::default()
    }
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ChatApiSchema {
        create_chat_completion_request:
            ::std::result::Result<super::CreateChatCompletionRequest, ::std::string::String>,
        create_chat_completion_response:
            ::std::result::Result<super::CreateChatCompletionResponse, ::std::string::String>,
        create_chat_completion_stream_response:
            ::std::result::Result<super::CreateChatCompletionStreamResponse, ::std::string::String>,
        delete_chat_completion_response:
            ::std::result::Result<super::ChatCompletionDeleted, ::std::string::String>,
        get_chat_completion_messages_params:
            ::std::result::Result<super::GetChatCompletionMessagesParams, ::std::string::String>,
        get_chat_completion_messages_response:
            ::std::result::Result<super::ChatCompletionMessageList, ::std::string::String>,
        get_chat_completion_response:
            ::std::result::Result<super::CreateChatCompletionResponse, ::std::string::String>,
        list_chat_completions_params:
            ::std::result::Result<super::ListChatCompletionsParams, ::std::string::String>,
        list_chat_completions_response:
            ::std::result::Result<super::ChatCompletionList, ::std::string::String>,
        update_chat_completion_request:
            ::std::result::Result<super::UpdateChatCompletionRequest, ::std::string::String>,
        update_chat_completion_response:
            ::std::result::Result<super::CreateChatCompletionResponse, ::std::string::String>,
    }
    impl ::std::default::Default for ChatApiSchema {
        fn default() -> Self {
            Self {
                create_chat_completion_request: Err(
                    "no value supplied for create_chat_completion_request".to_string(),
                ),
                create_chat_completion_response: Err(
                    "no value supplied for create_chat_completion_response".to_string(),
                ),
                create_chat_completion_stream_response: Err(
                    "no value supplied for create_chat_completion_stream_response".to_string(),
                ),
                delete_chat_completion_response: Err(
                    "no value supplied for delete_chat_completion_response".to_string(),
                ),
                get_chat_completion_messages_params: Err(
                    "no value supplied for get_chat_completion_messages_params".to_string(),
                ),
                get_chat_completion_messages_response: Err(
                    "no value supplied for get_chat_completion_messages_response".to_string(),
                ),
                get_chat_completion_response: Err(
                    "no value supplied for get_chat_completion_response".to_string(),
                ),
                list_chat_completions_params: Err(
                    "no value supplied for list_chat_completions_params".to_string(),
                ),
                list_chat_completions_response: Err(
                    "no value supplied for list_chat_completions_response".to_string(),
                ),
                update_chat_completion_request: Err(
                    "no value supplied for update_chat_completion_request".to_string(),
                ),
                update_chat_completion_response: Err(
                    "no value supplied for update_chat_completion_response".to_string(),
                ),
            }
        }
    }
    impl ChatApiSchema {
        pub fn create_chat_completion_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_completion_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_chat_completion_request: {e}")
            });
            self
        }
        pub fn create_chat_completion_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_completion_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_chat_completion_response: {e}")
            });
            self
        }
        pub fn create_chat_completion_stream_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionStreamResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_chat_completion_stream_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_chat_completion_stream_response: {e}")) ;
            self
        }
        pub fn delete_chat_completion_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionDeleted>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_chat_completion_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_chat_completion_response: {e}")
            });
            self
        }
        pub fn get_chat_completion_messages_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetChatCompletionMessagesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_chat_completion_messages_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_chat_completion_messages_params: {e}"
                )
            });
            self
        }
        pub fn get_chat_completion_messages_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionMessageList>,
            T::Error: ::std::fmt::Display,
        {
            self . get_chat_completion_messages_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for get_chat_completion_messages_response: {e}")) ;
            self
        }
        pub fn get_chat_completion_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_chat_completion_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_chat_completion_response: {e}")
            });
            self
        }
        pub fn list_chat_completions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListChatCompletionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_chat_completions_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_chat_completions_params: {e}")
            });
            self
        }
        pub fn list_chat_completions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionList>,
            T::Error: ::std::fmt::Display,
        {
            self.list_chat_completions_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_chat_completions_response: {e}")
            });
            self
        }
        pub fn update_chat_completion_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateChatCompletionRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.update_chat_completion_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_chat_completion_request: {e}")
            });
            self
        }
        pub fn update_chat_completion_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_chat_completion_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_chat_completion_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ChatApiSchema> for super::ChatApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_chat_completion_request: value.create_chat_completion_request?,
                create_chat_completion_response: value.create_chat_completion_response?,
                create_chat_completion_stream_response: value
                    .create_chat_completion_stream_response?,
                delete_chat_completion_response: value.delete_chat_completion_response?,
                get_chat_completion_messages_params: value.get_chat_completion_messages_params?,
                get_chat_completion_messages_response: value
                    .get_chat_completion_messages_response?,
                get_chat_completion_response: value.get_chat_completion_response?,
                list_chat_completions_params: value.list_chat_completions_params?,
                list_chat_completions_response: value.list_chat_completions_response?,
                update_chat_completion_request: value.update_chat_completion_request?,
                update_chat_completion_response: value.update_chat_completion_response?,
            })
        }
    }
    impl ::std::convert::From<super::ChatApiSchema> for ChatApiSchema {
        fn from(value: super::ChatApiSchema) -> Self {
            Self {
                create_chat_completion_request: Ok(value.create_chat_completion_request),
                create_chat_completion_response: Ok(value.create_chat_completion_response),
                create_chat_completion_stream_response: Ok(
                    value.create_chat_completion_stream_response
                ),
                delete_chat_completion_response: Ok(value.delete_chat_completion_response),
                get_chat_completion_messages_params: Ok(value.get_chat_completion_messages_params),
                get_chat_completion_messages_response: Ok(
                    value.get_chat_completion_messages_response
                ),
                get_chat_completion_response: Ok(value.get_chat_completion_response),
                list_chat_completions_params: Ok(value.list_chat_completions_params),
                list_chat_completions_response: Ok(value.list_chat_completions_response),
                update_chat_completion_request: Ok(value.update_chat_completion_request),
                update_chat_completion_response: Ok(value.update_chat_completion_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionAllowedTools {
        mode: ::std::result::Result<super::ChatCompletionAllowedToolsMode, ::std::string::String>,
        tools: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionAllowedTools {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
                tools: Err("no value supplied for tools".to_string()),
            }
        }
    }
    impl ChatCompletionAllowedTools {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionAllowedToolsMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionAllowedTools> for super::ChatCompletionAllowedTools {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionAllowedTools,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionAllowedTools> for ChatCompletionAllowedTools {
        fn from(value: super::ChatCompletionAllowedTools) -> Self {
            Self {
                mode: Ok(value.mode),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionAllowedToolsChoice {
        allowed_tools:
            ::std::result::Result<super::ChatCompletionAllowedTools, ::std::string::String>,
        type_: ::std::result::Result<
            super::ChatCompletionAllowedToolsChoiceType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionAllowedToolsChoice {
        fn default() -> Self {
            Self {
                allowed_tools: Err("no value supplied for allowed_tools".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionAllowedToolsChoice {
        pub fn allowed_tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionAllowedTools>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_tools: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionAllowedToolsChoiceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionAllowedToolsChoice>
        for super::ChatCompletionAllowedToolsChoice
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionAllowedToolsChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_tools: value.allowed_tools?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionAllowedToolsChoice>
        for ChatCompletionAllowedToolsChoice
    {
        fn from(value: super::ChatCompletionAllowedToolsChoice) -> Self {
            Self {
                allowed_tools: Ok(value.allowed_tools),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionDeleted {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ChatCompletionDeletedObject, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionDeleted {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ChatCompletionDeleted {
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
            T: ::std::convert::TryInto<super::ChatCompletionDeletedObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionDeleted> for super::ChatCompletionDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionDeleted> for ChatCompletionDeleted {
        fn from(value: super::ChatCompletionDeleted) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionFunctionCallOption {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionFunctionCallOption {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionFunctionCallOption {
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
    impl ::std::convert::TryFrom<ChatCompletionFunctionCallOption>
        for super::ChatCompletionFunctionCallOption
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionFunctionCallOption,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ChatCompletionFunctionCallOption>
        for ChatCompletionFunctionCallOption
    {
        fn from(value: super::ChatCompletionFunctionCallOption) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionFunctions {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parameters: ::std::result::Result<
            ::std::option::Option<super::FunctionParameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionFunctions {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                parameters: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionFunctions {
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
        pub fn parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FunctionParameters>>,
            T::Error: ::std::fmt::Display,
        {
            self.parameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parameters: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionFunctions> for super::ChatCompletionFunctions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionFunctions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                parameters: value.parameters?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionFunctions> for ChatCompletionFunctions {
        fn from(value: super::ChatCompletionFunctions) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                parameters: Ok(value.parameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionList {
        data: ::std::result::Result<
            ::std::vec::Vec<super::CreateChatCompletionResponse>,
            ::std::string::String,
        >,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ChatCompletionListObject, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionList {
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
    impl ChatCompletionList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateChatCompletionResponse>>,
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
            T: ::std::convert::TryInto<super::ChatCompletionListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionList> for super::ChatCompletionList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionList,
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
    impl ::std::convert::From<super::ChatCompletionList> for ChatCompletionList {
        fn from(value: super::ChatCompletionList) -> Self {
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
    pub struct ChatCompletionMessageCustomToolCall {
        custom: ::std::result::Result<
            super::ChatCompletionMessageCustomToolCallCustom,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::ChatCompletionMessageCustomToolCallType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageCustomToolCall {
        fn default() -> Self {
            Self {
                custom: Err("no value supplied for custom".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionMessageCustomToolCall {
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionMessageCustomToolCallCustom>,
            T::Error: ::std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for custom: {e}"));
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
            T: ::std::convert::TryInto<super::ChatCompletionMessageCustomToolCallType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageCustomToolCall>
        for super::ChatCompletionMessageCustomToolCall
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageCustomToolCall,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom: value.custom?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageCustomToolCall>
        for ChatCompletionMessageCustomToolCall
    {
        fn from(value: super::ChatCompletionMessageCustomToolCall) -> Self {
            Self {
                custom: Ok(value.custom),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageCustomToolCallCustom {
        input: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageCustomToolCallCustom {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionMessageCustomToolCallCustom {
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
    impl ::std::convert::TryFrom<ChatCompletionMessageCustomToolCallCustom>
        for super::ChatCompletionMessageCustomToolCallCustom
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageCustomToolCallCustom,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageCustomToolCallCustom>
        for ChatCompletionMessageCustomToolCallCustom
    {
        fn from(value: super::ChatCompletionMessageCustomToolCallCustom) -> Self {
            Self {
                input: Ok(value.input),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageList {
        data: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionMessageListDataItem>,
            ::std::string::String,
        >,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object:
            ::std::result::Result<super::ChatCompletionMessageListObject, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageList {
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
    impl ChatCompletionMessageList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChatCompletionMessageListDataItem>>,
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
            T: ::std::convert::TryInto<super::ChatCompletionMessageListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageList> for super::ChatCompletionMessageList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageList,
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
    impl ::std::convert::From<super::ChatCompletionMessageList> for ChatCompletionMessageList {
        fn from(value: super::ChatCompletionMessageList) -> Self {
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
    pub struct ChatCompletionMessageListDataItem {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        content_parts: ::std::result::Result<
            ::std::option::Option<
                ::std::vec::Vec<super::ChatCompletionMessageListDataItemContentPartsItem>,
            >,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionMessageListDataItemRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageListDataItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                content_parts: Err("no value supplied for content_parts".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ChatCompletionMessageListDataItem {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn content_parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::vec::Vec<super::ChatCompletionMessageListDataItemContentPartsItem>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content_parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_parts: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionMessageListDataItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageListDataItem>
        for super::ChatCompletionMessageListDataItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageListDataItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                content_parts: value.content_parts?,
                id: value.id?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageListDataItem>
        for ChatCompletionMessageListDataItem
    {
        fn from(value: super::ChatCompletionMessageListDataItem) -> Self {
            Self {
                content: Ok(value.content),
                content_parts: Ok(value.content_parts),
                id: Ok(value.id),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageListDataItemContentPartsItem {
        file: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageListDataItemContentPartsItemFile>,
            ::std::string::String,
        >,
        image_url: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageListDataItemContentPartsItemImageUrl>,
            ::std::string::String,
        >,
        input_audio: ::std::result::Result<
            ::std::option::Option<
                super::ChatCompletionMessageListDataItemContentPartsItemInputAudio,
            >,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionMessageListDataItemContentPartsItemType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageListDataItemContentPartsItem {
        fn default() -> Self {
            Self {
                file: Ok(Default::default()),
                image_url: Ok(Default::default()),
                input_audio: Ok(Default::default()),
                text: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionMessageListDataItemContentPartsItem {
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionMessageListDataItemContentPartsItemFile>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::ChatCompletionMessageListDataItemContentPartsItemImageUrl,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
        pub fn input_audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::ChatCompletionMessageListDataItemContentPartsItemInputAudio,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input_audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_audio: {e}"));
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
                super::ChatCompletionMessageListDataItemContentPartsItemType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageListDataItemContentPartsItem>
        for super::ChatCompletionMessageListDataItemContentPartsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageListDataItemContentPartsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file: value.file?,
                image_url: value.image_url?,
                input_audio: value.input_audio?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageListDataItemContentPartsItem>
        for ChatCompletionMessageListDataItemContentPartsItem
    {
        fn from(value: super::ChatCompletionMessageListDataItemContentPartsItem) -> Self {
            Self {
                file: Ok(value.file),
                image_url: Ok(value.image_url),
                input_audio: Ok(value.input_audio),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageListDataItemContentPartsItemFile {
        file_data: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        filename: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageListDataItemContentPartsItemFile {
        fn default() -> Self {
            Self {
                file_data: Ok(Default::default()),
                file_id: Ok(Default::default()),
                filename: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionMessageListDataItemContentPartsItemFile {
        pub fn file_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_data: {e}"));
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
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageListDataItemContentPartsItemFile>
        for super::ChatCompletionMessageListDataItemContentPartsItemFile
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageListDataItemContentPartsItemFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_data: value.file_data?,
                file_id: value.file_id?,
                filename: value.filename?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageListDataItemContentPartsItemFile>
        for ChatCompletionMessageListDataItemContentPartsItemFile
    {
        fn from(value: super::ChatCompletionMessageListDataItemContentPartsItemFile) -> Self {
            Self {
                file_data: Ok(value.file_data),
                file_id: Ok(value.file_id),
                filename: Ok(value.filename),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageListDataItemContentPartsItemImageUrl {
        detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageListDataItemContentPartsItemImageUrl {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl ChatCompletionMessageListDataItemContentPartsItemImageUrl {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    impl ::std::convert::TryFrom<ChatCompletionMessageListDataItemContentPartsItemImageUrl>
        for super::ChatCompletionMessageListDataItemContentPartsItemImageUrl
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageListDataItemContentPartsItemImageUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageListDataItemContentPartsItemImageUrl>
        for ChatCompletionMessageListDataItemContentPartsItemImageUrl
    {
        fn from(value: super::ChatCompletionMessageListDataItemContentPartsItemImageUrl) -> Self {
            Self {
                detail: Ok(value.detail),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageListDataItemContentPartsItemInputAudio {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        format: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageListDataItemContentPartsItemInputAudio {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                format: Err("no value supplied for format".to_string()),
            }
        }
    }
    impl ChatCompletionMessageListDataItemContentPartsItemInputAudio {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageListDataItemContentPartsItemInputAudio>
        for super::ChatCompletionMessageListDataItemContentPartsItemInputAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageListDataItemContentPartsItemInputAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                format: value.format?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageListDataItemContentPartsItemInputAudio>
        for ChatCompletionMessageListDataItemContentPartsItemInputAudio
    {
        fn from(value: super::ChatCompletionMessageListDataItemContentPartsItemInputAudio) -> Self {
            Self {
                data: Ok(value.data),
                format: Ok(value.format),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageToolCall {
        function: ::std::result::Result<
            super::ChatCompletionMessageToolCallFunction,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_:
            ::std::result::Result<super::ChatCompletionMessageToolCallType, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageToolCall {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionMessageToolCall {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionMessageToolCallFunction>,
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
            T: ::std::convert::TryInto<super::ChatCompletionMessageToolCallType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageToolCall>
        for super::ChatCompletionMessageToolCall
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageToolCall,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageToolCall> for ChatCompletionMessageToolCall {
        fn from(value: super::ChatCompletionMessageToolCall) -> Self {
            Self {
                function: Ok(value.function),
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageToolCallChunk {
        function: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageToolCallChunkFunction>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageToolCallChunkType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageToolCallChunk {
        fn default() -> Self {
            Self {
                function: Ok(Default::default()),
                id: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionMessageToolCallChunk {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionMessageToolCallChunkFunction>,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionMessageToolCallChunkType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageToolCallChunk>
        for super::ChatCompletionMessageToolCallChunk
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageToolCallChunk,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                id: value.id?,
                index: value.index?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageToolCallChunk>
        for ChatCompletionMessageToolCallChunk
    {
        fn from(value: super::ChatCompletionMessageToolCallChunk) -> Self {
            Self {
                function: Ok(value.function),
                id: Ok(value.id),
                index: Ok(value.index),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageToolCallChunkFunction {
        arguments: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionMessageToolCallChunkFunction {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionMessageToolCallChunkFunction {
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
    }
    impl ::std::convert::TryFrom<ChatCompletionMessageToolCallChunkFunction>
        for super::ChatCompletionMessageToolCallChunkFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageToolCallChunkFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageToolCallChunkFunction>
        for ChatCompletionMessageToolCallChunkFunction
    {
        fn from(value: super::ChatCompletionMessageToolCallChunkFunction) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionMessageToolCallFunction {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionMessageToolCallFunction {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionMessageToolCallFunction {
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
    impl ::std::convert::TryFrom<ChatCompletionMessageToolCallFunction>
        for super::ChatCompletionMessageToolCallFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionMessageToolCallFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionMessageToolCallFunction>
        for ChatCompletionMessageToolCallFunction
    {
        fn from(value: super::ChatCompletionMessageToolCallFunction) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionModeration {
        input: ::std::result::Result<super::ChatCompletionModerationInput, ::std::string::String>,
        output: ::std::result::Result<super::ChatCompletionModerationOutput, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionModeration {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                output: Err("no value supplied for output".to_string()),
            }
        }
    }
    impl ChatCompletionModeration {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionModerationInput>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionModerationOutput>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionModeration> for super::ChatCompletionModeration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionModeration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionModeration> for ChatCompletionModeration {
        fn from(value: super::ChatCompletionModeration) -> Self {
            Self {
                input: Ok(value.input),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionModerationError {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_:
            ::std::result::Result<super::ChatCompletionModerationErrorType, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionModerationError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionModerationError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionModerationErrorType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionModerationError>
        for super::ChatCompletionModerationError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionModerationError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionModerationError> for ChatCompletionModerationError {
        fn from(value: super::ChatCompletionModerationError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionModerationResults {
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::ModerationResultBody>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionModerationResultsType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionModerationResults {
        fn default() -> Self {
            Self {
                model: Err("no value supplied for model".to_string()),
                results: Err("no value supplied for results".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionModerationResults {
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
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ModerationResultBody>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionModerationResultsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionModerationResults>
        for super::ChatCompletionModerationResults
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionModerationResults,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
                results: value.results?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionModerationResults>
        for ChatCompletionModerationResults
    {
        fn from(value: super::ChatCompletionModerationResults) -> Self {
            Self {
                model: Ok(value.model),
                results: Ok(value.results),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionNamedToolChoice {
        function: ::std::result::Result<
            super::ChatCompletionNamedToolChoiceFunction,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::ChatCompletionNamedToolChoiceType, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionNamedToolChoice {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionNamedToolChoice {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionNamedToolChoiceFunction>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionNamedToolChoiceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionNamedToolChoice>
        for super::ChatCompletionNamedToolChoice
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionNamedToolChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionNamedToolChoice> for ChatCompletionNamedToolChoice {
        fn from(value: super::ChatCompletionNamedToolChoice) -> Self {
            Self {
                function: Ok(value.function),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionNamedToolChoiceCustom {
        custom: ::std::result::Result<
            super::ChatCompletionNamedToolChoiceCustomCustom,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionNamedToolChoiceCustomType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionNamedToolChoiceCustom {
        fn default() -> Self {
            Self {
                custom: Err("no value supplied for custom".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionNamedToolChoiceCustom {
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionNamedToolChoiceCustomCustom>,
            T::Error: ::std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for custom: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionNamedToolChoiceCustomType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionNamedToolChoiceCustom>
        for super::ChatCompletionNamedToolChoiceCustom
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionNamedToolChoiceCustom,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom: value.custom?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionNamedToolChoiceCustom>
        for ChatCompletionNamedToolChoiceCustom
    {
        fn from(value: super::ChatCompletionNamedToolChoiceCustom) -> Self {
            Self {
                custom: Ok(value.custom),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionNamedToolChoiceCustomCustom {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionNamedToolChoiceCustomCustom {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionNamedToolChoiceCustomCustom {
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
    impl ::std::convert::TryFrom<ChatCompletionNamedToolChoiceCustomCustom>
        for super::ChatCompletionNamedToolChoiceCustomCustom
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionNamedToolChoiceCustomCustom,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ChatCompletionNamedToolChoiceCustomCustom>
        for ChatCompletionNamedToolChoiceCustomCustom
    {
        fn from(value: super::ChatCompletionNamedToolChoiceCustomCustom) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionNamedToolChoiceFunction {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionNamedToolChoiceFunction {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionNamedToolChoiceFunction {
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
    impl ::std::convert::TryFrom<ChatCompletionNamedToolChoiceFunction>
        for super::ChatCompletionNamedToolChoiceFunction
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionNamedToolChoiceFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { name: value.name? })
        }
    }
    impl ::std::convert::From<super::ChatCompletionNamedToolChoiceFunction>
        for ChatCompletionNamedToolChoiceFunction
    {
        fn from(value: super::ChatCompletionNamedToolChoiceFunction) -> Self {
            Self {
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestAssistantMessage {
        audio: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionRequestAssistantMessageAudio>,
            ::std::string::String,
        >,
        content: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionRequestAssistantMessageContent>,
            ::std::string::String,
        >,
        function_call: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionRequestAssistantMessageFunctionCall>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionRequestAssistantMessageRole,
            ::std::string::String,
        >,
        tool_calls: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageToolCalls>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestAssistantMessage {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                content: Ok(Default::default()),
                function_call: Ok(Default::default()),
                name: Ok(Default::default()),
                refusal: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                tool_calls: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionRequestAssistantMessage {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionRequestAssistantMessageAudio>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionRequestAssistantMessageContent>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn function_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionRequestAssistantMessageFunctionCall>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function_call: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestAssistantMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionMessageToolCalls>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestAssistantMessage>
        for super::ChatCompletionRequestAssistantMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestAssistantMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                content: value.content?,
                function_call: value.function_call?,
                name: value.name?,
                refusal: value.refusal?,
                role: value.role?,
                tool_calls: value.tool_calls?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestAssistantMessage>
        for ChatCompletionRequestAssistantMessage
    {
        fn from(value: super::ChatCompletionRequestAssistantMessage) -> Self {
            Self {
                audio: Ok(value.audio),
                content: Ok(value.content),
                function_call: Ok(value.function_call),
                name: Ok(value.name),
                refusal: Ok(value.refusal),
                role: Ok(value.role),
                tool_calls: Ok(value.tool_calls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestAssistantMessageAudio {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionRequestAssistantMessageAudio {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl ChatCompletionRequestAssistantMessageAudio {
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
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestAssistantMessageAudio>
        for super::ChatCompletionRequestAssistantMessageAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestAssistantMessageAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestAssistantMessageAudio>
        for ChatCompletionRequestAssistantMessageAudio
    {
        fn from(value: super::ChatCompletionRequestAssistantMessageAudio) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestAssistantMessageFunctionCall {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionRequestAssistantMessageFunctionCall {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionRequestAssistantMessageFunctionCall {
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
    impl ::std::convert::TryFrom<ChatCompletionRequestAssistantMessageFunctionCall>
        for super::ChatCompletionRequestAssistantMessageFunctionCall
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestAssistantMessageFunctionCall,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestAssistantMessageFunctionCall>
        for ChatCompletionRequestAssistantMessageFunctionCall
    {
        fn from(value: super::ChatCompletionRequestAssistantMessageFunctionCall) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestDeveloperMessage {
        content: ::std::result::Result<
            super::ChatCompletionRequestDeveloperMessageContent,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionRequestDeveloperMessageRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestDeveloperMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                name: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ChatCompletionRequestDeveloperMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestDeveloperMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestDeveloperMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestDeveloperMessage>
        for super::ChatCompletionRequestDeveloperMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestDeveloperMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestDeveloperMessage>
        for ChatCompletionRequestDeveloperMessage
    {
        fn from(value: super::ChatCompletionRequestDeveloperMessage) -> Self {
            Self {
                content: Ok(value.content),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestFunctionMessage {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<
            super::ChatCompletionRequestFunctionMessageRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestFunctionMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                name: Err("no value supplied for name".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ChatCompletionRequestFunctionMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestFunctionMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestFunctionMessage>
        for super::ChatCompletionRequestFunctionMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestFunctionMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestFunctionMessage>
        for ChatCompletionRequestFunctionMessage
    {
        fn from(value: super::ChatCompletionRequestFunctionMessage) -> Self {
            Self {
                content: Ok(value.content),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartAudio {
        input_audio: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartAudioInputAudio,
            ::std::string::String,
        >,
        prompt_cache_breakpoint: ::std::result::Result<
            ::std::option::Option<super::PromptCacheBreakpointParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartAudioType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartAudio {
        fn default() -> Self {
            Self {
                input_audio: Err("no value supplied for input_audio".to_string()),
                prompt_cache_breakpoint: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartAudio {
        pub fn input_audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::ChatCompletionRequestMessageContentPartAudioInputAudio,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input_audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_audio: {e}"));
            self
        }
        pub fn prompt_cache_breakpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheBreakpointParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_breakpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_breakpoint: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartAudioType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartAudio>
        for super::ChatCompletionRequestMessageContentPartAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_audio: value.input_audio?,
                prompt_cache_breakpoint: value.prompt_cache_breakpoint?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartAudio>
        for ChatCompletionRequestMessageContentPartAudio
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartAudio) -> Self {
            Self {
                input_audio: Ok(value.input_audio),
                prompt_cache_breakpoint: Ok(value.prompt_cache_breakpoint),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        format: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartAudioInputAudio {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                format: Err("no value supplied for format".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartAudioInputAudio {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartAudioInputAudio>
        for super::ChatCompletionRequestMessageContentPartAudioInputAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartAudioInputAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                format: value.format?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartAudioInputAudio>
        for ChatCompletionRequestMessageContentPartAudioInputAudio
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartAudioInputAudio) -> Self {
            Self {
                data: Ok(value.data),
                format: Ok(value.format),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartFile {
        file: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartFileFile,
            ::std::string::String,
        >,
        prompt_cache_breakpoint: ::std::result::Result<
            ::std::option::Option<super::PromptCacheBreakpointParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartFileType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartFile {
        fn default() -> Self {
            Self {
                file: Err("no value supplied for file".to_string()),
                prompt_cache_breakpoint: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartFile {
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartFileFile>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn prompt_cache_breakpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheBreakpointParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_breakpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_breakpoint: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartFileType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartFile>
        for super::ChatCompletionRequestMessageContentPartFile
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file: value.file?,
                prompt_cache_breakpoint: value.prompt_cache_breakpoint?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartFile>
        for ChatCompletionRequestMessageContentPartFile
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartFile) -> Self {
            Self {
                file: Ok(value.file),
                prompt_cache_breakpoint: Ok(value.prompt_cache_breakpoint),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartFileFile {
        file_data: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        filename: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartFileFile {
        fn default() -> Self {
            Self {
                file_data: Ok(Default::default()),
                file_id: Ok(Default::default()),
                filename: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartFileFile {
        pub fn file_data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_data: {e}"));
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
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartFileFile>
        for super::ChatCompletionRequestMessageContentPartFileFile
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartFileFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_data: value.file_data?,
                file_id: value.file_id?,
                filename: value.filename?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartFileFile>
        for ChatCompletionRequestMessageContentPartFileFile
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartFileFile) -> Self {
            Self {
                file_data: Ok(value.file_data),
                file_id: Ok(value.file_id),
                filename: Ok(value.filename),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartImage {
        image_url: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartImageImageUrl,
            ::std::string::String,
        >,
        prompt_cache_breakpoint: ::std::result::Result<
            ::std::option::Option<super::PromptCacheBreakpointParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartImageType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartImage {
        fn default() -> Self {
            Self {
                image_url: Err("no value supplied for image_url".to_string()),
                prompt_cache_breakpoint: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartImage {
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartImageImageUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
        pub fn prompt_cache_breakpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheBreakpointParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_breakpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_breakpoint: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartImageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartImage>
        for super::ChatCompletionRequestMessageContentPartImage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_url: value.image_url?,
                prompt_cache_breakpoint: value.prompt_cache_breakpoint?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartImage>
        for ChatCompletionRequestMessageContentPartImage
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartImage) -> Self {
            Self {
                image_url: Ok(value.image_url),
                prompt_cache_breakpoint: Ok(value.prompt_cache_breakpoint),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
        detail: ::std::result::Result<
            ::std::option::Option<
                super::ChatCompletionRequestMessageContentPartImageImageUrlDetail,
            >,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartImageImageUrl {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartImageImageUrl {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::ChatCompletionRequestMessageContentPartImageImageUrlDetail,
                >,
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
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartImageImageUrl>
        for super::ChatCompletionRequestMessageContentPartImageImageUrl
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartImageImageUrl,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartImageImageUrl>
        for ChatCompletionRequestMessageContentPartImageImageUrl
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartImageImageUrl) -> Self {
            Self {
                detail: Ok(value.detail),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartRefusal {
        refusal: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartRefusalType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartRefusal {
        fn default() -> Self {
            Self {
                refusal: Err("no value supplied for refusal".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartRefusal {
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
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartRefusalType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartRefusal>
        for super::ChatCompletionRequestMessageContentPartRefusal
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartRefusal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                refusal: value.refusal?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartRefusal>
        for ChatCompletionRequestMessageContentPartRefusal
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartRefusal) -> Self {
            Self {
                refusal: Ok(value.refusal),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestMessageContentPartText {
        prompt_cache_breakpoint: ::std::result::Result<
            ::std::option::Option<super::PromptCacheBreakpointParam>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::ChatCompletionRequestMessageContentPartTextType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestMessageContentPartText {
        fn default() -> Self {
            Self {
                prompt_cache_breakpoint: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionRequestMessageContentPartText {
        pub fn prompt_cache_breakpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheBreakpointParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_breakpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_breakpoint: {e}")
            });
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
            T: ::std::convert::TryInto<super::ChatCompletionRequestMessageContentPartTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestMessageContentPartText>
        for super::ChatCompletionRequestMessageContentPartText
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestMessageContentPartText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt_cache_breakpoint: value.prompt_cache_breakpoint?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestMessageContentPartText>
        for ChatCompletionRequestMessageContentPartText
    {
        fn from(value: super::ChatCompletionRequestMessageContentPartText) -> Self {
            Self {
                prompt_cache_breakpoint: Ok(value.prompt_cache_breakpoint),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestSystemMessage {
        content: ::std::result::Result<
            super::ChatCompletionRequestSystemMessageContent,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionRequestSystemMessageRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestSystemMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                name: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ChatCompletionRequestSystemMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestSystemMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestSystemMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestSystemMessage>
        for super::ChatCompletionRequestSystemMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestSystemMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestSystemMessage>
        for ChatCompletionRequestSystemMessage
    {
        fn from(value: super::ChatCompletionRequestSystemMessage) -> Self {
            Self {
                content: Ok(value.content),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestToolMessage {
        content: ::std::result::Result<
            super::ChatCompletionRequestToolMessageContent,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionRequestToolMessageRole,
            ::std::string::String,
        >,
        tool_call_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionRequestToolMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
                tool_call_id: Err("no value supplied for tool_call_id".to_string()),
            }
        }
    }
    impl ChatCompletionRequestToolMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestToolMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestToolMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn tool_call_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_call_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_call_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestToolMessage>
        for super::ChatCompletionRequestToolMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestToolMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
                tool_call_id: value.tool_call_id?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestToolMessage>
        for ChatCompletionRequestToolMessage
    {
        fn from(value: super::ChatCompletionRequestToolMessage) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
                tool_call_id: Ok(value.tool_call_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionRequestUserMessage {
        content: ::std::result::Result<
            super::ChatCompletionRequestUserMessageContent,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ChatCompletionRequestUserMessageRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionRequestUserMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                name: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ChatCompletionRequestUserMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestUserMessageContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionRequestUserMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionRequestUserMessage>
        for super::ChatCompletionRequestUserMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionRequestUserMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionRequestUserMessage>
        for ChatCompletionRequestUserMessage
    {
        fn from(value: super::ChatCompletionRequestUserMessage) -> Self {
            Self {
                content: Ok(value.content),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionResponseMessage {
        annotations: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionResponseMessageAnnotationsItem>,
            ::std::string::String,
        >,
        audio: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionResponseMessageAudio>,
            ::std::string::String,
        >,
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        function_call: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionResponseMessageFunctionCall>,
            ::std::string::String,
        >,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role:
            ::std::result::Result<super::ChatCompletionResponseMessageRole, ::std::string::String>,
        tool_calls: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionMessageToolCalls>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionResponseMessage {
        fn default() -> Self {
            Self {
                annotations: Ok(Default::default()),
                audio: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                function_call: Ok(Default::default()),
                refusal: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                tool_calls: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionResponseMessage {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ChatCompletionResponseMessageAnnotationsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
            self
        }
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionResponseMessageAudio>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn function_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionResponseMessageFunctionCall>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function_call: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionResponseMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionMessageToolCalls>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionResponseMessage>
        for super::ChatCompletionResponseMessage
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionResponseMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                audio: value.audio?,
                content: value.content?,
                function_call: value.function_call?,
                refusal: value.refusal?,
                role: value.role?,
                tool_calls: value.tool_calls?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionResponseMessage> for ChatCompletionResponseMessage {
        fn from(value: super::ChatCompletionResponseMessage) -> Self {
            Self {
                annotations: Ok(value.annotations),
                audio: Ok(value.audio),
                content: Ok(value.content),
                function_call: Ok(value.function_call),
                refusal: Ok(value.refusal),
                role: Ok(value.role),
                tool_calls: Ok(value.tool_calls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionResponseMessageAnnotationsItem {
        type_: ::std::result::Result<
            super::ChatCompletionResponseMessageAnnotationsItemType,
            ::std::string::String,
        >,
        url_citation: ::std::result::Result<
            super::ChatCompletionResponseMessageAnnotationsItemUrlCitation,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionResponseMessageAnnotationsItem {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                url_citation: Err("no value supplied for url_citation".to_string()),
            }
        }
    }
    impl ChatCompletionResponseMessageAnnotationsItem {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionResponseMessageAnnotationsItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn url_citation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::ChatCompletionResponseMessageAnnotationsItemUrlCitation,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.url_citation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url_citation: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionResponseMessageAnnotationsItem>
        for super::ChatCompletionResponseMessageAnnotationsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionResponseMessageAnnotationsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                url_citation: value.url_citation?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionResponseMessageAnnotationsItem>
        for ChatCompletionResponseMessageAnnotationsItem
    {
        fn from(value: super::ChatCompletionResponseMessageAnnotationsItem) -> Self {
            Self {
                type_: Ok(value.type_),
                url_citation: Ok(value.url_citation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionResponseMessageAnnotationsItemUrlCitation {
        end_index: ::std::result::Result<i64, ::std::string::String>,
        start_index: ::std::result::Result<i64, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionResponseMessageAnnotationsItemUrlCitation {
        fn default() -> Self {
            Self {
                end_index: Err("no value supplied for end_index".to_string()),
                start_index: Err("no value supplied for start_index".to_string()),
                title: Err("no value supplied for title".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl ChatCompletionResponseMessageAnnotationsItemUrlCitation {
        pub fn end_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_index: {e}"));
            self
        }
        pub fn start_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_index: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
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
    impl ::std::convert::TryFrom<ChatCompletionResponseMessageAnnotationsItemUrlCitation>
        for super::ChatCompletionResponseMessageAnnotationsItemUrlCitation
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionResponseMessageAnnotationsItemUrlCitation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_index: value.end_index?,
                start_index: value.start_index?,
                title: value.title?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionResponseMessageAnnotationsItemUrlCitation>
        for ChatCompletionResponseMessageAnnotationsItemUrlCitation
    {
        fn from(value: super::ChatCompletionResponseMessageAnnotationsItemUrlCitation) -> Self {
            Self {
                end_index: Ok(value.end_index),
                start_index: Ok(value.start_index),
                title: Ok(value.title),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionResponseMessageAudio {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        expires_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        transcript: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionResponseMessageAudio {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                transcript: Err("no value supplied for transcript".to_string()),
            }
        }
    }
    impl ChatCompletionResponseMessageAudio {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
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
        pub fn transcript<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.transcript = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transcript: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionResponseMessageAudio>
        for super::ChatCompletionResponseMessageAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionResponseMessageAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                expires_at: value.expires_at?,
                id: value.id?,
                transcript: value.transcript?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionResponseMessageAudio>
        for ChatCompletionResponseMessageAudio
    {
        fn from(value: super::ChatCompletionResponseMessageAudio) -> Self {
            Self {
                data: Ok(value.data),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                transcript: Ok(value.transcript),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionResponseMessageFunctionCall {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionResponseMessageFunctionCall {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ChatCompletionResponseMessageFunctionCall {
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
    impl ::std::convert::TryFrom<ChatCompletionResponseMessageFunctionCall>
        for super::ChatCompletionResponseMessageFunctionCall
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionResponseMessageFunctionCall,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionResponseMessageFunctionCall>
        for ChatCompletionResponseMessageFunctionCall
    {
        fn from(value: super::ChatCompletionResponseMessageFunctionCall) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionStreamOptionsValue {
        include_obfuscation:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        include_usage: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionStreamOptionsValue {
        fn default() -> Self {
            Self {
                include_obfuscation: Ok(Default::default()),
                include_usage: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionStreamOptionsValue {
        pub fn include_obfuscation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.include_obfuscation = value.try_into().map_err(|e| {
                format!("error converting supplied value for include_obfuscation: {e}")
            });
            self
        }
        pub fn include_usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.include_usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include_usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionStreamOptionsValue>
        for super::ChatCompletionStreamOptionsValue
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionStreamOptionsValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include_obfuscation: value.include_obfuscation?,
                include_usage: value.include_usage?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionStreamOptionsValue>
        for ChatCompletionStreamOptionsValue
    {
        fn from(value: super::ChatCompletionStreamOptionsValue) -> Self {
            Self {
                include_obfuscation: Ok(value.include_obfuscation),
                include_usage: Ok(value.include_usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionStreamResponseDelta {
        audio: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionStreamResponseDeltaAudio>,
            ::std::string::String,
        >,
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        function_call: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionStreamResponseDeltaFunctionCall>,
            ::std::string::String,
        >,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionStreamResponseDeltaRole>,
            ::std::string::String,
        >,
        tool_calls: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionMessageToolCallChunk>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionStreamResponseDelta {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                content: Ok(Default::default()),
                function_call: Ok(Default::default()),
                refusal: Ok(Default::default()),
                role: Ok(Default::default()),
                tool_calls: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionStreamResponseDelta {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionStreamResponseDeltaAudio>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn function_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionStreamResponseDeltaFunctionCall>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function_call: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionStreamResponseDeltaRole>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChatCompletionMessageToolCallChunk>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_calls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_calls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionStreamResponseDelta>
        for super::ChatCompletionStreamResponseDelta
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionStreamResponseDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                content: value.content?,
                function_call: value.function_call?,
                refusal: value.refusal?,
                role: value.role?,
                tool_calls: value.tool_calls?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionStreamResponseDelta>
        for ChatCompletionStreamResponseDelta
    {
        fn from(value: super::ChatCompletionStreamResponseDelta) -> Self {
            Self {
                audio: Ok(value.audio),
                content: Ok(value.content),
                function_call: Ok(value.function_call),
                refusal: Ok(value.refusal),
                role: Ok(value.role),
                tool_calls: Ok(value.tool_calls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionStreamResponseDeltaAudio {
        data: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        transcript: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionStreamResponseDeltaAudio {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                id: Ok(Default::default()),
                transcript: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionStreamResponseDeltaAudio {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
        pub fn transcript<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.transcript = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transcript: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionStreamResponseDeltaAudio>
        for super::ChatCompletionStreamResponseDeltaAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionStreamResponseDeltaAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                expires_at: value.expires_at?,
                id: value.id?,
                transcript: value.transcript?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionStreamResponseDeltaAudio>
        for ChatCompletionStreamResponseDeltaAudio
    {
        fn from(value: super::ChatCompletionStreamResponseDeltaAudio) -> Self {
            Self {
                data: Ok(value.data),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                transcript: Ok(value.transcript),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionStreamResponseDeltaFunctionCall {
        arguments: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionStreamResponseDeltaFunctionCall {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl ChatCompletionStreamResponseDeltaFunctionCall {
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
    }
    impl ::std::convert::TryFrom<ChatCompletionStreamResponseDeltaFunctionCall>
        for super::ChatCompletionStreamResponseDeltaFunctionCall
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionStreamResponseDeltaFunctionCall,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionStreamResponseDeltaFunctionCall>
        for ChatCompletionStreamResponseDeltaFunctionCall
    {
        fn from(value: super::ChatCompletionStreamResponseDeltaFunctionCall) -> Self {
            Self {
                arguments: Ok(value.arguments),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionTokenLogprob {
        bytes: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<i64>>,
            ::std::string::String,
        >,
        logprob: ::std::result::Result<f64, ::std::string::String>,
        token: ::std::result::Result<::std::string::String, ::std::string::String>,
        top_logprobs: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionTokenLogprobTopLogprobsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatCompletionTokenLogprob {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                logprob: Err("no value supplied for logprob".to_string()),
                token: Err("no value supplied for token".to_string()),
                top_logprobs: Err("no value supplied for top_logprobs".to_string()),
            }
        }
    }
    impl ChatCompletionTokenLogprob {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<i64>>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprob: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
        pub fn top_logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ChatCompletionTokenLogprobTopLogprobsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.top_logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_logprobs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionTokenLogprob> for super::ChatCompletionTokenLogprob {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionTokenLogprob,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                logprob: value.logprob?,
                token: value.token?,
                top_logprobs: value.top_logprobs?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionTokenLogprob> for ChatCompletionTokenLogprob {
        fn from(value: super::ChatCompletionTokenLogprob) -> Self {
            Self {
                bytes: Ok(value.bytes),
                logprob: Ok(value.logprob),
                token: Ok(value.token),
                top_logprobs: Ok(value.top_logprobs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionTokenLogprobTopLogprobsItem {
        bytes: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<i64>>,
            ::std::string::String,
        >,
        logprob: ::std::result::Result<f64, ::std::string::String>,
        token: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionTokenLogprobTopLogprobsItem {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                logprob: Err("no value supplied for logprob".to_string()),
                token: Err("no value supplied for token".to_string()),
            }
        }
    }
    impl ChatCompletionTokenLogprobTopLogprobsItem {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<i64>>>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
        pub fn logprob<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.logprob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprob: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionTokenLogprobTopLogprobsItem>
        for super::ChatCompletionTokenLogprobTopLogprobsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionTokenLogprobTopLogprobsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                logprob: value.logprob?,
                token: value.token?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionTokenLogprobTopLogprobsItem>
        for ChatCompletionTokenLogprobTopLogprobsItem
    {
        fn from(value: super::ChatCompletionTokenLogprobTopLogprobsItem) -> Self {
            Self {
                bytes: Ok(value.bytes),
                logprob: Ok(value.logprob),
                token: Ok(value.token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatCompletionTool {
        function: ::std::result::Result<super::FunctionObject, ::std::string::String>,
        type_: ::std::result::Result<super::ChatCompletionToolType, ::std::string::String>,
    }
    impl ::std::default::Default for ChatCompletionTool {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatCompletionTool {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FunctionObject>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionToolType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatCompletionTool> for super::ChatCompletionTool {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatCompletionTool,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ChatCompletionTool> for ChatCompletionTool {
        fn from(value: super::ChatCompletionTool) -> Self {
            Self {
                function: Ok(value.function),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompletionUsage {
        completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        completion_tokens_details: ::std::result::Result<
            ::std::option::Option<super::CompletionUsageCompletionTokensDetails>,
            ::std::string::String,
        >,
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        prompt_tokens_details: ::std::result::Result<
            ::std::option::Option<super::CompletionUsagePromptTokensDetails>,
            ::std::string::String,
        >,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for CompletionUsage {
        fn default() -> Self {
            Self {
                completion_tokens: Err("no value supplied for completion_tokens".to_string()),
                completion_tokens_details: Ok(Default::default()),
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                prompt_tokens_details: Ok(Default::default()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl CompletionUsage {
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
        pub fn completion_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CompletionUsageCompletionTokensDetails>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.completion_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for completion_tokens_details: {e}")
            });
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
        pub fn prompt_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CompletionUsagePromptTokensDetails>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_tokens_details: {e}")
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
    impl ::std::convert::TryFrom<CompletionUsage> for super::CompletionUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompletionUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completion_tokens: value.completion_tokens?,
                completion_tokens_details: value.completion_tokens_details?,
                prompt_tokens: value.prompt_tokens?,
                prompt_tokens_details: value.prompt_tokens_details?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::CompletionUsage> for CompletionUsage {
        fn from(value: super::CompletionUsage) -> Self {
            Self {
                completion_tokens: Ok(value.completion_tokens),
                completion_tokens_details: Ok(value.completion_tokens_details),
                prompt_tokens: Ok(value.prompt_tokens),
                prompt_tokens_details: Ok(value.prompt_tokens_details),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompletionUsageCompletionTokensDetails {
        accepted_prediction_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        audio_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        reasoning_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        rejected_prediction_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        text_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for CompletionUsageCompletionTokensDetails {
        fn default() -> Self {
            Self {
                accepted_prediction_tokens: Ok(Default::default()),
                audio_tokens: Ok(Default::default()),
                reasoning_tokens: Ok(Default::default()),
                rejected_prediction_tokens: Ok(Default::default()),
                text_tokens: Ok(Default::default()),
            }
        }
    }
    impl CompletionUsageCompletionTokensDetails {
        pub fn accepted_prediction_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.accepted_prediction_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for accepted_prediction_tokens: {e}")
            });
            self
        }
        pub fn audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_tokens: {e}"));
            self
        }
        pub fn reasoning_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_tokens: {e}"));
            self
        }
        pub fn rejected_prediction_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.rejected_prediction_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for rejected_prediction_tokens: {e}")
            });
            self
        }
        pub fn text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompletionUsageCompletionTokensDetails>
        for super::CompletionUsageCompletionTokensDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompletionUsageCompletionTokensDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accepted_prediction_tokens: value.accepted_prediction_tokens?,
                audio_tokens: value.audio_tokens?,
                reasoning_tokens: value.reasoning_tokens?,
                rejected_prediction_tokens: value.rejected_prediction_tokens?,
                text_tokens: value.text_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::CompletionUsageCompletionTokensDetails>
        for CompletionUsageCompletionTokensDetails
    {
        fn from(value: super::CompletionUsageCompletionTokensDetails) -> Self {
            Self {
                accepted_prediction_tokens: Ok(value.accepted_prediction_tokens),
                audio_tokens: Ok(value.audio_tokens),
                reasoning_tokens: Ok(value.reasoning_tokens),
                rejected_prediction_tokens: Ok(value.rejected_prediction_tokens),
                text_tokens: Ok(value.text_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompletionUsagePromptTokensDetails {
        audio_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        cache_write_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        cached_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        image_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        text_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for CompletionUsagePromptTokensDetails {
        fn default() -> Self {
            Self {
                audio_tokens: Ok(Default::default()),
                cache_write_tokens: Ok(Default::default()),
                cached_tokens: Ok(Default::default()),
                image_tokens: Ok(Default::default()),
                text_tokens: Ok(Default::default()),
            }
        }
    }
    impl CompletionUsagePromptTokensDetails {
        pub fn audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_tokens: {e}"));
            self
        }
        pub fn cache_write_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_write_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for cache_write_tokens: {e}")
            });
            self
        }
        pub fn cached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cached_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cached_tokens: {e}"));
            self
        }
        pub fn image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.image_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_tokens: {e}"));
            self
        }
        pub fn text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CompletionUsagePromptTokensDetails>
        for super::CompletionUsagePromptTokensDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CompletionUsagePromptTokensDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio_tokens: value.audio_tokens?,
                cache_write_tokens: value.cache_write_tokens?,
                cached_tokens: value.cached_tokens?,
                image_tokens: value.image_tokens?,
                text_tokens: value.text_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::CompletionUsagePromptTokensDetails>
        for CompletionUsagePromptTokensDetails
    {
        fn from(value: super::CompletionUsagePromptTokensDetails) -> Self {
            Self {
                audio_tokens: Ok(value.audio_tokens),
                cache_write_tokens: Ok(value.cache_write_tokens),
                cached_tokens: Ok(value.cached_tokens),
                image_tokens: Ok(value.image_tokens),
                text_tokens: Ok(value.text_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionRequest {
        audio: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionRequestAudio>,
            ::std::string::String,
        >,
        frequency_penalty: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        function_call: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionRequestFunctionCall>,
            ::std::string::String,
        >,
        functions: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionFunctions>,
            ::std::string::String,
        >,
        logit_bias: ::std::result::Result<
            ::std::option::Option<::std::collections::HashMap<::std::string::String, i64>>,
            ::std::string::String,
        >,
        logprobs: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        messages: ::std::result::Result<
            ::std::vec::Vec<super::ChatCompletionRequestMessage>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        modalities: ::std::result::Result<
            ::std::option::Option<super::ResponseModalities>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::ModelIdsShared, ::std::string::String>,
        moderation: ::std::result::Result<
            ::std::option::Option<super::ModerationParam>,
            ::std::string::String,
        >,
        n: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        parallel_tool_calls: ::std::result::Result<
            ::std::option::Option<super::ParallelToolCalls>,
            ::std::string::String,
        >,
        prediction: ::std::result::Result<
            ::std::option::Option<super::PredictionContent>,
            ::std::string::String,
        >,
        presence_penalty: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        prompt_cache_key: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompt_cache_options: ::std::result::Result<
            ::std::option::Option<super::PromptCacheOptionsParam>,
            ::std::string::String,
        >,
        prompt_cache_retention: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionRequestPromptCacheRetention>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<super::ReasoningEffort>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionRequestResponseFormat>,
            ::std::string::String,
        >,
        safety_identifier: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionRequestSafetyIdentifier>,
            ::std::string::String,
        >,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        service_tier:
            ::std::result::Result<::std::option::Option<super::ServiceTier>, ::std::string::String>,
        stop: ::std::result::Result<
            ::std::option::Option<super::StopConfiguration>,
            ::std::string::String,
        >,
        store: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        stream: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        stream_options: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionStreamOptions>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionToolChoiceOption>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::CreateChatCompletionRequestToolsItem>,
            ::std::string::String,
        >,
        top_logprobs: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        user: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        verbosity:
            ::std::result::Result<::std::option::Option<super::Verbosity>, ::std::string::String>,
        web_search_options:
            ::std::result::Result<::std::option::Option<super::WebSearch>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateChatCompletionRequest {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                frequency_penalty: Ok(Default::default()),
                function_call: Ok(Default::default()),
                functions: Ok(Default::default()),
                logit_bias: Ok(Default::default()),
                logprobs: Ok(Default::default()),
                max_completion_tokens: Ok(Default::default()),
                max_tokens: Ok(Default::default()),
                messages: Err("no value supplied for messages".to_string()),
                metadata: Ok(Default::default()),
                modalities: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                moderation: Ok(Default::default()),
                n: Ok(Default::default()),
                parallel_tool_calls: Ok(Default::default()),
                prediction: Ok(Default::default()),
                presence_penalty: Ok(Default::default()),
                prompt_cache_key: Ok(Default::default()),
                prompt_cache_options: Ok(Default::default()),
                prompt_cache_retention: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                response_format: Ok(Default::default()),
                safety_identifier: Ok(Default::default()),
                seed: Ok(Default::default()),
                service_tier: Ok(Default::default()),
                stop: Ok(Default::default()),
                store: Ok(Default::default()),
                stream: Ok(Default::default()),
                stream_options: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_logprobs: Ok(Default::default()),
                top_p: Ok(Default::default()),
                user: Ok(Default::default()),
                verbosity: Ok(Default::default()),
                web_search_options: Ok(Default::default()),
            }
        }
    }
    impl CreateChatCompletionRequest {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionRequestAudio>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn frequency_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.frequency_penalty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for frequency_penalty: {e}"));
            self
        }
        pub fn function_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionRequestFunctionCall>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.function_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function_call: {e}"));
            self
        }
        pub fn functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChatCompletionFunctions>>,
            T::Error: ::std::fmt::Display,
        {
            self.functions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for functions: {e}"));
            self
        }
        pub fn logit_bias<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::collections::HashMap<::std::string::String, i64>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logit_bias = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logit_bias: {e}"));
            self
        }
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
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
        pub fn max_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_tokens: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChatCompletionRequestMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
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
        pub fn modalities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResponseModalities>>,
            T::Error: ::std::fmt::Display,
        {
            self.modalities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modalities: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModelIdsShared>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn moderation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModerationParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.moderation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moderation: {e}"));
            self
        }
        pub fn n<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.n = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n: {e}"));
            self
        }
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ParallelToolCalls>>,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn prediction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PredictionContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.prediction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prediction: {e}"));
            self
        }
        pub fn presence_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.presence_penalty = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for presence_penalty: {e}"));
            self
        }
        pub fn prompt_cache_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_cache_key: {e}"));
            self
        }
        pub fn prompt_cache_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheOptionsParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_options = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_options: {e}")
            });
            self
        }
        pub fn prompt_cache_retention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionRequestPromptCacheRetention>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_retention = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_retention: {e}")
            });
            self
        }
        pub fn reasoning_effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReasoningEffort>>,
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
                ::std::option::Option<super::CreateChatCompletionRequestResponseFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
            self
        }
        pub fn safety_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionRequestSafetyIdentifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.safety_identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for safety_identifier: {e}"));
            self
        }
        pub fn seed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.seed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seed: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServiceTier>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn stop<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StopConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.stop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop: {e}"));
            self
        }
        pub fn store<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.store = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for store: {e}"));
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
        pub fn stream_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChatCompletionStreamOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.stream_options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream_options: {e}"));
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ChatCompletionToolChoiceOption>,
            >,
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
                ::std::vec::Vec<super::CreateChatCompletionRequestToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn top_logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_logprobs: {e}"));
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
        pub fn verbosity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Verbosity>>,
            T::Error: ::std::fmt::Display,
        {
            self.verbosity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for verbosity: {e}"));
            self
        }
        pub fn web_search_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebSearch>>,
            T::Error: ::std::fmt::Display,
        {
            self.web_search_options = value.try_into().map_err(|e| {
                format!("error converting supplied value for web_search_options: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionRequest> for super::CreateChatCompletionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                frequency_penalty: value.frequency_penalty?,
                function_call: value.function_call?,
                functions: value.functions?,
                logit_bias: value.logit_bias?,
                logprobs: value.logprobs?,
                max_completion_tokens: value.max_completion_tokens?,
                max_tokens: value.max_tokens?,
                messages: value.messages?,
                metadata: value.metadata?,
                modalities: value.modalities?,
                model: value.model?,
                moderation: value.moderation?,
                n: value.n?,
                parallel_tool_calls: value.parallel_tool_calls?,
                prediction: value.prediction?,
                presence_penalty: value.presence_penalty?,
                prompt_cache_key: value.prompt_cache_key?,
                prompt_cache_options: value.prompt_cache_options?,
                prompt_cache_retention: value.prompt_cache_retention?,
                reasoning_effort: value.reasoning_effort?,
                response_format: value.response_format?,
                safety_identifier: value.safety_identifier?,
                seed: value.seed?,
                service_tier: value.service_tier?,
                stop: value.stop?,
                store: value.store?,
                stream: value.stream?,
                stream_options: value.stream_options?,
                temperature: value.temperature?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
                top_logprobs: value.top_logprobs?,
                top_p: value.top_p?,
                user: value.user?,
                verbosity: value.verbosity?,
                web_search_options: value.web_search_options?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionRequest> for CreateChatCompletionRequest {
        fn from(value: super::CreateChatCompletionRequest) -> Self {
            Self {
                audio: Ok(value.audio),
                frequency_penalty: Ok(value.frequency_penalty),
                function_call: Ok(value.function_call),
                functions: Ok(value.functions),
                logit_bias: Ok(value.logit_bias),
                logprobs: Ok(value.logprobs),
                max_completion_tokens: Ok(value.max_completion_tokens),
                max_tokens: Ok(value.max_tokens),
                messages: Ok(value.messages),
                metadata: Ok(value.metadata),
                modalities: Ok(value.modalities),
                model: Ok(value.model),
                moderation: Ok(value.moderation),
                n: Ok(value.n),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                prediction: Ok(value.prediction),
                presence_penalty: Ok(value.presence_penalty),
                prompt_cache_key: Ok(value.prompt_cache_key),
                prompt_cache_options: Ok(value.prompt_cache_options),
                prompt_cache_retention: Ok(value.prompt_cache_retention),
                reasoning_effort: Ok(value.reasoning_effort),
                response_format: Ok(value.response_format),
                safety_identifier: Ok(value.safety_identifier),
                seed: Ok(value.seed),
                service_tier: Ok(value.service_tier),
                stop: Ok(value.stop),
                store: Ok(value.store),
                stream: Ok(value.stream),
                stream_options: Ok(value.stream_options),
                temperature: Ok(value.temperature),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
                top_logprobs: Ok(value.top_logprobs),
                top_p: Ok(value.top_p),
                user: Ok(value.user),
                verbosity: Ok(value.verbosity),
                web_search_options: Ok(value.web_search_options),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionRequestAudio {
        format: ::std::result::Result<
            super::CreateChatCompletionRequestAudioFormat,
            ::std::string::String,
        >,
        voice: ::std::result::Result<super::VoiceIdsOrCustomVoice, ::std::string::String>,
    }
    impl ::std::default::Default for CreateChatCompletionRequestAudio {
        fn default() -> Self {
            Self {
                format: Err("no value supplied for format".to_string()),
                voice: Err("no value supplied for voice".to_string()),
            }
        }
    }
    impl CreateChatCompletionRequestAudio {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionRequestAudioFormat>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VoiceIdsOrCustomVoice>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionRequestAudio>
        for super::CreateChatCompletionRequestAudio
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionRequestAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionRequestAudio>
        for CreateChatCompletionRequestAudio
    {
        fn from(value: super::CreateChatCompletionRequestAudio) -> Self {
            Self {
                format: Ok(value.format),
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionResponse {
        choices: ::std::result::Result<
            ::std::vec::Vec<super::CreateChatCompletionResponseChoicesItem>,
            ::std::string::String,
        >,
        created: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        moderation: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionModeration>,
            ::std::string::String,
        >,
        object:
            ::std::result::Result<super::CreateChatCompletionResponseObject, ::std::string::String>,
        service_tier:
            ::std::result::Result<::std::option::Option<super::ServiceTier>, ::std::string::String>,
        system_fingerprint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<
            ::std::option::Option<super::CompletionUsage>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateChatCompletionResponse {
        fn default() -> Self {
            Self {
                choices: Err("no value supplied for choices".to_string()),
                created: Err("no value supplied for created".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                moderation: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                service_tier: Ok(Default::default()),
                system_fingerprint: Ok(Default::default()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl CreateChatCompletionResponse {
        pub fn choices<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateChatCompletionResponseChoicesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.choices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choices: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn moderation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChatCompletionModeration>>,
            T::Error: ::std::fmt::Display,
        {
            self.moderation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moderation: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServiceTier>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn system_fingerprint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_fingerprint = value.try_into().map_err(|e| {
                format!("error converting supplied value for system_fingerprint: {e}")
            });
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CompletionUsage>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionResponse> for super::CreateChatCompletionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                choices: value.choices?,
                created: value.created?,
                id: value.id?,
                metadata: value.metadata?,
                model: value.model?,
                moderation: value.moderation?,
                object: value.object?,
                service_tier: value.service_tier?,
                system_fingerprint: value.system_fingerprint?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionResponse> for CreateChatCompletionResponse {
        fn from(value: super::CreateChatCompletionResponse) -> Self {
            Self {
                choices: Ok(value.choices),
                created: Ok(value.created),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                moderation: Ok(value.moderation),
                object: Ok(value.object),
                service_tier: Ok(value.service_tier),
                system_fingerprint: Ok(value.system_fingerprint),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionResponseChoicesItem {
        finish_reason: ::std::result::Result<
            super::CreateChatCompletionResponseChoicesItemFinishReason,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        logprobs: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionResponseChoicesItemLogprobs>,
            ::std::string::String,
        >,
        message: ::std::result::Result<super::ChatCompletionResponseMessage, ::std::string::String>,
    }
    impl ::std::default::Default for CreateChatCompletionResponseChoicesItem {
        fn default() -> Self {
            Self {
                finish_reason: Err("no value supplied for finish_reason".to_string()),
                index: Err("no value supplied for index".to_string()),
                logprobs: Err("no value supplied for logprobs".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl CreateChatCompletionResponseChoicesItem {
        pub fn finish_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionResponseChoicesItemFinishReason>,
            T::Error: ::std::fmt::Display,
        {
            self.finish_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finish_reason: {e}"));
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
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionResponseChoicesItemLogprobs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionResponseMessage>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionResponseChoicesItem>
        for super::CreateChatCompletionResponseChoicesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionResponseChoicesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                finish_reason: value.finish_reason?,
                index: value.index?,
                logprobs: value.logprobs?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionResponseChoicesItem>
        for CreateChatCompletionResponseChoicesItem
    {
        fn from(value: super::CreateChatCompletionResponseChoicesItem) -> Self {
            Self {
                finish_reason: Ok(value.finish_reason),
                index: Ok(value.index),
                logprobs: Ok(value.logprobs),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionResponseChoicesItemLogprobs {
        content: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            ::std::string::String,
        >,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateChatCompletionResponseChoicesItemLogprobs {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                refusal: Err("no value supplied for refusal".to_string()),
            }
        }
    }
    impl CreateChatCompletionResponseChoicesItemLogprobs {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn refusal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.refusal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refusal: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionResponseChoicesItemLogprobs>
        for super::CreateChatCompletionResponseChoicesItemLogprobs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionResponseChoicesItemLogprobs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                refusal: value.refusal?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionResponseChoicesItemLogprobs>
        for CreateChatCompletionResponseChoicesItemLogprobs
    {
        fn from(value: super::CreateChatCompletionResponseChoicesItemLogprobs) -> Self {
            Self {
                content: Ok(value.content),
                refusal: Ok(value.refusal),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionStreamResponse {
        choices: ::std::result::Result<
            ::std::vec::Vec<super::CreateChatCompletionStreamResponseChoicesItem>,
            ::std::string::String,
        >,
        created: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        moderation: ::std::result::Result<
            ::std::option::Option<super::ChatCompletionModeration>,
            ::std::string::String,
        >,
        obfuscation: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::CreateChatCompletionStreamResponseObject,
            ::std::string::String,
        >,
        service_tier:
            ::std::result::Result<::std::option::Option<super::ServiceTier>, ::std::string::String>,
        system_fingerprint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<
            ::std::option::Option<super::CompletionUsage>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateChatCompletionStreamResponse {
        fn default() -> Self {
            Self {
                choices: Err("no value supplied for choices".to_string()),
                created: Err("no value supplied for created".to_string()),
                id: Err("no value supplied for id".to_string()),
                model: Err("no value supplied for model".to_string()),
                moderation: Ok(Default::default()),
                obfuscation: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                service_tier: Ok(Default::default()),
                system_fingerprint: Ok(Default::default()),
                usage: Ok(Default::default()),
            }
        }
    }
    impl CreateChatCompletionStreamResponse {
        pub fn choices<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateChatCompletionStreamResponseChoicesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.choices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choices: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
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
        pub fn moderation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChatCompletionModeration>>,
            T::Error: ::std::fmt::Display,
        {
            self.moderation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moderation: {e}"));
            self
        }
        pub fn obfuscation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.obfuscation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for obfuscation: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatCompletionStreamResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ServiceTier>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn system_fingerprint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_fingerprint = value.try_into().map_err(|e| {
                format!("error converting supplied value for system_fingerprint: {e}")
            });
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CompletionUsage>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionStreamResponse>
        for super::CreateChatCompletionStreamResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionStreamResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                choices: value.choices?,
                created: value.created?,
                id: value.id?,
                model: value.model?,
                moderation: value.moderation?,
                obfuscation: value.obfuscation?,
                object: value.object?,
                service_tier: value.service_tier?,
                system_fingerprint: value.system_fingerprint?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionStreamResponse>
        for CreateChatCompletionStreamResponse
    {
        fn from(value: super::CreateChatCompletionStreamResponse) -> Self {
            Self {
                choices: Ok(value.choices),
                created: Ok(value.created),
                id: Ok(value.id),
                model: Ok(value.model),
                moderation: Ok(value.moderation),
                obfuscation: Ok(value.obfuscation),
                object: Ok(value.object),
                service_tier: Ok(value.service_tier),
                system_fingerprint: Ok(value.system_fingerprint),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionStreamResponseChoicesItem {
        delta:
            ::std::result::Result<super::ChatCompletionStreamResponseDelta, ::std::string::String>,
        finish_reason: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionStreamResponseChoicesItemFinishReason>,
            ::std::string::String,
        >,
        index: ::std::result::Result<i64, ::std::string::String>,
        logprobs: ::std::result::Result<
            ::std::option::Option<super::CreateChatCompletionStreamResponseChoicesItemLogprobs>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateChatCompletionStreamResponseChoicesItem {
        fn default() -> Self {
            Self {
                delta: Err("no value supplied for delta".to_string()),
                finish_reason: Ok(Default::default()),
                index: Err("no value supplied for index".to_string()),
                logprobs: Ok(Default::default()),
            }
        }
    }
    impl CreateChatCompletionStreamResponseChoicesItem {
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatCompletionStreamResponseDelta>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
            self
        }
        pub fn finish_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::CreateChatCompletionStreamResponseChoicesItemFinishReason,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.finish_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finish_reason: {e}"));
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
        pub fn logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateChatCompletionStreamResponseChoicesItemLogprobs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logprobs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionStreamResponseChoicesItem>
        for super::CreateChatCompletionStreamResponseChoicesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionStreamResponseChoicesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delta: value.delta?,
                finish_reason: value.finish_reason?,
                index: value.index?,
                logprobs: value.logprobs?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionStreamResponseChoicesItem>
        for CreateChatCompletionStreamResponseChoicesItem
    {
        fn from(value: super::CreateChatCompletionStreamResponseChoicesItem) -> Self {
            Self {
                delta: Ok(value.delta),
                finish_reason: Ok(value.finish_reason),
                index: Ok(value.index),
                logprobs: Ok(value.logprobs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatCompletionStreamResponseChoicesItemLogprobs {
        content: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            ::std::string::String,
        >,
        refusal: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateChatCompletionStreamResponseChoicesItemLogprobs {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                refusal: Err("no value supplied for refusal".to_string()),
            }
        }
    }
    impl CreateChatCompletionStreamResponseChoicesItemLogprobs {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn refusal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::ChatCompletionTokenLogprob>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.refusal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refusal: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatCompletionStreamResponseChoicesItemLogprobs>
        for super::CreateChatCompletionStreamResponseChoicesItemLogprobs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatCompletionStreamResponseChoicesItemLogprobs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                refusal: value.refusal?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatCompletionStreamResponseChoicesItemLogprobs>
        for CreateChatCompletionStreamResponseChoicesItemLogprobs
    {
        fn from(value: super::CreateChatCompletionStreamResponseChoicesItemLogprobs) -> Self {
            Self {
                content: Ok(value.content),
                refusal: Ok(value.refusal),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateModelResponseProperties {
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        prompt_cache_key: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompt_cache_options: ::std::result::Result<
            ::std::option::Option<super::PromptCacheOptionsParam>,
            ::std::string::String,
        >,
        prompt_cache_retention: ::std::result::Result<
            ::std::option::Option<super::CreateModelResponsePropertiesPromptCacheRetention>,
            ::std::string::String,
        >,
        safety_identifier: ::std::result::Result<
            ::std::option::Option<super::CreateModelResponsePropertiesSafetyIdentifier>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        top_logprobs: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        user: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateModelResponseProperties {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                prompt_cache_key: Ok(Default::default()),
                prompt_cache_options: Ok(Default::default()),
                prompt_cache_retention: Ok(Default::default()),
                safety_identifier: Ok(Default::default()),
                temperature: Ok(Default::default()),
                top_logprobs: Ok(Default::default()),
                top_p: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl CreateModelResponseProperties {
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
        pub fn prompt_cache_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_cache_key: {e}"));
            self
        }
        pub fn prompt_cache_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheOptionsParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_options = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_options: {e}")
            });
            self
        }
        pub fn prompt_cache_retention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateModelResponsePropertiesPromptCacheRetention>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_retention = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_retention: {e}")
            });
            self
        }
        pub fn safety_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateModelResponsePropertiesSafetyIdentifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.safety_identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for safety_identifier: {e}"));
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
        pub fn top_logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_logprobs: {e}"));
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
    impl ::std::convert::TryFrom<CreateModelResponseProperties>
        for super::CreateModelResponseProperties
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateModelResponseProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                prompt_cache_key: value.prompt_cache_key?,
                prompt_cache_options: value.prompt_cache_options?,
                prompt_cache_retention: value.prompt_cache_retention?,
                safety_identifier: value.safety_identifier?,
                temperature: value.temperature?,
                top_logprobs: value.top_logprobs?,
                top_p: value.top_p?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::CreateModelResponseProperties> for CreateModelResponseProperties {
        fn from(value: super::CreateModelResponseProperties) -> Self {
            Self {
                metadata: Ok(value.metadata),
                prompt_cache_key: Ok(value.prompt_cache_key),
                prompt_cache_options: Ok(value.prompt_cache_options),
                prompt_cache_retention: Ok(value.prompt_cache_retention),
                safety_identifier: Ok(value.safety_identifier),
                temperature: Ok(value.temperature),
                top_logprobs: Ok(value.top_logprobs),
                top_p: Ok(value.top_p),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomToolChatCompletions {
        custom: ::std::result::Result<super::CustomToolProperties, ::std::string::String>,
        type_: ::std::result::Result<super::CustomToolChatCompletionsType, ::std::string::String>,
    }
    impl ::std::default::Default for CustomToolChatCompletions {
        fn default() -> Self {
            Self {
                custom: Err("no value supplied for custom".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CustomToolChatCompletions {
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomToolProperties>,
            T::Error: ::std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for custom: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomToolChatCompletionsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CustomToolChatCompletions> for super::CustomToolChatCompletions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomToolChatCompletions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom: value.custom?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CustomToolChatCompletions> for CustomToolChatCompletions {
        fn from(value: super::CustomToolChatCompletions) -> Self {
            Self {
                custom: Ok(value.custom),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomToolProperties {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format: ::std::result::Result<
            ::std::option::Option<super::CustomToolPropertiesFormat>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CustomToolProperties {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl CustomToolProperties {
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
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CustomToolPropertiesFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
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
    impl ::std::convert::TryFrom<CustomToolProperties> for super::CustomToolProperties {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomToolProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                format: value.format?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CustomToolProperties> for CustomToolProperties {
        fn from(value: super::CustomToolProperties) -> Self {
            Self {
                description: Ok(value.description),
                format: Ok(value.format),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FunctionObject {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parameters: ::std::result::Result<
            ::std::option::Option<super::FunctionParameters>,
            ::std::string::String,
        >,
        strict: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for FunctionObject {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                parameters: Ok(Default::default()),
                strict: Ok(Default::default()),
            }
        }
    }
    impl FunctionObject {
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
        pub fn parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FunctionParameters>>,
            T::Error: ::std::fmt::Display,
        {
            self.parameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parameters: {e}"));
            self
        }
        pub fn strict<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.strict = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strict: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FunctionObject> for super::FunctionObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FunctionObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                parameters: value.parameters?,
                strict: value.strict?,
            })
        }
    }
    impl ::std::convert::From<super::FunctionObject> for FunctionObject {
        fn from(value: super::FunctionObject) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                parameters: Ok(value.parameters),
                strict: Ok(value.strict),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetChatCompletionMessagesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::GetChatCompletionMessagesParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetChatCompletionMessagesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl GetChatCompletionMessagesParams {
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GetChatCompletionMessagesParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetChatCompletionMessagesParams>
        for super::GetChatCompletionMessagesParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetChatCompletionMessagesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::GetChatCompletionMessagesParams>
        for GetChatCompletionMessagesParams
    {
        fn from(value: super::GetChatCompletionMessagesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema: ::std::result::Result<
            ::std::option::Option<super::ResponseFormatJsonSchemaSchema>,
            ::std::string::String,
        >,
        strict: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for JsonSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                schema: Ok(Default::default()),
                strict: Ok(Default::default()),
            }
        }
    }
    impl JsonSchema {
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
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ResponseFormatJsonSchemaSchema>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn strict<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.strict = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strict: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonSchema> for super::JsonSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                schema: value.schema?,
                strict: value.strict?,
            })
        }
    }
    impl ::std::convert::From<super::JsonSchema> for JsonSchema {
        fn from(value: super::JsonSchema) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                schema: Ok(value.schema),
                strict: Ok(value.strict),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListChatCompletionsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<super::ListChatCompletionsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListChatCompletionsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListChatCompletionsParams {
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListChatCompletionsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListChatCompletionsParams> for super::ListChatCompletionsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListChatCompletionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                metadata: value.metadata?,
                model: value.model?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListChatCompletionsParams> for ListChatCompletionsParams {
        fn from(value: super::ListChatCompletionsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModelResponseProperties {
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        prompt_cache_key: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prompt_cache_retention: ::std::result::Result<
            ::std::option::Option<super::ModelResponsePropertiesPromptCacheRetention>,
            ::std::string::String,
        >,
        safety_identifier: ::std::result::Result<
            ::std::option::Option<super::ModelResponsePropertiesSafetyIdentifier>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        top_logprobs: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        user: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModelResponseProperties {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                prompt_cache_key: Ok(Default::default()),
                prompt_cache_retention: Ok(Default::default()),
                safety_identifier: Ok(Default::default()),
                temperature: Ok(Default::default()),
                top_logprobs: Ok(Default::default()),
                top_p: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl ModelResponseProperties {
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
        pub fn prompt_cache_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_cache_key: {e}"));
            self
        }
        pub fn prompt_cache_retention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModelResponsePropertiesPromptCacheRetention>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_cache_retention = value.try_into().map_err(|e| {
                format!("error converting supplied value for prompt_cache_retention: {e}")
            });
            self
        }
        pub fn safety_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ModelResponsePropertiesSafetyIdentifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.safety_identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for safety_identifier: {e}"));
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
        pub fn top_logprobs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.top_logprobs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_logprobs: {e}"));
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
    impl ::std::convert::TryFrom<ModelResponseProperties> for super::ModelResponseProperties {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModelResponseProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                prompt_cache_key: value.prompt_cache_key?,
                prompt_cache_retention: value.prompt_cache_retention?,
                safety_identifier: value.safety_identifier?,
                temperature: value.temperature?,
                top_logprobs: value.top_logprobs?,
                top_p: value.top_p?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::ModelResponseProperties> for ModelResponseProperties {
        fn from(value: super::ModelResponseProperties) -> Self {
            Self {
                metadata: Ok(value.metadata),
                prompt_cache_key: Ok(value.prompt_cache_key),
                prompt_cache_retention: Ok(value.prompt_cache_retention),
                safety_identifier: Ok(value.safety_identifier),
                temperature: Ok(value.temperature),
                top_logprobs: Ok(value.top_logprobs),
                top_p: Ok(value.top_p),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationConfigParam {
        mode: ::std::result::Result<super::ModerationMode, ::std::string::String>,
    }
    impl ::std::default::Default for ModerationConfigParam {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
            }
        }
    }
    impl ModerationConfigParam {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModerationMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationConfigParam> for super::ModerationConfigParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationConfigParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { mode: value.mode? })
        }
    }
    impl ::std::convert::From<super::ModerationConfigParam> for ModerationConfigParam {
        fn from(value: super::ModerationConfigParam) -> Self {
            Self {
                mode: Ok(value.mode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationParam {
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        policy: ::std::result::Result<
            ::std::option::Option<super::ModerationPolicyParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModerationParam {
        fn default() -> Self {
            Self {
                model: Err("no value supplied for model".to_string()),
                policy: Ok(Default::default()),
            }
        }
    }
    impl ModerationParam {
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
        pub fn policy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModerationPolicyParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.policy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for policy: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationParam> for super::ModerationParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
                policy: value.policy?,
            })
        }
    }
    impl ::std::convert::From<super::ModerationParam> for ModerationParam {
        fn from(value: super::ModerationParam) -> Self {
            Self {
                model: Ok(value.model),
                policy: Ok(value.policy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationPolicyParam {
        input: ::std::result::Result<
            ::std::option::Option<super::ModerationConfigParam>,
            ::std::string::String,
        >,
        output: ::std::result::Result<
            ::std::option::Option<super::ModerationConfigParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ModerationPolicyParam {
        fn default() -> Self {
            Self {
                input: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl ModerationPolicyParam {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModerationConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ModerationConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationPolicyParam> for super::ModerationPolicyParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationPolicyParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::ModerationPolicyParam> for ModerationPolicyParam {
        fn from(value: super::ModerationPolicyParam) -> Self {
            Self {
                input: Ok(value.input),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModerationResultBody {
        categories: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, bool>,
            ::std::string::String,
        >,
        category_applied_input_types: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<super::ModerationInputType>,
            >,
            ::std::string::String,
        >,
        category_scores: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, f64>,
            ::std::string::String,
        >,
        flagged: ::std::result::Result<bool, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::ModerationResultBodyType, ::std::string::String>,
    }
    impl ::std::default::Default for ModerationResultBody {
        fn default() -> Self {
            Self {
                categories: Err("no value supplied for categories".to_string()),
                category_applied_input_types: Err(
                    "no value supplied for category_applied_input_types".to_string(),
                ),
                category_scores: Err("no value supplied for category_scores".to_string()),
                flagged: Err("no value supplied for flagged".to_string()),
                model: Err("no value supplied for model".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ModerationResultBody {
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {e}"));
            self
        }
        pub fn category_applied_input_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::ModerationInputType>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.category_applied_input_types = value.try_into().map_err(|e| {
                format!("error converting supplied value for category_applied_input_types: {e}")
            });
            self
        }
        pub fn category_scores<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.category_scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category_scores: {e}"));
            self
        }
        pub fn flagged<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.flagged = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flagged: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModerationResultBodyType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModerationResultBody> for super::ModerationResultBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModerationResultBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                categories: value.categories?,
                category_applied_input_types: value.category_applied_input_types?,
                category_scores: value.category_scores?,
                flagged: value.flagged?,
                model: value.model?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ModerationResultBody> for ModerationResultBody {
        fn from(value: super::ModerationResultBody) -> Self {
            Self {
                categories: Ok(value.categories),
                category_applied_input_types: Ok(value.category_applied_input_types),
                category_scores: Ok(value.category_scores),
                flagged: Ok(value.flagged),
                model: Ok(value.model),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PredictionContent {
        content: ::std::result::Result<super::PredictionContentContent, ::std::string::String>,
        type_: ::std::result::Result<super::PredictionContentType, ::std::string::String>,
    }
    impl ::std::default::Default for PredictionContent {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl PredictionContent {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PredictionContentContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PredictionContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PredictionContent> for super::PredictionContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PredictionContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::PredictionContent> for PredictionContent {
        fn from(value: super::PredictionContent) -> Self {
            Self {
                content: Ok(value.content),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptCacheBreakpointParam {
        mode: ::std::result::Result<super::PromptCacheBreakpointParamMode, ::std::string::String>,
    }
    impl ::std::default::Default for PromptCacheBreakpointParam {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
            }
        }
    }
    impl PromptCacheBreakpointParam {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PromptCacheBreakpointParamMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptCacheBreakpointParam> for super::PromptCacheBreakpointParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptCacheBreakpointParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { mode: value.mode? })
        }
    }
    impl ::std::convert::From<super::PromptCacheBreakpointParam> for PromptCacheBreakpointParam {
        fn from(value: super::PromptCacheBreakpointParam) -> Self {
            Self {
                mode: Ok(value.mode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptCacheOptionsParam {
        mode: ::std::result::Result<
            ::std::option::Option<super::PromptCacheModeEnum>,
            ::std::string::String,
        >,
        ttl: ::std::result::Result<
            ::std::option::Option<super::PromptCacheTtlEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PromptCacheOptionsParam {
        fn default() -> Self {
            Self {
                mode: Ok(Default::default()),
                ttl: Ok(Default::default()),
            }
        }
    }
    impl PromptCacheOptionsParam {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheModeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn ttl<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheTtlEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.ttl = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ttl: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptCacheOptionsParam> for super::PromptCacheOptionsParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptCacheOptionsParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                ttl: value.ttl?,
            })
        }
    }
    impl ::std::convert::From<super::PromptCacheOptionsParam> for PromptCacheOptionsParam {
        fn from(value: super::PromptCacheOptionsParam) -> Self {
            Self {
                mode: Ok(value.mode),
                ttl: Ok(value.ttl),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponseFormatJsonObject {
        type_: ::std::result::Result<super::ResponseFormatJsonObjectType, ::std::string::String>,
    }
    impl ::std::default::Default for ResponseFormatJsonObject {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ResponseFormatJsonObject {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResponseFormatJsonObjectType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponseFormatJsonObject> for super::ResponseFormatJsonObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponseFormatJsonObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ResponseFormatJsonObject> for ResponseFormatJsonObject {
        fn from(value: super::ResponseFormatJsonObject) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponseFormatJsonSchema {
        json_schema: ::std::result::Result<super::JsonSchema, ::std::string::String>,
        type_: ::std::result::Result<super::ResponseFormatJsonSchemaType, ::std::string::String>,
    }
    impl ::std::default::Default for ResponseFormatJsonSchema {
        fn default() -> Self {
            Self {
                json_schema: Err("no value supplied for json_schema".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ResponseFormatJsonSchema {
        pub fn json_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonSchema>,
            T::Error: ::std::fmt::Display,
        {
            self.json_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for json_schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResponseFormatJsonSchemaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponseFormatJsonSchema> for super::ResponseFormatJsonSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponseFormatJsonSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                json_schema: value.json_schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ResponseFormatJsonSchema> for ResponseFormatJsonSchema {
        fn from(value: super::ResponseFormatJsonSchema) -> Self {
            Self {
                json_schema: Ok(value.json_schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponseFormatText {
        type_: ::std::result::Result<super::ResponseFormatTextType, ::std::string::String>,
    }
    impl ::std::default::Default for ResponseFormatText {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ResponseFormatText {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResponseFormatTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponseFormatText> for super::ResponseFormatText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponseFormatText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ResponseFormatText> for ResponseFormatText {
        fn from(value: super::ResponseFormatText) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateChatCompletionRequest {
        metadata: ::std::result::Result<super::Metadata, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateChatCompletionRequest {
        fn default() -> Self {
            Self {
                metadata: Err("no value supplied for metadata".to_string()),
            }
        }
    }
    impl UpdateChatCompletionRequest {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Metadata>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateChatCompletionRequest> for super::UpdateChatCompletionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateChatCompletionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateChatCompletionRequest> for UpdateChatCompletionRequest {
        fn from(value: super::UpdateChatCompletionRequest) -> Self {
            Self {
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebSearch {
        search_context_size: ::std::result::Result<
            ::std::option::Option<super::WebSearchContextSize>,
            ::std::string::String,
        >,
        user_location: ::std::result::Result<
            ::std::option::Option<super::WebSearchUserLocation>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebSearch {
        fn default() -> Self {
            Self {
                search_context_size: Ok(Default::default()),
                user_location: Ok(Default::default()),
            }
        }
    }
    impl WebSearch {
        pub fn search_context_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebSearchContextSize>>,
            T::Error: ::std::fmt::Display,
        {
            self.search_context_size = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_context_size: {e}")
            });
            self
        }
        pub fn user_location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WebSearchUserLocation>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_location: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebSearch> for super::WebSearch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebSearch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                search_context_size: value.search_context_size?,
                user_location: value.user_location?,
            })
        }
    }
    impl ::std::convert::From<super::WebSearch> for WebSearch {
        fn from(value: super::WebSearch) -> Self {
            Self {
                search_context_size: Ok(value.search_context_size),
                user_location: Ok(value.user_location),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebSearchLocation {
        city: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        country: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        region: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        timezone: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebSearchLocation {
        fn default() -> Self {
            Self {
                city: Ok(Default::default()),
                country: Ok(Default::default()),
                region: Ok(Default::default()),
                timezone: Ok(Default::default()),
            }
        }
    }
    impl WebSearchLocation {
        pub fn city<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.city = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for city: {e}"));
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for country: {e}"));
            self
        }
        pub fn region<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.region = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for region: {e}"));
            self
        }
        pub fn timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.timezone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timezone: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebSearchLocation> for super::WebSearchLocation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebSearchLocation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                city: value.city?,
                country: value.country?,
                region: value.region?,
                timezone: value.timezone?,
            })
        }
    }
    impl ::std::convert::From<super::WebSearchLocation> for WebSearchLocation {
        fn from(value: super::WebSearchLocation) -> Self {
            Self {
                city: Ok(value.city),
                country: Ok(value.country),
                region: Ok(value.region),
                timezone: Ok(value.timezone),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebSearchUserLocation {
        approximate: ::std::result::Result<super::WebSearchLocation, ::std::string::String>,
        type_: ::std::result::Result<super::WebSearchUserLocationType, ::std::string::String>,
    }
    impl ::std::default::Default for WebSearchUserLocation {
        fn default() -> Self {
            Self {
                approximate: Err("no value supplied for approximate".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl WebSearchUserLocation {
        pub fn approximate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebSearchLocation>,
            T::Error: ::std::fmt::Display,
        {
            self.approximate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for approximate: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebSearchUserLocationType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebSearchUserLocation> for super::WebSearchUserLocation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebSearchUserLocation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                approximate: value.approximate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::WebSearchUserLocation> for WebSearchUserLocation {
        fn from(value: super::WebSearchUserLocation) -> Self {
            Self {
                approximate: Ok(value.approximate),
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
