pub use crate::common::{AdditionalTools, AdditionalToolsItemParam, Annotation, ApplyPatchCallOutputStatus, ApplyPatchCallOutputStatusParam, ApplyPatchCallStatus, ApplyPatchCallStatusParam, ApplyPatchCreateFileOperation, ApplyPatchCreateFileOperationParam, ApplyPatchDeleteFileOperation, ApplyPatchDeleteFileOperationParam, ApplyPatchOperationParam, ApplyPatchToolCall, ApplyPatchToolCallItemParam, ApplyPatchToolCallOutput, ApplyPatchToolCallOutputItemParam, ApplyPatchToolParam, ApplyPatchUpdateFileOperation, ApplyPatchUpdateFileOperationParam, ApproximateLocation, AutoCodeInterpreterToolParam, CallableToolAllowedCaller, ClickButtonType, ClickParam, CodeInterpreterOutputImage, CodeInterpreterOutputLogs, CodeInterpreterTool, CodeInterpreterToolCall, CompactionBody, CompactionSummaryItemParam, CompactionTriggerItemParam, ComparisonFilter, CompoundFilter, ComputerAction, ComputerActionList, ComputerCallOutputItemParam, ComputerCallOutputStatus, ComputerCallSafetyCheckParam, ComputerEnvironment, ComputerScreenshotContent, ComputerScreenshotImage, ComputerTool, ComputerToolCall, ComputerToolCallOutput, ComputerToolCallOutputResource, ComputerUsePreviewTool, ContainerAutoParam, ContainerFileCitationBody, ContainerMemoryLimit, ContainerNetworkPolicyAllowlistParam, ContainerNetworkPolicyDisabledParam, ContainerNetworkPolicyDomainSecretParam, ContainerReferenceParam, ContainerReferenceResource, CoordParam, CustomGrammarFormatParam, CustomTextFormatParam, CustomToolCall, CustomToolCallOutput, CustomToolParam, DetailEnum, DirectToolCallCaller, DirectToolCallCallerParam, DoubleClickAction, DragParam, EasyInputMessage, EmptyModelParam, Error, ErrorResponse, FileCitationBody, FileDetailEnum, FileInputDetail, FilePath, FileSearchTool, FileSearchToolCall, Filters, FunctionAndCustomToolCallOutput, FunctionCallItemStatus, FunctionCallOutputItemParam, FunctionCallOutputStatusEnum, FunctionCallStatus, FunctionShellAction, FunctionShellActionParam, FunctionShellCall, FunctionShellCallItemParam, FunctionShellCallItemStatus, FunctionShellCallOutput, FunctionShellCallOutputContent, FunctionShellCallOutputContentParam, FunctionShellCallOutputExitOutcome, FunctionShellCallOutputExitOutcomeParam, FunctionShellCallOutputItemParam, FunctionShellCallOutputOutcomeParam, FunctionShellCallOutputStatusEnum, FunctionShellCallOutputTimeoutOutcome, FunctionShellCallOutputTimeoutOutcomeParam, FunctionShellCallStatus, FunctionShellToolParam, FunctionTool, FunctionToolCall, FunctionToolCallOutput, FunctionToolCallOutputResource, FunctionToolCallResource, FunctionToolParam, GrammarSyntax1, HttpError, HybridSearchOptions, ImageBackground, ImageDetail, ImageGenActionEnum, ImageGenTool, ImageGenToolCall, ImageOutputFormat, IncludeEnum, InlineSkillParam, InlineSkillSourceParam, InputContent, InputFidelity, InputFileContent, InputFileContentParam, InputImageContent, InputImageContentParamAutoParam, InputItem, InputMessage, InputMessageContentList, InputTextContent, InputTextContentParam, Item, ItemReferenceParam, KeyPressAction, LocalEnvironmentParam, LocalEnvironmentResource, LocalShellExecAction, LocalShellToolCall, LocalShellToolCallOutput, LocalShellToolParam, LocalSkillParam, LogProb, McpApprovalRequest, McpApprovalResponse, McpApprovalResponseResource, McpListTools, McpListToolsTool, McpProtocolError, McpTool, McpToolCall, McpToolCallError, McpToolCallStatus, McpToolExecutionError, McpToolFilter, Message, MessagePhase, MessagePhase2, MessageRole, MessageStatus, Metadata, MisalignmentErrorDetailsResource, MoveParam, NamespaceToolParam, OutputMessage, OutputMessageContent, OutputTextContent, Program, ProgramItemParam, ProgramOutput, ProgramOutputItemParam, ProgramOutputItemStatus, ProgramOutputStatus, ProgramToolCallCaller, ProgramToolCallCallerParam, ProgrammaticToolCallingParam, PromptCacheBreakpointConfig, PromptCacheBreakpointParam, RankerVersionType, RankingOptions, ReasoningEffort, ReasoningItem, ReasoningTextContent, RefusalContent, ResponseConfigurationUpdate, ResponseConfigurationUpdateItemParam, ScreenshotParam, ScrollParam, SearchContentType, SearchContextSize, SkillReferenceParam, SummaryTextContent, TextContent, Tool, ToolCallCaller, ToolCallCallerParam, ToolSearchCall, ToolSearchCallItemParam, ToolSearchExecutionType, ToolSearchOutput, ToolSearchOutputFunctionToolParam, ToolSearchOutputItemParam, ToolSearchOutputNamespaceToolParam, ToolSearchOutputTool, ToolSearchToolParam, TopLogProb, TypeParam, UrlCitationBody, VectorStoreFileAttributes, WaitParam, WebSearchActionFind, WebSearchActionOpenPage, WebSearchActionSearch, WebSearchApproximateLocation, WebSearchCallStatus, WebSearchPreviewTool, WebSearchTool, WebSearchToolCall, MisalignmentErrorType, MisalignmentSteer};
#[doc = "A single item within a conversation. The set of possible types are the same as the `output` type of a [Response object](https://developers.openai.com/api/reference/resources/responses#%28resource%29%20responses%20%3E%20%28model%29%20response%20%3E%20%28schema%29%20%3E%20%28property%29%20output)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ConversationItem {
    Variant0(::openai_api_types::common::Message),
    Variant1(::openai_api_types::common::FunctionToolCallResource),
    Variant2(::openai_api_types::common::FunctionToolCallOutputResource),
    Variant3(::openai_api_types::common::FileSearchToolCall),
    Variant4(::openai_api_types::common::WebSearchToolCall),
    Variant5(::openai_api_types::common::ImageGenToolCall),
    Variant6(::openai_api_types::common::ComputerToolCall),
    Variant7(::openai_api_types::common::ComputerToolCallOutputResource),
    Variant8(::openai_api_types::common::ToolSearchCall),
    Variant9(::openai_api_types::common::ToolSearchOutput),
    Variant10(::openai_api_types::common::AdditionalTools),
    Variant11(::openai_api_types::common::ResponseConfigurationUpdate),
    Variant12(::openai_api_types::common::ReasoningItem),
    Variant13(::openai_api_types::common::Program),
    Variant14(::openai_api_types::common::ProgramOutput),
    Variant15(::openai_api_types::common::CompactionBody),
    Variant16(::openai_api_types::common::CodeInterpreterToolCall),
    Variant17(::openai_api_types::common::LocalShellToolCall),
    Variant18(::openai_api_types::common::LocalShellToolCallOutput),
    Variant19(::openai_api_types::common::FunctionShellCall),
    Variant20(::openai_api_types::common::FunctionShellCallOutput),
    Variant21(::openai_api_types::common::ApplyPatchToolCall),
    Variant22(::openai_api_types::common::ApplyPatchToolCallOutput),
    Variant23(::openai_api_types::common::McpListTools),
    Variant24(::openai_api_types::common::McpApprovalRequest),
    Variant25(::openai_api_types::common::McpApprovalResponseResource),
    Variant26(::openai_api_types::common::McpToolCall),
    Variant27(::openai_api_types::common::CustomToolCall),
    Variant28(::openai_api_types::common::CustomToolCallOutput),
}
impl ::std::convert::From<::openai_api_types::common::Message> for ConversationItem {
    fn from(value: ::openai_api_types::common::Message) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::FunctionToolCallResource>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::FunctionToolCallResource) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::FunctionToolCallOutputResource>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::FunctionToolCallOutputResource) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::FileSearchToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::FileSearchToolCall) -> Self {
        Self::Variant3(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::WebSearchToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::WebSearchToolCall) -> Self {
        Self::Variant4(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ImageGenToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::ImageGenToolCall) -> Self {
        Self::Variant5(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ComputerToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::ComputerToolCall) -> Self {
        Self::Variant6(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ComputerToolCallOutputResource>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::ComputerToolCallOutputResource) -> Self {
        Self::Variant7(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ToolSearchCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::ToolSearchCall) -> Self {
        Self::Variant8(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ToolSearchOutput> for ConversationItem {
    fn from(value: ::openai_api_types::common::ToolSearchOutput) -> Self {
        Self::Variant9(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::AdditionalTools> for ConversationItem {
    fn from(value: ::openai_api_types::common::AdditionalTools) -> Self {
        Self::Variant10(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseConfigurationUpdate>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::ResponseConfigurationUpdate) -> Self {
        Self::Variant11(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ReasoningItem> for ConversationItem {
    fn from(value: ::openai_api_types::common::ReasoningItem) -> Self {
        Self::Variant12(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::Program> for ConversationItem {
    fn from(value: ::openai_api_types::common::Program) -> Self {
        Self::Variant13(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ProgramOutput> for ConversationItem {
    fn from(value: ::openai_api_types::common::ProgramOutput) -> Self {
        Self::Variant14(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::CompactionBody> for ConversationItem {
    fn from(value: ::openai_api_types::common::CompactionBody) -> Self {
        Self::Variant15(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::CodeInterpreterToolCall>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::CodeInterpreterToolCall) -> Self {
        Self::Variant16(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::LocalShellToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::LocalShellToolCall) -> Self {
        Self::Variant17(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::LocalShellToolCallOutput>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::LocalShellToolCallOutput) -> Self {
        Self::Variant18(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::FunctionShellCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::FunctionShellCall) -> Self {
        Self::Variant19(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::FunctionShellCallOutput>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::FunctionShellCallOutput) -> Self {
        Self::Variant20(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ApplyPatchToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::ApplyPatchToolCall) -> Self {
        Self::Variant21(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ApplyPatchToolCallOutput>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::ApplyPatchToolCallOutput) -> Self {
        Self::Variant22(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::McpListTools> for ConversationItem {
    fn from(value: ::openai_api_types::common::McpListTools) -> Self {
        Self::Variant23(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::McpApprovalRequest> for ConversationItem {
    fn from(value: ::openai_api_types::common::McpApprovalRequest) -> Self {
        Self::Variant24(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::McpApprovalResponseResource>
    for ConversationItem
{
    fn from(value: ::openai_api_types::common::McpApprovalResponseResource) -> Self {
        Self::Variant25(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::McpToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::McpToolCall) -> Self {
        Self::Variant26(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::CustomToolCall> for ConversationItem {
    fn from(value: ::openai_api_types::common::CustomToolCall) -> Self {
        Self::Variant27(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::CustomToolCallOutput> for ConversationItem {
    fn from(value: ::openai_api_types::common::CustomToolCallOutput) -> Self {
        Self::Variant28(value)
    }
}
#[doc = "A list of Conversation items."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ConversationItemList {
    #[doc = "A list of conversation items."]
    pub data: ::std::vec::Vec<ConversationItem>,
    #[doc = "The ID of the first item in the list."]
    pub first_id: ::std::string::String,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[doc = "The ID of the last item in the list."]
    pub last_id: ::std::string::String,
    #[doc = "The type of object returned, must be `list`."]
    pub object: ConversationItemListObject,
}
impl ConversationItemList {
    pub fn builder() -> builder::ConversationItemList {
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
pub enum ConversationItemListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ConversationItemListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ConversationItemListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConversationItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConversationItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ConversationResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ConversationResource {
    #[doc = "The time at which the conversation was created, measured in seconds since the Unix epoch."]
    pub created_at: i64,
    #[doc = "The unique ID of the conversation."]
    pub id: ::std::string::String,
    #[doc = "Set of 16 key-value pairs that can be attached to an object. This can be         useful for storing additional information about the object in a structured         format, and querying for objects via API or the dashboard.\n        Keys are strings with a maximum length of 64 characters. Values are strings         with a maximum length of 512 characters."]
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The object type, which is always `conversation`."]
    pub object: ConversationResourceObject,
}
impl ConversationResource {
    pub fn builder() -> builder::ConversationResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `conversation`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ConversationResourceObject {
    #[serde(rename = "conversation")]
    Conversation,
}
impl ::std::fmt::Display for ConversationResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Conversation => f.write_str("conversation"),
        }
    }
}
impl ::std::str::FromStr for ConversationResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation" => Ok(Self::Conversation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConversationResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConversationResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ConversationsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ConversationsApiSchema {
    #[serde(rename = "CreateConversationError400")]
    pub create_conversation_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationError429")]
    pub create_conversation_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationError500")]
    pub create_conversation_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationItemsError400")]
    pub create_conversation_items_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationItemsError404")]
    pub create_conversation_items_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationItemsError429")]
    pub create_conversation_items_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationItemsError500")]
    pub create_conversation_items_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateConversationItemsParams")]
    pub create_conversation_items_params: CreateConversationItemsParams,
    #[serde(rename = "CreateConversationItemsRequest")]
    pub create_conversation_items_request: CreateConversationItemsRequest,
    #[serde(rename = "CreateConversationItemsResponse")]
    pub create_conversation_items_response: ConversationItemList,
    #[serde(rename = "CreateConversationRequest")]
    pub create_conversation_request: CreateConversationBody,
    #[serde(rename = "CreateConversationResponse")]
    pub create_conversation_response: ConversationResource,
    #[serde(rename = "DeleteConversationError404")]
    pub delete_conversation_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationError429")]
    pub delete_conversation_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationItemError400")]
    pub delete_conversation_item_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationItemError404")]
    pub delete_conversation_item_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationItemError429")]
    pub delete_conversation_item_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationItemError500")]
    pub delete_conversation_item_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteConversationItemParams")]
    pub delete_conversation_item_params: DeleteConversationItemParams,
    #[serde(rename = "DeleteConversationItemResponse")]
    pub delete_conversation_item_response: ConversationResource,
    #[serde(rename = "DeleteConversationParams")]
    pub delete_conversation_params: DeleteConversationParams,
    #[serde(rename = "DeleteConversationResponse")]
    pub delete_conversation_response: DeletedConversationResource,
    #[serde(rename = "GetConversationError404")]
    pub get_conversation_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetConversationError429")]
    pub get_conversation_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetConversationItemError404")]
    pub get_conversation_item_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetConversationItemError429")]
    pub get_conversation_item_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetConversationItemParams")]
    pub get_conversation_item_params: GetConversationItemParams,
    #[serde(rename = "GetConversationItemResponse")]
    pub get_conversation_item_response: ConversationItem,
    #[serde(rename = "GetConversationParams")]
    pub get_conversation_params: GetConversationParams,
    #[serde(rename = "GetConversationResponse")]
    pub get_conversation_response: ConversationResource,
    #[serde(rename = "ListConversationItemsError400")]
    pub list_conversation_items_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListConversationItemsError404")]
    pub list_conversation_items_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListConversationItemsError429")]
    pub list_conversation_items_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListConversationItemsParams")]
    pub list_conversation_items_params: ListConversationItemsParams,
    #[serde(rename = "ListConversationItemsResponse")]
    pub list_conversation_items_response: ConversationItemList,
    #[serde(rename = "UpdateConversationError400")]
    pub update_conversation_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateConversationError404")]
    pub update_conversation_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateConversationError429")]
    pub update_conversation_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateConversationError500")]
    pub update_conversation_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateConversationParams")]
    pub update_conversation_params: UpdateConversationParams,
    #[serde(rename = "UpdateConversationRequest")]
    pub update_conversation_request: UpdateConversationBody,
    #[serde(rename = "UpdateConversationResponse")]
    pub update_conversation_response: ConversationResource,
}
impl ConversationsApiSchema {
    pub fn builder() -> builder::ConversationsApiSchema {
        Default::default()
    }
}
#[doc = "`CreateConversationBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateConversationBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::vec::Vec<::openai_api_types::common::InputItem>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
}
impl CreateConversationBody {
    pub fn builder() -> builder::CreateConversationBody {
        Default::default()
    }
}
#[doc = "`CreateConversationItemsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateConversationItemsParams {
    pub conversation_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
}
impl CreateConversationItemsParams {
    pub fn builder() -> builder::CreateConversationItemsParams {
        Default::default()
    }
}
#[doc = "`CreateConversationItemsRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateConversationItemsRequest {
    #[doc = "The items to add to the conversation. You may add up to 20 items at a time.\n"]
    pub items: ::std::vec::Vec<::openai_api_types::common::InputItem>,
}
impl CreateConversationItemsRequest {
    pub fn builder() -> builder::CreateConversationItemsRequest {
        Default::default()
    }
}
#[doc = "`DeleteConversationItemParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteConversationItemParams {
    pub conversation_id: ::std::string::String,
    pub item_id: ::std::string::String,
}
impl DeleteConversationItemParams {
    pub fn builder() -> builder::DeleteConversationItemParams {
        Default::default()
    }
}
#[doc = "`DeleteConversationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteConversationParams {
    pub conversation_id: ::std::string::String,
}
impl DeleteConversationParams {
    pub fn builder() -> builder::DeleteConversationParams {
        Default::default()
    }
}
#[doc = "`DeletedConversationResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedConversationResource {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeletedConversationResourceObject,
}
impl DeletedConversationResource {
    pub fn builder() -> builder::DeletedConversationResource {
        Default::default()
    }
}
#[doc = "`DeletedConversationResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedConversationResourceObject {
    #[serde(rename = "conversation.deleted")]
    ConversationDeleted,
}
impl ::std::fmt::Display for DeletedConversationResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConversationDeleted => f.write_str("conversation.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedConversationResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "conversation.deleted" => Ok(Self::ConversationDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedConversationResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedConversationResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetConversationItemParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetConversationItemParams {
    pub conversation_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
    pub item_id: ::std::string::String,
}
impl GetConversationItemParams {
    pub fn builder() -> builder::GetConversationItemParams {
        Default::default()
    }
}
#[doc = "`GetConversationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetConversationParams {
    pub conversation_id: ::std::string::String,
}
impl GetConversationParams {
    pub fn builder() -> builder::GetConversationParams {
        Default::default()
    }
}
#[doc = "`ListConversationItemsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListConversationItemsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub conversation_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListConversationItemsParamsOrder>,
}
impl ListConversationItemsParams {
    pub fn builder() -> builder::ListConversationItemsParams {
        Default::default()
    }
}
#[doc = "`ListConversationItemsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListConversationItemsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListConversationItemsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListConversationItemsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListConversationItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListConversationItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "`UpdateConversationBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateConversationBody {
    pub metadata: ::openai_api_types::common::Metadata,
}
impl UpdateConversationBody {
    pub fn builder() -> builder::UpdateConversationBody {
        Default::default()
    }
}
#[doc = "`UpdateConversationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateConversationParams {
    pub conversation_id: ::std::string::String,
}
impl UpdateConversationParams {
    pub fn builder() -> builder::UpdateConversationParams {
        Default::default()
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
#[doc = "The approximate location of the user.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct WebSearchApproximateLocationValue {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
    #[doc = "The type of location approximation. Always `approximate`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<WebSearchApproximateLocationValueType>,
}
impl WebSearchApproximateLocationValue {
    pub fn builder() -> builder::WebSearchApproximateLocationValue {
        Default::default()
    }
}
#[doc = "The type of location approximation. Always `approximate`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchApproximateLocationValueType {
    #[serde(rename = "approximate")]
    Approximate,
}
impl ::std::fmt::Display for WebSearchApproximateLocationValueType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Approximate => f.write_str("approximate"),
        }
    }
}
impl ::std::str::FromStr for WebSearchApproximateLocationValueType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "approximate" => Ok(Self::Approximate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchApproximateLocationValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchApproximateLocationValueType {
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
    pub struct ConversationItemList {
        data:
            ::std::result::Result<::std::vec::Vec<super::ConversationItem>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ConversationItemListObject, ::std::string::String>,
    }
    impl ::std::default::Default for ConversationItemList {
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
    impl ConversationItemList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ConversationItem>>,
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
            T: ::std::convert::TryInto<super::ConversationItemListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ConversationItemList> for super::ConversationItemList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConversationItemList,
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
    impl ::std::convert::From<super::ConversationItemList> for ConversationItemList {
        fn from(value: super::ConversationItemList) -> Self {
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
    pub struct ConversationResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ConversationResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ConversationResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ConversationResource {
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
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ConversationResource> for super::ConversationResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConversationResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                metadata: value.metadata?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ConversationResource> for ConversationResource {
        fn from(value: super::ConversationResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConversationsApiSchema {
        create_conversation_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_items_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_items_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_items_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_items_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_conversation_items_params:
            ::std::result::Result<super::CreateConversationItemsParams, ::std::string::String>,
        create_conversation_items_request:
            ::std::result::Result<super::CreateConversationItemsRequest, ::std::string::String>,
        create_conversation_items_response:
            ::std::result::Result<super::ConversationItemList, ::std::string::String>,
        create_conversation_request:
            ::std::result::Result<super::CreateConversationBody, ::std::string::String>,
        create_conversation_response:
            ::std::result::Result<super::ConversationResource, ::std::string::String>,
        delete_conversation_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_item_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_item_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_item_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_item_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_conversation_item_params:
            ::std::result::Result<super::DeleteConversationItemParams, ::std::string::String>,
        delete_conversation_item_response:
            ::std::result::Result<super::ConversationResource, ::std::string::String>,
        delete_conversation_params:
            ::std::result::Result<super::DeleteConversationParams, ::std::string::String>,
        delete_conversation_response:
            ::std::result::Result<super::DeletedConversationResource, ::std::string::String>,
        get_conversation_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_conversation_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_conversation_item_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_conversation_item_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_conversation_item_params:
            ::std::result::Result<super::GetConversationItemParams, ::std::string::String>,
        get_conversation_item_response:
            ::std::result::Result<super::ConversationItem, ::std::string::String>,
        get_conversation_params:
            ::std::result::Result<super::GetConversationParams, ::std::string::String>,
        get_conversation_response:
            ::std::result::Result<super::ConversationResource, ::std::string::String>,
        list_conversation_items_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_conversation_items_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_conversation_items_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_conversation_items_params:
            ::std::result::Result<super::ListConversationItemsParams, ::std::string::String>,
        list_conversation_items_response:
            ::std::result::Result<super::ConversationItemList, ::std::string::String>,
        update_conversation_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_conversation_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_conversation_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_conversation_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_conversation_params:
            ::std::result::Result<super::UpdateConversationParams, ::std::string::String>,
        update_conversation_request:
            ::std::result::Result<super::UpdateConversationBody, ::std::string::String>,
        update_conversation_response:
            ::std::result::Result<super::ConversationResource, ::std::string::String>,
    }
    impl ::std::default::Default for ConversationsApiSchema {
        fn default() -> Self {
            Self {
                create_conversation_error400: Err(
                    "no value supplied for create_conversation_error400".to_string(),
                ),
                create_conversation_error429: Err(
                    "no value supplied for create_conversation_error429".to_string(),
                ),
                create_conversation_error500: Err(
                    "no value supplied for create_conversation_error500".to_string(),
                ),
                create_conversation_items_error400: Err(
                    "no value supplied for create_conversation_items_error400".to_string(),
                ),
                create_conversation_items_error404: Err(
                    "no value supplied for create_conversation_items_error404".to_string(),
                ),
                create_conversation_items_error429: Err(
                    "no value supplied for create_conversation_items_error429".to_string(),
                ),
                create_conversation_items_error500: Err(
                    "no value supplied for create_conversation_items_error500".to_string(),
                ),
                create_conversation_items_params: Err(
                    "no value supplied for create_conversation_items_params".to_string(),
                ),
                create_conversation_items_request: Err(
                    "no value supplied for create_conversation_items_request".to_string(),
                ),
                create_conversation_items_response: Err(
                    "no value supplied for create_conversation_items_response".to_string(),
                ),
                create_conversation_request: Err(
                    "no value supplied for create_conversation_request".to_string(),
                ),
                create_conversation_response: Err(
                    "no value supplied for create_conversation_response".to_string(),
                ),
                delete_conversation_error404: Err(
                    "no value supplied for delete_conversation_error404".to_string(),
                ),
                delete_conversation_error429: Err(
                    "no value supplied for delete_conversation_error429".to_string(),
                ),
                delete_conversation_item_error400: Err(
                    "no value supplied for delete_conversation_item_error400".to_string(),
                ),
                delete_conversation_item_error404: Err(
                    "no value supplied for delete_conversation_item_error404".to_string(),
                ),
                delete_conversation_item_error429: Err(
                    "no value supplied for delete_conversation_item_error429".to_string(),
                ),
                delete_conversation_item_error500: Err(
                    "no value supplied for delete_conversation_item_error500".to_string(),
                ),
                delete_conversation_item_params: Err(
                    "no value supplied for delete_conversation_item_params".to_string(),
                ),
                delete_conversation_item_response: Err(
                    "no value supplied for delete_conversation_item_response".to_string(),
                ),
                delete_conversation_params: Err(
                    "no value supplied for delete_conversation_params".to_string()
                ),
                delete_conversation_response: Err(
                    "no value supplied for delete_conversation_response".to_string(),
                ),
                get_conversation_error404: Err(
                    "no value supplied for get_conversation_error404".to_string()
                ),
                get_conversation_error429: Err(
                    "no value supplied for get_conversation_error429".to_string()
                ),
                get_conversation_item_error404: Err(
                    "no value supplied for get_conversation_item_error404".to_string(),
                ),
                get_conversation_item_error429: Err(
                    "no value supplied for get_conversation_item_error429".to_string(),
                ),
                get_conversation_item_params: Err(
                    "no value supplied for get_conversation_item_params".to_string(),
                ),
                get_conversation_item_response: Err(
                    "no value supplied for get_conversation_item_response".to_string(),
                ),
                get_conversation_params: Err(
                    "no value supplied for get_conversation_params".to_string()
                ),
                get_conversation_response: Err(
                    "no value supplied for get_conversation_response".to_string()
                ),
                list_conversation_items_error400: Err(
                    "no value supplied for list_conversation_items_error400".to_string(),
                ),
                list_conversation_items_error404: Err(
                    "no value supplied for list_conversation_items_error404".to_string(),
                ),
                list_conversation_items_error429: Err(
                    "no value supplied for list_conversation_items_error429".to_string(),
                ),
                list_conversation_items_params: Err(
                    "no value supplied for list_conversation_items_params".to_string(),
                ),
                list_conversation_items_response: Err(
                    "no value supplied for list_conversation_items_response".to_string(),
                ),
                update_conversation_error400: Err(
                    "no value supplied for update_conversation_error400".to_string(),
                ),
                update_conversation_error404: Err(
                    "no value supplied for update_conversation_error404".to_string(),
                ),
                update_conversation_error429: Err(
                    "no value supplied for update_conversation_error429".to_string(),
                ),
                update_conversation_error500: Err(
                    "no value supplied for update_conversation_error500".to_string(),
                ),
                update_conversation_params: Err(
                    "no value supplied for update_conversation_params".to_string()
                ),
                update_conversation_request: Err(
                    "no value supplied for update_conversation_request".to_string(),
                ),
                update_conversation_response: Err(
                    "no value supplied for update_conversation_response".to_string(),
                ),
            }
        }
    }
    impl ConversationsApiSchema {
        pub fn create_conversation_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_error400: {e}")
            });
            self
        }
        pub fn create_conversation_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_error429: {e}")
            });
            self
        }
        pub fn create_conversation_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_error500: {e}")
            });
            self
        }
        pub fn create_conversation_items_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_error400: {e}"
                )
            });
            self
        }
        pub fn create_conversation_items_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_error404: {e}"
                )
            });
            self
        }
        pub fn create_conversation_items_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_error429: {e}"
                )
            });
            self
        }
        pub fn create_conversation_items_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_error500 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_error500: {e}"
                )
            });
            self
        }
        pub fn create_conversation_items_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateConversationItemsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_items_params: {e}")
            });
            self
        }
        pub fn create_conversation_items_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateConversationItemsRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_request: {e}"
                )
            });
            self
        }
        pub fn create_conversation_items_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationItemList>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_items_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_conversation_items_response: {e}"
                )
            });
            self
        }
        pub fn create_conversation_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateConversationBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_request: {e}")
            });
            self
        }
        pub fn create_conversation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_conversation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_conversation_response: {e}")
            });
            self
        }
        pub fn delete_conversation_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_conversation_error404: {e}")
            });
            self
        }
        pub fn delete_conversation_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_conversation_error429: {e}")
            });
            self
        }
        pub fn delete_conversation_item_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_conversation_item_error400: {e}"
                )
            });
            self
        }
        pub fn delete_conversation_item_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_conversation_item_error404: {e}"
                )
            });
            self
        }
        pub fn delete_conversation_item_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_conversation_item_error429: {e}"
                )
            });
            self
        }
        pub fn delete_conversation_item_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_error500 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_conversation_item_error500: {e}"
                )
            });
            self
        }
        pub fn delete_conversation_item_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteConversationItemParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_conversation_item_params: {e}")
            });
            self
        }
        pub fn delete_conversation_item_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_item_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for delete_conversation_item_response: {e}"
                )
            });
            self
        }
        pub fn delete_conversation_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteConversationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_conversation_params: {e}")
            });
            self
        }
        pub fn delete_conversation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedConversationResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_conversation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_conversation_response: {e}")
            });
            self
        }
        pub fn get_conversation_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_error404: {e}")
            });
            self
        }
        pub fn get_conversation_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_error429: {e}")
            });
            self
        }
        pub fn get_conversation_item_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_item_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_item_error404: {e}")
            });
            self
        }
        pub fn get_conversation_item_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_item_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_item_error429: {e}")
            });
            self
        }
        pub fn get_conversation_item_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetConversationItemParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_item_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_item_params: {e}")
            });
            self
        }
        pub fn get_conversation_item_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationItem>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_item_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_item_response: {e}")
            });
            self
        }
        pub fn get_conversation_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetConversationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_params: {e}")
            });
            self
        }
        pub fn get_conversation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_conversation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_conversation_response: {e}")
            });
            self
        }
        pub fn list_conversation_items_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_conversation_items_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_conversation_items_error400: {e}")
            });
            self
        }
        pub fn list_conversation_items_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_conversation_items_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_conversation_items_error404: {e}")
            });
            self
        }
        pub fn list_conversation_items_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_conversation_items_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_conversation_items_error429: {e}")
            });
            self
        }
        pub fn list_conversation_items_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListConversationItemsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_conversation_items_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_conversation_items_params: {e}")
            });
            self
        }
        pub fn list_conversation_items_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationItemList>,
            T::Error: ::std::fmt::Display,
        {
            self.list_conversation_items_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_conversation_items_response: {e}")
            });
            self
        }
        pub fn update_conversation_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_error400: {e}")
            });
            self
        }
        pub fn update_conversation_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_error404: {e}")
            });
            self
        }
        pub fn update_conversation_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_error429: {e}")
            });
            self
        }
        pub fn update_conversation_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_error500: {e}")
            });
            self
        }
        pub fn update_conversation_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateConversationParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_params: {e}")
            });
            self
        }
        pub fn update_conversation_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateConversationBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_request: {e}")
            });
            self
        }
        pub fn update_conversation_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConversationResource>,
            T::Error: ::std::fmt::Display,
        {
            self.update_conversation_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_conversation_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ConversationsApiSchema> for super::ConversationsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConversationsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_conversation_error400: value.create_conversation_error400?,
                create_conversation_error429: value.create_conversation_error429?,
                create_conversation_error500: value.create_conversation_error500?,
                create_conversation_items_error400: value.create_conversation_items_error400?,
                create_conversation_items_error404: value.create_conversation_items_error404?,
                create_conversation_items_error429: value.create_conversation_items_error429?,
                create_conversation_items_error500: value.create_conversation_items_error500?,
                create_conversation_items_params: value.create_conversation_items_params?,
                create_conversation_items_request: value.create_conversation_items_request?,
                create_conversation_items_response: value.create_conversation_items_response?,
                create_conversation_request: value.create_conversation_request?,
                create_conversation_response: value.create_conversation_response?,
                delete_conversation_error404: value.delete_conversation_error404?,
                delete_conversation_error429: value.delete_conversation_error429?,
                delete_conversation_item_error400: value.delete_conversation_item_error400?,
                delete_conversation_item_error404: value.delete_conversation_item_error404?,
                delete_conversation_item_error429: value.delete_conversation_item_error429?,
                delete_conversation_item_error500: value.delete_conversation_item_error500?,
                delete_conversation_item_params: value.delete_conversation_item_params?,
                delete_conversation_item_response: value.delete_conversation_item_response?,
                delete_conversation_params: value.delete_conversation_params?,
                delete_conversation_response: value.delete_conversation_response?,
                get_conversation_error404: value.get_conversation_error404?,
                get_conversation_error429: value.get_conversation_error429?,
                get_conversation_item_error404: value.get_conversation_item_error404?,
                get_conversation_item_error429: value.get_conversation_item_error429?,
                get_conversation_item_params: value.get_conversation_item_params?,
                get_conversation_item_response: value.get_conversation_item_response?,
                get_conversation_params: value.get_conversation_params?,
                get_conversation_response: value.get_conversation_response?,
                list_conversation_items_error400: value.list_conversation_items_error400?,
                list_conversation_items_error404: value.list_conversation_items_error404?,
                list_conversation_items_error429: value.list_conversation_items_error429?,
                list_conversation_items_params: value.list_conversation_items_params?,
                list_conversation_items_response: value.list_conversation_items_response?,
                update_conversation_error400: value.update_conversation_error400?,
                update_conversation_error404: value.update_conversation_error404?,
                update_conversation_error429: value.update_conversation_error429?,
                update_conversation_error500: value.update_conversation_error500?,
                update_conversation_params: value.update_conversation_params?,
                update_conversation_request: value.update_conversation_request?,
                update_conversation_response: value.update_conversation_response?,
            })
        }
    }
    impl ::std::convert::From<super::ConversationsApiSchema> for ConversationsApiSchema {
        fn from(value: super::ConversationsApiSchema) -> Self {
            Self {
                create_conversation_error400: Ok(value.create_conversation_error400),
                create_conversation_error429: Ok(value.create_conversation_error429),
                create_conversation_error500: Ok(value.create_conversation_error500),
                create_conversation_items_error400: Ok(value.create_conversation_items_error400),
                create_conversation_items_error404: Ok(value.create_conversation_items_error404),
                create_conversation_items_error429: Ok(value.create_conversation_items_error429),
                create_conversation_items_error500: Ok(value.create_conversation_items_error500),
                create_conversation_items_params: Ok(value.create_conversation_items_params),
                create_conversation_items_request: Ok(value.create_conversation_items_request),
                create_conversation_items_response: Ok(value.create_conversation_items_response),
                create_conversation_request: Ok(value.create_conversation_request),
                create_conversation_response: Ok(value.create_conversation_response),
                delete_conversation_error404: Ok(value.delete_conversation_error404),
                delete_conversation_error429: Ok(value.delete_conversation_error429),
                delete_conversation_item_error400: Ok(value.delete_conversation_item_error400),
                delete_conversation_item_error404: Ok(value.delete_conversation_item_error404),
                delete_conversation_item_error429: Ok(value.delete_conversation_item_error429),
                delete_conversation_item_error500: Ok(value.delete_conversation_item_error500),
                delete_conversation_item_params: Ok(value.delete_conversation_item_params),
                delete_conversation_item_response: Ok(value.delete_conversation_item_response),
                delete_conversation_params: Ok(value.delete_conversation_params),
                delete_conversation_response: Ok(value.delete_conversation_response),
                get_conversation_error404: Ok(value.get_conversation_error404),
                get_conversation_error429: Ok(value.get_conversation_error429),
                get_conversation_item_error404: Ok(value.get_conversation_item_error404),
                get_conversation_item_error429: Ok(value.get_conversation_item_error429),
                get_conversation_item_params: Ok(value.get_conversation_item_params),
                get_conversation_item_response: Ok(value.get_conversation_item_response),
                get_conversation_params: Ok(value.get_conversation_params),
                get_conversation_response: Ok(value.get_conversation_response),
                list_conversation_items_error400: Ok(value.list_conversation_items_error400),
                list_conversation_items_error404: Ok(value.list_conversation_items_error404),
                list_conversation_items_error429: Ok(value.list_conversation_items_error429),
                list_conversation_items_params: Ok(value.list_conversation_items_params),
                list_conversation_items_response: Ok(value.list_conversation_items_response),
                update_conversation_error400: Ok(value.update_conversation_error400),
                update_conversation_error404: Ok(value.update_conversation_error404),
                update_conversation_error429: Ok(value.update_conversation_error429),
                update_conversation_error500: Ok(value.update_conversation_error500),
                update_conversation_params: Ok(value.update_conversation_params),
                update_conversation_request: Ok(value.update_conversation_request),
                update_conversation_response: Ok(value.update_conversation_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateConversationBody {
        items: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::openai_api_types::common::InputItem>>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateConversationBody {
        fn default() -> Self {
            Self {
                items: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl CreateConversationBody {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::openai_api_types::common::InputItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
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
    impl ::std::convert::TryFrom<CreateConversationBody> for super::CreateConversationBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateConversationBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::CreateConversationBody> for CreateConversationBody {
        fn from(value: super::CreateConversationBody) -> Self {
            Self {
                items: Ok(value.items),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateConversationItemsParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        include: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateConversationItemsParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
                include: Ok(Default::default()),
            }
        }
    }
    impl CreateConversationItemsParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::IncludeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateConversationItemsParams>
        for super::CreateConversationItemsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateConversationItemsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
                include: value.include?,
            })
        }
    }
    impl ::std::convert::From<super::CreateConversationItemsParams> for CreateConversationItemsParams {
        fn from(value: super::CreateConversationItemsParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
                include: Ok(value.include),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateConversationItemsRequest {
        items: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::InputItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateConversationItemsRequest {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl CreateConversationItemsRequest {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::InputItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateConversationItemsRequest>
        for super::CreateConversationItemsRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateConversationItemsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::CreateConversationItemsRequest>
        for CreateConversationItemsRequest
    {
        fn from(value: super::CreateConversationItemsRequest) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteConversationItemParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        item_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteConversationItemParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
                item_id: Err("no value supplied for item_id".to_string()),
            }
        }
    }
    impl DeleteConversationItemParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
        pub fn item_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteConversationItemParams> for super::DeleteConversationItemParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteConversationItemParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
                item_id: value.item_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteConversationItemParams> for DeleteConversationItemParams {
        fn from(value: super::DeleteConversationItemParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
                item_id: Ok(value.item_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteConversationParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteConversationParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
            }
        }
    }
    impl DeleteConversationParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteConversationParams> for super::DeleteConversationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteConversationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteConversationParams> for DeleteConversationParams {
        fn from(value: super::DeleteConversationParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedConversationResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::DeletedConversationResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedConversationResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedConversationResource {
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
            T: ::std::convert::TryInto<super::DeletedConversationResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedConversationResource> for super::DeletedConversationResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedConversationResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedConversationResource> for DeletedConversationResource {
        fn from(value: super::DeletedConversationResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetConversationItemParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        include: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
            ::std::string::String,
        >,
        item_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetConversationItemParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
                include: Ok(Default::default()),
                item_id: Err("no value supplied for item_id".to_string()),
            }
        }
    }
    impl GetConversationItemParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::IncludeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.include = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include: {e}"));
            self
        }
        pub fn item_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetConversationItemParams> for super::GetConversationItemParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetConversationItemParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
                include: value.include?,
                item_id: value.item_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetConversationItemParams> for GetConversationItemParams {
        fn from(value: super::GetConversationItemParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
                include: Ok(value.include),
                item_id: Ok(value.item_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetConversationParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetConversationParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
            }
        }
    }
    impl GetConversationParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetConversationParams> for super::GetConversationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetConversationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetConversationParams> for GetConversationParams {
        fn from(value: super::GetConversationParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListConversationItemsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        include: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::IncludeEnum>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListConversationItemsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListConversationItemsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                conversation_id: Err("no value supplied for conversation_id".to_string()),
                include: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListConversationItemsParams {
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
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
        pub fn include<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::IncludeEnum>>,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListConversationItemsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListConversationItemsParams> for super::ListConversationItemsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListConversationItemsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                conversation_id: value.conversation_id?,
                include: value.include?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListConversationItemsParams> for ListConversationItemsParams {
        fn from(value: super::ListConversationItemsParams) -> Self {
            Self {
                after: Ok(value.after),
                conversation_id: Ok(value.conversation_id),
                include: Ok(value.include),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateConversationBody {
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateConversationBody {
        fn default() -> Self {
            Self {
                metadata: Err("no value supplied for metadata".to_string()),
            }
        }
    }
    impl UpdateConversationBody {
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
    }
    impl ::std::convert::TryFrom<UpdateConversationBody> for super::UpdateConversationBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateConversationBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateConversationBody> for UpdateConversationBody {
        fn from(value: super::UpdateConversationBody) -> Self {
            Self {
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateConversationParams {
        conversation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateConversationParams {
        fn default() -> Self {
            Self {
                conversation_id: Err("no value supplied for conversation_id".to_string()),
            }
        }
    }
    impl UpdateConversationParams {
        pub fn conversation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateConversationParams> for super::UpdateConversationParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateConversationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation_id: value.conversation_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateConversationParams> for UpdateConversationParams {
        fn from(value: super::UpdateConversationParams) -> Self {
            Self {
                conversation_id: Ok(value.conversation_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebSearchApproximateLocationValue {
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
        type_: ::std::result::Result<
            ::std::option::Option<super::WebSearchApproximateLocationValueType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WebSearchApproximateLocationValue {
        fn default() -> Self {
            Self {
                city: Ok(Default::default()),
                country: Ok(Default::default()),
                region: Ok(Default::default()),
                timezone: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl WebSearchApproximateLocationValue {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::WebSearchApproximateLocationValueType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebSearchApproximateLocationValue>
        for super::WebSearchApproximateLocationValue
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebSearchApproximateLocationValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                city: value.city?,
                country: value.country?,
                region: value.region?,
                timezone: value.timezone?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::WebSearchApproximateLocationValue>
        for WebSearchApproximateLocationValue
    {
        fn from(value: super::WebSearchApproximateLocationValue) -> Self {
            Self {
                city: Ok(value.city),
                country: Ok(value.country),
                region: Ok(value.region),
                timezone: Ok(value.timezone),
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
