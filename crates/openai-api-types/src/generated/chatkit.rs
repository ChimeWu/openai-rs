pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, OrderEnum, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Indicates that a thread is active."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ActiveStatus {
    #[doc = "Status discriminator that is always `active`."]
    #[serde(rename = "type")]
    pub type_: ActiveStatusType,
}
impl ActiveStatus {
    pub fn builder() -> builder::ActiveStatus {
        Default::default()
    }
}
#[doc = "Status discriminator that is always `active`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ActiveStatusType {
    #[serde(rename = "active")]
    Active,
}
impl ::std::fmt::Display for ActiveStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
        }
    }
}
impl ::std::str::FromStr for ActiveStatusType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActiveStatusType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActiveStatusType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Assistant-authored message within a thread."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssistantMessageItem {
    #[doc = "Ordered assistant response segments."]
    pub content: ::std::vec::Vec<ResponseOutputText>,
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: AssistantMessageItemObject,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.assistant_message`."]
    #[serde(rename = "type")]
    pub type_: AssistantMessageItemType,
}
impl AssistantMessageItem {
    pub fn builder() -> builder::AssistantMessageItem {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AssistantMessageItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for AssistantMessageItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for AssistantMessageItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `chatkit.assistant_message`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AssistantMessageItemType {
    #[serde(rename = "chatkit.assistant_message")]
    ChatkitAssistantMessage,
}
impl ::std::fmt::Display for AssistantMessageItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitAssistantMessage => f.write_str("chatkit.assistant_message"),
        }
    }
}
impl ::std::str::FromStr for AssistantMessageItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.assistant_message" => Ok(Self::ChatkitAssistantMessage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssistantMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssistantMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Attachment metadata included on thread items."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Attachment {
    #[doc = "Identifier for the attachment."]
    pub id: ::std::string::String,
    #[doc = "MIME type of the attachment."]
    pub mime_type: ::std::string::String,
    #[doc = "Original display name for the attachment."]
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub preview_url: ::std::option::Option<::std::string::String>,
    #[doc = "Attachment discriminator."]
    #[serde(rename = "type")]
    pub type_: AttachmentType,
}
impl Attachment {
    pub fn builder() -> builder::Attachment {
        Default::default()
    }
}
#[doc = "`AttachmentType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AttachmentType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("image"),
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for AttachmentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image" => Ok(Self::Image),
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AttachmentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AttachmentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls whether ChatKit automatically generates thread titles."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AutomaticThreadTitlingParam {
    #[doc = "Enable automatic thread title generation. Defaults to true."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
}
impl AutomaticThreadTitlingParam {
    pub fn builder() -> builder::AutomaticThreadTitlingParam {
        Default::default()
    }
}
#[doc = "`CancelChatSessionMethodParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CancelChatSessionMethodParams {
    pub session_id: ::std::string::String,
}
impl CancelChatSessionMethodParams {
    pub fn builder() -> builder::CancelChatSessionMethodParams {
        Default::default()
    }
}
#[doc = "`ChatKitApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatKitApiSchema {
    #[serde(rename = "CancelChatSessionMethodError400")]
    pub cancel_chat_session_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelChatSessionMethodError404")]
    pub cancel_chat_session_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelChatSessionMethodError429")]
    pub cancel_chat_session_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelChatSessionMethodParams")]
    pub cancel_chat_session_method_params: CancelChatSessionMethodParams,
    #[serde(rename = "CancelChatSessionMethodResponse")]
    pub cancel_chat_session_method_response: ChatSessionResource,
    #[serde(rename = "CreateChatSessionMethodError400")]
    pub create_chat_session_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateChatSessionMethodError404")]
    pub create_chat_session_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateChatSessionMethodError429")]
    pub create_chat_session_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateChatSessionMethodRequest")]
    pub create_chat_session_method_request: CreateChatSessionBody,
    #[serde(rename = "CreateChatSessionMethodResponse")]
    pub create_chat_session_method_response: ChatSessionResource,
    #[serde(rename = "DeleteThreadMethodError400")]
    pub delete_thread_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteThreadMethodError404")]
    pub delete_thread_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteThreadMethodError429")]
    pub delete_thread_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteThreadMethodParams")]
    pub delete_thread_method_params: DeleteThreadMethodParams,
    #[serde(rename = "DeleteThreadMethodResponse")]
    pub delete_thread_method_response: DeletedThreadResource,
    #[serde(rename = "GetThreadMethodError400")]
    pub get_thread_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetThreadMethodError404")]
    pub get_thread_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetThreadMethodError429")]
    pub get_thread_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetThreadMethodParams")]
    pub get_thread_method_params: GetThreadMethodParams,
    #[serde(rename = "GetThreadMethodResponse")]
    pub get_thread_method_response: ThreadResource,
    #[serde(rename = "ListThreadItemsMethodError400")]
    pub list_thread_items_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadItemsMethodError404")]
    pub list_thread_items_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadItemsMethodError429")]
    pub list_thread_items_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadItemsMethodParams")]
    pub list_thread_items_method_params: ListThreadItemsMethodParams,
    #[serde(rename = "ListThreadItemsMethodResponse")]
    pub list_thread_items_method_response: ThreadItemListResource,
    #[serde(rename = "ListThreadsMethodError400")]
    pub list_threads_method_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadsMethodError404")]
    pub list_threads_method_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadsMethodError429")]
    pub list_threads_method_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListThreadsMethodParams")]
    pub list_threads_method_params: ListThreadsMethodParams,
    #[serde(rename = "ListThreadsMethodResponse")]
    pub list_threads_method_response: ThreadListResource,
}
impl ChatKitApiSchema {
    pub fn builder() -> builder::ChatKitApiSchema {
        Default::default()
    }
}
#[doc = "Automatic thread title preferences for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionAutomaticThreadTitling {
    #[doc = "Whether automatic thread titling is enabled."]
    pub enabled: bool,
}
impl ChatSessionAutomaticThreadTitling {
    pub fn builder() -> builder::ChatSessionAutomaticThreadTitling {
        Default::default()
    }
}
#[doc = "ChatKit configuration for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionChatkitConfiguration {
    #[doc = "Automatic thread titling preferences."]
    pub automatic_thread_titling: ChatSessionAutomaticThreadTitling,
    #[doc = "Upload settings for the session."]
    pub file_upload: ChatSessionFileUpload,
    #[doc = "History retention configuration."]
    pub history: ChatSessionHistory,
}
impl ChatSessionChatkitConfiguration {
    pub fn builder() -> builder::ChatSessionChatkitConfiguration {
        Default::default()
    }
}
#[doc = "Upload permissions and limits applied to the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionFileUpload {
    #[doc = "Indicates if uploads are enabled for the session."]
    pub enabled: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_file_size: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_files: ::std::option::Option<i64>,
}
impl ChatSessionFileUpload {
    pub fn builder() -> builder::ChatSessionFileUpload {
        Default::default()
    }
}
#[doc = "History retention preferences returned for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionHistory {
    #[doc = "Indicates if chat history is persisted for the session."]
    pub enabled: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub recent_threads: ::std::option::Option<i64>,
}
impl ChatSessionHistory {
    pub fn builder() -> builder::ChatSessionHistory {
        Default::default()
    }
}
#[doc = "Active per-minute request limit for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionRateLimits {
    #[doc = "Maximum allowed requests per one-minute window."]
    pub max_requests_per_1_minute: i64,
}
impl ChatSessionRateLimits {
    pub fn builder() -> builder::ChatSessionRateLimits {
        Default::default()
    }
}
#[doc = "Represents a ChatKit session and its resolved configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatSessionResource {
    #[doc = "Resolved ChatKit feature configuration for the session."]
    pub chatkit_configuration: ChatSessionChatkitConfiguration,
    #[doc = "Ephemeral client secret that authenticates session requests."]
    pub client_secret: ::std::string::String,
    #[doc = "Unix timestamp (in seconds) for when the session expires."]
    pub expires_at: i64,
    #[doc = "Identifier for the ChatKit session."]
    pub id: ::std::string::String,
    #[doc = "Convenience copy of the per-minute request limit."]
    pub max_requests_per_1_minute: i64,
    #[doc = "Type discriminator that is always `chatkit.session`."]
    pub object: ChatSessionResourceObject,
    #[doc = "Resolved rate limit values."]
    pub rate_limits: ChatSessionRateLimits,
    #[doc = "Current lifecycle state of the session."]
    pub status: ChatSessionStatus,
    #[doc = "User identifier associated with the session."]
    pub user: ::std::string::String,
    #[doc = "Workflow metadata for the session."]
    pub workflow: ChatkitWorkflow,
}
impl ChatSessionResource {
    pub fn builder() -> builder::ChatSessionResource {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.session`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ChatSessionResourceObject {
    #[serde(rename = "chatkit.session")]
    ChatkitSession,
}
impl ::std::fmt::Display for ChatSessionResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitSession => f.write_str("chatkit.session"),
        }
    }
}
impl ::std::str::FromStr for ChatSessionResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.session" => Ok(Self::ChatkitSession),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatSessionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatSessionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChatSessionStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ChatSessionStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ::std::fmt::Display for ChatSessionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Expired => f.write_str("expired"),
            Self::Cancelled => f.write_str("cancelled"),
        }
    }
}
impl ::std::str::FromStr for ChatSessionStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatSessionStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatSessionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Optional per-session configuration settings for ChatKit behavior."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ChatkitConfigurationParam {
    #[doc = "Configuration for automatic thread titling. When omitted, automatic thread titling is enabled by default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub automatic_thread_titling: ::std::option::Option<AutomaticThreadTitlingParam>,
    #[doc = "Configuration for upload enablement and limits. When omitted, uploads are disabled by default (max_files 10, max_file_size 512 MB)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_upload: ::std::option::Option<FileUploadParam>,
    #[doc = "Configuration for chat history retention. When omitted, history is enabled by default with no limit on recent_threads (null)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub history: ::std::option::Option<HistoryParam>,
}
impl ChatkitConfigurationParam {
    pub fn builder() -> builder::ChatkitConfigurationParam {
        Default::default()
    }
}
#[doc = "Workflow metadata and state returned for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatkitWorkflow {
    #[doc = "Identifier of the workflow backing the session."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub state_variables: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ChatkitWorkflowStateVariablesValue>,
    >,
    #[doc = "Tracing settings applied to the workflow."]
    pub tracing: ChatkitWorkflowTracing,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ChatkitWorkflow {
    pub fn builder() -> builder::ChatkitWorkflow {
        Default::default()
    }
}
#[doc = "`ChatkitWorkflowStateVariablesValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChatkitWorkflowStateVariablesValue {
    String(::std::string::String),
    Integer(i64),
    Boolean(bool),
    Number(f64),
    Null,
}
impl ::std::convert::From<i64> for ChatkitWorkflowStateVariablesValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<bool> for ChatkitWorkflowStateVariablesValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for ChatkitWorkflowStateVariablesValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "Controls diagnostic tracing during the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ChatkitWorkflowTracing {
    #[doc = "Indicates whether tracing is enabled."]
    pub enabled: bool,
}
impl ChatkitWorkflowTracing {
    pub fn builder() -> builder::ChatkitWorkflowTracing {
        Default::default()
    }
}
#[doc = "Record of a client side tool invocation initiated by the assistant."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ClientToolCallItem {
    #[doc = "JSON-encoded arguments that were sent to the tool."]
    pub arguments: ::std::string::String,
    #[doc = "Identifier for the client tool call."]
    pub call_id: ::std::string::String,
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[doc = "Tool name that was invoked."]
    pub name: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: ClientToolCallItemObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub output: ::std::option::Option<::std::string::String>,
    #[doc = "Execution status for the tool call."]
    pub status: ClientToolCallStatus,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.client_tool_call`."]
    #[serde(rename = "type")]
    pub type_: ClientToolCallItemType,
}
impl ClientToolCallItem {
    pub fn builder() -> builder::ClientToolCallItem {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClientToolCallItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for ClientToolCallItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for ClientToolCallItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientToolCallItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientToolCallItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `chatkit.client_tool_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClientToolCallItemType {
    #[serde(rename = "chatkit.client_tool_call")]
    ChatkitClientToolCall,
}
impl ::std::fmt::Display for ClientToolCallItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitClientToolCall => f.write_str("chatkit.client_tool_call"),
        }
    }
}
impl ::std::str::FromStr for ClientToolCallItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.client_tool_call" => Ok(Self::ChatkitClientToolCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientToolCallItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientToolCallItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClientToolCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClientToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for ClientToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for ClientToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClientToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClientToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Indicates that a thread has been closed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ClosedStatus {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "Status discriminator that is always `closed`."]
    #[serde(rename = "type")]
    pub type_: ClosedStatusType,
}
impl ClosedStatus {
    pub fn builder() -> builder::ClosedStatus {
        Default::default()
    }
}
#[doc = "Status discriminator that is always `closed`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClosedStatusType {
    #[serde(rename = "closed")]
    Closed,
}
impl ::std::fmt::Display for ClosedStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => f.write_str("closed"),
        }
    }
}
impl ::std::str::FromStr for ClosedStatusType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClosedStatusType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClosedStatusType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Parameters for provisioning a new ChatKit session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateChatSessionBody {
    #[doc = "Optional overrides for ChatKit runtime configuration features"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chatkit_configuration: ::std::option::Option<ChatkitConfigurationParam>,
    #[doc = "Optional override for session expiration timing in seconds from creation. Defaults to 10 minutes."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<ExpiresAfterParam>,
    #[doc = "Optional override for per-minute request limits. When omitted, defaults to 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rate_limits: ::std::option::Option<RateLimitsParam>,
    #[doc = "A free-form string that identifies your end user; ensures this Session can access other objects that have the same `user` scope."]
    pub user: CreateChatSessionBodyUser,
    #[doc = "Workflow that powers the session."]
    pub workflow: WorkflowParam,
}
impl CreateChatSessionBody {
    pub fn builder() -> builder::CreateChatSessionBody {
        Default::default()
    }
}
#[doc = "A free-form string that identifies your end user; ensures this Session can access other objects that have the same `user` scope."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateChatSessionBodyUser(::std::string::String);
impl ::std::ops::Deref for CreateChatSessionBodyUser {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateChatSessionBodyUser> for ::std::string::String {
    fn from(value: CreateChatSessionBodyUser) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateChatSessionBodyUser {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatSessionBodyUser {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateChatSessionBodyUser {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateChatSessionBodyUser {
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
#[doc = "`DeleteThreadMethodParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteThreadMethodParams {
    pub thread_id: ::std::string::String,
}
impl DeleteThreadMethodParams {
    pub fn builder() -> builder::DeleteThreadMethodParams {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting a thread."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedThreadResource {
    #[doc = "Indicates that the thread has been deleted."]
    pub deleted: bool,
    #[doc = "Identifier of the deleted thread."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread.deleted`."]
    pub object: DeletedThreadResourceObject,
}
impl DeletedThreadResource {
    pub fn builder() -> builder::DeletedThreadResource {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedThreadResourceObject {
    #[serde(rename = "chatkit.thread.deleted")]
    ChatkitThreadDeleted,
}
impl ::std::fmt::Display for DeletedThreadResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadDeleted => f.write_str("chatkit.thread.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedThreadResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread.deleted" => Ok(Self::ChatkitThreadDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedThreadResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedThreadResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls when the session expires relative to an anchor timestamp."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ExpiresAfterParam {
    #[doc = "Base timestamp used to calculate expiration. Currently fixed to `created_at`."]
    pub anchor: ExpiresAfterParamAnchor,
    #[doc = "Number of seconds after the anchor when the session expires."]
    pub seconds: ::std::num::NonZeroU64,
}
impl ExpiresAfterParam {
    pub fn builder() -> builder::ExpiresAfterParam {
        Default::default()
    }
}
#[doc = "Base timestamp used to calculate expiration. Currently fixed to `created_at`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExpiresAfterParamAnchor {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl ::std::fmt::Display for ExpiresAfterParamAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
        }
    }
}
impl ::std::str::FromStr for ExpiresAfterParamAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExpiresAfterParamAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpiresAfterParamAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Annotation that references an uploaded file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileAnnotation {
    #[doc = "File attachment referenced by the annotation."]
    pub source: FileAnnotationSource,
    #[doc = "Type discriminator that is always `file` for this annotation."]
    #[serde(rename = "type")]
    pub type_: FileAnnotationType,
}
impl FileAnnotation {
    pub fn builder() -> builder::FileAnnotation {
        Default::default()
    }
}
#[doc = "Attachment source referenced by an annotation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileAnnotationSource {
    #[doc = "Filename referenced by the annotation."]
    pub filename: ::std::string::String,
    #[doc = "Type discriminator that is always `file`."]
    #[serde(rename = "type")]
    pub type_: FileAnnotationSourceType,
}
impl FileAnnotationSource {
    pub fn builder() -> builder::FileAnnotationSource {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileAnnotationSourceType {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for FileAnnotationSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for FileAnnotationSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileAnnotationSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileAnnotationSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `file` for this annotation."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileAnnotationType {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for FileAnnotationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for FileAnnotationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileAnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileAnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls whether users can upload files."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FileUploadParam {
    #[doc = "Enable uploads for this session. Defaults to false."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[doc = "Maximum size in megabytes for each uploaded file. Defaults to 512 MB, which is the maximum allowable size."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_file_size: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Maximum number of files that can be uploaded to the session. Defaults to 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_files: ::std::option::Option<::std::num::NonZeroU64>,
}
impl FileUploadParam {
    pub fn builder() -> builder::FileUploadParam {
        Default::default()
    }
}
#[doc = "`GetThreadMethodParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetThreadMethodParams {
    pub thread_id: ::std::string::String,
}
impl GetThreadMethodParams {
    pub fn builder() -> builder::GetThreadMethodParams {
        Default::default()
    }
}
#[doc = "Controls how much historical context is retained for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct HistoryParam {
    #[doc = "Enables chat users to access previous ChatKit threads. Defaults to true."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[doc = "Number of recent ChatKit threads users have access to. Defaults to unlimited when unset."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub recent_threads: ::std::option::Option<::std::num::NonZeroU64>,
}
impl HistoryParam {
    pub fn builder() -> builder::HistoryParam {
        Default::default()
    }
}
#[doc = "Model and tool overrides applied when generating the assistant response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InferenceOptions {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub tool_choice: ::std::option::Option<ToolChoice>,
}
impl InferenceOptions {
    pub fn builder() -> builder::InferenceOptions {
        Default::default()
    }
}
#[doc = "`ListThreadItemsMethodParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListThreadItemsMethodParams {
    #[doc = "List items created after this thread item ID. Defaults to null for the first page."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[doc = "List items created before this thread item ID. Defaults to null for the newest results."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::OrderEnum>,
    pub thread_id: ::std::string::String,
}
impl ListThreadItemsMethodParams {
    pub fn builder() -> builder::ListThreadItemsMethodParams {
        Default::default()
    }
}
#[doc = "`ListThreadsMethodParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListThreadsMethodParams {
    #[doc = "List items created after this thread item ID. Defaults to null for the first page."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[doc = "List items created before this thread item ID. Defaults to null for the newest results."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::OrderEnum>,
    #[doc = "Filter threads that belong to this user identifier. Defaults to null to return all users."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<ListThreadsMethodParamsUser>,
}
impl ListThreadsMethodParams {
    pub fn builder() -> builder::ListThreadsMethodParams {
        Default::default()
    }
}
#[doc = "Filter threads that belong to this user identifier. Defaults to null to return all users."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListThreadsMethodParamsUser(::std::string::String);
impl ::std::ops::Deref for ListThreadsMethodParamsUser {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListThreadsMethodParamsUser> for ::std::string::String {
    fn from(value: ListThreadsMethodParamsUser) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListThreadsMethodParamsUser {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListThreadsMethodParamsUser {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListThreadsMethodParamsUser {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListThreadsMethodParamsUser {
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
#[doc = "Indicates that a thread is locked and cannot accept new input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LockedStatus {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "Status discriminator that is always `locked`."]
    #[serde(rename = "type")]
    pub type_: LockedStatusType,
}
impl LockedStatus {
    pub fn builder() -> builder::LockedStatus {
        Default::default()
    }
}
#[doc = "Status discriminator that is always `locked`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LockedStatusType {
    #[serde(rename = "locked")]
    Locked,
}
impl ::std::fmt::Display for LockedStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Locked => f.write_str("locked"),
        }
    }
}
impl ::std::str::FromStr for LockedStatusType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "locked" => Ok(Self::Locked),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LockedStatusType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LockedStatusType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls request rate limits for the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitsParam {
    #[doc = "Maximum number of requests allowed per minute for the session. Defaults to 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_requests_per_1_minute: ::std::option::Option<::std::num::NonZeroU64>,
}
impl RateLimitsParam {
    pub fn builder() -> builder::RateLimitsParam {
        Default::default()
    }
}
#[doc = "Assistant response text accompanied by optional annotations."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseOutputText {
    #[doc = "Ordered list of annotations attached to the response text."]
    pub annotations: ::std::vec::Vec<ResponseOutputTextAnnotationsItem>,
    #[doc = "Assistant generated text."]
    pub text: ::std::string::String,
    #[doc = "Type discriminator that is always `output_text`."]
    #[serde(rename = "type")]
    pub type_: ResponseOutputTextType,
}
impl ResponseOutputText {
    pub fn builder() -> builder::ResponseOutputText {
        Default::default()
    }
}
#[doc = "Annotation object describing a cited source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponseOutputTextAnnotationsItem {
    FileAnnotation(FileAnnotation),
    UrlAnnotation(UrlAnnotation),
}
impl ::std::convert::From<FileAnnotation> for ResponseOutputTextAnnotationsItem {
    fn from(value: FileAnnotation) -> Self {
        Self::FileAnnotation(value)
    }
}
impl ::std::convert::From<UrlAnnotation> for ResponseOutputTextAnnotationsItem {
    fn from(value: UrlAnnotation) -> Self {
        Self::UrlAnnotation(value)
    }
}
#[doc = "Type discriminator that is always `output_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseOutputTextType {
    #[serde(rename = "output_text")]
    OutputText,
}
impl ::std::fmt::Display for ResponseOutputTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputText => f.write_str("output_text"),
        }
    }
}
impl ::std::str::FromStr for ResponseOutputTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_text" => Ok(Self::OutputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseOutputTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseOutputTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Collection of workflow tasks grouped together in the thread."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TaskGroupItem {
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: TaskGroupItemObject,
    #[doc = "Tasks included in the group."]
    pub tasks: ::std::vec::Vec<TaskGroupTask>,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.task_group`."]
    #[serde(rename = "type")]
    pub type_: TaskGroupItemType,
}
impl TaskGroupItem {
    pub fn builder() -> builder::TaskGroupItem {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskGroupItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for TaskGroupItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for TaskGroupItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskGroupItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskGroupItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `chatkit.task_group`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskGroupItemType {
    #[serde(rename = "chatkit.task_group")]
    ChatkitTaskGroup,
}
impl ::std::fmt::Display for TaskGroupItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitTaskGroup => f.write_str("chatkit.task_group"),
        }
    }
}
impl ::std::str::FromStr for TaskGroupItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.task_group" => Ok(Self::ChatkitTaskGroup),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskGroupItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskGroupItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Task entry that appears within a TaskGroup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TaskGroupTask {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub heading: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "Subtype for the grouped task."]
    #[serde(rename = "type")]
    pub type_: TaskType,
}
impl TaskGroupTask {
    pub fn builder() -> builder::TaskGroupTask {
        Default::default()
    }
}
#[doc = "Task emitted by the workflow to show progress and status updates."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TaskItem {
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub heading: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: TaskItemObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "Subtype for the task."]
    pub task_type: TaskType,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.task`."]
    #[serde(rename = "type")]
    pub type_: TaskItemType,
}
impl TaskItem {
    pub fn builder() -> builder::TaskItem {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for TaskItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for TaskItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `chatkit.task`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskItemType {
    #[serde(rename = "chatkit.task")]
    ChatkitTask,
}
impl ::std::fmt::Display for TaskItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitTask => f.write_str("chatkit.task"),
        }
    }
}
impl ::std::str::FromStr for TaskItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.task" => Ok(Self::ChatkitTask),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TaskType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskType {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "thought")]
    Thought,
}
impl ::std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
            Self::Thought => f.write_str("thought"),
        }
    }
}
impl ::std::str::FromStr for TaskType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            "thought" => Ok(Self::Thought),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ThreadItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ThreadItem {
    UserMessageItem(UserMessageItem),
    AssistantMessageItem(AssistantMessageItem),
    WidgetMessageItem(WidgetMessageItem),
    ClientToolCallItem(ClientToolCallItem),
    TaskItem(TaskItem),
    TaskGroupItem(TaskGroupItem),
}
impl ::std::convert::From<UserMessageItem> for ThreadItem {
    fn from(value: UserMessageItem) -> Self {
        Self::UserMessageItem(value)
    }
}
impl ::std::convert::From<AssistantMessageItem> for ThreadItem {
    fn from(value: AssistantMessageItem) -> Self {
        Self::AssistantMessageItem(value)
    }
}
impl ::std::convert::From<WidgetMessageItem> for ThreadItem {
    fn from(value: WidgetMessageItem) -> Self {
        Self::WidgetMessageItem(value)
    }
}
impl ::std::convert::From<ClientToolCallItem> for ThreadItem {
    fn from(value: ClientToolCallItem) -> Self {
        Self::ClientToolCallItem(value)
    }
}
impl ::std::convert::From<TaskItem> for ThreadItem {
    fn from(value: TaskItem) -> Self {
        Self::TaskItem(value)
    }
}
impl ::std::convert::From<TaskGroupItem> for ThreadItem {
    fn from(value: TaskGroupItem) -> Self {
        Self::TaskGroupItem(value)
    }
}
#[doc = "A paginated list of thread items rendered for the ChatKit API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ThreadItemListResource {
    #[doc = "A list of items"]
    pub data: ::std::vec::Vec<ThreadItem>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of object returned, must be `list`."]
    pub object: ThreadItemListResourceObject,
}
impl ThreadItemListResource {
    pub fn builder() -> builder::ThreadItemListResource {
        Default::default()
    }
}
#[doc = "The type of object returned, must be `list`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadItemListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ThreadItemListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ThreadItemListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadItemListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadItemListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A paginated list of ChatKit threads."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ThreadListResource {
    #[doc = "A list of items"]
    pub data: ::std::vec::Vec<ThreadResource>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of object returned, must be `list`."]
    pub object: ThreadListResourceObject,
}
impl ThreadListResource {
    pub fn builder() -> builder::ThreadListResource {
        Default::default()
    }
}
#[doc = "The type of object returned, must be `list`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ThreadListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ThreadListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a ChatKit thread and its current status."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ThreadResource {
    #[doc = "Unix timestamp (in seconds) for when the thread was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread`."]
    pub object: ThreadResourceObject,
    #[doc = "Current status for the thread. Defaults to `active` for newly created threads."]
    pub status: ThreadResourceStatus,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "Free-form string that identifies your end user who owns the thread."]
    pub user: ::std::string::String,
}
impl ThreadResource {
    pub fn builder() -> builder::ThreadResource {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadResourceObject {
    #[serde(rename = "chatkit.thread")]
    ChatkitThread,
}
impl ::std::fmt::Display for ThreadResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThread => f.write_str("chatkit.thread"),
        }
    }
}
impl ::std::str::FromStr for ThreadResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread" => Ok(Self::ChatkitThread),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Current status for the thread. Defaults to `active` for newly created threads."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ThreadResourceStatus {
    ActiveStatus(ActiveStatus),
    LockedStatus(LockedStatus),
    ClosedStatus(ClosedStatus),
}
impl ::std::convert::From<ActiveStatus> for ThreadResourceStatus {
    fn from(value: ActiveStatus) -> Self {
        Self::ActiveStatus(value)
    }
}
impl ::std::convert::From<LockedStatus> for ThreadResourceStatus {
    fn from(value: LockedStatus) -> Self {
        Self::LockedStatus(value)
    }
}
impl ::std::convert::From<ClosedStatus> for ThreadResourceStatus {
    fn from(value: ClosedStatus) -> Self {
        Self::ClosedStatus(value)
    }
}
#[doc = "Tool selection that the assistant should honor when executing the item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoice {
    #[doc = "Identifier of the requested tool."]
    pub id: ::std::string::String,
}
impl ToolChoice {
    pub fn builder() -> builder::ToolChoice {
        Default::default()
    }
}
#[doc = "Annotation that references a URL."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UrlAnnotation {
    #[doc = "URL referenced by the annotation."]
    pub source: UrlAnnotationSource,
    #[doc = "Type discriminator that is always `url` for this annotation."]
    #[serde(rename = "type")]
    pub type_: UrlAnnotationType,
}
impl UrlAnnotation {
    pub fn builder() -> builder::UrlAnnotation {
        Default::default()
    }
}
#[doc = "URL backing an annotation entry."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UrlAnnotationSource {
    #[doc = "Type discriminator that is always `url`."]
    #[serde(rename = "type")]
    pub type_: UrlAnnotationSourceType,
    #[doc = "URL referenced by the annotation."]
    pub url: ::std::string::String,
}
impl UrlAnnotationSource {
    pub fn builder() -> builder::UrlAnnotationSource {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `url`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UrlAnnotationSourceType {
    #[serde(rename = "url")]
    Url,
}
impl ::std::fmt::Display for UrlAnnotationSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Url => f.write_str("url"),
        }
    }
}
impl ::std::str::FromStr for UrlAnnotationSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url" => Ok(Self::Url),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UrlAnnotationSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UrlAnnotationSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `url` for this annotation."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UrlAnnotationType {
    #[serde(rename = "url")]
    Url,
}
impl ::std::fmt::Display for UrlAnnotationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Url => f.write_str("url"),
        }
    }
}
impl ::std::str::FromStr for UrlAnnotationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url" => Ok(Self::Url),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UrlAnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UrlAnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Text block that a user contributed to the thread."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserMessageInputText {
    #[doc = "Plain-text content supplied by the user."]
    pub text: ::std::string::String,
    #[doc = "Type discriminator that is always `input_text`."]
    #[serde(rename = "type")]
    pub type_: UserMessageInputTextType,
}
impl UserMessageInputText {
    pub fn builder() -> builder::UserMessageInputText {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `input_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserMessageInputTextType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for UserMessageInputTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for UserMessageInputTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserMessageInputTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserMessageInputTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "User-authored messages within a thread."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserMessageItem {
    #[doc = "Attachments associated with the user message. Defaults to an empty list."]
    pub attachments: ::std::vec::Vec<Attachment>,
    #[doc = "Ordered content elements supplied by the user."]
    pub content: ::std::vec::Vec<UserMessageItemContentItem>,
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub inference_options: ::std::option::Option<InferenceOptions>,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: UserMessageItemObject,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: UserMessageItemType,
}
impl UserMessageItem {
    pub fn builder() -> builder::UserMessageItem {
        Default::default()
    }
}
#[doc = "Content blocks that comprise a user message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum UserMessageItemContentItem {
    InputText(UserMessageInputText),
    QuotedText(UserMessageQuotedText),
}
impl ::std::convert::From<UserMessageInputText> for UserMessageItemContentItem {
    fn from(value: UserMessageInputText) -> Self {
        Self::InputText(value)
    }
}
impl ::std::convert::From<UserMessageQuotedText> for UserMessageItemContentItem {
    fn from(value: UserMessageQuotedText) -> Self {
        Self::QuotedText(value)
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserMessageItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for UserMessageItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for UserMessageItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UserMessageItemType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserMessageItemType {
    #[serde(rename = "chatkit.user_message")]
    ChatkitUserMessage,
}
impl ::std::fmt::Display for UserMessageItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitUserMessage => f.write_str("chatkit.user_message"),
        }
    }
}
impl ::std::str::FromStr for UserMessageItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.user_message" => Ok(Self::ChatkitUserMessage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Quoted snippet that the user referenced in their message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserMessageQuotedText {
    #[doc = "Quoted text content."]
    pub text: ::std::string::String,
    #[doc = "Type discriminator that is always `quoted_text`."]
    #[serde(rename = "type")]
    pub type_: UserMessageQuotedTextType,
}
impl UserMessageQuotedText {
    pub fn builder() -> builder::UserMessageQuotedText {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `quoted_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserMessageQuotedTextType {
    #[serde(rename = "quoted_text")]
    QuotedText,
}
impl ::std::fmt::Display for UserMessageQuotedTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::QuotedText => f.write_str("quoted_text"),
        }
    }
}
impl ::std::str::FromStr for UserMessageQuotedTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "quoted_text" => Ok(Self::QuotedText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserMessageQuotedTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserMessageQuotedTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Thread item that renders a widget payload."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WidgetMessageItem {
    #[doc = "Unix timestamp (in seconds) for when the item was created."]
    pub created_at: i64,
    #[doc = "Identifier of the thread item."]
    pub id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.thread_item`."]
    pub object: WidgetMessageItemObject,
    #[doc = "Identifier of the parent thread."]
    pub thread_id: ::std::string::String,
    #[doc = "Type discriminator that is always `chatkit.widget`."]
    #[serde(rename = "type")]
    pub type_: WidgetMessageItemType,
    #[doc = "Serialized widget payload rendered in the UI."]
    pub widget: ::std::string::String,
}
impl WidgetMessageItem {
    pub fn builder() -> builder::WidgetMessageItem {
        Default::default()
    }
}
#[doc = "Type discriminator that is always `chatkit.thread_item`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WidgetMessageItemObject {
    #[serde(rename = "chatkit.thread_item")]
    ChatkitThreadItem,
}
impl ::std::fmt::Display for WidgetMessageItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitThreadItem => f.write_str("chatkit.thread_item"),
        }
    }
}
impl ::std::str::FromStr for WidgetMessageItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.thread_item" => Ok(Self::ChatkitThreadItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WidgetMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WidgetMessageItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Type discriminator that is always `chatkit.widget`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WidgetMessageItemType {
    #[serde(rename = "chatkit.widget")]
    ChatkitWidget,
}
impl ::std::fmt::Display for WidgetMessageItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ChatkitWidget => f.write_str("chatkit.widget"),
        }
    }
}
impl ::std::str::FromStr for WidgetMessageItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chatkit.widget" => Ok(Self::ChatkitWidget),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WidgetMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WidgetMessageItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Workflow reference and overrides applied to the chat session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowParam {
    #[doc = "Identifier for the workflow invoked by the session."]
    pub id: ::std::string::String,
    #[doc = "State variables forwarded to the workflow. Keys may be up to 64 characters, values must be primitive types, and the map defaults to an empty object."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub state_variables:
        ::std::collections::HashMap<::std::string::String, WorkflowParamStateVariablesValue>,
    #[doc = "Optional tracing overrides for the workflow invocation. When omitted, tracing is enabled by default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tracing: ::std::option::Option<WorkflowTracingParam>,
    #[doc = "Specific workflow version to run. Defaults to the latest deployed version."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl WorkflowParam {
    pub fn builder() -> builder::WorkflowParam {
        Default::default()
    }
}
#[doc = "`WorkflowParamStateVariablesValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum WorkflowParamStateVariablesValue {
    String(WorkflowParamStateVariablesValueString),
    Integer(i64),
    Boolean(bool),
    Number(f64),
}
impl ::std::fmt::Display for WorkflowParamStateVariablesValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for WorkflowParamStateVariablesValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for WorkflowParamStateVariablesValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkflowParamStateVariablesValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<WorkflowParamStateVariablesValueString>
    for WorkflowParamStateVariablesValue
{
    fn from(value: WorkflowParamStateVariablesValueString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<i64> for WorkflowParamStateVariablesValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<bool> for WorkflowParamStateVariablesValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for WorkflowParamStateVariablesValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`WorkflowParamStateVariablesValueString`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WorkflowParamStateVariablesValueString(::std::string::String);
impl ::std::ops::Deref for WorkflowParamStateVariablesValueString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WorkflowParamStateVariablesValueString> for ::std::string::String {
    fn from(value: WorkflowParamStateVariablesValueString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WorkflowParamStateVariablesValueString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WorkflowParamStateVariablesValueString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkflowParamStateVariablesValueString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WorkflowParamStateVariablesValueString {
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
#[doc = "Controls diagnostic tracing during the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct WorkflowTracingParam {
    #[doc = "Whether tracing is enabled during the session. Defaults to true."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
}
impl WorkflowTracingParam {
    pub fn builder() -> builder::WorkflowTracingParam {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ActiveStatus {
        type_: ::std::result::Result<super::ActiveStatusType, ::std::string::String>,
    }
    impl ::std::default::Default for ActiveStatus {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ActiveStatus {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActiveStatusType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActiveStatus> for super::ActiveStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActiveStatus,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ActiveStatus> for ActiveStatus {
        fn from(value: super::ActiveStatus) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AssistantMessageItem {
        content: ::std::result::Result<
            ::std::vec::Vec<super::ResponseOutputText>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::AssistantMessageItemObject, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::AssistantMessageItemType, ::std::string::String>,
    }
    impl ::std::default::Default for AssistantMessageItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AssistantMessageItem {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResponseOutputText>>,
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssistantMessageItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
            T: ::std::convert::TryInto<super::AssistantMessageItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AssistantMessageItem> for super::AssistantMessageItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssistantMessageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                thread_id: value.thread_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AssistantMessageItem> for AssistantMessageItem {
        fn from(value: super::AssistantMessageItem) -> Self {
            Self {
                content: Ok(value.content),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Attachment {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        preview_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AttachmentType, ::std::string::String>,
    }
    impl ::std::default::Default for Attachment {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                mime_type: Err("no value supplied for mime_type".to_string()),
                name: Err("no value supplied for name".to_string()),
                preview_url: Err("no value supplied for preview_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Attachment {
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
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {e}"));
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
        pub fn preview_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.preview_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for preview_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AttachmentType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Attachment> for super::Attachment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Attachment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                mime_type: value.mime_type?,
                name: value.name?,
                preview_url: value.preview_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Attachment> for Attachment {
        fn from(value: super::Attachment) -> Self {
            Self {
                id: Ok(value.id),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                preview_url: Ok(value.preview_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AutomaticThreadTitlingParam {
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for AutomaticThreadTitlingParam {
        fn default() -> Self {
            Self {
                enabled: Ok(Default::default()),
            }
        }
    }
    impl AutomaticThreadTitlingParam {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AutomaticThreadTitlingParam> for super::AutomaticThreadTitlingParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AutomaticThreadTitlingParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::AutomaticThreadTitlingParam> for AutomaticThreadTitlingParam {
        fn from(value: super::AutomaticThreadTitlingParam) -> Self {
            Self {
                enabled: Ok(value.enabled),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelChatSessionMethodParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelChatSessionMethodParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl CancelChatSessionMethodParams {
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelChatSessionMethodParams>
        for super::CancelChatSessionMethodParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelChatSessionMethodParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelChatSessionMethodParams> for CancelChatSessionMethodParams {
        fn from(value: super::CancelChatSessionMethodParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatKitApiSchema {
        cancel_chat_session_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_chat_session_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_chat_session_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_chat_session_method_params:
            ::std::result::Result<super::CancelChatSessionMethodParams, ::std::string::String>,
        cancel_chat_session_method_response:
            ::std::result::Result<super::ChatSessionResource, ::std::string::String>,
        create_chat_session_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_chat_session_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_chat_session_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_chat_session_method_request:
            ::std::result::Result<super::CreateChatSessionBody, ::std::string::String>,
        create_chat_session_method_response:
            ::std::result::Result<super::ChatSessionResource, ::std::string::String>,
        delete_thread_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_thread_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_thread_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_thread_method_params:
            ::std::result::Result<super::DeleteThreadMethodParams, ::std::string::String>,
        delete_thread_method_response:
            ::std::result::Result<super::DeletedThreadResource, ::std::string::String>,
        get_thread_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_thread_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_thread_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_thread_method_params:
            ::std::result::Result<super::GetThreadMethodParams, ::std::string::String>,
        get_thread_method_response:
            ::std::result::Result<super::ThreadResource, ::std::string::String>,
        list_thread_items_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_thread_items_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_thread_items_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_thread_items_method_params:
            ::std::result::Result<super::ListThreadItemsMethodParams, ::std::string::String>,
        list_thread_items_method_response:
            ::std::result::Result<super::ThreadItemListResource, ::std::string::String>,
        list_threads_method_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_threads_method_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_threads_method_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_threads_method_params:
            ::std::result::Result<super::ListThreadsMethodParams, ::std::string::String>,
        list_threads_method_response:
            ::std::result::Result<super::ThreadListResource, ::std::string::String>,
    }
    impl ::std::default::Default for ChatKitApiSchema {
        fn default() -> Self {
            Self {
                cancel_chat_session_method_error400: Err(
                    "no value supplied for cancel_chat_session_method_error400".to_string(),
                ),
                cancel_chat_session_method_error404: Err(
                    "no value supplied for cancel_chat_session_method_error404".to_string(),
                ),
                cancel_chat_session_method_error429: Err(
                    "no value supplied for cancel_chat_session_method_error429".to_string(),
                ),
                cancel_chat_session_method_params: Err(
                    "no value supplied for cancel_chat_session_method_params".to_string(),
                ),
                cancel_chat_session_method_response: Err(
                    "no value supplied for cancel_chat_session_method_response".to_string(),
                ),
                create_chat_session_method_error400: Err(
                    "no value supplied for create_chat_session_method_error400".to_string(),
                ),
                create_chat_session_method_error404: Err(
                    "no value supplied for create_chat_session_method_error404".to_string(),
                ),
                create_chat_session_method_error429: Err(
                    "no value supplied for create_chat_session_method_error429".to_string(),
                ),
                create_chat_session_method_request: Err(
                    "no value supplied for create_chat_session_method_request".to_string(),
                ),
                create_chat_session_method_response: Err(
                    "no value supplied for create_chat_session_method_response".to_string(),
                ),
                delete_thread_method_error400: Err(
                    "no value supplied for delete_thread_method_error400".to_string(),
                ),
                delete_thread_method_error404: Err(
                    "no value supplied for delete_thread_method_error404".to_string(),
                ),
                delete_thread_method_error429: Err(
                    "no value supplied for delete_thread_method_error429".to_string(),
                ),
                delete_thread_method_params: Err(
                    "no value supplied for delete_thread_method_params".to_string(),
                ),
                delete_thread_method_response: Err(
                    "no value supplied for delete_thread_method_response".to_string(),
                ),
                get_thread_method_error400: Err(
                    "no value supplied for get_thread_method_error400".to_string()
                ),
                get_thread_method_error404: Err(
                    "no value supplied for get_thread_method_error404".to_string()
                ),
                get_thread_method_error429: Err(
                    "no value supplied for get_thread_method_error429".to_string()
                ),
                get_thread_method_params: Err(
                    "no value supplied for get_thread_method_params".to_string()
                ),
                get_thread_method_response: Err(
                    "no value supplied for get_thread_method_response".to_string()
                ),
                list_thread_items_method_error400: Err(
                    "no value supplied for list_thread_items_method_error400".to_string(),
                ),
                list_thread_items_method_error404: Err(
                    "no value supplied for list_thread_items_method_error404".to_string(),
                ),
                list_thread_items_method_error429: Err(
                    "no value supplied for list_thread_items_method_error429".to_string(),
                ),
                list_thread_items_method_params: Err(
                    "no value supplied for list_thread_items_method_params".to_string(),
                ),
                list_thread_items_method_response: Err(
                    "no value supplied for list_thread_items_method_response".to_string(),
                ),
                list_threads_method_error400: Err(
                    "no value supplied for list_threads_method_error400".to_string(),
                ),
                list_threads_method_error404: Err(
                    "no value supplied for list_threads_method_error404".to_string(),
                ),
                list_threads_method_error429: Err(
                    "no value supplied for list_threads_method_error429".to_string(),
                ),
                list_threads_method_params: Err(
                    "no value supplied for list_threads_method_params".to_string()
                ),
                list_threads_method_response: Err(
                    "no value supplied for list_threads_method_response".to_string(),
                ),
            }
        }
    }
    impl ChatKitApiSchema {
        pub fn cancel_chat_session_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_chat_session_method_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cancel_chat_session_method_error400: {e}"
                )
            });
            self
        }
        pub fn cancel_chat_session_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_chat_session_method_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cancel_chat_session_method_error404: {e}"
                )
            });
            self
        }
        pub fn cancel_chat_session_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_chat_session_method_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cancel_chat_session_method_error429: {e}"
                )
            });
            self
        }
        pub fn cancel_chat_session_method_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelChatSessionMethodParams>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_chat_session_method_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cancel_chat_session_method_params: {e}"
                )
            });
            self
        }
        pub fn cancel_chat_session_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionResource>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_chat_session_method_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cancel_chat_session_method_response: {e}"
                )
            });
            self
        }
        pub fn create_chat_session_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_session_method_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_chat_session_method_error400: {e}"
                )
            });
            self
        }
        pub fn create_chat_session_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_session_method_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_chat_session_method_error404: {e}"
                )
            });
            self
        }
        pub fn create_chat_session_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_session_method_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_chat_session_method_error429: {e}"
                )
            });
            self
        }
        pub fn create_chat_session_method_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatSessionBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_session_method_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_chat_session_method_request: {e}"
                )
            });
            self
        }
        pub fn create_chat_session_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_chat_session_method_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_chat_session_method_response: {e}"
                )
            });
            self
        }
        pub fn delete_thread_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_method_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_method_error400: {e}")
            });
            self
        }
        pub fn delete_thread_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_method_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_method_error404: {e}")
            });
            self
        }
        pub fn delete_thread_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_method_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_method_error429: {e}")
            });
            self
        }
        pub fn delete_thread_method_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteThreadMethodParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_method_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_method_params: {e}")
            });
            self
        }
        pub fn delete_thread_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedThreadResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_thread_method_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_thread_method_response: {e}")
            });
            self
        }
        pub fn get_thread_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_method_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_method_error400: {e}")
            });
            self
        }
        pub fn get_thread_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_method_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_method_error404: {e}")
            });
            self
        }
        pub fn get_thread_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_method_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_method_error429: {e}")
            });
            self
        }
        pub fn get_thread_method_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetThreadMethodParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_method_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_method_params: {e}")
            });
            self
        }
        pub fn get_thread_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_thread_method_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_thread_method_response: {e}")
            });
            self
        }
        pub fn list_thread_items_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_thread_items_method_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_thread_items_method_error400: {e}"
                )
            });
            self
        }
        pub fn list_thread_items_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_thread_items_method_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_thread_items_method_error404: {e}"
                )
            });
            self
        }
        pub fn list_thread_items_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_thread_items_method_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_thread_items_method_error429: {e}"
                )
            });
            self
        }
        pub fn list_thread_items_method_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListThreadItemsMethodParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_thread_items_method_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_thread_items_method_params: {e}")
            });
            self
        }
        pub fn list_thread_items_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadItemListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_thread_items_method_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_thread_items_method_response: {e}"
                )
            });
            self
        }
        pub fn list_threads_method_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_threads_method_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_threads_method_error400: {e}")
            });
            self
        }
        pub fn list_threads_method_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_threads_method_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_threads_method_error404: {e}")
            });
            self
        }
        pub fn list_threads_method_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_threads_method_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_threads_method_error429: {e}")
            });
            self
        }
        pub fn list_threads_method_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListThreadsMethodParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_threads_method_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_threads_method_params: {e}")
            });
            self
        }
        pub fn list_threads_method_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_threads_method_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_threads_method_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ChatKitApiSchema> for super::ChatKitApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatKitApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_chat_session_method_error400: value.cancel_chat_session_method_error400?,
                cancel_chat_session_method_error404: value.cancel_chat_session_method_error404?,
                cancel_chat_session_method_error429: value.cancel_chat_session_method_error429?,
                cancel_chat_session_method_params: value.cancel_chat_session_method_params?,
                cancel_chat_session_method_response: value.cancel_chat_session_method_response?,
                create_chat_session_method_error400: value.create_chat_session_method_error400?,
                create_chat_session_method_error404: value.create_chat_session_method_error404?,
                create_chat_session_method_error429: value.create_chat_session_method_error429?,
                create_chat_session_method_request: value.create_chat_session_method_request?,
                create_chat_session_method_response: value.create_chat_session_method_response?,
                delete_thread_method_error400: value.delete_thread_method_error400?,
                delete_thread_method_error404: value.delete_thread_method_error404?,
                delete_thread_method_error429: value.delete_thread_method_error429?,
                delete_thread_method_params: value.delete_thread_method_params?,
                delete_thread_method_response: value.delete_thread_method_response?,
                get_thread_method_error400: value.get_thread_method_error400?,
                get_thread_method_error404: value.get_thread_method_error404?,
                get_thread_method_error429: value.get_thread_method_error429?,
                get_thread_method_params: value.get_thread_method_params?,
                get_thread_method_response: value.get_thread_method_response?,
                list_thread_items_method_error400: value.list_thread_items_method_error400?,
                list_thread_items_method_error404: value.list_thread_items_method_error404?,
                list_thread_items_method_error429: value.list_thread_items_method_error429?,
                list_thread_items_method_params: value.list_thread_items_method_params?,
                list_thread_items_method_response: value.list_thread_items_method_response?,
                list_threads_method_error400: value.list_threads_method_error400?,
                list_threads_method_error404: value.list_threads_method_error404?,
                list_threads_method_error429: value.list_threads_method_error429?,
                list_threads_method_params: value.list_threads_method_params?,
                list_threads_method_response: value.list_threads_method_response?,
            })
        }
    }
    impl ::std::convert::From<super::ChatKitApiSchema> for ChatKitApiSchema {
        fn from(value: super::ChatKitApiSchema) -> Self {
            Self {
                cancel_chat_session_method_error400: Ok(value.cancel_chat_session_method_error400),
                cancel_chat_session_method_error404: Ok(value.cancel_chat_session_method_error404),
                cancel_chat_session_method_error429: Ok(value.cancel_chat_session_method_error429),
                cancel_chat_session_method_params: Ok(value.cancel_chat_session_method_params),
                cancel_chat_session_method_response: Ok(value.cancel_chat_session_method_response),
                create_chat_session_method_error400: Ok(value.create_chat_session_method_error400),
                create_chat_session_method_error404: Ok(value.create_chat_session_method_error404),
                create_chat_session_method_error429: Ok(value.create_chat_session_method_error429),
                create_chat_session_method_request: Ok(value.create_chat_session_method_request),
                create_chat_session_method_response: Ok(value.create_chat_session_method_response),
                delete_thread_method_error400: Ok(value.delete_thread_method_error400),
                delete_thread_method_error404: Ok(value.delete_thread_method_error404),
                delete_thread_method_error429: Ok(value.delete_thread_method_error429),
                delete_thread_method_params: Ok(value.delete_thread_method_params),
                delete_thread_method_response: Ok(value.delete_thread_method_response),
                get_thread_method_error400: Ok(value.get_thread_method_error400),
                get_thread_method_error404: Ok(value.get_thread_method_error404),
                get_thread_method_error429: Ok(value.get_thread_method_error429),
                get_thread_method_params: Ok(value.get_thread_method_params),
                get_thread_method_response: Ok(value.get_thread_method_response),
                list_thread_items_method_error400: Ok(value.list_thread_items_method_error400),
                list_thread_items_method_error404: Ok(value.list_thread_items_method_error404),
                list_thread_items_method_error429: Ok(value.list_thread_items_method_error429),
                list_thread_items_method_params: Ok(value.list_thread_items_method_params),
                list_thread_items_method_response: Ok(value.list_thread_items_method_response),
                list_threads_method_error400: Ok(value.list_threads_method_error400),
                list_threads_method_error404: Ok(value.list_threads_method_error404),
                list_threads_method_error429: Ok(value.list_threads_method_error429),
                list_threads_method_params: Ok(value.list_threads_method_params),
                list_threads_method_response: Ok(value.list_threads_method_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionAutomaticThreadTitling {
        enabled: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionAutomaticThreadTitling {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
            }
        }
    }
    impl ChatSessionAutomaticThreadTitling {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionAutomaticThreadTitling>
        for super::ChatSessionAutomaticThreadTitling
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionAutomaticThreadTitling,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionAutomaticThreadTitling>
        for ChatSessionAutomaticThreadTitling
    {
        fn from(value: super::ChatSessionAutomaticThreadTitling) -> Self {
            Self {
                enabled: Ok(value.enabled),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionChatkitConfiguration {
        automatic_thread_titling:
            ::std::result::Result<super::ChatSessionAutomaticThreadTitling, ::std::string::String>,
        file_upload: ::std::result::Result<super::ChatSessionFileUpload, ::std::string::String>,
        history: ::std::result::Result<super::ChatSessionHistory, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionChatkitConfiguration {
        fn default() -> Self {
            Self {
                automatic_thread_titling: Err(
                    "no value supplied for automatic_thread_titling".to_string()
                ),
                file_upload: Err("no value supplied for file_upload".to_string()),
                history: Err("no value supplied for history".to_string()),
            }
        }
    }
    impl ChatSessionChatkitConfiguration {
        pub fn automatic_thread_titling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionAutomaticThreadTitling>,
            T::Error: ::std::fmt::Display,
        {
            self.automatic_thread_titling = value.try_into().map_err(|e| {
                format!("error converting supplied value for automatic_thread_titling: {e}")
            });
            self
        }
        pub fn file_upload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionFileUpload>,
            T::Error: ::std::fmt::Display,
        {
            self.file_upload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_upload: {e}"));
            self
        }
        pub fn history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionHistory>,
            T::Error: ::std::fmt::Display,
        {
            self.history = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionChatkitConfiguration>
        for super::ChatSessionChatkitConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionChatkitConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                automatic_thread_titling: value.automatic_thread_titling?,
                file_upload: value.file_upload?,
                history: value.history?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionChatkitConfiguration>
        for ChatSessionChatkitConfiguration
    {
        fn from(value: super::ChatSessionChatkitConfiguration) -> Self {
            Self {
                automatic_thread_titling: Ok(value.automatic_thread_titling),
                file_upload: Ok(value.file_upload),
                history: Ok(value.history),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionFileUpload {
        enabled: ::std::result::Result<bool, ::std::string::String>,
        max_file_size: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_files: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionFileUpload {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
                max_file_size: Err("no value supplied for max_file_size".to_string()),
                max_files: Err("no value supplied for max_files".to_string()),
            }
        }
    }
    impl ChatSessionFileUpload {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
        pub fn max_file_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_file_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_file_size: {e}"));
            self
        }
        pub fn max_files<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_files: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionFileUpload> for super::ChatSessionFileUpload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionFileUpload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                max_file_size: value.max_file_size?,
                max_files: value.max_files?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionFileUpload> for ChatSessionFileUpload {
        fn from(value: super::ChatSessionFileUpload) -> Self {
            Self {
                enabled: Ok(value.enabled),
                max_file_size: Ok(value.max_file_size),
                max_files: Ok(value.max_files),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionHistory {
        enabled: ::std::result::Result<bool, ::std::string::String>,
        recent_threads: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionHistory {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
                recent_threads: Err("no value supplied for recent_threads".to_string()),
            }
        }
    }
    impl ChatSessionHistory {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
        pub fn recent_threads<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.recent_threads = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recent_threads: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionHistory> for super::ChatSessionHistory {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionHistory,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                recent_threads: value.recent_threads?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionHistory> for ChatSessionHistory {
        fn from(value: super::ChatSessionHistory) -> Self {
            Self {
                enabled: Ok(value.enabled),
                recent_threads: Ok(value.recent_threads),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionRateLimits {
        max_requests_per_1_minute: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionRateLimits {
        fn default() -> Self {
            Self {
                max_requests_per_1_minute: Err(
                    "no value supplied for max_requests_per_1_minute".to_string()
                ),
            }
        }
    }
    impl ChatSessionRateLimits {
        pub fn max_requests_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_minute: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionRateLimits> for super::ChatSessionRateLimits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionRateLimits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_requests_per_1_minute: value.max_requests_per_1_minute?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionRateLimits> for ChatSessionRateLimits {
        fn from(value: super::ChatSessionRateLimits) -> Self {
            Self {
                max_requests_per_1_minute: Ok(value.max_requests_per_1_minute),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatSessionResource {
        chatkit_configuration:
            ::std::result::Result<super::ChatSessionChatkitConfiguration, ::std::string::String>,
        client_secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        expires_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        max_requests_per_1_minute: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::ChatSessionResourceObject, ::std::string::String>,
        rate_limits: ::std::result::Result<super::ChatSessionRateLimits, ::std::string::String>,
        status: ::std::result::Result<super::ChatSessionStatus, ::std::string::String>,
        user: ::std::result::Result<::std::string::String, ::std::string::String>,
        workflow: ::std::result::Result<super::ChatkitWorkflow, ::std::string::String>,
    }
    impl ::std::default::Default for ChatSessionResource {
        fn default() -> Self {
            Self {
                chatkit_configuration: Err(
                    "no value supplied for chatkit_configuration".to_string()
                ),
                client_secret: Err("no value supplied for client_secret".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                max_requests_per_1_minute: Err(
                    "no value supplied for max_requests_per_1_minute".to_string()
                ),
                object: Err("no value supplied for object".to_string()),
                rate_limits: Err("no value supplied for rate_limits".to_string()),
                status: Err("no value supplied for status".to_string()),
                user: Err("no value supplied for user".to_string()),
                workflow: Err("no value supplied for workflow".to_string()),
            }
        }
    }
    impl ChatSessionResource {
        pub fn chatkit_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionChatkitConfiguration>,
            T::Error: ::std::fmt::Display,
        {
            self.chatkit_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for chatkit_configuration: {e}")
            });
            self
        }
        pub fn client_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.client_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
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
        pub fn max_requests_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_minute: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn rate_limits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionRateLimits>,
            T::Error: ::std::fmt::Display,
        {
            self.rate_limits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate_limits: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatSessionStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
        pub fn workflow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatkitWorkflow>,
            T::Error: ::std::fmt::Display,
        {
            self.workflow = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workflow: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatSessionResource> for super::ChatSessionResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatSessionResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chatkit_configuration: value.chatkit_configuration?,
                client_secret: value.client_secret?,
                expires_at: value.expires_at?,
                id: value.id?,
                max_requests_per_1_minute: value.max_requests_per_1_minute?,
                object: value.object?,
                rate_limits: value.rate_limits?,
                status: value.status?,
                user: value.user?,
                workflow: value.workflow?,
            })
        }
    }
    impl ::std::convert::From<super::ChatSessionResource> for ChatSessionResource {
        fn from(value: super::ChatSessionResource) -> Self {
            Self {
                chatkit_configuration: Ok(value.chatkit_configuration),
                client_secret: Ok(value.client_secret),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                max_requests_per_1_minute: Ok(value.max_requests_per_1_minute),
                object: Ok(value.object),
                rate_limits: Ok(value.rate_limits),
                status: Ok(value.status),
                user: Ok(value.user),
                workflow: Ok(value.workflow),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatkitConfigurationParam {
        automatic_thread_titling: ::std::result::Result<
            ::std::option::Option<super::AutomaticThreadTitlingParam>,
            ::std::string::String,
        >,
        file_upload: ::std::result::Result<
            ::std::option::Option<super::FileUploadParam>,
            ::std::string::String,
        >,
        history: ::std::result::Result<
            ::std::option::Option<super::HistoryParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatkitConfigurationParam {
        fn default() -> Self {
            Self {
                automatic_thread_titling: Ok(Default::default()),
                file_upload: Ok(Default::default()),
                history: Ok(Default::default()),
            }
        }
    }
    impl ChatkitConfigurationParam {
        pub fn automatic_thread_titling<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AutomaticThreadTitlingParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.automatic_thread_titling = value.try_into().map_err(|e| {
                format!("error converting supplied value for automatic_thread_titling: {e}")
            });
            self
        }
        pub fn file_upload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FileUploadParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_upload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_upload: {e}"));
            self
        }
        pub fn history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HistoryParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.history = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatkitConfigurationParam> for super::ChatkitConfigurationParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatkitConfigurationParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                automatic_thread_titling: value.automatic_thread_titling?,
                file_upload: value.file_upload?,
                history: value.history?,
            })
        }
    }
    impl ::std::convert::From<super::ChatkitConfigurationParam> for ChatkitConfigurationParam {
        fn from(value: super::ChatkitConfigurationParam) -> Self {
            Self {
                automatic_thread_titling: Ok(value.automatic_thread_titling),
                file_upload: Ok(value.file_upload),
                history: Ok(value.history),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatkitWorkflow {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        state_variables: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::ChatkitWorkflowStateVariablesValue,
                >,
            >,
            ::std::string::String,
        >,
        tracing: ::std::result::Result<super::ChatkitWorkflowTracing, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ChatkitWorkflow {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                state_variables: Err("no value supplied for state_variables".to_string()),
                tracing: Err("no value supplied for tracing".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl ChatkitWorkflow {
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
        pub fn state_variables<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::ChatkitWorkflowStateVariablesValue,
                    >,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.state_variables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state_variables: {e}"));
            self
        }
        pub fn tracing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChatkitWorkflowTracing>,
            T::Error: ::std::fmt::Display,
        {
            self.tracing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tracing: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatkitWorkflow> for super::ChatkitWorkflow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatkitWorkflow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                state_variables: value.state_variables?,
                tracing: value.tracing?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::ChatkitWorkflow> for ChatkitWorkflow {
        fn from(value: super::ChatkitWorkflow) -> Self {
            Self {
                id: Ok(value.id),
                state_variables: Ok(value.state_variables),
                tracing: Ok(value.tracing),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatkitWorkflowTracing {
        enabled: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ChatkitWorkflowTracing {
        fn default() -> Self {
            Self {
                enabled: Err("no value supplied for enabled".to_string()),
            }
        }
    }
    impl ChatkitWorkflowTracing {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChatkitWorkflowTracing> for super::ChatkitWorkflowTracing {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChatkitWorkflowTracing,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::ChatkitWorkflowTracing> for ChatkitWorkflowTracing {
        fn from(value: super::ChatkitWorkflowTracing) -> Self {
            Self {
                enabled: Ok(value.enabled),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientToolCallItem {
        arguments: ::std::result::Result<::std::string::String, ::std::string::String>,
        call_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ClientToolCallItemObject, ::std::string::String>,
        output: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::ClientToolCallStatus, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::ClientToolCallItemType, ::std::string::String>,
    }
    impl ::std::default::Default for ClientToolCallItem {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                call_id: Err("no value supplied for call_id".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                output: Err("no value supplied for output".to_string()),
                status: Err("no value supplied for status".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ClientToolCallItem {
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
        pub fn call_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.call_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for call_id: {e}"));
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
            T: ::std::convert::TryInto<super::ClientToolCallItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClientToolCallStatus>,
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClientToolCallItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientToolCallItem> for super::ClientToolCallItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientToolCallItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                call_id: value.call_id?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                output: value.output?,
                status: value.status?,
                thread_id: value.thread_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ClientToolCallItem> for ClientToolCallItem {
        fn from(value: super::ClientToolCallItem) -> Self {
            Self {
                arguments: Ok(value.arguments),
                call_id: Ok(value.call_id),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                output: Ok(value.output),
                status: Ok(value.status),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClosedStatus {
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::ClosedStatusType, ::std::string::String>,
    }
    impl ::std::default::Default for ClosedStatus {
        fn default() -> Self {
            Self {
                reason: Err("no value supplied for reason".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ClosedStatus {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClosedStatusType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClosedStatus> for super::ClosedStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClosedStatus,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ClosedStatus> for ClosedStatus {
        fn from(value: super::ClosedStatus) -> Self {
            Self {
                reason: Ok(value.reason),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateChatSessionBody {
        chatkit_configuration: ::std::result::Result<
            ::std::option::Option<super::ChatkitConfigurationParam>,
            ::std::string::String,
        >,
        expires_after: ::std::result::Result<
            ::std::option::Option<super::ExpiresAfterParam>,
            ::std::string::String,
        >,
        rate_limits: ::std::result::Result<
            ::std::option::Option<super::RateLimitsParam>,
            ::std::string::String,
        >,
        user: ::std::result::Result<super::CreateChatSessionBodyUser, ::std::string::String>,
        workflow: ::std::result::Result<super::WorkflowParam, ::std::string::String>,
    }
    impl ::std::default::Default for CreateChatSessionBody {
        fn default() -> Self {
            Self {
                chatkit_configuration: Ok(Default::default()),
                expires_after: Ok(Default::default()),
                rate_limits: Ok(Default::default()),
                user: Err("no value supplied for user".to_string()),
                workflow: Err("no value supplied for workflow".to_string()),
            }
        }
    }
    impl CreateChatSessionBody {
        pub fn chatkit_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChatkitConfigurationParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.chatkit_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for chatkit_configuration: {e}")
            });
            self
        }
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExpiresAfterParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
            self
        }
        pub fn rate_limits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RateLimitsParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate_limits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate_limits: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateChatSessionBodyUser>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
        pub fn workflow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WorkflowParam>,
            T::Error: ::std::fmt::Display,
        {
            self.workflow = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workflow: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateChatSessionBody> for super::CreateChatSessionBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateChatSessionBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                chatkit_configuration: value.chatkit_configuration?,
                expires_after: value.expires_after?,
                rate_limits: value.rate_limits?,
                user: value.user?,
                workflow: value.workflow?,
            })
        }
    }
    impl ::std::convert::From<super::CreateChatSessionBody> for CreateChatSessionBody {
        fn from(value: super::CreateChatSessionBody) -> Self {
            Self {
                chatkit_configuration: Ok(value.chatkit_configuration),
                expires_after: Ok(value.expires_after),
                rate_limits: Ok(value.rate_limits),
                user: Ok(value.user),
                workflow: Ok(value.workflow),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteThreadMethodParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteThreadMethodParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl DeleteThreadMethodParams {
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
    impl ::std::convert::TryFrom<DeleteThreadMethodParams> for super::DeleteThreadMethodParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteThreadMethodParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteThreadMethodParams> for DeleteThreadMethodParams {
        fn from(value: super::DeleteThreadMethodParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedThreadResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeletedThreadResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedThreadResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedThreadResource {
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
            T: ::std::convert::TryInto<super::DeletedThreadResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedThreadResource> for super::DeletedThreadResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedThreadResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedThreadResource> for DeletedThreadResource {
        fn from(value: super::DeletedThreadResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExpiresAfterParam {
        anchor: ::std::result::Result<super::ExpiresAfterParamAnchor, ::std::string::String>,
        seconds: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for ExpiresAfterParam {
        fn default() -> Self {
            Self {
                anchor: Err("no value supplied for anchor".to_string()),
                seconds: Err("no value supplied for seconds".to_string()),
            }
        }
    }
    impl ExpiresAfterParam {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExpiresAfterParamAnchor>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExpiresAfterParam> for super::ExpiresAfterParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExpiresAfterParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                seconds: value.seconds?,
            })
        }
    }
    impl ::std::convert::From<super::ExpiresAfterParam> for ExpiresAfterParam {
        fn from(value: super::ExpiresAfterParam) -> Self {
            Self {
                anchor: Ok(value.anchor),
                seconds: Ok(value.seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileAnnotation {
        source: ::std::result::Result<super::FileAnnotationSource, ::std::string::String>,
        type_: ::std::result::Result<super::FileAnnotationType, ::std::string::String>,
    }
    impl ::std::default::Default for FileAnnotation {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FileAnnotation {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileAnnotationSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileAnnotationType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FileAnnotation> for super::FileAnnotation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileAnnotation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FileAnnotation> for FileAnnotation {
        fn from(value: super::FileAnnotation) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileAnnotationSource {
        filename: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::FileAnnotationSourceType, ::std::string::String>,
    }
    impl ::std::default::Default for FileAnnotationSource {
        fn default() -> Self {
            Self {
                filename: Err("no value supplied for filename".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FileAnnotationSource {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FileAnnotationSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FileAnnotationSource> for super::FileAnnotationSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileAnnotationSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                filename: value.filename?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FileAnnotationSource> for FileAnnotationSource {
        fn from(value: super::FileAnnotationSource) -> Self {
            Self {
                filename: Ok(value.filename),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileUploadParam {
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        max_file_size: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        max_files: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FileUploadParam {
        fn default() -> Self {
            Self {
                enabled: Ok(Default::default()),
                max_file_size: Ok(Default::default()),
                max_files: Ok(Default::default()),
            }
        }
    }
    impl FileUploadParam {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
        pub fn max_file_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_file_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_file_size: {e}"));
            self
        }
        pub fn max_files<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_files: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FileUploadParam> for super::FileUploadParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileUploadParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                max_file_size: value.max_file_size?,
                max_files: value.max_files?,
            })
        }
    }
    impl ::std::convert::From<super::FileUploadParam> for FileUploadParam {
        fn from(value: super::FileUploadParam) -> Self {
            Self {
                enabled: Ok(value.enabled),
                max_file_size: Ok(value.max_file_size),
                max_files: Ok(value.max_files),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetThreadMethodParams {
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetThreadMethodParams {
        fn default() -> Self {
            Self {
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl GetThreadMethodParams {
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
    impl ::std::convert::TryFrom<GetThreadMethodParams> for super::GetThreadMethodParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetThreadMethodParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetThreadMethodParams> for GetThreadMethodParams {
        fn from(value: super::GetThreadMethodParams) -> Self {
            Self {
                thread_id: Ok(value.thread_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HistoryParam {
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        recent_threads: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for HistoryParam {
        fn default() -> Self {
            Self {
                enabled: Ok(Default::default()),
                recent_threads: Ok(Default::default()),
            }
        }
    }
    impl HistoryParam {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
        pub fn recent_threads<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.recent_threads = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recent_threads: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HistoryParam> for super::HistoryParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HistoryParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
                recent_threads: value.recent_threads?,
            })
        }
    }
    impl ::std::convert::From<super::HistoryParam> for HistoryParam {
        fn from(value: super::HistoryParam) -> Self {
            Self {
                enabled: Ok(value.enabled),
                recent_threads: Ok(value.recent_threads),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InferenceOptions {
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tool_choice:
            ::std::result::Result<::std::option::Option<super::ToolChoice>, ::std::string::String>,
    }
    impl ::std::default::Default for InferenceOptions {
        fn default() -> Self {
            Self {
                model: Err("no value supplied for model".to_string()),
                tool_choice: Err("no value supplied for tool_choice".to_string()),
            }
        }
    }
    impl InferenceOptions {
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
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolChoice>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_choice: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InferenceOptions> for super::InferenceOptions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InferenceOptions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                model: value.model?,
                tool_choice: value.tool_choice?,
            })
        }
    }
    impl ::std::convert::From<super::InferenceOptions> for InferenceOptions {
        fn from(value: super::InferenceOptions) -> Self {
            Self {
                model: Ok(value.model),
                tool_choice: Ok(value.tool_choice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListThreadItemsMethodParams {
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
            ::std::option::Option<::openai_api_types::common::OrderEnum>,
            ::std::string::String,
        >,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListThreadItemsMethodParams {
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
    impl ListThreadItemsMethodParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::OrderEnum>,
            >,
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
    impl ::std::convert::TryFrom<ListThreadItemsMethodParams> for super::ListThreadItemsMethodParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListThreadItemsMethodParams,
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
    impl ::std::convert::From<super::ListThreadItemsMethodParams> for ListThreadItemsMethodParams {
        fn from(value: super::ListThreadItemsMethodParams) -> Self {
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
    pub struct ListThreadsMethodParams {
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
            ::std::option::Option<::openai_api_types::common::OrderEnum>,
            ::std::string::String,
        >,
        user: ::std::result::Result<
            ::std::option::Option<super::ListThreadsMethodParamsUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListThreadsMethodParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl ListThreadsMethodParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::OrderEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListThreadsMethodParamsUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListThreadsMethodParams> for super::ListThreadsMethodParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListThreadsMethodParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                order: value.order?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::ListThreadsMethodParams> for ListThreadsMethodParams {
        fn from(value: super::ListThreadsMethodParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                order: Ok(value.order),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LockedStatus {
        reason: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LockedStatusType, ::std::string::String>,
    }
    impl ::std::default::Default for LockedStatus {
        fn default() -> Self {
            Self {
                reason: Err("no value supplied for reason".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LockedStatus {
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LockedStatusType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LockedStatus> for super::LockedStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LockedStatus,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reason: value.reason?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LockedStatus> for LockedStatus {
        fn from(value: super::LockedStatus) -> Self {
            Self {
                reason: Ok(value.reason),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RateLimitsParam {
        max_requests_per_1_minute: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RateLimitsParam {
        fn default() -> Self {
            Self {
                max_requests_per_1_minute: Ok(Default::default()),
            }
        }
    }
    impl RateLimitsParam {
        pub fn max_requests_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_minute: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<RateLimitsParam> for super::RateLimitsParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RateLimitsParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_requests_per_1_minute: value.max_requests_per_1_minute?,
            })
        }
    }
    impl ::std::convert::From<super::RateLimitsParam> for RateLimitsParam {
        fn from(value: super::RateLimitsParam) -> Self {
            Self {
                max_requests_per_1_minute: Ok(value.max_requests_per_1_minute),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponseOutputText {
        annotations: ::std::result::Result<
            ::std::vec::Vec<super::ResponseOutputTextAnnotationsItem>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::ResponseOutputTextType, ::std::string::String>,
    }
    impl ::std::default::Default for ResponseOutputText {
        fn default() -> Self {
            Self {
                annotations: Err("no value supplied for annotations".to_string()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ResponseOutputText {
        pub fn annotations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ResponseOutputTextAnnotationsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.annotations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annotations: {e}"));
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
            T: ::std::convert::TryInto<super::ResponseOutputTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponseOutputText> for super::ResponseOutputText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponseOutputText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                annotations: value.annotations?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ResponseOutputText> for ResponseOutputText {
        fn from(value: super::ResponseOutputText) -> Self {
            Self {
                annotations: Ok(value.annotations),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskGroupItem {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::TaskGroupItemObject, ::std::string::String>,
        tasks: ::std::result::Result<::std::vec::Vec<super::TaskGroupTask>, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::TaskGroupItemType, ::std::string::String>,
    }
    impl ::std::default::Default for TaskGroupItem {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                tasks: Err("no value supplied for tasks".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TaskGroupItem {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskGroupItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn tasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaskGroupTask>>,
            T::Error: ::std::fmt::Display,
        {
            self.tasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tasks: {e}"));
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
            T: ::std::convert::TryInto<super::TaskGroupItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskGroupItem> for super::TaskGroupItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskGroupItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                tasks: value.tasks?,
                thread_id: value.thread_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaskGroupItem> for TaskGroupItem {
        fn from(value: super::TaskGroupItem) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                tasks: Ok(value.tasks),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskGroupTask {
        heading: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::TaskType, ::std::string::String>,
    }
    impl ::std::default::Default for TaskGroupTask {
        fn default() -> Self {
            Self {
                heading: Err("no value supplied for heading".to_string()),
                summary: Err("no value supplied for summary".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TaskGroupTask {
        pub fn heading<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.heading = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heading: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskGroupTask> for super::TaskGroupTask {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskGroupTask,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                heading: value.heading?,
                summary: value.summary?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaskGroupTask> for TaskGroupTask {
        fn from(value: super::TaskGroupTask) -> Self {
            Self {
                heading: Ok(value.heading),
                summary: Ok(value.summary),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskItem {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        heading: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::TaskItemObject, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        task_type: ::std::result::Result<super::TaskType, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::TaskItemType, ::std::string::String>,
    }
    impl ::std::default::Default for TaskItem {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                heading: Err("no value supplied for heading".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                summary: Err("no value supplied for summary".to_string()),
                task_type: Err("no value supplied for task_type".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TaskItem {
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
        pub fn heading<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.heading = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heading: {e}"));
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
            T: ::std::convert::TryInto<super::TaskItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
        pub fn task_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskType>,
            T::Error: ::std::fmt::Display,
        {
            self.task_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_type: {e}"));
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
            T: ::std::convert::TryInto<super::TaskItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskItem> for super::TaskItem {
        type Error = super::error::ConversionError;
        fn try_from(value: TaskItem) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                heading: value.heading?,
                id: value.id?,
                object: value.object?,
                summary: value.summary?,
                task_type: value.task_type?,
                thread_id: value.thread_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaskItem> for TaskItem {
        fn from(value: super::TaskItem) -> Self {
            Self {
                created_at: Ok(value.created_at),
                heading: Ok(value.heading),
                id: Ok(value.id),
                object: Ok(value.object),
                summary: Ok(value.summary),
                task_type: Ok(value.task_type),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadItemListResource {
        data: ::std::result::Result<::std::vec::Vec<super::ThreadItem>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ThreadItemListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadItemListResource {
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
    impl ThreadItemListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ThreadItem>>,
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
            T: ::std::convert::TryInto<super::ThreadItemListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadItemListResource> for super::ThreadItemListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadItemListResource,
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
    impl ::std::convert::From<super::ThreadItemListResource> for ThreadItemListResource {
        fn from(value: super::ThreadItemListResource) -> Self {
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
    pub struct ThreadListResource {
        data: ::std::result::Result<::std::vec::Vec<super::ThreadResource>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ThreadListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadListResource {
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
    impl ThreadListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ThreadResource>>,
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
            T: ::std::convert::TryInto<super::ThreadListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadListResource> for super::ThreadListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadListResource,
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
    impl ::std::convert::From<super::ThreadListResource> for ThreadListResource {
        fn from(value: super::ThreadListResource) -> Self {
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
    pub struct ThreadResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ThreadResourceObject, ::std::string::String>,
        status: ::std::result::Result<super::ThreadResourceStatus, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                status: Err("no value supplied for status".to_string()),
                title: Err("no value supplied for title".to_string()),
                user: Err("no value supplied for user".to_string()),
            }
        }
    }
    impl ThreadResource {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadResourceStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadResource> for super::ThreadResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                status: value.status?,
                title: value.title?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadResource> for ThreadResource {
        fn from(value: super::ThreadResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                status: Ok(value.status),
                title: Ok(value.title),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolChoice {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolChoice {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl ToolChoice {
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
    impl ::std::convert::TryFrom<ToolChoice> for super::ToolChoice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::ToolChoice> for ToolChoice {
        fn from(value: super::ToolChoice) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UrlAnnotation {
        source: ::std::result::Result<super::UrlAnnotationSource, ::std::string::String>,
        type_: ::std::result::Result<super::UrlAnnotationType, ::std::string::String>,
    }
    impl ::std::default::Default for UrlAnnotation {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UrlAnnotation {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UrlAnnotationSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UrlAnnotationType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UrlAnnotation> for super::UrlAnnotation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UrlAnnotation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UrlAnnotation> for UrlAnnotation {
        fn from(value: super::UrlAnnotation) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UrlAnnotationSource {
        type_: ::std::result::Result<super::UrlAnnotationSourceType, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UrlAnnotationSource {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl UrlAnnotationSource {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UrlAnnotationSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
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
    impl ::std::convert::TryFrom<UrlAnnotationSource> for super::UrlAnnotationSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UrlAnnotationSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::UrlAnnotationSource> for UrlAnnotationSource {
        fn from(value: super::UrlAnnotationSource) -> Self {
            Self {
                type_: Ok(value.type_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserMessageInputText {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::UserMessageInputTextType, ::std::string::String>,
    }
    impl ::std::default::Default for UserMessageInputText {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UserMessageInputText {
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
            T: ::std::convert::TryInto<super::UserMessageInputTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserMessageInputText> for super::UserMessageInputText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserMessageInputText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UserMessageInputText> for UserMessageInputText {
        fn from(value: super::UserMessageInputText) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserMessageItem {
        attachments:
            ::std::result::Result<::std::vec::Vec<super::Attachment>, ::std::string::String>,
        content: ::std::result::Result<
            ::std::vec::Vec<super::UserMessageItemContentItem>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        inference_options: ::std::result::Result<
            ::std::option::Option<super::InferenceOptions>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::UserMessageItemObject, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::UserMessageItemType, ::std::string::String>,
    }
    impl ::std::default::Default for UserMessageItem {
        fn default() -> Self {
            Self {
                attachments: Err("no value supplied for attachments".to_string()),
                content: Err("no value supplied for content".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                inference_options: Err("no value supplied for inference_options".to_string()),
                object: Err("no value supplied for object".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UserMessageItem {
        pub fn attachments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Attachment>>,
            T::Error: ::std::fmt::Display,
        {
            self.attachments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attachments: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UserMessageItemContentItem>>,
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
        pub fn inference_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InferenceOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.inference_options = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inference_options: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserMessageItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
            T: ::std::convert::TryInto<super::UserMessageItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserMessageItem> for super::UserMessageItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserMessageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attachments: value.attachments?,
                content: value.content?,
                created_at: value.created_at?,
                id: value.id?,
                inference_options: value.inference_options?,
                object: value.object?,
                thread_id: value.thread_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UserMessageItem> for UserMessageItem {
        fn from(value: super::UserMessageItem) -> Self {
            Self {
                attachments: Ok(value.attachments),
                content: Ok(value.content),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                inference_options: Ok(value.inference_options),
                object: Ok(value.object),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserMessageQuotedText {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::UserMessageQuotedTextType, ::std::string::String>,
    }
    impl ::std::default::Default for UserMessageQuotedText {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UserMessageQuotedText {
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
            T: ::std::convert::TryInto<super::UserMessageQuotedTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserMessageQuotedText> for super::UserMessageQuotedText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserMessageQuotedText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::UserMessageQuotedText> for UserMessageQuotedText {
        fn from(value: super::UserMessageQuotedText) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WidgetMessageItem {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::WidgetMessageItemObject, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::WidgetMessageItemType, ::std::string::String>,
        widget: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WidgetMessageItem {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                widget: Err("no value supplied for widget".to_string()),
            }
        }
    }
    impl WidgetMessageItem {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WidgetMessageItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
            T: ::std::convert::TryInto<super::WidgetMessageItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn widget<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.widget = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for widget: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WidgetMessageItem> for super::WidgetMessageItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WidgetMessageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                thread_id: value.thread_id?,
                type_: value.type_?,
                widget: value.widget?,
            })
        }
    }
    impl ::std::convert::From<super::WidgetMessageItem> for WidgetMessageItem {
        fn from(value: super::WidgetMessageItem) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                thread_id: Ok(value.thread_id),
                type_: Ok(value.type_),
                widget: Ok(value.widget),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkflowParam {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        state_variables: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                super::WorkflowParamStateVariablesValue,
            >,
            ::std::string::String,
        >,
        tracing: ::std::result::Result<
            ::std::option::Option<super::WorkflowTracingParam>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WorkflowParam {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                state_variables: Ok(Default::default()),
                tracing: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl WorkflowParam {
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
        pub fn state_variables<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::WorkflowParamStateVariablesValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.state_variables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state_variables: {e}"));
            self
        }
        pub fn tracing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WorkflowTracingParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.tracing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tracing: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkflowParam> for super::WorkflowParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkflowParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                state_variables: value.state_variables?,
                tracing: value.tracing?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::WorkflowParam> for WorkflowParam {
        fn from(value: super::WorkflowParam) -> Self {
            Self {
                id: Ok(value.id),
                state_variables: Ok(value.state_variables),
                tracing: Ok(value.tracing),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkflowTracingParam {
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for WorkflowTracingParam {
        fn default() -> Self {
            Self {
                enabled: Ok(Default::default()),
            }
        }
    }
    impl WorkflowTracingParam {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkflowTracingParam> for super::WorkflowTracingParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkflowTracingParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enabled: value.enabled?,
            })
        }
    }
    impl ::std::convert::From<super::WorkflowTracingParam> for WorkflowTracingParam {
        fn from(value: super::WorkflowTracingParam) -> Self {
            Self {
                enabled: Ok(value.enabled),
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
