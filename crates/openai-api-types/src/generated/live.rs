pub use crate::common::{AdditionalToolsItemParam, Annotation, ApplyPatchCallOutputStatusParam, ApplyPatchCallStatusParam, ApplyPatchCreateFileOperationParam, ApplyPatchDeleteFileOperationParam, ApplyPatchOperationParam, ApplyPatchToolCallItemParam, ApplyPatchToolCallOutputItemParam, ApplyPatchToolParam, ApplyPatchUpdateFileOperationParam, ApproximateLocation, AutoCodeInterpreterToolParam, CallableToolAllowedCaller, ClickButtonType, ClickParam, CodeInterpreterOutputImage, CodeInterpreterOutputLogs, CodeInterpreterTool, CodeInterpreterToolCall, CompactionSummaryItemParam, CompactionTriggerItemParam, ComparisonFilter, CompoundFilter, ComputerAction, ComputerActionList, ComputerCallOutputItemParam, ComputerCallSafetyCheckParam, ComputerEnvironment, ComputerScreenshotImage, ComputerTool, ComputerToolCall, ComputerUsePreviewTool, ContainerAutoParam, ContainerFileCitationBody, ContainerMemoryLimit, ContainerNetworkPolicyAllowlistParam, ContainerNetworkPolicyDisabledParam, ContainerNetworkPolicyDomainSecretParam, ContainerReferenceParam, CoordParam, CustomGrammarFormatParam, CustomTextFormatParam, CustomToolCall, CustomToolCallOutput, CustomToolParam, DetailEnum, DirectToolCallCaller, DirectToolCallCallerParam, DoubleClickAction, DragParam, EasyInputMessage, EmptyModelParam, Error, ErrorResponse, FileCitationBody, FileDetailEnum, FileInputDetail, FilePath, FileSearchTool, FileSearchToolCall, Filters, FunctionAndCustomToolCallOutput, FunctionCallItemStatus, FunctionCallOutputItemParam, FunctionShellActionParam, FunctionShellCallItemParam, FunctionShellCallItemStatus, FunctionShellCallOutputContentParam, FunctionShellCallOutputExitOutcomeParam, FunctionShellCallOutputItemParam, FunctionShellCallOutputOutcomeParam, FunctionShellCallOutputTimeoutOutcomeParam, FunctionShellToolParam, FunctionTool, FunctionToolCall, FunctionToolParam, GrammarSyntax1, HttpError, HybridSearchOptions, ImageBackground, ImageDetail, ImageGenActionEnum, ImageGenTool, ImageGenToolCall, ImageOutputFormat, InlineSkillParam, InlineSkillSourceParam, InputContent, InputFidelity, InputFileContent, InputFileContentParam, InputImageContent, InputImageContentParamAutoParam, InputItem, InputMessage, InputMessageContentList, InputTextContent, InputTextContentParam, Item, ItemReferenceParam, KeyPressAction, LocalEnvironmentParam, LocalShellExecAction, LocalShellToolCall, LocalShellToolCallOutput, LocalShellToolParam, LocalSkillParam, LogProb, McpApprovalRequest, McpApprovalResponse, McpListTools, McpListToolsTool, McpProtocolError, McpTool, McpToolCall, McpToolCallError, McpToolCallStatus, McpToolExecutionError, McpToolFilter, MessagePhase, MisalignmentErrorDetailsResource, MoveParam, NamespaceToolParam, OutputMessage, OutputMessageContent, OutputTextContent, ProgramItemParam, ProgramOutputItemParam, ProgramOutputItemStatus, ProgramToolCallCaller, ProgramToolCallCallerParam, ProgrammaticToolCallingParam, PromptCacheBreakpointConfig, PromptCacheBreakpointParam, RankerVersionType, RankingOptions, ReasoningEffort, ReasoningItem, ReasoningTextContent, RefusalContent, ResponseConfigurationUpdateItemParam, ScreenshotParam, ScrollParam, SearchContentType, SearchContextSize, SkillReferenceParam, SummaryTextContent, Tool, ToolCallCaller, ToolCallCallerParam, ToolSearchCallItemParam, ToolSearchExecutionType, ToolSearchOutputFunctionToolParam, ToolSearchOutputItemParam, ToolSearchOutputNamespaceToolParam, ToolSearchOutputTool, ToolSearchToolParam, TopLogProb, TypeParam, UrlCitationBody, VectorStoreFileAttributes, WaitParam, WebSearchActionFind, WebSearchActionOpenPage, WebSearchActionSearch, WebSearchApproximateLocation, WebSearchCallStatus, WebSearchPreviewTool, WebSearchTool, WebSearchToolCall, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AcceptLiveSessionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AcceptLiveSessionParams {
    pub session_id: ::std::string::String,
}
impl AcceptLiveSessionParams {
    pub fn builder() -> builder::AcceptLiveSessionParams {
        Default::default()
    }
}
#[doc = "`Delegation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Delegation(pub ::std::option::Option<DelegationValue>);
impl ::std::ops::Deref for Delegation {
    type Target = ::std::option::Option<DelegationValue>;
    fn deref(&self) -> &::std::option::Option<DelegationValue> {
        &self.0
    }
}
impl ::std::convert::From<Delegation> for ::std::option::Option<DelegationValue> {
    fn from(value: Delegation) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<DelegationValue>> for Delegation {
    fn from(value: ::std::option::Option<DelegationValue>) -> Self {
        Self(value)
    }
}
#[doc = "Who handles tasks delegated by the Live model. Omitted or null selects your application; use `responses` to let the API manage a Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DelegationValue {
    ClientDelegationParam(LiveClientDelegationParam),
    ResponsesDelegationParam(LiveResponsesDelegationParam),
}
impl ::std::convert::From<LiveClientDelegationParam> for DelegationValue {
    fn from(value: LiveClientDelegationParam) -> Self {
        Self::ClientDelegationParam(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationParam> for DelegationValue {
    fn from(value: LiveResponsesDelegationParam) -> Self {
        Self::ResponsesDelegationParam(value)
    }
}
#[doc = "`DownloadLiveRecordingParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DownloadLiveRecordingParams {
    #[doc = "The ID of the stored Live session to download. Use the session ID returned when the session started with storage enabled."]
    pub session_id: DownloadLiveRecordingParamsSessionId,
}
impl DownloadLiveRecordingParams {
    pub fn builder() -> builder::DownloadLiveRecordingParams {
        Default::default()
    }
}
#[doc = "The ID of the stored Live session to download. Use the session ID returned when the session started with storage enabled."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DownloadLiveRecordingParamsSessionId(::std::string::String);
impl ::std::ops::Deref for DownloadLiveRecordingParamsSessionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DownloadLiveRecordingParamsSessionId> for ::std::string::String {
    fn from(value: DownloadLiveRecordingParamsSessionId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DownloadLiveRecordingParamsSessionId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^live_[A-Za-z0-9_-]{1,128}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^live_[A-Za-z0-9_-]{1,128}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DownloadLiveRecordingParamsSessionId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DownloadLiveRecordingParamsSessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DownloadLiveRecordingParamsSessionId {
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
#[doc = "`ForkLiveSessionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ForkLiveSessionParams {
    pub session_id: ::std::string::String,
}
impl ForkLiveSessionParams {
    pub fn builder() -> builder::ForkLiveSessionParams {
        Default::default()
    }
}
#[doc = "`HangupLiveSessionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HangupLiveSessionParams {
    pub session_id: ::std::string::String,
}
impl HangupLiveSessionParams {
    pub fn builder() -> builder::HangupLiveSessionParams {
        Default::default()
    }
}
#[doc = "Ordered text-only history supplied before startup. Supports developer, user, and assistant messages with one text part each; at most 128 messages and 8,192 rendered tokens in total."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Input(pub ::std::vec::Vec<LiveInitialItem>);
impl ::std::ops::Deref for Input {
    type Target = ::std::vec::Vec<LiveInitialItem>;
    fn deref(&self) -> &::std::vec::Vec<LiveInitialItem> {
        &self.0
    }
}
impl ::std::convert::From<Input> for ::std::vec::Vec<LiveInitialItem> {
    fn from(value: Input) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<LiveInitialItem>> for Input {
    fn from(value: ::std::vec::Vec<LiveInitialItem>) -> Self {
        Self(value)
    }
}
#[doc = "`Instructions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Instructions(pub ::std::option::Option<InstructionsValue>);
impl ::std::ops::Deref for Instructions {
    type Target = ::std::option::Option<InstructionsValue>;
    fn deref(&self) -> &::std::option::Option<InstructionsValue> {
        &self.0
    }
}
impl ::std::convert::From<Instructions> for ::std::option::Option<InstructionsValue> {
    fn from(value: Instructions) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<InstructionsValue>> for Instructions {
    fn from(value: ::std::option::Option<InstructionsValue>) -> Self {
        Self(value)
    }
}
#[doc = "Frontend instructions for voice, conversation, interruptions, and when to delegate. Start with the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting); put business rules and tool workflows in a separate [backend prompt](https://developers.openai.com/api/docs/guides/live-delegation#start-with-your-existing-backend-prompt). Limited to 16,384 client-supplied tokens. Omitted or blank instructions use server defaults. Immutable after startup."]
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
pub struct InstructionsValue(pub ::std::string::String);
impl ::std::ops::Deref for InstructionsValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstructionsValue> for ::std::string::String {
    fn from(value: InstructionsValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for InstructionsValue {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for InstructionsValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for InstructionsValue {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "A Live server event selector for the WebRTC frontend data channel."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveAllowedServerEventParam {
    #[doc = "The nested Responses event type. Required when type is 'response.event'; forbidden for other event types."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_event: ::std::option::Option<LiveAllowedServerEventParamResponseEvent>,
    #[doc = "The outer Live server event type. Use 'response.event' for Responses events."]
    #[serde(rename = "type")]
    pub type_: LiveAllowedServerEventParamType,
}
impl LiveAllowedServerEventParam {
    pub fn builder() -> builder::LiveAllowedServerEventParam {
        Default::default()
    }
}
#[doc = "The nested Responses event type. Required when type is 'response.event'; forbidden for other event types."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveAllowedServerEventParamResponseEvent(::std::string::String);
impl ::std::ops::Deref for LiveAllowedServerEventParamResponseEvent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveAllowedServerEventParamResponseEvent> for ::std::string::String {
    fn from(value: LiveAllowedServerEventParamResponseEvent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveAllowedServerEventParamResponseEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveAllowedServerEventParamResponseEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveAllowedServerEventParamResponseEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveAllowedServerEventParamResponseEvent {
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
#[doc = "The outer Live server event type. Use 'response.event' for Responses events."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveAllowedServerEventParamType(::std::string::String);
impl ::std::ops::Deref for LiveAllowedServerEventParamType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveAllowedServerEventParamType> for ::std::string::String {
    fn from(value: LiveAllowedServerEventParamType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveAllowedServerEventParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveAllowedServerEventParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveAllowedServerEventParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveAllowedServerEventParamType {
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
#[doc = "`LiveAllowedToolsChoiceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveAllowedToolsChoiceParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<LiveToolChoiceValueEnum>,
    #[doc = "The tools that the delegated Responses model may call."]
    pub tools: ::std::vec::Vec<LiveAllowedToolsChoiceParamToolsItem>,
    #[doc = "The tool choice type. Always `allowed_tools`."]
    #[serde(rename = "type")]
    pub type_: LiveAllowedToolsChoiceParamType,
}
impl LiveAllowedToolsChoiceParam {
    pub fn builder() -> builder::LiveAllowedToolsChoiceParam {
        Default::default()
    }
}
#[doc = "`LiveAllowedToolsChoiceParamToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveAllowedToolsChoiceParamToolsItem {
    FunctionToolChoiceParam(LiveFunctionToolChoiceParam),
    McpToolChoiceParam(LiveMcpToolChoiceParam),
    SpecificFileSearchParam(LiveSpecificFileSearchParam),
    SpecificWebSearchParam(LiveSpecificWebSearchParam),
    SpecificWebSearchPreviewParam(LiveSpecificWebSearchPreviewParam),
    SpecificImageGenParam(LiveSpecificImageGenParam),
    SpecificComputerParam(LiveSpecificComputerParam),
    SpecificCodeInterpreterParam(LiveSpecificCodeInterpreterParam),
    SpecificProgrammaticToolCallingParam(LiveSpecificProgrammaticToolCallingParam),
    SpecificFunctionShellParam(LiveSpecificFunctionShellParam),
    SpecificCustomToolParam(LiveSpecificCustomToolParam),
    SpecificApplyPatchParam(LiveSpecificApplyPatchParam),
}
impl ::std::convert::From<LiveFunctionToolChoiceParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveFunctionToolChoiceParam) -> Self {
        Self::FunctionToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveMcpToolChoiceParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveMcpToolChoiceParam) -> Self {
        Self::McpToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFileSearchParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificFileSearchParam) -> Self {
        Self::SpecificFileSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificWebSearchParam) -> Self {
        Self::SpecificWebSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchPreviewParam>
    for LiveAllowedToolsChoiceParamToolsItem
{
    fn from(value: LiveSpecificWebSearchPreviewParam) -> Self {
        Self::SpecificWebSearchPreviewParam(value)
    }
}
impl ::std::convert::From<LiveSpecificImageGenParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificImageGenParam) -> Self {
        Self::SpecificImageGenParam(value)
    }
}
impl ::std::convert::From<LiveSpecificComputerParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificComputerParam) -> Self {
        Self::SpecificComputerParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCodeInterpreterParam>
    for LiveAllowedToolsChoiceParamToolsItem
{
    fn from(value: LiveSpecificCodeInterpreterParam) -> Self {
        Self::SpecificCodeInterpreterParam(value)
    }
}
impl ::std::convert::From<LiveSpecificProgrammaticToolCallingParam>
    for LiveAllowedToolsChoiceParamToolsItem
{
    fn from(value: LiveSpecificProgrammaticToolCallingParam) -> Self {
        Self::SpecificProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFunctionShellParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificFunctionShellParam) -> Self {
        Self::SpecificFunctionShellParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCustomToolParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificCustomToolParam) -> Self {
        Self::SpecificCustomToolParam(value)
    }
}
impl ::std::convert::From<LiveSpecificApplyPatchParam> for LiveAllowedToolsChoiceParamToolsItem {
    fn from(value: LiveSpecificApplyPatchParam) -> Self {
        Self::SpecificApplyPatchParam(value)
    }
}
#[doc = "The tool choice type. Always `allowed_tools`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveAllowedToolsChoiceParamType {
    #[serde(rename = "allowed_tools")]
    AllowedTools,
}
impl ::std::fmt::Display for LiveAllowedToolsChoiceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllowedTools => f.write_str("allowed_tools"),
        }
    }
}
impl ::std::str::FromStr for LiveAllowedToolsChoiceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowed_tools" => Ok(Self::AllowedTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveAllowedToolsChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveAllowedToolsChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveApiSchema {
    #[serde(rename = "AcceptLiveSessionError400")]
    pub accept_live_session_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AcceptLiveSessionError404")]
    pub accept_live_session_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AcceptLiveSessionError409")]
    pub accept_live_session_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AcceptLiveSessionError429")]
    pub accept_live_session_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AcceptLiveSessionError503")]
    pub accept_live_session_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AcceptLiveSessionParams")]
    pub accept_live_session_params: AcceptLiveSessionParams,
    #[serde(rename = "AcceptLiveSessionRequest")]
    pub accept_live_session_request: LiveCallAcceptRequest,
    #[serde(rename = "CreateLiveError400")]
    pub create_live_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateLiveError413")]
    pub create_live_error413: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateLiveError429")]
    pub create_live_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateLiveError503")]
    pub create_live_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateLiveError504")]
    pub create_live_error504: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateLiveRequest")]
    pub create_live_request: LiveSessionCreateRequest,
    #[serde(rename = "CreateLiveResponse")]
    pub create_live_response: LiveSessionCreateResponse,
    #[serde(rename = "DownloadLiveRecordingError400")]
    pub download_live_recording_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadLiveRecordingError404")]
    pub download_live_recording_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadLiveRecordingError429")]
    pub download_live_recording_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadLiveRecordingError500")]
    pub download_live_recording_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadLiveRecordingError503")]
    pub download_live_recording_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DownloadLiveRecordingParams")]
    pub download_live_recording_params: DownloadLiveRecordingParams,
    #[serde(rename = "ForkLiveSessionError400")]
    pub fork_live_session_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ForkLiveSessionError413")]
    pub fork_live_session_error413: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ForkLiveSessionError429")]
    pub fork_live_session_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ForkLiveSessionError503")]
    pub fork_live_session_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ForkLiveSessionParams")]
    pub fork_live_session_params: ForkLiveSessionParams,
    #[serde(rename = "ForkLiveSessionRequest")]
    pub fork_live_session_request: LiveForkRequest,
    #[serde(rename = "ForkLiveSessionResponse")]
    pub fork_live_session_response: LiveCreateResponse,
    #[serde(rename = "HangupLiveSessionError404")]
    pub hangup_live_session_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "HangupLiveSessionParams")]
    pub hangup_live_session_params: HangupLiveSessionParams,
    #[serde(rename = "LiveClientEvent")]
    pub live_client_event: LiveClientEvent,
    #[serde(rename = "LiveForkClientEvent")]
    pub live_fork_client_event: LiveForkClientEvent,
    #[serde(rename = "LiveForkServerEvent")]
    pub live_fork_server_event: LiveForkServerEvent,
    #[serde(rename = "LiveServerEvent")]
    pub live_server_event: LiveServerEvent,
    #[serde(rename = "LiveSidebandClientEvent")]
    pub live_sideband_client_event: LiveSidebandClientEvent,
    #[serde(rename = "LiveSidebandServerEvent")]
    pub live_sideband_server_event: LiveSidebandServerEvent,
    #[serde(rename = "ReferLiveSessionError400")]
    pub refer_live_session_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ReferLiveSessionError404")]
    pub refer_live_session_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ReferLiveSessionParams")]
    pub refer_live_session_params: ReferLiveSessionParams,
    #[serde(rename = "ReferLiveSessionRequest")]
    pub refer_live_session_request: LiveCallReferRequest,
    #[serde(rename = "RejectLiveSessionError400")]
    pub reject_live_session_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RejectLiveSessionError404")]
    pub reject_live_session_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RejectLiveSessionError409")]
    pub reject_live_session_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RejectLiveSessionParams")]
    pub reject_live_session_params: RejectLiveSessionParams,
    #[serde(rename = "RejectLiveSessionRequest")]
    pub reject_live_session_request: LiveCallRejectRequest,
}
impl LiveApiSchema {
    pub fn builder() -> builder::LiveApiSchema {
        Default::default()
    }
}
#[doc = "`LiveApplyPatchToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveApplyPatchToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveApplyPatchToolInputParamType,
}
impl LiveApplyPatchToolInputParam {
    pub fn builder() -> builder::LiveApplyPatchToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveApplyPatchToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveApplyPatchToolInputParamType {
    #[serde(rename = "apply_patch")]
    ApplyPatch,
}
impl ::std::fmt::Display for LiveApplyPatchToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatch => f.write_str("apply_patch"),
        }
    }
}
impl ::std::str::FromStr for LiveApplyPatchToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch" => Ok(Self::ApplyPatch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveApplyPatchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveApplyPatchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Audio encoding and sample rate for audio sent and received over a Live WebSocket connection. WebRTC and SIP negotiate their media format separately."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveAudioFormat {
    PcmParam(LiveSessionAudioFormatPcmParam),
    PcmuParam(LiveSessionAudioFormatPcmuParam),
    PcmaParam(LiveSessionAudioFormatPcmaParam),
}
impl ::std::convert::From<LiveSessionAudioFormatPcmParam> for LiveAudioFormat {
    fn from(value: LiveSessionAudioFormatPcmParam) -> Self {
        Self::PcmParam(value)
    }
}
impl ::std::convert::From<LiveSessionAudioFormatPcmuParam> for LiveAudioFormat {
    fn from(value: LiveSessionAudioFormatPcmuParam) -> Self {
        Self::PcmuParam(value)
    }
}
impl ::std::convert::From<LiveSessionAudioFormatPcmaParam> for LiveAudioFormat {
    fn from(value: LiveSessionAudioFormatPcmaParam) -> Self {
        Self::PcmaParam(value)
    }
}
#[doc = "Accept an incoming SIP call with Live startup configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveCallAcceptRequest {
    #[doc = "Model and startup configuration for the Live session that answers the incoming SIP call."]
    pub session: LiveCallAcceptSession,
}
impl LiveCallAcceptRequest {
    pub fn builder() -> builder::LiveCallAcceptRequest {
        Default::default()
    }
}
#[doc = "Startup configuration for accepting an incoming Live SIP call. Follow the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) when writing frontend and backend instructions."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveCallAcceptSession {
    #[doc = "Startup audio output configuration. SIP negotiates the media format; audio.format is only accepted for primary WebSockets. Voice cannot change after startup."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<LiveMediaSessionAudioParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<Delegation>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<Input>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<Instructions>,
    #[doc = "The Live model to use for the accepted call."]
    pub model: LiveCallAcceptSessionModel,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<Store>,
    #[doc = "The session type. Always `live`."]
    #[serde(rename = "type")]
    pub type_: LiveCallAcceptSessionType,
}
impl LiveCallAcceptSession {
    pub fn builder() -> builder::LiveCallAcceptSession {
        Default::default()
    }
}
#[doc = "`LiveCallAcceptSessionModel`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveCallAcceptSessionModel(::std::string::String);
impl ::std::ops::Deref for LiveCallAcceptSessionModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveCallAcceptSessionModel> for ::std::string::String {
    fn from(value: LiveCallAcceptSessionModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveCallAcceptSessionModel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveCallAcceptSessionModel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCallAcceptSessionModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveCallAcceptSessionModel {
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
#[doc = "The session type. Always `live`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveCallAcceptSessionType {
    #[serde(rename = "live")]
    Live,
}
impl ::std::fmt::Display for LiveCallAcceptSessionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Live => f.write_str("live"),
        }
    }
}
impl ::std::str::FromStr for LiveCallAcceptSessionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "live" => Ok(Self::Live),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveCallAcceptSessionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCallAcceptSessionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Parameters used to transfer a Live SIP call to another destination."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveCallReferRequest {
    #[doc = "Nonblank URI for the SIP Refer-To header, such as tel:+14155550123 or sip:agent@example.com."]
    pub target_uri: LiveCallReferRequestTargetUri,
}
impl LiveCallReferRequest {
    pub fn builder() -> builder::LiveCallReferRequest {
        Default::default()
    }
}
#[doc = "`LiveCallReferRequestTargetUri`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveCallReferRequestTargetUri(::std::string::String);
impl ::std::ops::Deref for LiveCallReferRequestTargetUri {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveCallReferRequestTargetUri> for ::std::string::String {
    fn from(value: LiveCallReferRequestTargetUri) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveCallReferRequestTargetUri {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("\\S").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"\\S\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveCallReferRequestTargetUri {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCallReferRequestTargetUri {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveCallReferRequestTargetUri {
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
#[doc = "Parameters used to reject an incoming Live SIP call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveCallRejectRequest {
    #[doc = "SIP rejection status sent to the caller. This field is required."]
    pub status_code: i64,
}
impl LiveCallRejectRequest {
    pub fn builder() -> builder::LiveCallRejectRequest {
        Default::default()
    }
}
#[doc = "Startup-only capabilities for an untrusted frontend attached to a unified WebRTC session. Trusted sideband connections are unaffected."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveClientConfigParam {
    #[doc = "Client and server event permissions for the WebRTC frontend data channel."]
    pub data_channel: LiveDataChannelConfigParam,
}
impl LiveClientConfigParam {
    pub fn builder() -> builder::LiveClientConfigParam {
        Default::default()
    }
}
#[doc = "Delegate tasks to your application. The Live session emits delegation events that your backend handles."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveClientDelegationParam {
    #[doc = "The delegation owner. Always `client` for tasks handled by your application."]
    #[serde(rename = "type")]
    pub type_: LiveClientDelegationParamType,
}
impl LiveClientDelegationParam {
    pub fn builder() -> builder::LiveClientDelegationParam {
        Default::default()
    }
}
#[doc = "The delegation owner. Always `client` for tasks handled by your application."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveClientDelegationParamType {
    #[serde(rename = "client")]
    Client,
}
impl ::std::fmt::Display for LiveClientDelegationParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Client => f.write_str("client"),
        }
    }
}
impl ::std::str::FromStr for LiveClientDelegationParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client" => Ok(Self::Client),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveClientDelegationParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveClientDelegationParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Client events for Live. Initialize a primary WebSocket with session.start and wait for session.started. WebRTC creation already starts the session. Audio append is primary WebSocket-only. See the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) before writing frontend instructions and delegation policies."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveClientEvent {
    SessionStartEvent(LiveSessionStartEvent),
    SessionUpdateParam(LiveSessionUpdateParam),
    InputAudioAppendEvent(LiveInputAudioAppendEvent),
    InputAudioMuteParam(LiveInputAudioMuteParam),
    InputAudioUnmuteParam(LiveInputAudioUnmuteParam),
    InstructionsAppendParam(LiveInstructionsAppendParam),
    ThinkingAppendParam(LiveThinkingAppendParam),
    CommentaryAppendParam(LiveCommentaryAppendParam),
    ResponseItemCreateParam(LiveResponseItemCreateParam),
    ResponseCreateParam(LiveResponseCreateParam),
    SessionCloseParam(LiveSessionCloseParam),
}
impl ::std::convert::From<LiveSessionStartEvent> for LiveClientEvent {
    fn from(value: LiveSessionStartEvent) -> Self {
        Self::SessionStartEvent(value)
    }
}
impl ::std::convert::From<LiveSessionUpdateParam> for LiveClientEvent {
    fn from(value: LiveSessionUpdateParam) -> Self {
        Self::SessionUpdateParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioAppendEvent> for LiveClientEvent {
    fn from(value: LiveInputAudioAppendEvent) -> Self {
        Self::InputAudioAppendEvent(value)
    }
}
impl ::std::convert::From<LiveInputAudioMuteParam> for LiveClientEvent {
    fn from(value: LiveInputAudioMuteParam) -> Self {
        Self::InputAudioMuteParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioUnmuteParam> for LiveClientEvent {
    fn from(value: LiveInputAudioUnmuteParam) -> Self {
        Self::InputAudioUnmuteParam(value)
    }
}
impl ::std::convert::From<LiveInstructionsAppendParam> for LiveClientEvent {
    fn from(value: LiveInstructionsAppendParam) -> Self {
        Self::InstructionsAppendParam(value)
    }
}
impl ::std::convert::From<LiveThinkingAppendParam> for LiveClientEvent {
    fn from(value: LiveThinkingAppendParam) -> Self {
        Self::ThinkingAppendParam(value)
    }
}
impl ::std::convert::From<LiveCommentaryAppendParam> for LiveClientEvent {
    fn from(value: LiveCommentaryAppendParam) -> Self {
        Self::CommentaryAppendParam(value)
    }
}
impl ::std::convert::From<LiveResponseItemCreateParam> for LiveClientEvent {
    fn from(value: LiveResponseItemCreateParam) -> Self {
        Self::ResponseItemCreateParam(value)
    }
}
impl ::std::convert::From<LiveResponseCreateParam> for LiveClientEvent {
    fn from(value: LiveResponseCreateParam) -> Self {
        Self::ResponseCreateParam(value)
    }
}
impl ::std::convert::From<LiveSessionCloseParam> for LiveClientEvent {
    fn from(value: LiveSessionCloseParam) -> Self {
        Self::SessionCloseParam(value)
    }
}
#[doc = "`LiveCodeInterpreterToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCodeInterpreterToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveCodeInterpreterToolInputParamType,
}
impl LiveCodeInterpreterToolInputParam {
    pub fn builder() -> builder::LiveCodeInterpreterToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveCodeInterpreterToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveCodeInterpreterToolInputParamType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for LiveCodeInterpreterToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for LiveCodeInterpreterToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveCodeInterpreterToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCodeInterpreterToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Provide context the Live model can communicate to the user, optionally for an existing client delegation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCommentaryAppendParam {
    #[doc = "Speakable context for the Live model, limited to 500 tokens. Use this for a result the model should communicate; use session.thinking.append for silent context."]
    pub content: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub delegation_id: ::std::option::Option<LiveCommentaryAppendParamDelegationId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveCommentaryAppendParamEventId>,
    #[doc = "The Live client event type. Always `session.commentary.append`."]
    #[serde(rename = "type")]
    pub type_: LiveCommentaryAppendParamType,
}
impl LiveCommentaryAppendParam {
    pub fn builder() -> builder::LiveCommentaryAppendParam {
        Default::default()
    }
}
#[doc = "Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveCommentaryAppendParamDelegationId(::std::string::String);
impl ::std::ops::Deref for LiveCommentaryAppendParamDelegationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveCommentaryAppendParamDelegationId> for ::std::string::String {
    fn from(value: LiveCommentaryAppendParamDelegationId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveCommentaryAppendParamDelegationId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveCommentaryAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCommentaryAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveCommentaryAppendParamDelegationId {
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
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveCommentaryAppendParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveCommentaryAppendParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveCommentaryAppendParamEventId> for ::std::string::String {
    fn from(value: LiveCommentaryAppendParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveCommentaryAppendParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveCommentaryAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCommentaryAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveCommentaryAppendParamEventId {
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
#[doc = "The Live client event type. Always `session.commentary.append`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveCommentaryAppendParamType {
    #[serde(rename = "session.commentary.append")]
    SessionCommentaryAppend,
}
impl ::std::fmt::Display for LiveCommentaryAppendParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCommentaryAppend => f.write_str("session.commentary.append"),
        }
    }
}
impl ::std::str::FromStr for LiveCommentaryAppendParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.commentary.append" => Ok(Self::SessionCommentaryAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveCommentaryAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCommentaryAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session.commentary.append command is accepted into the Live session timeline. Acknowledges the added commentary without guaranteeing exact wording or completed audio playback."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCommentaryAppended {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms."]
    pub end_ms: i64,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The start of this event on the Live session timeline, in milliseconds from the beginning of the session."]
    pub start_ms: i64,
    #[doc = "The event type, always `session.commentary.appended`."]
    #[serde(rename = "type")]
    pub type_: LiveCommentaryAppendedType,
}
impl LiveCommentaryAppended {
    pub fn builder() -> builder::LiveCommentaryAppended {
        Default::default()
    }
}
#[doc = "The event type, always `session.commentary.appended`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveCommentaryAppendedType {
    #[serde(rename = "session.commentary.appended")]
    SessionCommentaryAppended,
}
impl ::std::fmt::Display for LiveCommentaryAppendedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCommentaryAppended => f.write_str("session.commentary.appended"),
        }
    }
}
impl ::std::str::FromStr for LiveCommentaryAppendedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.commentary.appended" => Ok(Self::SessionCommentaryAppended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveCommentaryAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCommentaryAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveComputerToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveComputerToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveComputerToolInputParamType,
}
impl LiveComputerToolInputParam {
    pub fn builder() -> builder::LiveComputerToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveComputerToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveComputerToolInputParamType {
    #[serde(rename = "computer")]
    Computer,
}
impl ::std::fmt::Display for LiveComputerToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Computer => f.write_str("computer"),
        }
    }
}
impl ::std::str::FromStr for LiveComputerToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer" => Ok(Self::Computer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveComputerToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveComputerToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveContainerMemoryLimit`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveContainerMemoryLimit {
    #[serde(rename = "1g")]
    X1g,
    #[serde(rename = "4g")]
    X4g,
    #[serde(rename = "16g")]
    X16g,
    #[serde(rename = "64g")]
    X64g,
}
impl ::std::fmt::Display for LiveContainerMemoryLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1g => f.write_str("1g"),
            Self::X4g => f.write_str("4g"),
            Self::X16g => f.write_str("16g"),
            Self::X64g => f.write_str("64g"),
        }
    }
}
impl ::std::str::FromStr for LiveContainerMemoryLimit {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1g" => Ok(Self::X1g),
            "4g" => Ok(Self::X4g),
            "16g" => Ok(Self::X16g),
            "64g" => Ok(Self::X64g),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveContainerMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveContainerMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveContainerNetworkPolicyDisabledParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveContainerNetworkPolicyDisabledParam {
    #[doc = "Disable outbound network access. Always `disabled`."]
    #[serde(rename = "type")]
    pub type_: LiveContainerNetworkPolicyDisabledParamType,
}
impl LiveContainerNetworkPolicyDisabledParam {
    pub fn builder() -> builder::LiveContainerNetworkPolicyDisabledParam {
        Default::default()
    }
}
#[doc = "Disable outbound network access. Always `disabled`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveContainerNetworkPolicyDisabledParamType {
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for LiveContainerNetworkPolicyDisabledParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for LiveContainerNetworkPolicyDisabledParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveContainerNetworkPolicyDisabledParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveContainerNetworkPolicyDisabledParamType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveContainerReferenceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveContainerReferenceParam {
    #[doc = "The ID of the referenced container."]
    pub container_id: ::std::string::String,
    #[doc = "References a container created with the /v1/containers endpoint"]
    #[serde(rename = "type")]
    pub type_: LiveContainerReferenceParamType,
}
impl LiveContainerReferenceParam {
    pub fn builder() -> builder::LiveContainerReferenceParam {
        Default::default()
    }
}
#[doc = "References a container created with the /v1/containers endpoint"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveContainerReferenceParamType {
    #[serde(rename = "container_reference")]
    ContainerReference,
}
impl ::std::fmt::Display for LiveContainerReferenceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerReference => f.write_str("container_reference"),
        }
    }
}
impl ::std::str::FromStr for LiveContainerReferenceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_reference" => Ok(Self::ContainerReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveContainerReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveContainerReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The latest measured context-window usage of the Live model. This is separate from billing usage and delegated Responses token usage."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveContextWindowUsage {
    #[doc = "The latest active context token count divided by the Live model context limit. Can decrease after compaction and may lag between measured audio frames."]
    pub usage_ratio: f64,
}
impl LiveContextWindowUsage {
    pub fn builder() -> builder::LiveContextWindowUsage {
        Default::default()
    }
}
#[doc = "The created Live session identifier and WebRTC answer. Apply transport.sdp as the peer's remote answer and wait for session.started on the data channel before sending commands."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCreateResponse {
    pub session: LiveCreateResponseSession,
    #[doc = "WebRTC transport with the SDP answer."]
    pub transport: LiveWebRtcTransport,
}
impl LiveCreateResponse {
    pub fn builder() -> builder::LiveCreateResponse {
        Default::default()
    }
}
#[doc = "The newly created Live session. Use its ID for session controls and sideband connections."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCreateResponseSession {
    #[doc = "Opaque session identifier. Preserve the returned value unchanged, including its prefix."]
    pub id: ::std::string::String,
}
impl LiveCreateResponseSession {
    pub fn builder() -> builder::LiveCreateResponseSession {
        Default::default()
    }
}
#[doc = "`LiveCustomToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCustomToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveCustomToolInputParamType,
}
impl LiveCustomToolInputParam {
    pub fn builder() -> builder::LiveCustomToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveCustomToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveCustomToolInputParamType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for LiveCustomToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for LiveCustomToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveCustomToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCustomToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveCustomVoiceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCustomVoiceParam {
    pub id: LiveCustomVoiceParamId,
}
impl LiveCustomVoiceParam {
    pub fn builder() -> builder::LiveCustomVoiceParam {
        Default::default()
    }
}
#[doc = "`LiveCustomVoiceParamId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveCustomVoiceParamId(::std::string::String);
impl ::std::ops::Deref for LiveCustomVoiceParamId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveCustomVoiceParamId> for ::std::string::String {
    fn from(value: LiveCustomVoiceParamId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveCustomVoiceParamId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveCustomVoiceParamId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveCustomVoiceParamId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveCustomVoiceParamId {
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
#[doc = "Control which Live events an untrusted WebRTC frontend can send and receive over its data channel. These restrictions do not apply to trusted sideband connections."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveDataChannelConfigParam {
    #[doc = "Client event types that the frontend data channel may send. Use 'all' to allow every client event; an empty array allows none. Omission preserves the existing allow-all behavior."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_client_events: ::std::option::Option<LiveDataChannelConfigParamAllowedClientEvents>,
    #[doc = "Server events that may be sent to the frontend data channel. Use 'all' to allow every server event; an empty array allows none. Omission preserves the existing allow-all behavior. Responses events use an object with type 'response.event' and a response_event selector."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_server_events: ::std::option::Option<LiveDataChannelConfigParamAllowedServerEvents>,
}
impl LiveDataChannelConfigParam {
    pub fn builder() -> builder::LiveDataChannelConfigParam {
        Default::default()
    }
}
#[doc = "Client event types that the frontend data channel may send. Use 'all' to allow every client event; an empty array allows none. Omission preserves the existing allow-all behavior."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveDataChannelConfigParamAllowedClientEvents {
    String(LiveDataChannelConfigParamAllowedClientEventsString),
    Array(::std::vec::Vec<LiveDataChannelConfigParamAllowedClientEventsArrayItem>),
}
impl ::std::convert::From<LiveDataChannelConfigParamAllowedClientEventsString>
    for LiveDataChannelConfigParamAllowedClientEvents
{
    fn from(value: LiveDataChannelConfigParamAllowedClientEventsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<LiveDataChannelConfigParamAllowedClientEventsArrayItem>>
    for LiveDataChannelConfigParamAllowedClientEvents
{
    fn from(
        value: ::std::vec::Vec<LiveDataChannelConfigParamAllowedClientEventsArrayItem>,
    ) -> Self {
        Self::Array(value)
    }
}
#[doc = "A Live client event type allowed on the frontend data channel, such as `session.input_audio.mute`."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveDataChannelConfigParamAllowedClientEventsArrayItem(::std::string::String);
impl ::std::ops::Deref for LiveDataChannelConfigParamAllowedClientEventsArrayItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveDataChannelConfigParamAllowedClientEventsArrayItem>
    for ::std::string::String
{
    fn from(value: LiveDataChannelConfigParamAllowedClientEventsArrayItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveDataChannelConfigParamAllowedClientEventsArrayItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^(?:error|info|[a-z][a-z0-9_-]*(?:\\.[a-z0-9_-]+)+)$")
                    .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?:error|info|[a-z][a-z0-9_-]*(?:\\.[a-z0-9_-]+)+)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveDataChannelConfigParamAllowedClientEventsArrayItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveDataChannelConfigParamAllowedClientEventsArrayItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveDataChannelConfigParamAllowedClientEventsArrayItem {
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
#[doc = "`LiveDataChannelConfigParamAllowedClientEventsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveDataChannelConfigParamAllowedClientEventsString {
    #[serde(rename = "all")]
    All,
}
impl ::std::fmt::Display for LiveDataChannelConfigParamAllowedClientEventsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
        }
    }
}
impl ::std::str::FromStr for LiveDataChannelConfigParamAllowedClientEventsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveDataChannelConfigParamAllowedClientEventsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveDataChannelConfigParamAllowedClientEventsString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Server events that may be sent to the frontend data channel. Use 'all' to allow every server event; an empty array allows none. Omission preserves the existing allow-all behavior. Responses events use an object with type 'response.event' and a response_event selector."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveDataChannelConfigParamAllowedServerEvents {
    String(LiveDataChannelConfigParamAllowedServerEventsString),
    Array(::std::vec::Vec<LiveAllowedServerEventParam>),
}
impl ::std::convert::From<LiveDataChannelConfigParamAllowedServerEventsString>
    for LiveDataChannelConfigParamAllowedServerEvents
{
    fn from(value: LiveDataChannelConfigParamAllowedServerEventsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<LiveAllowedServerEventParam>>
    for LiveDataChannelConfigParamAllowedServerEvents
{
    fn from(value: ::std::vec::Vec<LiveAllowedServerEventParam>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`LiveDataChannelConfigParamAllowedServerEventsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveDataChannelConfigParamAllowedServerEventsString {
    #[serde(rename = "all")]
    All,
}
impl ::std::fmt::Display for LiveDataChannelConfigParamAllowedServerEventsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
        }
    }
}
impl ::std::str::FromStr for LiveDataChannelConfigParamAllowedServerEventsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveDataChannelConfigParamAllowedServerEventsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveDataChannelConfigParamAllowedServerEventsString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when the Live model delegates work to your application or a Responses backend. Contains delegation metadata and the position on the session timeline where the work was delegated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveDelegationCreated {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The delegated work identifier and destination. This object contains metadata, not the task text."]
    pub delegation: LiveDelegationItem,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The position on the Live session timeline where the delegation was created, in milliseconds from the beginning of the session."]
    pub offset_ms: i64,
    #[doc = "The event type, always `session.delegation.created`."]
    #[serde(rename = "type")]
    pub type_: LiveDelegationCreatedType,
}
impl LiveDelegationCreated {
    pub fn builder() -> builder::LiveDelegationCreated {
        Default::default()
    }
}
#[doc = "The event type, always `session.delegation.created`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveDelegationCreatedType {
    #[serde(rename = "session.delegation.created")]
    SessionDelegationCreated,
}
impl ::std::fmt::Display for LiveDelegationCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionDelegationCreated => f.write_str("session.delegation.created"),
        }
    }
}
impl ::std::str::FromStr for LiveDelegationCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.delegation.created" => Ok(Self::SessionDelegationCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveDelegationCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveDelegationCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Metadata for work delegated by the Live model. Client delegations are handled by your application; Responses delegations run on the configured backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveDelegationItem {
    #[doc = "The unique ID of the delegation. Use this as delegation_id when replying to client-owned work or correlating Responses events."]
    pub id: ::std::string::String,
    #[doc = "The ID of the Responses API response associated with a Responses delegation. Omitted for client delegations."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_id: ::std::option::Option<::std::string::String>,
    #[doc = "Where the Live model delegated the work: `client` for your application, or `responses` for the configured Responses backend."]
    pub target: LiveDelegationItemTarget,
    #[doc = "The object type, always `delegation`."]
    #[serde(rename = "type")]
    pub type_: LiveDelegationItemType,
}
impl LiveDelegationItem {
    pub fn builder() -> builder::LiveDelegationItem {
        Default::default()
    }
}
#[doc = "Where the Live model delegated the work: `client` for your application, or `responses` for the configured Responses backend."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveDelegationItemTarget {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "responses")]
    Responses,
}
impl ::std::fmt::Display for LiveDelegationItemTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Client => f.write_str("client"),
            Self::Responses => f.write_str("responses"),
        }
    }
}
impl ::std::str::FromStr for LiveDelegationItemTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client" => Ok(Self::Client),
            "responses" => Ok(Self::Responses),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveDelegationItemTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveDelegationItemTarget {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, always `delegation`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveDelegationItemType {
    #[serde(rename = "delegation")]
    Delegation,
}
impl ::std::fmt::Display for LiveDelegationItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Delegation => f.write_str("delegation"),
        }
    }
}
impl ::std::str::FromStr for LiveDelegationItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "delegation" => Ok(Self::Delegation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveDelegationItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveDelegationItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reasoning options for Responses requests made on behalf of the Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveDelegationReasoningInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<LiveReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<LiveReasoningSummary>,
}
impl LiveDelegationReasoningInputParam {
    pub fn builder() -> builder::LiveDelegationReasoningInputParam {
        Default::default()
    }
}
#[doc = "Text generation options for the Live session’s Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveDelegationTextInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verbosity: ::std::option::Option<LiveTextVerbosity>,
}
impl LiveDelegationTextInputParam {
    pub fn builder() -> builder::LiveDelegationTextInputParam {
        Default::default()
    }
}
#[doc = "Reports an error in the Live session, such as an invalid client command. Use error.client_event_id, when present, to identify the command that caused the error."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveErrorEvent {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "Details of the Live error and the client command that caused it, when known."]
    pub error: LiveLiveError,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, always `error`."]
    #[serde(rename = "type")]
    pub type_: LiveErrorEventType,
}
impl LiveErrorEvent {
    pub fn builder() -> builder::LiveErrorEvent {
        Default::default()
    }
}
#[doc = "The event type, always `error`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveErrorEventType {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for LiveErrorEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for LiveErrorEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveErrorEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveErrorEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveFileSearchToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveFileSearchToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveFileSearchToolInputParamType,
}
impl LiveFileSearchToolInputParam {
    pub fn builder() -> builder::LiveFileSearchToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveFileSearchToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveFileSearchToolInputParamType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for LiveFileSearchToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for LiveFileSearchToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveFileSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveFileSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Audio format for the new WebSocket connection to a forked Live session. The stored voice is preserved."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveForkAudioParam {
    #[doc = "Audio encoding and sample rate for audio sent and received over a Live WebSocket connection. WebRTC and SIP negotiate their media format separately."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<LiveAudioFormat>,
}
impl LiveForkAudioParam {
    pub fn builder() -> builder::LiveForkAudioParam {
        Default::default()
    }
}
#[doc = "Client events for a Live fork WebSocket. First send session.start with an overrides object (which may be empty), then wait for session.started before sending other commands. The model and conversation are inherited from the stored session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveForkClientEvent {
    ForkSessionStartEvent(LiveForkSessionStartEvent),
    SessionUpdateParam(LiveSessionUpdateParam),
    InputAudioAppendEvent(LiveInputAudioAppendEvent),
    InputAudioMuteParam(LiveInputAudioMuteParam),
    InputAudioUnmuteParam(LiveInputAudioUnmuteParam),
    InstructionsAppendParam(LiveInstructionsAppendParam),
    ThinkingAppendParam(LiveThinkingAppendParam),
    CommentaryAppendParam(LiveCommentaryAppendParam),
    ResponseItemCreateParam(LiveResponseItemCreateParam),
    ResponseCreateParam(LiveResponseCreateParam),
    SessionCloseParam(LiveSessionCloseParam),
}
impl ::std::convert::From<LiveForkSessionStartEvent> for LiveForkClientEvent {
    fn from(value: LiveForkSessionStartEvent) -> Self {
        Self::ForkSessionStartEvent(value)
    }
}
impl ::std::convert::From<LiveSessionUpdateParam> for LiveForkClientEvent {
    fn from(value: LiveSessionUpdateParam) -> Self {
        Self::SessionUpdateParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioAppendEvent> for LiveForkClientEvent {
    fn from(value: LiveInputAudioAppendEvent) -> Self {
        Self::InputAudioAppendEvent(value)
    }
}
impl ::std::convert::From<LiveInputAudioMuteParam> for LiveForkClientEvent {
    fn from(value: LiveInputAudioMuteParam) -> Self {
        Self::InputAudioMuteParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioUnmuteParam> for LiveForkClientEvent {
    fn from(value: LiveInputAudioUnmuteParam) -> Self {
        Self::InputAudioUnmuteParam(value)
    }
}
impl ::std::convert::From<LiveInstructionsAppendParam> for LiveForkClientEvent {
    fn from(value: LiveInstructionsAppendParam) -> Self {
        Self::InstructionsAppendParam(value)
    }
}
impl ::std::convert::From<LiveThinkingAppendParam> for LiveForkClientEvent {
    fn from(value: LiveThinkingAppendParam) -> Self {
        Self::ThinkingAppendParam(value)
    }
}
impl ::std::convert::From<LiveCommentaryAppendParam> for LiveForkClientEvent {
    fn from(value: LiveCommentaryAppendParam) -> Self {
        Self::CommentaryAppendParam(value)
    }
}
impl ::std::convert::From<LiveResponseItemCreateParam> for LiveForkClientEvent {
    fn from(value: LiveResponseItemCreateParam) -> Self {
        Self::ResponseItemCreateParam(value)
    }
}
impl ::std::convert::From<LiveResponseCreateParam> for LiveForkClientEvent {
    fn from(value: LiveResponseCreateParam) -> Self {
        Self::ResponseCreateParam(value)
    }
}
impl ::std::convert::From<LiveSessionCloseParam> for LiveForkClientEvent {
    fn from(value: LiveSessionCloseParam) -> Self {
        Self::SessionCloseParam(value)
    }
}
#[doc = "Fork a stored Live session onto a new WebRTC connection. Omit session or send an empty object to inherit its configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveForkRequest {
    #[doc = "Optional configuration overrides for the new Live session. Omit this object or send an empty object to inherit the stored session's settings."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub session: ::std::option::Option<LiveMediaSessionForkParams>,
    #[doc = "WebRTC transport with an SDP offer for the new connection to the forked session."]
    pub transport: LiveWebRtcTransport,
}
impl LiveForkRequest {
    pub fn builder() -> builder::LiveForkRequest {
        Default::default()
    }
}
#[doc = "`LiveForkServerEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct LiveForkServerEvent(pub LiveServerEvent);
impl ::std::ops::Deref for LiveForkServerEvent {
    type Target = LiveServerEvent;
    fn deref(&self) -> &LiveServerEvent {
        &self.0
    }
}
impl ::std::convert::From<LiveForkServerEvent> for LiveServerEvent {
    fn from(value: LiveForkServerEvent) -> Self {
        value.0
    }
}
impl ::std::convert::From<LiveServerEvent> for LiveForkServerEvent {
    fn from(value: LiveServerEvent) -> Self {
        Self(value)
    }
}
#[doc = "Overrides for a stored session after connecting to the fork WebSocket. An empty object inherits the stored configuration; do not supply a new model. audio.format applies only to the new WebSocket connection. client overrides are only supported for WebRTC forks."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveForkSessionConfigParam {
    #[doc = "Audio format for a WebSocket fork. WebRTC forks negotiate their audio format and must omit this field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<LiveForkAudioParam>,
    #[doc = "Frontend data-channel permissions for a WebRTC fork. Omitted permissions inherit the stored values. Not supported for WebSocket forks."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<LiveClientConfigParam>,
    #[doc = "Overrides for the stored session’s Responses backend. Only supported when the stored session already uses Responses delegation; the delegation type cannot change."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<LiveResponsesDelegationUpdateParam>,
    #[doc = "Whether to store the forked session. Omission inherits the stored session's setting."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<bool>,
}
impl LiveForkSessionConfigParam {
    pub fn builder() -> builder::LiveForkSessionConfigParam {
        Default::default()
    }
}
#[doc = "Start a Live session after connecting to a stored session’s fork WebSocket. Send an empty `session` object to use the stored configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveForkSessionStartEvent {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveForkSessionStartEventEventId>,
    #[doc = "Overrides for a stored session after connecting to the fork WebSocket. An empty object inherits the stored configuration; do not supply a new model. audio.format applies only to the new WebSocket connection. client overrides are only supported for WebRTC forks."]
    pub session: LiveForkSessionConfigParam,
    #[doc = "The Live client event type. Always `session.start`."]
    #[serde(rename = "type")]
    pub type_: LiveForkSessionStartEventType,
}
impl LiveForkSessionStartEvent {
    pub fn builder() -> builder::LiveForkSessionStartEvent {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveForkSessionStartEventEventId(::std::string::String);
impl ::std::ops::Deref for LiveForkSessionStartEventEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveForkSessionStartEventEventId> for ::std::string::String {
    fn from(value: LiveForkSessionStartEventEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveForkSessionStartEventEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveForkSessionStartEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveForkSessionStartEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveForkSessionStartEventEventId {
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
#[doc = "The Live client event type. Always `session.start`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveForkSessionStartEventType {
    #[serde(rename = "session.start")]
    SessionStart,
}
impl ::std::fmt::Display for LiveForkSessionStartEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStart => f.write_str("session.start"),
        }
    }
}
impl ::std::str::FromStr for LiveForkSessionStartEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.start" => Ok(Self::SessionStart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveForkSessionStartEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveForkSessionStartEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveFunctionToolChoiceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveFunctionToolChoiceParam {
    #[doc = "The name of the function tool to call."]
    pub name: ::std::string::String,
    #[doc = "The tool to call. Always `function`."]
    #[serde(rename = "type")]
    pub type_: LiveFunctionToolChoiceParamType,
}
impl LiveFunctionToolChoiceParam {
    pub fn builder() -> builder::LiveFunctionToolChoiceParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `function`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveFunctionToolChoiceParamType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for LiveFunctionToolChoiceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for LiveFunctionToolChoiceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveFunctionToolChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveFunctionToolChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A function tool available to the Responses backend when the Live model delegates a task."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveFunctionToolInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name the delegated Responses model uses when calling this function."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
    #[doc = "The tool type. Always `function`."]
    #[serde(rename = "type")]
    pub type_: LiveFunctionToolInputParamType,
}
impl LiveFunctionToolInputParam {
    pub fn builder() -> builder::LiveFunctionToolInputParam {
        Default::default()
    }
}
#[doc = "The tool type. Always `function`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveFunctionToolInputParamType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for LiveFunctionToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for LiveFunctionToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveFunctionToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveFunctionToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveHostedShellContainerAutoParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveHostedShellContainerAutoParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_limit: ::std::option::Option<LiveContainerMemoryLimit>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_policy: ::std::option::Option<LiveHostedShellContainerAutoParamNetworkPolicy>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub skills: ::std::option::Option<::std::vec::Vec<LiveHostedShellContainerAutoParamSkillsItem>>,
    #[doc = "Automatically creates a container for this request"]
    #[serde(rename = "type")]
    pub type_: LiveHostedShellContainerAutoParamType,
}
impl LiveHostedShellContainerAutoParam {
    pub fn builder() -> builder::LiveHostedShellContainerAutoParam {
        Default::default()
    }
}
#[doc = "Network access policy for the container."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveHostedShellContainerAutoParamNetworkPolicy {
    ContainerNetworkPolicyDisabledParam(LiveContainerNetworkPolicyDisabledParam),
    HostedShellNetworkPolicyAllowlistParam(LiveHostedShellNetworkPolicyAllowlistParam),
}
impl ::std::convert::From<LiveContainerNetworkPolicyDisabledParam>
    for LiveHostedShellContainerAutoParamNetworkPolicy
{
    fn from(value: LiveContainerNetworkPolicyDisabledParam) -> Self {
        Self::ContainerNetworkPolicyDisabledParam(value)
    }
}
impl ::std::convert::From<LiveHostedShellNetworkPolicyAllowlistParam>
    for LiveHostedShellContainerAutoParamNetworkPolicy
{
    fn from(value: LiveHostedShellNetworkPolicyAllowlistParam) -> Self {
        Self::HostedShellNetworkPolicyAllowlistParam(value)
    }
}
#[doc = "`LiveHostedShellContainerAutoParamSkillsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveHostedShellContainerAutoParamSkillsItem {
    SkillReferenceParam(LiveSkillReferenceParam),
    InlineSkillParam(LiveInlineSkillParam),
}
impl ::std::convert::From<LiveSkillReferenceParam> for LiveHostedShellContainerAutoParamSkillsItem {
    fn from(value: LiveSkillReferenceParam) -> Self {
        Self::SkillReferenceParam(value)
    }
}
impl ::std::convert::From<LiveInlineSkillParam> for LiveHostedShellContainerAutoParamSkillsItem {
    fn from(value: LiveInlineSkillParam) -> Self {
        Self::InlineSkillParam(value)
    }
}
#[doc = "Automatically creates a container for this request"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveHostedShellContainerAutoParamType {
    #[serde(rename = "container_auto")]
    ContainerAuto,
}
impl ::std::fmt::Display for LiveHostedShellContainerAutoParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerAuto => f.write_str("container_auto"),
        }
    }
}
impl ::std::str::FromStr for LiveHostedShellContainerAutoParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_auto" => Ok(Self::ContainerAuto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveHostedShellContainerAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveHostedShellContainerAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveHostedShellNetworkPolicyAllowlistParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveHostedShellNetworkPolicyAllowlistParam {
    #[doc = "A list of allowed domains when type is `allowlist`."]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "Allow outbound network access only to specified domains. Always `allowlist`."]
    #[serde(rename = "type")]
    pub type_: LiveHostedShellNetworkPolicyAllowlistParamType,
}
impl LiveHostedShellNetworkPolicyAllowlistParam {
    pub fn builder() -> builder::LiveHostedShellNetworkPolicyAllowlistParam {
        Default::default()
    }
}
#[doc = "Allow outbound network access only to specified domains. Always `allowlist`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveHostedShellNetworkPolicyAllowlistParamType {
    #[serde(rename = "allowlist")]
    Allowlist,
}
impl ::std::fmt::Display for LiveHostedShellNetworkPolicyAllowlistParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allowlist => f.write_str("allowlist"),
        }
    }
}
impl ::std::str::FromStr for LiveHostedShellNetworkPolicyAllowlistParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowlist" => Ok(Self::Allowlist),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveHostedShellNetworkPolicyAllowlistParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveHostedShellNetworkPolicyAllowlistParamType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Responses shell tool. Use a hosted container or return local shell results with response.item.create. Domain secrets are not supported."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveHostedShellToolInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<LiveHostedShellToolInputParamEnvironment>,
    #[serde(rename = "type")]
    pub type_: LiveHostedShellToolInputParamType,
}
impl LiveHostedShellToolInputParam {
    pub fn builder() -> builder::LiveHostedShellToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveHostedShellToolInputParamEnvironment`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveHostedShellToolInputParamEnvironment {
    HostedShellContainerAutoParam(LiveHostedShellContainerAutoParam),
    ContainerReferenceParam(LiveContainerReferenceParam),
    LocalEnvironmentParam(LiveLocalEnvironmentParam),
}
impl ::std::convert::From<LiveHostedShellContainerAutoParam>
    for LiveHostedShellToolInputParamEnvironment
{
    fn from(value: LiveHostedShellContainerAutoParam) -> Self {
        Self::HostedShellContainerAutoParam(value)
    }
}
impl ::std::convert::From<LiveContainerReferenceParam>
    for LiveHostedShellToolInputParamEnvironment
{
    fn from(value: LiveContainerReferenceParam) -> Self {
        Self::ContainerReferenceParam(value)
    }
}
impl ::std::convert::From<LiveLocalEnvironmentParam> for LiveHostedShellToolInputParamEnvironment {
    fn from(value: LiveLocalEnvironmentParam) -> Self {
        Self::LocalEnvironmentParam(value)
    }
}
#[doc = "`LiveHostedShellToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveHostedShellToolInputParamType {
    #[serde(rename = "shell")]
    Shell,
}
impl ::std::fmt::Display for LiveHostedShellToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
        }
    }
}
impl ::std::str::FromStr for LiveHostedShellToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveHostedShellToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveHostedShellToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveImageGenerationToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveImageGenerationToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveImageGenerationToolInputParamType,
}
impl LiveImageGenerationToolInputParam {
    pub fn builder() -> builder::LiveImageGenerationToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveImageGenerationToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveImageGenerationToolInputParamType {
    #[serde(rename = "image_generation")]
    ImageGeneration,
}
impl ::std::fmt::Display for LiveImageGenerationToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGeneration => f.write_str("image_generation"),
        }
    }
}
impl ::std::str::FromStr for LiveImageGenerationToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation" => Ok(Self::ImageGeneration),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveImageGenerationToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveImageGenerationToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An informational notice about the Live session, such as the event permissions applied to a frontend data channel."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInfoEvent {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "A machine-readable code for the notice, such as `data_channel_permissions`."]
    pub code: ::std::string::String,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "A human-readable explanation of the Live session notice."]
    pub message: ::std::string::String,
    #[doc = "The event type, always `info`."]
    #[serde(rename = "type")]
    pub type_: LiveInfoEventType,
}
impl LiveInfoEvent {
    pub fn builder() -> builder::LiveInfoEvent {
        Default::default()
    }
}
#[doc = "The event type, always `info`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInfoEventType {
    #[serde(rename = "info")]
    Info,
}
impl ::std::fmt::Display for LiveInfoEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Info => f.write_str("info"),
        }
    }
}
impl ::std::str::FromStr for LiveInfoEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "info" => Ok(Self::Info),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInfoEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInfoEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An assistant message included in the initial text history of a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialAssistantMessageItemParam {
    #[doc = "The message content. Supply exactly one text part for the initial Live conversation history."]
    pub content: [LiveInitialAssistantMessageItemParamContentItem; 1usize],
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The author of this history message. Always `assistant`."]
    pub role: LiveInitialAssistantMessageItemParamRole,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<LiveInitialMessageStatus>,
    #[doc = "The history item type. Always `message`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiveInitialAssistantMessageItemParamType>,
}
impl LiveInitialAssistantMessageItemParam {
    pub fn builder() -> builder::LiveInitialAssistantMessageItemParam {
        Default::default()
    }
}
#[doc = "`LiveInitialAssistantMessageItemParamContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveInitialAssistantMessageItemParamContentItem {
    TextContentPartParam(LiveInitialTextContentPartParam),
    OutputTextContentPartParam(LiveInitialOutputTextContentPartParam),
}
impl ::std::convert::From<LiveInitialTextContentPartParam>
    for LiveInitialAssistantMessageItemParamContentItem
{
    fn from(value: LiveInitialTextContentPartParam) -> Self {
        Self::TextContentPartParam(value)
    }
}
impl ::std::convert::From<LiveInitialOutputTextContentPartParam>
    for LiveInitialAssistantMessageItemParamContentItem
{
    fn from(value: LiveInitialOutputTextContentPartParam) -> Self {
        Self::OutputTextContentPartParam(value)
    }
}
#[doc = "The author of this history message. Always `assistant`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialAssistantMessageItemParamRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for LiveInitialAssistantMessageItemParamRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialAssistantMessageItemParamRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialAssistantMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialAssistantMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The history item type. Always `message`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialAssistantMessageItemParamType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for LiveInitialAssistantMessageItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialAssistantMessageItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialAssistantMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialAssistantMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A developer message included in the initial text history of a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialDeveloperMessageItemParam {
    #[doc = "The message content. Supply exactly one text part for the initial Live conversation history."]
    pub content: [LiveInitialInputTextContentPartParam; 1usize],
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The author of this history message. Always `developer`."]
    pub role: LiveInitialDeveloperMessageItemParamRole,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<LiveInitialMessageStatus>,
    #[doc = "The history item type. Always `message`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiveInitialDeveloperMessageItemParamType>,
}
impl LiveInitialDeveloperMessageItemParam {
    pub fn builder() -> builder::LiveInitialDeveloperMessageItemParam {
        Default::default()
    }
}
#[doc = "The author of this history message. Always `developer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialDeveloperMessageItemParamRole {
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for LiveInitialDeveloperMessageItemParamRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialDeveloperMessageItemParamRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialDeveloperMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialDeveloperMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The history item type. Always `message`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialDeveloperMessageItemParamType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for LiveInitialDeveloperMessageItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialDeveloperMessageItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialDeveloperMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialDeveloperMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Text supplied in a developer or user message when starting a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialInputTextContentPartParam {
    #[doc = "The message text to include in the Live session’s initial conversation history."]
    pub text: ::std::string::String,
    #[doc = "The text content type. Always `input_text`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiveInitialInputTextContentPartParamType>,
}
impl LiveInitialInputTextContentPartParam {
    pub fn builder() -> builder::LiveInitialInputTextContentPartParam {
        Default::default()
    }
}
#[doc = "The text content type. Always `input_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialInputTextContentPartParamType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for LiveInitialInputTextContentPartParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialInputTextContentPartParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialInputTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialInputTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A developer, user, or assistant message supplied as text history before the Live session starts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveInitialItem {
    DeveloperMessageItemParam(LiveInitialDeveloperMessageItemParam),
    UserMessageItemParam(LiveInitialUserMessageItemParam),
    AssistantMessageItemParam(LiveInitialAssistantMessageItemParam),
}
impl ::std::convert::From<LiveInitialDeveloperMessageItemParam> for LiveInitialItem {
    fn from(value: LiveInitialDeveloperMessageItemParam) -> Self {
        Self::DeveloperMessageItemParam(value)
    }
}
impl ::std::convert::From<LiveInitialUserMessageItemParam> for LiveInitialItem {
    fn from(value: LiveInitialUserMessageItemParam) -> Self {
        Self::UserMessageItemParam(value)
    }
}
impl ::std::convert::From<LiveInitialAssistantMessageItemParam> for LiveInitialItem {
    fn from(value: LiveInitialAssistantMessageItemParam) -> Self {
        Self::AssistantMessageItemParam(value)
    }
}
#[doc = "`LiveInitialMessageStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialMessageStatus {
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for LiveInitialMessageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Incomplete => f.write_str("incomplete"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialMessageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "incomplete" => Ok(Self::Incomplete),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Assistant output text supplied as conversation history when starting a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialOutputTextContentPartParam {
    #[doc = "The message text to include in the Live session’s initial conversation history."]
    pub text: ::std::string::String,
    #[doc = "The text content type. Always `output_text`."]
    #[serde(rename = "type")]
    pub type_: LiveInitialOutputTextContentPartParamType,
}
impl LiveInitialOutputTextContentPartParam {
    pub fn builder() -> builder::LiveInitialOutputTextContentPartParam {
        Default::default()
    }
}
#[doc = "The text content type. Always `output_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialOutputTextContentPartParamType {
    #[serde(rename = "output_text")]
    OutputText,
}
impl ::std::fmt::Display for LiveInitialOutputTextContentPartParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputText => f.write_str("output_text"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialOutputTextContentPartParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_text" => Ok(Self::OutputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialOutputTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialOutputTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Settings for speech generated by the Live model. Choose the voice before starting the session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveInitialSessionAudioOutputParam {
    #[doc = "The voice used for Live speech, as a built-in voice name or a custom voice object containing its ID. Defaults to `marin` and cannot change after startup."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<LiveInitialSessionAudioOutputParamVoice>,
}
impl LiveInitialSessionAudioOutputParam {
    pub fn builder() -> builder::LiveInitialSessionAudioOutputParam {
        Default::default()
    }
}
#[doc = "The voice used for Live speech, as a built-in voice name or a custom voice object containing its ID. Defaults to `marin` and cannot change after startup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveInitialSessionAudioOutputParamVoice {
    String(::std::string::String),
    LiveCustomVoiceParam(LiveCustomVoiceParam),
}
impl ::std::convert::From<LiveCustomVoiceParam> for LiveInitialSessionAudioOutputParamVoice {
    fn from(value: LiveCustomVoiceParam) -> Self {
        Self::LiveCustomVoiceParam(value)
    }
}
#[doc = "Startup audio configuration. Only primary WebSockets accept audio.format; WebRTC and SIP negotiate their media format. Voice and format are immutable after startup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveInitialSessionAudioParam {
    #[doc = "Audio encoding and sample rate for audio sent and received over a Live WebSocket connection. WebRTC and SIP negotiate their media format separately."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<LiveAudioFormat>,
    #[doc = "The voice used for speech generated by the Live model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<LiveInitialSessionAudioOutputParam>,
}
impl LiveInitialSessionAudioParam {
    pub fn builder() -> builder::LiveInitialSessionAudioParam {
        Default::default()
    }
}
#[doc = "Assistant text supplied as conversation history when starting a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialTextContentPartParam {
    #[doc = "The message text to include in the Live session’s initial conversation history."]
    pub text: ::std::string::String,
    #[doc = "The text content type. Always `text`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiveInitialTextContentPartParamType>,
}
impl LiveInitialTextContentPartParam {
    pub fn builder() -> builder::LiveInitialTextContentPartParam {
        Default::default()
    }
}
#[doc = "The text content type. Always `text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialTextContentPartParamType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for LiveInitialTextContentPartParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialTextContentPartParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialTextContentPartParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A user message included in the initial text history of a Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInitialUserMessageItemParam {
    #[doc = "The message content. Supply exactly one text part for the initial Live conversation history."]
    pub content: [LiveInitialInputTextContentPartParam; 1usize],
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The author of this history message. Always `user`."]
    pub role: LiveInitialUserMessageItemParamRole,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<LiveInitialMessageStatus>,
    #[doc = "The history item type. Always `message`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiveInitialUserMessageItemParamType>,
}
impl LiveInitialUserMessageItemParam {
    pub fn builder() -> builder::LiveInitialUserMessageItemParam {
        Default::default()
    }
}
#[doc = "The author of this history message. Always `user`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialUserMessageItemParamRole {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for LiveInitialUserMessageItemParamRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialUserMessageItemParamRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialUserMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialUserMessageItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The history item type. Always `message`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInitialUserMessageItemParamType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for LiveInitialUserMessageItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for LiveInitialUserMessageItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInitialUserMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInitialUserMessageItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveInlineSkillParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInlineSkillParam {
    #[doc = "The description of the skill."]
    pub description: ::std::string::String,
    #[doc = "The name of the skill."]
    pub name: ::std::string::String,
    #[doc = "Inline skill payload"]
    pub source: LiveInlineSkillSourceParam,
    #[doc = "Defines an inline skill for this request."]
    #[serde(rename = "type")]
    pub type_: LiveInlineSkillParamType,
}
impl LiveInlineSkillParam {
    pub fn builder() -> builder::LiveInlineSkillParam {
        Default::default()
    }
}
#[doc = "Defines an inline skill for this request."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInlineSkillParamType {
    #[serde(rename = "inline")]
    Inline,
}
impl ::std::fmt::Display for LiveInlineSkillParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inline => f.write_str("inline"),
        }
    }
}
impl ::std::str::FromStr for LiveInlineSkillParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInlineSkillParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInlineSkillParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Inline skill payload"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInlineSkillSourceParam {
    #[doc = "Base64-encoded skill zip bundle."]
    pub data: LiveInlineSkillSourceParamData,
    #[doc = "The media type of the inline skill payload. Must be `application/zip`."]
    pub media_type: LiveInlineSkillSourceParamMediaType,
    #[doc = "The type of the inline skill source. Must be `base64`."]
    #[serde(rename = "type")]
    pub type_: LiveInlineSkillSourceParamType,
}
impl LiveInlineSkillSourceParam {
    pub fn builder() -> builder::LiveInlineSkillSourceParam {
        Default::default()
    }
}
#[doc = "Base64-encoded skill zip bundle."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInlineSkillSourceParamData(::std::string::String);
impl ::std::ops::Deref for LiveInlineSkillSourceParamData {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInlineSkillSourceParamData> for ::std::string::String {
    fn from(value: LiveInlineSkillSourceParamData) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInlineSkillSourceParamData {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 70254592usize {
            return Err("longer than 70254592 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInlineSkillSourceParamData {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInlineSkillSourceParamData {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInlineSkillSourceParamData {
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
#[doc = "The media type of the inline skill payload. Must be `application/zip`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInlineSkillSourceParamMediaType {
    #[serde(rename = "application/zip")]
    ApplicationZip,
}
impl ::std::fmt::Display for LiveInlineSkillSourceParamMediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplicationZip => f.write_str("application/zip"),
        }
    }
}
impl ::std::str::FromStr for LiveInlineSkillSourceParamMediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "application/zip" => Ok(Self::ApplicationZip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInlineSkillSourceParamMediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInlineSkillSourceParamMediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the inline skill source. Must be `base64`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInlineSkillSourceParamType {
    #[serde(rename = "base64")]
    Base64,
}
impl ::std::fmt::Display for LiveInlineSkillSourceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Base64 => f.write_str("base64"),
        }
    }
}
impl ::std::str::FromStr for LiveInlineSkillSourceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "base64" => Ok(Self::Base64),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInlineSkillSourceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInlineSkillSourceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Input audio received from the primary transport and reflected to a Live sideband connection before model-input muting."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioAppend {
    #[doc = "Base64-encoded raw mono PCM16LE at 24 kHz received from the primary transport, reflected to the sideband before model-input muting. This server event uses the same audio key as the client command, but is not an acknowledgment of it."]
    pub audio: ::std::string::String,
    #[doc = "The event type, always `session.input_audio.append`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioAppendType,
}
impl LiveInputAudioAppend {
    pub fn builder() -> builder::LiveInputAudioAppend {
        Default::default()
    }
}
#[doc = "Send audio to a Live session over its primary WebSocket. WebRTC and SIP sessions send audio over their media transport."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioAppendEvent {
    #[doc = "Base64-encoded raw audio in the startup-selected format, without a WAV or other container header. Primary WebSocket only; media transports use their audio track. Audio appends have no acknowledgment. Reflected sideband server events reuse this event type and audio key, with no timestamps or event_id; their audio is always mono PCM16LE at 24 kHz."]
    pub audio: LiveInputAudioAppendEventAudio,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveInputAudioAppendEventEventId>,
    #[doc = "The Live client event type. Always `session.input_audio.append`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioAppendEventType,
}
impl LiveInputAudioAppendEvent {
    pub fn builder() -> builder::LiveInputAudioAppendEvent {
        Default::default()
    }
}
#[doc = "Base64-encoded raw audio in the startup-selected format, without a WAV or other container header. Primary WebSocket only; media transports use their audio track. Audio appends have no acknowledgment. Reflected sideband server events reuse this event type and audio key, with no timestamps or event_id; their audio is always mono PCM16LE at 24 kHz."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInputAudioAppendEventAudio(::std::string::String);
impl ::std::ops::Deref for LiveInputAudioAppendEventAudio {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInputAudioAppendEventAudio> for ::std::string::String {
    fn from(value: LiveInputAudioAppendEventAudio) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInputAudioAppendEventAudio {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioAppendEventAudio {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioAppendEventAudio {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInputAudioAppendEventAudio {
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
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInputAudioAppendEventEventId(::std::string::String);
impl ::std::ops::Deref for LiveInputAudioAppendEventEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInputAudioAppendEventEventId> for ::std::string::String {
    fn from(value: LiveInputAudioAppendEventEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInputAudioAppendEventEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioAppendEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioAppendEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInputAudioAppendEventEventId {
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
#[doc = "The Live client event type. Always `session.input_audio.append`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioAppendEventType {
    #[serde(rename = "session.input_audio.append")]
    SessionInputAudioAppend,
}
impl ::std::fmt::Display for LiveInputAudioAppendEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioAppend => f.write_str("session.input_audio.append"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioAppendEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.append" => Ok(Self::SessionInputAudioAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioAppendEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioAppendEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, always `session.input_audio.append`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioAppendType {
    #[serde(rename = "session.input_audio.append")]
    SessionInputAudioAppend,
}
impl ::std::fmt::Display for LiveInputAudioAppendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioAppend => f.write_str("session.input_audio.append"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioAppendType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.append" => Ok(Self::SessionInputAudioAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioAppendType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioAppendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mute audio input to the Live model without closing the session. The server acknowledges with `session.input_audio.muted`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioMuteParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveInputAudioMuteParamEventId>,
    #[doc = "The Live client event type. Always `session.input_audio.mute`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioMuteParamType,
}
impl LiveInputAudioMuteParam {
    pub fn builder() -> builder::LiveInputAudioMuteParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInputAudioMuteParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveInputAudioMuteParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInputAudioMuteParamEventId> for ::std::string::String {
    fn from(value: LiveInputAudioMuteParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInputAudioMuteParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioMuteParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioMuteParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInputAudioMuteParamEventId {
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
#[doc = "The Live client event type. Always `session.input_audio.mute`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioMuteParamType {
    #[serde(rename = "session.input_audio.mute")]
    SessionInputAudioMute,
}
impl ::std::fmt::Display for LiveInputAudioMuteParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioMute => f.write_str("session.input_audio.mute"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioMuteParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.mute" => Ok(Self::SessionInputAudioMute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioMuteParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioMuteParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session.input_audio.mute command is accepted. Input audio is no longer sent to the model; sideband audio reflection continues."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioMuted {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, always `session.input_audio.muted`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioMutedType,
}
impl LiveInputAudioMuted {
    pub fn builder() -> builder::LiveInputAudioMuted {
        Default::default()
    }
}
#[doc = "The event type, always `session.input_audio.muted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioMutedType {
    #[serde(rename = "session.input_audio.muted")]
    SessionInputAudioMuted,
}
impl ::std::fmt::Display for LiveInputAudioMutedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioMuted => f.write_str("session.input_audio.muted"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioMutedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.muted" => Ok(Self::SessionInputAudioMuted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioMutedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioMutedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Resume audio input to a Live model after muting it. The server acknowledges with `session.input_audio.unmuted`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioUnmuteParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveInputAudioUnmuteParamEventId>,
    #[doc = "The Live client event type. Always `session.input_audio.unmute`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioUnmuteParamType,
}
impl LiveInputAudioUnmuteParam {
    pub fn builder() -> builder::LiveInputAudioUnmuteParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInputAudioUnmuteParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveInputAudioUnmuteParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInputAudioUnmuteParamEventId> for ::std::string::String {
    fn from(value: LiveInputAudioUnmuteParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInputAudioUnmuteParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioUnmuteParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioUnmuteParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInputAudioUnmuteParamEventId {
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
#[doc = "The Live client event type. Always `session.input_audio.unmute`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioUnmuteParamType {
    #[serde(rename = "session.input_audio.unmute")]
    SessionInputAudioUnmute,
}
impl ::std::fmt::Display for LiveInputAudioUnmuteParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioUnmute => f.write_str("session.input_audio.unmute"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioUnmuteParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.unmute" => Ok(Self::SessionInputAudioUnmute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioUnmuteParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioUnmuteParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session.input_audio.unmute command is accepted. Input audio is sent to the model again."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputAudioUnmuted {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, always `session.input_audio.unmuted`."]
    #[serde(rename = "type")]
    pub type_: LiveInputAudioUnmutedType,
}
impl LiveInputAudioUnmuted {
    pub fn builder() -> builder::LiveInputAudioUnmuted {
        Default::default()
    }
}
#[doc = "The event type, always `session.input_audio.unmuted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputAudioUnmutedType {
    #[serde(rename = "session.input_audio.unmuted")]
    SessionInputAudioUnmuted,
}
impl ::std::fmt::Display for LiveInputAudioUnmutedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputAudioUnmuted => f.write_str("session.input_audio.unmuted"),
        }
    }
}
impl ::std::str::FromStr for LiveInputAudioUnmutedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_audio.unmuted" => Ok(Self::SessionInputAudioUnmuted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputAudioUnmutedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputAudioUnmutedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A transcript fragment for user input audio in the Live session. Accumulate fragments in delivery order; these events do not define complete turns or include a transcript-done event."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInputTranscriptDelta {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The transcript text fragment for the audio in this time range. Append fragments in delivery order to build the transcript."]
    pub delta: ::std::string::String,
    #[doc = "The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms."]
    pub end_ms: i64,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The start of this event on the Live session timeline, in milliseconds from the beginning of the session."]
    pub start_ms: i64,
    #[doc = "The event type, always `session.input_transcript.delta`."]
    #[serde(rename = "type")]
    pub type_: LiveInputTranscriptDeltaType,
}
impl LiveInputTranscriptDelta {
    pub fn builder() -> builder::LiveInputTranscriptDelta {
        Default::default()
    }
}
#[doc = "The event type, always `session.input_transcript.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInputTranscriptDeltaType {
    #[serde(rename = "session.input_transcript.delta")]
    SessionInputTranscriptDelta,
}
impl ::std::fmt::Display for LiveInputTranscriptDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInputTranscriptDelta => f.write_str("session.input_transcript.delta"),
        }
    }
}
impl ::std::str::FromStr for LiveInputTranscriptDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.input_transcript.delta" => Ok(Self::SessionInputTranscriptDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInputTranscriptDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInputTranscriptDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Append instructions to the Live conversation while it is running, optionally associating them with an existing client delegation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInstructionsAppendParam {
    #[doc = "Instruction text to append, limited to 500 tokens. This is a plain string, not an array of content parts."]
    pub content: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub delegation_id: ::std::option::Option<LiveInstructionsAppendParamDelegationId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveInstructionsAppendParamEventId>,
    #[doc = "The Live client event type. Always `session.instructions.append`."]
    #[serde(rename = "type")]
    pub type_: LiveInstructionsAppendParamType,
}
impl LiveInstructionsAppendParam {
    pub fn builder() -> builder::LiveInstructionsAppendParam {
        Default::default()
    }
}
#[doc = "Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInstructionsAppendParamDelegationId(::std::string::String);
impl ::std::ops::Deref for LiveInstructionsAppendParamDelegationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInstructionsAppendParamDelegationId> for ::std::string::String {
    fn from(value: LiveInstructionsAppendParamDelegationId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInstructionsAppendParamDelegationId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInstructionsAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInstructionsAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInstructionsAppendParamDelegationId {
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
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveInstructionsAppendParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveInstructionsAppendParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveInstructionsAppendParamEventId> for ::std::string::String {
    fn from(value: LiveInstructionsAppendParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveInstructionsAppendParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveInstructionsAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInstructionsAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveInstructionsAppendParamEventId {
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
#[doc = "The Live client event type. Always `session.instructions.append`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInstructionsAppendParamType {
    #[serde(rename = "session.instructions.append")]
    SessionInstructionsAppend,
}
impl ::std::fmt::Display for LiveInstructionsAppendParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInstructionsAppend => f.write_str("session.instructions.append"),
        }
    }
}
impl ::std::str::FromStr for LiveInstructionsAppendParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.instructions.append" => Ok(Self::SessionInstructionsAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInstructionsAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInstructionsAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session.instructions.append command is accepted into the Live session timeline. Acknowledges the appended instructions without guaranteeing that the model has acted on them."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveInstructionsAppended {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms."]
    pub end_ms: i64,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The start of this event on the Live session timeline, in milliseconds from the beginning of the session."]
    pub start_ms: i64,
    #[doc = "The event type, always `session.instructions.appended`."]
    #[serde(rename = "type")]
    pub type_: LiveInstructionsAppendedType,
}
impl LiveInstructionsAppended {
    pub fn builder() -> builder::LiveInstructionsAppended {
        Default::default()
    }
}
#[doc = "The event type, always `session.instructions.appended`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveInstructionsAppendedType {
    #[serde(rename = "session.instructions.appended")]
    SessionInstructionsAppended,
}
impl ::std::fmt::Display for LiveInstructionsAppendedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInstructionsAppended => f.write_str("session.instructions.appended"),
        }
    }
}
impl ::std::str::FromStr for LiveInstructionsAppendedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.instructions.appended" => Ok(Self::SessionInstructionsAppended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveInstructionsAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveInstructionsAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of an error encountered by the Live session, including the affected parameter or client command when available."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveLiveError {
    #[doc = "The event_id of the client command that caused the error, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "A machine-readable code identifying the Live error, such as `unknown_parameter`."]
    pub code: ::std::string::String,
    #[doc = "A human-readable explanation of the Live error."]
    pub message: ::std::string::String,
    #[doc = "The parameter that caused the error, when applicable, such as `session.voice`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub param: ::std::option::Option<::std::string::String>,
    #[doc = "The category of error, such as `invalid_request_error` for an invalid Live client command."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl LiveLiveError {
    pub fn builder() -> builder::LiveLiveError {
        Default::default()
    }
}
#[doc = "`LiveLocalEnvironmentParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveLocalEnvironmentParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub skills: ::std::option::Option<::std::vec::Vec<LiveLocalSkillParam>>,
    #[doc = "Use a local computer environment."]
    #[serde(rename = "type")]
    pub type_: LiveLocalEnvironmentParamType,
}
impl LiveLocalEnvironmentParam {
    pub fn builder() -> builder::LiveLocalEnvironmentParam {
        Default::default()
    }
}
#[doc = "Use a local computer environment."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveLocalEnvironmentParamType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for LiveLocalEnvironmentParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for LiveLocalEnvironmentParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local" => Ok(Self::Local),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveLocalEnvironmentParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveLocalEnvironmentParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveLocalSkillParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveLocalSkillParam {
    #[doc = "The description of the skill."]
    pub description: ::std::string::String,
    #[doc = "The name of the skill."]
    pub name: ::std::string::String,
    #[doc = "The path to the directory containing the skill."]
    pub path: ::std::string::String,
}
impl LiveLocalSkillParam {
    pub fn builder() -> builder::LiveLocalSkillParam {
        Default::default()
    }
}
#[doc = "`LiveMcpToolChoiceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveMcpToolChoiceParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server to call."]
    pub server_label: ::std::string::String,
    #[doc = "The tool to call. Always `mcp`."]
    #[serde(rename = "type")]
    pub type_: LiveMcpToolChoiceParamType,
}
impl LiveMcpToolChoiceParam {
    pub fn builder() -> builder::LiveMcpToolChoiceParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `mcp`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveMcpToolChoiceParamType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for LiveMcpToolChoiceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for LiveMcpToolChoiceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveMcpToolChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveMcpToolChoiceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveMcpToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveMcpToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveMcpToolInputParamType,
}
impl LiveMcpToolInputParam {
    pub fn builder() -> builder::LiveMcpToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveMcpToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveMcpToolInputParamType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for LiveMcpToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for LiveMcpToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveMcpToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveMcpToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Startup audio output configuration. WebRTC and SIP negotiate the media format; audio.format is only accepted for primary WebSockets. Voice cannot change after startup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveMediaSessionAudioParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<LiveInitialSessionAudioOutputParam>,
}
impl LiveMediaSessionAudioParam {
    pub fn builder() -> builder::LiveMediaSessionAudioParam {
        Default::default()
    }
}
#[doc = "Startup configuration for a Live media session. Follow the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) when writing frontend instructions and the backend prompt under delegation.responses.instructions."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveMediaSessionCreateParams {
    #[doc = "Startup audio configuration. WebRTC and SIP negotiate their audio format on the media transport."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<LiveMediaSessionAudioParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<LiveClientConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<Delegation>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<Input>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<Instructions>,
    pub model: ModelIdsLive,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<Store>,
}
impl LiveMediaSessionCreateParams {
    pub fn builder() -> builder::LiveMediaSessionCreateParams {
        Default::default()
    }
}
#[doc = "Optional overrides for a stored Live session. Omitted settings are inherited. The model, voice, frontend instructions, and prior conversation come from the stored session. WebRTC negotiates its audio format; audio.format is only supported on WebSocket forks."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveMediaSessionForkParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<LiveClientConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<LiveResponsesDelegationUpdateParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<PropertiesStore>,
}
impl LiveMediaSessionForkParams {
    pub fn builder() -> builder::LiveMediaSessionForkParams {
        Default::default()
    }
}
#[doc = "`LiveNamespaceToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveNamespaceToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveNamespaceToolInputParamType,
}
impl LiveNamespaceToolInputParam {
    pub fn builder() -> builder::LiveNamespaceToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveNamespaceToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveNamespaceToolInputParamType {
    #[serde(rename = "namespace")]
    Namespace,
}
impl ::std::fmt::Display for LiveNamespaceToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Namespace => f.write_str("namespace"),
        }
    }
}
impl ::std::str::FromStr for LiveNamespaceToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "namespace" => Ok(Self::Namespace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveNamespaceToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveNamespaceToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An audio chunk generated by the Live model. Decode and play primary WebSocket chunks in delivery order using the configured session audio format. Sideband connections receive reflected output audio with timestamps."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveOutputAudioDelta {
    #[doc = "Base64-encoded raw audio. Primary WebSocket events use the session's configured format; reflected sideband events use mono PCM16LE at 24 kHz."]
    pub delta: ::std::string::String,
    #[doc = "Exclusive session-relative end in milliseconds. Required on reflected sideband events; omitted on the primary WebSocket. Dropped output frames leave gaps between reflected ranges."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_ms: ::std::option::Option<i64>,
    #[doc = "Inclusive session-relative start in milliseconds. Required on reflected sideband events; omitted on the primary WebSocket."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_ms: ::std::option::Option<i64>,
    #[doc = "The event type, always `session.output_audio.delta`."]
    #[serde(rename = "type")]
    pub type_: LiveOutputAudioDeltaType,
}
impl LiveOutputAudioDelta {
    pub fn builder() -> builder::LiveOutputAudioDelta {
        Default::default()
    }
}
#[doc = "The event type, always `session.output_audio.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveOutputAudioDeltaType {
    #[serde(rename = "session.output_audio.delta")]
    SessionOutputAudioDelta,
}
impl ::std::fmt::Display for LiveOutputAudioDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionOutputAudioDelta => f.write_str("session.output_audio.delta"),
        }
    }
}
impl ::std::str::FromStr for LiveOutputAudioDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.output_audio.delta" => Ok(Self::SessionOutputAudioDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveOutputAudioDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveOutputAudioDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A transcript fragment for assistant output audio in the Live session. Accumulate fragments in delivery order; these events do not define complete turns or include a transcript-done event."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveOutputTranscriptDelta {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The transcript text fragment for the audio in this time range. Append fragments in delivery order to build the transcript."]
    pub delta: ::std::string::String,
    #[doc = "The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms."]
    pub end_ms: i64,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The start of this event on the Live session timeline, in milliseconds from the beginning of the session."]
    pub start_ms: i64,
    #[doc = "The event type, always `session.output_transcript.delta`."]
    #[serde(rename = "type")]
    pub type_: LiveOutputTranscriptDeltaType,
}
impl LiveOutputTranscriptDelta {
    pub fn builder() -> builder::LiveOutputTranscriptDelta {
        Default::default()
    }
}
#[doc = "The event type, always `session.output_transcript.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveOutputTranscriptDeltaType {
    #[serde(rename = "session.output_transcript.delta")]
    SessionOutputTranscriptDelta,
}
impl ::std::fmt::Display for LiveOutputTranscriptDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionOutputTranscriptDelta => f.write_str("session.output_transcript.delta"),
        }
    }
}
impl ::std::str::FromStr for LiveOutputTranscriptDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.output_transcript.delta" => Ok(Self::SessionOutputTranscriptDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveOutputTranscriptDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveOutputTranscriptDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveProgrammaticToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveProgrammaticToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveProgrammaticToolInputParamType,
}
impl LiveProgrammaticToolInputParam {
    pub fn builder() -> builder::LiveProgrammaticToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveProgrammaticToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveProgrammaticToolInputParamType {
    #[serde(rename = "programmatic_tool_calling")]
    ProgrammaticToolCalling,
}
impl ::std::fmt::Display for LiveProgrammaticToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgrammaticToolCalling => f.write_str("programmatic_tool_calling"),
        }
    }
}
impl ::std::str::FromStr for LiveProgrammaticToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "programmatic_tool_calling" => Ok(Self::ProgrammaticToolCalling),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveProgrammaticToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveProgrammaticToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveReasoningEffort`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveReasoningEffort {
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
}
impl ::std::fmt::Display for LiveReasoningEffort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
        }
    }
}
impl ::std::str::FromStr for LiveReasoningEffort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveReasoningSummary`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveReasoningSummary {
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for LiveReasoningSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Concise => f.write_str("concise"),
            Self::Detailed => f.write_str("detailed"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for LiveReasoningSummary {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Request a response from the Live session’s Responses backend, or continue a delegated response waiting for tool results. Requires Responses delegation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponseCreateParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveResponseCreateParamEventId>,
    #[doc = "The Live client event type. Always `response.create`."]
    #[serde(rename = "type")]
    pub type_: LiveResponseCreateParamType,
}
impl LiveResponseCreateParam {
    pub fn builder() -> builder::LiveResponseCreateParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveResponseCreateParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveResponseCreateParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveResponseCreateParamEventId> for ::std::string::String {
    fn from(value: LiveResponseCreateParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveResponseCreateParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponseCreateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponseCreateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveResponseCreateParamEventId {
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
#[doc = "The Live client event type. Always `response.create`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponseCreateParamType {
    #[serde(rename = "response.create")]
    ResponseCreate,
}
impl ::std::fmt::Display for LiveResponseCreateParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCreate => f.write_str("response.create"),
        }
    }
}
impl ::std::str::FromStr for LiveResponseCreateParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.create" => Ok(Self::ResponseCreate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponseCreateParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponseCreateParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A streaming Responses API event from a backend delegated to by the Live session. Use the outer delegation_id to associate the nested stream with its Live delegation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponseEvent {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation_id: ::std::option::Option<::std::string::String>,
    #[doc = "The nested Responses streaming event. Dispatch on its type field. Response lifecycle snapshots omit input and clear instructions, tools, and output to keep messages small; consume granular output events for the generated content."]
    pub event: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, always `response.event`."]
    #[serde(rename = "type")]
    pub type_: LiveResponseEventType,
}
impl LiveResponseEvent {
    pub fn builder() -> builder::LiveResponseEvent {
        Default::default()
    }
}
#[doc = "The event type, always `response.event`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponseEventType {
    #[serde(rename = "response.event")]
    ResponseEvent,
}
impl ::std::fmt::Display for LiveResponseEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseEvent => f.write_str("response.event"),
        }
    }
}
impl ::std::str::FromStr for LiveResponseEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.event" => Ok(Self::ResponseEvent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponseEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponseEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Add an input item to the Live session’s Responses backend. Requires Responses delegation; use `response.create` to request a response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponseItemCreateParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveResponseItemCreateParamEventId>,
    pub item: ::openai_api_types::common::InputItem,
    #[doc = "The Live client event type. Always `response.item.create`."]
    #[serde(rename = "type")]
    pub type_: LiveResponseItemCreateParamType,
}
impl LiveResponseItemCreateParam {
    pub fn builder() -> builder::LiveResponseItemCreateParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveResponseItemCreateParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveResponseItemCreateParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveResponseItemCreateParamEventId> for ::std::string::String {
    fn from(value: LiveResponseItemCreateParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveResponseItemCreateParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponseItemCreateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponseItemCreateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveResponseItemCreateParamEventId {
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
#[doc = "The Live client event type. Always `response.item.create`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponseItemCreateParamType {
    #[serde(rename = "response.item.create")]
    ResponseItemCreate,
}
impl ::std::fmt::Display for LiveResponseItemCreateParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseItemCreate => f.write_str("response.item.create"),
        }
    }
}
impl ::std::str::FromStr for LiveResponseItemCreateParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.item.create" => Ok(Self::ResponseItemCreate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponseItemCreateParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponseItemCreateParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Delegate tasks to a Responses model managed by the Live session."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponsesDelegationParam {
    #[doc = "Backend model, prompt, and tools used when the Live session delegates a task to Responses."]
    pub responses: LiveResponsesDelegationSettingsInputParam,
    #[doc = "The delegation owner. Always `responses` for tasks handled by the Responses API."]
    #[serde(rename = "type")]
    pub type_: LiveResponsesDelegationParamType,
}
impl LiveResponsesDelegationParam {
    pub fn builder() -> builder::LiveResponsesDelegationParam {
        Default::default()
    }
}
#[doc = "The delegation owner. Always `responses` for tasks handled by the Responses API."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponsesDelegationParamType {
    #[serde(rename = "responses")]
    Responses,
}
impl ::std::fmt::Display for LiveResponsesDelegationParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Responses => f.write_str("responses"),
        }
    }
}
impl ::std::str::FromStr for LiveResponsesDelegationParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "responses" => Ok(Self::Responses),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponsesDelegationParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponsesDelegationParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Model, prompt, and tool settings for tasks delegated by the Live session to a Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponsesDelegationSettingsInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<i64>,
    #[doc = "The model used for server-owned Responses delegations."]
    pub model: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<LiveDelegationReasoningInputParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<LiveResponsesServiceTier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<LiveDelegationTextInputParam>,
    #[doc = "Controls which tool the Responses backend uses when handling a task delegated by the Live model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<LiveResponsesDelegationSettingsInputParamToolChoice>,
    #[doc = "Tools available to the Responses backend while it handles tasks delegated by the Live model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<LiveResponsesDelegationSettingsInputParamToolsItem>,
}
impl LiveResponsesDelegationSettingsInputParam {
    pub fn builder() -> builder::LiveResponsesDelegationSettingsInputParam {
        Default::default()
    }
}
#[doc = "Controls which tool the Responses backend uses when handling a task delegated by the Live model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolChoice {
    Variant0(LiveToolChoiceEnum),
    Variant1(LiveResponsesDelegationSettingsInputParamToolChoiceVariant1),
    Variant2(LiveAllowedToolsChoiceParam),
}
impl ::std::convert::From<LiveToolChoiceEnum>
    for LiveResponsesDelegationSettingsInputParamToolChoice
{
    fn from(value: LiveToolChoiceEnum) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationSettingsInputParamToolChoiceVariant1>
    for LiveResponsesDelegationSettingsInputParamToolChoice
{
    fn from(value: LiveResponsesDelegationSettingsInputParamToolChoiceVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<LiveAllowedToolsChoiceParam>
    for LiveResponsesDelegationSettingsInputParamToolChoice
{
    fn from(value: LiveAllowedToolsChoiceParam) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "`LiveResponsesDelegationSettingsInputParamToolChoiceVariant1`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolChoiceVariant1 {
    FunctionToolChoiceParam(LiveFunctionToolChoiceParam),
    McpToolChoiceParam(LiveMcpToolChoiceParam),
    SpecificFileSearchParam(LiveSpecificFileSearchParam),
    SpecificWebSearchParam(LiveSpecificWebSearchParam),
    SpecificWebSearchPreviewParam(LiveSpecificWebSearchPreviewParam),
    SpecificImageGenParam(LiveSpecificImageGenParam),
    SpecificComputerParam(LiveSpecificComputerParam),
    SpecificCodeInterpreterParam(LiveSpecificCodeInterpreterParam),
    SpecificProgrammaticToolCallingParam(LiveSpecificProgrammaticToolCallingParam),
    SpecificFunctionShellParam(LiveSpecificFunctionShellParam),
    SpecificCustomToolParam(LiveSpecificCustomToolParam),
    SpecificApplyPatchParam(LiveSpecificApplyPatchParam),
}
impl ::std::convert::From<LiveFunctionToolChoiceParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveFunctionToolChoiceParam) -> Self {
        Self::FunctionToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveMcpToolChoiceParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveMcpToolChoiceParam) -> Self {
        Self::McpToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFileSearchParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificFileSearchParam) -> Self {
        Self::SpecificFileSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificWebSearchParam) -> Self {
        Self::SpecificWebSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchPreviewParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificWebSearchPreviewParam) -> Self {
        Self::SpecificWebSearchPreviewParam(value)
    }
}
impl ::std::convert::From<LiveSpecificImageGenParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificImageGenParam) -> Self {
        Self::SpecificImageGenParam(value)
    }
}
impl ::std::convert::From<LiveSpecificComputerParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificComputerParam) -> Self {
        Self::SpecificComputerParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCodeInterpreterParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificCodeInterpreterParam) -> Self {
        Self::SpecificCodeInterpreterParam(value)
    }
}
impl ::std::convert::From<LiveSpecificProgrammaticToolCallingParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificProgrammaticToolCallingParam) -> Self {
        Self::SpecificProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFunctionShellParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificFunctionShellParam) -> Self {
        Self::SpecificFunctionShellParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCustomToolParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificCustomToolParam) -> Self {
        Self::SpecificCustomToolParam(value)
    }
}
impl ::std::convert::From<LiveSpecificApplyPatchParam>
    for LiveResponsesDelegationSettingsInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificApplyPatchParam) -> Self {
        Self::SpecificApplyPatchParam(value)
    }
}
#[doc = "`LiveResponsesDelegationSettingsInputParamToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsInputParamToolsItem {
    FunctionToolInputParam(LiveFunctionToolInputParam),
    WebSearchToolInputParam(LiveWebSearchToolInputParam),
    FileSearchToolInputParam(LiveFileSearchToolInputParam),
    CodeInterpreterToolInputParam(LiveCodeInterpreterToolInputParam),
    HostedShellToolInputParam(LiveHostedShellToolInputParam),
    ImageGenerationToolInputParam(LiveImageGenerationToolInputParam),
    McpToolInputParam(LiveMcpToolInputParam),
    CustomToolInputParam(LiveCustomToolInputParam),
    NamespaceToolInputParam(LiveNamespaceToolInputParam),
    ToolSearchToolInputParam(LiveToolSearchToolInputParam),
    ProgrammaticToolInputParam(LiveProgrammaticToolInputParam),
    ComputerToolInputParam(LiveComputerToolInputParam),
    ApplyPatchToolInputParam(LiveApplyPatchToolInputParam),
}
impl ::std::convert::From<LiveFunctionToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveFunctionToolInputParam) -> Self {
        Self::FunctionToolInputParam(value)
    }
}
impl ::std::convert::From<LiveWebSearchToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveWebSearchToolInputParam) -> Self {
        Self::WebSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveFileSearchToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveFileSearchToolInputParam) -> Self {
        Self::FileSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveCodeInterpreterToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveCodeInterpreterToolInputParam) -> Self {
        Self::CodeInterpreterToolInputParam(value)
    }
}
impl ::std::convert::From<LiveHostedShellToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveHostedShellToolInputParam) -> Self {
        Self::HostedShellToolInputParam(value)
    }
}
impl ::std::convert::From<LiveImageGenerationToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveImageGenerationToolInputParam) -> Self {
        Self::ImageGenerationToolInputParam(value)
    }
}
impl ::std::convert::From<LiveMcpToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveMcpToolInputParam) -> Self {
        Self::McpToolInputParam(value)
    }
}
impl ::std::convert::From<LiveCustomToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveCustomToolInputParam) -> Self {
        Self::CustomToolInputParam(value)
    }
}
impl ::std::convert::From<LiveNamespaceToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveNamespaceToolInputParam) -> Self {
        Self::NamespaceToolInputParam(value)
    }
}
impl ::std::convert::From<LiveToolSearchToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveToolSearchToolInputParam) -> Self {
        Self::ToolSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveProgrammaticToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveProgrammaticToolInputParam) -> Self {
        Self::ProgrammaticToolInputParam(value)
    }
}
impl ::std::convert::From<LiveComputerToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveComputerToolInputParam) -> Self {
        Self::ComputerToolInputParam(value)
    }
}
impl ::std::convert::From<LiveApplyPatchToolInputParam>
    for LiveResponsesDelegationSettingsInputParamToolsItem
{
    fn from(value: LiveApplyPatchToolInputParam) -> Self {
        Self::ApplyPatchToolInputParam(value)
    }
}
#[doc = "Updates to the Responses backend of an existing Live session. Omitted settings retain their current values."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveResponsesDelegationSettingsUpdateInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<i64>,
    #[doc = "The Responses backend model to use for subsequent delegated requests. Omit to keep the current backend model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<LiveDelegationReasoningInputParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<LiveResponsesServiceTier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<LiveDelegationTextInputParam>,
    #[doc = "Controls which tool the Responses backend uses when handling a task delegated by the Live model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice:
        ::std::option::Option<LiveResponsesDelegationSettingsUpdateInputParamToolChoice>,
    #[doc = "Tools available to the Responses backend while it handles tasks delegated by the Live model."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<LiveResponsesDelegationSettingsUpdateInputParamToolsItem>,
}
impl LiveResponsesDelegationSettingsUpdateInputParam {
    pub fn builder() -> builder::LiveResponsesDelegationSettingsUpdateInputParam {
        Default::default()
    }
}
#[doc = "Controls which tool the Responses backend uses when handling a task delegated by the Live model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsUpdateInputParamToolChoice {
    Variant0(LiveToolChoiceEnum),
    Variant1(LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1),
    Variant2(LiveAllowedToolsChoiceParam),
}
impl ::std::convert::From<LiveToolChoiceEnum>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoice
{
    fn from(value: LiveToolChoiceEnum) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoice
{
    fn from(value: LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<LiveAllowedToolsChoiceParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoice
{
    fn from(value: LiveAllowedToolsChoiceParam) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "`LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1 {
    FunctionToolChoiceParam(LiveFunctionToolChoiceParam),
    McpToolChoiceParam(LiveMcpToolChoiceParam),
    SpecificFileSearchParam(LiveSpecificFileSearchParam),
    SpecificWebSearchParam(LiveSpecificWebSearchParam),
    SpecificWebSearchPreviewParam(LiveSpecificWebSearchPreviewParam),
    SpecificImageGenParam(LiveSpecificImageGenParam),
    SpecificComputerParam(LiveSpecificComputerParam),
    SpecificCodeInterpreterParam(LiveSpecificCodeInterpreterParam),
    SpecificProgrammaticToolCallingParam(LiveSpecificProgrammaticToolCallingParam),
    SpecificFunctionShellParam(LiveSpecificFunctionShellParam),
    SpecificCustomToolParam(LiveSpecificCustomToolParam),
    SpecificApplyPatchParam(LiveSpecificApplyPatchParam),
}
impl ::std::convert::From<LiveFunctionToolChoiceParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveFunctionToolChoiceParam) -> Self {
        Self::FunctionToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveMcpToolChoiceParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveMcpToolChoiceParam) -> Self {
        Self::McpToolChoiceParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFileSearchParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificFileSearchParam) -> Self {
        Self::SpecificFileSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificWebSearchParam) -> Self {
        Self::SpecificWebSearchParam(value)
    }
}
impl ::std::convert::From<LiveSpecificWebSearchPreviewParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificWebSearchPreviewParam) -> Self {
        Self::SpecificWebSearchPreviewParam(value)
    }
}
impl ::std::convert::From<LiveSpecificImageGenParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificImageGenParam) -> Self {
        Self::SpecificImageGenParam(value)
    }
}
impl ::std::convert::From<LiveSpecificComputerParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificComputerParam) -> Self {
        Self::SpecificComputerParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCodeInterpreterParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificCodeInterpreterParam) -> Self {
        Self::SpecificCodeInterpreterParam(value)
    }
}
impl ::std::convert::From<LiveSpecificProgrammaticToolCallingParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificProgrammaticToolCallingParam) -> Self {
        Self::SpecificProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<LiveSpecificFunctionShellParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificFunctionShellParam) -> Self {
        Self::SpecificFunctionShellParam(value)
    }
}
impl ::std::convert::From<LiveSpecificCustomToolParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificCustomToolParam) -> Self {
        Self::SpecificCustomToolParam(value)
    }
}
impl ::std::convert::From<LiveSpecificApplyPatchParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolChoiceVariant1
{
    fn from(value: LiveSpecificApplyPatchParam) -> Self {
        Self::SpecificApplyPatchParam(value)
    }
}
#[doc = "`LiveResponsesDelegationSettingsUpdateInputParamToolsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveResponsesDelegationSettingsUpdateInputParamToolsItem {
    FunctionToolInputParam(LiveFunctionToolInputParam),
    WebSearchToolInputParam(LiveWebSearchToolInputParam),
    FileSearchToolInputParam(LiveFileSearchToolInputParam),
    CodeInterpreterToolInputParam(LiveCodeInterpreterToolInputParam),
    HostedShellToolInputParam(LiveHostedShellToolInputParam),
    ImageGenerationToolInputParam(LiveImageGenerationToolInputParam),
    McpToolInputParam(LiveMcpToolInputParam),
    CustomToolInputParam(LiveCustomToolInputParam),
    NamespaceToolInputParam(LiveNamespaceToolInputParam),
    ToolSearchToolInputParam(LiveToolSearchToolInputParam),
    ProgrammaticToolInputParam(LiveProgrammaticToolInputParam),
    ComputerToolInputParam(LiveComputerToolInputParam),
    ApplyPatchToolInputParam(LiveApplyPatchToolInputParam),
}
impl ::std::convert::From<LiveFunctionToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveFunctionToolInputParam) -> Self {
        Self::FunctionToolInputParam(value)
    }
}
impl ::std::convert::From<LiveWebSearchToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveWebSearchToolInputParam) -> Self {
        Self::WebSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveFileSearchToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveFileSearchToolInputParam) -> Self {
        Self::FileSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveCodeInterpreterToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveCodeInterpreterToolInputParam) -> Self {
        Self::CodeInterpreterToolInputParam(value)
    }
}
impl ::std::convert::From<LiveHostedShellToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveHostedShellToolInputParam) -> Self {
        Self::HostedShellToolInputParam(value)
    }
}
impl ::std::convert::From<LiveImageGenerationToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveImageGenerationToolInputParam) -> Self {
        Self::ImageGenerationToolInputParam(value)
    }
}
impl ::std::convert::From<LiveMcpToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveMcpToolInputParam) -> Self {
        Self::McpToolInputParam(value)
    }
}
impl ::std::convert::From<LiveCustomToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveCustomToolInputParam) -> Self {
        Self::CustomToolInputParam(value)
    }
}
impl ::std::convert::From<LiveNamespaceToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveNamespaceToolInputParam) -> Self {
        Self::NamespaceToolInputParam(value)
    }
}
impl ::std::convert::From<LiveToolSearchToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveToolSearchToolInputParam) -> Self {
        Self::ToolSearchToolInputParam(value)
    }
}
impl ::std::convert::From<LiveProgrammaticToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveProgrammaticToolInputParam) -> Self {
        Self::ProgrammaticToolInputParam(value)
    }
}
impl ::std::convert::From<LiveComputerToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveComputerToolInputParam) -> Self {
        Self::ComputerToolInputParam(value)
    }
}
impl ::std::convert::From<LiveApplyPatchToolInputParam>
    for LiveResponsesDelegationSettingsUpdateInputParamToolsItem
{
    fn from(value: LiveApplyPatchToolInputParam) -> Self {
        Self::ApplyPatchToolInputParam(value)
    }
}
#[doc = "Update the Responses backend for an existing Live session without changing delegation ownership."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveResponsesDelegationUpdateParam {
    #[doc = "Responses backend settings to update. Omitted settings keep their existing values."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub responses: ::std::option::Option<LiveResponsesDelegationSettingsUpdateInputParam>,
    #[doc = "The delegation owner. Always `responses` for tasks handled by the Responses API."]
    #[serde(rename = "type")]
    pub type_: LiveResponsesDelegationUpdateParamType,
}
impl LiveResponsesDelegationUpdateParam {
    pub fn builder() -> builder::LiveResponsesDelegationUpdateParam {
        Default::default()
    }
}
#[doc = "The delegation owner. Always `responses` for tasks handled by the Responses API."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponsesDelegationUpdateParamType {
    #[serde(rename = "responses")]
    Responses,
}
impl ::std::fmt::Display for LiveResponsesDelegationUpdateParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Responses => f.write_str("responses"),
        }
    }
}
impl ::std::str::FromStr for LiveResponsesDelegationUpdateParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "responses" => Ok(Self::Responses),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponsesDelegationUpdateParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponsesDelegationUpdateParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveResponsesServiceTier`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveResponsesServiceTier {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fast_tier_temp_pilot")]
    FastTierTempPilot,
    #[serde(rename = "flex")]
    Flex,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "ultrafast")]
    Ultrafast,
}
impl ::std::fmt::Display for LiveResponsesServiceTier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::FastTierTempPilot => f.write_str("fast_tier_temp_pilot"),
            Self::Flex => f.write_str("flex"),
            Self::Priority => f.write_str("priority"),
            Self::Ultrafast => f.write_str("ultrafast"),
        }
    }
}
impl ::std::str::FromStr for LiveResponsesServiceTier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "fast_tier_temp_pilot" => Ok(Self::FastTierTempPilot),
            "flex" => Ok(Self::Flex),
            "priority" => Ok(Self::Priority),
            "ultrafast" => Ok(Self::Ultrafast),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveResponsesServiceTier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveResponsesServiceTier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Server events for Live. Response lifecycle events are wrapped inside response.event; dispatch the nested event by its full type and tolerate new response event types. Follow the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) when designing the conversation and delegation policy."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct LiveServerEvent(pub LiveServerEvent2);
impl ::std::ops::Deref for LiveServerEvent {
    type Target = LiveServerEvent2;
    fn deref(&self) -> &LiveServerEvent2 {
        &self.0
    }
}
impl ::std::convert::From<LiveServerEvent> for LiveServerEvent2 {
    fn from(value: LiveServerEvent) -> Self {
        value.0
    }
}
impl ::std::convert::From<LiveServerEvent2> for LiveServerEvent {
    fn from(value: LiveServerEvent2) -> Self {
        Self(value)
    }
}
#[doc = "An event sent by the Live API on a primary WebSocket or sideband connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveServerEvent2 {
    SessionStarted(LiveSessionStarted),
    SessionUpdated(LiveSessionUpdated),
    InputAudioMuted(LiveInputAudioMuted),
    InputAudioUnmuted(LiveInputAudioUnmuted),
    InstructionsAppended(LiveInstructionsAppended),
    ThinkingAppended(LiveThinkingAppended),
    CommentaryAppended(LiveCommentaryAppended),
    InputAudioAppend(LiveInputAudioAppend),
    OutputAudioDelta(LiveOutputAudioDelta),
    InputTranscriptDelta(LiveInputTranscriptDelta),
    OutputTranscriptDelta(LiveOutputTranscriptDelta),
    DelegationCreated(LiveDelegationCreated),
    ResponseEvent(LiveResponseEvent),
    SessionUsageUpdated(LiveSessionUsageUpdated),
    SessionClosed(LiveSessionClosed),
    ErrorEvent(LiveErrorEvent),
    InfoEvent(LiveInfoEvent),
    TransportDtmfReceived(LiveTransportDtmfReceived),
    TransportDtmfSend(LiveTransportDtmfSend),
    TransportRinging(LiveTransportRinging),
    TransportAnswered(LiveTransportAnswered),
    TransportFailed(LiveTransportFailed),
}
impl ::std::convert::From<LiveSessionStarted> for LiveServerEvent2 {
    fn from(value: LiveSessionStarted) -> Self {
        Self::SessionStarted(value)
    }
}
impl ::std::convert::From<LiveSessionUpdated> for LiveServerEvent2 {
    fn from(value: LiveSessionUpdated) -> Self {
        Self::SessionUpdated(value)
    }
}
impl ::std::convert::From<LiveInputAudioMuted> for LiveServerEvent2 {
    fn from(value: LiveInputAudioMuted) -> Self {
        Self::InputAudioMuted(value)
    }
}
impl ::std::convert::From<LiveInputAudioUnmuted> for LiveServerEvent2 {
    fn from(value: LiveInputAudioUnmuted) -> Self {
        Self::InputAudioUnmuted(value)
    }
}
impl ::std::convert::From<LiveInstructionsAppended> for LiveServerEvent2 {
    fn from(value: LiveInstructionsAppended) -> Self {
        Self::InstructionsAppended(value)
    }
}
impl ::std::convert::From<LiveThinkingAppended> for LiveServerEvent2 {
    fn from(value: LiveThinkingAppended) -> Self {
        Self::ThinkingAppended(value)
    }
}
impl ::std::convert::From<LiveCommentaryAppended> for LiveServerEvent2 {
    fn from(value: LiveCommentaryAppended) -> Self {
        Self::CommentaryAppended(value)
    }
}
impl ::std::convert::From<LiveInputAudioAppend> for LiveServerEvent2 {
    fn from(value: LiveInputAudioAppend) -> Self {
        Self::InputAudioAppend(value)
    }
}
impl ::std::convert::From<LiveOutputAudioDelta> for LiveServerEvent2 {
    fn from(value: LiveOutputAudioDelta) -> Self {
        Self::OutputAudioDelta(value)
    }
}
impl ::std::convert::From<LiveInputTranscriptDelta> for LiveServerEvent2 {
    fn from(value: LiveInputTranscriptDelta) -> Self {
        Self::InputTranscriptDelta(value)
    }
}
impl ::std::convert::From<LiveOutputTranscriptDelta> for LiveServerEvent2 {
    fn from(value: LiveOutputTranscriptDelta) -> Self {
        Self::OutputTranscriptDelta(value)
    }
}
impl ::std::convert::From<LiveDelegationCreated> for LiveServerEvent2 {
    fn from(value: LiveDelegationCreated) -> Self {
        Self::DelegationCreated(value)
    }
}
impl ::std::convert::From<LiveResponseEvent> for LiveServerEvent2 {
    fn from(value: LiveResponseEvent) -> Self {
        Self::ResponseEvent(value)
    }
}
impl ::std::convert::From<LiveSessionUsageUpdated> for LiveServerEvent2 {
    fn from(value: LiveSessionUsageUpdated) -> Self {
        Self::SessionUsageUpdated(value)
    }
}
impl ::std::convert::From<LiveSessionClosed> for LiveServerEvent2 {
    fn from(value: LiveSessionClosed) -> Self {
        Self::SessionClosed(value)
    }
}
impl ::std::convert::From<LiveErrorEvent> for LiveServerEvent2 {
    fn from(value: LiveErrorEvent) -> Self {
        Self::ErrorEvent(value)
    }
}
impl ::std::convert::From<LiveInfoEvent> for LiveServerEvent2 {
    fn from(value: LiveInfoEvent) -> Self {
        Self::InfoEvent(value)
    }
}
impl ::std::convert::From<LiveTransportDtmfReceived> for LiveServerEvent2 {
    fn from(value: LiveTransportDtmfReceived) -> Self {
        Self::TransportDtmfReceived(value)
    }
}
impl ::std::convert::From<LiveTransportDtmfSend> for LiveServerEvent2 {
    fn from(value: LiveTransportDtmfSend) -> Self {
        Self::TransportDtmfSend(value)
    }
}
impl ::std::convert::From<LiveTransportRinging> for LiveServerEvent2 {
    fn from(value: LiveTransportRinging) -> Self {
        Self::TransportRinging(value)
    }
}
impl ::std::convert::From<LiveTransportAnswered> for LiveServerEvent2 {
    fn from(value: LiveTransportAnswered) -> Self {
        Self::TransportAnswered(value)
    }
}
impl ::std::convert::From<LiveTransportFailed> for LiveServerEvent2 {
    fn from(value: LiveTransportFailed) -> Self {
        Self::TransportFailed(value)
    }
}
#[doc = "Raw, mono 16-bit little-endian PCM audio for a Live WebSocket connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionAudioFormatPcmParam {
    #[doc = "Audio sample rate in hertz. Live WebSocket PCM audio supports 16000 or 24000 Hz."]
    pub rate: LiveSessionAudioFormatPcmParamRate,
    #[doc = "The audio encoding. Always `audio/pcm`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionAudioFormatPcmParamType,
}
impl LiveSessionAudioFormatPcmParam {
    pub fn builder() -> builder::LiveSessionAudioFormatPcmParam {
        Default::default()
    }
}
#[doc = "Audio sample rate in hertz. Live WebSocket PCM audio supports 16000 or 24000 Hz."]
#[derive(:: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct LiveSessionAudioFormatPcmParamRate(i64);
impl ::std::ops::Deref for LiveSessionAudioFormatPcmParamRate {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<LiveSessionAudioFormatPcmParamRate> for i64 {
    fn from(value: LiveSessionAudioFormatPcmParamRate) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for LiveSessionAudioFormatPcmParamRate {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![16000_i64, 24000_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSessionAudioFormatPcmParamRate {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "The audio encoding. Always `audio/pcm`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionAudioFormatPcmParamType {
    #[serde(rename = "audio/pcm")]
    AudioPcm,
}
impl ::std::fmt::Display for LiveSessionAudioFormatPcmParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcm => f.write_str("audio/pcm"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionAudioFormatPcmParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcm" => Ok(Self::AudioPcm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionAudioFormatPcmParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionAudioFormatPcmParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Raw, mono G.711 A-law audio for a Live WebSocket connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionAudioFormatPcmaParam {
    #[doc = "Audio sample rate in hertz. G.711 audio uses 8000 Hz."]
    pub rate: i64,
    #[doc = "The audio encoding. Always `audio/pcma`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionAudioFormatPcmaParamType,
}
impl LiveSessionAudioFormatPcmaParam {
    pub fn builder() -> builder::LiveSessionAudioFormatPcmaParam {
        Default::default()
    }
}
#[doc = "The audio encoding. Always `audio/pcma`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionAudioFormatPcmaParamType {
    #[serde(rename = "audio/pcma")]
    AudioPcma,
}
impl ::std::fmt::Display for LiveSessionAudioFormatPcmaParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcma => f.write_str("audio/pcma"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionAudioFormatPcmaParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcma" => Ok(Self::AudioPcma),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionAudioFormatPcmaParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionAudioFormatPcmaParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Raw, mono G.711 μ-law audio for a Live WebSocket connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionAudioFormatPcmuParam {
    #[doc = "Audio sample rate in hertz. G.711 audio uses 8000 Hz."]
    pub rate: i64,
    #[doc = "The audio encoding. Always `audio/pcmu`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionAudioFormatPcmuParamType,
}
impl LiveSessionAudioFormatPcmuParam {
    pub fn builder() -> builder::LiveSessionAudioFormatPcmuParam {
        Default::default()
    }
}
#[doc = "The audio encoding. Always `audio/pcmu`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionAudioFormatPcmuParamType {
    #[serde(rename = "audio/pcmu")]
    AudioPcmu,
}
impl ::std::fmt::Display for LiveSessionAudioFormatPcmuParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AudioPcmu => f.write_str("audio/pcmu"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionAudioFormatPcmuParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "audio/pcmu" => Ok(Self::AudioPcmu),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionAudioFormatPcmuParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionAudioFormatPcmuParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Request that the Live session close. The terminal `session.closed` event contains the close reason and final usage."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionCloseParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveSessionCloseParamEventId>,
    #[doc = "The Live client event type. Always `session.close`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionCloseParamType,
}
impl LiveSessionCloseParam {
    pub fn builder() -> builder::LiveSessionCloseParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSessionCloseParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveSessionCloseParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSessionCloseParamEventId> for ::std::string::String {
    fn from(value: LiveSessionCloseParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSessionCloseParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionCloseParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionCloseParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSessionCloseParamEventId {
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
#[doc = "The Live client event type. Always `session.close`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionCloseParamType {
    #[serde(rename = "session.close")]
    SessionClose,
}
impl ::std::fmt::Display for LiveSessionCloseParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionClose => f.write_str("session.close"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionCloseParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.close" => Ok(Self::SessionClose),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionCloseParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionCloseParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned after the Live session finishes finalizing, with the close reason, final session snapshot, and cumulative audio usage. A connection closing without this event does not confirm successful finalization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionClosed {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "Why the Live session ended: `close_requested` for an application close or hangup request, `expired` for the session duration limit, `content` for a safety filter, `remote_hangup` for a graceful remote disconnect, or `connection_lost` for an unexpected primary or upstream disconnection."]
    pub reason: LiveSessionClosedReason,
    pub session: LiveSessionResourceParam,
    #[doc = "The event type, always `session.closed`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionClosedType,
    #[doc = "The final cumulative Live audio usage after session finalization."]
    pub usage: LiveSessionUsage,
}
impl LiveSessionClosed {
    pub fn builder() -> builder::LiveSessionClosed {
        Default::default()
    }
}
#[doc = "Why the Live session ended: `close_requested` for an application close or hangup request, `expired` for the session duration limit, `content` for a safety filter, `remote_hangup` for a graceful remote disconnect, or `connection_lost` for an unexpected primary or upstream disconnection."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionClosedReason {
    #[serde(rename = "close_requested")]
    CloseRequested,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "remote_hangup")]
    RemoteHangup,
    #[serde(rename = "connection_lost")]
    ConnectionLost,
}
impl ::std::fmt::Display for LiveSessionClosedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CloseRequested => f.write_str("close_requested"),
            Self::Expired => f.write_str("expired"),
            Self::Content => f.write_str("content"),
            Self::RemoteHangup => f.write_str("remote_hangup"),
            Self::ConnectionLost => f.write_str("connection_lost"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionClosedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "close_requested" => Ok(Self::CloseRequested),
            "expired" => Ok(Self::Expired),
            "content" => Ok(Self::Content),
            "remote_hangup" => Ok(Self::RemoteHangup),
            "connection_lost" => Ok(Self::ConnectionLost),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionClosedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionClosedReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The event type, always `session.closed`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionClosedType {
    #[serde(rename = "session.closed")]
    SessionClosed,
}
impl ::std::fmt::Display for LiveSessionClosedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionClosed => f.write_str("session.closed"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionClosedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.closed" => Ok(Self::SessionClosed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionClosedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionClosedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Initial configuration for a Live session, including its model, conversation instructions, audio, and delegated task handling."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionCreateParams {
    #[doc = "Startup audio configuration. Only primary WebSockets accept audio.format; WebRTC and SIP negotiate their media format. Voice and format are immutable after startup."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<LiveInitialSessionAudioParam>,
    #[doc = "Startup-only capabilities for an untrusted frontend attached to a unified WebRTC session. Trusted sideband connections are unaffected."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<LiveClientConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<LiveSessionCreateParamsDelegation>,
    #[doc = "Ordered text-only history supplied before startup. Supports developer, user, and assistant messages with one text part each; at most 128 messages and 8,192 rendered tokens in total."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub input: ::std::vec::Vec<LiveInitialItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    pub model: ModelIdsLive,
    #[doc = "Whether to store the session for later forking and recording download. Defaults to false for new sessions."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<bool>,
}
impl LiveSessionCreateParams {
    pub fn builder() -> builder::LiveSessionCreateParams {
        Default::default()
    }
}
#[doc = "Who handles tasks delegated by the Live model. Omitted or null selects your application; use `responses` to let the API manage a Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSessionCreateParamsDelegation {
    ClientDelegationParam(LiveClientDelegationParam),
    ResponsesDelegationParam(LiveResponsesDelegationParam),
}
impl ::std::convert::From<LiveClientDelegationParam> for LiveSessionCreateParamsDelegation {
    fn from(value: LiveClientDelegationParam) -> Self {
        Self::ClientDelegationParam(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationParam> for LiveSessionCreateParamsDelegation {
    fn from(value: LiveResponsesDelegationParam) -> Self {
        Self::ResponsesDelegationParam(value)
    }
}
#[doc = "Create a Live session with a WebRTC SDP offer or place an outbound SIP call. Follow the [Live prompting guide](https://developers.openai.com/api/docs/guides/live-prompting) before choosing frontend and backend instructions. The request starts the session; do not send session.start again on the data channel or sideband."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveSessionCreateRequest {
    #[doc = "Startup configuration for the Live session."]
    pub session: LiveMediaSessionCreateParams,
    #[doc = "WebRTC transport with an SDP offer, or SIP transport with a destination and per-call trunk credentials."]
    pub transport: LiveSessionCreateRequestTransport,
}
impl LiveSessionCreateRequest {
    pub fn builder() -> builder::LiveSessionCreateRequest {
        Default::default()
    }
}
#[doc = "WebRTC transport with an SDP offer, or SIP transport with a destination and per-call trunk credentials."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSessionCreateRequestTransport {
    WebRtcTransport(LiveWebRtcTransport),
    SipTransport(LiveSipTransport),
}
impl ::std::convert::From<LiveWebRtcTransport> for LiveSessionCreateRequestTransport {
    fn from(value: LiveWebRtcTransport) -> Self {
        Self::WebRtcTransport(value)
    }
}
impl ::std::convert::From<LiveSipTransport> for LiveSessionCreateRequestTransport {
    fn from(value: LiveSipTransport) -> Self {
        Self::SipTransport(value)
    }
}
#[doc = "The created Live session and its transport. WebRTC creation returns 201 Created with an SDP answer. Outbound SIP creation returns 201 Created after initialization, before the callee necessarily answers, without SDP or trunk credentials."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionCreateResponse {
    pub session: Session,
    #[doc = "WebRTC transport with an SDP answer, or SIP transport without SDP or trunk credentials. For SIP, attach a sideband using session.id to receive transport.ringing, transport.answered, and transport.failed notifications."]
    pub transport: LiveSessionCreateResponseTransport,
}
impl LiveSessionCreateResponse {
    pub fn builder() -> builder::LiveSessionCreateResponse {
        Default::default()
    }
}
#[doc = "WebRTC transport with an SDP answer, or SIP transport without SDP or trunk credentials. For SIP, attach a sideband using session.id to receive transport.ringing, transport.answered, and transport.failed notifications."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSessionCreateResponseTransport {
    LiveWebRtcTransport(LiveWebRtcTransport),
    Object {
        #[doc = "The transport used for the Live session. Always `sip`."]
        #[serde(rename = "type")]
        type_: LiveSessionCreateResponseTransportObjectType,
    },
}
impl ::std::convert::From<LiveWebRtcTransport> for LiveSessionCreateResponseTransport {
    fn from(value: LiveWebRtcTransport) -> Self {
        Self::LiveWebRtcTransport(value)
    }
}
#[doc = "The transport used for the Live session. Always `sip`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionCreateResponseTransportObjectType {
    #[serde(rename = "sip")]
    Sip,
}
impl ::std::fmt::Display for LiveSessionCreateResponseTransportObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sip => f.write_str("sip"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionCreateResponseTransportObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sip" => Ok(Self::Sip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionCreateResponseTransportObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveSessionCreateResponseTransportObjectType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The resolved Live session configuration and server-assigned session metadata."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionResourceParam {
    #[doc = "Startup audio configuration. Only primary WebSockets accept audio.format; WebRTC and SIP negotiate their media format. Voice and format are immutable after startup."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub audio: ::std::option::Option<LiveInitialSessionAudioParam>,
    #[doc = "Startup-only capabilities for an untrusted frontend attached to a unified WebRTC session. Trusted sideband connections are unaffected."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<LiveClientConfigParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<LiveSessionResourceParamDelegation>,
    #[doc = "The Unix timestamp, in seconds, at which the Live session expires."]
    pub expires_at: i64,
    #[doc = "The unique ID of the Live session. Use this ID for sideband connections, forking, and recording download."]
    pub id: ::std::string::String,
    #[doc = "Ordered text-only history supplied before startup. Supports developer, user, and assistant messages with one text part each; at most 128 messages and 8,192 rendered tokens in total."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub input: ::std::vec::Vec<LiveInitialItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    pub model: ModelIdsLive,
    #[doc = "The status of the session snapshot. Always `active`, including the final snapshot in session.closed; use the event type to determine that the session has closed."]
    pub status: LiveSessionResourceParamStatus,
    #[doc = "Whether to store the session for later forking and recording download. Defaults to false for new sessions."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<bool>,
}
impl LiveSessionResourceParam {
    pub fn builder() -> builder::LiveSessionResourceParam {
        Default::default()
    }
}
#[doc = "Who handles tasks delegated by the Live model. Omitted or null selects your application; use `responses` to let the API manage a Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSessionResourceParamDelegation {
    ClientDelegationParam(LiveClientDelegationParam),
    ResponsesDelegationParam(LiveResponsesDelegationParam),
}
impl ::std::convert::From<LiveClientDelegationParam> for LiveSessionResourceParamDelegation {
    fn from(value: LiveClientDelegationParam) -> Self {
        Self::ClientDelegationParam(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationParam> for LiveSessionResourceParamDelegation {
    fn from(value: LiveResponsesDelegationParam) -> Self {
        Self::ResponsesDelegationParam(value)
    }
}
#[doc = "The status of the session snapshot. Always `active`, including the final snapshot in session.closed; use the event type to determine that the session has closed."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionResourceParamStatus {
    #[serde(rename = "active")]
    Active,
}
impl ::std::fmt::Display for LiveSessionResourceParamStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionResourceParamStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionResourceParamStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionResourceParamStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Start a Live session on a primary WebSocket. Send this event before other commands and wait for `session.started`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionStartEvent {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveSessionStartEventEventId>,
    #[doc = "Initial configuration for a primary WebSocket. Send session.start first and wait for session.started before application commands. WebRTC creation already starts the session; do not send this event again on its data channel."]
    pub session: LiveSessionCreateParams,
    #[doc = "The Live client event type. Always `session.start`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionStartEventType,
}
impl LiveSessionStartEvent {
    pub fn builder() -> builder::LiveSessionStartEvent {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSessionStartEventEventId(::std::string::String);
impl ::std::ops::Deref for LiveSessionStartEventEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSessionStartEventEventId> for ::std::string::String {
    fn from(value: LiveSessionStartEventEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSessionStartEventEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionStartEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionStartEventEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSessionStartEventEventId {
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
#[doc = "The Live client event type. Always `session.start`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionStartEventType {
    #[serde(rename = "session.start")]
    SessionStart,
}
impl ::std::fmt::Display for LiveSessionStartEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStart => f.write_str("session.start"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionStartEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.start" => Ok(Self::SessionStart),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionStartEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionStartEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a Live session has started. Contains the resolved session configuration, including server defaults."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionStarted {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    pub session: LiveSessionResourceParam,
    #[doc = "The event type, always `session.started`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionStartedType,
}
impl LiveSessionStarted {
    pub fn builder() -> builder::LiveSessionStarted {
        Default::default()
    }
}
#[doc = "The event type, always `session.started`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionStartedType {
    #[serde(rename = "session.started")]
    SessionStarted,
}
impl ::std::fmt::Display for LiveSessionStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStarted => f.write_str("session.started"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionStartedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.started" => Ok(Self::SessionStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Update the delegation settings of an active Live session. The server acknowledges accepted changes with `session.updated`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionUpdateParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveSessionUpdateParamEventId>,
    #[doc = "Sparse delegation updates. Omitted settings retain their values. The delegation type cannot change, including resetting Responses delegation to null or client. Model, frontend instructions, audio, and startup input are immutable."]
    pub session: LiveSessionUpdateParams,
    #[doc = "The Live client event type. Always `session.update`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionUpdateParamType,
}
impl LiveSessionUpdateParam {
    pub fn builder() -> builder::LiveSessionUpdateParam {
        Default::default()
    }
}
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSessionUpdateParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveSessionUpdateParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSessionUpdateParamEventId> for ::std::string::String {
    fn from(value: LiveSessionUpdateParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSessionUpdateParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionUpdateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionUpdateParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSessionUpdateParamEventId {
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
#[doc = "The Live client event type. Always `session.update`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionUpdateParamType {
    #[serde(rename = "session.update")]
    SessionUpdate,
}
impl ::std::fmt::Display for LiveSessionUpdateParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdate => f.write_str("session.update"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionUpdateParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.update" => Ok(Self::SessionUpdate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionUpdateParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionUpdateParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Changes to an active Live session. Only delegation backend settings can be updated after startup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveSessionUpdateParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delegation: ::std::option::Option<LiveSessionUpdateParamsDelegation>,
}
impl LiveSessionUpdateParams {
    pub fn builder() -> builder::LiveSessionUpdateParams {
        Default::default()
    }
}
#[doc = "Delegation settings to update. The delegation type must match the current session; omitted settings retain their values."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSessionUpdateParamsDelegation {
    ClientDelegationParam(LiveClientDelegationParam),
    ResponsesDelegationUpdateParam(LiveResponsesDelegationUpdateParam),
}
impl ::std::convert::From<LiveClientDelegationParam> for LiveSessionUpdateParamsDelegation {
    fn from(value: LiveClientDelegationParam) -> Self {
        Self::ClientDelegationParam(value)
    }
}
impl ::std::convert::From<LiveResponsesDelegationUpdateParam>
    for LiveSessionUpdateParamsDelegation
{
    fn from(value: LiveResponsesDelegationUpdateParam) -> Self {
        Self::ResponsesDelegationUpdateParam(value)
    }
}
#[doc = "Returned when a Live session update is accepted. Contains the resolved session configuration after the update."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionUpdated {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    pub session: LiveSessionResourceParam,
    #[doc = "The event type, always `session.updated`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionUpdatedType,
}
impl LiveSessionUpdated {
    pub fn builder() -> builder::LiveSessionUpdated {
        Default::default()
    }
}
#[doc = "The event type, always `session.updated`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionUpdatedType {
    #[serde(rename = "session.updated")]
    SessionUpdated,
}
impl ::std::fmt::Display for LiveSessionUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUpdated => f.write_str("session.updated"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.updated" => Ok(Self::SessionUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Cumulative audio duration for a Live session. Values are totals for the session, not increments to sum across usage events."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionUsage {
    #[doc = "The cumulative Live audio duration in seconds. Do not sum this value across usage events."]
    pub seconds: f64,
}
impl LiveSessionUsage {
    pub fn builder() -> builder::LiveSessionUsage {
        Default::default()
    }
}
#[doc = "Reports cumulative Live audio usage and, when available, the most recent context-window usage. Delegated Responses token usage is reported separately in response.event events."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSessionUsageUpdated {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The latest measured Live context-window usage. Omitted when the context limit is unknown."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context_window: ::std::option::Option<LiveContextWindowUsage>,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The event type, always `session.usage.updated`."]
    #[serde(rename = "type")]
    pub type_: LiveSessionUsageUpdatedType,
    #[doc = "The cumulative Live audio usage so far."]
    pub usage: LiveSessionUsage,
}
impl LiveSessionUsageUpdated {
    pub fn builder() -> builder::LiveSessionUsageUpdated {
        Default::default()
    }
}
#[doc = "The event type, always `session.usage.updated`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSessionUsageUpdatedType {
    #[serde(rename = "session.usage.updated")]
    SessionUsageUpdated,
}
impl ::std::fmt::Display for LiveSessionUsageUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUsageUpdated => f.write_str("session.usage.updated"),
        }
    }
}
impl ::std::str::FromStr for LiveSessionUsageUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.usage.updated" => Ok(Self::SessionUsageUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSessionUsageUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSessionUsageUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Client events accepted by an attached Live sideband WebSocket. The session is already started; send audio over the primary connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSidebandClientEvent {
    SessionUpdateParam(LiveSessionUpdateParam),
    InputAudioMuteParam(LiveInputAudioMuteParam),
    InputAudioUnmuteParam(LiveInputAudioUnmuteParam),
    InstructionsAppendParam(LiveInstructionsAppendParam),
    ThinkingAppendParam(LiveThinkingAppendParam),
    CommentaryAppendParam(LiveCommentaryAppendParam),
    ResponseItemCreateParam(LiveResponseItemCreateParam),
    ResponseCreateParam(LiveResponseCreateParam),
    SessionCloseParam(LiveSessionCloseParam),
}
impl ::std::convert::From<LiveSessionUpdateParam> for LiveSidebandClientEvent {
    fn from(value: LiveSessionUpdateParam) -> Self {
        Self::SessionUpdateParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioMuteParam> for LiveSidebandClientEvent {
    fn from(value: LiveInputAudioMuteParam) -> Self {
        Self::InputAudioMuteParam(value)
    }
}
impl ::std::convert::From<LiveInputAudioUnmuteParam> for LiveSidebandClientEvent {
    fn from(value: LiveInputAudioUnmuteParam) -> Self {
        Self::InputAudioUnmuteParam(value)
    }
}
impl ::std::convert::From<LiveInstructionsAppendParam> for LiveSidebandClientEvent {
    fn from(value: LiveInstructionsAppendParam) -> Self {
        Self::InstructionsAppendParam(value)
    }
}
impl ::std::convert::From<LiveThinkingAppendParam> for LiveSidebandClientEvent {
    fn from(value: LiveThinkingAppendParam) -> Self {
        Self::ThinkingAppendParam(value)
    }
}
impl ::std::convert::From<LiveCommentaryAppendParam> for LiveSidebandClientEvent {
    fn from(value: LiveCommentaryAppendParam) -> Self {
        Self::CommentaryAppendParam(value)
    }
}
impl ::std::convert::From<LiveResponseItemCreateParam> for LiveSidebandClientEvent {
    fn from(value: LiveResponseItemCreateParam) -> Self {
        Self::ResponseItemCreateParam(value)
    }
}
impl ::std::convert::From<LiveResponseCreateParam> for LiveSidebandClientEvent {
    fn from(value: LiveResponseCreateParam) -> Self {
        Self::ResponseCreateParam(value)
    }
}
impl ::std::convert::From<LiveSessionCloseParam> for LiveSidebandClientEvent {
    fn from(value: LiveSessionCloseParam) -> Self {
        Self::SessionCloseParam(value)
    }
}
#[doc = "Server events received by an attached Live sideband WebSocket. Audio deltas are delivered over the primary connection."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum LiveSidebandServerEvent {
    TransportRinging(LiveTransportRinging),
    TransportAnswered(LiveTransportAnswered),
    TransportFailed(LiveTransportFailed),
    SessionStarted(LiveSessionStarted),
    SessionUpdated(LiveSessionUpdated),
    InputAudioMuted(LiveInputAudioMuted),
    InputAudioUnmuted(LiveInputAudioUnmuted),
    InstructionsAppended(LiveInstructionsAppended),
    ThinkingAppended(LiveThinkingAppended),
    CommentaryAppended(LiveCommentaryAppended),
    InputTranscriptDelta(LiveInputTranscriptDelta),
    OutputTranscriptDelta(LiveOutputTranscriptDelta),
    DelegationCreated(LiveDelegationCreated),
    ResponseEvent(LiveResponseEvent),
    SessionUsageUpdated(LiveSessionUsageUpdated),
    SessionClosed(LiveSessionClosed),
    ErrorEvent(LiveErrorEvent),
    InfoEvent(LiveInfoEvent),
}
impl ::std::convert::From<LiveTransportRinging> for LiveSidebandServerEvent {
    fn from(value: LiveTransportRinging) -> Self {
        Self::TransportRinging(value)
    }
}
impl ::std::convert::From<LiveTransportAnswered> for LiveSidebandServerEvent {
    fn from(value: LiveTransportAnswered) -> Self {
        Self::TransportAnswered(value)
    }
}
impl ::std::convert::From<LiveTransportFailed> for LiveSidebandServerEvent {
    fn from(value: LiveTransportFailed) -> Self {
        Self::TransportFailed(value)
    }
}
impl ::std::convert::From<LiveSessionStarted> for LiveSidebandServerEvent {
    fn from(value: LiveSessionStarted) -> Self {
        Self::SessionStarted(value)
    }
}
impl ::std::convert::From<LiveSessionUpdated> for LiveSidebandServerEvent {
    fn from(value: LiveSessionUpdated) -> Self {
        Self::SessionUpdated(value)
    }
}
impl ::std::convert::From<LiveInputAudioMuted> for LiveSidebandServerEvent {
    fn from(value: LiveInputAudioMuted) -> Self {
        Self::InputAudioMuted(value)
    }
}
impl ::std::convert::From<LiveInputAudioUnmuted> for LiveSidebandServerEvent {
    fn from(value: LiveInputAudioUnmuted) -> Self {
        Self::InputAudioUnmuted(value)
    }
}
impl ::std::convert::From<LiveInstructionsAppended> for LiveSidebandServerEvent {
    fn from(value: LiveInstructionsAppended) -> Self {
        Self::InstructionsAppended(value)
    }
}
impl ::std::convert::From<LiveThinkingAppended> for LiveSidebandServerEvent {
    fn from(value: LiveThinkingAppended) -> Self {
        Self::ThinkingAppended(value)
    }
}
impl ::std::convert::From<LiveCommentaryAppended> for LiveSidebandServerEvent {
    fn from(value: LiveCommentaryAppended) -> Self {
        Self::CommentaryAppended(value)
    }
}
impl ::std::convert::From<LiveInputTranscriptDelta> for LiveSidebandServerEvent {
    fn from(value: LiveInputTranscriptDelta) -> Self {
        Self::InputTranscriptDelta(value)
    }
}
impl ::std::convert::From<LiveOutputTranscriptDelta> for LiveSidebandServerEvent {
    fn from(value: LiveOutputTranscriptDelta) -> Self {
        Self::OutputTranscriptDelta(value)
    }
}
impl ::std::convert::From<LiveDelegationCreated> for LiveSidebandServerEvent {
    fn from(value: LiveDelegationCreated) -> Self {
        Self::DelegationCreated(value)
    }
}
impl ::std::convert::From<LiveResponseEvent> for LiveSidebandServerEvent {
    fn from(value: LiveResponseEvent) -> Self {
        Self::ResponseEvent(value)
    }
}
impl ::std::convert::From<LiveSessionUsageUpdated> for LiveSidebandServerEvent {
    fn from(value: LiveSessionUsageUpdated) -> Self {
        Self::SessionUsageUpdated(value)
    }
}
impl ::std::convert::From<LiveSessionClosed> for LiveSidebandServerEvent {
    fn from(value: LiveSessionClosed) -> Self {
        Self::SessionClosed(value)
    }
}
impl ::std::convert::From<LiveErrorEvent> for LiveSidebandServerEvent {
    fn from(value: LiveErrorEvent) -> Self {
        Self::ErrorEvent(value)
    }
}
impl ::std::convert::From<LiveInfoEvent> for LiveSidebandServerEvent {
    fn from(value: LiveInfoEvent) -> Self {
        Self::InfoEvent(value)
    }
}
#[doc = "Place an outbound SIP call using your provider's trunk. Outbound SIP calling must be enabled for your organization. The trunk must support TLS signaling, Opus audio, and SDES-SRTP media. See [Telephony and SIP](https://developers.openai.com/api/docs/guides/voice-sip?api=live#place-an-outbound-call)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveSipTransport {
    #[doc = "Phone number to call in E.164 format. SIP URI destinations are not supported."]
    pub destination: LiveSipTransportDestination,
    pub trunk: LiveSipTrunk,
    #[doc = "The transport used for the Live session. Always `sip`."]
    #[serde(rename = "type")]
    pub type_: LiveSipTransportType,
}
impl LiveSipTransport {
    pub fn builder() -> builder::LiveSipTransport {
        Default::default()
    }
}
#[doc = "Phone number to call in E.164 format. SIP URI destinations are not supported."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSipTransportDestination(::std::string::String);
impl ::std::ops::Deref for LiveSipTransportDestination {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSipTransportDestination> for ::std::string::String {
    fn from(value: LiveSipTransportDestination) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSipTransportDestination {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\+[1-9][0-9]{1,14}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\+[1-9][0-9]{1,14}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTransportDestination {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTransportDestination {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSipTransportDestination {
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
#[doc = "The transport used for the Live session. Always `sip`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSipTransportType {
    #[serde(rename = "sip")]
    Sip,
}
impl ::std::fmt::Display for LiveSipTransportType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sip => f.write_str("sip"),
        }
    }
}
impl ::std::str::FromStr for LiveSipTransportType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sip" => Ok(Self::Sip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTransportType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTransportType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SIP trunk configuration supplied for this call. Keep trunk credentials on your server. The creation response does not return this configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveSipTrunk {
    pub auth: LiveSipTrunkAuth,
    #[doc = "Caller phone number in E.164 format to use in the SIP From header."]
    pub caller_number: LiveSipTrunkCallerNumber,
    #[doc = "Provider endpoint in the form `sips:host[:port][;transport=tcp]`. The default port is 5061. IPv6 addresses must be bracketed. TLS signaling is required; plaintext SIP is not supported. Do not include userinfo, a path, URI headers, or other URI parameters. Local hostnames and literal private, loopback, link-local, unspecified, multicast, and broadcast IP addresses are rejected."]
    pub provider_url: ::std::string::String,
}
impl LiveSipTrunk {
    pub fn builder() -> builder::LiveSipTrunk {
        Default::default()
    }
}
#[doc = "Credentials for SIP Digest authentication with your trunk provider."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveSipTrunkAuth {
    #[doc = "Provider password. Must be at most 4096 bytes and contain no CR, LF, or NUL."]
    pub password: LiveSipTrunkAuthPassword,
    #[doc = "Use `digest` for SIP Digest authentication. Handles 401 and 407 challenges."]
    #[serde(rename = "type")]
    pub type_: LiveSipTrunkAuthType,
    #[doc = "Provider username. Must contain a non-whitespace character, be at most 256 bytes, and contain no CR, LF, or NUL."]
    pub username: LiveSipTrunkAuthUsername,
}
impl LiveSipTrunkAuth {
    pub fn builder() -> builder::LiveSipTrunkAuth {
        Default::default()
    }
}
#[doc = "Provider password. Must be at most 4096 bytes and contain no CR, LF, or NUL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSipTrunkAuthPassword(::std::string::String);
impl ::std::ops::Deref for LiveSipTrunkAuthPassword {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSipTrunkAuthPassword> for ::std::string::String {
    fn from(value: LiveSipTrunkAuthPassword) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSipTrunkAuthPassword {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTrunkAuthPassword {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTrunkAuthPassword {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSipTrunkAuthPassword {
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
#[doc = "Use `digest` for SIP Digest authentication. Handles 401 and 407 challenges."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSipTrunkAuthType {
    #[serde(rename = "digest")]
    Digest,
}
impl ::std::fmt::Display for LiveSipTrunkAuthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Digest => f.write_str("digest"),
        }
    }
}
impl ::std::str::FromStr for LiveSipTrunkAuthType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "digest" => Ok(Self::Digest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTrunkAuthType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTrunkAuthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Provider username. Must contain a non-whitespace character, be at most 256 bytes, and contain no CR, LF, or NUL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSipTrunkAuthUsername(::std::string::String);
impl ::std::ops::Deref for LiveSipTrunkAuthUsername {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSipTrunkAuthUsername> for ::std::string::String {
    fn from(value: LiveSipTrunkAuthUsername) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSipTrunkAuthUsername {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTrunkAuthUsername {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTrunkAuthUsername {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSipTrunkAuthUsername {
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
#[doc = "Caller phone number in E.164 format to use in the SIP From header."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSipTrunkCallerNumber(::std::string::String);
impl ::std::ops::Deref for LiveSipTrunkCallerNumber {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSipTrunkCallerNumber> for ::std::string::String {
    fn from(value: LiveSipTrunkCallerNumber) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSipTrunkCallerNumber {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\+[1-9][0-9]{1,14}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\+[1-9][0-9]{1,14}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSipTrunkCallerNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSipTrunkCallerNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSipTrunkCallerNumber {
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
#[doc = "`LiveSkillReferenceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSkillReferenceParam {
    #[doc = "The ID of the referenced skill."]
    pub skill_id: LiveSkillReferenceParamSkillId,
    #[doc = "References a skill created with the /v1/skills endpoint."]
    #[serde(rename = "type")]
    pub type_: LiveSkillReferenceParamType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl LiveSkillReferenceParam {
    pub fn builder() -> builder::LiveSkillReferenceParam {
        Default::default()
    }
}
#[doc = "The ID of the referenced skill."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveSkillReferenceParamSkillId(::std::string::String);
impl ::std::ops::Deref for LiveSkillReferenceParamSkillId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveSkillReferenceParamSkillId> for ::std::string::String {
    fn from(value: LiveSkillReferenceParamSkillId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveSkillReferenceParamSkillId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveSkillReferenceParamSkillId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSkillReferenceParamSkillId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveSkillReferenceParamSkillId {
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
#[doc = "References a skill created with the /v1/skills endpoint."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSkillReferenceParamType {
    #[serde(rename = "skill_reference")]
    SkillReference,
}
impl ::std::fmt::Display for LiveSkillReferenceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SkillReference => f.write_str("skill_reference"),
        }
    }
}
impl ::std::str::FromStr for LiveSkillReferenceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill_reference" => Ok(Self::SkillReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSkillReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSkillReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Forces the model to call the apply_patch tool when executing a tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificApplyPatchParam {
    #[doc = "The tool to call. Always `apply_patch`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificApplyPatchParamType,
}
impl LiveSpecificApplyPatchParam {
    pub fn builder() -> builder::LiveSpecificApplyPatchParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `apply_patch`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificApplyPatchParamType {
    #[serde(rename = "apply_patch")]
    ApplyPatch,
}
impl ::std::fmt::Display for LiveSpecificApplyPatchParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatch => f.write_str("apply_patch"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificApplyPatchParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch" => Ok(Self::ApplyPatch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificApplyPatchParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificApplyPatchParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificCodeInterpreterParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificCodeInterpreterParam {
    #[doc = "The tool to call. Always `code_interpreter`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificCodeInterpreterParamType,
}
impl LiveSpecificCodeInterpreterParam {
    pub fn builder() -> builder::LiveSpecificCodeInterpreterParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `code_interpreter`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificCodeInterpreterParamType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for LiveSpecificCodeInterpreterParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificCodeInterpreterParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificCodeInterpreterParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificCodeInterpreterParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificComputerParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificComputerParam {
    #[doc = "The tool to call. Always `computer`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificComputerParamType,
}
impl LiveSpecificComputerParam {
    pub fn builder() -> builder::LiveSpecificComputerParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `computer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificComputerParamType {
    #[serde(rename = "computer")]
    Computer,
}
impl ::std::fmt::Display for LiveSpecificComputerParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Computer => f.write_str("computer"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificComputerParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer" => Ok(Self::Computer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificComputerParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificComputerParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificCustomToolParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificCustomToolParam {
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
    #[doc = "The tool to call. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificCustomToolParamType,
}
impl LiveSpecificCustomToolParam {
    pub fn builder() -> builder::LiveSpecificCustomToolParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `custom`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificCustomToolParamType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for LiveSpecificCustomToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificCustomToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificCustomToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificCustomToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificFileSearchParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificFileSearchParam {
    #[doc = "The tool to call. Always `file_search`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificFileSearchParamType,
}
impl LiveSpecificFileSearchParam {
    pub fn builder() -> builder::LiveSpecificFileSearchParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `file_search`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificFileSearchParamType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for LiveSpecificFileSearchParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificFileSearchParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificFileSearchParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificFileSearchParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Forces the model to call the shell tool when a tool call is required."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificFunctionShellParam {
    #[doc = "The tool to call. Always `shell`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificFunctionShellParamType,
}
impl LiveSpecificFunctionShellParam {
    pub fn builder() -> builder::LiveSpecificFunctionShellParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `shell`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificFunctionShellParamType {
    #[serde(rename = "shell")]
    Shell,
}
impl ::std::fmt::Display for LiveSpecificFunctionShellParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificFunctionShellParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificFunctionShellParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificFunctionShellParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificImageGenParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificImageGenParam {
    #[doc = "The tool to call. Always `image_generation`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificImageGenParamType,
}
impl LiveSpecificImageGenParam {
    pub fn builder() -> builder::LiveSpecificImageGenParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `image_generation`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificImageGenParamType {
    #[serde(rename = "image_generation")]
    ImageGeneration,
}
impl ::std::fmt::Display for LiveSpecificImageGenParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGeneration => f.write_str("image_generation"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificImageGenParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation" => Ok(Self::ImageGeneration),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificImageGenParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificImageGenParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificProgrammaticToolCallingParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificProgrammaticToolCallingParam {
    #[doc = "The tool to call. Always `programmatic_tool_calling`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificProgrammaticToolCallingParamType,
}
impl LiveSpecificProgrammaticToolCallingParam {
    pub fn builder() -> builder::LiveSpecificProgrammaticToolCallingParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `programmatic_tool_calling`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificProgrammaticToolCallingParamType {
    #[serde(rename = "programmatic_tool_calling")]
    ProgrammaticToolCalling,
}
impl ::std::fmt::Display for LiveSpecificProgrammaticToolCallingParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgrammaticToolCalling => f.write_str("programmatic_tool_calling"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificProgrammaticToolCallingParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "programmatic_tool_calling" => Ok(Self::ProgrammaticToolCalling),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificProgrammaticToolCallingParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for LiveSpecificProgrammaticToolCallingParamType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificWebSearchParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificWebSearchParam {
    #[doc = "The tool to call. Always `web_search`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificWebSearchParamType,
}
impl LiveSpecificWebSearchParam {
    pub fn builder() -> builder::LiveSpecificWebSearchParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `web_search`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificWebSearchParamType {
    #[serde(rename = "web_search")]
    WebSearch,
}
impl ::std::fmt::Display for LiveSpecificWebSearchParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearch => f.write_str("web_search"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificWebSearchParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search" => Ok(Self::WebSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificWebSearchParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificWebSearchParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveSpecificWebSearchPreviewParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveSpecificWebSearchPreviewParam {
    #[doc = "The tool to call. Always `web_search_preview`."]
    #[serde(rename = "type")]
    pub type_: LiveSpecificWebSearchPreviewParamType,
}
impl LiveSpecificWebSearchPreviewParam {
    pub fn builder() -> builder::LiveSpecificWebSearchPreviewParam {
        Default::default()
    }
}
#[doc = "The tool to call. Always `web_search_preview`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveSpecificWebSearchPreviewParamType {
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
}
impl ::std::fmt::Display for LiveSpecificWebSearchPreviewParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearchPreview => f.write_str("web_search_preview"),
        }
    }
}
impl ::std::str::FromStr for LiveSpecificWebSearchPreviewParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search_preview" => Ok(Self::WebSearchPreview),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveSpecificWebSearchPreviewParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveSpecificWebSearchPreviewParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveTextVerbosity`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTextVerbosity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for LiveTextVerbosity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for LiveTextVerbosity {
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
impl ::std::convert::TryFrom<&str> for LiveTextVerbosity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTextVerbosity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Provide silent reasoning or progress context to the Live model, optionally for an existing client delegation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveThinkingAppendParam {
    #[doc = "Silent reasoning or progress context, limited to 500 tokens. It does not directly request speech, but can influence later speech and is not a secrecy boundary."]
    pub content: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub delegation_id: ::std::option::Option<LiveThinkingAppendParamDelegationId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub event_id: ::std::option::Option<LiveThinkingAppendParamEventId>,
    #[doc = "The Live client event type. Always `session.thinking.append`."]
    #[serde(rename = "type")]
    pub type_: LiveThinkingAppendParamType,
}
impl LiveThinkingAppendParam {
    pub fn builder() -> builder::LiveThinkingAppendParam {
        Default::default()
    }
}
#[doc = "Required, nullable. Set null for general session context, or use the ID from session.delegation.created for an existing client delegation. Non-null IDs are not accepted with Responses delegation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveThinkingAppendParamDelegationId(::std::string::String);
impl ::std::ops::Deref for LiveThinkingAppendParamDelegationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveThinkingAppendParamDelegationId> for ::std::string::String {
    fn from(value: LiveThinkingAppendParamDelegationId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveThinkingAppendParamDelegationId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveThinkingAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveThinkingAppendParamDelegationId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveThinkingAppendParamDelegationId {
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
#[doc = "Optional client identifier for correlating this command with a server event's client_event_id or error.client_event_id."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveThinkingAppendParamEventId(::std::string::String);
impl ::std::ops::Deref for LiveThinkingAppendParamEventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveThinkingAppendParamEventId> for ::std::string::String {
    fn from(value: LiveThinkingAppendParamEventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveThinkingAppendParamEventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveThinkingAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveThinkingAppendParamEventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveThinkingAppendParamEventId {
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
#[doc = "The Live client event type. Always `session.thinking.append`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveThinkingAppendParamType {
    #[serde(rename = "session.thinking.append")]
    SessionThinkingAppend,
}
impl ::std::fmt::Display for LiveThinkingAppendParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionThinkingAppend => f.write_str("session.thinking.append"),
        }
    }
}
impl ::std::str::FromStr for LiveThinkingAppendParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.thinking.append" => Ok(Self::SessionThinkingAppend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveThinkingAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveThinkingAppendParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returned when a session.thinking.append command is accepted into the Live session timeline. Acknowledges the added reasoning context without guaranteeing any spoken output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveThinkingAppended {
    #[doc = "The event_id of the client command associated with this server event, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    #[doc = "The end of this event on the Live session timeline, in milliseconds from the beginning of the session. For appended context, this can equal start_ms."]
    pub end_ms: i64,
    #[doc = "The unique ID of the Live server event."]
    pub event_id: ::std::string::String,
    #[doc = "The start of this event on the Live session timeline, in milliseconds from the beginning of the session."]
    pub start_ms: i64,
    #[doc = "The event type, always `session.thinking.appended`."]
    #[serde(rename = "type")]
    pub type_: LiveThinkingAppendedType,
}
impl LiveThinkingAppended {
    pub fn builder() -> builder::LiveThinkingAppended {
        Default::default()
    }
}
#[doc = "The event type, always `session.thinking.appended`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveThinkingAppendedType {
    #[serde(rename = "session.thinking.appended")]
    SessionThinkingAppended,
}
impl ::std::fmt::Display for LiveThinkingAppendedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionThinkingAppended => f.write_str("session.thinking.appended"),
        }
    }
}
impl ::std::str::FromStr for LiveThinkingAppendedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.thinking.appended" => Ok(Self::SessionThinkingAppended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveThinkingAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveThinkingAppendedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveToolChoiceEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveToolChoiceEnum {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for LiveToolChoiceEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::None => f.write_str("none"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for LiveToolChoiceEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveToolChoiceEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveToolChoiceEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveToolChoiceValueEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveToolChoiceValueEnum {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for LiveToolChoiceValueEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for LiveToolChoiceValueEnum {
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
impl ::std::convert::TryFrom<&str> for LiveToolChoiceValueEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveToolChoiceValueEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveToolSearchToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveToolSearchToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveToolSearchToolInputParamType,
}
impl LiveToolSearchToolInputParam {
    pub fn builder() -> builder::LiveToolSearchToolInputParam {
        Default::default()
    }
}
#[doc = "`LiveToolSearchToolInputParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveToolSearchToolInputParamType {
    #[serde(rename = "tool_search")]
    ToolSearch,
}
impl ::std::fmt::Display for LiveToolSearchToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearch => f.write_str("tool_search"),
        }
    }
}
impl ::std::str::FromStr for LiveToolSearchToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search" => Ok(Self::ToolSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveToolSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveToolSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The outbound SIP provider leg answered and media is established. Delivered only to sideband observers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportAnswered {
    pub event_id: ::std::string::String,
    #[doc = "The canonical Live session ID."]
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: LiveTransportAnsweredType,
}
impl LiveTransportAnswered {
    pub fn builder() -> builder::LiveTransportAnswered {
        Default::default()
    }
}
#[doc = "`LiveTransportAnsweredType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportAnsweredType {
    #[serde(rename = "transport.answered")]
    TransportAnswered,
}
impl ::std::fmt::Display for LiveTransportAnsweredType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportAnswered => f.write_str("transport.answered"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportAnsweredType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transport.answered" => Ok(Self::TransportAnswered),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportAnsweredType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportAnsweredType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiveTransportCallError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportCallError {
    #[doc = "The call setup failure code."]
    pub code: ::std::string::String,
    pub message: ::std::string::String,
    #[doc = "The parameter related to the error, if any. Empty when no parameter applies."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub param: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: LiveTransportCallErrorType,
}
impl LiveTransportCallError {
    pub fn builder() -> builder::LiveTransportCallError {
        Default::default()
    }
}
#[doc = "`LiveTransportCallErrorType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportCallErrorType {
    #[serde(rename = "call_error")]
    CallError,
}
impl ::std::fmt::Display for LiveTransportCallErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CallError => f.write_str("call_error"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportCallErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call_error" => Ok(Self::CallError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportCallErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportCallErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A SIP DTMF keypress received from the caller. Delivered only to sideband observers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportDtmfReceived {
    pub event: LiveTransportDtmfReceivedEvent,
    pub event_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: LiveTransportDtmfReceivedType,
}
impl LiveTransportDtmfReceived {
    pub fn builder() -> builder::LiveTransportDtmfReceived {
        Default::default()
    }
}
#[doc = "`LiveTransportDtmfReceivedEvent`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveTransportDtmfReceivedEvent(::std::string::String);
impl ::std::ops::Deref for LiveTransportDtmfReceivedEvent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveTransportDtmfReceivedEvent> for ::std::string::String {
    fn from(value: LiveTransportDtmfReceivedEvent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveTransportDtmfReceivedEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1usize {
            return Err("longer than 1 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9A-D*#]$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-D*#]$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportDtmfReceivedEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportDtmfReceivedEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveTransportDtmfReceivedEvent {
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
#[doc = "`LiveTransportDtmfReceivedType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportDtmfReceivedType {
    #[serde(rename = "transport.dtmf.received")]
    TransportDtmfReceived,
}
impl ::std::fmt::Display for LiveTransportDtmfReceivedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportDtmfReceived => f.write_str("transport.dtmf.received"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportDtmfReceivedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transport.dtmf.received" => Ok(Self::TransportDtmfReceived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportDtmfReceivedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportDtmfReceivedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A SIP DTMF keypress successfully sent to the SIP trunk. Delivered only to sideband observers; this is not a client command."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportDtmfSend {
    #[doc = "The event_id of the client command, when supplied."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_event_id: ::std::option::Option<::std::string::String>,
    pub event: LiveTransportDtmfSendEvent,
    pub event_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: LiveTransportDtmfSendType,
}
impl LiveTransportDtmfSend {
    pub fn builder() -> builder::LiveTransportDtmfSend {
        Default::default()
    }
}
#[doc = "`LiveTransportDtmfSendEvent`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveTransportDtmfSendEvent(::std::string::String);
impl ::std::ops::Deref for LiveTransportDtmfSendEvent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveTransportDtmfSendEvent> for ::std::string::String {
    fn from(value: LiveTransportDtmfSendEvent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveTransportDtmfSendEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1usize {
            return Err("longer than 1 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9A-D*#]$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-D*#]$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportDtmfSendEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportDtmfSendEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveTransportDtmfSendEvent {
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
#[doc = "`LiveTransportDtmfSendType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportDtmfSendType {
    #[serde(rename = "transport.dtmf.send")]
    TransportDtmfSend,
}
impl ::std::fmt::Display for LiveTransportDtmfSendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportDtmfSend => f.write_str("transport.dtmf.send"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportDtmfSendType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transport.dtmf.send" => Ok(Self::TransportDtmfSend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportDtmfSendType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportDtmfSendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An asynchronous outbound SIP setup failure. Delivered only to sideband observers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportFailed {
    pub error: LiveTransportCallError,
    pub event_id: ::std::string::String,
    #[doc = "The canonical Live session ID."]
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: LiveTransportFailedType,
}
impl LiveTransportFailed {
    pub fn builder() -> builder::LiveTransportFailed {
        Default::default()
    }
}
#[doc = "`LiveTransportFailedType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportFailedType {
    #[serde(rename = "transport.failed")]
    TransportFailed,
}
impl ::std::fmt::Display for LiveTransportFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportFailed => f.write_str("transport.failed"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportFailedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transport.failed" => Ok(Self::TransportFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportFailedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The outbound SIP provider leg is ringing or providing early media. Delivered only to sideband observers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveTransportRinging {
    pub event_id: ::std::string::String,
    #[doc = "The canonical Live session ID."]
    pub session_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: LiveTransportRingingType,
}
impl LiveTransportRinging {
    pub fn builder() -> builder::LiveTransportRinging {
        Default::default()
    }
}
#[doc = "`LiveTransportRingingType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveTransportRingingType {
    #[serde(rename = "transport.ringing")]
    TransportRinging,
}
impl ::std::fmt::Display for LiveTransportRingingType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportRinging => f.write_str("transport.ringing"),
        }
    }
}
impl ::std::str::FromStr for LiveTransportRingingType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transport.ringing" => Ok(Self::TransportRinging),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveTransportRingingType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveTransportRingingType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "WebRTC transport carrying the offer SDP in a creation request or answer SDP in its response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveWebRtcTransport {
    #[doc = "Session Description Protocol message for the WebRTC connection."]
    pub sdp: LiveWebRtcTransportSdp,
    #[doc = "The transport used for the Live session. Always `webrtc`."]
    #[serde(rename = "type")]
    pub type_: LiveWebRtcTransportType,
}
impl LiveWebRtcTransport {
    pub fn builder() -> builder::LiveWebRtcTransport {
        Default::default()
    }
}
#[doc = "Session Description Protocol message for the WebRTC connection."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LiveWebRtcTransportSdp(::std::string::String);
impl ::std::ops::Deref for LiveWebRtcTransportSdp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiveWebRtcTransportSdp> for ::std::string::String {
    fn from(value: LiveWebRtcTransportSdp) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LiveWebRtcTransportSdp {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LiveWebRtcTransportSdp {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveWebRtcTransportSdp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LiveWebRtcTransportSdp {
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
#[doc = "The transport used for the Live session. Always `webrtc`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveWebRtcTransportType {
    #[serde(rename = "webrtc")]
    Webrtc,
}
impl ::std::fmt::Display for LiveWebRtcTransportType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Webrtc => f.write_str("webrtc"),
        }
    }
}
impl ::std::str::FromStr for LiveWebRtcTransportType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "webrtc" => Ok(Self::Webrtc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveWebRtcTransportType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveWebRtcTransportType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A web search tool available to the Live session’s Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveWebSearchToolInputParam {
    #[doc = "The tool type. Always `web_search`."]
    #[serde(rename = "type")]
    pub type_: LiveWebSearchToolInputParamType,
}
impl LiveWebSearchToolInputParam {
    pub fn builder() -> builder::LiveWebSearchToolInputParam {
        Default::default()
    }
}
#[doc = "The tool type. Always `web_search`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LiveWebSearchToolInputParamType {
    #[serde(rename = "web_search")]
    WebSearch,
}
impl ::std::fmt::Display for LiveWebSearchToolInputParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearch => f.write_str("web_search"),
        }
    }
}
impl ::std::str::FromStr for LiveWebSearchToolInputParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search" => Ok(Self::WebSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiveWebSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiveWebSearchToolInputParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ModelIdsLive`"]
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
pub struct ModelIdsLive(pub ::std::string::String);
impl ::std::ops::Deref for ModelIdsLive {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelIdsLive> for ::std::string::String {
    fn from(value: ModelIdsLive) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ModelIdsLive {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ModelIdsLive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ModelIdsLive {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "Whether to store the forked session. Omission inherits the stored session's setting."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PropertiesStore(pub bool);
impl ::std::ops::Deref for PropertiesStore {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<PropertiesStore> for bool {
    fn from(value: PropertiesStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for PropertiesStore {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for PropertiesStore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for PropertiesStore {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PropertiesStore {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PropertiesStore {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
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
#[doc = "`ReferLiveSessionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ReferLiveSessionParams {
    pub session_id: ::std::string::String,
}
impl ReferLiveSessionParams {
    pub fn builder() -> builder::ReferLiveSessionParams {
        Default::default()
    }
}
#[doc = "`RejectLiveSessionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RejectLiveSessionParams {
    pub session_id: ::std::string::String,
}
impl RejectLiveSessionParams {
    pub fn builder() -> builder::RejectLiveSessionParams {
        Default::default()
    }
}
#[doc = "The newly created Live session. Use its ID for session controls and sideband connections."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Session {
    #[doc = "Opaque session identifier. Preserve the returned value unchanged, including its prefix."]
    pub id: ::std::string::String,
}
impl Session {
    pub fn builder() -> builder::Session {
        Default::default()
    }
}
#[doc = "Whether to store the session for later forking and recording download. Defaults to false for new sessions."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Store(pub bool);
impl ::std::ops::Deref for Store {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<Store> for bool {
    fn from(value: Store) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for Store {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for Store {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for Store {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Store {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Store {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
#[doc = "URI that should appear in the SIP Refer-To header. Supports values like\n`tel:+14155550123` or `sip:agent@example.com`."]
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
pub struct TargetUri(pub ::std::string::String);
impl ::std::ops::Deref for TargetUri {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TargetUri> for ::std::string::String {
    fn from(value: TargetUri) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for TargetUri {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for TargetUri {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for TargetUri {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
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
    pub struct AcceptLiveSessionParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AcceptLiveSessionParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl AcceptLiveSessionParams {
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
    impl ::std::convert::TryFrom<AcceptLiveSessionParams> for super::AcceptLiveSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AcceptLiveSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::AcceptLiveSessionParams> for AcceptLiveSessionParams {
        fn from(value: super::AcceptLiveSessionParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DownloadLiveRecordingParams {
        session_id: ::std::result::Result<
            super::DownloadLiveRecordingParamsSessionId,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DownloadLiveRecordingParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl DownloadLiveRecordingParams {
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DownloadLiveRecordingParamsSessionId>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DownloadLiveRecordingParams> for super::DownloadLiveRecordingParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DownloadLiveRecordingParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::DownloadLiveRecordingParams> for DownloadLiveRecordingParams {
        fn from(value: super::DownloadLiveRecordingParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ForkLiveSessionParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ForkLiveSessionParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl ForkLiveSessionParams {
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
    impl ::std::convert::TryFrom<ForkLiveSessionParams> for super::ForkLiveSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ForkLiveSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::ForkLiveSessionParams> for ForkLiveSessionParams {
        fn from(value: super::ForkLiveSessionParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HangupLiveSessionParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for HangupLiveSessionParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl HangupLiveSessionParams {
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
    impl ::std::convert::TryFrom<HangupLiveSessionParams> for super::HangupLiveSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HangupLiveSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::HangupLiveSessionParams> for HangupLiveSessionParams {
        fn from(value: super::HangupLiveSessionParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveAllowedServerEventParam {
        response_event: ::std::result::Result<
            ::std::option::Option<super::LiveAllowedServerEventParamResponseEvent>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveAllowedServerEventParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveAllowedServerEventParam {
        fn default() -> Self {
            Self {
                response_event: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveAllowedServerEventParam {
        pub fn response_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveAllowedServerEventParamResponseEvent>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_event: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveAllowedServerEventParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveAllowedServerEventParam> for super::LiveAllowedServerEventParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveAllowedServerEventParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                response_event: value.response_event?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveAllowedServerEventParam> for LiveAllowedServerEventParam {
        fn from(value: super::LiveAllowedServerEventParam) -> Self {
            Self {
                response_event: Ok(value.response_event),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveAllowedToolsChoiceParam {
        mode: ::std::result::Result<
            ::std::option::Option<super::LiveToolChoiceValueEnum>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::LiveAllowedToolsChoiceParamToolsItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveAllowedToolsChoiceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveAllowedToolsChoiceParam {
        fn default() -> Self {
            Self {
                mode: Ok(Default::default()),
                tools: Err("no value supplied for tools".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveAllowedToolsChoiceParam {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveToolChoiceValueEnum>>,
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
                ::std::vec::Vec<super::LiveAllowedToolsChoiceParamToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveAllowedToolsChoiceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveAllowedToolsChoiceParam> for super::LiveAllowedToolsChoiceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveAllowedToolsChoiceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                tools: value.tools?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveAllowedToolsChoiceParam> for LiveAllowedToolsChoiceParam {
        fn from(value: super::LiveAllowedToolsChoiceParam) -> Self {
            Self {
                mode: Ok(value.mode),
                tools: Ok(value.tools),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveApiSchema {
        accept_live_session_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        accept_live_session_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        accept_live_session_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        accept_live_session_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        accept_live_session_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        accept_live_session_params:
            ::std::result::Result<super::AcceptLiveSessionParams, ::std::string::String>,
        accept_live_session_request:
            ::std::result::Result<super::LiveCallAcceptRequest, ::std::string::String>,
        create_live_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_live_error413:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_live_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_live_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_live_error504:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_live_request:
            ::std::result::Result<super::LiveSessionCreateRequest, ::std::string::String>,
        create_live_response:
            ::std::result::Result<super::LiveSessionCreateResponse, ::std::string::String>,
        download_live_recording_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_live_recording_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_live_recording_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_live_recording_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_live_recording_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        download_live_recording_params:
            ::std::result::Result<super::DownloadLiveRecordingParams, ::std::string::String>,
        fork_live_session_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        fork_live_session_error413:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        fork_live_session_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        fork_live_session_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        fork_live_session_params:
            ::std::result::Result<super::ForkLiveSessionParams, ::std::string::String>,
        fork_live_session_request:
            ::std::result::Result<super::LiveForkRequest, ::std::string::String>,
        fork_live_session_response:
            ::std::result::Result<super::LiveCreateResponse, ::std::string::String>,
        hangup_live_session_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        hangup_live_session_params:
            ::std::result::Result<super::HangupLiveSessionParams, ::std::string::String>,
        live_client_event: ::std::result::Result<super::LiveClientEvent, ::std::string::String>,
        live_fork_client_event:
            ::std::result::Result<super::LiveForkClientEvent, ::std::string::String>,
        live_fork_server_event:
            ::std::result::Result<super::LiveForkServerEvent, ::std::string::String>,
        live_server_event: ::std::result::Result<super::LiveServerEvent, ::std::string::String>,
        live_sideband_client_event:
            ::std::result::Result<super::LiveSidebandClientEvent, ::std::string::String>,
        live_sideband_server_event:
            ::std::result::Result<super::LiveSidebandServerEvent, ::std::string::String>,
        refer_live_session_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        refer_live_session_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        refer_live_session_params:
            ::std::result::Result<super::ReferLiveSessionParams, ::std::string::String>,
        refer_live_session_request:
            ::std::result::Result<super::LiveCallReferRequest, ::std::string::String>,
        reject_live_session_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        reject_live_session_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        reject_live_session_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        reject_live_session_params:
            ::std::result::Result<super::RejectLiveSessionParams, ::std::string::String>,
        reject_live_session_request:
            ::std::result::Result<super::LiveCallRejectRequest, ::std::string::String>,
    }
    impl ::std::default::Default for LiveApiSchema {
        fn default() -> Self {
            Self {
                accept_live_session_error400: Err(
                    "no value supplied for accept_live_session_error400".to_string(),
                ),
                accept_live_session_error404: Err(
                    "no value supplied for accept_live_session_error404".to_string(),
                ),
                accept_live_session_error409: Err(
                    "no value supplied for accept_live_session_error409".to_string(),
                ),
                accept_live_session_error429: Err(
                    "no value supplied for accept_live_session_error429".to_string(),
                ),
                accept_live_session_error503: Err(
                    "no value supplied for accept_live_session_error503".to_string(),
                ),
                accept_live_session_params: Err(
                    "no value supplied for accept_live_session_params".to_string()
                ),
                accept_live_session_request: Err(
                    "no value supplied for accept_live_session_request".to_string(),
                ),
                create_live_error400: Err("no value supplied for create_live_error400".to_string()),
                create_live_error413: Err("no value supplied for create_live_error413".to_string()),
                create_live_error429: Err("no value supplied for create_live_error429".to_string()),
                create_live_error503: Err("no value supplied for create_live_error503".to_string()),
                create_live_error504: Err("no value supplied for create_live_error504".to_string()),
                create_live_request: Err("no value supplied for create_live_request".to_string()),
                create_live_response: Err("no value supplied for create_live_response".to_string()),
                download_live_recording_error400: Err(
                    "no value supplied for download_live_recording_error400".to_string(),
                ),
                download_live_recording_error404: Err(
                    "no value supplied for download_live_recording_error404".to_string(),
                ),
                download_live_recording_error429: Err(
                    "no value supplied for download_live_recording_error429".to_string(),
                ),
                download_live_recording_error500: Err(
                    "no value supplied for download_live_recording_error500".to_string(),
                ),
                download_live_recording_error503: Err(
                    "no value supplied for download_live_recording_error503".to_string(),
                ),
                download_live_recording_params: Err(
                    "no value supplied for download_live_recording_params".to_string(),
                ),
                fork_live_session_error400: Err(
                    "no value supplied for fork_live_session_error400".to_string()
                ),
                fork_live_session_error413: Err(
                    "no value supplied for fork_live_session_error413".to_string()
                ),
                fork_live_session_error429: Err(
                    "no value supplied for fork_live_session_error429".to_string()
                ),
                fork_live_session_error503: Err(
                    "no value supplied for fork_live_session_error503".to_string()
                ),
                fork_live_session_params: Err(
                    "no value supplied for fork_live_session_params".to_string()
                ),
                fork_live_session_request: Err(
                    "no value supplied for fork_live_session_request".to_string()
                ),
                fork_live_session_response: Err(
                    "no value supplied for fork_live_session_response".to_string()
                ),
                hangup_live_session_error404: Err(
                    "no value supplied for hangup_live_session_error404".to_string(),
                ),
                hangup_live_session_params: Err(
                    "no value supplied for hangup_live_session_params".to_string()
                ),
                live_client_event: Err("no value supplied for live_client_event".to_string()),
                live_fork_client_event: Err(
                    "no value supplied for live_fork_client_event".to_string()
                ),
                live_fork_server_event: Err(
                    "no value supplied for live_fork_server_event".to_string()
                ),
                live_server_event: Err("no value supplied for live_server_event".to_string()),
                live_sideband_client_event: Err(
                    "no value supplied for live_sideband_client_event".to_string()
                ),
                live_sideband_server_event: Err(
                    "no value supplied for live_sideband_server_event".to_string()
                ),
                refer_live_session_error400: Err(
                    "no value supplied for refer_live_session_error400".to_string(),
                ),
                refer_live_session_error404: Err(
                    "no value supplied for refer_live_session_error404".to_string(),
                ),
                refer_live_session_params: Err(
                    "no value supplied for refer_live_session_params".to_string()
                ),
                refer_live_session_request: Err(
                    "no value supplied for refer_live_session_request".to_string()
                ),
                reject_live_session_error400: Err(
                    "no value supplied for reject_live_session_error400".to_string(),
                ),
                reject_live_session_error404: Err(
                    "no value supplied for reject_live_session_error404".to_string(),
                ),
                reject_live_session_error409: Err(
                    "no value supplied for reject_live_session_error409".to_string(),
                ),
                reject_live_session_params: Err(
                    "no value supplied for reject_live_session_params".to_string()
                ),
                reject_live_session_request: Err(
                    "no value supplied for reject_live_session_request".to_string(),
                ),
            }
        }
    }
    impl LiveApiSchema {
        pub fn accept_live_session_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_error400: {e}")
            });
            self
        }
        pub fn accept_live_session_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_error404: {e}")
            });
            self
        }
        pub fn accept_live_session_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_error409: {e}")
            });
            self
        }
        pub fn accept_live_session_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_error429: {e}")
            });
            self
        }
        pub fn accept_live_session_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_error503: {e}")
            });
            self
        }
        pub fn accept_live_session_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AcceptLiveSessionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_params: {e}")
            });
            self
        }
        pub fn accept_live_session_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallAcceptRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.accept_live_session_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for accept_live_session_request: {e}")
            });
            self
        }
        pub fn create_live_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_error400: {e}")
            });
            self
        }
        pub fn create_live_error413<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_error413 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_error413: {e}")
            });
            self
        }
        pub fn create_live_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_error429: {e}")
            });
            self
        }
        pub fn create_live_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_error503: {e}")
            });
            self
        }
        pub fn create_live_error504<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_error504 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_error504: {e}")
            });
            self
        }
        pub fn create_live_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCreateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_request: {e}")
            });
            self
        }
        pub fn create_live_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCreateResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_live_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_live_response: {e}")
            });
            self
        }
        pub fn download_live_recording_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_error400: {e}")
            });
            self
        }
        pub fn download_live_recording_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_error404: {e}")
            });
            self
        }
        pub fn download_live_recording_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_error429: {e}")
            });
            self
        }
        pub fn download_live_recording_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_error500: {e}")
            });
            self
        }
        pub fn download_live_recording_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_error503: {e}")
            });
            self
        }
        pub fn download_live_recording_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DownloadLiveRecordingParams>,
            T::Error: ::std::fmt::Display,
        {
            self.download_live_recording_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for download_live_recording_params: {e}")
            });
            self
        }
        pub fn fork_live_session_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_error400: {e}")
            });
            self
        }
        pub fn fork_live_session_error413<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_error413 = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_error413: {e}")
            });
            self
        }
        pub fn fork_live_session_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_error429: {e}")
            });
            self
        }
        pub fn fork_live_session_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_error503: {e}")
            });
            self
        }
        pub fn fork_live_session_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ForkLiveSessionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_params: {e}")
            });
            self
        }
        pub fn fork_live_session_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveForkRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_request: {e}")
            });
            self
        }
        pub fn fork_live_session_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCreateResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.fork_live_session_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for fork_live_session_response: {e}")
            });
            self
        }
        pub fn hangup_live_session_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.hangup_live_session_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for hangup_live_session_error404: {e}")
            });
            self
        }
        pub fn hangup_live_session_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HangupLiveSessionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.hangup_live_session_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for hangup_live_session_params: {e}")
            });
            self
        }
        pub fn live_client_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveClientEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_client_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for live_client_event: {e}"));
            self
        }
        pub fn live_fork_client_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveForkClientEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_fork_client_event = value.try_into().map_err(|e| {
                format!("error converting supplied value for live_fork_client_event: {e}")
            });
            self
        }
        pub fn live_fork_server_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveForkServerEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_fork_server_event = value.try_into().map_err(|e| {
                format!("error converting supplied value for live_fork_server_event: {e}")
            });
            self
        }
        pub fn live_server_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveServerEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_server_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for live_server_event: {e}"));
            self
        }
        pub fn live_sideband_client_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSidebandClientEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_sideband_client_event = value.try_into().map_err(|e| {
                format!("error converting supplied value for live_sideband_client_event: {e}")
            });
            self
        }
        pub fn live_sideband_server_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSidebandServerEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.live_sideband_server_event = value.try_into().map_err(|e| {
                format!("error converting supplied value for live_sideband_server_event: {e}")
            });
            self
        }
        pub fn refer_live_session_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.refer_live_session_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for refer_live_session_error400: {e}")
            });
            self
        }
        pub fn refer_live_session_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.refer_live_session_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for refer_live_session_error404: {e}")
            });
            self
        }
        pub fn refer_live_session_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReferLiveSessionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.refer_live_session_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for refer_live_session_params: {e}")
            });
            self
        }
        pub fn refer_live_session_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallReferRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.refer_live_session_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for refer_live_session_request: {e}")
            });
            self
        }
        pub fn reject_live_session_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.reject_live_session_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for reject_live_session_error400: {e}")
            });
            self
        }
        pub fn reject_live_session_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.reject_live_session_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for reject_live_session_error404: {e}")
            });
            self
        }
        pub fn reject_live_session_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.reject_live_session_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for reject_live_session_error409: {e}")
            });
            self
        }
        pub fn reject_live_session_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RejectLiveSessionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.reject_live_session_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for reject_live_session_params: {e}")
            });
            self
        }
        pub fn reject_live_session_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallRejectRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.reject_live_session_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for reject_live_session_request: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<LiveApiSchema> for super::LiveApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accept_live_session_error400: value.accept_live_session_error400?,
                accept_live_session_error404: value.accept_live_session_error404?,
                accept_live_session_error409: value.accept_live_session_error409?,
                accept_live_session_error429: value.accept_live_session_error429?,
                accept_live_session_error503: value.accept_live_session_error503?,
                accept_live_session_params: value.accept_live_session_params?,
                accept_live_session_request: value.accept_live_session_request?,
                create_live_error400: value.create_live_error400?,
                create_live_error413: value.create_live_error413?,
                create_live_error429: value.create_live_error429?,
                create_live_error503: value.create_live_error503?,
                create_live_error504: value.create_live_error504?,
                create_live_request: value.create_live_request?,
                create_live_response: value.create_live_response?,
                download_live_recording_error400: value.download_live_recording_error400?,
                download_live_recording_error404: value.download_live_recording_error404?,
                download_live_recording_error429: value.download_live_recording_error429?,
                download_live_recording_error500: value.download_live_recording_error500?,
                download_live_recording_error503: value.download_live_recording_error503?,
                download_live_recording_params: value.download_live_recording_params?,
                fork_live_session_error400: value.fork_live_session_error400?,
                fork_live_session_error413: value.fork_live_session_error413?,
                fork_live_session_error429: value.fork_live_session_error429?,
                fork_live_session_error503: value.fork_live_session_error503?,
                fork_live_session_params: value.fork_live_session_params?,
                fork_live_session_request: value.fork_live_session_request?,
                fork_live_session_response: value.fork_live_session_response?,
                hangup_live_session_error404: value.hangup_live_session_error404?,
                hangup_live_session_params: value.hangup_live_session_params?,
                live_client_event: value.live_client_event?,
                live_fork_client_event: value.live_fork_client_event?,
                live_fork_server_event: value.live_fork_server_event?,
                live_server_event: value.live_server_event?,
                live_sideband_client_event: value.live_sideband_client_event?,
                live_sideband_server_event: value.live_sideband_server_event?,
                refer_live_session_error400: value.refer_live_session_error400?,
                refer_live_session_error404: value.refer_live_session_error404?,
                refer_live_session_params: value.refer_live_session_params?,
                refer_live_session_request: value.refer_live_session_request?,
                reject_live_session_error400: value.reject_live_session_error400?,
                reject_live_session_error404: value.reject_live_session_error404?,
                reject_live_session_error409: value.reject_live_session_error409?,
                reject_live_session_params: value.reject_live_session_params?,
                reject_live_session_request: value.reject_live_session_request?,
            })
        }
    }
    impl ::std::convert::From<super::LiveApiSchema> for LiveApiSchema {
        fn from(value: super::LiveApiSchema) -> Self {
            Self {
                accept_live_session_error400: Ok(value.accept_live_session_error400),
                accept_live_session_error404: Ok(value.accept_live_session_error404),
                accept_live_session_error409: Ok(value.accept_live_session_error409),
                accept_live_session_error429: Ok(value.accept_live_session_error429),
                accept_live_session_error503: Ok(value.accept_live_session_error503),
                accept_live_session_params: Ok(value.accept_live_session_params),
                accept_live_session_request: Ok(value.accept_live_session_request),
                create_live_error400: Ok(value.create_live_error400),
                create_live_error413: Ok(value.create_live_error413),
                create_live_error429: Ok(value.create_live_error429),
                create_live_error503: Ok(value.create_live_error503),
                create_live_error504: Ok(value.create_live_error504),
                create_live_request: Ok(value.create_live_request),
                create_live_response: Ok(value.create_live_response),
                download_live_recording_error400: Ok(value.download_live_recording_error400),
                download_live_recording_error404: Ok(value.download_live_recording_error404),
                download_live_recording_error429: Ok(value.download_live_recording_error429),
                download_live_recording_error500: Ok(value.download_live_recording_error500),
                download_live_recording_error503: Ok(value.download_live_recording_error503),
                download_live_recording_params: Ok(value.download_live_recording_params),
                fork_live_session_error400: Ok(value.fork_live_session_error400),
                fork_live_session_error413: Ok(value.fork_live_session_error413),
                fork_live_session_error429: Ok(value.fork_live_session_error429),
                fork_live_session_error503: Ok(value.fork_live_session_error503),
                fork_live_session_params: Ok(value.fork_live_session_params),
                fork_live_session_request: Ok(value.fork_live_session_request),
                fork_live_session_response: Ok(value.fork_live_session_response),
                hangup_live_session_error404: Ok(value.hangup_live_session_error404),
                hangup_live_session_params: Ok(value.hangup_live_session_params),
                live_client_event: Ok(value.live_client_event),
                live_fork_client_event: Ok(value.live_fork_client_event),
                live_fork_server_event: Ok(value.live_fork_server_event),
                live_server_event: Ok(value.live_server_event),
                live_sideband_client_event: Ok(value.live_sideband_client_event),
                live_sideband_server_event: Ok(value.live_sideband_server_event),
                refer_live_session_error400: Ok(value.refer_live_session_error400),
                refer_live_session_error404: Ok(value.refer_live_session_error404),
                refer_live_session_params: Ok(value.refer_live_session_params),
                refer_live_session_request: Ok(value.refer_live_session_request),
                reject_live_session_error400: Ok(value.reject_live_session_error400),
                reject_live_session_error404: Ok(value.reject_live_session_error404),
                reject_live_session_error409: Ok(value.reject_live_session_error409),
                reject_live_session_params: Ok(value.reject_live_session_params),
                reject_live_session_request: Ok(value.reject_live_session_request),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveApplyPatchToolInputParam {
        type_:
            ::std::result::Result<super::LiveApplyPatchToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveApplyPatchToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveApplyPatchToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveApplyPatchToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveApplyPatchToolInputParam> for super::LiveApplyPatchToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveApplyPatchToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveApplyPatchToolInputParam> for LiveApplyPatchToolInputParam {
        fn from(value: super::LiveApplyPatchToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCallAcceptRequest {
        session: ::std::result::Result<super::LiveCallAcceptSession, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCallAcceptRequest {
        fn default() -> Self {
            Self {
                session: Err("no value supplied for session".to_string()),
            }
        }
    }
    impl LiveCallAcceptRequest {
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallAcceptSession>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCallAcceptRequest> for super::LiveCallAcceptRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCallAcceptRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session: value.session?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCallAcceptRequest> for LiveCallAcceptRequest {
        fn from(value: super::LiveCallAcceptRequest) -> Self {
            Self {
                session: Ok(value.session),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCallAcceptSession {
        audio: ::std::result::Result<
            ::std::option::Option<super::LiveMediaSessionAudioParam>,
            ::std::string::String,
        >,
        delegation:
            ::std::result::Result<::std::option::Option<super::Delegation>, ::std::string::String>,
        input: ::std::result::Result<::std::option::Option<super::Input>, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<super::Instructions>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::LiveCallAcceptSessionModel, ::std::string::String>,
        store: ::std::result::Result<::std::option::Option<super::Store>, ::std::string::String>,
        type_: ::std::result::Result<super::LiveCallAcceptSessionType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCallAcceptSession {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                delegation: Ok(Default::default()),
                input: Ok(Default::default()),
                instructions: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                store: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveCallAcceptSession {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveMediaSessionAudioParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Delegation>>,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Input>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Instructions>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallAcceptSessionModel>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn store<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Store>>,
            T::Error: ::std::fmt::Display,
        {
            self.store = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for store: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallAcceptSessionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCallAcceptSession> for super::LiveCallAcceptSession {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCallAcceptSession,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                delegation: value.delegation?,
                input: value.input?,
                instructions: value.instructions?,
                model: value.model?,
                store: value.store?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCallAcceptSession> for LiveCallAcceptSession {
        fn from(value: super::LiveCallAcceptSession) -> Self {
            Self {
                audio: Ok(value.audio),
                delegation: Ok(value.delegation),
                input: Ok(value.input),
                instructions: Ok(value.instructions),
                model: Ok(value.model),
                store: Ok(value.store),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCallReferRequest {
        target_uri:
            ::std::result::Result<super::LiveCallReferRequestTargetUri, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCallReferRequest {
        fn default() -> Self {
            Self {
                target_uri: Err("no value supplied for target_uri".to_string()),
            }
        }
    }
    impl LiveCallReferRequest {
        pub fn target_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCallReferRequestTargetUri>,
            T::Error: ::std::fmt::Display,
        {
            self.target_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCallReferRequest> for super::LiveCallReferRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCallReferRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                target_uri: value.target_uri?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCallReferRequest> for LiveCallReferRequest {
        fn from(value: super::LiveCallReferRequest) -> Self {
            Self {
                target_uri: Ok(value.target_uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCallRejectRequest {
        status_code: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCallRejectRequest {
        fn default() -> Self {
            Self {
                status_code: Err("no value supplied for status_code".to_string()),
            }
        }
    }
    impl LiveCallRejectRequest {
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_code: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCallRejectRequest> for super::LiveCallRejectRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCallRejectRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                status_code: value.status_code?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCallRejectRequest> for LiveCallRejectRequest {
        fn from(value: super::LiveCallRejectRequest) -> Self {
            Self {
                status_code: Ok(value.status_code),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveClientConfigParam {
        data_channel:
            ::std::result::Result<super::LiveDataChannelConfigParam, ::std::string::String>,
    }
    impl ::std::default::Default for LiveClientConfigParam {
        fn default() -> Self {
            Self {
                data_channel: Err("no value supplied for data_channel".to_string()),
            }
        }
    }
    impl LiveClientConfigParam {
        pub fn data_channel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveDataChannelConfigParam>,
            T::Error: ::std::fmt::Display,
        {
            self.data_channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_channel: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveClientConfigParam> for super::LiveClientConfigParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveClientConfigParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data_channel: value.data_channel?,
            })
        }
    }
    impl ::std::convert::From<super::LiveClientConfigParam> for LiveClientConfigParam {
        fn from(value: super::LiveClientConfigParam) -> Self {
            Self {
                data_channel: Ok(value.data_channel),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveClientDelegationParam {
        type_: ::std::result::Result<super::LiveClientDelegationParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveClientDelegationParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveClientDelegationParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveClientDelegationParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveClientDelegationParam> for super::LiveClientDelegationParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveClientDelegationParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveClientDelegationParam> for LiveClientDelegationParam {
        fn from(value: super::LiveClientDelegationParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCodeInterpreterToolInputParam {
        type_: ::std::result::Result<
            super::LiveCodeInterpreterToolInputParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveCodeInterpreterToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveCodeInterpreterToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCodeInterpreterToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCodeInterpreterToolInputParam>
        for super::LiveCodeInterpreterToolInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCodeInterpreterToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCodeInterpreterToolInputParam>
        for LiveCodeInterpreterToolInputParam
    {
        fn from(value: super::LiveCodeInterpreterToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCommentaryAppendParam {
        content: ::std::result::Result<::std::string::String, ::std::string::String>,
        delegation_id: ::std::result::Result<
            ::std::option::Option<super::LiveCommentaryAppendParamDelegationId>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveCommentaryAppendParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveCommentaryAppendParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCommentaryAppendParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                delegation_id: Err("no value supplied for delegation_id".to_string()),
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveCommentaryAppendParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn delegation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveCommentaryAppendParamDelegationId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveCommentaryAppendParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCommentaryAppendParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCommentaryAppendParam> for super::LiveCommentaryAppendParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCommentaryAppendParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                delegation_id: value.delegation_id?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCommentaryAppendParam> for LiveCommentaryAppendParam {
        fn from(value: super::LiveCommentaryAppendParam) -> Self {
            Self {
                content: Ok(value.content),
                delegation_id: Ok(value.delegation_id),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCommentaryAppended {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_ms: ::std::result::Result<i64, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveCommentaryAppendedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCommentaryAppended {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                end_ms: Err("no value supplied for end_ms".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                start_ms: Err("no value supplied for start_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveCommentaryAppended {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCommentaryAppendedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCommentaryAppended> for super::LiveCommentaryAppended {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCommentaryAppended,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                end_ms: value.end_ms?,
                event_id: value.event_id?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCommentaryAppended> for LiveCommentaryAppended {
        fn from(value: super::LiveCommentaryAppended) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                end_ms: Ok(value.end_ms),
                event_id: Ok(value.event_id),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveComputerToolInputParam {
        type_: ::std::result::Result<super::LiveComputerToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveComputerToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveComputerToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveComputerToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveComputerToolInputParam> for super::LiveComputerToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveComputerToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveComputerToolInputParam> for LiveComputerToolInputParam {
        fn from(value: super::LiveComputerToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveContainerNetworkPolicyDisabledParam {
        type_: ::std::result::Result<
            super::LiveContainerNetworkPolicyDisabledParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveContainerNetworkPolicyDisabledParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveContainerNetworkPolicyDisabledParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveContainerNetworkPolicyDisabledParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveContainerNetworkPolicyDisabledParam>
        for super::LiveContainerNetworkPolicyDisabledParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveContainerNetworkPolicyDisabledParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveContainerNetworkPolicyDisabledParam>
        for LiveContainerNetworkPolicyDisabledParam
    {
        fn from(value: super::LiveContainerNetworkPolicyDisabledParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveContainerReferenceParam {
        container_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveContainerReferenceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveContainerReferenceParam {
        fn default() -> Self {
            Self {
                container_id: Err("no value supplied for container_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveContainerReferenceParam {
        pub fn container_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.container_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for container_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveContainerReferenceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveContainerReferenceParam> for super::LiveContainerReferenceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveContainerReferenceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                container_id: value.container_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveContainerReferenceParam> for LiveContainerReferenceParam {
        fn from(value: super::LiveContainerReferenceParam) -> Self {
            Self {
                container_id: Ok(value.container_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveContextWindowUsage {
        usage_ratio: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for LiveContextWindowUsage {
        fn default() -> Self {
            Self {
                usage_ratio: Err("no value supplied for usage_ratio".to_string()),
            }
        }
    }
    impl LiveContextWindowUsage {
        pub fn usage_ratio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_ratio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage_ratio: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveContextWindowUsage> for super::LiveContextWindowUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveContextWindowUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                usage_ratio: value.usage_ratio?,
            })
        }
    }
    impl ::std::convert::From<super::LiveContextWindowUsage> for LiveContextWindowUsage {
        fn from(value: super::LiveContextWindowUsage) -> Self {
            Self {
                usage_ratio: Ok(value.usage_ratio),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCreateResponse {
        session: ::std::result::Result<super::LiveCreateResponseSession, ::std::string::String>,
        transport: ::std::result::Result<super::LiveWebRtcTransport, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCreateResponse {
        fn default() -> Self {
            Self {
                session: Err("no value supplied for session".to_string()),
                transport: Err("no value supplied for transport".to_string()),
            }
        }
    }
    impl LiveCreateResponse {
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCreateResponseSession>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveWebRtcTransport>,
            T::Error: ::std::fmt::Display,
        {
            self.transport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCreateResponse> for super::LiveCreateResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCreateResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session: value.session?,
                transport: value.transport?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCreateResponse> for LiveCreateResponse {
        fn from(value: super::LiveCreateResponse) -> Self {
            Self {
                session: Ok(value.session),
                transport: Ok(value.transport),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCreateResponseSession {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCreateResponseSession {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl LiveCreateResponseSession {
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
    impl ::std::convert::TryFrom<LiveCreateResponseSession> for super::LiveCreateResponseSession {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCreateResponseSession,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::LiveCreateResponseSession> for LiveCreateResponseSession {
        fn from(value: super::LiveCreateResponseSession) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCustomToolInputParam {
        type_: ::std::result::Result<super::LiveCustomToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCustomToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveCustomToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCustomToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCustomToolInputParam> for super::LiveCustomToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCustomToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveCustomToolInputParam> for LiveCustomToolInputParam {
        fn from(value: super::LiveCustomToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveCustomVoiceParam {
        id: ::std::result::Result<super::LiveCustomVoiceParamId, ::std::string::String>,
    }
    impl ::std::default::Default for LiveCustomVoiceParam {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl LiveCustomVoiceParam {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveCustomVoiceParamId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveCustomVoiceParam> for super::LiveCustomVoiceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveCustomVoiceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::LiveCustomVoiceParam> for LiveCustomVoiceParam {
        fn from(value: super::LiveCustomVoiceParam) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveDataChannelConfigParam {
        allowed_client_events: ::std::result::Result<
            ::std::option::Option<super::LiveDataChannelConfigParamAllowedClientEvents>,
            ::std::string::String,
        >,
        allowed_server_events: ::std::result::Result<
            ::std::option::Option<super::LiveDataChannelConfigParamAllowedServerEvents>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveDataChannelConfigParam {
        fn default() -> Self {
            Self {
                allowed_client_events: Ok(Default::default()),
                allowed_server_events: Ok(Default::default()),
            }
        }
    }
    impl LiveDataChannelConfigParam {
        pub fn allowed_client_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveDataChannelConfigParamAllowedClientEvents>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_client_events = value.try_into().map_err(|e| {
                format!("error converting supplied value for allowed_client_events: {e}")
            });
            self
        }
        pub fn allowed_server_events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveDataChannelConfigParamAllowedServerEvents>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_server_events = value.try_into().map_err(|e| {
                format!("error converting supplied value for allowed_server_events: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<LiveDataChannelConfigParam> for super::LiveDataChannelConfigParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveDataChannelConfigParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_client_events: value.allowed_client_events?,
                allowed_server_events: value.allowed_server_events?,
            })
        }
    }
    impl ::std::convert::From<super::LiveDataChannelConfigParam> for LiveDataChannelConfigParam {
        fn from(value: super::LiveDataChannelConfigParam) -> Self {
            Self {
                allowed_client_events: Ok(value.allowed_client_events),
                allowed_server_events: Ok(value.allowed_server_events),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveDelegationCreated {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        delegation: ::std::result::Result<super::LiveDelegationItem, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        offset_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveDelegationCreatedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveDelegationCreated {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                delegation: Err("no value supplied for delegation".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                offset_ms: Err("no value supplied for offset_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveDelegationCreated {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveDelegationItem>,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn offset_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.offset_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for offset_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveDelegationCreatedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveDelegationCreated> for super::LiveDelegationCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveDelegationCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                delegation: value.delegation?,
                event_id: value.event_id?,
                offset_ms: value.offset_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveDelegationCreated> for LiveDelegationCreated {
        fn from(value: super::LiveDelegationCreated) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                delegation: Ok(value.delegation),
                event_id: Ok(value.event_id),
                offset_ms: Ok(value.offset_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveDelegationItem {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        response_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        target: ::std::result::Result<super::LiveDelegationItemTarget, ::std::string::String>,
        type_: ::std::result::Result<super::LiveDelegationItemType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveDelegationItem {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                response_id: Ok(Default::default()),
                target: Err("no value supplied for target".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveDelegationItem {
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
        pub fn response_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_id: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveDelegationItemTarget>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveDelegationItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveDelegationItem> for super::LiveDelegationItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveDelegationItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                response_id: value.response_id?,
                target: value.target?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveDelegationItem> for LiveDelegationItem {
        fn from(value: super::LiveDelegationItem) -> Self {
            Self {
                id: Ok(value.id),
                response_id: Ok(value.response_id),
                target: Ok(value.target),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveDelegationReasoningInputParam {
        effort: ::std::result::Result<
            ::std::option::Option<super::LiveReasoningEffort>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<
            ::std::option::Option<super::LiveReasoningSummary>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveDelegationReasoningInputParam {
        fn default() -> Self {
            Self {
                effort: Ok(Default::default()),
                summary: Ok(Default::default()),
            }
        }
    }
    impl LiveDelegationReasoningInputParam {
        pub fn effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveReasoningEffort>>,
            T::Error: ::std::fmt::Display,
        {
            self.effort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effort: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveReasoningSummary>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveDelegationReasoningInputParam>
        for super::LiveDelegationReasoningInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveDelegationReasoningInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                effort: value.effort?,
                summary: value.summary?,
            })
        }
    }
    impl ::std::convert::From<super::LiveDelegationReasoningInputParam>
        for LiveDelegationReasoningInputParam
    {
        fn from(value: super::LiveDelegationReasoningInputParam) -> Self {
            Self {
                effort: Ok(value.effort),
                summary: Ok(value.summary),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveDelegationTextInputParam {
        verbosity: ::std::result::Result<
            ::std::option::Option<super::LiveTextVerbosity>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveDelegationTextInputParam {
        fn default() -> Self {
            Self {
                verbosity: Ok(Default::default()),
            }
        }
    }
    impl LiveDelegationTextInputParam {
        pub fn verbosity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveTextVerbosity>>,
            T::Error: ::std::fmt::Display,
        {
            self.verbosity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for verbosity: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveDelegationTextInputParam> for super::LiveDelegationTextInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveDelegationTextInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                verbosity: value.verbosity?,
            })
        }
    }
    impl ::std::convert::From<super::LiveDelegationTextInputParam> for LiveDelegationTextInputParam {
        fn from(value: super::LiveDelegationTextInputParam) -> Self {
            Self {
                verbosity: Ok(value.verbosity),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveErrorEvent {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error: ::std::result::Result<super::LiveLiveError, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveErrorEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveErrorEvent {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                error: Err("no value supplied for error".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveErrorEvent {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveLiveError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveErrorEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveErrorEvent> for super::LiveErrorEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveErrorEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                error: value.error?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveErrorEvent> for LiveErrorEvent {
        fn from(value: super::LiveErrorEvent) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                error: Ok(value.error),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveFileSearchToolInputParam {
        type_:
            ::std::result::Result<super::LiveFileSearchToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveFileSearchToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveFileSearchToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveFileSearchToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveFileSearchToolInputParam> for super::LiveFileSearchToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveFileSearchToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveFileSearchToolInputParam> for LiveFileSearchToolInputParam {
        fn from(value: super::LiveFileSearchToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveForkAudioParam {
        format: ::std::result::Result<
            ::std::option::Option<super::LiveAudioFormat>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveForkAudioParam {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
            }
        }
    }
    impl LiveForkAudioParam {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveAudioFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveForkAudioParam> for super::LiveForkAudioParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveForkAudioParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
            })
        }
    }
    impl ::std::convert::From<super::LiveForkAudioParam> for LiveForkAudioParam {
        fn from(value: super::LiveForkAudioParam) -> Self {
            Self {
                format: Ok(value.format),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveForkRequest {
        session: ::std::result::Result<
            ::std::option::Option<super::LiveMediaSessionForkParams>,
            ::std::string::String,
        >,
        transport: ::std::result::Result<super::LiveWebRtcTransport, ::std::string::String>,
    }
    impl ::std::default::Default for LiveForkRequest {
        fn default() -> Self {
            Self {
                session: Ok(Default::default()),
                transport: Err("no value supplied for transport".to_string()),
            }
        }
    }
    impl LiveForkRequest {
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveMediaSessionForkParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveWebRtcTransport>,
            T::Error: ::std::fmt::Display,
        {
            self.transport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveForkRequest> for super::LiveForkRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveForkRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session: value.session?,
                transport: value.transport?,
            })
        }
    }
    impl ::std::convert::From<super::LiveForkRequest> for LiveForkRequest {
        fn from(value: super::LiveForkRequest) -> Self {
            Self {
                session: Ok(value.session),
                transport: Ok(value.transport),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveForkSessionConfigParam {
        audio: ::std::result::Result<
            ::std::option::Option<super::LiveForkAudioParam>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<super::LiveClientConfigParam>,
            ::std::string::String,
        >,
        delegation: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesDelegationUpdateParam>,
            ::std::string::String,
        >,
        store: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for LiveForkSessionConfigParam {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                client: Ok(Default::default()),
                delegation: Ok(Default::default()),
                store: Ok(Default::default()),
            }
        }
    }
    impl LiveForkSessionConfigParam {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveForkAudioParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveClientConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponsesDelegationUpdateParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
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
    }
    impl ::std::convert::TryFrom<LiveForkSessionConfigParam> for super::LiveForkSessionConfigParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveForkSessionConfigParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                client: value.client?,
                delegation: value.delegation?,
                store: value.store?,
            })
        }
    }
    impl ::std::convert::From<super::LiveForkSessionConfigParam> for LiveForkSessionConfigParam {
        fn from(value: super::LiveForkSessionConfigParam) -> Self {
            Self {
                audio: Ok(value.audio),
                client: Ok(value.client),
                delegation: Ok(value.delegation),
                store: Ok(value.store),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveForkSessionStartEvent {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveForkSessionStartEventEventId>,
            ::std::string::String,
        >,
        session: ::std::result::Result<super::LiveForkSessionConfigParam, ::std::string::String>,
        type_: ::std::result::Result<super::LiveForkSessionStartEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveForkSessionStartEvent {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveForkSessionStartEvent {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveForkSessionStartEventEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveForkSessionConfigParam>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveForkSessionStartEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveForkSessionStartEvent> for super::LiveForkSessionStartEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveForkSessionStartEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveForkSessionStartEvent> for LiveForkSessionStartEvent {
        fn from(value: super::LiveForkSessionStartEvent) -> Self {
            Self {
                event_id: Ok(value.event_id),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveFunctionToolChoiceParam {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveFunctionToolChoiceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveFunctionToolChoiceParam {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveFunctionToolChoiceParam {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveFunctionToolChoiceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveFunctionToolChoiceParam> for super::LiveFunctionToolChoiceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveFunctionToolChoiceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveFunctionToolChoiceParam> for LiveFunctionToolChoiceParam {
        fn from(value: super::LiveFunctionToolChoiceParam) -> Self {
            Self {
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveFunctionToolInputParam {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parameters: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        strict: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_: ::std::result::Result<super::LiveFunctionToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveFunctionToolInputParam {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                parameters: Ok(Default::default()),
                strict: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveFunctionToolInputParam {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveFunctionToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveFunctionToolInputParam> for super::LiveFunctionToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveFunctionToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                parameters: value.parameters?,
                strict: value.strict?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveFunctionToolInputParam> for LiveFunctionToolInputParam {
        fn from(value: super::LiveFunctionToolInputParam) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                parameters: Ok(value.parameters),
                strict: Ok(value.strict),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveHostedShellContainerAutoParam {
        file_ids: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        memory_limit: ::std::result::Result<
            ::std::option::Option<super::LiveContainerMemoryLimit>,
            ::std::string::String,
        >,
        network_policy: ::std::result::Result<
            ::std::option::Option<super::LiveHostedShellContainerAutoParamNetworkPolicy>,
            ::std::string::String,
        >,
        skills: ::std::result::Result<
            ::std::option::Option<
                ::std::vec::Vec<super::LiveHostedShellContainerAutoParamSkillsItem>,
            >,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::LiveHostedShellContainerAutoParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveHostedShellContainerAutoParam {
        fn default() -> Self {
            Self {
                file_ids: Ok(Default::default()),
                memory_limit: Ok(Default::default()),
                network_policy: Ok(Default::default()),
                skills: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveHostedShellContainerAutoParam {
        pub fn file_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.file_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_ids: {e}"));
            self
        }
        pub fn memory_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveContainerMemoryLimit>>,
            T::Error: ::std::fmt::Display,
        {
            self.memory_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memory_limit: {e}"));
            self
        }
        pub fn network_policy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveHostedShellContainerAutoParamNetworkPolicy>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.network_policy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_policy: {e}"));
            self
        }
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::vec::Vec<super::LiveHostedShellContainerAutoParamSkillsItem>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveHostedShellContainerAutoParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveHostedShellContainerAutoParam>
        for super::LiveHostedShellContainerAutoParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveHostedShellContainerAutoParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_ids: value.file_ids?,
                memory_limit: value.memory_limit?,
                network_policy: value.network_policy?,
                skills: value.skills?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveHostedShellContainerAutoParam>
        for LiveHostedShellContainerAutoParam
    {
        fn from(value: super::LiveHostedShellContainerAutoParam) -> Self {
            Self {
                file_ids: Ok(value.file_ids),
                memory_limit: Ok(value.memory_limit),
                network_policy: Ok(value.network_policy),
                skills: Ok(value.skills),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveHostedShellNetworkPolicyAllowlistParam {
        allowed_domains:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<
            super::LiveHostedShellNetworkPolicyAllowlistParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveHostedShellNetworkPolicyAllowlistParam {
        fn default() -> Self {
            Self {
                allowed_domains: Err("no value supplied for allowed_domains".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveHostedShellNetworkPolicyAllowlistParam {
        pub fn allowed_domains<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_domains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_domains: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveHostedShellNetworkPolicyAllowlistParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveHostedShellNetworkPolicyAllowlistParam>
        for super::LiveHostedShellNetworkPolicyAllowlistParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveHostedShellNetworkPolicyAllowlistParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_domains: value.allowed_domains?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveHostedShellNetworkPolicyAllowlistParam>
        for LiveHostedShellNetworkPolicyAllowlistParam
    {
        fn from(value: super::LiveHostedShellNetworkPolicyAllowlistParam) -> Self {
            Self {
                allowed_domains: Ok(value.allowed_domains),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveHostedShellToolInputParam {
        environment: ::std::result::Result<
            ::std::option::Option<super::LiveHostedShellToolInputParamEnvironment>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::LiveHostedShellToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveHostedShellToolInputParam {
        fn default() -> Self {
            Self {
                environment: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveHostedShellToolInputParam {
        pub fn environment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveHostedShellToolInputParamEnvironment>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.environment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for environment: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveHostedShellToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveHostedShellToolInputParam>
        for super::LiveHostedShellToolInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveHostedShellToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                environment: value.environment?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveHostedShellToolInputParam> for LiveHostedShellToolInputParam {
        fn from(value: super::LiveHostedShellToolInputParam) -> Self {
            Self {
                environment: Ok(value.environment),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveImageGenerationToolInputParam {
        type_: ::std::result::Result<
            super::LiveImageGenerationToolInputParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveImageGenerationToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveImageGenerationToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveImageGenerationToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveImageGenerationToolInputParam>
        for super::LiveImageGenerationToolInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveImageGenerationToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveImageGenerationToolInputParam>
        for LiveImageGenerationToolInputParam
    {
        fn from(value: super::LiveImageGenerationToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInfoEvent {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInfoEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInfoEvent {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                code: Err("no value supplied for code".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                message: Err("no value supplied for message".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInfoEvent {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
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
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
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
            T: ::std::convert::TryInto<super::LiveInfoEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInfoEvent> for super::LiveInfoEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInfoEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                code: value.code?,
                event_id: value.event_id?,
                message: value.message?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInfoEvent> for LiveInfoEvent {
        fn from(value: super::LiveInfoEvent) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                code: Ok(value.code),
                event_id: Ok(value.event_id),
                message: Ok(value.message),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialAssistantMessageItemParam {
        content: ::std::result::Result<
            [super::LiveInitialAssistantMessageItemParamContentItem; 1usize],
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::LiveInitialAssistantMessageItemParamRole,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::LiveInitialMessageStatus>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiveInitialAssistantMessageItemParamType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialAssistantMessageItemParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                status: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialAssistantMessageItemParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                [super::LiveInitialAssistantMessageItemParamContentItem; 1usize],
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInitialAssistantMessageItemParamRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveInitialMessageStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialAssistantMessageItemParamType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialAssistantMessageItemParam>
        for super::LiveInitialAssistantMessageItemParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialAssistantMessageItemParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
                role: value.role?,
                status: value.status?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialAssistantMessageItemParam>
        for LiveInitialAssistantMessageItemParam
    {
        fn from(value: super::LiveInitialAssistantMessageItemParam) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
                role: Ok(value.role),
                status: Ok(value.status),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialDeveloperMessageItemParam {
        content: ::std::result::Result<
            [super::LiveInitialInputTextContentPartParam; 1usize],
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::LiveInitialDeveloperMessageItemParamRole,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::LiveInitialMessageStatus>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiveInitialDeveloperMessageItemParamType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialDeveloperMessageItemParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                status: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialDeveloperMessageItemParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[super::LiveInitialInputTextContentPartParam; 1usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInitialDeveloperMessageItemParamRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveInitialMessageStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialDeveloperMessageItemParamType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialDeveloperMessageItemParam>
        for super::LiveInitialDeveloperMessageItemParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialDeveloperMessageItemParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
                role: value.role?,
                status: value.status?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialDeveloperMessageItemParam>
        for LiveInitialDeveloperMessageItemParam
    {
        fn from(value: super::LiveInitialDeveloperMessageItemParam) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
                role: Ok(value.role),
                status: Ok(value.status),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialInputTextContentPartParam {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiveInitialInputTextContentPartParamType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialInputTextContentPartParam {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialInputTextContentPartParam {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialInputTextContentPartParamType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialInputTextContentPartParam>
        for super::LiveInitialInputTextContentPartParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialInputTextContentPartParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialInputTextContentPartParam>
        for LiveInitialInputTextContentPartParam
    {
        fn from(value: super::LiveInitialInputTextContentPartParam) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialOutputTextContentPartParam {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::LiveInitialOutputTextContentPartParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialOutputTextContentPartParam {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInitialOutputTextContentPartParam {
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
            T: ::std::convert::TryInto<super::LiveInitialOutputTextContentPartParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialOutputTextContentPartParam>
        for super::LiveInitialOutputTextContentPartParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialOutputTextContentPartParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialOutputTextContentPartParam>
        for LiveInitialOutputTextContentPartParam
    {
        fn from(value: super::LiveInitialOutputTextContentPartParam) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialSessionAudioOutputParam {
        voice: ::std::result::Result<
            ::std::option::Option<super::LiveInitialSessionAudioOutputParamVoice>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialSessionAudioOutputParam {
        fn default() -> Self {
            Self {
                voice: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialSessionAudioOutputParam {
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialSessionAudioOutputParamVoice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialSessionAudioOutputParam>
        for super::LiveInitialSessionAudioOutputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialSessionAudioOutputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                voice: value.voice?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialSessionAudioOutputParam>
        for LiveInitialSessionAudioOutputParam
    {
        fn from(value: super::LiveInitialSessionAudioOutputParam) -> Self {
            Self {
                voice: Ok(value.voice),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialSessionAudioParam {
        format: ::std::result::Result<
            ::std::option::Option<super::LiveAudioFormat>,
            ::std::string::String,
        >,
        output: ::std::result::Result<
            ::std::option::Option<super::LiveInitialSessionAudioOutputParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialSessionAudioParam {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialSessionAudioParam {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveAudioFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialSessionAudioOutputParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialSessionAudioParam> for super::LiveInitialSessionAudioParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialSessionAudioParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialSessionAudioParam> for LiveInitialSessionAudioParam {
        fn from(value: super::LiveInitialSessionAudioParam) -> Self {
            Self {
                format: Ok(value.format),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialTextContentPartParam {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiveInitialTextContentPartParamType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialTextContentPartParam {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialTextContentPartParam {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialTextContentPartParamType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialTextContentPartParam>
        for super::LiveInitialTextContentPartParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialTextContentPartParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialTextContentPartParam>
        for LiveInitialTextContentPartParam
    {
        fn from(value: super::LiveInitialTextContentPartParam) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInitialUserMessageItemParam {
        content: ::std::result::Result<
            [super::LiveInitialInputTextContentPartParam; 1usize],
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::LiveInitialUserMessageItemParamRole,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::LiveInitialMessageStatus>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiveInitialUserMessageItemParamType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveInitialUserMessageItemParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                status: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LiveInitialUserMessageItemParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[super::LiveInitialInputTextContentPartParam; 1usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInitialUserMessageItemParamRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveInitialMessageStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialUserMessageItemParamType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInitialUserMessageItemParam>
        for super::LiveInitialUserMessageItemParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInitialUserMessageItemParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
                role: value.role?,
                status: value.status?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInitialUserMessageItemParam>
        for LiveInitialUserMessageItemParam
    {
        fn from(value: super::LiveInitialUserMessageItemParam) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
                role: Ok(value.role),
                status: Ok(value.status),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInlineSkillParam {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::LiveInlineSkillSourceParam, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInlineSkillParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInlineSkillParam {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInlineSkillParam {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInlineSkillSourceParam>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInlineSkillParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInlineSkillParam> for super::LiveInlineSkillParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInlineSkillParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInlineSkillParam> for LiveInlineSkillParam {
        fn from(value: super::LiveInlineSkillParam) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInlineSkillSourceParam {
        data: ::std::result::Result<super::LiveInlineSkillSourceParamData, ::std::string::String>,
        media_type: ::std::result::Result<
            super::LiveInlineSkillSourceParamMediaType,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveInlineSkillSourceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInlineSkillSourceParam {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                media_type: Err("no value supplied for media_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInlineSkillSourceParam {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInlineSkillSourceParamData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInlineSkillSourceParamMediaType>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInlineSkillSourceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInlineSkillSourceParam> for super::LiveInlineSkillSourceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInlineSkillSourceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                media_type: value.media_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInlineSkillSourceParam> for LiveInlineSkillSourceParam {
        fn from(value: super::LiveInlineSkillSourceParam) -> Self {
            Self {
                data: Ok(value.data),
                media_type: Ok(value.media_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioAppend {
        audio: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInputAudioAppendType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioAppend {
        fn default() -> Self {
            Self {
                audio: Err("no value supplied for audio".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioAppend {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioAppendType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioAppend> for super::LiveInputAudioAppend {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioAppend,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioAppend> for LiveInputAudioAppend {
        fn from(value: super::LiveInputAudioAppend) -> Self {
            Self {
                audio: Ok(value.audio),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioAppendEvent {
        audio: ::std::result::Result<super::LiveInputAudioAppendEventAudio, ::std::string::String>,
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveInputAudioAppendEventEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveInputAudioAppendEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioAppendEvent {
        fn default() -> Self {
            Self {
                audio: Err("no value supplied for audio".to_string()),
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioAppendEvent {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioAppendEventAudio>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInputAudioAppendEventEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioAppendEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioAppendEvent> for super::LiveInputAudioAppendEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioAppendEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioAppendEvent> for LiveInputAudioAppendEvent {
        fn from(value: super::LiveInputAudioAppendEvent) -> Self {
            Self {
                audio: Ok(value.audio),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioMuteParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveInputAudioMuteParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveInputAudioMuteParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioMuteParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioMuteParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInputAudioMuteParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioMuteParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioMuteParam> for super::LiveInputAudioMuteParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioMuteParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioMuteParam> for LiveInputAudioMuteParam {
        fn from(value: super::LiveInputAudioMuteParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioMuted {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInputAudioMutedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioMuted {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioMuted {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioMutedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioMuted> for super::LiveInputAudioMuted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioMuted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioMuted> for LiveInputAudioMuted {
        fn from(value: super::LiveInputAudioMuted) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioUnmuteParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveInputAudioUnmuteParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveInputAudioUnmuteParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioUnmuteParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioUnmuteParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInputAudioUnmuteParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioUnmuteParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioUnmuteParam> for super::LiveInputAudioUnmuteParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioUnmuteParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioUnmuteParam> for LiveInputAudioUnmuteParam {
        fn from(value: super::LiveInputAudioUnmuteParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputAudioUnmuted {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInputAudioUnmutedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputAudioUnmuted {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputAudioUnmuted {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputAudioUnmutedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputAudioUnmuted> for super::LiveInputAudioUnmuted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputAudioUnmuted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputAudioUnmuted> for LiveInputAudioUnmuted {
        fn from(value: super::LiveInputAudioUnmuted) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInputTranscriptDelta {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        delta: ::std::result::Result<::std::string::String, ::std::string::String>,
        end_ms: ::std::result::Result<i64, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInputTranscriptDeltaType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInputTranscriptDelta {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                delta: Err("no value supplied for delta".to_string()),
                end_ms: Err("no value supplied for end_ms".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                start_ms: Err("no value supplied for start_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInputTranscriptDelta {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInputTranscriptDeltaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInputTranscriptDelta> for super::LiveInputTranscriptDelta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInputTranscriptDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                delta: value.delta?,
                end_ms: value.end_ms?,
                event_id: value.event_id?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInputTranscriptDelta> for LiveInputTranscriptDelta {
        fn from(value: super::LiveInputTranscriptDelta) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                delta: Ok(value.delta),
                end_ms: Ok(value.end_ms),
                event_id: Ok(value.event_id),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInstructionsAppendParam {
        content: ::std::result::Result<::std::string::String, ::std::string::String>,
        delegation_id: ::std::result::Result<
            ::std::option::Option<super::LiveInstructionsAppendParamDelegationId>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveInstructionsAppendParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveInstructionsAppendParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInstructionsAppendParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                delegation_id: Err("no value supplied for delegation_id".to_string()),
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInstructionsAppendParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn delegation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInstructionsAppendParamDelegationId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInstructionsAppendParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInstructionsAppendParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInstructionsAppendParam> for super::LiveInstructionsAppendParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInstructionsAppendParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                delegation_id: value.delegation_id?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInstructionsAppendParam> for LiveInstructionsAppendParam {
        fn from(value: super::LiveInstructionsAppendParam) -> Self {
            Self {
                content: Ok(value.content),
                delegation_id: Ok(value.delegation_id),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveInstructionsAppended {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_ms: ::std::result::Result<i64, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveInstructionsAppendedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveInstructionsAppended {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                end_ms: Err("no value supplied for end_ms".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                start_ms: Err("no value supplied for start_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveInstructionsAppended {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveInstructionsAppendedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveInstructionsAppended> for super::LiveInstructionsAppended {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveInstructionsAppended,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                end_ms: value.end_ms?,
                event_id: value.event_id?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveInstructionsAppended> for LiveInstructionsAppended {
        fn from(value: super::LiveInstructionsAppended) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                end_ms: Ok(value.end_ms),
                event_id: Ok(value.event_id),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveLiveError {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        param: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LiveLiveError {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
                param: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveLiveError {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveLiveError> for super::LiveLiveError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveLiveError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                code: value.code?,
                message: value.message?,
                param: value.param?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveLiveError> for LiveLiveError {
        fn from(value: super::LiveLiveError) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                code: Ok(value.code),
                message: Ok(value.message),
                param: Ok(value.param),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveLocalEnvironmentParam {
        skills: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::LiveLocalSkillParam>>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveLocalEnvironmentParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveLocalEnvironmentParam {
        fn default() -> Self {
            Self {
                skills: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveLocalEnvironmentParam {
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::LiveLocalSkillParam>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveLocalEnvironmentParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveLocalEnvironmentParam> for super::LiveLocalEnvironmentParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveLocalEnvironmentParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skills: value.skills?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveLocalEnvironmentParam> for LiveLocalEnvironmentParam {
        fn from(value: super::LiveLocalEnvironmentParam) -> Self {
            Self {
                skills: Ok(value.skills),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveLocalSkillParam {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LiveLocalSkillParam {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                name: Err("no value supplied for name".to_string()),
                path: Err("no value supplied for path".to_string()),
            }
        }
    }
    impl LiveLocalSkillParam {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveLocalSkillParam> for super::LiveLocalSkillParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveLocalSkillParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                path: value.path?,
            })
        }
    }
    impl ::std::convert::From<super::LiveLocalSkillParam> for LiveLocalSkillParam {
        fn from(value: super::LiveLocalSkillParam) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                path: Ok(value.path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveMcpToolChoiceParam {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        server_label: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveMcpToolChoiceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveMcpToolChoiceParam {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                server_label: Err("no value supplied for server_label".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveMcpToolChoiceParam {
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
        pub fn server_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.server_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for server_label: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveMcpToolChoiceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveMcpToolChoiceParam> for super::LiveMcpToolChoiceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveMcpToolChoiceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                server_label: value.server_label?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveMcpToolChoiceParam> for LiveMcpToolChoiceParam {
        fn from(value: super::LiveMcpToolChoiceParam) -> Self {
            Self {
                name: Ok(value.name),
                server_label: Ok(value.server_label),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveMcpToolInputParam {
        type_: ::std::result::Result<super::LiveMcpToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveMcpToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveMcpToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveMcpToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveMcpToolInputParam> for super::LiveMcpToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveMcpToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveMcpToolInputParam> for LiveMcpToolInputParam {
        fn from(value: super::LiveMcpToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveMediaSessionAudioParam {
        output: ::std::result::Result<
            ::std::option::Option<super::LiveInitialSessionAudioOutputParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveMediaSessionAudioParam {
        fn default() -> Self {
            Self {
                output: Ok(Default::default()),
            }
        }
    }
    impl LiveMediaSessionAudioParam {
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveInitialSessionAudioOutputParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveMediaSessionAudioParam> for super::LiveMediaSessionAudioParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveMediaSessionAudioParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::LiveMediaSessionAudioParam> for LiveMediaSessionAudioParam {
        fn from(value: super::LiveMediaSessionAudioParam) -> Self {
            Self {
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveMediaSessionCreateParams {
        audio: ::std::result::Result<
            ::std::option::Option<super::LiveMediaSessionAudioParam>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<super::LiveClientConfigParam>,
            ::std::string::String,
        >,
        delegation:
            ::std::result::Result<::std::option::Option<super::Delegation>, ::std::string::String>,
        input: ::std::result::Result<::std::option::Option<super::Input>, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<super::Instructions>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::ModelIdsLive, ::std::string::String>,
        store: ::std::result::Result<::std::option::Option<super::Store>, ::std::string::String>,
    }
    impl ::std::default::Default for LiveMediaSessionCreateParams {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                client: Ok(Default::default()),
                delegation: Ok(Default::default()),
                input: Ok(Default::default()),
                instructions: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                store: Ok(Default::default()),
            }
        }
    }
    impl LiveMediaSessionCreateParams {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveMediaSessionAudioParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveClientConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Delegation>>,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Input>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Instructions>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModelIdsLive>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn store<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Store>>,
            T::Error: ::std::fmt::Display,
        {
            self.store = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for store: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveMediaSessionCreateParams> for super::LiveMediaSessionCreateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveMediaSessionCreateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                client: value.client?,
                delegation: value.delegation?,
                input: value.input?,
                instructions: value.instructions?,
                model: value.model?,
                store: value.store?,
            })
        }
    }
    impl ::std::convert::From<super::LiveMediaSessionCreateParams> for LiveMediaSessionCreateParams {
        fn from(value: super::LiveMediaSessionCreateParams) -> Self {
            Self {
                audio: Ok(value.audio),
                client: Ok(value.client),
                delegation: Ok(value.delegation),
                input: Ok(value.input),
                instructions: Ok(value.instructions),
                model: Ok(value.model),
                store: Ok(value.store),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveMediaSessionForkParams {
        client: ::std::result::Result<
            ::std::option::Option<super::LiveClientConfigParam>,
            ::std::string::String,
        >,
        delegation: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesDelegationUpdateParam>,
            ::std::string::String,
        >,
        store: ::std::result::Result<
            ::std::option::Option<super::PropertiesStore>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveMediaSessionForkParams {
        fn default() -> Self {
            Self {
                client: Ok(Default::default()),
                delegation: Ok(Default::default()),
                store: Ok(Default::default()),
            }
        }
    }
    impl LiveMediaSessionForkParams {
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveClientConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponsesDelegationUpdateParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
        pub fn store<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PropertiesStore>>,
            T::Error: ::std::fmt::Display,
        {
            self.store = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for store: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveMediaSessionForkParams> for super::LiveMediaSessionForkParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveMediaSessionForkParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client: value.client?,
                delegation: value.delegation?,
                store: value.store?,
            })
        }
    }
    impl ::std::convert::From<super::LiveMediaSessionForkParams> for LiveMediaSessionForkParams {
        fn from(value: super::LiveMediaSessionForkParams) -> Self {
            Self {
                client: Ok(value.client),
                delegation: Ok(value.delegation),
                store: Ok(value.store),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveNamespaceToolInputParam {
        type_: ::std::result::Result<super::LiveNamespaceToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveNamespaceToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveNamespaceToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveNamespaceToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveNamespaceToolInputParam> for super::LiveNamespaceToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveNamespaceToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveNamespaceToolInputParam> for LiveNamespaceToolInputParam {
        fn from(value: super::LiveNamespaceToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveOutputAudioDelta {
        delta: ::std::result::Result<::std::string::String, ::std::string::String>,
        end_ms: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        start_ms: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        type_: ::std::result::Result<super::LiveOutputAudioDeltaType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveOutputAudioDelta {
        fn default() -> Self {
            Self {
                delta: Err("no value supplied for delta".to_string()),
                end_ms: Ok(Default::default()),
                start_ms: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveOutputAudioDelta {
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveOutputAudioDeltaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveOutputAudioDelta> for super::LiveOutputAudioDelta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveOutputAudioDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delta: value.delta?,
                end_ms: value.end_ms?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveOutputAudioDelta> for LiveOutputAudioDelta {
        fn from(value: super::LiveOutputAudioDelta) -> Self {
            Self {
                delta: Ok(value.delta),
                end_ms: Ok(value.end_ms),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveOutputTranscriptDelta {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        delta: ::std::result::Result<::std::string::String, ::std::string::String>,
        end_ms: ::std::result::Result<i64, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveOutputTranscriptDeltaType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveOutputTranscriptDelta {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                delta: Err("no value supplied for delta".to_string()),
                end_ms: Err("no value supplied for end_ms".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                start_ms: Err("no value supplied for start_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveOutputTranscriptDelta {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delta: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveOutputTranscriptDeltaType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveOutputTranscriptDelta> for super::LiveOutputTranscriptDelta {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveOutputTranscriptDelta,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                delta: value.delta?,
                end_ms: value.end_ms?,
                event_id: value.event_id?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveOutputTranscriptDelta> for LiveOutputTranscriptDelta {
        fn from(value: super::LiveOutputTranscriptDelta) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                delta: Ok(value.delta),
                end_ms: Ok(value.end_ms),
                event_id: Ok(value.event_id),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveProgrammaticToolInputParam {
        type_:
            ::std::result::Result<super::LiveProgrammaticToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveProgrammaticToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveProgrammaticToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveProgrammaticToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveProgrammaticToolInputParam>
        for super::LiveProgrammaticToolInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveProgrammaticToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveProgrammaticToolInputParam>
        for LiveProgrammaticToolInputParam
    {
        fn from(value: super::LiveProgrammaticToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponseCreateParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveResponseCreateParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveResponseCreateParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveResponseCreateParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveResponseCreateParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponseCreateParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponseCreateParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponseCreateParam> for super::LiveResponseCreateParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponseCreateParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponseCreateParam> for LiveResponseCreateParam {
        fn from(value: super::LiveResponseCreateParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponseEvent {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        delegation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveResponseEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveResponseEvent {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                delegation_id: Ok(Default::default()),
                event: Err("no value supplied for event".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveResponseEvent {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn delegation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.delegation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation_id: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponseEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponseEvent> for super::LiveResponseEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponseEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                delegation_id: value.delegation_id?,
                event: value.event?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponseEvent> for LiveResponseEvent {
        fn from(value: super::LiveResponseEvent) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                delegation_id: Ok(value.delegation_id),
                event: Ok(value.event),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponseItemCreateParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveResponseItemCreateParamEventId>,
            ::std::string::String,
        >,
        item: ::std::result::Result<::openai_api_types::common::InputItem, ::std::string::String>,
        type_: ::std::result::Result<super::LiveResponseItemCreateParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveResponseItemCreateParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                item: Err("no value supplied for item".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveResponseItemCreateParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponseItemCreateParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn item<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::InputItem>,
            T::Error: ::std::fmt::Display,
        {
            self.item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponseItemCreateParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponseItemCreateParam> for super::LiveResponseItemCreateParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponseItemCreateParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                item: value.item?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponseItemCreateParam> for LiveResponseItemCreateParam {
        fn from(value: super::LiveResponseItemCreateParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                item: Ok(value.item),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponsesDelegationParam {
        responses: ::std::result::Result<
            super::LiveResponsesDelegationSettingsInputParam,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::LiveResponsesDelegationParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveResponsesDelegationParam {
        fn default() -> Self {
            Self {
                responses: Err("no value supplied for responses".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveResponsesDelegationParam {
        pub fn responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponsesDelegationSettingsInputParam>,
            T::Error: ::std::fmt::Display,
        {
            self.responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for responses: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponsesDelegationParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponsesDelegationParam> for super::LiveResponsesDelegationParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponsesDelegationParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                responses: value.responses?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponsesDelegationParam> for LiveResponsesDelegationParam {
        fn from(value: super::LiveResponsesDelegationParam) -> Self {
            Self {
                responses: Ok(value.responses),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponsesDelegationSettingsInputParam {
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_output_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        parallel_tool_calls:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        reasoning: ::std::result::Result<
            ::std::option::Option<super::LiveDelegationReasoningInputParam>,
            ::std::string::String,
        >,
        service_tier: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesServiceTier>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<super::LiveDelegationTextInputParam>,
            ::std::string::String,
        >,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesDelegationSettingsInputParamToolChoice>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::LiveResponsesDelegationSettingsInputParamToolsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveResponsesDelegationSettingsInputParam {
        fn default() -> Self {
            Self {
                instructions: Ok(Default::default()),
                max_output_tokens: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                parallel_tool_calls: Ok(Default::default()),
                reasoning: Ok(Default::default()),
                service_tier: Ok(Default::default()),
                text: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl LiveResponsesDelegationSettingsInputParam {
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
        pub fn max_output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_output_tokens: {e}"));
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
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn reasoning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveDelegationReasoningInputParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveResponsesServiceTier>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveDelegationTextInputParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponsesDelegationSettingsInputParamToolChoice>,
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
                ::std::vec::Vec<super::LiveResponsesDelegationSettingsInputParamToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponsesDelegationSettingsInputParam>
        for super::LiveResponsesDelegationSettingsInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponsesDelegationSettingsInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instructions: value.instructions?,
                max_output_tokens: value.max_output_tokens?,
                model: value.model?,
                parallel_tool_calls: value.parallel_tool_calls?,
                reasoning: value.reasoning?,
                service_tier: value.service_tier?,
                text: value.text?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponsesDelegationSettingsInputParam>
        for LiveResponsesDelegationSettingsInputParam
    {
        fn from(value: super::LiveResponsesDelegationSettingsInputParam) -> Self {
            Self {
                instructions: Ok(value.instructions),
                max_output_tokens: Ok(value.max_output_tokens),
                model: Ok(value.model),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                reasoning: Ok(value.reasoning),
                service_tier: Ok(value.service_tier),
                text: Ok(value.text),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponsesDelegationSettingsUpdateInputParam {
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max_output_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parallel_tool_calls:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        reasoning: ::std::result::Result<
            ::std::option::Option<super::LiveDelegationReasoningInputParam>,
            ::std::string::String,
        >,
        service_tier: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesServiceTier>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<super::LiveDelegationTextInputParam>,
            ::std::string::String,
        >,
        tool_choice: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesDelegationSettingsUpdateInputParamToolChoice>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<super::LiveResponsesDelegationSettingsUpdateInputParamToolsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveResponsesDelegationSettingsUpdateInputParam {
        fn default() -> Self {
            Self {
                instructions: Ok(Default::default()),
                max_output_tokens: Ok(Default::default()),
                model: Ok(Default::default()),
                parallel_tool_calls: Ok(Default::default()),
                reasoning: Ok(Default::default()),
                service_tier: Ok(Default::default()),
                text: Ok(Default::default()),
                tool_choice: Ok(Default::default()),
                tools: Ok(Default::default()),
            }
        }
    }
    impl LiveResponsesDelegationSettingsUpdateInputParam {
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
        pub fn max_output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_output_tokens: {e}"));
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
        pub fn parallel_tool_calls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.parallel_tool_calls = value.try_into().map_err(|e| {
                format!("error converting supplied value for parallel_tool_calls: {e}")
            });
            self
        }
        pub fn reasoning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveDelegationReasoningInputParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning: {e}"));
            self
        }
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveResponsesServiceTier>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveDelegationTextInputParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn tool_choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::LiveResponsesDelegationSettingsUpdateInputParamToolChoice,
                >,
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
                ::std::vec::Vec<super::LiveResponsesDelegationSettingsUpdateInputParamToolsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tools: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponsesDelegationSettingsUpdateInputParam>
        for super::LiveResponsesDelegationSettingsUpdateInputParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponsesDelegationSettingsUpdateInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instructions: value.instructions?,
                max_output_tokens: value.max_output_tokens?,
                model: value.model?,
                parallel_tool_calls: value.parallel_tool_calls?,
                reasoning: value.reasoning?,
                service_tier: value.service_tier?,
                text: value.text?,
                tool_choice: value.tool_choice?,
                tools: value.tools?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponsesDelegationSettingsUpdateInputParam>
        for LiveResponsesDelegationSettingsUpdateInputParam
    {
        fn from(value: super::LiveResponsesDelegationSettingsUpdateInputParam) -> Self {
            Self {
                instructions: Ok(value.instructions),
                max_output_tokens: Ok(value.max_output_tokens),
                model: Ok(value.model),
                parallel_tool_calls: Ok(value.parallel_tool_calls),
                reasoning: Ok(value.reasoning),
                service_tier: Ok(value.service_tier),
                text: Ok(value.text),
                tool_choice: Ok(value.tool_choice),
                tools: Ok(value.tools),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveResponsesDelegationUpdateParam {
        responses: ::std::result::Result<
            ::std::option::Option<super::LiveResponsesDelegationSettingsUpdateInputParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::LiveResponsesDelegationUpdateParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveResponsesDelegationUpdateParam {
        fn default() -> Self {
            Self {
                responses: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveResponsesDelegationUpdateParam {
        pub fn responses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveResponsesDelegationSettingsUpdateInputParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.responses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for responses: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveResponsesDelegationUpdateParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveResponsesDelegationUpdateParam>
        for super::LiveResponsesDelegationUpdateParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveResponsesDelegationUpdateParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                responses: value.responses?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveResponsesDelegationUpdateParam>
        for LiveResponsesDelegationUpdateParam
    {
        fn from(value: super::LiveResponsesDelegationUpdateParam) -> Self {
            Self {
                responses: Ok(value.responses),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionAudioFormatPcmParam {
        rate:
            ::std::result::Result<super::LiveSessionAudioFormatPcmParamRate, ::std::string::String>,
        type_:
            ::std::result::Result<super::LiveSessionAudioFormatPcmParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionAudioFormatPcmParam {
        fn default() -> Self {
            Self {
                rate: Err("no value supplied for rate".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionAudioFormatPcmParam {
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionAudioFormatPcmParamRate>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionAudioFormatPcmParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionAudioFormatPcmParam>
        for super::LiveSessionAudioFormatPcmParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionAudioFormatPcmParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rate: value.rate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionAudioFormatPcmParam>
        for LiveSessionAudioFormatPcmParam
    {
        fn from(value: super::LiveSessionAudioFormatPcmParam) -> Self {
            Self {
                rate: Ok(value.rate),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionAudioFormatPcmaParam {
        rate: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::LiveSessionAudioFormatPcmaParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSessionAudioFormatPcmaParam {
        fn default() -> Self {
            Self {
                rate: Err("no value supplied for rate".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionAudioFormatPcmaParam {
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionAudioFormatPcmaParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionAudioFormatPcmaParam>
        for super::LiveSessionAudioFormatPcmaParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionAudioFormatPcmaParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rate: value.rate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionAudioFormatPcmaParam>
        for LiveSessionAudioFormatPcmaParam
    {
        fn from(value: super::LiveSessionAudioFormatPcmaParam) -> Self {
            Self {
                rate: Ok(value.rate),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionAudioFormatPcmuParam {
        rate: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<
            super::LiveSessionAudioFormatPcmuParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSessionAudioFormatPcmuParam {
        fn default() -> Self {
            Self {
                rate: Err("no value supplied for rate".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionAudioFormatPcmuParam {
        pub fn rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionAudioFormatPcmuParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionAudioFormatPcmuParam>
        for super::LiveSessionAudioFormatPcmuParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionAudioFormatPcmuParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rate: value.rate?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionAudioFormatPcmuParam>
        for LiveSessionAudioFormatPcmuParam
    {
        fn from(value: super::LiveSessionAudioFormatPcmuParam) -> Self {
            Self {
                rate: Ok(value.rate),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionCloseParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveSessionCloseParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveSessionCloseParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionCloseParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionCloseParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveSessionCloseParamEventId>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCloseParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionCloseParam> for super::LiveSessionCloseParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionCloseParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionCloseParam> for LiveSessionCloseParam {
        fn from(value: super::LiveSessionCloseParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionClosed {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        reason: ::std::result::Result<super::LiveSessionClosedReason, ::std::string::String>,
        session: ::std::result::Result<super::LiveSessionResourceParam, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionClosedType, ::std::string::String>,
        usage: ::std::result::Result<super::LiveSessionUsage, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionClosed {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                reason: Err("no value supplied for reason".to_string()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl LiveSessionClosed {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionClosedReason>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionResourceParam>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionClosedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionClosed> for super::LiveSessionClosed {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionClosed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event_id: value.event_id?,
                reason: value.reason?,
                session: value.session?,
                type_: value.type_?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionClosed> for LiveSessionClosed {
        fn from(value: super::LiveSessionClosed) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event_id: Ok(value.event_id),
                reason: Ok(value.reason),
                session: Ok(value.session),
                type_: Ok(value.type_),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionCreateParams {
        audio: ::std::result::Result<
            ::std::option::Option<super::LiveInitialSessionAudioParam>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<super::LiveClientConfigParam>,
            ::std::string::String,
        >,
        delegation: ::std::result::Result<
            ::std::option::Option<super::LiveSessionCreateParamsDelegation>,
            ::std::string::String,
        >,
        input:
            ::std::result::Result<::std::vec::Vec<super::LiveInitialItem>, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::ModelIdsLive, ::std::string::String>,
        store: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionCreateParams {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                client: Ok(Default::default()),
                delegation: Ok(Default::default()),
                input: Ok(Default::default()),
                instructions: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                store: Ok(Default::default()),
            }
        }
    }
    impl LiveSessionCreateParams {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveInitialSessionAudioParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveClientConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveSessionCreateParamsDelegation>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LiveInitialItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModelIdsLive>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
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
    }
    impl ::std::convert::TryFrom<LiveSessionCreateParams> for super::LiveSessionCreateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionCreateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                client: value.client?,
                delegation: value.delegation?,
                input: value.input?,
                instructions: value.instructions?,
                model: value.model?,
                store: value.store?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionCreateParams> for LiveSessionCreateParams {
        fn from(value: super::LiveSessionCreateParams) -> Self {
            Self {
                audio: Ok(value.audio),
                client: Ok(value.client),
                delegation: Ok(value.delegation),
                input: Ok(value.input),
                instructions: Ok(value.instructions),
                model: Ok(value.model),
                store: Ok(value.store),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionCreateRequest {
        session: ::std::result::Result<super::LiveMediaSessionCreateParams, ::std::string::String>,
        transport:
            ::std::result::Result<super::LiveSessionCreateRequestTransport, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionCreateRequest {
        fn default() -> Self {
            Self {
                session: Err("no value supplied for session".to_string()),
                transport: Err("no value supplied for transport".to_string()),
            }
        }
    }
    impl LiveSessionCreateRequest {
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveMediaSessionCreateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCreateRequestTransport>,
            T::Error: ::std::fmt::Display,
        {
            self.transport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionCreateRequest> for super::LiveSessionCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session: value.session?,
                transport: value.transport?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionCreateRequest> for LiveSessionCreateRequest {
        fn from(value: super::LiveSessionCreateRequest) -> Self {
            Self {
                session: Ok(value.session),
                transport: Ok(value.transport),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionCreateResponse {
        session: ::std::result::Result<super::Session, ::std::string::String>,
        transport:
            ::std::result::Result<super::LiveSessionCreateResponseTransport, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionCreateResponse {
        fn default() -> Self {
            Self {
                session: Err("no value supplied for session".to_string()),
                transport: Err("no value supplied for transport".to_string()),
            }
        }
    }
    impl LiveSessionCreateResponse {
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Session>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCreateResponseTransport>,
            T::Error: ::std::fmt::Display,
        {
            self.transport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionCreateResponse> for super::LiveSessionCreateResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionCreateResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session: value.session?,
                transport: value.transport?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionCreateResponse> for LiveSessionCreateResponse {
        fn from(value: super::LiveSessionCreateResponse) -> Self {
            Self {
                session: Ok(value.session),
                transport: Ok(value.transport),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionResourceParam {
        audio: ::std::result::Result<
            ::std::option::Option<super::LiveInitialSessionAudioParam>,
            ::std::string::String,
        >,
        client: ::std::result::Result<
            ::std::option::Option<super::LiveClientConfigParam>,
            ::std::string::String,
        >,
        delegation: ::std::result::Result<
            ::std::option::Option<super::LiveSessionResourceParamDelegation>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        input:
            ::std::result::Result<::std::vec::Vec<super::LiveInitialItem>, ::std::string::String>,
        instructions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        model: ::std::result::Result<super::ModelIdsLive, ::std::string::String>,
        status: ::std::result::Result<super::LiveSessionResourceParamStatus, ::std::string::String>,
        store: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionResourceParam {
        fn default() -> Self {
            Self {
                audio: Ok(Default::default()),
                client: Ok(Default::default()),
                delegation: Ok(Default::default()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                input: Ok(Default::default()),
                instructions: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                status: Err("no value supplied for status".to_string()),
                store: Ok(Default::default()),
            }
        }
    }
    impl LiveSessionResourceParam {
        pub fn audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveInitialSessionAudioParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio: {e}"));
            self
        }
        pub fn client<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveClientConfigParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.client = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client: {e}"));
            self
        }
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveSessionResourceParamDelegation>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
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
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LiveInitialItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModelIdsLive>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionResourceParamStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
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
    }
    impl ::std::convert::TryFrom<LiveSessionResourceParam> for super::LiveSessionResourceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionResourceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                audio: value.audio?,
                client: value.client?,
                delegation: value.delegation?,
                expires_at: value.expires_at?,
                id: value.id?,
                input: value.input?,
                instructions: value.instructions?,
                model: value.model?,
                status: value.status?,
                store: value.store?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionResourceParam> for LiveSessionResourceParam {
        fn from(value: super::LiveSessionResourceParam) -> Self {
            Self {
                audio: Ok(value.audio),
                client: Ok(value.client),
                delegation: Ok(value.delegation),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                input: Ok(value.input),
                instructions: Ok(value.instructions),
                model: Ok(value.model),
                status: Ok(value.status),
                store: Ok(value.store),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionStartEvent {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveSessionStartEventEventId>,
            ::std::string::String,
        >,
        session: ::std::result::Result<super::LiveSessionCreateParams, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionStartEventType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionStartEvent {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionStartEvent {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveSessionStartEventEventId>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionCreateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionStartEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionStartEvent> for super::LiveSessionStartEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionStartEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionStartEvent> for LiveSessionStartEvent {
        fn from(value: super::LiveSessionStartEvent) -> Self {
            Self {
                event_id: Ok(value.event_id),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionStarted {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        session: ::std::result::Result<super::LiveSessionResourceParam, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionStartedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionStarted {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionStarted {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionResourceParam>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionStartedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionStarted> for super::LiveSessionStarted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionStarted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event_id: value.event_id?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionStarted> for LiveSessionStarted {
        fn from(value: super::LiveSessionStarted) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event_id: Ok(value.event_id),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionUpdateParam {
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveSessionUpdateParamEventId>,
            ::std::string::String,
        >,
        session: ::std::result::Result<super::LiveSessionUpdateParams, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionUpdateParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionUpdateParam {
        fn default() -> Self {
            Self {
                event_id: Ok(Default::default()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionUpdateParam {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveSessionUpdateParamEventId>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUpdateParams>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUpdateParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionUpdateParam> for super::LiveSessionUpdateParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionUpdateParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionUpdateParam> for LiveSessionUpdateParam {
        fn from(value: super::LiveSessionUpdateParam) -> Self {
            Self {
                event_id: Ok(value.event_id),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionUpdateParams {
        delegation: ::std::result::Result<
            ::std::option::Option<super::LiveSessionUpdateParamsDelegation>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSessionUpdateParams {
        fn default() -> Self {
            Self {
                delegation: Ok(Default::default()),
            }
        }
    }
    impl LiveSessionUpdateParams {
        pub fn delegation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveSessionUpdateParamsDelegation>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionUpdateParams> for super::LiveSessionUpdateParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionUpdateParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delegation: value.delegation?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionUpdateParams> for LiveSessionUpdateParams {
        fn from(value: super::LiveSessionUpdateParams) -> Self {
            Self {
                delegation: Ok(value.delegation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionUpdated {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        session: ::std::result::Result<super::LiveSessionResourceParam, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionUpdatedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionUpdated {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                session: Err("no value supplied for session".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSessionUpdated {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionResourceParam>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUpdatedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionUpdated> for super::LiveSessionUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event_id: value.event_id?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionUpdated> for LiveSessionUpdated {
        fn from(value: super::LiveSessionUpdated) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event_id: Ok(value.event_id),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionUsage {
        seconds: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionUsage {
        fn default() -> Self {
            Self {
                seconds: Err("no value supplied for seconds".to_string()),
            }
        }
    }
    impl LiveSessionUsage {
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionUsage> for super::LiveSessionUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                seconds: value.seconds?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionUsage> for LiveSessionUsage {
        fn from(value: super::LiveSessionUsage) -> Self {
            Self {
                seconds: Ok(value.seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSessionUsageUpdated {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        context_window: ::std::result::Result<
            ::std::option::Option<super::LiveContextWindowUsage>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSessionUsageUpdatedType, ::std::string::String>,
        usage: ::std::result::Result<super::LiveSessionUsage, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSessionUsageUpdated {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                context_window: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl LiveSessionUsageUpdated {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn context_window<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiveContextWindowUsage>>,
            T::Error: ::std::fmt::Display,
        {
            self.context_window = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_window: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUsageUpdatedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSessionUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSessionUsageUpdated> for super::LiveSessionUsageUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSessionUsageUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                context_window: value.context_window?,
                event_id: value.event_id?,
                type_: value.type_?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSessionUsageUpdated> for LiveSessionUsageUpdated {
        fn from(value: super::LiveSessionUsageUpdated) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                context_window: Ok(value.context_window),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSipTransport {
        destination:
            ::std::result::Result<super::LiveSipTransportDestination, ::std::string::String>,
        trunk: ::std::result::Result<super::LiveSipTrunk, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSipTransportType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSipTransport {
        fn default() -> Self {
            Self {
                destination: Err("no value supplied for destination".to_string()),
                trunk: Err("no value supplied for trunk".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSipTransport {
        pub fn destination<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTransportDestination>,
            T::Error: ::std::fmt::Display,
        {
            self.destination = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for destination: {e}"));
            self
        }
        pub fn trunk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunk>,
            T::Error: ::std::fmt::Display,
        {
            self.trunk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trunk: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTransportType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSipTransport> for super::LiveSipTransport {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSipTransport,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                destination: value.destination?,
                trunk: value.trunk?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSipTransport> for LiveSipTransport {
        fn from(value: super::LiveSipTransport) -> Self {
            Self {
                destination: Ok(value.destination),
                trunk: Ok(value.trunk),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSipTrunk {
        auth: ::std::result::Result<super::LiveSipTrunkAuth, ::std::string::String>,
        caller_number:
            ::std::result::Result<super::LiveSipTrunkCallerNumber, ::std::string::String>,
        provider_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSipTrunk {
        fn default() -> Self {
            Self {
                auth: Err("no value supplied for auth".to_string()),
                caller_number: Err("no value supplied for caller_number".to_string()),
                provider_url: Err("no value supplied for provider_url".to_string()),
            }
        }
    }
    impl LiveSipTrunk {
        pub fn auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunkAuth>,
            T::Error: ::std::fmt::Display,
        {
            self.auth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth: {e}"));
            self
        }
        pub fn caller_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunkCallerNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.caller_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for caller_number: {e}"));
            self
        }
        pub fn provider_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.provider_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider_url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSipTrunk> for super::LiveSipTrunk {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSipTrunk,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auth: value.auth?,
                caller_number: value.caller_number?,
                provider_url: value.provider_url?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSipTrunk> for LiveSipTrunk {
        fn from(value: super::LiveSipTrunk) -> Self {
            Self {
                auth: Ok(value.auth),
                caller_number: Ok(value.caller_number),
                provider_url: Ok(value.provider_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSipTrunkAuth {
        password: ::std::result::Result<super::LiveSipTrunkAuthPassword, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSipTrunkAuthType, ::std::string::String>,
        username: ::std::result::Result<super::LiveSipTrunkAuthUsername, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSipTrunkAuth {
        fn default() -> Self {
            Self {
                password: Err("no value supplied for password".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                username: Err("no value supplied for username".to_string()),
            }
        }
    }
    impl LiveSipTrunkAuth {
        pub fn password<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunkAuthPassword>,
            T::Error: ::std::fmt::Display,
        {
            self.password = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for password: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunkAuthType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn username<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSipTrunkAuthUsername>,
            T::Error: ::std::fmt::Display,
        {
            self.username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for username: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSipTrunkAuth> for super::LiveSipTrunkAuth {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSipTrunkAuth,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                password: value.password?,
                type_: value.type_?,
                username: value.username?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSipTrunkAuth> for LiveSipTrunkAuth {
        fn from(value: super::LiveSipTrunkAuth) -> Self {
            Self {
                password: Ok(value.password),
                type_: Ok(value.type_),
                username: Ok(value.username),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSkillReferenceParam {
        skill_id:
            ::std::result::Result<super::LiveSkillReferenceParamSkillId, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSkillReferenceParamType, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSkillReferenceParam {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl LiveSkillReferenceParam {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSkillReferenceParamSkillId>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSkillReferenceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
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
    impl ::std::convert::TryFrom<LiveSkillReferenceParam> for super::LiveSkillReferenceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSkillReferenceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSkillReferenceParam> for LiveSkillReferenceParam {
        fn from(value: super::LiveSkillReferenceParam) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificApplyPatchParam {
        type_: ::std::result::Result<super::LiveSpecificApplyPatchParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificApplyPatchParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificApplyPatchParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificApplyPatchParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificApplyPatchParam> for super::LiveSpecificApplyPatchParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificApplyPatchParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificApplyPatchParam> for LiveSpecificApplyPatchParam {
        fn from(value: super::LiveSpecificApplyPatchParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificCodeInterpreterParam {
        type_: ::std::result::Result<
            super::LiveSpecificCodeInterpreterParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSpecificCodeInterpreterParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificCodeInterpreterParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificCodeInterpreterParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificCodeInterpreterParam>
        for super::LiveSpecificCodeInterpreterParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificCodeInterpreterParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificCodeInterpreterParam>
        for LiveSpecificCodeInterpreterParam
    {
        fn from(value: super::LiveSpecificCodeInterpreterParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificComputerParam {
        type_: ::std::result::Result<super::LiveSpecificComputerParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificComputerParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificComputerParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificComputerParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificComputerParam> for super::LiveSpecificComputerParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificComputerParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificComputerParam> for LiveSpecificComputerParam {
        fn from(value: super::LiveSpecificComputerParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificCustomToolParam {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveSpecificCustomToolParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificCustomToolParam {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificCustomToolParam {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificCustomToolParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificCustomToolParam> for super::LiveSpecificCustomToolParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificCustomToolParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificCustomToolParam> for LiveSpecificCustomToolParam {
        fn from(value: super::LiveSpecificCustomToolParam) -> Self {
            Self {
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificFileSearchParam {
        type_: ::std::result::Result<super::LiveSpecificFileSearchParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificFileSearchParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificFileSearchParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificFileSearchParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificFileSearchParam> for super::LiveSpecificFileSearchParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificFileSearchParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificFileSearchParam> for LiveSpecificFileSearchParam {
        fn from(value: super::LiveSpecificFileSearchParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificFunctionShellParam {
        type_:
            ::std::result::Result<super::LiveSpecificFunctionShellParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificFunctionShellParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificFunctionShellParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificFunctionShellParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificFunctionShellParam>
        for super::LiveSpecificFunctionShellParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificFunctionShellParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificFunctionShellParam>
        for LiveSpecificFunctionShellParam
    {
        fn from(value: super::LiveSpecificFunctionShellParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificImageGenParam {
        type_: ::std::result::Result<super::LiveSpecificImageGenParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificImageGenParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificImageGenParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificImageGenParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificImageGenParam> for super::LiveSpecificImageGenParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificImageGenParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificImageGenParam> for LiveSpecificImageGenParam {
        fn from(value: super::LiveSpecificImageGenParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificProgrammaticToolCallingParam {
        type_: ::std::result::Result<
            super::LiveSpecificProgrammaticToolCallingParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSpecificProgrammaticToolCallingParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificProgrammaticToolCallingParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificProgrammaticToolCallingParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificProgrammaticToolCallingParam>
        for super::LiveSpecificProgrammaticToolCallingParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificProgrammaticToolCallingParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificProgrammaticToolCallingParam>
        for LiveSpecificProgrammaticToolCallingParam
    {
        fn from(value: super::LiveSpecificProgrammaticToolCallingParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificWebSearchParam {
        type_: ::std::result::Result<super::LiveSpecificWebSearchParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveSpecificWebSearchParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificWebSearchParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificWebSearchParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificWebSearchParam> for super::LiveSpecificWebSearchParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificWebSearchParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificWebSearchParam> for LiveSpecificWebSearchParam {
        fn from(value: super::LiveSpecificWebSearchParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveSpecificWebSearchPreviewParam {
        type_: ::std::result::Result<
            super::LiveSpecificWebSearchPreviewParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LiveSpecificWebSearchPreviewParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveSpecificWebSearchPreviewParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveSpecificWebSearchPreviewParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveSpecificWebSearchPreviewParam>
        for super::LiveSpecificWebSearchPreviewParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveSpecificWebSearchPreviewParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveSpecificWebSearchPreviewParam>
        for LiveSpecificWebSearchPreviewParam
    {
        fn from(value: super::LiveSpecificWebSearchPreviewParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveThinkingAppendParam {
        content: ::std::result::Result<::std::string::String, ::std::string::String>,
        delegation_id: ::std::result::Result<
            ::std::option::Option<super::LiveThinkingAppendParamDelegationId>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<
            ::std::option::Option<super::LiveThinkingAppendParamEventId>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveThinkingAppendParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveThinkingAppendParam {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                delegation_id: Err("no value supplied for delegation_id".to_string()),
                event_id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveThinkingAppendParam {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn delegation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveThinkingAppendParamDelegationId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.delegation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delegation_id: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LiveThinkingAppendParamEventId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveThinkingAppendParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveThinkingAppendParam> for super::LiveThinkingAppendParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveThinkingAppendParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                delegation_id: value.delegation_id?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveThinkingAppendParam> for LiveThinkingAppendParam {
        fn from(value: super::LiveThinkingAppendParam) -> Self {
            Self {
                content: Ok(value.content),
                delegation_id: Ok(value.delegation_id),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveThinkingAppended {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_ms: ::std::result::Result<i64, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_ms: ::std::result::Result<i64, ::std::string::String>,
        type_: ::std::result::Result<super::LiveThinkingAppendedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveThinkingAppended {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                end_ms: Err("no value supplied for end_ms".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                start_ms: Err("no value supplied for start_ms".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveThinkingAppended {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn end_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_ms: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn start_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_ms: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveThinkingAppendedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveThinkingAppended> for super::LiveThinkingAppended {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveThinkingAppended,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                end_ms: value.end_ms?,
                event_id: value.event_id?,
                start_ms: value.start_ms?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveThinkingAppended> for LiveThinkingAppended {
        fn from(value: super::LiveThinkingAppended) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                end_ms: Ok(value.end_ms),
                event_id: Ok(value.event_id),
                start_ms: Ok(value.start_ms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveToolSearchToolInputParam {
        type_:
            ::std::result::Result<super::LiveToolSearchToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveToolSearchToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveToolSearchToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveToolSearchToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveToolSearchToolInputParam> for super::LiveToolSearchToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveToolSearchToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveToolSearchToolInputParam> for LiveToolSearchToolInputParam {
        fn from(value: super::LiveToolSearchToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportAnswered {
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveTransportAnsweredType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportAnswered {
        fn default() -> Self {
            Self {
                event_id: Err("no value supplied for event_id".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportAnswered {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportAnsweredType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportAnswered> for super::LiveTransportAnswered {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportAnswered,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportAnswered> for LiveTransportAnswered {
        fn from(value: super::LiveTransportAnswered) -> Self {
            Self {
                event_id: Ok(value.event_id),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportCallError {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        param: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LiveTransportCallErrorType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportCallError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
                param: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportCallError {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportCallErrorType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportCallError> for super::LiveTransportCallError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportCallError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
                param: value.param?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportCallError> for LiveTransportCallError {
        fn from(value: super::LiveTransportCallError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
                param: Ok(value.param),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportDtmfReceived {
        event: ::std::result::Result<super::LiveTransportDtmfReceivedEvent, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveTransportDtmfReceivedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportDtmfReceived {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportDtmfReceived {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportDtmfReceivedEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportDtmfReceivedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportDtmfReceived> for super::LiveTransportDtmfReceived {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportDtmfReceived,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportDtmfReceived> for LiveTransportDtmfReceived {
        fn from(value: super::LiveTransportDtmfReceived) -> Self {
            Self {
                event: Ok(value.event),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportDtmfSend {
        client_event_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event: ::std::result::Result<super::LiveTransportDtmfSendEvent, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveTransportDtmfSendType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportDtmfSend {
        fn default() -> Self {
            Self {
                client_event_id: Ok(Default::default()),
                event: Err("no value supplied for event".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportDtmfSend {
        pub fn client_event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_event_id: {e}"));
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportDtmfSendEvent>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportDtmfSendType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportDtmfSend> for super::LiveTransportDtmfSend {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportDtmfSend,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_event_id: value.client_event_id?,
                event: value.event?,
                event_id: value.event_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportDtmfSend> for LiveTransportDtmfSend {
        fn from(value: super::LiveTransportDtmfSend) -> Self {
            Self {
                client_event_id: Ok(value.client_event_id),
                event: Ok(value.event),
                event_id: Ok(value.event_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportFailed {
        error: ::std::result::Result<super::LiveTransportCallError, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveTransportFailedType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportFailed {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportFailed {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportCallError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportFailedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportFailed> for super::LiveTransportFailed {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportFailed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                event_id: value.event_id?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportFailed> for LiveTransportFailed {
        fn from(value: super::LiveTransportFailed) -> Self {
            Self {
                error: Ok(value.error),
                event_id: Ok(value.event_id),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveTransportRinging {
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::LiveTransportRingingType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveTransportRinging {
        fn default() -> Self {
            Self {
                event_id: Err("no value supplied for event_id".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveTransportRinging {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveTransportRingingType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveTransportRinging> for super::LiveTransportRinging {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveTransportRinging,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                session_id: value.session_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveTransportRinging> for LiveTransportRinging {
        fn from(value: super::LiveTransportRinging) -> Self {
            Self {
                event_id: Ok(value.event_id),
                session_id: Ok(value.session_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveWebRtcTransport {
        sdp: ::std::result::Result<super::LiveWebRtcTransportSdp, ::std::string::String>,
        type_: ::std::result::Result<super::LiveWebRtcTransportType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveWebRtcTransport {
        fn default() -> Self {
            Self {
                sdp: Err("no value supplied for sdp".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveWebRtcTransport {
        pub fn sdp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveWebRtcTransportSdp>,
            T::Error: ::std::fmt::Display,
        {
            self.sdp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sdp: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveWebRtcTransportType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveWebRtcTransport> for super::LiveWebRtcTransport {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveWebRtcTransport,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                sdp: value.sdp?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveWebRtcTransport> for LiveWebRtcTransport {
        fn from(value: super::LiveWebRtcTransport) -> Self {
            Self {
                sdp: Ok(value.sdp),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveWebSearchToolInputParam {
        type_: ::std::result::Result<super::LiveWebSearchToolInputParamType, ::std::string::String>,
    }
    impl ::std::default::Default for LiveWebSearchToolInputParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl LiveWebSearchToolInputParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiveWebSearchToolInputParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LiveWebSearchToolInputParam> for super::LiveWebSearchToolInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiveWebSearchToolInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LiveWebSearchToolInputParam> for LiveWebSearchToolInputParam {
        fn from(value: super::LiveWebSearchToolInputParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReferLiveSessionParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ReferLiveSessionParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl ReferLiveSessionParams {
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
    impl ::std::convert::TryFrom<ReferLiveSessionParams> for super::ReferLiveSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferLiveSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::ReferLiveSessionParams> for ReferLiveSessionParams {
        fn from(value: super::ReferLiveSessionParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RejectLiveSessionParams {
        session_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RejectLiveSessionParams {
        fn default() -> Self {
            Self {
                session_id: Err("no value supplied for session_id".to_string()),
            }
        }
    }
    impl RejectLiveSessionParams {
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
    impl ::std::convert::TryFrom<RejectLiveSessionParams> for super::RejectLiveSessionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RejectLiveSessionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
            })
        }
    }
    impl ::std::convert::From<super::RejectLiveSessionParams> for RejectLiveSessionParams {
        fn from(value: super::RejectLiveSessionParams) -> Self {
            Self {
                session_id: Ok(value.session_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Session {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Session {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl Session {
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
    impl ::std::convert::TryFrom<Session> for super::Session {
        type Error = super::error::ConversionError;
        fn try_from(value: Session) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::Session> for Session {
        fn from(value: super::Session) -> Self {
            Self { id: Ok(value.id) }
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
