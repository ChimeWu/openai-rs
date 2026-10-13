pub use crate::common::{ApplyPatchToolParam, ApproximateLocation, AutoCodeInterpreterToolParam, CallableToolAllowedCaller, ChatCompletionTool, CodeInterpreterTool, ComparisonFilter, CompoundFilter, ComputerEnvironment, ComputerTool, ComputerUsePreviewTool, ContainerAutoParam, ContainerMemoryLimit, ContainerNetworkPolicyAllowlistParam, ContainerNetworkPolicyDisabledParam, ContainerNetworkPolicyDomainSecretParam, ContainerReferenceParam, CustomGrammarFormatParam, CustomTextFormatParam, CustomToolParam, EasyInputMessage, EmptyModelParam, Error, ErrorResponse, EvalItem, EvalItemContent, EvalItemContentArray, EvalItemContentItem, EvalItemContentOutputText, EvalItemContentText, EvalItemInputImage, FileInputDetail, FileSearchTool, Filters, FunctionObject, FunctionParameters, FunctionShellToolParam, FunctionTool, FunctionToolParam, GraderLabelModel, GraderPython, GraderScoreModel, GraderStringCheck, GraderTextSimilarity, GrammarSyntax1, HybridSearchOptions, ImageDetail, ImageGenActionEnum, ImageGenTool, InlineSkillParam, InlineSkillSourceParam, InputAudio, InputContent, InputFidelity, InputFileContent, InputImageContent, InputMessageContentList, InputTextContent, LocalEnvironmentParam, LocalShellToolParam, LocalSkillParam, McpTool, McpToolFilter, MessagePhase, Metadata, MisalignmentErrorDetailsResource, NamespaceToolParam, ProgrammaticToolCallingParam, PromptCacheBreakpointConfig, RankerVersionType, RankingOptions, ReasoningEffort, ResponseFormatJsonObject, ResponseFormatJsonSchema, ResponseFormatJsonSchemaSchema, ResponseFormatText, SearchContentType, SearchContextSize, SkillReferenceParam, TextResponseFormatConfiguration, TextResponseFormatJsonSchema, Tool, ToolSearchExecutionType, ToolSearchToolParam, WebSearchApproximateLocation, WebSearchPreviewTool, WebSearchTool, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`CancelEvalRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CancelEvalRunParams {
    pub eval_id: ::std::string::String,
    pub run_id: ::std::string::String,
}
impl CancelEvalRunParams {
    pub fn builder() -> builder::CancelEvalRunParams {
        Default::default()
    }
}
#[doc = "A CompletionsRunDataSource object describing a model sampling configuration.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalCompletionsRunDataSource {
    #[doc = "Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_messages: ::std::option::Option<CreateEvalCompletionsRunDataSourceInputMessages>,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sampling_params: ::std::option::Option<CreateEvalCompletionsRunDataSourceSamplingParams>,
    #[doc = "Determines what populates the `item` namespace in this run's data source."]
    pub source: CreateEvalCompletionsRunDataSourceSource,
    #[doc = "The type of run data source. Always `completions`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalCompletionsRunDataSourceType,
}
impl CreateEvalCompletionsRunDataSource {
    pub fn builder() -> builder::CreateEvalCompletionsRunDataSource {
        Default::default()
    }
}
#[doc = "Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum CreateEvalCompletionsRunDataSourceInputMessages {
    #[doc = "TemplateInputMessages"]
    #[serde(rename = "template")]
    Template {
        #[doc = "A list of chat messages forming the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}."]
        template: ::std::vec::Vec<TemplateInputMessagesTemplateItem>,
    },
    #[doc = "ItemReferenceInputMessages"]
    #[serde(rename = "item_reference")]
    ItemReference {
        #[doc = "A reference to a variable in the `item` namespace. Ie, \"item.input_trajectory\""]
        item_reference: ::std::string::String,
    },
}
#[doc = "`CreateEvalCompletionsRunDataSourceSamplingParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
    #[doc = "The maximum number of tokens in the generated output."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format:
        ::std::option::Option<CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat>,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    #[doc = "A higher temperature increases randomness in the outputs."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[doc = "A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<::openai_api_types::common::ChatCompletionTool>,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
impl CreateEvalCompletionsRunDataSourceSamplingParams {
    pub fn builder() -> builder::CreateEvalCompletionsRunDataSourceSamplingParams {
        Default::default()
    }
}
#[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat {
    Variant0(::openai_api_types::common::ResponseFormatText),
    Variant1(::openai_api_types::common::ResponseFormatJsonSchema),
    Variant2(::openai_api_types::common::ResponseFormatJsonObject),
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatText>
    for CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat
{
    fn from(value: ::openai_api_types::common::ResponseFormatText) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatJsonSchema>
    for CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat
{
    fn from(value: ::openai_api_types::common::ResponseFormatJsonSchema) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::ResponseFormatJsonObject>
    for CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat
{
    fn from(value: ::openai_api_types::common::ResponseFormatJsonObject) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "Determines what populates the `item` namespace in this run's data source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceSource {
    JsonlFileContentSource(EvalJsonlFileContentSource),
    JsonlFileIdSource(EvalJsonlFileIdSource),
    StoredCompletionsSource(EvalStoredCompletionsSource),
}
impl ::std::convert::From<EvalJsonlFileContentSource> for CreateEvalCompletionsRunDataSourceSource {
    fn from(value: EvalJsonlFileContentSource) -> Self {
        Self::JsonlFileContentSource(value)
    }
}
impl ::std::convert::From<EvalJsonlFileIdSource> for CreateEvalCompletionsRunDataSourceSource {
    fn from(value: EvalJsonlFileIdSource) -> Self {
        Self::JsonlFileIdSource(value)
    }
}
impl ::std::convert::From<EvalStoredCompletionsSource>
    for CreateEvalCompletionsRunDataSourceSource
{
    fn from(value: EvalStoredCompletionsSource) -> Self {
        Self::StoredCompletionsSource(value)
    }
}
#[doc = "The type of run data source. Always `completions`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalCompletionsRunDataSourceType {
    #[serde(rename = "completions")]
    Completions,
}
impl ::std::fmt::Display for CreateEvalCompletionsRunDataSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completions => f.write_str("completions"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalCompletionsRunDataSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completions" => Ok(Self::Completions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalCompletionsRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalCompletionsRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation runs.\nThis schema is used to define the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalCustomDataSourceConfig {
    #[doc = "Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source)"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_sample_schema: ::std::option::Option<bool>,
    #[doc = "The json schema for each row in the data source."]
    pub item_schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalCustomDataSourceConfigType,
}
impl CreateEvalCustomDataSourceConfig {
    pub fn builder() -> builder::CreateEvalCustomDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `custom`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalCustomDataSourceConfigType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for CreateEvalCustomDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalCustomDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalCustomDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalCustomDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A chat message that makes up the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalItem {
    Variant0 {
        #[doc = "The content of the message."]
        content: ::std::string::String,
        #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
        role: ::std::string::String,
    },
    Variant1(::openai_api_types::common::EvalItem),
}
impl ::std::convert::From<::openai_api_types::common::EvalItem> for CreateEvalItem {
    fn from(value: ::openai_api_types::common::EvalItem) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "A JsonlRunDataSource object with that specifies a JSONL file that matches the eval \n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalJsonlRunDataSource {
    #[doc = "Determines what populates the `item` namespace in the data source."]
    pub source: CreateEvalJsonlRunDataSourceSource,
    #[doc = "The type of data source. Always `jsonl`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalJsonlRunDataSourceType,
}
impl CreateEvalJsonlRunDataSource {
    pub fn builder() -> builder::CreateEvalJsonlRunDataSource {
        Default::default()
    }
}
#[doc = "Determines what populates the `item` namespace in the data source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalJsonlRunDataSourceSource {
    ContentSource(EvalJsonlFileContentSource),
    IdSource(EvalJsonlFileIdSource),
}
impl ::std::convert::From<EvalJsonlFileContentSource> for CreateEvalJsonlRunDataSourceSource {
    fn from(value: EvalJsonlFileContentSource) -> Self {
        Self::ContentSource(value)
    }
}
impl ::std::convert::From<EvalJsonlFileIdSource> for CreateEvalJsonlRunDataSourceSource {
    fn from(value: EvalJsonlFileIdSource) -> Self {
        Self::IdSource(value)
    }
}
#[doc = "The type of data source. Always `jsonl`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalJsonlRunDataSourceType {
    #[serde(rename = "jsonl")]
    Jsonl,
}
impl ::std::fmt::Display for CreateEvalJsonlRunDataSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Jsonl => f.write_str("jsonl"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalJsonlRunDataSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "jsonl" => Ok(Self::Jsonl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalJsonlRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalJsonlRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalLabelModelGrader {
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}."]
    pub input: ::std::vec::Vec<CreateEvalItem>,
    #[doc = "The labels to classify to each item in the evaluation."]
    pub labels: ::std::vec::Vec<::std::string::String>,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    pub model: ::std::string::String,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    pub passing_labels: ::std::vec::Vec<::std::string::String>,
    #[doc = "The object type, which is always `label_model`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalLabelModelGraderType,
}
impl CreateEvalLabelModelGrader {
    pub fn builder() -> builder::CreateEvalLabelModelGrader {
        Default::default()
    }
}
#[doc = "The object type, which is always `label_model`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalLabelModelGraderType {
    #[serde(rename = "label_model")]
    LabelModel,
}
impl ::std::fmt::Display for CreateEvalLabelModelGraderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LabelModel => f.write_str("label_model"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalLabelModelGraderType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "label_model" => Ok(Self::LabelModel),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalLabelModelGraderType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalLabelModelGraderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A data source config which specifies the metadata property of your logs query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalLogsDataSourceConfig {
    #[doc = "Metadata filters for the logs data source."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `logs`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalLogsDataSourceConfigType,
}
impl CreateEvalLogsDataSourceConfig {
    pub fn builder() -> builder::CreateEvalLogsDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `logs`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalLogsDataSourceConfigType {
    #[serde(rename = "logs")]
    Logs,
}
impl ::std::fmt::Display for CreateEvalLogsDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logs => f.write_str("logs"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalLogsDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logs" => Ok(Self::Logs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalLogsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalLogsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateEvalRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalRequest {
    #[doc = "The configuration for the data source used for the evaluation runs. Dictates the schema of the data used in the evaluation."]
    pub data_source_config: CreateEvalRequestDataSourceConfig,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The name of the evaluation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "A list of graders for all eval runs in this group. Graders can reference variables in the data source using double curly braces notation, like `{{item.variable_name}}`. To reference the model's output, use the `sample` namespace (ie, `{{sample.output_text}}`)."]
    pub testing_criteria: ::std::vec::Vec<CreateEvalRequestTestingCriteriaItem>,
}
impl CreateEvalRequest {
    pub fn builder() -> builder::CreateEvalRequest {
        Default::default()
    }
}
#[doc = "The configuration for the data source used for the evaluation runs. Dictates the schema of the data used in the evaluation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalRequestDataSourceConfig {
    CustomDataSourceConfig(CreateEvalCustomDataSourceConfig),
    LogsDataSourceConfig(CreateEvalLogsDataSourceConfig),
    StoredCompletionsDataSourceConfig(CreateEvalStoredCompletionsDataSourceConfig),
}
impl ::std::convert::From<CreateEvalCustomDataSourceConfig> for CreateEvalRequestDataSourceConfig {
    fn from(value: CreateEvalCustomDataSourceConfig) -> Self {
        Self::CustomDataSourceConfig(value)
    }
}
impl ::std::convert::From<CreateEvalLogsDataSourceConfig> for CreateEvalRequestDataSourceConfig {
    fn from(value: CreateEvalLogsDataSourceConfig) -> Self {
        Self::LogsDataSourceConfig(value)
    }
}
impl ::std::convert::From<CreateEvalStoredCompletionsDataSourceConfig>
    for CreateEvalRequestDataSourceConfig
{
    fn from(value: CreateEvalStoredCompletionsDataSourceConfig) -> Self {
        Self::StoredCompletionsDataSourceConfig(value)
    }
}
#[doc = "`CreateEvalRequestTestingCriteriaItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalRequestTestingCriteriaItem {
    CreateEvalLabelModelGrader(CreateEvalLabelModelGrader),
    EvalGraderStringCheck(EvalGraderStringCheck),
    EvalGraderTextSimilarity(EvalGraderTextSimilarity),
    EvalGraderPython(EvalGraderPython),
    EvalGraderScoreModel(EvalGraderScoreModel),
}
impl ::std::convert::From<CreateEvalLabelModelGrader> for CreateEvalRequestTestingCriteriaItem {
    fn from(value: CreateEvalLabelModelGrader) -> Self {
        Self::CreateEvalLabelModelGrader(value)
    }
}
impl ::std::convert::From<EvalGraderStringCheck> for CreateEvalRequestTestingCriteriaItem {
    fn from(value: EvalGraderStringCheck) -> Self {
        Self::EvalGraderStringCheck(value)
    }
}
impl ::std::convert::From<EvalGraderTextSimilarity> for CreateEvalRequestTestingCriteriaItem {
    fn from(value: EvalGraderTextSimilarity) -> Self {
        Self::EvalGraderTextSimilarity(value)
    }
}
impl ::std::convert::From<EvalGraderPython> for CreateEvalRequestTestingCriteriaItem {
    fn from(value: EvalGraderPython) -> Self {
        Self::EvalGraderPython(value)
    }
}
impl ::std::convert::From<EvalGraderScoreModel> for CreateEvalRequestTestingCriteriaItem {
    fn from(value: EvalGraderScoreModel) -> Self {
        Self::EvalGraderScoreModel(value)
    }
}
#[doc = "A ResponsesRunDataSource object describing a model sampling configuration.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalResponsesRunDataSource {
    #[doc = "Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_messages: ::std::option::Option<CreateEvalResponsesRunDataSourceInputMessages>,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sampling_params: ::std::option::Option<CreateEvalResponsesRunDataSourceSamplingParams>,
    #[doc = "Determines what populates the `item` namespace in this run's data source."]
    pub source: CreateEvalResponsesRunDataSourceSource,
    #[doc = "The type of run data source. Always `responses`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalResponsesRunDataSourceType,
}
impl CreateEvalResponsesRunDataSource {
    pub fn builder() -> builder::CreateEvalResponsesRunDataSource {
        Default::default()
    }
}
#[doc = "Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum CreateEvalResponsesRunDataSourceInputMessages {
    #[doc = "InputMessagesTemplate"]
    #[serde(rename = "template")]
    Template {
        #[doc = "A list of chat messages forming the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}."]
        template: ::std::vec::Vec<InputMessagesTemplateTemplateItem>,
    },
    #[doc = "InputMessagesItemReference"]
    #[serde(rename = "item_reference")]
    ItemReference {
        #[doc = "A reference to a variable in the `item` namespace. Ie, \"item.name\""]
        item_reference: ::std::string::String,
    },
}
#[doc = "`CreateEvalResponsesRunDataSourceSamplingParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateEvalResponsesRunDataSourceSamplingParams {
    #[doc = "The maximum number of tokens in the generated output."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    #[doc = "A higher temperature increases randomness in the outputs."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<CreateEvalResponsesRunDataSourceSamplingParamsText>,
    #[doc = "An array of tools the model may call while generating a response. You\ncan specify which tool to use by setting the `tool_choice` parameter.\n\nThe two categories of tools you can provide the model are:\n\n- **Built-in tools**: Tools that are provided by OpenAI that extend the\n  model's capabilities, like [web search](https://developers.openai.com/api/docs/guides/tools-web-search)\n  or [file search](https://developers.openai.com/api/docs/guides/tools-file-search). Learn more about\n  [built-in tools](https://developers.openai.com/api/docs/guides/tools).\n- **Function calls (custom tools)**: Functions that are defined by you,\n  enabling the model to call your own code. Learn more about\n  [function calling](https://developers.openai.com/api/docs/guides/function-calling).\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<::openai_api_types::common::Tool>,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
impl CreateEvalResponsesRunDataSourceSamplingParams {
    pub fn builder() -> builder::CreateEvalResponsesRunDataSourceSamplingParams {
        Default::default()
    }
}
#[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](https://developers.openai.com/api/docs/guides/text)\n- [Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs)\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateEvalResponsesRunDataSourceSamplingParamsText {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::openai_api_types::common::TextResponseFormatConfiguration>,
}
impl CreateEvalResponsesRunDataSourceSamplingParamsText {
    pub fn builder() -> builder::CreateEvalResponsesRunDataSourceSamplingParamsText {
        Default::default()
    }
}
#[doc = "Determines what populates the `item` namespace in this run's data source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceSource {
    JsonlFileContentSource(EvalJsonlFileContentSource),
    JsonlFileIdSource(EvalJsonlFileIdSource),
    ResponsesSource(EvalResponsesSource),
}
impl ::std::convert::From<EvalJsonlFileContentSource> for CreateEvalResponsesRunDataSourceSource {
    fn from(value: EvalJsonlFileContentSource) -> Self {
        Self::JsonlFileContentSource(value)
    }
}
impl ::std::convert::From<EvalJsonlFileIdSource> for CreateEvalResponsesRunDataSourceSource {
    fn from(value: EvalJsonlFileIdSource) -> Self {
        Self::JsonlFileIdSource(value)
    }
}
impl ::std::convert::From<EvalResponsesSource> for CreateEvalResponsesRunDataSourceSource {
    fn from(value: EvalResponsesSource) -> Self {
        Self::ResponsesSource(value)
    }
}
#[doc = "The type of run data source. Always `responses`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalResponsesRunDataSourceType {
    #[serde(rename = "responses")]
    Responses,
}
impl ::std::fmt::Display for CreateEvalResponsesRunDataSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Responses => f.write_str("responses"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalResponsesRunDataSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "responses" => Ok(Self::Responses),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalResponsesRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateEvalResponsesRunDataSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateEvalRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalRunParams {
    pub eval_id: ::std::string::String,
}
impl CreateEvalRunParams {
    pub fn builder() -> builder::CreateEvalRunParams {
        Default::default()
    }
}
#[doc = "`CreateEvalRunRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalRunRequest {
    #[doc = "Details about the run's data source."]
    pub data_source: CreateEvalRunRequestDataSource,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The name of the run."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl CreateEvalRunRequest {
    pub fn builder() -> builder::CreateEvalRunRequest {
        Default::default()
    }
}
#[doc = "Details about the run's data source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateEvalRunRequestDataSource {
    JsonlRunDataSource(CreateEvalJsonlRunDataSource),
    CompletionsRunDataSource(CreateEvalCompletionsRunDataSource),
    ResponsesRunDataSource(CreateEvalResponsesRunDataSource),
}
impl ::std::convert::From<CreateEvalJsonlRunDataSource> for CreateEvalRunRequestDataSource {
    fn from(value: CreateEvalJsonlRunDataSource) -> Self {
        Self::JsonlRunDataSource(value)
    }
}
impl ::std::convert::From<CreateEvalCompletionsRunDataSource> for CreateEvalRunRequestDataSource {
    fn from(value: CreateEvalCompletionsRunDataSource) -> Self {
        Self::CompletionsRunDataSource(value)
    }
}
impl ::std::convert::From<CreateEvalResponsesRunDataSource> for CreateEvalRunRequestDataSource {
    fn from(value: CreateEvalResponsesRunDataSource) -> Self {
        Self::ResponsesRunDataSource(value)
    }
}
#[doc = "Deprecated in favor of LogsDataSourceConfig.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateEvalStoredCompletionsDataSourceConfig {
    #[doc = "Metadata filters for the stored completions data source."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `stored_completions`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalStoredCompletionsDataSourceConfigType,
}
impl CreateEvalStoredCompletionsDataSourceConfig {
    pub fn builder() -> builder::CreateEvalStoredCompletionsDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `stored_completions`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateEvalStoredCompletionsDataSourceConfigType {
    #[serde(rename = "stored_completions")]
    StoredCompletions,
}
impl ::std::fmt::Display for CreateEvalStoredCompletionsDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StoredCompletions => f.write_str("stored_completions"),
        }
    }
}
impl ::std::str::FromStr for CreateEvalStoredCompletionsDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stored_completions" => Ok(Self::StoredCompletions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateEvalStoredCompletionsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateEvalStoredCompletionsDataSourceConfigType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteEvalParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteEvalParams {
    pub eval_id: ::std::string::String,
}
impl DeleteEvalParams {
    pub fn builder() -> builder::DeleteEvalParams {
        Default::default()
    }
}
#[doc = "`DeleteEvalResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteEvalResponse {
    pub deleted: bool,
    pub eval_id: ::std::string::String,
    pub object: ::std::string::String,
}
impl DeleteEvalResponse {
    pub fn builder() -> builder::DeleteEvalResponse {
        Default::default()
    }
}
#[doc = "`DeleteEvalRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteEvalRunParams {
    pub eval_id: ::std::string::String,
    pub run_id: ::std::string::String,
}
impl DeleteEvalRunParams {
    pub fn builder() -> builder::DeleteEvalRunParams {
        Default::default()
    }
}
#[doc = "`DeleteEvalRunResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct DeleteEvalRunResponse {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deleted: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub run_id: ::std::option::Option<::std::string::String>,
}
impl DeleteEvalRunResponse {
    pub fn builder() -> builder::DeleteEvalRunResponse {
        Default::default()
    }
}
#[doc = "An Eval object with a data source config and testing criteria.\nAn Eval represents a task to be done for your LLM integration.\nLike:\n - Improve the quality of my chatbot\n - See how well my chatbot handles customer support\n - Check if o4-mini is better at my usecase than gpt-6-astra\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Eval {
    #[doc = "The Unix timestamp (in seconds) for when the eval was created."]
    pub created_at: i64,
    #[doc = "Configuration of data sources used in runs of the evaluation."]
    pub data_source_config: EvalDataSourceConfig,
    #[doc = "Unique identifier for the evaluation."]
    pub id: ::std::string::String,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The name of the evaluation."]
    pub name: ::std::string::String,
    #[doc = "The object type."]
    pub object: EvalObject,
    #[doc = "A list of testing criteria."]
    pub testing_criteria: ::std::vec::Vec<EvalTestingCriteriaItem>,
}
impl Eval {
    pub fn builder() -> builder::Eval {
        Default::default()
    }
}
#[doc = "An object representing an error response from the Eval API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalApiError {
    #[doc = "The error code."]
    pub code: ::std::string::String,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl EvalApiError {
    pub fn builder() -> builder::EvalApiError {
        Default::default()
    }
}
#[doc = "A CustomDataSourceConfig which specifies the schema of your `item` and optionally `sample` namespaces.\nThe response schema defines the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalCustomDataSourceConfig {
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    pub schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: EvalCustomDataSourceConfigType,
}
impl EvalCustomDataSourceConfig {
    pub fn builder() -> builder::EvalCustomDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `custom`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalCustomDataSourceConfigType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for EvalCustomDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for EvalCustomDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalCustomDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalCustomDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration of data sources used in runs of the evaluation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EvalDataSourceConfig {
    CustomDataSourceConfig(EvalCustomDataSourceConfig),
    LogsDataSourceConfig(EvalLogsDataSourceConfig),
    StoredCompletionsDataSourceConfig(EvalStoredCompletionsDataSourceConfig),
}
impl ::std::convert::From<EvalCustomDataSourceConfig> for EvalDataSourceConfig {
    fn from(value: EvalCustomDataSourceConfig) -> Self {
        Self::CustomDataSourceConfig(value)
    }
}
impl ::std::convert::From<EvalLogsDataSourceConfig> for EvalDataSourceConfig {
    fn from(value: EvalLogsDataSourceConfig) -> Self {
        Self::LogsDataSourceConfig(value)
    }
}
impl ::std::convert::From<EvalStoredCompletionsDataSourceConfig> for EvalDataSourceConfig {
    fn from(value: EvalStoredCompletionsDataSourceConfig) -> Self {
        Self::StoredCompletionsDataSourceConfig(value)
    }
}
#[doc = "`EvalGraderLabelModel`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct EvalGraderLabelModel(pub ::openai_api_types::common::GraderLabelModel);
impl ::std::ops::Deref for EvalGraderLabelModel {
    type Target = ::openai_api_types::common::GraderLabelModel;
    fn deref(&self) -> &::openai_api_types::common::GraderLabelModel {
        &self.0
    }
}
impl ::std::convert::From<EvalGraderLabelModel> for ::openai_api_types::common::GraderLabelModel {
    fn from(value: EvalGraderLabelModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<::openai_api_types::common::GraderLabelModel> for EvalGraderLabelModel {
    fn from(value: ::openai_api_types::common::GraderLabelModel) -> Self {
        Self(value)
    }
}
#[doc = "`EvalGraderPython`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct EvalGraderPython {
    #[doc = "The threshold for the score."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pass_threshold: ::std::option::Option<f64>,
}
impl EvalGraderPython {
    pub fn builder() -> builder::EvalGraderPython {
        Default::default()
    }
}
#[doc = "`EvalGraderScoreModel`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct EvalGraderScoreModel {
    #[doc = "The threshold for the score."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pass_threshold: ::std::option::Option<f64>,
}
impl EvalGraderScoreModel {
    pub fn builder() -> builder::EvalGraderScoreModel {
        Default::default()
    }
}
#[doc = "`EvalGraderStringCheck`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct EvalGraderStringCheck(pub ::openai_api_types::common::GraderStringCheck);
impl ::std::ops::Deref for EvalGraderStringCheck {
    type Target = ::openai_api_types::common::GraderStringCheck;
    fn deref(&self) -> &::openai_api_types::common::GraderStringCheck {
        &self.0
    }
}
impl ::std::convert::From<EvalGraderStringCheck> for ::openai_api_types::common::GraderStringCheck {
    fn from(value: EvalGraderStringCheck) -> Self {
        value.0
    }
}
impl ::std::convert::From<::openai_api_types::common::GraderStringCheck> for EvalGraderStringCheck {
    fn from(value: ::openai_api_types::common::GraderStringCheck) -> Self {
        Self(value)
    }
}
#[doc = "`EvalGraderTextSimilarity`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalGraderTextSimilarity {
    #[doc = "The threshold for the score."]
    pub pass_threshold: f64,
}
impl EvalGraderTextSimilarity {
    pub fn builder() -> builder::EvalGraderTextSimilarity {
        Default::default()
    }
}
#[doc = "`EvalJsonlFileContentSource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalJsonlFileContentSource {
    #[doc = "The content of the jsonl file."]
    pub content: ::std::vec::Vec<EvalJsonlFileContentSourceContentItem>,
    #[doc = "The type of jsonl source. Always `file_content`."]
    #[serde(rename = "type")]
    pub type_: EvalJsonlFileContentSourceType,
}
impl EvalJsonlFileContentSource {
    pub fn builder() -> builder::EvalJsonlFileContentSource {
        Default::default()
    }
}
#[doc = "`EvalJsonlFileContentSourceContentItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalJsonlFileContentSourceContentItem {
    pub item: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub sample: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl EvalJsonlFileContentSourceContentItem {
    pub fn builder() -> builder::EvalJsonlFileContentSourceContentItem {
        Default::default()
    }
}
#[doc = "The type of jsonl source. Always `file_content`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalJsonlFileContentSourceType {
    #[serde(rename = "file_content")]
    FileContent,
}
impl ::std::fmt::Display for EvalJsonlFileContentSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileContent => f.write_str("file_content"),
        }
    }
}
impl ::std::str::FromStr for EvalJsonlFileContentSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_content" => Ok(Self::FileContent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalJsonlFileContentSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalJsonlFileContentSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EvalJsonlFileIdSource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalJsonlFileIdSource {
    #[doc = "The identifier of the file."]
    pub id: ::std::string::String,
    #[doc = "The type of jsonl source. Always `file_id`."]
    #[serde(rename = "type")]
    pub type_: EvalJsonlFileIdSourceType,
}
impl EvalJsonlFileIdSource {
    pub fn builder() -> builder::EvalJsonlFileIdSource {
        Default::default()
    }
}
#[doc = "The type of jsonl source. Always `file_id`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalJsonlFileIdSourceType {
    #[serde(rename = "file_id")]
    FileId,
}
impl ::std::fmt::Display for EvalJsonlFileIdSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileId => f.write_str("file_id"),
        }
    }
}
impl ::std::str::FromStr for EvalJsonlFileIdSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_id" => Ok(Self::FileId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalJsonlFileIdSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalJsonlFileIdSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An object representing a list of evals.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalList {
    #[doc = "An array of eval objects.\n"]
    pub data: ::std::vec::Vec<Eval>,
    #[doc = "The identifier of the first eval in the data array."]
    pub first_id: ::std::string::String,
    #[doc = "Indicates whether there are more evals available."]
    pub has_more: bool,
    #[doc = "The identifier of the last eval in the data array."]
    pub last_id: ::std::string::String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalListObject,
}
impl EvalList {
    pub fn builder() -> builder::EvalList {
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
pub enum EvalListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for EvalListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for EvalListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A LogsDataSourceConfig which specifies the metadata property of your logs query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\nThe schema returned by this data source config is used to defined what variables are available in your evals.\n`item` and `sample` are both defined when using this data source config.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalLogsDataSourceConfig {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    pub schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `logs`."]
    #[serde(rename = "type")]
    pub type_: EvalLogsDataSourceConfigType,
}
impl EvalLogsDataSourceConfig {
    pub fn builder() -> builder::EvalLogsDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `logs`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalLogsDataSourceConfigType {
    #[serde(rename = "logs")]
    Logs,
}
impl ::std::fmt::Display for EvalLogsDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logs => f.write_str("logs"),
        }
    }
}
impl ::std::str::FromStr for EvalLogsDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logs" => Ok(Self::Logs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalLogsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalLogsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalObject {
    #[serde(rename = "eval")]
    Eval,
}
impl ::std::fmt::Display for EvalObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Eval => f.write_str("eval"),
        }
    }
}
impl ::std::str::FromStr for EvalObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eval" => Ok(Self::Eval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A EvalResponsesSource object describing a run data source configuration.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalResponsesSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_after: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_before: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions_search: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[doc = "The type of run data source. Always `responses`."]
    #[serde(rename = "type")]
    pub type_: EvalResponsesSourceType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl EvalResponsesSource {
    pub fn builder() -> builder::EvalResponsesSource {
        Default::default()
    }
}
#[doc = "The type of run data source. Always `responses`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalResponsesSourceType {
    #[serde(rename = "responses")]
    Responses,
}
impl ::std::fmt::Display for EvalResponsesSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Responses => f.write_str("responses"),
        }
    }
}
impl ::std::str::FromStr for EvalResponsesSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "responses" => Ok(Self::Responses),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalResponsesSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalResponsesSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A schema representing an evaluation run.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRun {
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    pub created_at: i64,
    #[doc = "Information about the run's data source."]
    pub data_source: EvalRunDataSource,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<EvalApiError>,
    #[doc = "The identifier of the associated evaluation."]
    pub eval_id: ::std::string::String,
    #[doc = "Unique identifier for the evaluation run."]
    pub id: ::std::string::String,
    pub metadata: ::openai_api_types::common::Metadata,
    #[doc = "The model that is evaluated, if applicable."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the evaluation run."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the object. Always \"eval.run\"."]
    pub object: EvalRunObject,
    #[doc = "Usage statistics for each model during the evaluation run."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub per_model_usage: ::std::option::Option<::std::vec::Vec<EvalRunPerModelUsageItem>>,
    #[doc = "Results per testing criteria applied during the evaluation run."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub per_testing_criteria_results:
        ::std::option::Option<::std::vec::Vec<EvalRunPerTestingCriteriaResultsItem>>,
    #[doc = "The URL to the rendered evaluation run report on the UI dashboard."]
    pub report_url: ::std::string::String,
    pub result_counts: EvalRunResultCounts,
    #[doc = "The status of the evaluation run."]
    pub status: ::std::string::String,
}
impl EvalRun {
    pub fn builder() -> builder::EvalRun {
        Default::default()
    }
}
#[doc = "Information about the run's data source."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EvalRunDataSource {
    JsonlRunDataSource(CreateEvalJsonlRunDataSource),
    CompletionsRunDataSource(CreateEvalCompletionsRunDataSource),
    ResponsesRunDataSource(CreateEvalResponsesRunDataSource),
}
impl ::std::convert::From<CreateEvalJsonlRunDataSource> for EvalRunDataSource {
    fn from(value: CreateEvalJsonlRunDataSource) -> Self {
        Self::JsonlRunDataSource(value)
    }
}
impl ::std::convert::From<CreateEvalCompletionsRunDataSource> for EvalRunDataSource {
    fn from(value: CreateEvalCompletionsRunDataSource) -> Self {
        Self::CompletionsRunDataSource(value)
    }
}
impl ::std::convert::From<CreateEvalResponsesRunDataSource> for EvalRunDataSource {
    fn from(value: CreateEvalResponsesRunDataSource) -> Self {
        Self::ResponsesRunDataSource(value)
    }
}
#[doc = "An object representing a list of runs for an evaluation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunList {
    #[doc = "An array of eval run objects.\n"]
    pub data: ::std::vec::Vec<EvalRun>,
    #[doc = "The identifier of the first eval run in the data array."]
    pub first_id: ::std::string::String,
    #[doc = "Indicates whether there are more evals available."]
    pub has_more: bool,
    #[doc = "The identifier of the last eval run in the data array."]
    pub last_id: ::std::string::String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalRunListObject,
}
impl EvalRunList {
    pub fn builder() -> builder::EvalRunList {
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
pub enum EvalRunListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for EvalRunListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for EvalRunListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalRunListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalRunListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the object. Always \"eval.run\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalRunObject {
    #[serde(rename = "eval.run")]
    EvalRun,
}
impl ::std::fmt::Display for EvalRunObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EvalRun => f.write_str("eval.run"),
        }
    }
}
impl ::std::str::FromStr for EvalRunObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eval.run" => Ok(Self::EvalRun),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalRunObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalRunObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A schema representing an evaluation run output item.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItem {
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    pub created_at: i64,
    #[doc = "Details of the input data source item."]
    pub datasource_item: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The identifier for the data source item."]
    pub datasource_item_id: i64,
    #[doc = "The identifier of the evaluation group."]
    pub eval_id: ::std::string::String,
    #[doc = "Unique identifier for the evaluation run output item."]
    pub id: ::std::string::String,
    #[doc = "The type of the object. Always \"eval.run.output_item\"."]
    pub object: EvalRunOutputItemObject,
    #[doc = "A list of grader results for this output item."]
    pub results: ::std::vec::Vec<EvalRunOutputItemResult>,
    #[doc = "The identifier of the evaluation run associated with this output item."]
    pub run_id: ::std::string::String,
    pub sample: EvalRunOutputItemSample,
    #[doc = "The status of the evaluation run."]
    pub status: ::std::string::String,
}
impl EvalRunOutputItem {
    pub fn builder() -> builder::EvalRunOutputItem {
        Default::default()
    }
}
#[doc = "An object representing a list of output items for an evaluation run.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItemList {
    #[doc = "An array of eval run output item objects.\n"]
    pub data: ::std::vec::Vec<EvalRunOutputItem>,
    #[doc = "The identifier of the first eval run output item in the data array."]
    pub first_id: ::std::string::String,
    #[doc = "Indicates whether there are more eval run output items available."]
    pub has_more: bool,
    #[doc = "The identifier of the last eval run output item in the data array."]
    pub last_id: ::std::string::String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalRunOutputItemListObject,
}
impl EvalRunOutputItemList {
    pub fn builder() -> builder::EvalRunOutputItemList {
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
pub enum EvalRunOutputItemListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for EvalRunOutputItemListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for EvalRunOutputItemListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalRunOutputItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalRunOutputItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the object. Always \"eval.run.output_item\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalRunOutputItemObject {
    #[serde(rename = "eval.run.output_item")]
    EvalRunOutputItem,
}
impl ::std::fmt::Display for EvalRunOutputItemObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EvalRunOutputItem => f.write_str("eval.run.output_item"),
        }
    }
}
impl ::std::str::FromStr for EvalRunOutputItemObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eval.run.output_item" => Ok(Self::EvalRunOutputItem),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalRunOutputItemObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalRunOutputItemObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A single grader result for an evaluation run output item.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItemResult {
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "Whether the grader considered the output a pass."]
    pub passed: bool,
    #[doc = "Optional sample or intermediate data produced by the grader."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sample:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "The numeric score produced by the grader."]
    pub score: f64,
    #[doc = "The grader type (for example, \"string-check-grader\")."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl EvalRunOutputItemResult {
    pub fn builder() -> builder::EvalRunOutputItemResult {
        Default::default()
    }
}
#[doc = "A sample containing the input and output of the evaluation run."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItemSample {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<EvalApiError>,
    #[doc = "The reason why the sample generation was finished."]
    pub finish_reason: ::std::string::String,
    #[doc = "An array of input messages."]
    pub input: ::std::vec::Vec<EvalRunOutputItemSampleInputItem>,
    #[doc = "The maximum number of tokens allowed for completion."]
    pub max_completion_tokens: i64,
    #[doc = "The model used for generating the sample."]
    pub model: ::std::string::String,
    #[doc = "An array of output messages."]
    pub output: ::std::vec::Vec<EvalRunOutputItemSampleOutputItem>,
    #[doc = "The seed used for generating the sample."]
    pub seed: i64,
    #[doc = "The sampling temperature used."]
    pub temperature: f64,
    #[doc = "The top_p value used for sampling."]
    pub top_p: f64,
    pub usage: EvalRunOutputItemSampleUsage,
}
impl EvalRunOutputItemSample {
    pub fn builder() -> builder::EvalRunOutputItemSample {
        Default::default()
    }
}
#[doc = "An input message."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItemSampleInputItem {
    #[doc = "The content of the message."]
    pub content: ::std::string::String,
    #[doc = "The role of the message sender (e.g., system, user, developer)."]
    pub role: ::std::string::String,
}
impl EvalRunOutputItemSampleInputItem {
    pub fn builder() -> builder::EvalRunOutputItemSampleInputItem {
        Default::default()
    }
}
#[doc = "`EvalRunOutputItemSampleOutputItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct EvalRunOutputItemSampleOutputItem {
    #[doc = "The content of the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl EvalRunOutputItemSampleOutputItem {
    pub fn builder() -> builder::EvalRunOutputItemSampleOutputItem {
        Default::default()
    }
}
#[doc = "Token usage details for the sample."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunOutputItemSampleUsage {
    #[doc = "The number of tokens retrieved from cache."]
    pub cached_tokens: i64,
    #[doc = "The number of completion tokens generated."]
    pub completion_tokens: i64,
    #[doc = "The number of prompt tokens used."]
    pub prompt_tokens: i64,
    #[doc = "The total number of tokens used."]
    pub total_tokens: i64,
}
impl EvalRunOutputItemSampleUsage {
    pub fn builder() -> builder::EvalRunOutputItemSampleUsage {
        Default::default()
    }
}
#[doc = "`EvalRunPerModelUsageItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunPerModelUsageItem {
    #[doc = "The number of tokens retrieved from cache."]
    pub cached_tokens: i64,
    #[doc = "The number of completion tokens generated."]
    pub completion_tokens: i64,
    #[doc = "The number of invocations."]
    pub invocation_count: i64,
    #[doc = "The name of the model."]
    pub model_name: ::std::string::String,
    #[doc = "The number of prompt tokens used."]
    pub prompt_tokens: i64,
    #[doc = "The total number of tokens used."]
    pub total_tokens: i64,
}
impl EvalRunPerModelUsageItem {
    pub fn builder() -> builder::EvalRunPerModelUsageItem {
        Default::default()
    }
}
#[doc = "`EvalRunPerTestingCriteriaResultsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunPerTestingCriteriaResultsItem {
    #[doc = "Number of tests failed for this criteria."]
    pub failed: i64,
    #[doc = "Number of tests passed for this criteria."]
    pub passed: i64,
    #[doc = "A description of the testing criteria."]
    pub testing_criteria: ::std::string::String,
}
impl EvalRunPerTestingCriteriaResultsItem {
    pub fn builder() -> builder::EvalRunPerTestingCriteriaResultsItem {
        Default::default()
    }
}
#[doc = "Counters summarizing the outcomes of the evaluation run."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalRunResultCounts {
    #[doc = "Number of output items that resulted in an error."]
    pub errored: i64,
    #[doc = "Number of output items that failed to pass the evaluation."]
    pub failed: i64,
    #[doc = "Number of output items that passed the evaluation."]
    pub passed: i64,
    #[doc = "Total number of executed output items."]
    pub total: i64,
}
impl EvalRunResultCounts {
    pub fn builder() -> builder::EvalRunResultCounts {
        Default::default()
    }
}
#[doc = "Deprecated in favor of LogsDataSourceConfig.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalStoredCompletionsDataSourceConfig {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    pub schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of data source. Always `stored_completions`."]
    #[serde(rename = "type")]
    pub type_: EvalStoredCompletionsDataSourceConfigType,
}
impl EvalStoredCompletionsDataSourceConfig {
    pub fn builder() -> builder::EvalStoredCompletionsDataSourceConfig {
        Default::default()
    }
}
#[doc = "The type of data source. Always `stored_completions`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalStoredCompletionsDataSourceConfigType {
    #[serde(rename = "stored_completions")]
    StoredCompletions,
}
impl ::std::fmt::Display for EvalStoredCompletionsDataSourceConfigType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StoredCompletions => f.write_str("stored_completions"),
        }
    }
}
impl ::std::str::FromStr for EvalStoredCompletionsDataSourceConfigType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stored_completions" => Ok(Self::StoredCompletions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalStoredCompletionsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalStoredCompletionsDataSourceConfigType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A StoredCompletionsRunDataSource configuration describing a set of filters\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalStoredCompletionsSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_after: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_before: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The type of source. Always `stored_completions`."]
    #[serde(rename = "type")]
    pub type_: EvalStoredCompletionsSourceType,
}
impl EvalStoredCompletionsSource {
    pub fn builder() -> builder::EvalStoredCompletionsSource {
        Default::default()
    }
}
#[doc = "The type of source. Always `stored_completions`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalStoredCompletionsSourceType {
    #[serde(rename = "stored_completions")]
    StoredCompletions,
}
impl ::std::fmt::Display for EvalStoredCompletionsSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StoredCompletions => f.write_str("stored_completions"),
        }
    }
}
impl ::std::str::FromStr for EvalStoredCompletionsSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stored_completions" => Ok(Self::StoredCompletions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalStoredCompletionsSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalStoredCompletionsSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EvalTestingCriteriaItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EvalTestingCriteriaItem {
    LabelModel(EvalGraderLabelModel),
    StringCheck(EvalGraderStringCheck),
    TextSimilarity(EvalGraderTextSimilarity),
    Python(EvalGraderPython),
    ScoreModel(EvalGraderScoreModel),
}
impl ::std::convert::From<EvalGraderLabelModel> for EvalTestingCriteriaItem {
    fn from(value: EvalGraderLabelModel) -> Self {
        Self::LabelModel(value)
    }
}
impl ::std::convert::From<EvalGraderStringCheck> for EvalTestingCriteriaItem {
    fn from(value: EvalGraderStringCheck) -> Self {
        Self::StringCheck(value)
    }
}
impl ::std::convert::From<EvalGraderTextSimilarity> for EvalTestingCriteriaItem {
    fn from(value: EvalGraderTextSimilarity) -> Self {
        Self::TextSimilarity(value)
    }
}
impl ::std::convert::From<EvalGraderPython> for EvalTestingCriteriaItem {
    fn from(value: EvalGraderPython) -> Self {
        Self::Python(value)
    }
}
impl ::std::convert::From<EvalGraderScoreModel> for EvalTestingCriteriaItem {
    fn from(value: EvalGraderScoreModel) -> Self {
        Self::ScoreModel(value)
    }
}
#[doc = "`EvalsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalsApiSchema {
    #[serde(rename = "CancelEvalRunError429")]
    pub cancel_eval_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CancelEvalRunParams")]
    pub cancel_eval_run_params: CancelEvalRunParams,
    #[serde(rename = "CancelEvalRunResponse")]
    pub cancel_eval_run_response: EvalRun,
    #[serde(rename = "CreateEvalError400")]
    pub create_eval_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalError429")]
    pub create_eval_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalRequest")]
    pub create_eval_request: CreateEvalRequest,
    #[serde(rename = "CreateEvalResponse")]
    pub create_eval_response: Eval,
    #[serde(rename = "CreateEvalRunError400")]
    pub create_eval_run_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalRunError404")]
    pub create_eval_run_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalRunError429")]
    pub create_eval_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalRunError500")]
    pub create_eval_run_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateEvalRunParams")]
    pub create_eval_run_params: CreateEvalRunParams,
    #[serde(rename = "CreateEvalRunRequest")]
    pub create_eval_run_request: CreateEvalRunRequest,
    #[serde(rename = "CreateEvalRunResponse")]
    pub create_eval_run_response: EvalRun,
    #[serde(rename = "DeleteEvalError400")]
    pub delete_eval_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalError404")]
    pub delete_eval_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalError429")]
    pub delete_eval_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalParams")]
    pub delete_eval_params: DeleteEvalParams,
    #[serde(rename = "DeleteEvalResponse")]
    pub delete_eval_response: DeleteEvalResponse,
    #[serde(rename = "DeleteEvalRunError400")]
    pub delete_eval_run_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalRunError404")]
    pub delete_eval_run_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalRunError429")]
    pub delete_eval_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteEvalRunParams")]
    pub delete_eval_run_params: DeleteEvalRunParams,
    #[serde(rename = "DeleteEvalRunResponse")]
    pub delete_eval_run_response: DeleteEvalRunResponse,
    #[serde(rename = "GetEvalError404")]
    pub get_eval_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalError429")]
    pub get_eval_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalParams")]
    pub get_eval_params: GetEvalParams,
    #[serde(rename = "GetEvalResponse")]
    pub get_eval_response: Eval,
    #[serde(rename = "GetEvalRunError400")]
    pub get_eval_run_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunError404")]
    pub get_eval_run_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunError429")]
    pub get_eval_run_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemError400")]
    pub get_eval_run_output_item_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemError404")]
    pub get_eval_run_output_item_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemError429")]
    pub get_eval_run_output_item_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemParams")]
    pub get_eval_run_output_item_params: GetEvalRunOutputItemParams,
    #[serde(rename = "GetEvalRunOutputItemResponse")]
    pub get_eval_run_output_item_response: EvalRunOutputItem,
    #[serde(rename = "GetEvalRunOutputItemsError400")]
    pub get_eval_run_output_items_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemsError404")]
    pub get_eval_run_output_items_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemsError429")]
    pub get_eval_run_output_items_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunOutputItemsParams")]
    pub get_eval_run_output_items_params: GetEvalRunOutputItemsParams,
    #[serde(rename = "GetEvalRunOutputItemsResponse")]
    pub get_eval_run_output_items_response: EvalRunOutputItemList,
    #[serde(rename = "GetEvalRunParams")]
    pub get_eval_run_params: GetEvalRunParams,
    #[serde(rename = "GetEvalRunResponse")]
    pub get_eval_run_response: EvalRun,
    #[serde(rename = "GetEvalRunsError400")]
    pub get_eval_runs_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunsError404")]
    pub get_eval_runs_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunsError429")]
    pub get_eval_runs_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetEvalRunsParams")]
    pub get_eval_runs_params: GetEvalRunsParams,
    #[serde(rename = "GetEvalRunsResponse")]
    pub get_eval_runs_response: EvalRunList,
    #[serde(rename = "ListEvalsError429")]
    pub list_evals_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListEvalsParams")]
    pub list_evals_params: ListEvalsParams,
    #[serde(rename = "ListEvalsResponse")]
    pub list_evals_response: EvalList,
    #[serde(rename = "UpdateEvalError400")]
    pub update_eval_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateEvalError404")]
    pub update_eval_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateEvalError429")]
    pub update_eval_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateEvalParams")]
    pub update_eval_params: UpdateEvalParams,
    #[serde(rename = "UpdateEvalRequest")]
    pub update_eval_request: UpdateEvalRequest,
    #[serde(rename = "UpdateEvalResponse")]
    pub update_eval_response: Eval,
}
impl EvalsApiSchema {
    pub fn builder() -> builder::EvalsApiSchema {
        Default::default()
    }
}
#[doc = "`GetEvalParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetEvalParams {
    pub eval_id: ::std::string::String,
}
impl GetEvalParams {
    pub fn builder() -> builder::GetEvalParams {
        Default::default()
    }
}
#[doc = "`GetEvalRunOutputItemParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetEvalRunOutputItemParams {
    pub eval_id: ::std::string::String,
    pub output_item_id: ::std::string::String,
    pub run_id: ::std::string::String,
}
impl GetEvalRunOutputItemParams {
    pub fn builder() -> builder::GetEvalRunOutputItemParams {
        Default::default()
    }
}
#[doc = "`GetEvalRunOutputItemsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetEvalRunOutputItemsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub eval_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<GetEvalRunOutputItemsParamsOrder>,
    pub run_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<GetEvalRunOutputItemsParamsStatus>,
}
impl GetEvalRunOutputItemsParams {
    pub fn builder() -> builder::GetEvalRunOutputItemsParams {
        Default::default()
    }
}
#[doc = "`GetEvalRunOutputItemsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GetEvalRunOutputItemsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for GetEvalRunOutputItemsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for GetEvalRunOutputItemsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetEvalRunOutputItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetEvalRunOutputItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetEvalRunOutputItemsParamsStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GetEvalRunOutputItemsParamsStatus {
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "pass")]
    Pass,
}
impl ::std::fmt::Display for GetEvalRunOutputItemsParamsStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fail => f.write_str("fail"),
            Self::Pass => f.write_str("pass"),
        }
    }
}
impl ::std::str::FromStr for GetEvalRunOutputItemsParamsStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fail" => Ok(Self::Fail),
            "pass" => Ok(Self::Pass),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetEvalRunOutputItemsParamsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetEvalRunOutputItemsParamsStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetEvalRunParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetEvalRunParams {
    pub eval_id: ::std::string::String,
    pub run_id: ::std::string::String,
}
impl GetEvalRunParams {
    pub fn builder() -> builder::GetEvalRunParams {
        Default::default()
    }
}
#[doc = "`GetEvalRunsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetEvalRunsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub eval_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<GetEvalRunsParamsOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<GetEvalRunsParamsStatus>,
}
impl GetEvalRunsParams {
    pub fn builder() -> builder::GetEvalRunsParams {
        Default::default()
    }
}
#[doc = "`GetEvalRunsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GetEvalRunsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for GetEvalRunsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for GetEvalRunsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetEvalRunsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetEvalRunsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetEvalRunsParamsStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GetEvalRunsParamsStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for GetEvalRunsParamsStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Canceled => f.write_str("canceled"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for GetEvalRunsParamsStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetEvalRunsParamsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetEvalRunsParamsStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InputMessagesTemplateTemplateItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum InputMessagesTemplateTemplateItem {
    Variant0 {
        #[doc = "The content of the message."]
        content: ::std::string::String,
        #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
        role: ::std::string::String,
    },
    Variant1(::openai_api_types::common::EvalItem),
}
impl ::std::convert::From<::openai_api_types::common::EvalItem>
    for InputMessagesTemplateTemplateItem
{
    fn from(value: ::openai_api_types::common::EvalItem) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`ListEvalsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListEvalsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListEvalsParamsOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order_by: ::std::option::Option<ListEvalsParamsOrderBy>,
}
impl ListEvalsParams {
    pub fn builder() -> builder::ListEvalsParams {
        Default::default()
    }
}
#[doc = "`ListEvalsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListEvalsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListEvalsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListEvalsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListEvalsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListEvalsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListEvalsParamsOrderBy`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListEvalsParamsOrderBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
impl ::std::fmt::Display for ListEvalsParamsOrderBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreatedAt => f.write_str("created_at"),
            Self::UpdatedAt => f.write_str("updated_at"),
        }
    }
}
impl ::std::str::FromStr for ListEvalsParamsOrderBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListEvalsParamsOrderBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListEvalsParamsOrderBy {
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
#[doc = "`TemplateInputMessagesTemplateItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TemplateInputMessagesTemplateItem {
    Variant0(::openai_api_types::common::EasyInputMessage),
    Variant1(::openai_api_types::common::EvalItem),
}
impl ::std::convert::From<::openai_api_types::common::EasyInputMessage>
    for TemplateInputMessagesTemplateItem
{
    fn from(value: ::openai_api_types::common::EasyInputMessage) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::EvalItem>
    for TemplateInputMessagesTemplateItem
{
    fn from(value: ::openai_api_types::common::EvalItem) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`UpdateEvalParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateEvalParams {
    pub eval_id: ::std::string::String,
}
impl UpdateEvalParams {
    pub fn builder() -> builder::UpdateEvalParams {
        Default::default()
    }
}
#[doc = "`UpdateEvalRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateEvalRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::openai_api_types::common::Metadata>,
    #[doc = "Rename the evaluation."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl UpdateEvalRequest {
    pub fn builder() -> builder::UpdateEvalRequest {
        Default::default()
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
    pub struct CancelEvalRunParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelEvalRunParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
            }
        }
    }
    impl CancelEvalRunParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<CancelEvalRunParams> for super::CancelEvalRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelEvalRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
                run_id: value.run_id?,
            })
        }
    }
    impl ::std::convert::From<super::CancelEvalRunParams> for CancelEvalRunParams {
        fn from(value: super::CancelEvalRunParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
                run_id: Ok(value.run_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalCompletionsRunDataSource {
        input_messages: ::std::result::Result<
            ::std::option::Option<super::CreateEvalCompletionsRunDataSourceInputMessages>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sampling_params: ::std::result::Result<
            ::std::option::Option<super::CreateEvalCompletionsRunDataSourceSamplingParams>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            super::CreateEvalCompletionsRunDataSourceSource,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateEvalCompletionsRunDataSourceType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalCompletionsRunDataSource {
        fn default() -> Self {
            Self {
                input_messages: Ok(Default::default()),
                model: Ok(Default::default()),
                sampling_params: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalCompletionsRunDataSource {
        pub fn input_messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEvalCompletionsRunDataSourceInputMessages>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input_messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_messages: {e}"));
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
        pub fn sampling_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEvalCompletionsRunDataSourceSamplingParams>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sampling_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling_params: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalCompletionsRunDataSourceSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalCompletionsRunDataSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalCompletionsRunDataSource>
        for super::CreateEvalCompletionsRunDataSource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalCompletionsRunDataSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_messages: value.input_messages?,
                model: value.model?,
                sampling_params: value.sampling_params?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalCompletionsRunDataSource>
        for CreateEvalCompletionsRunDataSource
    {
        fn from(value: super::CreateEvalCompletionsRunDataSource) -> Self {
            Self {
                input_messages: Ok(value.input_messages),
                model: Ok(value.model),
                sampling_params: Ok(value.sampling_params),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        response_format: ::std::result::Result<
            ::std::option::Option<
                super::CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat,
            >,
            ::std::string::String,
        >,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tools: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::ChatCompletionTool>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalCompletionsRunDataSourceSamplingParams {
        fn default() -> Self {
            Self {
                max_completion_tokens: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                response_format: Ok(Default::default()),
                seed: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl CreateEvalCompletionsRunDataSourceSamplingParams {
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
                ::std::option::Option<
                    super::CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.response_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_format: {e}"));
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
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<::openai_api_types::common::ChatCompletionTool>,
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
    }
    impl ::std::convert::TryFrom<CreateEvalCompletionsRunDataSourceSamplingParams>
        for super::CreateEvalCompletionsRunDataSourceSamplingParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalCompletionsRunDataSourceSamplingParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_completion_tokens: value.max_completion_tokens?,
                reasoning_effort: value.reasoning_effort?,
                response_format: value.response_format?,
                seed: value.seed?,
                temperature: value.temperature?,
                tools: value.tools?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalCompletionsRunDataSourceSamplingParams>
        for CreateEvalCompletionsRunDataSourceSamplingParams
    {
        fn from(value: super::CreateEvalCompletionsRunDataSourceSamplingParams) -> Self {
            Self {
                max_completion_tokens: Ok(value.max_completion_tokens),
                reasoning_effort: Ok(value.reasoning_effort),
                response_format: Ok(value.response_format),
                seed: Ok(value.seed),
                temperature: Ok(value.temperature),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalCustomDataSourceConfig {
        include_sample_schema:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        item_schema: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateEvalCustomDataSourceConfigType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalCustomDataSourceConfig {
        fn default() -> Self {
            Self {
                include_sample_schema: Ok(Default::default()),
                item_schema: Err("no value supplied for item_schema".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalCustomDataSourceConfig {
        pub fn include_sample_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.include_sample_schema = value.try_into().map_err(|e| {
                format!("error converting supplied value for include_sample_schema: {e}")
            });
            self
        }
        pub fn item_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.item_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalCustomDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalCustomDataSourceConfig>
        for super::CreateEvalCustomDataSourceConfig
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalCustomDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                include_sample_schema: value.include_sample_schema?,
                item_schema: value.item_schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalCustomDataSourceConfig>
        for CreateEvalCustomDataSourceConfig
    {
        fn from(value: super::CreateEvalCustomDataSourceConfig) -> Self {
            Self {
                include_sample_schema: Ok(value.include_sample_schema),
                item_schema: Ok(value.item_schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalJsonlRunDataSource {
        source:
            ::std::result::Result<super::CreateEvalJsonlRunDataSourceSource, ::std::string::String>,
        type_:
            ::std::result::Result<super::CreateEvalJsonlRunDataSourceType, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalJsonlRunDataSource {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalJsonlRunDataSource {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalJsonlRunDataSourceSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalJsonlRunDataSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalJsonlRunDataSource> for super::CreateEvalJsonlRunDataSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalJsonlRunDataSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalJsonlRunDataSource> for CreateEvalJsonlRunDataSource {
        fn from(value: super::CreateEvalJsonlRunDataSource) -> Self {
            Self {
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalLabelModelGrader {
        input: ::std::result::Result<::std::vec::Vec<super::CreateEvalItem>, ::std::string::String>,
        labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        passing_labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<super::CreateEvalLabelModelGraderType, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalLabelModelGrader {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                labels: Err("no value supplied for labels".to_string()),
                model: Err("no value supplied for model".to_string()),
                name: Err("no value supplied for name".to_string()),
                passing_labels: Err("no value supplied for passing_labels".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalLabelModelGrader {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateEvalItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labels: {e}"));
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn passing_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.passing_labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for passing_labels: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalLabelModelGraderType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalLabelModelGrader> for super::CreateEvalLabelModelGrader {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalLabelModelGrader,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                labels: value.labels?,
                model: value.model?,
                name: value.name?,
                passing_labels: value.passing_labels?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalLabelModelGrader> for CreateEvalLabelModelGrader {
        fn from(value: super::CreateEvalLabelModelGrader) -> Self {
            Self {
                input: Ok(value.input),
                labels: Ok(value.labels),
                model: Ok(value.model),
                name: Ok(value.name),
                passing_labels: Ok(value.passing_labels),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalLogsDataSourceConfig {
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_:
            ::std::result::Result<super::CreateEvalLogsDataSourceConfigType, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalLogsDataSourceConfig {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalLogsDataSourceConfig {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalLogsDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalLogsDataSourceConfig>
        for super::CreateEvalLogsDataSourceConfig
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalLogsDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalLogsDataSourceConfig>
        for CreateEvalLogsDataSourceConfig
    {
        fn from(value: super::CreateEvalLogsDataSourceConfig) -> Self {
            Self {
                metadata: Ok(value.metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalRequest {
        data_source_config:
            ::std::result::Result<super::CreateEvalRequestDataSourceConfig, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        testing_criteria: ::std::result::Result<
            ::std::vec::Vec<super::CreateEvalRequestTestingCriteriaItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalRequest {
        fn default() -> Self {
            Self {
                data_source_config: Err("no value supplied for data_source_config".to_string()),
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
                testing_criteria: Err("no value supplied for testing_criteria".to_string()),
            }
        }
    }
    impl CreateEvalRequest {
        pub fn data_source_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalRequestDataSourceConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.data_source_config = value.try_into().map_err(|e| {
                format!("error converting supplied value for data_source_config: {e}")
            });
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
        pub fn testing_criteria<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CreateEvalRequestTestingCriteriaItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.testing_criteria = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for testing_criteria: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalRequest> for super::CreateEvalRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data_source_config: value.data_source_config?,
                metadata: value.metadata?,
                name: value.name?,
                testing_criteria: value.testing_criteria?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalRequest> for CreateEvalRequest {
        fn from(value: super::CreateEvalRequest) -> Self {
            Self {
                data_source_config: Ok(value.data_source_config),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                testing_criteria: Ok(value.testing_criteria),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalResponsesRunDataSource {
        input_messages: ::std::result::Result<
            ::std::option::Option<super::CreateEvalResponsesRunDataSourceInputMessages>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sampling_params: ::std::result::Result<
            ::std::option::Option<super::CreateEvalResponsesRunDataSourceSamplingParams>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            super::CreateEvalResponsesRunDataSourceSource,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateEvalResponsesRunDataSourceType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalResponsesRunDataSource {
        fn default() -> Self {
            Self {
                input_messages: Ok(Default::default()),
                model: Ok(Default::default()),
                sampling_params: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalResponsesRunDataSource {
        pub fn input_messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEvalResponsesRunDataSourceInputMessages>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input_messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_messages: {e}"));
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
        pub fn sampling_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEvalResponsesRunDataSourceSamplingParams>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sampling_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling_params: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalResponsesRunDataSourceSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalResponsesRunDataSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalResponsesRunDataSource>
        for super::CreateEvalResponsesRunDataSource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalResponsesRunDataSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_messages: value.input_messages?,
                model: value.model?,
                sampling_params: value.sampling_params?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalResponsesRunDataSource>
        for CreateEvalResponsesRunDataSource
    {
        fn from(value: super::CreateEvalResponsesRunDataSource) -> Self {
            Self {
                input_messages: Ok(value.input_messages),
                model: Ok(value.model),
                sampling_params: Ok(value.sampling_params),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalResponsesRunDataSourceSamplingParams {
        max_completion_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<super::CreateEvalResponsesRunDataSourceSamplingParamsText>,
            ::std::string::String,
        >,
        tools: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::Tool>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalResponsesRunDataSourceSamplingParams {
        fn default() -> Self {
            Self {
                max_completion_tokens: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                seed: Ok(Default::default()),
                temperature: Ok(Default::default()),
                text: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl CreateEvalResponsesRunDataSourceSamplingParams {
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
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateEvalResponsesRunDataSourceSamplingParamsText>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::Tool>>,
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
    impl ::std::convert::TryFrom<CreateEvalResponsesRunDataSourceSamplingParams>
        for super::CreateEvalResponsesRunDataSourceSamplingParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalResponsesRunDataSourceSamplingParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_completion_tokens: value.max_completion_tokens?,
                reasoning_effort: value.reasoning_effort?,
                seed: value.seed?,
                temperature: value.temperature?,
                text: value.text?,
                tools: value.tools?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalResponsesRunDataSourceSamplingParams>
        for CreateEvalResponsesRunDataSourceSamplingParams
    {
        fn from(value: super::CreateEvalResponsesRunDataSourceSamplingParams) -> Self {
            Self {
                max_completion_tokens: Ok(value.max_completion_tokens),
                reasoning_effort: Ok(value.reasoning_effort),
                seed: Ok(value.seed),
                temperature: Ok(value.temperature),
                text: Ok(value.text),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalResponsesRunDataSourceSamplingParamsText {
        format: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::TextResponseFormatConfiguration>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalResponsesRunDataSourceSamplingParamsText {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
            }
        }
    }
    impl CreateEvalResponsesRunDataSourceSamplingParamsText {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::TextResponseFormatConfiguration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalResponsesRunDataSourceSamplingParamsText>
        for super::CreateEvalResponsesRunDataSourceSamplingParamsText
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalResponsesRunDataSourceSamplingParamsText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalResponsesRunDataSourceSamplingParamsText>
        for CreateEvalResponsesRunDataSourceSamplingParamsText
    {
        fn from(value: super::CreateEvalResponsesRunDataSourceSamplingParamsText) -> Self {
            Self {
                format: Ok(value.format),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalRunParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateEvalRunParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
            }
        }
    }
    impl CreateEvalRunParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalRunParams> for super::CreateEvalRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalRunParams> for CreateEvalRunParams {
        fn from(value: super::CreateEvalRunParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalRunRequest {
        data_source:
            ::std::result::Result<super::CreateEvalRunRequestDataSource, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalRunRequest {
        fn default() -> Self {
            Self {
                data_source: Err("no value supplied for data_source".to_string()),
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl CreateEvalRunRequest {
        pub fn data_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalRunRequestDataSource>,
            T::Error: ::std::fmt::Display,
        {
            self.data_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_source: {e}"));
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
    impl ::std::convert::TryFrom<CreateEvalRunRequest> for super::CreateEvalRunRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalRunRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data_source: value.data_source?,
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalRunRequest> for CreateEvalRunRequest {
        fn from(value: super::CreateEvalRunRequest) -> Self {
            Self {
                data_source: Ok(value.data_source),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateEvalStoredCompletionsDataSourceConfig {
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateEvalStoredCompletionsDataSourceConfigType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateEvalStoredCompletionsDataSourceConfig {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateEvalStoredCompletionsDataSourceConfig {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalStoredCompletionsDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateEvalStoredCompletionsDataSourceConfig>
        for super::CreateEvalStoredCompletionsDataSourceConfig
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateEvalStoredCompletionsDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateEvalStoredCompletionsDataSourceConfig>
        for CreateEvalStoredCompletionsDataSourceConfig
    {
        fn from(value: super::CreateEvalStoredCompletionsDataSourceConfig) -> Self {
            Self {
                metadata: Ok(value.metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteEvalParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteEvalParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
            }
        }
    }
    impl DeleteEvalParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteEvalParams> for super::DeleteEvalParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteEvalParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteEvalParams> for DeleteEvalParams {
        fn from(value: super::DeleteEvalParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteEvalResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteEvalResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                eval_id: Err("no value supplied for eval_id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteEvalResponse {
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
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
    impl ::std::convert::TryFrom<DeleteEvalResponse> for super::DeleteEvalResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteEvalResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                eval_id: value.eval_id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteEvalResponse> for DeleteEvalResponse {
        fn from(value: super::DeleteEvalResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                eval_id: Ok(value.eval_id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteEvalRunParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteEvalRunParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
            }
        }
    }
    impl DeleteEvalRunParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<DeleteEvalRunParams> for super::DeleteEvalRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteEvalRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
                run_id: value.run_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteEvalRunParams> for DeleteEvalRunParams {
        fn from(value: super::DeleteEvalRunParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
                run_id: Ok(value.run_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteEvalRunResponse {
        deleted: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        object: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteEvalRunResponse {
        fn default() -> Self {
            Self {
                deleted: Ok(Default::default()),
                object: Ok(Default::default()),
                run_id: Ok(Default::default()),
            }
        }
    }
    impl DeleteEvalRunResponse {
        pub fn deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted: {e}"));
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
    }
    impl ::std::convert::TryFrom<DeleteEvalRunResponse> for super::DeleteEvalRunResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteEvalRunResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
                run_id: value.run_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteEvalRunResponse> for DeleteEvalRunResponse {
        fn from(value: super::DeleteEvalRunResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
                run_id: Ok(value.run_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Eval {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        data_source_config:
            ::std::result::Result<super::EvalDataSourceConfig, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::EvalObject, ::std::string::String>,
        testing_criteria: ::std::result::Result<
            ::std::vec::Vec<super::EvalTestingCriteriaItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Eval {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data_source_config: Err("no value supplied for data_source_config".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                testing_criteria: Err("no value supplied for testing_criteria".to_string()),
            }
        }
    }
    impl Eval {
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
        pub fn data_source_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalDataSourceConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.data_source_config = value.try_into().map_err(|e| {
                format!("error converting supplied value for data_source_config: {e}")
            });
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
            T: ::std::convert::TryInto<super::EvalObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn testing_criteria<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalTestingCriteriaItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.testing_criteria = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for testing_criteria: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Eval> for super::Eval {
        type Error = super::error::ConversionError;
        fn try_from(value: Eval) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data_source_config: value.data_source_config?,
                id: value.id?,
                metadata: value.metadata?,
                name: value.name?,
                object: value.object?,
                testing_criteria: value.testing_criteria?,
            })
        }
    }
    impl ::std::convert::From<super::Eval> for Eval {
        fn from(value: super::Eval) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data_source_config: Ok(value.data_source_config),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                object: Ok(value.object),
                testing_criteria: Ok(value.testing_criteria),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalApiError {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EvalApiError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl EvalApiError {
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
    }
    impl ::std::convert::TryFrom<EvalApiError> for super::EvalApiError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalApiError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::EvalApiError> for EvalApiError {
        fn from(value: super::EvalApiError) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalCustomDataSourceConfig {
        schema: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EvalCustomDataSourceConfigType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalCustomDataSourceConfig {
        fn default() -> Self {
            Self {
                schema: Err("no value supplied for schema".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalCustomDataSourceConfig {
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalCustomDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalCustomDataSourceConfig> for super::EvalCustomDataSourceConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalCustomDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                schema: value.schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalCustomDataSourceConfig> for EvalCustomDataSourceConfig {
        fn from(value: super::EvalCustomDataSourceConfig) -> Self {
            Self {
                schema: Ok(value.schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalGraderPython {
        pass_threshold: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for EvalGraderPython {
        fn default() -> Self {
            Self {
                pass_threshold: Ok(Default::default()),
            }
        }
    }
    impl EvalGraderPython {
        pub fn pass_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pass_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pass_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalGraderPython> for super::EvalGraderPython {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalGraderPython,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pass_threshold: value.pass_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::EvalGraderPython> for EvalGraderPython {
        fn from(value: super::EvalGraderPython) -> Self {
            Self {
                pass_threshold: Ok(value.pass_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalGraderScoreModel {
        pass_threshold: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for EvalGraderScoreModel {
        fn default() -> Self {
            Self {
                pass_threshold: Ok(Default::default()),
            }
        }
    }
    impl EvalGraderScoreModel {
        pub fn pass_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pass_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pass_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalGraderScoreModel> for super::EvalGraderScoreModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalGraderScoreModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pass_threshold: value.pass_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::EvalGraderScoreModel> for EvalGraderScoreModel {
        fn from(value: super::EvalGraderScoreModel) -> Self {
            Self {
                pass_threshold: Ok(value.pass_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalGraderTextSimilarity {
        pass_threshold: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for EvalGraderTextSimilarity {
        fn default() -> Self {
            Self {
                pass_threshold: Err("no value supplied for pass_threshold".to_string()),
            }
        }
    }
    impl EvalGraderTextSimilarity {
        pub fn pass_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.pass_threshold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pass_threshold: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalGraderTextSimilarity> for super::EvalGraderTextSimilarity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalGraderTextSimilarity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pass_threshold: value.pass_threshold?,
            })
        }
    }
    impl ::std::convert::From<super::EvalGraderTextSimilarity> for EvalGraderTextSimilarity {
        fn from(value: super::EvalGraderTextSimilarity) -> Self {
            Self {
                pass_threshold: Ok(value.pass_threshold),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalJsonlFileContentSource {
        content: ::std::result::Result<
            ::std::vec::Vec<super::EvalJsonlFileContentSourceContentItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EvalJsonlFileContentSourceType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalJsonlFileContentSource {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalJsonlFileContentSource {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::EvalJsonlFileContentSourceContentItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalJsonlFileContentSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalJsonlFileContentSource> for super::EvalJsonlFileContentSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalJsonlFileContentSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalJsonlFileContentSource> for EvalJsonlFileContentSource {
        fn from(value: super::EvalJsonlFileContentSource) -> Self {
            Self {
                content: Ok(value.content),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalJsonlFileContentSourceContentItem {
        item: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        sample: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalJsonlFileContentSourceContentItem {
        fn default() -> Self {
            Self {
                item: Err("no value supplied for item".to_string()),
                sample: Ok(Default::default()),
            }
        }
    }
    impl EvalJsonlFileContentSourceContentItem {
        pub fn item<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item: {e}"));
            self
        }
        pub fn sample<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sample = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sample: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalJsonlFileContentSourceContentItem>
        for super::EvalJsonlFileContentSourceContentItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalJsonlFileContentSourceContentItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                item: value.item?,
                sample: value.sample?,
            })
        }
    }
    impl ::std::convert::From<super::EvalJsonlFileContentSourceContentItem>
        for EvalJsonlFileContentSourceContentItem
    {
        fn from(value: super::EvalJsonlFileContentSourceContentItem) -> Self {
            Self {
                item: Ok(value.item),
                sample: Ok(value.sample),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalJsonlFileIdSource {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::EvalJsonlFileIdSourceType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalJsonlFileIdSource {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalJsonlFileIdSource {
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
            T: ::std::convert::TryInto<super::EvalJsonlFileIdSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalJsonlFileIdSource> for super::EvalJsonlFileIdSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalJsonlFileIdSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalJsonlFileIdSource> for EvalJsonlFileIdSource {
        fn from(value: super::EvalJsonlFileIdSource) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalList {
        data: ::std::result::Result<::std::vec::Vec<super::Eval>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::EvalListObject, ::std::string::String>,
    }
    impl ::std::default::Default for EvalList {
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
    impl EvalList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Eval>>,
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
            T: ::std::convert::TryInto<super::EvalListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalList> for super::EvalList {
        type Error = super::error::ConversionError;
        fn try_from(value: EvalList) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                first_id: value.first_id?,
                has_more: value.has_more?,
                last_id: value.last_id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::EvalList> for EvalList {
        fn from(value: super::EvalList) -> Self {
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
    pub struct EvalLogsDataSourceConfig {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        schema: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EvalLogsDataSourceConfigType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalLogsDataSourceConfig {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                schema: Err("no value supplied for schema".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalLogsDataSourceConfig {
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
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalLogsDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalLogsDataSourceConfig> for super::EvalLogsDataSourceConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalLogsDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                schema: value.schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalLogsDataSourceConfig> for EvalLogsDataSourceConfig {
        fn from(value: super::EvalLogsDataSourceConfig) -> Self {
            Self {
                metadata: Ok(value.metadata),
                schema: Ok(value.schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalResponsesSource {
        created_after: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        created_before: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        instructions_search: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ReasoningEffort>,
            ::std::string::String,
        >,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        tools: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<super::EvalResponsesSourceType, ::std::string::String>,
        users: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalResponsesSource {
        fn default() -> Self {
            Self {
                created_after: Ok(Default::default()),
                created_before: Ok(Default::default()),
                instructions_search: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                temperature: Ok(Default::default()),
                tools: Ok(Default::default()),
                top_p: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                users: Ok(Default::default()),
            }
        }
    }
    impl EvalResponsesSource {
        pub fn created_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_after: {e}"));
            self
        }
        pub fn created_before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_before: {e}"));
            self
        }
        pub fn instructions_search<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions_search = value.try_into().map_err(|e| {
                format!("error converting supplied value for instructions_search: {e}")
            });
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
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
        pub fn tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalResponsesSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn users<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.users = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for users: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalResponsesSource> for super::EvalResponsesSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalResponsesSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_after: value.created_after?,
                created_before: value.created_before?,
                instructions_search: value.instructions_search?,
                metadata: value.metadata?,
                model: value.model?,
                reasoning_effort: value.reasoning_effort?,
                temperature: value.temperature?,
                tools: value.tools?,
                top_p: value.top_p?,
                type_: value.type_?,
                users: value.users?,
            })
        }
    }
    impl ::std::convert::From<super::EvalResponsesSource> for EvalResponsesSource {
        fn from(value: super::EvalResponsesSource) -> Self {
            Self {
                created_after: Ok(value.created_after),
                created_before: Ok(value.created_before),
                instructions_search: Ok(value.instructions_search),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                reasoning_effort: Ok(value.reasoning_effort),
                temperature: Ok(value.temperature),
                tools: Ok(value.tools),
                top_p: Ok(value.top_p),
                type_: Ok(value.type_),
                users: Ok(value.users),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRun {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        data_source: ::std::result::Result<super::EvalRunDataSource, ::std::string::String>,
        error: ::std::result::Result<
            ::std::option::Option<super::EvalApiError>,
            ::std::string::String,
        >,
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::openai_api_types::common::Metadata, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::EvalRunObject, ::std::string::String>,
        per_model_usage: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::EvalRunPerModelUsageItem>>,
            ::std::string::String,
        >,
        per_testing_criteria_results: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::EvalRunPerTestingCriteriaResultsItem>>,
            ::std::string::String,
        >,
        report_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        result_counts: ::std::result::Result<super::EvalRunResultCounts, ::std::string::String>,
        status: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRun {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data_source: Err("no value supplied for data_source".to_string()),
                error: Err("no value supplied for error".to_string()),
                eval_id: Err("no value supplied for eval_id".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                model: Err("no value supplied for model".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                per_model_usage: Err("no value supplied for per_model_usage".to_string()),
                per_testing_criteria_results: Err(
                    "no value supplied for per_testing_criteria_results".to_string(),
                ),
                report_url: Err("no value supplied for report_url".to_string()),
                result_counts: Err("no value supplied for result_counts".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl EvalRun {
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
        pub fn data_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunDataSource>,
            T::Error: ::std::fmt::Display,
        {
            self.data_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data_source: {e}"));
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EvalApiError>>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn per_model_usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::EvalRunPerModelUsageItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.per_model_usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for per_model_usage: {e}"));
            self
        }
        pub fn per_testing_criteria_results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::EvalRunPerTestingCriteriaResultsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.per_testing_criteria_results = value.try_into().map_err(|e| {
                format!("error converting supplied value for per_testing_criteria_results: {e}")
            });
            self
        }
        pub fn report_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.report_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for report_url: {e}"));
            self
        }
        pub fn result_counts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunResultCounts>,
            T::Error: ::std::fmt::Display,
        {
            self.result_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_counts: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRun> for super::EvalRun {
        type Error = super::error::ConversionError;
        fn try_from(value: EvalRun) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data_source: value.data_source?,
                error: value.error?,
                eval_id: value.eval_id?,
                id: value.id?,
                metadata: value.metadata?,
                model: value.model?,
                name: value.name?,
                object: value.object?,
                per_model_usage: value.per_model_usage?,
                per_testing_criteria_results: value.per_testing_criteria_results?,
                report_url: value.report_url?,
                result_counts: value.result_counts?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRun> for EvalRun {
        fn from(value: super::EvalRun) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data_source: Ok(value.data_source),
                error: Ok(value.error),
                eval_id: Ok(value.eval_id),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                name: Ok(value.name),
                object: Ok(value.object),
                per_model_usage: Ok(value.per_model_usage),
                per_testing_criteria_results: Ok(value.per_testing_criteria_results),
                report_url: Ok(value.report_url),
                result_counts: Ok(value.result_counts),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunList {
        data: ::std::result::Result<::std::vec::Vec<super::EvalRun>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::EvalRunListObject, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunList {
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
    impl EvalRunList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalRun>>,
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
            T: ::std::convert::TryInto<super::EvalRunListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunList> for super::EvalRunList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunList,
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
    impl ::std::convert::From<super::EvalRunList> for EvalRunList {
        fn from(value: super::EvalRunList) -> Self {
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
    pub struct EvalRunOutputItem {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        datasource_item: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        datasource_item_id: ::std::result::Result<i64, ::std::string::String>,
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::EvalRunOutputItemObject, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::EvalRunOutputItemResult>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        sample: ::std::result::Result<super::EvalRunOutputItemSample, ::std::string::String>,
        status: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunOutputItem {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                datasource_item: Err("no value supplied for datasource_item".to_string()),
                datasource_item_id: Err("no value supplied for datasource_item_id".to_string()),
                eval_id: Err("no value supplied for eval_id".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                results: Err("no value supplied for results".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
                sample: Err("no value supplied for sample".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl EvalRunOutputItem {
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
        pub fn datasource_item<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.datasource_item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for datasource_item: {e}"));
            self
        }
        pub fn datasource_item_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.datasource_item_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for datasource_item_id: {e}")
            });
            self
        }
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
            T: ::std::convert::TryInto<super::EvalRunOutputItemObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalRunOutputItemResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
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
        pub fn sample<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunOutputItemSample>,
            T::Error: ::std::fmt::Display,
        {
            self.sample = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sample: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunOutputItem> for super::EvalRunOutputItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                datasource_item: value.datasource_item?,
                datasource_item_id: value.datasource_item_id?,
                eval_id: value.eval_id?,
                id: value.id?,
                object: value.object?,
                results: value.results?,
                run_id: value.run_id?,
                sample: value.sample?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItem> for EvalRunOutputItem {
        fn from(value: super::EvalRunOutputItem) -> Self {
            Self {
                created_at: Ok(value.created_at),
                datasource_item: Ok(value.datasource_item),
                datasource_item_id: Ok(value.datasource_item_id),
                eval_id: Ok(value.eval_id),
                id: Ok(value.id),
                object: Ok(value.object),
                results: Ok(value.results),
                run_id: Ok(value.run_id),
                sample: Ok(value.sample),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunOutputItemList {
        data:
            ::std::result::Result<::std::vec::Vec<super::EvalRunOutputItem>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::EvalRunOutputItemListObject, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunOutputItemList {
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
    impl EvalRunOutputItemList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalRunOutputItem>>,
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
            T: ::std::convert::TryInto<super::EvalRunOutputItemListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunOutputItemList> for super::EvalRunOutputItemList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemList,
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
    impl ::std::convert::From<super::EvalRunOutputItemList> for EvalRunOutputItemList {
        fn from(value: super::EvalRunOutputItemList) -> Self {
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
    pub struct EvalRunOutputItemResult {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        passed: ::std::result::Result<bool, ::std::string::String>,
        sample: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        score: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalRunOutputItemResult {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                passed: Err("no value supplied for passed".to_string()),
                sample: Ok(Default::default()),
                score: Err("no value supplied for score".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EvalRunOutputItemResult {
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
        pub fn passed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.passed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for passed: {e}"));
            self
        }
        pub fn sample<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sample = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sample: {e}"));
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
    impl ::std::convert::TryFrom<EvalRunOutputItemResult> for super::EvalRunOutputItemResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                passed: value.passed?,
                sample: value.sample?,
                score: value.score?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItemResult> for EvalRunOutputItemResult {
        fn from(value: super::EvalRunOutputItemResult) -> Self {
            Self {
                name: Ok(value.name),
                passed: Ok(value.passed),
                sample: Ok(value.sample),
                score: Ok(value.score),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunOutputItemSample {
        error: ::std::result::Result<
            ::std::option::Option<super::EvalApiError>,
            ::std::string::String,
        >,
        finish_reason: ::std::result::Result<::std::string::String, ::std::string::String>,
        input: ::std::result::Result<
            ::std::vec::Vec<super::EvalRunOutputItemSampleInputItem>,
            ::std::string::String,
        >,
        max_completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        output: ::std::result::Result<
            ::std::vec::Vec<super::EvalRunOutputItemSampleOutputItem>,
            ::std::string::String,
        >,
        seed: ::std::result::Result<i64, ::std::string::String>,
        temperature: ::std::result::Result<f64, ::std::string::String>,
        top_p: ::std::result::Result<f64, ::std::string::String>,
        usage: ::std::result::Result<super::EvalRunOutputItemSampleUsage, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunOutputItemSample {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                finish_reason: Err("no value supplied for finish_reason".to_string()),
                input: Err("no value supplied for input".to_string()),
                max_completion_tokens: Err(
                    "no value supplied for max_completion_tokens".to_string()
                ),
                model: Err("no value supplied for model".to_string()),
                output: Err("no value supplied for output".to_string()),
                seed: Err("no value supplied for seed".to_string()),
                temperature: Err("no value supplied for temperature".to_string()),
                top_p: Err("no value supplied for top_p".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl EvalRunOutputItemSample {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EvalApiError>>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn finish_reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.finish_reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finish_reason: {e}"));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalRunOutputItemSampleInputItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn max_completion_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_completion_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_completion_tokens: {e}")
            });
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
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalRunOutputItemSampleOutputItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {e}"));
            self
        }
        pub fn seed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.seed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seed: {e}"));
            self
        }
        pub fn temperature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.temperature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temperature: {e}"));
            self
        }
        pub fn top_p<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.top_p = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for top_p: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunOutputItemSampleUsage>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunOutputItemSample> for super::EvalRunOutputItemSample {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemSample,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                finish_reason: value.finish_reason?,
                input: value.input?,
                max_completion_tokens: value.max_completion_tokens?,
                model: value.model?,
                output: value.output?,
                seed: value.seed?,
                temperature: value.temperature?,
                top_p: value.top_p?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItemSample> for EvalRunOutputItemSample {
        fn from(value: super::EvalRunOutputItemSample) -> Self {
            Self {
                error: Ok(value.error),
                finish_reason: Ok(value.finish_reason),
                input: Ok(value.input),
                max_completion_tokens: Ok(value.max_completion_tokens),
                model: Ok(value.model),
                output: Ok(value.output),
                seed: Ok(value.seed),
                temperature: Ok(value.temperature),
                top_p: Ok(value.top_p),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunOutputItemSampleInputItem {
        content: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunOutputItemSampleInputItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl EvalRunOutputItemSampleInputItem {
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunOutputItemSampleInputItem>
        for super::EvalRunOutputItemSampleInputItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemSampleInputItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItemSampleInputItem>
        for EvalRunOutputItemSampleInputItem
    {
        fn from(value: super::EvalRunOutputItemSampleInputItem) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunOutputItemSampleOutputItem {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalRunOutputItemSampleOutputItem {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                role: Ok(Default::default()),
            }
        }
    }
    impl EvalRunOutputItemSampleOutputItem {
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunOutputItemSampleOutputItem>
        for super::EvalRunOutputItemSampleOutputItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemSampleOutputItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItemSampleOutputItem>
        for EvalRunOutputItemSampleOutputItem
    {
        fn from(value: super::EvalRunOutputItemSampleOutputItem) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunOutputItemSampleUsage {
        cached_tokens: ::std::result::Result<i64, ::std::string::String>,
        completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunOutputItemSampleUsage {
        fn default() -> Self {
            Self {
                cached_tokens: Err("no value supplied for cached_tokens".to_string()),
                completion_tokens: Err("no value supplied for completion_tokens".to_string()),
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl EvalRunOutputItemSampleUsage {
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
    impl ::std::convert::TryFrom<EvalRunOutputItemSampleUsage> for super::EvalRunOutputItemSampleUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunOutputItemSampleUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cached_tokens: value.cached_tokens?,
                completion_tokens: value.completion_tokens?,
                prompt_tokens: value.prompt_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunOutputItemSampleUsage> for EvalRunOutputItemSampleUsage {
        fn from(value: super::EvalRunOutputItemSampleUsage) -> Self {
            Self {
                cached_tokens: Ok(value.cached_tokens),
                completion_tokens: Ok(value.completion_tokens),
                prompt_tokens: Ok(value.prompt_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunPerModelUsageItem {
        cached_tokens: ::std::result::Result<i64, ::std::string::String>,
        completion_tokens: ::std::result::Result<i64, ::std::string::String>,
        invocation_count: ::std::result::Result<i64, ::std::string::String>,
        model_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt_tokens: ::std::result::Result<i64, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunPerModelUsageItem {
        fn default() -> Self {
            Self {
                cached_tokens: Err("no value supplied for cached_tokens".to_string()),
                completion_tokens: Err("no value supplied for completion_tokens".to_string()),
                invocation_count: Err("no value supplied for invocation_count".to_string()),
                model_name: Err("no value supplied for model_name".to_string()),
                prompt_tokens: Err("no value supplied for prompt_tokens".to_string()),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl EvalRunPerModelUsageItem {
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
        pub fn invocation_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.invocation_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invocation_count: {e}"));
            self
        }
        pub fn model_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_name: {e}"));
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
    impl ::std::convert::TryFrom<EvalRunPerModelUsageItem> for super::EvalRunPerModelUsageItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunPerModelUsageItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cached_tokens: value.cached_tokens?,
                completion_tokens: value.completion_tokens?,
                invocation_count: value.invocation_count?,
                model_name: value.model_name?,
                prompt_tokens: value.prompt_tokens?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunPerModelUsageItem> for EvalRunPerModelUsageItem {
        fn from(value: super::EvalRunPerModelUsageItem) -> Self {
            Self {
                cached_tokens: Ok(value.cached_tokens),
                completion_tokens: Ok(value.completion_tokens),
                invocation_count: Ok(value.invocation_count),
                model_name: Ok(value.model_name),
                prompt_tokens: Ok(value.prompt_tokens),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunPerTestingCriteriaResultsItem {
        failed: ::std::result::Result<i64, ::std::string::String>,
        passed: ::std::result::Result<i64, ::std::string::String>,
        testing_criteria: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunPerTestingCriteriaResultsItem {
        fn default() -> Self {
            Self {
                failed: Err("no value supplied for failed".to_string()),
                passed: Err("no value supplied for passed".to_string()),
                testing_criteria: Err("no value supplied for testing_criteria".to_string()),
            }
        }
    }
    impl EvalRunPerTestingCriteriaResultsItem {
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
        pub fn passed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.passed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for passed: {e}"));
            self
        }
        pub fn testing_criteria<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.testing_criteria = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for testing_criteria: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalRunPerTestingCriteriaResultsItem>
        for super::EvalRunPerTestingCriteriaResultsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunPerTestingCriteriaResultsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                failed: value.failed?,
                passed: value.passed?,
                testing_criteria: value.testing_criteria?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunPerTestingCriteriaResultsItem>
        for EvalRunPerTestingCriteriaResultsItem
    {
        fn from(value: super::EvalRunPerTestingCriteriaResultsItem) -> Self {
            Self {
                failed: Ok(value.failed),
                passed: Ok(value.passed),
                testing_criteria: Ok(value.testing_criteria),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalRunResultCounts {
        errored: ::std::result::Result<i64, ::std::string::String>,
        failed: ::std::result::Result<i64, ::std::string::String>,
        passed: ::std::result::Result<i64, ::std::string::String>,
        total: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for EvalRunResultCounts {
        fn default() -> Self {
            Self {
                errored: Err("no value supplied for errored".to_string()),
                failed: Err("no value supplied for failed".to_string()),
                passed: Err("no value supplied for passed".to_string()),
                total: Err("no value supplied for total".to_string()),
            }
        }
    }
    impl EvalRunResultCounts {
        pub fn errored<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.errored = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for errored: {e}"));
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
        pub fn passed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.passed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for passed: {e}"));
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
    impl ::std::convert::TryFrom<EvalRunResultCounts> for super::EvalRunResultCounts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalRunResultCounts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                errored: value.errored?,
                failed: value.failed?,
                passed: value.passed?,
                total: value.total?,
            })
        }
    }
    impl ::std::convert::From<super::EvalRunResultCounts> for EvalRunResultCounts {
        fn from(value: super::EvalRunResultCounts) -> Self {
            Self {
                errored: Ok(value.errored),
                failed: Ok(value.failed),
                passed: Ok(value.passed),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalStoredCompletionsDataSourceConfig {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        schema: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::EvalStoredCompletionsDataSourceConfigType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalStoredCompletionsDataSourceConfig {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                schema: Err("no value supplied for schema".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalStoredCompletionsDataSourceConfig {
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
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalStoredCompletionsDataSourceConfigType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalStoredCompletionsDataSourceConfig>
        for super::EvalStoredCompletionsDataSourceConfig
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalStoredCompletionsDataSourceConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                schema: value.schema?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalStoredCompletionsDataSourceConfig>
        for EvalStoredCompletionsDataSourceConfig
    {
        fn from(value: super::EvalStoredCompletionsDataSourceConfig) -> Self {
            Self {
                metadata: Ok(value.metadata),
                schema: Ok(value.schema),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalStoredCompletionsSource {
        created_after: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_before: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EvalStoredCompletionsSourceType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalStoredCompletionsSource {
        fn default() -> Self {
            Self {
                created_after: Ok(Default::default()),
                created_before: Ok(Default::default()),
                limit: Ok(Default::default()),
                metadata: Ok(Default::default()),
                model: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalStoredCompletionsSource {
        pub fn created_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_after: {e}"));
            self
        }
        pub fn created_before<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_before = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_before: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalStoredCompletionsSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalStoredCompletionsSource> for super::EvalStoredCompletionsSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalStoredCompletionsSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_after: value.created_after?,
                created_before: value.created_before?,
                limit: value.limit?,
                metadata: value.metadata?,
                model: value.model?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalStoredCompletionsSource> for EvalStoredCompletionsSource {
        fn from(value: super::EvalStoredCompletionsSource) -> Self {
            Self {
                created_after: Ok(value.created_after),
                created_before: Ok(value.created_before),
                limit: Ok(value.limit),
                metadata: Ok(value.metadata),
                model: Ok(value.model),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalsApiSchema {
        cancel_eval_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        cancel_eval_run_params:
            ::std::result::Result<super::CancelEvalRunParams, ::std::string::String>,
        cancel_eval_run_response: ::std::result::Result<super::EvalRun, ::std::string::String>,
        create_eval_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_request: ::std::result::Result<super::CreateEvalRequest, ::std::string::String>,
        create_eval_response: ::std::result::Result<super::Eval, ::std::string::String>,
        create_eval_run_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_run_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_run_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_eval_run_params:
            ::std::result::Result<super::CreateEvalRunParams, ::std::string::String>,
        create_eval_run_request:
            ::std::result::Result<super::CreateEvalRunRequest, ::std::string::String>,
        create_eval_run_response: ::std::result::Result<super::EvalRun, ::std::string::String>,
        delete_eval_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_params: ::std::result::Result<super::DeleteEvalParams, ::std::string::String>,
        delete_eval_response:
            ::std::result::Result<super::DeleteEvalResponse, ::std::string::String>,
        delete_eval_run_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_run_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_eval_run_params:
            ::std::result::Result<super::DeleteEvalRunParams, ::std::string::String>,
        delete_eval_run_response:
            ::std::result::Result<super::DeleteEvalRunResponse, ::std::string::String>,
        get_eval_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_params: ::std::result::Result<super::GetEvalParams, ::std::string::String>,
        get_eval_response: ::std::result::Result<super::Eval, ::std::string::String>,
        get_eval_run_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_item_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_item_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_item_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_item_params:
            ::std::result::Result<super::GetEvalRunOutputItemParams, ::std::string::String>,
        get_eval_run_output_item_response:
            ::std::result::Result<super::EvalRunOutputItem, ::std::string::String>,
        get_eval_run_output_items_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_items_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_items_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_run_output_items_params:
            ::std::result::Result<super::GetEvalRunOutputItemsParams, ::std::string::String>,
        get_eval_run_output_items_response:
            ::std::result::Result<super::EvalRunOutputItemList, ::std::string::String>,
        get_eval_run_params: ::std::result::Result<super::GetEvalRunParams, ::std::string::String>,
        get_eval_run_response: ::std::result::Result<super::EvalRun, ::std::string::String>,
        get_eval_runs_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_runs_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_runs_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_eval_runs_params:
            ::std::result::Result<super::GetEvalRunsParams, ::std::string::String>,
        get_eval_runs_response: ::std::result::Result<super::EvalRunList, ::std::string::String>,
        list_evals_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_evals_params: ::std::result::Result<super::ListEvalsParams, ::std::string::String>,
        list_evals_response: ::std::result::Result<super::EvalList, ::std::string::String>,
        update_eval_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_eval_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_eval_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_eval_params: ::std::result::Result<super::UpdateEvalParams, ::std::string::String>,
        update_eval_request: ::std::result::Result<super::UpdateEvalRequest, ::std::string::String>,
        update_eval_response: ::std::result::Result<super::Eval, ::std::string::String>,
    }
    impl ::std::default::Default for EvalsApiSchema {
        fn default() -> Self {
            Self {
                cancel_eval_run_error429: Err(
                    "no value supplied for cancel_eval_run_error429".to_string()
                ),
                cancel_eval_run_params: Err(
                    "no value supplied for cancel_eval_run_params".to_string()
                ),
                cancel_eval_run_response: Err(
                    "no value supplied for cancel_eval_run_response".to_string()
                ),
                create_eval_error400: Err("no value supplied for create_eval_error400".to_string()),
                create_eval_error429: Err("no value supplied for create_eval_error429".to_string()),
                create_eval_request: Err("no value supplied for create_eval_request".to_string()),
                create_eval_response: Err("no value supplied for create_eval_response".to_string()),
                create_eval_run_error400: Err(
                    "no value supplied for create_eval_run_error400".to_string()
                ),
                create_eval_run_error404: Err(
                    "no value supplied for create_eval_run_error404".to_string()
                ),
                create_eval_run_error429: Err(
                    "no value supplied for create_eval_run_error429".to_string()
                ),
                create_eval_run_error500: Err(
                    "no value supplied for create_eval_run_error500".to_string()
                ),
                create_eval_run_params: Err(
                    "no value supplied for create_eval_run_params".to_string()
                ),
                create_eval_run_request: Err(
                    "no value supplied for create_eval_run_request".to_string()
                ),
                create_eval_run_response: Err(
                    "no value supplied for create_eval_run_response".to_string()
                ),
                delete_eval_error400: Err("no value supplied for delete_eval_error400".to_string()),
                delete_eval_error404: Err("no value supplied for delete_eval_error404".to_string()),
                delete_eval_error429: Err("no value supplied for delete_eval_error429".to_string()),
                delete_eval_params: Err("no value supplied for delete_eval_params".to_string()),
                delete_eval_response: Err("no value supplied for delete_eval_response".to_string()),
                delete_eval_run_error400: Err(
                    "no value supplied for delete_eval_run_error400".to_string()
                ),
                delete_eval_run_error404: Err(
                    "no value supplied for delete_eval_run_error404".to_string()
                ),
                delete_eval_run_error429: Err(
                    "no value supplied for delete_eval_run_error429".to_string()
                ),
                delete_eval_run_params: Err(
                    "no value supplied for delete_eval_run_params".to_string()
                ),
                delete_eval_run_response: Err(
                    "no value supplied for delete_eval_run_response".to_string()
                ),
                get_eval_error404: Err("no value supplied for get_eval_error404".to_string()),
                get_eval_error429: Err("no value supplied for get_eval_error429".to_string()),
                get_eval_params: Err("no value supplied for get_eval_params".to_string()),
                get_eval_response: Err("no value supplied for get_eval_response".to_string()),
                get_eval_run_error400: Err(
                    "no value supplied for get_eval_run_error400".to_string()
                ),
                get_eval_run_error404: Err(
                    "no value supplied for get_eval_run_error404".to_string()
                ),
                get_eval_run_error429: Err(
                    "no value supplied for get_eval_run_error429".to_string()
                ),
                get_eval_run_output_item_error400: Err(
                    "no value supplied for get_eval_run_output_item_error400".to_string(),
                ),
                get_eval_run_output_item_error404: Err(
                    "no value supplied for get_eval_run_output_item_error404".to_string(),
                ),
                get_eval_run_output_item_error429: Err(
                    "no value supplied for get_eval_run_output_item_error429".to_string(),
                ),
                get_eval_run_output_item_params: Err(
                    "no value supplied for get_eval_run_output_item_params".to_string(),
                ),
                get_eval_run_output_item_response: Err(
                    "no value supplied for get_eval_run_output_item_response".to_string(),
                ),
                get_eval_run_output_items_error400: Err(
                    "no value supplied for get_eval_run_output_items_error400".to_string(),
                ),
                get_eval_run_output_items_error404: Err(
                    "no value supplied for get_eval_run_output_items_error404".to_string(),
                ),
                get_eval_run_output_items_error429: Err(
                    "no value supplied for get_eval_run_output_items_error429".to_string(),
                ),
                get_eval_run_output_items_params: Err(
                    "no value supplied for get_eval_run_output_items_params".to_string(),
                ),
                get_eval_run_output_items_response: Err(
                    "no value supplied for get_eval_run_output_items_response".to_string(),
                ),
                get_eval_run_params: Err("no value supplied for get_eval_run_params".to_string()),
                get_eval_run_response: Err(
                    "no value supplied for get_eval_run_response".to_string()
                ),
                get_eval_runs_error400: Err(
                    "no value supplied for get_eval_runs_error400".to_string()
                ),
                get_eval_runs_error404: Err(
                    "no value supplied for get_eval_runs_error404".to_string()
                ),
                get_eval_runs_error429: Err(
                    "no value supplied for get_eval_runs_error429".to_string()
                ),
                get_eval_runs_params: Err("no value supplied for get_eval_runs_params".to_string()),
                get_eval_runs_response: Err(
                    "no value supplied for get_eval_runs_response".to_string()
                ),
                list_evals_error429: Err("no value supplied for list_evals_error429".to_string()),
                list_evals_params: Err("no value supplied for list_evals_params".to_string()),
                list_evals_response: Err("no value supplied for list_evals_response".to_string()),
                update_eval_error400: Err("no value supplied for update_eval_error400".to_string()),
                update_eval_error404: Err("no value supplied for update_eval_error404".to_string()),
                update_eval_error429: Err("no value supplied for update_eval_error429".to_string()),
                update_eval_params: Err("no value supplied for update_eval_params".to_string()),
                update_eval_request: Err("no value supplied for update_eval_request".to_string()),
                update_eval_response: Err("no value supplied for update_eval_response".to_string()),
            }
        }
    }
    impl EvalsApiSchema {
        pub fn cancel_eval_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_eval_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_eval_run_error429: {e}")
            });
            self
        }
        pub fn cancel_eval_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelEvalRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_eval_run_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_eval_run_params: {e}")
            });
            self
        }
        pub fn cancel_eval_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRun>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_eval_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_eval_run_response: {e}")
            });
            self
        }
        pub fn create_eval_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_error400: {e}")
            });
            self
        }
        pub fn create_eval_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_error429: {e}")
            });
            self
        }
        pub fn create_eval_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_request: {e}")
            });
            self
        }
        pub fn create_eval_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Eval>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_response: {e}")
            });
            self
        }
        pub fn create_eval_run_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_error400: {e}")
            });
            self
        }
        pub fn create_eval_run_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_error404: {e}")
            });
            self
        }
        pub fn create_eval_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_error429: {e}")
            });
            self
        }
        pub fn create_eval_run_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_error500: {e}")
            });
            self
        }
        pub fn create_eval_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_params: {e}")
            });
            self
        }
        pub fn create_eval_run_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateEvalRunRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_request: {e}")
            });
            self
        }
        pub fn create_eval_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRun>,
            T::Error: ::std::fmt::Display,
        {
            self.create_eval_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_eval_run_response: {e}")
            });
            self
        }
        pub fn delete_eval_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_error400: {e}")
            });
            self
        }
        pub fn delete_eval_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_error404: {e}")
            });
            self
        }
        pub fn delete_eval_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_error429: {e}")
            });
            self
        }
        pub fn delete_eval_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteEvalParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_params: {e}")
            });
            self
        }
        pub fn delete_eval_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteEvalResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_response: {e}")
            });
            self
        }
        pub fn delete_eval_run_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_run_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_run_error400: {e}")
            });
            self
        }
        pub fn delete_eval_run_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_run_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_run_error404: {e}")
            });
            self
        }
        pub fn delete_eval_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_run_error429: {e}")
            });
            self
        }
        pub fn delete_eval_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteEvalRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_run_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_run_params: {e}")
            });
            self
        }
        pub fn delete_eval_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteEvalRunResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_eval_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_eval_run_response: {e}")
            });
            self
        }
        pub fn get_eval_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_error404 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_eval_error404: {e}"));
            self
        }
        pub fn get_eval_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_error429 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_eval_error429: {e}"));
            self
        }
        pub fn get_eval_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetEvalParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_eval_params: {e}"));
            self
        }
        pub fn get_eval_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Eval>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_response = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_eval_response: {e}"));
            self
        }
        pub fn get_eval_run_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_error400: {e}")
            });
            self
        }
        pub fn get_eval_run_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_error404: {e}")
            });
            self
        }
        pub fn get_eval_run_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_error429: {e}")
            });
            self
        }
        pub fn get_eval_run_output_item_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_item_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_item_error400: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_item_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_item_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_item_error404: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_item_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_item_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_item_error429: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_item_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetEvalRunOutputItemParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_item_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_output_item_params: {e}")
            });
            self
        }
        pub fn get_eval_run_output_item_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunOutputItem>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_item_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_item_response: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_items_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_items_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_items_error400: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_items_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_items_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_items_error404: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_items_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_items_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_items_error429: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_output_items_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetEvalRunOutputItemsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_items_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_output_items_params: {e}")
            });
            self
        }
        pub fn get_eval_run_output_items_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunOutputItemList>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_output_items_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_eval_run_output_items_response: {e}"
                )
            });
            self
        }
        pub fn get_eval_run_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetEvalRunParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_params: {e}")
            });
            self
        }
        pub fn get_eval_run_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRun>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_run_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_run_response: {e}")
            });
            self
        }
        pub fn get_eval_runs_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_runs_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_runs_error400: {e}")
            });
            self
        }
        pub fn get_eval_runs_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_runs_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_runs_error404: {e}")
            });
            self
        }
        pub fn get_eval_runs_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_runs_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_runs_error429: {e}")
            });
            self
        }
        pub fn get_eval_runs_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetEvalRunsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_runs_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_runs_params: {e}")
            });
            self
        }
        pub fn get_eval_runs_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalRunList>,
            T::Error: ::std::fmt::Display,
        {
            self.get_eval_runs_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_eval_runs_response: {e}")
            });
            self
        }
        pub fn list_evals_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_evals_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_evals_error429: {e}")
            });
            self
        }
        pub fn list_evals_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListEvalsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_evals_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_evals_params: {e}"));
            self
        }
        pub fn list_evals_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalList>,
            T::Error: ::std::fmt::Display,
        {
            self.list_evals_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_evals_response: {e}")
            });
            self
        }
        pub fn update_eval_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_error400: {e}")
            });
            self
        }
        pub fn update_eval_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_error404: {e}")
            });
            self
        }
        pub fn update_eval_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_error429: {e}")
            });
            self
        }
        pub fn update_eval_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateEvalParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_params: {e}")
            });
            self
        }
        pub fn update_eval_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateEvalRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_request: {e}")
            });
            self
        }
        pub fn update_eval_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Eval>,
            T::Error: ::std::fmt::Display,
        {
            self.update_eval_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_eval_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<EvalsApiSchema> for super::EvalsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_eval_run_error429: value.cancel_eval_run_error429?,
                cancel_eval_run_params: value.cancel_eval_run_params?,
                cancel_eval_run_response: value.cancel_eval_run_response?,
                create_eval_error400: value.create_eval_error400?,
                create_eval_error429: value.create_eval_error429?,
                create_eval_request: value.create_eval_request?,
                create_eval_response: value.create_eval_response?,
                create_eval_run_error400: value.create_eval_run_error400?,
                create_eval_run_error404: value.create_eval_run_error404?,
                create_eval_run_error429: value.create_eval_run_error429?,
                create_eval_run_error500: value.create_eval_run_error500?,
                create_eval_run_params: value.create_eval_run_params?,
                create_eval_run_request: value.create_eval_run_request?,
                create_eval_run_response: value.create_eval_run_response?,
                delete_eval_error400: value.delete_eval_error400?,
                delete_eval_error404: value.delete_eval_error404?,
                delete_eval_error429: value.delete_eval_error429?,
                delete_eval_params: value.delete_eval_params?,
                delete_eval_response: value.delete_eval_response?,
                delete_eval_run_error400: value.delete_eval_run_error400?,
                delete_eval_run_error404: value.delete_eval_run_error404?,
                delete_eval_run_error429: value.delete_eval_run_error429?,
                delete_eval_run_params: value.delete_eval_run_params?,
                delete_eval_run_response: value.delete_eval_run_response?,
                get_eval_error404: value.get_eval_error404?,
                get_eval_error429: value.get_eval_error429?,
                get_eval_params: value.get_eval_params?,
                get_eval_response: value.get_eval_response?,
                get_eval_run_error400: value.get_eval_run_error400?,
                get_eval_run_error404: value.get_eval_run_error404?,
                get_eval_run_error429: value.get_eval_run_error429?,
                get_eval_run_output_item_error400: value.get_eval_run_output_item_error400?,
                get_eval_run_output_item_error404: value.get_eval_run_output_item_error404?,
                get_eval_run_output_item_error429: value.get_eval_run_output_item_error429?,
                get_eval_run_output_item_params: value.get_eval_run_output_item_params?,
                get_eval_run_output_item_response: value.get_eval_run_output_item_response?,
                get_eval_run_output_items_error400: value.get_eval_run_output_items_error400?,
                get_eval_run_output_items_error404: value.get_eval_run_output_items_error404?,
                get_eval_run_output_items_error429: value.get_eval_run_output_items_error429?,
                get_eval_run_output_items_params: value.get_eval_run_output_items_params?,
                get_eval_run_output_items_response: value.get_eval_run_output_items_response?,
                get_eval_run_params: value.get_eval_run_params?,
                get_eval_run_response: value.get_eval_run_response?,
                get_eval_runs_error400: value.get_eval_runs_error400?,
                get_eval_runs_error404: value.get_eval_runs_error404?,
                get_eval_runs_error429: value.get_eval_runs_error429?,
                get_eval_runs_params: value.get_eval_runs_params?,
                get_eval_runs_response: value.get_eval_runs_response?,
                list_evals_error429: value.list_evals_error429?,
                list_evals_params: value.list_evals_params?,
                list_evals_response: value.list_evals_response?,
                update_eval_error400: value.update_eval_error400?,
                update_eval_error404: value.update_eval_error404?,
                update_eval_error429: value.update_eval_error429?,
                update_eval_params: value.update_eval_params?,
                update_eval_request: value.update_eval_request?,
                update_eval_response: value.update_eval_response?,
            })
        }
    }
    impl ::std::convert::From<super::EvalsApiSchema> for EvalsApiSchema {
        fn from(value: super::EvalsApiSchema) -> Self {
            Self {
                cancel_eval_run_error429: Ok(value.cancel_eval_run_error429),
                cancel_eval_run_params: Ok(value.cancel_eval_run_params),
                cancel_eval_run_response: Ok(value.cancel_eval_run_response),
                create_eval_error400: Ok(value.create_eval_error400),
                create_eval_error429: Ok(value.create_eval_error429),
                create_eval_request: Ok(value.create_eval_request),
                create_eval_response: Ok(value.create_eval_response),
                create_eval_run_error400: Ok(value.create_eval_run_error400),
                create_eval_run_error404: Ok(value.create_eval_run_error404),
                create_eval_run_error429: Ok(value.create_eval_run_error429),
                create_eval_run_error500: Ok(value.create_eval_run_error500),
                create_eval_run_params: Ok(value.create_eval_run_params),
                create_eval_run_request: Ok(value.create_eval_run_request),
                create_eval_run_response: Ok(value.create_eval_run_response),
                delete_eval_error400: Ok(value.delete_eval_error400),
                delete_eval_error404: Ok(value.delete_eval_error404),
                delete_eval_error429: Ok(value.delete_eval_error429),
                delete_eval_params: Ok(value.delete_eval_params),
                delete_eval_response: Ok(value.delete_eval_response),
                delete_eval_run_error400: Ok(value.delete_eval_run_error400),
                delete_eval_run_error404: Ok(value.delete_eval_run_error404),
                delete_eval_run_error429: Ok(value.delete_eval_run_error429),
                delete_eval_run_params: Ok(value.delete_eval_run_params),
                delete_eval_run_response: Ok(value.delete_eval_run_response),
                get_eval_error404: Ok(value.get_eval_error404),
                get_eval_error429: Ok(value.get_eval_error429),
                get_eval_params: Ok(value.get_eval_params),
                get_eval_response: Ok(value.get_eval_response),
                get_eval_run_error400: Ok(value.get_eval_run_error400),
                get_eval_run_error404: Ok(value.get_eval_run_error404),
                get_eval_run_error429: Ok(value.get_eval_run_error429),
                get_eval_run_output_item_error400: Ok(value.get_eval_run_output_item_error400),
                get_eval_run_output_item_error404: Ok(value.get_eval_run_output_item_error404),
                get_eval_run_output_item_error429: Ok(value.get_eval_run_output_item_error429),
                get_eval_run_output_item_params: Ok(value.get_eval_run_output_item_params),
                get_eval_run_output_item_response: Ok(value.get_eval_run_output_item_response),
                get_eval_run_output_items_error400: Ok(value.get_eval_run_output_items_error400),
                get_eval_run_output_items_error404: Ok(value.get_eval_run_output_items_error404),
                get_eval_run_output_items_error429: Ok(value.get_eval_run_output_items_error429),
                get_eval_run_output_items_params: Ok(value.get_eval_run_output_items_params),
                get_eval_run_output_items_response: Ok(value.get_eval_run_output_items_response),
                get_eval_run_params: Ok(value.get_eval_run_params),
                get_eval_run_response: Ok(value.get_eval_run_response),
                get_eval_runs_error400: Ok(value.get_eval_runs_error400),
                get_eval_runs_error404: Ok(value.get_eval_runs_error404),
                get_eval_runs_error429: Ok(value.get_eval_runs_error429),
                get_eval_runs_params: Ok(value.get_eval_runs_params),
                get_eval_runs_response: Ok(value.get_eval_runs_response),
                list_evals_error429: Ok(value.list_evals_error429),
                list_evals_params: Ok(value.list_evals_params),
                list_evals_response: Ok(value.list_evals_response),
                update_eval_error400: Ok(value.update_eval_error400),
                update_eval_error404: Ok(value.update_eval_error404),
                update_eval_error429: Ok(value.update_eval_error429),
                update_eval_params: Ok(value.update_eval_params),
                update_eval_request: Ok(value.update_eval_request),
                update_eval_response: Ok(value.update_eval_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetEvalParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetEvalParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
            }
        }
    }
    impl GetEvalParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetEvalParams> for super::GetEvalParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetEvalParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetEvalParams> for GetEvalParams {
        fn from(value: super::GetEvalParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetEvalRunOutputItemParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        output_item_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetEvalRunOutputItemParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
                output_item_id: Err("no value supplied for output_item_id".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
            }
        }
    }
    impl GetEvalRunOutputItemParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
            self
        }
        pub fn output_item_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.output_item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_item_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<GetEvalRunOutputItemParams> for super::GetEvalRunOutputItemParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetEvalRunOutputItemParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
                output_item_id: value.output_item_id?,
                run_id: value.run_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetEvalRunOutputItemParams> for GetEvalRunOutputItemParams {
        fn from(value: super::GetEvalRunOutputItemParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
                output_item_id: Ok(value.output_item_id),
                run_id: Ok(value.run_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetEvalRunOutputItemsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::GetEvalRunOutputItemsParamsOrder>,
            ::std::string::String,
        >,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<
            ::std::option::Option<super::GetEvalRunOutputItemsParamsStatus>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetEvalRunOutputItemsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                eval_id: Err("no value supplied for eval_id".to_string()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                run_id: Err("no value supplied for run_id".to_string()),
                status: Ok(Default::default()),
            }
        }
    }
    impl GetEvalRunOutputItemsParams {
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
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
                ::std::option::Option<super::GetEvalRunOutputItemsParamsOrder>,
            >,
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
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GetEvalRunOutputItemsParamsStatus>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetEvalRunOutputItemsParams> for super::GetEvalRunOutputItemsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetEvalRunOutputItemsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                eval_id: value.eval_id?,
                limit: value.limit?,
                order: value.order?,
                run_id: value.run_id?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::GetEvalRunOutputItemsParams> for GetEvalRunOutputItemsParams {
        fn from(value: super::GetEvalRunOutputItemsParams) -> Self {
            Self {
                after: Ok(value.after),
                eval_id: Ok(value.eval_id),
                limit: Ok(value.limit),
                order: Ok(value.order),
                run_id: Ok(value.run_id),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetEvalRunParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        run_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetEvalRunParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
                run_id: Err("no value supplied for run_id".to_string()),
            }
        }
    }
    impl GetEvalRunParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<GetEvalRunParams> for super::GetEvalRunParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetEvalRunParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
                run_id: value.run_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetEvalRunParams> for GetEvalRunParams {
        fn from(value: super::GetEvalRunParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
                run_id: Ok(value.run_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetEvalRunsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::GetEvalRunsParamsOrder>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::GetEvalRunsParamsStatus>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetEvalRunsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                eval_id: Err("no value supplied for eval_id".to_string()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                status: Ok(Default::default()),
            }
        }
    }
    impl GetEvalRunsParams {
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
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::GetEvalRunsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GetEvalRunsParamsStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetEvalRunsParams> for super::GetEvalRunsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetEvalRunsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                eval_id: value.eval_id?,
                limit: value.limit?,
                order: value.order?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::GetEvalRunsParams> for GetEvalRunsParams {
        fn from(value: super::GetEvalRunsParams) -> Self {
            Self {
                after: Ok(value.after),
                eval_id: Ok(value.eval_id),
                limit: Ok(value.limit),
                order: Ok(value.order),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListEvalsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListEvalsParamsOrder>,
            ::std::string::String,
        >,
        order_by: ::std::result::Result<
            ::std::option::Option<super::ListEvalsParamsOrderBy>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListEvalsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                order_by: Ok(Default::default()),
            }
        }
    }
    impl ListEvalsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListEvalsParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn order_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListEvalsParamsOrderBy>>,
            T::Error: ::std::fmt::Display,
        {
            self.order_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order_by: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListEvalsParams> for super::ListEvalsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListEvalsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                order_by: value.order_by?,
            })
        }
    }
    impl ::std::convert::From<super::ListEvalsParams> for ListEvalsParams {
        fn from(value: super::ListEvalsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                order_by: Ok(value.order_by),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateEvalParams {
        eval_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateEvalParams {
        fn default() -> Self {
            Self {
                eval_id: Err("no value supplied for eval_id".to_string()),
            }
        }
    }
    impl UpdateEvalParams {
        pub fn eval_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.eval_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateEvalParams> for super::UpdateEvalParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateEvalParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eval_id: value.eval_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateEvalParams> for UpdateEvalParams {
        fn from(value: super::UpdateEvalParams) -> Self {
            Self {
                eval_id: Ok(value.eval_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateEvalRequest {
        metadata: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::Metadata>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateEvalRequest {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl UpdateEvalRequest {
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
    impl ::std::convert::TryFrom<UpdateEvalRequest> for super::UpdateEvalRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateEvalRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateEvalRequest> for UpdateEvalRequest {
        fn from(value: super::UpdateEvalRequest) -> Self {
            Self {
                metadata: Ok(value.metadata),
                name: Ok(value.name),
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
