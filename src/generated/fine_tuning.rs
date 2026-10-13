#[doc = "`CreateFineTuningCheckpointPermissionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateFineTuningCheckpointPermissionRequest {
    #[doc = "The project identifiers to grant access to."]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
}
#[doc = "`CreateFineTuningJobRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateFineTuningJobRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hyperparameters: ::std::option::Option<CreateFineTuningJobRequestHyperparameters>,
    #[doc = "A list of integrations to enable for your fine-tuning job."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integrations:
        ::std::option::Option<::std::vec::Vec<CreateFineTuningJobRequestIntegrationsItem>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<FineTuneMethod>,
    #[doc = "The name of the model to fine-tune. You can select one of the\n[supported models](https://developers.openai.com/api/docs/guides/model-optimization#fine-tuning-methods).\n"]
    pub model: ::std::string::String,
    #[doc = "The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.\nIf a seed is not specified, one will be generated for you.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    #[doc = "A string of up to 64 characters that will be added to your fine-tuned model name.\n\nFor example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub suffix: ::std::option::Option<CreateFineTuningJobRequestSuffix>,
    #[doc = "The ID of an uploaded file that contains training data.\n\nSee [upload file](https://developers.openai.com/api/reference/resources/files/methods/create) for how to upload a file.\n\nYour dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.\n\nThe contents of the file should differ depending on if the model uses the [chat](https://developers.openai.com/api/docs/guides/supervised-fine-tuning#formatting-your-data), [completions](https://developers.openai.com/api/docs/guides/supervised-fine-tuning#formatting-your-data) format, or if the fine-tuning method uses the [preference](https://developers.openai.com/api/docs/guides/direct-preference-optimization) format.\n\nSee the [fine-tuning guide](https://developers.openai.com/api/docs/guides/model-optimization) for more details.\n"]
    pub training_file: ::std::string::String,
    #[doc = "The ID of an uploaded file that contains validation data.\n\nIf you provide this file, the data is used to generate validation\nmetrics periodically during fine-tuning. These metrics can be viewed in\nthe fine-tuning results file.\nThe same data should not be present in both train and validation files.\n\nYour dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.\n\nSee the [fine-tuning guide](https://developers.openai.com/api/docs/guides/model-optimization) for more details.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub validation_file: ::std::option::Option<::std::string::String>,
}
#[doc = "The hyperparameters used for the fine-tuning job.\nThis value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateFineTuningJobRequestHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size: ::std::option::Option<CreateFineTuningJobRequestHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub learning_rate_multiplier:
        ::std::option::Option<CreateFineTuningJobRequestHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n_epochs: ::std::option::Option<CreateFineTuningJobRequestHyperparametersNEpochs>,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
    String(CreateFineTuningJobRequestHyperparametersBatchSizeString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersBatchSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersBatchSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersBatchSize
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<CreateFineTuningJobRequestHyperparametersBatchSizeString>
    for CreateFineTuningJobRequestHyperparametersBatchSize
{
    fn from(value: CreateFineTuningJobRequestHyperparametersBatchSizeString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64>
    for CreateFineTuningJobRequestHyperparametersBatchSize
{
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`CreateFineTuningJobRequestHyperparametersBatchSizeString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSizeString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersBatchSizeString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersBatchSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersBatchSizeString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    String(CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString>
    for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier
{
    fn from(value: CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
    String(CreateFineTuningJobRequestHyperparametersNEpochsString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersNEpochs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersNEpochs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersNEpochs
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<CreateFineTuningJobRequestHyperparametersNEpochsString>
    for CreateFineTuningJobRequestHyperparametersNEpochs
{
    fn from(value: CreateFineTuningJobRequestHyperparametersNEpochsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64>
    for CreateFineTuningJobRequestHyperparametersNEpochs
{
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`CreateFineTuningJobRequestHyperparametersNEpochsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochsString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateFineTuningJobRequestHyperparametersNEpochsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestHyperparametersNEpochsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestHyperparametersNEpochsString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateFineTuningJobRequestIntegrationsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrationsItem {
    #[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
    #[serde(rename = "type")]
    pub type_: CreateFineTuningJobRequestIntegrationsItemType,
    pub wandb: CreateFineTuningJobRequestIntegrationsItemWandb,
}
#[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateFineTuningJobRequestIntegrationsItemType {
    #[serde(rename = "wandb")]
    Wandb,
}
impl ::std::fmt::Display for CreateFineTuningJobRequestIntegrationsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wandb => f.write_str("wandb"),
        }
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestIntegrationsItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wandb" => Ok(Self::Wandb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestIntegrationsItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateFineTuningJobRequestIntegrationsItemType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateFineTuningJobRequestIntegrationsItemWandb {
    #[doc = "The entity to use for the run. This allows you to set the team or username of the WandB user that you would\nlike associated with the run. If not set, the default entity for the registered WandB API key is used.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub entity: ::std::option::Option<::std::string::String>,
    #[doc = "A display name to set for the run. If not set, we will use the Job ID as the name.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the project that the new run will be created under.\n"]
    pub project: ::std::string::String,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
#[doc = "A string of up to 64 characters that will be added to your fine-tuned model name.\n\nFor example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateFineTuningJobRequestSuffix(::std::string::String);
impl ::std::ops::Deref for CreateFineTuningJobRequestSuffix {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateFineTuningJobRequestSuffix> for ::std::string::String {
    fn from(value: CreateFineTuningJobRequestSuffix) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateFineTuningJobRequestSuffix {
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
impl ::std::convert::TryFrom<&str> for CreateFineTuningJobRequestSuffix {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateFineTuningJobRequestSuffix {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateFineTuningJobRequestSuffix {
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
#[doc = "`DeleteFineTuningCheckpointPermissionResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteFineTuningCheckpointPermissionResponse {
    #[doc = "Whether the fine-tuned model checkpoint permission was successfully deleted."]
    pub deleted: bool,
    #[doc = "The ID of the fine-tuned model checkpoint permission that was deleted."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    pub object: DeleteFineTuningCheckpointPermissionResponseObject,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeleteFineTuningCheckpointPermissionResponseObject {
    #[serde(rename = "checkpoint.permission")]
    CheckpointPermission,
}
impl ::std::fmt::Display for DeleteFineTuningCheckpointPermissionResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CheckpointPermission => f.write_str("checkpoint.permission"),
        }
    }
}
impl ::std::str::FromStr for DeleteFineTuningCheckpointPermissionResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "checkpoint.permission" => Ok(Self::CheckpointPermission),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteFineTuningCheckpointPermissionResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for DeleteFineTuningCheckpointPermissionResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalItem {
    pub content: EvalItemContent,
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    pub role: EvalItemRole,
    #[doc = "The type of the message input. Always `message`.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EvalItemType>,
}
#[doc = "Inputs to the model - can contain template strings. Supports text, output text, input images, and input audio, either as a single item or an array of items.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EvalItemContent {
    Item(EvalItemContentItem),
    Array(EvalItemContentArray),
}
impl ::std::convert::From<EvalItemContentItem> for EvalItemContent {
    fn from(value: EvalItemContentItem) -> Self {
        Self::Item(value)
    }
}
impl ::std::convert::From<EvalItemContentArray> for EvalItemContent {
    fn from(value: EvalItemContentArray) -> Self {
        Self::Array(value)
    }
}
#[doc = "A list of inputs, each of which may be either an input text, output text, input\nimage, or input audio object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct EvalItemContentArray(pub ::std::vec::Vec<EvalItemContentItem>);
impl ::std::ops::Deref for EvalItemContentArray {
    type Target = ::std::vec::Vec<EvalItemContentItem>;
    fn deref(&self) -> &::std::vec::Vec<EvalItemContentItem> {
        &self.0
    }
}
impl ::std::convert::From<EvalItemContentArray> for ::std::vec::Vec<EvalItemContentItem> {
    fn from(value: EvalItemContentArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<EvalItemContentItem>> for EvalItemContentArray {
    fn from(value: ::std::vec::Vec<EvalItemContentItem>) -> Self {
        Self(value)
    }
}
#[doc = "A single content item: input text, output text, input image, or input audio.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EvalItemContentItem {
    EvalItemContentText(EvalItemContentText),
    InputTextContent(InputTextContent),
    EvalItemContentOutputText(EvalItemContentOutputText),
    EvalItemInputImage(EvalItemInputImage),
    InputAudio(InputAudio),
}
impl ::std::convert::From<EvalItemContentText> for EvalItemContentItem {
    fn from(value: EvalItemContentText) -> Self {
        Self::EvalItemContentText(value)
    }
}
impl ::std::convert::From<InputTextContent> for EvalItemContentItem {
    fn from(value: InputTextContent) -> Self {
        Self::InputTextContent(value)
    }
}
impl ::std::convert::From<EvalItemContentOutputText> for EvalItemContentItem {
    fn from(value: EvalItemContentOutputText) -> Self {
        Self::EvalItemContentOutputText(value)
    }
}
impl ::std::convert::From<EvalItemInputImage> for EvalItemContentItem {
    fn from(value: EvalItemInputImage) -> Self {
        Self::EvalItemInputImage(value)
    }
}
impl ::std::convert::From<InputAudio> for EvalItemContentItem {
    fn from(value: InputAudio) -> Self {
        Self::InputAudio(value)
    }
}
#[doc = "A text output from the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalItemContentOutputText {
    #[doc = "The text output from the model.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the output text. Always `output_text`.\n"]
    #[serde(rename = "type")]
    pub type_: EvalItemContentOutputTextType,
}
#[doc = "The type of the output text. Always `output_text`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalItemContentOutputTextType {
    #[serde(rename = "output_text")]
    OutputText,
}
impl ::std::fmt::Display for EvalItemContentOutputTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputText => f.write_str("output_text"),
        }
    }
}
impl ::std::str::FromStr for EvalItemContentOutputTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_text" => Ok(Self::OutputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalItemContentOutputTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalItemContentOutputTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text input to the model.\n"]
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
pub struct EvalItemContentText(pub ::std::string::String);
impl ::std::ops::Deref for EvalItemContentText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EvalItemContentText> for ::std::string::String {
    fn from(value: EvalItemContentText) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for EvalItemContentText {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for EvalItemContentText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for EvalItemContentText {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "An image input block used within EvalItem content arrays."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EvalItemInputImage {
    #[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the image input.\n"]
    pub image_url: ::std::string::String,
    #[doc = "The type of the image input. Always `input_image`.\n"]
    #[serde(rename = "type")]
    pub type_: EvalItemInputImageType,
}
#[doc = "The type of the image input. Always `input_image`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalItemInputImageType {
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for EvalItemInputImageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for EvalItemInputImageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalItemInputImageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalItemInputImageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalItemRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for EvalItemRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::System => f.write_str("system"),
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for EvalItemRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalItemRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalItemRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the message input. Always `message`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EvalItemType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for EvalItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for EvalItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EvalItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EvalItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The hyperparameters used for the DPO fine-tuning job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuneDpoHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size: ::std::option::Option<FineTuneDpoHyperparametersBatchSize>,
    #[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub beta: ::std::option::Option<FineTuneDpoHyperparametersBeta>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub learning_rate_multiplier:
        ::std::option::Option<FineTuneDpoHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n_epochs: ::std::option::Option<FineTuneDpoHyperparametersNEpochs>,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersBatchSize {
    String(FineTuneDpoHyperparametersBatchSizeString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersBatchSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersBatchSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneDpoHyperparametersBatchSizeString>
    for FineTuneDpoHyperparametersBatchSize
{
    fn from(value: FineTuneDpoHyperparametersBatchSizeString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuneDpoHyperparametersBatchSize {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneDpoHyperparametersBatchSizeString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneDpoHyperparametersBatchSizeString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersBatchSizeString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersBatchSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersBeta {
    String(FineTuneDpoHyperparametersBetaString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersBeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersBeta {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersBeta {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersBeta {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneDpoHyperparametersBetaString> for FineTuneDpoHyperparametersBeta {
    fn from(value: FineTuneDpoHyperparametersBetaString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuneDpoHyperparametersBeta {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuneDpoHyperparametersBetaString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneDpoHyperparametersBetaString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersBetaString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersBetaString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersBetaString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersBetaString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersLearningRateMultiplier {
    String(FineTuneDpoHyperparametersLearningRateMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersLearningRateMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersLearningRateMultiplier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneDpoHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneDpoHyperparametersLearningRateMultiplierString>
    for FineTuneDpoHyperparametersLearningRateMultiplier
{
    fn from(value: FineTuneDpoHyperparametersLearningRateMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuneDpoHyperparametersLearningRateMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuneDpoHyperparametersLearningRateMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneDpoHyperparametersLearningRateMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersLearningRateMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersLearningRateMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersLearningRateMultiplierString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneDpoHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneDpoHyperparametersNEpochs {
    String(FineTuneDpoHyperparametersNEpochsString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersNEpochs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersNEpochs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneDpoHyperparametersNEpochsString>
    for FineTuneDpoHyperparametersNEpochs
{
    fn from(value: FineTuneDpoHyperparametersNEpochsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuneDpoHyperparametersNEpochs {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneDpoHyperparametersNEpochsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneDpoHyperparametersNEpochsString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneDpoHyperparametersNEpochsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneDpoHyperparametersNEpochsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneDpoHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneDpoHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for the DPO fine-tuning method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuneDpoMethod {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hyperparameters: ::std::option::Option<FineTuneDpoHyperparameters>,
}
#[doc = "The method used for fine-tuning."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuneMethod {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dpo: ::std::option::Option<FineTuneDpoMethod>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reinforcement: ::std::option::Option<FineTuneReinforcementMethod>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub supervised: ::std::option::Option<FineTuneSupervisedMethod>,
    #[doc = "The type of method. Is either `supervised`, `dpo`, or `reinforcement`."]
    #[serde(rename = "type")]
    pub type_: FineTuneMethodType,
}
#[doc = "The type of method. Is either `supervised`, `dpo`, or `reinforcement`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneMethodType {
    #[serde(rename = "supervised")]
    Supervised,
    #[serde(rename = "dpo")]
    Dpo,
    #[serde(rename = "reinforcement")]
    Reinforcement,
}
impl ::std::fmt::Display for FineTuneMethodType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Supervised => f.write_str("supervised"),
            Self::Dpo => f.write_str("dpo"),
            Self::Reinforcement => f.write_str("reinforcement"),
        }
    }
}
impl ::std::str::FromStr for FineTuneMethodType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "supervised" => Ok(Self::Supervised),
            "dpo" => Ok(Self::Dpo),
            "reinforcement" => Ok(Self::Reinforcement),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneMethodType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneMethodType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The hyperparameters used for the reinforcement fine-tuning job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuneReinforcementHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size: ::std::option::Option<FineTuneReinforcementHyperparametersBatchSize>,
    #[doc = "Multiplier on amount of compute used for exploring search space during training.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compute_multiplier:
        ::std::option::Option<FineTuneReinforcementHyperparametersComputeMultiplier>,
    #[doc = "The number of training steps between evaluation runs.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub eval_interval: ::std::option::Option<FineTuneReinforcementHyperparametersEvalInterval>,
    #[doc = "Number of evaluation samples to generate per training step.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub eval_samples: ::std::option::Option<FineTuneReinforcementHyperparametersEvalSamples>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub learning_rate_multiplier:
        ::std::option::Option<FineTuneReinforcementHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n_epochs: ::std::option::Option<FineTuneReinforcementHyperparametersNEpochs>,
    #[doc = "Level of reasoning effort.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort:
        ::std::option::Option<FineTuneReinforcementHyperparametersReasoningEffort>,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersBatchSize {
    String(FineTuneReinforcementHyperparametersBatchSizeString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersBatchSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersBatchSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersBatchSize
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersBatchSizeString>
    for FineTuneReinforcementHyperparametersBatchSize
{
    fn from(value: FineTuneReinforcementHyperparametersBatchSizeString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64>
    for FineTuneReinforcementHyperparametersBatchSize
{
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersBatchSizeString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersBatchSizeString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersBatchSizeString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersBatchSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersBatchSizeString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Multiplier on amount of compute used for exploring search space during training.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersComputeMultiplier {
    String(FineTuneReinforcementHyperparametersComputeMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersComputeMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersComputeMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersComputeMultiplier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersComputeMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersComputeMultiplierString>
    for FineTuneReinforcementHyperparametersComputeMultiplier
{
    fn from(value: FineTuneReinforcementHyperparametersComputeMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuneReinforcementHyperparametersComputeMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersComputeMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersComputeMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersComputeMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersComputeMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersComputeMultiplierString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersComputeMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of training steps between evaluation runs.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalInterval {
    String(FineTuneReinforcementHyperparametersEvalIntervalString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersEvalInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersEvalInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersEvalInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersEvalInterval
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersEvalIntervalString>
    for FineTuneReinforcementHyperparametersEvalInterval
{
    fn from(value: FineTuneReinforcementHyperparametersEvalIntervalString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64>
    for FineTuneReinforcementHyperparametersEvalInterval
{
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersEvalIntervalString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersEvalIntervalString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersEvalIntervalString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersEvalIntervalString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersEvalIntervalString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersEvalIntervalString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Number of evaluation samples to generate per training step.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalSamples {
    String(FineTuneReinforcementHyperparametersEvalSamplesString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersEvalSamples {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersEvalSamples {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersEvalSamples {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersEvalSamples
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersEvalSamplesString>
    for FineTuneReinforcementHyperparametersEvalSamples
{
    fn from(value: FineTuneReinforcementHyperparametersEvalSamplesString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64>
    for FineTuneReinforcementHyperparametersEvalSamples
{
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersEvalSamplesString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersEvalSamplesString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersEvalSamplesString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersEvalSamplesString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersEvalSamplesString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersEvalSamplesString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersLearningRateMultiplier {
    String(FineTuneReinforcementHyperparametersLearningRateMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersLearningRateMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersLearningRateMultiplier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersLearningRateMultiplierString>
    for FineTuneReinforcementHyperparametersLearningRateMultiplier
{
    fn from(value: FineTuneReinforcementHyperparametersLearningRateMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuneReinforcementHyperparametersLearningRateMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersLearningRateMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersLearningRateMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersLearningRateMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersLearningRateMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for FineTuneReinforcementHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersNEpochs {
    String(FineTuneReinforcementHyperparametersNEpochsString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersNEpochs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersNEpochs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersNEpochs
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneReinforcementHyperparametersNEpochsString>
    for FineTuneReinforcementHyperparametersNEpochs
{
    fn from(value: FineTuneReinforcementHyperparametersNEpochsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuneReinforcementHyperparametersNEpochs {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneReinforcementHyperparametersNEpochsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersNEpochsString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersNEpochsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersNEpochsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersNEpochsString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Level of reasoning effort.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneReinforcementHyperparametersReasoningEffort {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for FineTuneReinforcementHyperparametersReasoningEffort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for FineTuneReinforcementHyperparametersReasoningEffort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneReinforcementHyperparametersReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersReasoningEffort
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for the reinforcement fine-tuning method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuneReinforcementMethod {
    #[doc = "The grader used for the fine-tuning job."]
    pub grader: FineTuneReinforcementMethodGrader,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hyperparameters: ::std::option::Option<FineTuneReinforcementHyperparameters>,
}
#[doc = "The grader used for the fine-tuning job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneReinforcementMethodGrader {
    StringCheck(GraderStringCheck),
    TextSimilarity(GraderTextSimilarity),
    Python(GraderPython),
    ScoreModel(GraderScoreModel),
    Multi(GraderMulti),
}
impl ::std::convert::From<GraderStringCheck> for FineTuneReinforcementMethodGrader {
    fn from(value: GraderStringCheck) -> Self {
        Self::StringCheck(value)
    }
}
impl ::std::convert::From<GraderTextSimilarity> for FineTuneReinforcementMethodGrader {
    fn from(value: GraderTextSimilarity) -> Self {
        Self::TextSimilarity(value)
    }
}
impl ::std::convert::From<GraderPython> for FineTuneReinforcementMethodGrader {
    fn from(value: GraderPython) -> Self {
        Self::Python(value)
    }
}
impl ::std::convert::From<GraderScoreModel> for FineTuneReinforcementMethodGrader {
    fn from(value: GraderScoreModel) -> Self {
        Self::ScoreModel(value)
    }
}
impl ::std::convert::From<GraderMulti> for FineTuneReinforcementMethodGrader {
    fn from(value: GraderMulti) -> Self {
        Self::Multi(value)
    }
}
#[doc = "The hyperparameters used for the fine-tuning job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuneSupervisedHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size: ::std::option::Option<FineTuneSupervisedHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub learning_rate_multiplier:
        ::std::option::Option<FineTuneSupervisedHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n_epochs: ::std::option::Option<FineTuneSupervisedHyperparametersNEpochs>,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersBatchSize {
    String(FineTuneSupervisedHyperparametersBatchSizeString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersBatchSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersBatchSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneSupervisedHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneSupervisedHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneSupervisedHyperparametersBatchSizeString>
    for FineTuneSupervisedHyperparametersBatchSize
{
    fn from(value: FineTuneSupervisedHyperparametersBatchSizeString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuneSupervisedHyperparametersBatchSize {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneSupervisedHyperparametersBatchSizeString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneSupervisedHyperparametersBatchSizeString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersBatchSizeString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersBatchSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneSupervisedHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneSupervisedHyperparametersBatchSizeString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersLearningRateMultiplier {
    String(FineTuneSupervisedHyperparametersLearningRateMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersLearningRateMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneSupervisedHyperparametersLearningRateMultiplier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneSupervisedHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneSupervisedHyperparametersLearningRateMultiplierString>
    for FineTuneSupervisedHyperparametersLearningRateMultiplier
{
    fn from(value: FineTuneSupervisedHyperparametersLearningRateMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuneSupervisedHyperparametersLearningRateMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuneSupervisedHyperparametersLearningRateMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneSupervisedHyperparametersLearningRateMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersLearningRateMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersLearningRateMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for FineTuneSupervisedHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneSupervisedHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersNEpochs {
    String(FineTuneSupervisedHyperparametersNEpochsString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersNEpochs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersNEpochs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneSupervisedHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuneSupervisedHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuneSupervisedHyperparametersNEpochsString>
    for FineTuneSupervisedHyperparametersNEpochs
{
    fn from(value: FineTuneSupervisedHyperparametersNEpochsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuneSupervisedHyperparametersNEpochs {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuneSupervisedHyperparametersNEpochsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuneSupervisedHyperparametersNEpochsString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuneSupervisedHyperparametersNEpochsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuneSupervisedHyperparametersNEpochsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuneSupervisedHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneSupervisedHyperparametersNEpochsString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for the supervised fine-tuning method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuneSupervisedMethod {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hyperparameters: ::std::option::Option<FineTuneSupervisedHyperparameters>,
}
#[doc = "`FineTuningApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningApiSchema {
    #[serde(rename = "CancelFineTuningJobResponse")]
    pub cancel_fine_tuning_job_response: FineTuningJob,
    #[serde(rename = "CreateFineTuningCheckpointPermissionRequest")]
    pub create_fine_tuning_checkpoint_permission_request:
        CreateFineTuningCheckpointPermissionRequest,
    #[serde(rename = "CreateFineTuningCheckpointPermissionResponse")]
    pub create_fine_tuning_checkpoint_permission_response:
        ListFineTuningCheckpointPermissionResponse,
    #[serde(rename = "CreateFineTuningJobRequest")]
    pub create_fine_tuning_job_request: CreateFineTuningJobRequest,
    #[serde(rename = "CreateFineTuningJobResponse")]
    pub create_fine_tuning_job_response: FineTuningJob,
    #[serde(rename = "DeleteFineTuningCheckpointPermissionResponse")]
    pub delete_fine_tuning_checkpoint_permission_response:
        DeleteFineTuningCheckpointPermissionResponse,
    #[serde(rename = "ListFineTuningCheckpointPermissionsParams")]
    pub list_fine_tuning_checkpoint_permissions_params: ListFineTuningCheckpointPermissionsParams,
    #[serde(rename = "ListFineTuningCheckpointPermissionsResponse")]
    pub list_fine_tuning_checkpoint_permissions_response:
        ListFineTuningCheckpointPermissionResponse,
    #[serde(rename = "ListFineTuningEventsParams")]
    pub list_fine_tuning_events_params: ListFineTuningEventsParams,
    #[serde(rename = "ListFineTuningEventsResponse")]
    pub list_fine_tuning_events_response: ListFineTuningJobEventsResponse,
    #[serde(rename = "ListFineTuningJobCheckpointsParams")]
    pub list_fine_tuning_job_checkpoints_params: ListFineTuningJobCheckpointsParams,
    #[serde(rename = "ListFineTuningJobCheckpointsResponse")]
    pub list_fine_tuning_job_checkpoints_response: ListFineTuningJobCheckpointsResponse,
    #[serde(rename = "ListPaginatedFineTuningJobsParams")]
    pub list_paginated_fine_tuning_jobs_params: ListPaginatedFineTuningJobsParams,
    #[serde(rename = "ListPaginatedFineTuningJobsResponse")]
    pub list_paginated_fine_tuning_jobs_response: ListPaginatedFineTuningJobsResponse,
    #[serde(rename = "PauseFineTuningJobResponse")]
    pub pause_fine_tuning_job_response: FineTuningJob,
    #[serde(rename = "ResumeFineTuningJobResponse")]
    pub resume_fine_tuning_job_response: FineTuningJob,
    #[serde(rename = "RetrieveFineTuningJobResponse")]
    pub retrieve_fine_tuning_job_response: FineTuningJob,
}
#[doc = "The `checkpoint.permission` object represents a permission for a fine-tuned model checkpoint.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningCheckpointPermission {
    #[doc = "The Unix timestamp (in seconds) for when the permission was created."]
    pub created_at: i64,
    #[doc = "The permission identifier, which can be referenced in the API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    pub object: FineTuningCheckpointPermissionObject,
    #[doc = "The project identifier that the permission is for."]
    pub project_id: ::std::string::String,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningCheckpointPermissionObject {
    #[serde(rename = "checkpoint.permission")]
    CheckpointPermission,
}
impl ::std::fmt::Display for FineTuningCheckpointPermissionObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CheckpointPermission => f.write_str("checkpoint.permission"),
        }
    }
}
impl ::std::str::FromStr for FineTuningCheckpointPermissionObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "checkpoint.permission" => Ok(Self::CheckpointPermission),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningCheckpointPermissionObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningCheckpointPermissionObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FineTuningIntegration`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningIntegration {
    #[doc = "The type of the integration being enabled for the fine-tuning job"]
    #[serde(rename = "type")]
    pub type_: FineTuningIntegrationType,
    pub wandb: FineTuningIntegrationWandb,
}
#[doc = "The type of the integration being enabled for the fine-tuning job"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningIntegrationType {
    #[serde(rename = "wandb")]
    Wandb,
}
impl ::std::fmt::Display for FineTuningIntegrationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wandb => f.write_str("wandb"),
        }
    }
}
impl ::std::str::FromStr for FineTuningIntegrationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wandb" => Ok(Self::Wandb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningIntegrationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningIntegrationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningIntegrationWandb {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub entity: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the project that the new run will be created under.\n"]
    pub project: ::std::string::String,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
#[doc = "The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningJob {
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    pub created_at: i64,
    pub error: FineTuningJobError,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub estimated_finish: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub fine_tuned_model: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub finished_at: ::std::option::Option<i64>,
    #[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hyperparameters: ::std::option::Option<FineTuningJobHyperparameters>,
    #[doc = "The object identifier, which can be referenced in the API endpoints."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub integrations: ::std::option::Option<::std::vec::Vec<FineTuningIntegration>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub method: ::std::option::Option<FineTuneMethod>,
    #[doc = "The base model that is being fine-tuned."]
    pub model: ::std::string::String,
    #[doc = "The object type, which is always \"fine_tuning.job\"."]
    pub object: FineTuningJobObject,
    #[doc = "The organization that owns the fine-tuning job."]
    pub organization_id: ::std::string::String,
    #[doc = "The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](https://developers.openai.com/api/reference/resources/files/methods/content)."]
    pub result_files: ::std::vec::Vec<::std::string::String>,
    #[doc = "The seed used for the fine-tuning job."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub seed: ::std::option::Option<i64>,
    #[doc = "The current status of the fine-tuning job, which can be `validating_files`, `queued`, `running`, `pausing`, `paused`, `succeeded`, `failed`, or `cancelled`."]
    pub status: FineTuningJobStatus,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub trained_tokens: ::std::option::Option<i64>,
    #[doc = "The file ID used for training. You can retrieve the training data with the [Files API](https://developers.openai.com/api/reference/resources/files/methods/content)."]
    pub training_file: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub validation_file: ::std::option::Option<::std::string::String>,
}
#[doc = "The `fine_tuning.job.checkpoint` object represents a model checkpoint for a fine-tuning job that is ready to use.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningJobCheckpoint {
    #[doc = "The Unix timestamp (in seconds) for when the checkpoint was created."]
    pub created_at: i64,
    #[doc = "The name of the fine-tuned checkpoint model that is created."]
    pub fine_tuned_model_checkpoint: ::std::string::String,
    #[doc = "The name of the fine-tuning job that this checkpoint was created from."]
    pub fine_tuning_job_id: ::std::string::String,
    #[doc = "The checkpoint identifier, which can be referenced in the API endpoints."]
    pub id: ::std::string::String,
    pub metrics: FineTuningJobCheckpointMetrics,
    #[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
    pub object: FineTuningJobCheckpointObject,
    #[doc = "The step number that the checkpoint was created at."]
    pub step_number: i64,
}
#[doc = "Metrics at the step number during the fine-tuning job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuningJobCheckpointMetrics {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub full_valid_loss: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub full_valid_mean_token_accuracy: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub step: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub train_loss: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub train_mean_token_accuracy: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub valid_loss: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub valid_mean_token_accuracy: ::std::option::Option<f64>,
}
#[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobCheckpointObject {
    #[serde(rename = "fine_tuning.job.checkpoint")]
    FineTuningJobCheckpoint,
}
impl ::std::fmt::Display for FineTuningJobCheckpointObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FineTuningJobCheckpoint => f.write_str("fine_tuning.job.checkpoint"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobCheckpointObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fine_tuning.job.checkpoint" => Ok(Self::FineTuningJobCheckpoint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobCheckpointObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobCheckpointObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FineTuningJobError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuningJobError {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<FineTuningJobErrorSubtype0>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<()>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
}
#[doc = "For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningJobErrorSubtype0 {
    #[doc = "A machine-readable error code."]
    pub code: ::std::string::String,
    #[doc = "A human-readable error message."]
    pub message: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub param: ::std::option::Option<::std::string::String>,
}
#[doc = "Fine-tuning job event object"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FineTuningJobEvent {
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    pub created_at: i64,
    #[doc = "The data associated with the event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "The object identifier."]
    pub id: ::std::string::String,
    #[doc = "The log level of the event."]
    pub level: FineTuningJobEventLevel,
    #[doc = "The message of the event."]
    pub message: ::std::string::String,
    #[doc = "The object type, which is always \"fine_tuning.job.event\"."]
    pub object: FineTuningJobEventObject,
    #[doc = "The type of event."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<FineTuningJobEventType>,
}
#[doc = "The log level of the event."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobEventLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for FineTuningJobEventLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Info => f.write_str("info"),
            Self::Warn => f.write_str("warn"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobEventLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobEventLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobEventLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always \"fine_tuning.job.event\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobEventObject {
    #[serde(rename = "fine_tuning.job.event")]
    FineTuningJobEvent,
}
impl ::std::fmt::Display for FineTuningJobEventObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FineTuningJobEvent => f.write_str("fine_tuning.job.event"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobEventObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fine_tuning.job.event" => Ok(Self::FineTuningJobEvent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobEventObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobEventObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of event."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobEventType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "metrics")]
    Metrics,
}
impl ::std::fmt::Display for FineTuningJobEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
            Self::Metrics => f.write_str("metrics"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            "metrics" => Ok(Self::Metrics),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FineTuningJobHyperparameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_size: ::std::option::Option<FineTuningJobHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub learning_rate_multiplier:
        ::std::option::Option<FineTuningJobHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n_epochs: ::std::option::Option<FineTuningJobHyperparametersNEpochs>,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersBatchSize {
    String(FineTuningJobHyperparametersBatchSizeString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuningJobHyperparametersBatchSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersBatchSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobHyperparametersBatchSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuningJobHyperparametersBatchSizeString>
    for FineTuningJobHyperparametersBatchSize
{
    fn from(value: FineTuningJobHyperparametersBatchSizeString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuningJobHyperparametersBatchSize {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuningJobHyperparametersBatchSizeString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobHyperparametersBatchSizeString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuningJobHyperparametersBatchSizeString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersBatchSizeString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersBatchSizeString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuningJobHyperparametersBatchSizeString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersLearningRateMultiplier {
    String(FineTuningJobHyperparametersLearningRateMultiplierString),
    Number(f64),
}
impl ::std::fmt::Display for FineTuningJobHyperparametersLearningRateMultiplier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersLearningRateMultiplier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersLearningRateMultiplier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuningJobHyperparametersLearningRateMultiplier
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuningJobHyperparametersLearningRateMultiplierString>
    for FineTuningJobHyperparametersLearningRateMultiplier
{
    fn from(value: FineTuningJobHyperparametersLearningRateMultiplierString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<f64> for FineTuningJobHyperparametersLearningRateMultiplier {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`FineTuningJobHyperparametersLearningRateMultiplierString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobHyperparametersLearningRateMultiplierString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuningJobHyperparametersLearningRateMultiplierString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersLearningRateMultiplierString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersLearningRateMultiplierString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FineTuningJobHyperparametersLearningRateMultiplierString
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersNEpochs {
    String(FineTuningJobHyperparametersNEpochsString),
    Integer(::std::num::NonZeroU64),
}
impl ::std::fmt::Display for FineTuningJobHyperparametersNEpochs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersNEpochs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobHyperparametersNEpochs {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<FineTuningJobHyperparametersNEpochsString>
    for FineTuningJobHyperparametersNEpochs
{
    fn from(value: FineTuningJobHyperparametersNEpochsString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for FineTuningJobHyperparametersNEpochs {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "`FineTuningJobHyperparametersNEpochsString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobHyperparametersNEpochsString {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for FineTuningJobHyperparametersNEpochsString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobHyperparametersNEpochsString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobHyperparametersNEpochsString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always \"fine_tuning.job\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobObject {
    #[serde(rename = "fine_tuning.job")]
    FineTuningJob,
}
impl ::std::fmt::Display for FineTuningJobObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FineTuningJob => f.write_str("fine_tuning.job"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fine_tuning.job" => Ok(Self::FineTuningJob),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The current status of the fine-tuning job, which can be `validating_files`, `queued`, `running`, `pausing`, `paused`, `succeeded`, `failed`, or `cancelled`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FineTuningJobStatus {
    #[serde(rename = "validating_files")]
    ValidatingFiles,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "pausing")]
    Pausing,
    #[serde(rename = "paused")]
    Paused,
}
impl ::std::fmt::Display for FineTuningJobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ValidatingFiles => f.write_str("validating_files"),
            Self::Queued => f.write_str("queued"),
            Self::Running => f.write_str("running"),
            Self::Succeeded => f.write_str("succeeded"),
            Self::Failed => f.write_str("failed"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Pausing => f.write_str("pausing"),
            Self::Paused => f.write_str("paused"),
        }
    }
}
impl ::std::str::FromStr for FineTuningJobStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "validating_files" => Ok(Self::ValidatingFiles),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "pausing" => Ok(Self::Pausing),
            "paused" => Ok(Self::Paused),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FineTuningJobStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FineTuningJobStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderLabelModel {
    pub input: ::std::vec::Vec<EvalItem>,
    #[doc = "The labels to assign to each item in the evaluation."]
    pub labels: ::std::vec::Vec<::std::string::String>,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    pub model: ::std::string::String,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    pub passing_labels: ::std::vec::Vec<::std::string::String>,
    #[doc = "The object type, which is always `label_model`."]
    #[serde(rename = "type")]
    pub type_: GraderLabelModelType,
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
pub enum GraderLabelModelType {
    #[serde(rename = "label_model")]
    LabelModel,
}
impl ::std::fmt::Display for GraderLabelModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LabelModel => f.write_str("label_model"),
        }
    }
}
impl ::std::str::FromStr for GraderLabelModelType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "label_model" => Ok(Self::LabelModel),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderLabelModelType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderLabelModelType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A MultiGrader object combines the output of multiple graders to produce a single score."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderMulti {
    #[doc = "A formula to calculate the output based on grader results."]
    pub calculate_output: ::std::string::String,
    pub graders: ::std::collections::HashMap<::std::string::String, GraderMultiGradersValue>,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `multi`."]
    #[serde(rename = "type")]
    pub type_: GraderMultiType,
}
#[doc = "`GraderMultiGradersValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum GraderMultiGradersValue {
    StringCheck(GraderStringCheck),
    TextSimilarity(GraderTextSimilarity),
    Python(GraderPython),
    ScoreModel(GraderScoreModel),
    LabelModel(GraderLabelModel),
}
impl ::std::convert::From<GraderStringCheck> for GraderMultiGradersValue {
    fn from(value: GraderStringCheck) -> Self {
        Self::StringCheck(value)
    }
}
impl ::std::convert::From<GraderTextSimilarity> for GraderMultiGradersValue {
    fn from(value: GraderTextSimilarity) -> Self {
        Self::TextSimilarity(value)
    }
}
impl ::std::convert::From<GraderPython> for GraderMultiGradersValue {
    fn from(value: GraderPython) -> Self {
        Self::Python(value)
    }
}
impl ::std::convert::From<GraderScoreModel> for GraderMultiGradersValue {
    fn from(value: GraderScoreModel) -> Self {
        Self::ScoreModel(value)
    }
}
impl ::std::convert::From<GraderLabelModel> for GraderMultiGradersValue {
    fn from(value: GraderLabelModel) -> Self {
        Self::LabelModel(value)
    }
}
#[doc = "The object type, which is always `multi`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderMultiType {
    #[serde(rename = "multi")]
    Multi,
}
impl ::std::fmt::Display for GraderMultiType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Multi => f.write_str("multi"),
        }
    }
}
impl ::std::str::FromStr for GraderMultiType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "multi" => Ok(Self::Multi),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderMultiType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderMultiType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A PythonGrader object that runs a python script on the input.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderPython {
    #[doc = "The image tag to use for the python script."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_tag: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The source code of the python script."]
    pub source: ::std::string::String,
    #[doc = "The object type, which is always `python`."]
    #[serde(rename = "type")]
    pub type_: GraderPythonType,
}
#[doc = "The object type, which is always `python`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderPythonType {
    #[serde(rename = "python")]
    Python,
}
impl ::std::fmt::Display for GraderPythonType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Python => f.write_str("python"),
        }
    }
}
impl ::std::str::FromStr for GraderPythonType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "python" => Ok(Self::Python),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderPythonType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderPythonType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A ScoreModelGrader object that uses a model to assign a score to the input.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderScoreModel {
    #[doc = "The input messages evaluated by the grader. Supports text, output text, input image, and input audio content blocks, and may include template strings.\n"]
    pub input: ::std::vec::Vec<EvalItem>,
    #[doc = "The model to use for the evaluation."]
    pub model: ::std::string::String,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The service requires two numbers for the score range. Defaults to `[0, 1]`.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub range: ::std::vec::Vec<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sampling_params: ::std::option::Option<GraderScoreModelSamplingParams>,
    #[doc = "The object type, which is always `score_model`."]
    #[serde(rename = "type")]
    pub type_: GraderScoreModelType,
}
#[doc = "The sampling parameters for the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct GraderScoreModelSamplingParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completions_tokens: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
}
#[doc = "The object type, which is always `score_model`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderScoreModelType {
    #[serde(rename = "score_model")]
    ScoreModel,
}
impl ::std::fmt::Display for GraderScoreModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ScoreModel => f.write_str("score_model"),
        }
    }
}
impl ::std::str::FromStr for GraderScoreModelType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score_model" => Ok(Self::ScoreModel),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderScoreModelType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderScoreModelType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A StringCheckGrader object that performs a string comparison between input and reference using a specified operation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderStringCheck {
    #[doc = "The input text. This may include template strings."]
    pub input: ::std::string::String,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
    pub operation: GraderStringCheckOperation,
    #[doc = "The reference text. This may include template strings."]
    pub reference: ::std::string::String,
    #[doc = "The object type, which is always `string_check`."]
    #[serde(rename = "type")]
    pub type_: GraderStringCheckType,
}
#[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderStringCheckOperation {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "like")]
    Like,
    #[serde(rename = "ilike")]
    Ilike,
}
impl ::std::fmt::Display for GraderStringCheckOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Eq => f.write_str("eq"),
            Self::Ne => f.write_str("ne"),
            Self::Like => f.write_str("like"),
            Self::Ilike => f.write_str("ilike"),
        }
    }
}
impl ::std::str::FromStr for GraderStringCheckOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "like" => Ok(Self::Like),
            "ilike" => Ok(Self::Ilike),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderStringCheckOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderStringCheckOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `string_check`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderStringCheckType {
    #[serde(rename = "string_check")]
    StringCheck,
}
impl ::std::fmt::Display for GraderStringCheckType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StringCheck => f.write_str("string_check"),
        }
    }
}
impl ::std::str::FromStr for GraderStringCheckType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string_check" => Ok(Self::StringCheck),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderStringCheckType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderStringCheckType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A TextSimilarityGrader object which grades text based on similarity metrics.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GraderTextSimilarity {
    #[doc = "The evaluation metric to use. One of `cosine`, `fuzzy_match`, `bleu`, \n`gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, \nor `rouge_l`.\n"]
    pub evaluation_metric: GraderTextSimilarityEvaluationMetric,
    #[doc = "The text being graded."]
    pub input: ::std::string::String,
    #[doc = "The name of the grader."]
    pub name: ::std::string::String,
    #[doc = "The text being graded against."]
    pub reference: ::std::string::String,
    #[doc = "The type of grader."]
    #[serde(rename = "type")]
    pub type_: GraderTextSimilarityType,
}
#[doc = "The evaluation metric to use. One of `cosine`, `fuzzy_match`, `bleu`, \n`gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, \nor `rouge_l`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderTextSimilarityEvaluationMetric {
    #[serde(rename = "cosine")]
    Cosine,
    #[serde(rename = "fuzzy_match")]
    FuzzyMatch,
    #[serde(rename = "bleu")]
    Bleu,
    #[serde(rename = "gleu")]
    Gleu,
    #[serde(rename = "meteor")]
    Meteor,
    #[serde(rename = "rouge_1")]
    Rouge1,
    #[serde(rename = "rouge_2")]
    Rouge2,
    #[serde(rename = "rouge_3")]
    Rouge3,
    #[serde(rename = "rouge_4")]
    Rouge4,
    #[serde(rename = "rouge_5")]
    Rouge5,
    #[serde(rename = "rouge_l")]
    RougeL,
}
impl ::std::fmt::Display for GraderTextSimilarityEvaluationMetric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cosine => f.write_str("cosine"),
            Self::FuzzyMatch => f.write_str("fuzzy_match"),
            Self::Bleu => f.write_str("bleu"),
            Self::Gleu => f.write_str("gleu"),
            Self::Meteor => f.write_str("meteor"),
            Self::Rouge1 => f.write_str("rouge_1"),
            Self::Rouge2 => f.write_str("rouge_2"),
            Self::Rouge3 => f.write_str("rouge_3"),
            Self::Rouge4 => f.write_str("rouge_4"),
            Self::Rouge5 => f.write_str("rouge_5"),
            Self::RougeL => f.write_str("rouge_l"),
        }
    }
}
impl ::std::str::FromStr for GraderTextSimilarityEvaluationMetric {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cosine" => Ok(Self::Cosine),
            "fuzzy_match" => Ok(Self::FuzzyMatch),
            "bleu" => Ok(Self::Bleu),
            "gleu" => Ok(Self::Gleu),
            "meteor" => Ok(Self::Meteor),
            "rouge_1" => Ok(Self::Rouge1),
            "rouge_2" => Ok(Self::Rouge2),
            "rouge_3" => Ok(Self::Rouge3),
            "rouge_4" => Ok(Self::Rouge4),
            "rouge_5" => Ok(Self::Rouge5),
            "rouge_l" => Ok(Self::RougeL),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderTextSimilarityEvaluationMetric {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderTextSimilarityEvaluationMetric {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of grader."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GraderTextSimilarityType {
    #[serde(rename = "text_similarity")]
    TextSimilarity,
}
impl ::std::fmt::Display for GraderTextSimilarityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TextSimilarity => f.write_str("text_similarity"),
        }
    }
}
impl ::std::str::FromStr for GraderTextSimilarityType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text_similarity" => Ok(Self::TextSimilarity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraderTextSimilarityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraderTextSimilarityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An audio input to the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputAudio {
    pub input_audio: InputAudioInputAudio,
    #[doc = "The type of the input item. Always `input_audio`.\n"]
    #[serde(rename = "type")]
    pub type_: InputAudioType,
}
#[doc = "`InputAudioInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputAudioInputAudio {
    #[doc = "Base64-encoded audio data.\n"]
    pub data: ::std::string::String,
    #[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
    pub format: InputAudioInputAudioFormat,
}
#[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputAudioInputAudioFormat {
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "wav")]
    Wav,
}
impl ::std::fmt::Display for InputAudioInputAudioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mp3 => f.write_str("mp3"),
            Self::Wav => f.write_str("wav"),
        }
    }
}
impl ::std::str::FromStr for InputAudioInputAudioFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mp3" => Ok(Self::Mp3),
            "wav" => Ok(Self::Wav),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputAudioInputAudioFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputAudioInputAudioFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the input item. Always `input_audio`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputAudioType {
    #[serde(rename = "input_audio")]
    InputAudio,
}
impl ::std::fmt::Display for InputAudioType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputAudio => f.write_str("input_audio"),
        }
    }
}
impl ::std::str::FromStr for InputAudioType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_audio" => Ok(Self::InputAudio),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputAudioType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputAudioType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputTextContent {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The text input to the model."]
    pub text: ::std::string::String,
    #[doc = "The type of the input item. Always `input_text`."]
    #[serde(rename = "type")]
    pub type_: InputTextContentType,
}
#[doc = "The type of the input item. Always `input_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputTextContentType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for InputTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for InputTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFineTuningCheckpointPermissionResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListFineTuningCheckpointPermissionResponse {
    pub data: ::std::vec::Vec<FineTuningCheckpointPermission>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListFineTuningCheckpointPermissionResponseObject,
}
#[doc = "`ListFineTuningCheckpointPermissionResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListFineTuningCheckpointPermissionResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListFineTuningCheckpointPermissionResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListFineTuningCheckpointPermissionResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFineTuningCheckpointPermissionResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ListFineTuningCheckpointPermissionResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFineTuningCheckpointPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListFineTuningCheckpointPermissionsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListFineTuningCheckpointPermissionsParamsOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[doc = "`ListFineTuningCheckpointPermissionsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListFineTuningCheckpointPermissionsParamsOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}
impl ::std::fmt::Display for ListFineTuningCheckpointPermissionsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ascending => f.write_str("ascending"),
            Self::Descending => f.write_str("descending"),
        }
    }
}
impl ::std::str::FromStr for ListFineTuningCheckpointPermissionsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ascending" => Ok(Self::Ascending),
            "descending" => Ok(Self::Descending),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFineTuningCheckpointPermissionsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ListFineTuningCheckpointPermissionsParamsOrder
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFineTuningEventsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListFineTuningEventsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
#[doc = "`ListFineTuningJobCheckpointsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListFineTuningJobCheckpointsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
#[doc = "`ListFineTuningJobCheckpointsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListFineTuningJobCheckpointsResponse {
    pub data: ::std::vec::Vec<FineTuningJobCheckpoint>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListFineTuningJobCheckpointsResponseObject,
}
#[doc = "`ListFineTuningJobCheckpointsResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListFineTuningJobCheckpointsResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListFineTuningJobCheckpointsResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListFineTuningJobCheckpointsResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFineTuningJobCheckpointsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListFineTuningJobCheckpointsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListFineTuningJobEventsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListFineTuningJobEventsResponse {
    pub data: ::std::vec::Vec<FineTuningJobEvent>,
    pub has_more: bool,
    pub object: ListFineTuningJobEventsResponseObject,
}
#[doc = "`ListFineTuningJobEventsResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListFineTuningJobEventsResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListFineTuningJobEventsResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListFineTuningJobEventsResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListFineTuningJobEventsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListFineTuningJobEventsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListPaginatedFineTuningJobsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListPaginatedFineTuningJobsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
#[doc = "`ListPaginatedFineTuningJobsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListPaginatedFineTuningJobsResponse {
    pub data: ::std::vec::Vec<FineTuningJob>,
    pub has_more: bool,
    pub object: ListPaginatedFineTuningJobsResponseObject,
}
#[doc = "`ListPaginatedFineTuningJobsResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListPaginatedFineTuningJobsResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListPaginatedFineTuningJobsResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListPaginatedFineTuningJobsResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListPaginatedFineTuningJobsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListPaginatedFineTuningJobsResponseObject {
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
#[doc = "Marks the exact end of a reusable prompt prefix. The breakpoint inherits its TTL from the request's `prompt_cache_options.ttl`; the boundary is not rounded to a token block."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheBreakpointConfig {
    #[doc = "The breakpoint mode. Always `explicit`."]
    pub mode: PromptCacheBreakpointConfigMode,
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
pub enum PromptCacheBreakpointConfigMode {
    #[serde(rename = "explicit")]
    Explicit,
}
impl ::std::fmt::Display for PromptCacheBreakpointConfigMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Explicit => f.write_str("explicit"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheBreakpointConfigMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "explicit" => Ok(Self::Explicit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheBreakpointConfigMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheBreakpointConfigMode {
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
