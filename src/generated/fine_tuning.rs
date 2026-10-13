#[doc = "`CreateFineTuningCheckpointPermissionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateFineTuningCheckpointPermissionRequest {
    #[doc = "The project identifiers to grant access to."]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
}
impl CreateFineTuningCheckpointPermissionRequest {
    pub fn builder() -> builder::CreateFineTuningCheckpointPermissionRequest {
        Default::default()
    }
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
impl CreateFineTuningJobRequest {
    pub fn builder() -> builder::CreateFineTuningJobRequest {
        Default::default()
    }
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
impl CreateFineTuningJobRequestHyperparameters {
    pub fn builder() -> builder::CreateFineTuningJobRequestHyperparameters {
        Default::default()
    }
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
impl CreateFineTuningJobRequestIntegrationsItem {
    pub fn builder() -> builder::CreateFineTuningJobRequestIntegrationsItem {
        Default::default()
    }
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
impl CreateFineTuningJobRequestIntegrationsItemWandb {
    pub fn builder() -> builder::CreateFineTuningJobRequestIntegrationsItemWandb {
        Default::default()
    }
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
impl DeleteFineTuningCheckpointPermissionResponse {
    pub fn builder() -> builder::DeleteFineTuningCheckpointPermissionResponse {
        Default::default()
    }
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
impl EvalItem {
    pub fn builder() -> builder::EvalItem {
        Default::default()
    }
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
impl EvalItemContentOutputText {
    pub fn builder() -> builder::EvalItemContentOutputText {
        Default::default()
    }
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
impl EvalItemInputImage {
    pub fn builder() -> builder::EvalItemInputImage {
        Default::default()
    }
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
impl FineTuneDpoHyperparameters {
    pub fn builder() -> builder::FineTuneDpoHyperparameters {
        Default::default()
    }
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
impl FineTuneDpoMethod {
    pub fn builder() -> builder::FineTuneDpoMethod {
        Default::default()
    }
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
impl FineTuneMethod {
    pub fn builder() -> builder::FineTuneMethod {
        Default::default()
    }
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
impl FineTuneReinforcementHyperparameters {
    pub fn builder() -> builder::FineTuneReinforcementHyperparameters {
        Default::default()
    }
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
impl FineTuneReinforcementMethod {
    pub fn builder() -> builder::FineTuneReinforcementMethod {
        Default::default()
    }
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
impl FineTuneSupervisedHyperparameters {
    pub fn builder() -> builder::FineTuneSupervisedHyperparameters {
        Default::default()
    }
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
impl FineTuneSupervisedMethod {
    pub fn builder() -> builder::FineTuneSupervisedMethod {
        Default::default()
    }
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
impl FineTuningApiSchema {
    pub fn builder() -> builder::FineTuningApiSchema {
        Default::default()
    }
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
impl FineTuningCheckpointPermission {
    pub fn builder() -> builder::FineTuningCheckpointPermission {
        Default::default()
    }
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
impl FineTuningIntegration {
    pub fn builder() -> builder::FineTuningIntegration {
        Default::default()
    }
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
impl FineTuningIntegrationWandb {
    pub fn builder() -> builder::FineTuningIntegrationWandb {
        Default::default()
    }
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
impl FineTuningJob {
    pub fn builder() -> builder::FineTuningJob {
        Default::default()
    }
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
impl FineTuningJobCheckpoint {
    pub fn builder() -> builder::FineTuningJobCheckpoint {
        Default::default()
    }
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
impl FineTuningJobCheckpointMetrics {
    pub fn builder() -> builder::FineTuningJobCheckpointMetrics {
        Default::default()
    }
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
impl FineTuningJobError {
    pub fn builder() -> builder::FineTuningJobError {
        Default::default()
    }
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
impl FineTuningJobErrorSubtype0 {
    pub fn builder() -> builder::FineTuningJobErrorSubtype0 {
        Default::default()
    }
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
impl FineTuningJobEvent {
    pub fn builder() -> builder::FineTuningJobEvent {
        Default::default()
    }
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
impl FineTuningJobHyperparameters {
    pub fn builder() -> builder::FineTuningJobHyperparameters {
        Default::default()
    }
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
impl GraderLabelModel {
    pub fn builder() -> builder::GraderLabelModel {
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
impl GraderMulti {
    pub fn builder() -> builder::GraderMulti {
        Default::default()
    }
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
impl GraderPython {
    pub fn builder() -> builder::GraderPython {
        Default::default()
    }
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
impl GraderScoreModel {
    pub fn builder() -> builder::GraderScoreModel {
        Default::default()
    }
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
impl GraderScoreModelSamplingParams {
    pub fn builder() -> builder::GraderScoreModelSamplingParams {
        Default::default()
    }
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
impl GraderStringCheck {
    pub fn builder() -> builder::GraderStringCheck {
        Default::default()
    }
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
impl GraderTextSimilarity {
    pub fn builder() -> builder::GraderTextSimilarity {
        Default::default()
    }
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
impl InputAudio {
    pub fn builder() -> builder::InputAudio {
        Default::default()
    }
}
#[doc = "`InputAudioInputAudio`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputAudioInputAudio {
    #[doc = "Base64-encoded audio data.\n"]
    pub data: ::std::string::String,
    #[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
    pub format: InputAudioInputAudioFormat,
}
impl InputAudioInputAudio {
    pub fn builder() -> builder::InputAudioInputAudio {
        Default::default()
    }
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
impl InputTextContent {
    pub fn builder() -> builder::InputTextContent {
        Default::default()
    }
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
impl ListFineTuningCheckpointPermissionResponse {
    pub fn builder() -> builder::ListFineTuningCheckpointPermissionResponse {
        Default::default()
    }
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
impl ListFineTuningCheckpointPermissionsParams {
    pub fn builder() -> builder::ListFineTuningCheckpointPermissionsParams {
        Default::default()
    }
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
impl ListFineTuningEventsParams {
    pub fn builder() -> builder::ListFineTuningEventsParams {
        Default::default()
    }
}
#[doc = "`ListFineTuningJobCheckpointsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListFineTuningJobCheckpointsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
impl ListFineTuningJobCheckpointsParams {
    pub fn builder() -> builder::ListFineTuningJobCheckpointsParams {
        Default::default()
    }
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
impl ListFineTuningJobCheckpointsResponse {
    pub fn builder() -> builder::ListFineTuningJobCheckpointsResponse {
        Default::default()
    }
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
impl ListFineTuningJobEventsResponse {
    pub fn builder() -> builder::ListFineTuningJobEventsResponse {
        Default::default()
    }
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
impl ListPaginatedFineTuningJobsParams {
    pub fn builder() -> builder::ListPaginatedFineTuningJobsParams {
        Default::default()
    }
}
#[doc = "`ListPaginatedFineTuningJobsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListPaginatedFineTuningJobsResponse {
    pub data: ::std::vec::Vec<FineTuningJob>,
    pub has_more: bool,
    pub object: ListPaginatedFineTuningJobsResponseObject,
}
impl ListPaginatedFineTuningJobsResponse {
    pub fn builder() -> builder::ListPaginatedFineTuningJobsResponse {
        Default::default()
    }
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
impl PromptCacheBreakpointConfig {
    pub fn builder() -> builder::PromptCacheBreakpointConfig {
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateFineTuningCheckpointPermissionRequest {
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateFineTuningCheckpointPermissionRequest {
        fn default() -> Self {
            Self {
                project_ids: Err("no value supplied for project_ids".to_string()),
            }
        }
    }
    impl CreateFineTuningCheckpointPermissionRequest {
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFineTuningCheckpointPermissionRequest>
        for super::CreateFineTuningCheckpointPermissionRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFineTuningCheckpointPermissionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_ids: value.project_ids?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFineTuningCheckpointPermissionRequest>
        for CreateFineTuningCheckpointPermissionRequest
    {
        fn from(value: super::CreateFineTuningCheckpointPermissionRequest) -> Self {
            Self {
                project_ids: Ok(value.project_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateFineTuningJobRequest {
        hyperparameters: ::std::result::Result<
            ::std::option::Option<super::CreateFineTuningJobRequestHyperparameters>,
            ::std::string::String,
        >,
        integrations: ::std::result::Result<
            ::std::option::Option<
                ::std::vec::Vec<super::CreateFineTuningJobRequestIntegrationsItem>,
            >,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        method: ::std::result::Result<
            ::std::option::Option<super::FineTuneMethod>,
            ::std::string::String,
        >,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        suffix: ::std::result::Result<
            ::std::option::Option<super::CreateFineTuningJobRequestSuffix>,
            ::std::string::String,
        >,
        training_file: ::std::result::Result<::std::string::String, ::std::string::String>,
        validation_file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateFineTuningJobRequest {
        fn default() -> Self {
            Self {
                hyperparameters: Ok(Default::default()),
                integrations: Ok(Default::default()),
                metadata: Ok(Default::default()),
                method: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                seed: Ok(Default::default()),
                suffix: Ok(Default::default()),
                training_file: Err("no value supplied for training_file".to_string()),
                validation_file: Ok(Default::default()),
            }
        }
    }
    impl CreateFineTuningJobRequest {
        pub fn hyperparameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateFineTuningJobRequestHyperparameters>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hyperparameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hyperparameters: {e}"));
            self
        }
        pub fn integrations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::vec::Vec<super::CreateFineTuningJobRequestIntegrationsItem>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.integrations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for integrations: {e}"));
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
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneMethod>>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
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
        pub fn suffix<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateFineTuningJobRequestSuffix>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suffix: {e}"));
            self
        }
        pub fn training_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.training_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for training_file: {e}"));
            self
        }
        pub fn validation_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.validation_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for validation_file: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFineTuningJobRequest> for super::CreateFineTuningJobRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFineTuningJobRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                hyperparameters: value.hyperparameters?,
                integrations: value.integrations?,
                metadata: value.metadata?,
                method: value.method?,
                model: value.model?,
                seed: value.seed?,
                suffix: value.suffix?,
                training_file: value.training_file?,
                validation_file: value.validation_file?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFineTuningJobRequest> for CreateFineTuningJobRequest {
        fn from(value: super::CreateFineTuningJobRequest) -> Self {
            Self {
                hyperparameters: Ok(value.hyperparameters),
                integrations: Ok(value.integrations),
                metadata: Ok(value.metadata),
                method: Ok(value.method),
                model: Ok(value.model),
                seed: Ok(value.seed),
                suffix: Ok(value.suffix),
                training_file: Ok(value.training_file),
                validation_file: Ok(value.validation_file),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateFineTuningJobRequestHyperparameters {
        batch_size: ::std::result::Result<
            ::std::option::Option<super::CreateFineTuningJobRequestHyperparametersBatchSize>,
            ::std::string::String,
        >,
        learning_rate_multiplier: ::std::result::Result<
            ::std::option::Option<
                super::CreateFineTuningJobRequestHyperparametersLearningRateMultiplier,
            >,
            ::std::string::String,
        >,
        n_epochs: ::std::result::Result<
            ::std::option::Option<super::CreateFineTuningJobRequestHyperparametersNEpochs>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateFineTuningJobRequestHyperparameters {
        fn default() -> Self {
            Self {
                batch_size: Ok(Default::default()),
                learning_rate_multiplier: Ok(Default::default()),
                n_epochs: Ok(Default::default()),
            }
        }
    }
    impl CreateFineTuningJobRequestHyperparameters {
        pub fn batch_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateFineTuningJobRequestHyperparametersBatchSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size: {e}"));
            self
        }
        pub fn learning_rate_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::CreateFineTuningJobRequestHyperparametersLearningRateMultiplier,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.learning_rate_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for learning_rate_multiplier: {e}")
            });
            self
        }
        pub fn n_epochs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateFineTuningJobRequestHyperparametersNEpochs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.n_epochs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n_epochs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFineTuningJobRequestHyperparameters>
        for super::CreateFineTuningJobRequestHyperparameters
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFineTuningJobRequestHyperparameters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size: value.batch_size?,
                learning_rate_multiplier: value.learning_rate_multiplier?,
                n_epochs: value.n_epochs?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFineTuningJobRequestHyperparameters>
        for CreateFineTuningJobRequestHyperparameters
    {
        fn from(value: super::CreateFineTuningJobRequestHyperparameters) -> Self {
            Self {
                batch_size: Ok(value.batch_size),
                learning_rate_multiplier: Ok(value.learning_rate_multiplier),
                n_epochs: Ok(value.n_epochs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateFineTuningJobRequestIntegrationsItem {
        type_: ::std::result::Result<
            super::CreateFineTuningJobRequestIntegrationsItemType,
            ::std::string::String,
        >,
        wandb: ::std::result::Result<
            super::CreateFineTuningJobRequestIntegrationsItemWandb,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateFineTuningJobRequestIntegrationsItem {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                wandb: Err("no value supplied for wandb".to_string()),
            }
        }
    }
    impl CreateFineTuningJobRequestIntegrationsItem {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFineTuningJobRequestIntegrationsItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn wandb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFineTuningJobRequestIntegrationsItemWandb>,
            T::Error: ::std::fmt::Display,
        {
            self.wandb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wandb: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFineTuningJobRequestIntegrationsItem>
        for super::CreateFineTuningJobRequestIntegrationsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFineTuningJobRequestIntegrationsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                wandb: value.wandb?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFineTuningJobRequestIntegrationsItem>
        for CreateFineTuningJobRequestIntegrationsItem
    {
        fn from(value: super::CreateFineTuningJobRequestIntegrationsItem) -> Self {
            Self {
                type_: Ok(value.type_),
                wandb: Ok(value.wandb),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateFineTuningJobRequestIntegrationsItemWandb {
        entity: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project: ::std::result::Result<::std::string::String, ::std::string::String>,
        tags: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateFineTuningJobRequestIntegrationsItemWandb {
        fn default() -> Self {
            Self {
                entity: Ok(Default::default()),
                name: Ok(Default::default()),
                project: Err("no value supplied for project".to_string()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl CreateFineTuningJobRequestIntegrationsItemWandb {
        pub fn entity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.entity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity: {e}"));
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
        pub fn project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateFineTuningJobRequestIntegrationsItemWandb>
        for super::CreateFineTuningJobRequestIntegrationsItemWandb
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateFineTuningJobRequestIntegrationsItemWandb,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                entity: value.entity?,
                name: value.name?,
                project: value.project?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::CreateFineTuningJobRequestIntegrationsItemWandb>
        for CreateFineTuningJobRequestIntegrationsItemWandb
    {
        fn from(value: super::CreateFineTuningJobRequestIntegrationsItemWandb) -> Self {
            Self {
                entity: Ok(value.entity),
                name: Ok(value.name),
                project: Ok(value.project),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteFineTuningCheckpointPermissionResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::DeleteFineTuningCheckpointPermissionResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteFineTuningCheckpointPermissionResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteFineTuningCheckpointPermissionResponse {
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
            T: ::std::convert::TryInto<super::DeleteFineTuningCheckpointPermissionResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteFineTuningCheckpointPermissionResponse>
        for super::DeleteFineTuningCheckpointPermissionResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteFineTuningCheckpointPermissionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteFineTuningCheckpointPermissionResponse>
        for DeleteFineTuningCheckpointPermissionResponse
    {
        fn from(value: super::DeleteFineTuningCheckpointPermissionResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalItem {
        content: ::std::result::Result<super::EvalItemContent, ::std::string::String>,
        role: ::std::result::Result<super::EvalItemRole, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::EvalItemType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EvalItem {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EvalItem {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalItemContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EvalItemType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalItem> for super::EvalItem {
        type Error = super::error::ConversionError;
        fn try_from(value: EvalItem) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalItem> for EvalItem {
        fn from(value: super::EvalItem) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalItemContentOutputText {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::EvalItemContentOutputTextType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalItemContentOutputText {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalItemContentOutputText {
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
            T: ::std::convert::TryInto<super::EvalItemContentOutputTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalItemContentOutputText> for super::EvalItemContentOutputText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalItemContentOutputText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalItemContentOutputText> for EvalItemContentOutputText {
        fn from(value: super::EvalItemContentOutputText) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EvalItemInputImage {
        detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        image_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::EvalItemInputImageType, ::std::string::String>,
    }
    impl ::std::default::Default for EvalItemInputImage {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                image_url: Err("no value supplied for image_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl EvalItemInputImage {
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
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EvalItemInputImageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EvalItemInputImage> for super::EvalItemInputImage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EvalItemInputImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                image_url: value.image_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EvalItemInputImage> for EvalItemInputImage {
        fn from(value: super::EvalItemInputImage) -> Self {
            Self {
                detail: Ok(value.detail),
                image_url: Ok(value.image_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneDpoHyperparameters {
        batch_size: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoHyperparametersBatchSize>,
            ::std::string::String,
        >,
        beta: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoHyperparametersBeta>,
            ::std::string::String,
        >,
        learning_rate_multiplier: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoHyperparametersLearningRateMultiplier>,
            ::std::string::String,
        >,
        n_epochs: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoHyperparametersNEpochs>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneDpoHyperparameters {
        fn default() -> Self {
            Self {
                batch_size: Ok(Default::default()),
                beta: Ok(Default::default()),
                learning_rate_multiplier: Ok(Default::default()),
                n_epochs: Ok(Default::default()),
            }
        }
    }
    impl FineTuneDpoHyperparameters {
        pub fn batch_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneDpoHyperparametersBatchSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size: {e}"));
            self
        }
        pub fn beta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneDpoHyperparametersBeta>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.beta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beta: {e}"));
            self
        }
        pub fn learning_rate_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneDpoHyperparametersLearningRateMultiplier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.learning_rate_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for learning_rate_multiplier: {e}")
            });
            self
        }
        pub fn n_epochs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneDpoHyperparametersNEpochs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.n_epochs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n_epochs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneDpoHyperparameters> for super::FineTuneDpoHyperparameters {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneDpoHyperparameters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size: value.batch_size?,
                beta: value.beta?,
                learning_rate_multiplier: value.learning_rate_multiplier?,
                n_epochs: value.n_epochs?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneDpoHyperparameters> for FineTuneDpoHyperparameters {
        fn from(value: super::FineTuneDpoHyperparameters) -> Self {
            Self {
                batch_size: Ok(value.batch_size),
                beta: Ok(value.beta),
                learning_rate_multiplier: Ok(value.learning_rate_multiplier),
                n_epochs: Ok(value.n_epochs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneDpoMethod {
        hyperparameters: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoHyperparameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneDpoMethod {
        fn default() -> Self {
            Self {
                hyperparameters: Ok(Default::default()),
            }
        }
    }
    impl FineTuneDpoMethod {
        pub fn hyperparameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneDpoHyperparameters>>,
            T::Error: ::std::fmt::Display,
        {
            self.hyperparameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hyperparameters: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneDpoMethod> for super::FineTuneDpoMethod {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneDpoMethod,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                hyperparameters: value.hyperparameters?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneDpoMethod> for FineTuneDpoMethod {
        fn from(value: super::FineTuneDpoMethod) -> Self {
            Self {
                hyperparameters: Ok(value.hyperparameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneMethod {
        dpo: ::std::result::Result<
            ::std::option::Option<super::FineTuneDpoMethod>,
            ::std::string::String,
        >,
        reinforcement: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementMethod>,
            ::std::string::String,
        >,
        supervised: ::std::result::Result<
            ::std::option::Option<super::FineTuneSupervisedMethod>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FineTuneMethodType, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuneMethod {
        fn default() -> Self {
            Self {
                dpo: Ok(Default::default()),
                reinforcement: Ok(Default::default()),
                supervised: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FineTuneMethod {
        pub fn dpo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneDpoMethod>>,
            T::Error: ::std::fmt::Display,
        {
            self.dpo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dpo: {e}"));
            self
        }
        pub fn reinforcement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneReinforcementMethod>>,
            T::Error: ::std::fmt::Display,
        {
            self.reinforcement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reinforcement: {e}"));
            self
        }
        pub fn supervised<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneSupervisedMethod>>,
            T::Error: ::std::fmt::Display,
        {
            self.supervised = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supervised: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuneMethodType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneMethod> for super::FineTuneMethod {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneMethod,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dpo: value.dpo?,
                reinforcement: value.reinforcement?,
                supervised: value.supervised?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneMethod> for FineTuneMethod {
        fn from(value: super::FineTuneMethod) -> Self {
            Self {
                dpo: Ok(value.dpo),
                reinforcement: Ok(value.reinforcement),
                supervised: Ok(value.supervised),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneReinforcementHyperparameters {
        batch_size: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersBatchSize>,
            ::std::string::String,
        >,
        compute_multiplier: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersComputeMultiplier>,
            ::std::string::String,
        >,
        eval_interval: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersEvalInterval>,
            ::std::string::String,
        >,
        eval_samples: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersEvalSamples>,
            ::std::string::String,
        >,
        learning_rate_multiplier: ::std::result::Result<
            ::std::option::Option<
                super::FineTuneReinforcementHyperparametersLearningRateMultiplier,
            >,
            ::std::string::String,
        >,
        n_epochs: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersNEpochs>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparametersReasoningEffort>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneReinforcementHyperparameters {
        fn default() -> Self {
            Self {
                batch_size: Ok(Default::default()),
                compute_multiplier: Ok(Default::default()),
                eval_interval: Ok(Default::default()),
                eval_samples: Ok(Default::default()),
                learning_rate_multiplier: Ok(Default::default()),
                n_epochs: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
            }
        }
    }
    impl FineTuneReinforcementHyperparameters {
        pub fn batch_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersBatchSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size: {e}"));
            self
        }
        pub fn compute_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersComputeMultiplier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.compute_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for compute_multiplier: {e}")
            });
            self
        }
        pub fn eval_interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersEvalInterval>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.eval_interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_interval: {e}"));
            self
        }
        pub fn eval_samples<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersEvalSamples>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.eval_samples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eval_samples: {e}"));
            self
        }
        pub fn learning_rate_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FineTuneReinforcementHyperparametersLearningRateMultiplier,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.learning_rate_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for learning_rate_multiplier: {e}")
            });
            self
        }
        pub fn n_epochs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersNEpochs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.n_epochs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n_epochs: {e}"));
            self
        }
        pub fn reasoning_effort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparametersReasoningEffort>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_effort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_effort: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneReinforcementHyperparameters>
        for super::FineTuneReinforcementHyperparameters
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneReinforcementHyperparameters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size: value.batch_size?,
                compute_multiplier: value.compute_multiplier?,
                eval_interval: value.eval_interval?,
                eval_samples: value.eval_samples?,
                learning_rate_multiplier: value.learning_rate_multiplier?,
                n_epochs: value.n_epochs?,
                reasoning_effort: value.reasoning_effort?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneReinforcementHyperparameters>
        for FineTuneReinforcementHyperparameters
    {
        fn from(value: super::FineTuneReinforcementHyperparameters) -> Self {
            Self {
                batch_size: Ok(value.batch_size),
                compute_multiplier: Ok(value.compute_multiplier),
                eval_interval: Ok(value.eval_interval),
                eval_samples: Ok(value.eval_samples),
                learning_rate_multiplier: Ok(value.learning_rate_multiplier),
                n_epochs: Ok(value.n_epochs),
                reasoning_effort: Ok(value.reasoning_effort),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneReinforcementMethod {
        grader:
            ::std::result::Result<super::FineTuneReinforcementMethodGrader, ::std::string::String>,
        hyperparameters: ::std::result::Result<
            ::std::option::Option<super::FineTuneReinforcementHyperparameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneReinforcementMethod {
        fn default() -> Self {
            Self {
                grader: Err("no value supplied for grader".to_string()),
                hyperparameters: Ok(Default::default()),
            }
        }
    }
    impl FineTuneReinforcementMethod {
        pub fn grader<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuneReinforcementMethodGrader>,
            T::Error: ::std::fmt::Display,
        {
            self.grader = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grader: {e}"));
            self
        }
        pub fn hyperparameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneReinforcementHyperparameters>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hyperparameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hyperparameters: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneReinforcementMethod> for super::FineTuneReinforcementMethod {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneReinforcementMethod,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                grader: value.grader?,
                hyperparameters: value.hyperparameters?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneReinforcementMethod> for FineTuneReinforcementMethod {
        fn from(value: super::FineTuneReinforcementMethod) -> Self {
            Self {
                grader: Ok(value.grader),
                hyperparameters: Ok(value.hyperparameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneSupervisedHyperparameters {
        batch_size: ::std::result::Result<
            ::std::option::Option<super::FineTuneSupervisedHyperparametersBatchSize>,
            ::std::string::String,
        >,
        learning_rate_multiplier: ::std::result::Result<
            ::std::option::Option<super::FineTuneSupervisedHyperparametersLearningRateMultiplier>,
            ::std::string::String,
        >,
        n_epochs: ::std::result::Result<
            ::std::option::Option<super::FineTuneSupervisedHyperparametersNEpochs>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneSupervisedHyperparameters {
        fn default() -> Self {
            Self {
                batch_size: Ok(Default::default()),
                learning_rate_multiplier: Ok(Default::default()),
                n_epochs: Ok(Default::default()),
            }
        }
    }
    impl FineTuneSupervisedHyperparameters {
        pub fn batch_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneSupervisedHyperparametersBatchSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size: {e}"));
            self
        }
        pub fn learning_rate_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FineTuneSupervisedHyperparametersLearningRateMultiplier,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.learning_rate_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for learning_rate_multiplier: {e}")
            });
            self
        }
        pub fn n_epochs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneSupervisedHyperparametersNEpochs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.n_epochs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n_epochs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneSupervisedHyperparameters>
        for super::FineTuneSupervisedHyperparameters
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneSupervisedHyperparameters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size: value.batch_size?,
                learning_rate_multiplier: value.learning_rate_multiplier?,
                n_epochs: value.n_epochs?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneSupervisedHyperparameters>
        for FineTuneSupervisedHyperparameters
    {
        fn from(value: super::FineTuneSupervisedHyperparameters) -> Self {
            Self {
                batch_size: Ok(value.batch_size),
                learning_rate_multiplier: Ok(value.learning_rate_multiplier),
                n_epochs: Ok(value.n_epochs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuneSupervisedMethod {
        hyperparameters: ::std::result::Result<
            ::std::option::Option<super::FineTuneSupervisedHyperparameters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuneSupervisedMethod {
        fn default() -> Self {
            Self {
                hyperparameters: Ok(Default::default()),
            }
        }
    }
    impl FineTuneSupervisedMethod {
        pub fn hyperparameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuneSupervisedHyperparameters>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hyperparameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hyperparameters: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuneSupervisedMethod> for super::FineTuneSupervisedMethod {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuneSupervisedMethod,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                hyperparameters: value.hyperparameters?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuneSupervisedMethod> for FineTuneSupervisedMethod {
        fn from(value: super::FineTuneSupervisedMethod) -> Self {
            Self {
                hyperparameters: Ok(value.hyperparameters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningApiSchema {
        cancel_fine_tuning_job_response:
            ::std::result::Result<super::FineTuningJob, ::std::string::String>,
        create_fine_tuning_checkpoint_permission_request: ::std::result::Result<
            super::CreateFineTuningCheckpointPermissionRequest,
            ::std::string::String,
        >,
        create_fine_tuning_checkpoint_permission_response: ::std::result::Result<
            super::ListFineTuningCheckpointPermissionResponse,
            ::std::string::String,
        >,
        create_fine_tuning_job_request:
            ::std::result::Result<super::CreateFineTuningJobRequest, ::std::string::String>,
        create_fine_tuning_job_response:
            ::std::result::Result<super::FineTuningJob, ::std::string::String>,
        delete_fine_tuning_checkpoint_permission_response: ::std::result::Result<
            super::DeleteFineTuningCheckpointPermissionResponse,
            ::std::string::String,
        >,
        list_fine_tuning_checkpoint_permissions_params: ::std::result::Result<
            super::ListFineTuningCheckpointPermissionsParams,
            ::std::string::String,
        >,
        list_fine_tuning_checkpoint_permissions_response: ::std::result::Result<
            super::ListFineTuningCheckpointPermissionResponse,
            ::std::string::String,
        >,
        list_fine_tuning_events_params:
            ::std::result::Result<super::ListFineTuningEventsParams, ::std::string::String>,
        list_fine_tuning_events_response:
            ::std::result::Result<super::ListFineTuningJobEventsResponse, ::std::string::String>,
        list_fine_tuning_job_checkpoints_params:
            ::std::result::Result<super::ListFineTuningJobCheckpointsParams, ::std::string::String>,
        list_fine_tuning_job_checkpoints_response: ::std::result::Result<
            super::ListFineTuningJobCheckpointsResponse,
            ::std::string::String,
        >,
        list_paginated_fine_tuning_jobs_params:
            ::std::result::Result<super::ListPaginatedFineTuningJobsParams, ::std::string::String>,
        list_paginated_fine_tuning_jobs_response: ::std::result::Result<
            super::ListPaginatedFineTuningJobsResponse,
            ::std::string::String,
        >,
        pause_fine_tuning_job_response:
            ::std::result::Result<super::FineTuningJob, ::std::string::String>,
        resume_fine_tuning_job_response:
            ::std::result::Result<super::FineTuningJob, ::std::string::String>,
        retrieve_fine_tuning_job_response:
            ::std::result::Result<super::FineTuningJob, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningApiSchema {
        fn default() -> Self {
            Self {
                cancel_fine_tuning_job_response: Err(
                    "no value supplied for cancel_fine_tuning_job_response".to_string(),
                ),
                create_fine_tuning_checkpoint_permission_request: Err(
                    "no value supplied for create_fine_tuning_checkpoint_permission_request"
                        .to_string(),
                ),
                create_fine_tuning_checkpoint_permission_response: Err(
                    "no value supplied for create_fine_tuning_checkpoint_permission_response"
                        .to_string(),
                ),
                create_fine_tuning_job_request: Err(
                    "no value supplied for create_fine_tuning_job_request".to_string(),
                ),
                create_fine_tuning_job_response: Err(
                    "no value supplied for create_fine_tuning_job_response".to_string(),
                ),
                delete_fine_tuning_checkpoint_permission_response: Err(
                    "no value supplied for delete_fine_tuning_checkpoint_permission_response"
                        .to_string(),
                ),
                list_fine_tuning_checkpoint_permissions_params: Err(
                    "no value supplied for list_fine_tuning_checkpoint_permissions_params"
                        .to_string(),
                ),
                list_fine_tuning_checkpoint_permissions_response: Err(
                    "no value supplied for list_fine_tuning_checkpoint_permissions_response"
                        .to_string(),
                ),
                list_fine_tuning_events_params: Err(
                    "no value supplied for list_fine_tuning_events_params".to_string(),
                ),
                list_fine_tuning_events_response: Err(
                    "no value supplied for list_fine_tuning_events_response".to_string(),
                ),
                list_fine_tuning_job_checkpoints_params: Err(
                    "no value supplied for list_fine_tuning_job_checkpoints_params".to_string(),
                ),
                list_fine_tuning_job_checkpoints_response: Err(
                    "no value supplied for list_fine_tuning_job_checkpoints_response".to_string(),
                ),
                list_paginated_fine_tuning_jobs_params: Err(
                    "no value supplied for list_paginated_fine_tuning_jobs_params".to_string(),
                ),
                list_paginated_fine_tuning_jobs_response: Err(
                    "no value supplied for list_paginated_fine_tuning_jobs_response".to_string(),
                ),
                pause_fine_tuning_job_response: Err(
                    "no value supplied for pause_fine_tuning_job_response".to_string(),
                ),
                resume_fine_tuning_job_response: Err(
                    "no value supplied for resume_fine_tuning_job_response".to_string(),
                ),
                retrieve_fine_tuning_job_response: Err(
                    "no value supplied for retrieve_fine_tuning_job_response".to_string(),
                ),
            }
        }
    }
    impl FineTuningApiSchema {
        pub fn cancel_fine_tuning_job_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJob>,
            T::Error: ::std::fmt::Display,
        {
            self.cancel_fine_tuning_job_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for cancel_fine_tuning_job_response: {e}")
            });
            self
        }
        pub fn create_fine_tuning_checkpoint_permission_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFineTuningCheckpointPermissionRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . create_fine_tuning_checkpoint_permission_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_fine_tuning_checkpoint_permission_request: {e}")) ;
            self
        }
        pub fn create_fine_tuning_checkpoint_permission_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningCheckpointPermissionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_fine_tuning_checkpoint_permission_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_fine_tuning_checkpoint_permission_response: {e}")) ;
            self
        }
        pub fn create_fine_tuning_job_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateFineTuningJobRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_fine_tuning_job_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_fine_tuning_job_request: {e}")
            });
            self
        }
        pub fn create_fine_tuning_job_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJob>,
            T::Error: ::std::fmt::Display,
        {
            self.create_fine_tuning_job_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_fine_tuning_job_response: {e}")
            });
            self
        }
        pub fn delete_fine_tuning_checkpoint_permission_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteFineTuningCheckpointPermissionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_fine_tuning_checkpoint_permission_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_fine_tuning_checkpoint_permission_response: {e}")) ;
            self
        }
        pub fn list_fine_tuning_checkpoint_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningCheckpointPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_fine_tuning_checkpoint_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_fine_tuning_checkpoint_permissions_params: {e}")) ;
            self
        }
        pub fn list_fine_tuning_checkpoint_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningCheckpointPermissionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_fine_tuning_checkpoint_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_fine_tuning_checkpoint_permissions_response: {e}")) ;
            self
        }
        pub fn list_fine_tuning_events_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningEventsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_fine_tuning_events_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_fine_tuning_events_params: {e}")
            });
            self
        }
        pub fn list_fine_tuning_events_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningJobEventsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_fine_tuning_events_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_fine_tuning_events_response: {e}")
            });
            self
        }
        pub fn list_fine_tuning_job_checkpoints_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningJobCheckpointsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_fine_tuning_job_checkpoints_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_fine_tuning_job_checkpoints_params: {e}")) ;
            self
        }
        pub fn list_fine_tuning_job_checkpoints_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningJobCheckpointsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_fine_tuning_job_checkpoints_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_fine_tuning_job_checkpoints_response: {e}")) ;
            self
        }
        pub fn list_paginated_fine_tuning_jobs_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListPaginatedFineTuningJobsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_paginated_fine_tuning_jobs_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_paginated_fine_tuning_jobs_params: {e}")) ;
            self
        }
        pub fn list_paginated_fine_tuning_jobs_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListPaginatedFineTuningJobsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_paginated_fine_tuning_jobs_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_paginated_fine_tuning_jobs_response: {e}")) ;
            self
        }
        pub fn pause_fine_tuning_job_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJob>,
            T::Error: ::std::fmt::Display,
        {
            self.pause_fine_tuning_job_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for pause_fine_tuning_job_response: {e}")
            });
            self
        }
        pub fn resume_fine_tuning_job_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJob>,
            T::Error: ::std::fmt::Display,
        {
            self.resume_fine_tuning_job_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for resume_fine_tuning_job_response: {e}")
            });
            self
        }
        pub fn retrieve_fine_tuning_job_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJob>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_fine_tuning_job_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_fine_tuning_job_response: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningApiSchema> for super::FineTuningApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cancel_fine_tuning_job_response: value.cancel_fine_tuning_job_response?,
                create_fine_tuning_checkpoint_permission_request: value
                    .create_fine_tuning_checkpoint_permission_request?,
                create_fine_tuning_checkpoint_permission_response: value
                    .create_fine_tuning_checkpoint_permission_response?,
                create_fine_tuning_job_request: value.create_fine_tuning_job_request?,
                create_fine_tuning_job_response: value.create_fine_tuning_job_response?,
                delete_fine_tuning_checkpoint_permission_response: value
                    .delete_fine_tuning_checkpoint_permission_response?,
                list_fine_tuning_checkpoint_permissions_params: value
                    .list_fine_tuning_checkpoint_permissions_params?,
                list_fine_tuning_checkpoint_permissions_response: value
                    .list_fine_tuning_checkpoint_permissions_response?,
                list_fine_tuning_events_params: value.list_fine_tuning_events_params?,
                list_fine_tuning_events_response: value.list_fine_tuning_events_response?,
                list_fine_tuning_job_checkpoints_params: value
                    .list_fine_tuning_job_checkpoints_params?,
                list_fine_tuning_job_checkpoints_response: value
                    .list_fine_tuning_job_checkpoints_response?,
                list_paginated_fine_tuning_jobs_params: value
                    .list_paginated_fine_tuning_jobs_params?,
                list_paginated_fine_tuning_jobs_response: value
                    .list_paginated_fine_tuning_jobs_response?,
                pause_fine_tuning_job_response: value.pause_fine_tuning_job_response?,
                resume_fine_tuning_job_response: value.resume_fine_tuning_job_response?,
                retrieve_fine_tuning_job_response: value.retrieve_fine_tuning_job_response?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningApiSchema> for FineTuningApiSchema {
        fn from(value: super::FineTuningApiSchema) -> Self {
            Self {
                cancel_fine_tuning_job_response: Ok(value.cancel_fine_tuning_job_response),
                create_fine_tuning_checkpoint_permission_request: Ok(
                    value.create_fine_tuning_checkpoint_permission_request
                ),
                create_fine_tuning_checkpoint_permission_response: Ok(
                    value.create_fine_tuning_checkpoint_permission_response
                ),
                create_fine_tuning_job_request: Ok(value.create_fine_tuning_job_request),
                create_fine_tuning_job_response: Ok(value.create_fine_tuning_job_response),
                delete_fine_tuning_checkpoint_permission_response: Ok(
                    value.delete_fine_tuning_checkpoint_permission_response
                ),
                list_fine_tuning_checkpoint_permissions_params: Ok(
                    value.list_fine_tuning_checkpoint_permissions_params
                ),
                list_fine_tuning_checkpoint_permissions_response: Ok(
                    value.list_fine_tuning_checkpoint_permissions_response
                ),
                list_fine_tuning_events_params: Ok(value.list_fine_tuning_events_params),
                list_fine_tuning_events_response: Ok(value.list_fine_tuning_events_response),
                list_fine_tuning_job_checkpoints_params: Ok(
                    value.list_fine_tuning_job_checkpoints_params
                ),
                list_fine_tuning_job_checkpoints_response: Ok(
                    value.list_fine_tuning_job_checkpoints_response
                ),
                list_paginated_fine_tuning_jobs_params: Ok(
                    value.list_paginated_fine_tuning_jobs_params
                ),
                list_paginated_fine_tuning_jobs_response: Ok(
                    value.list_paginated_fine_tuning_jobs_response
                ),
                pause_fine_tuning_job_response: Ok(value.pause_fine_tuning_job_response),
                resume_fine_tuning_job_response: Ok(value.resume_fine_tuning_job_response),
                retrieve_fine_tuning_job_response: Ok(value.retrieve_fine_tuning_job_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningCheckpointPermission {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::FineTuningCheckpointPermissionObject,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningCheckpointPermission {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl FineTuningCheckpointPermission {
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
            T: ::std::convert::TryInto<super::FineTuningCheckpointPermissionObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningCheckpointPermission>
        for super::FineTuningCheckpointPermission
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningCheckpointPermission,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningCheckpointPermission>
        for FineTuningCheckpointPermission
    {
        fn from(value: super::FineTuningCheckpointPermission) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningIntegration {
        type_: ::std::result::Result<super::FineTuningIntegrationType, ::std::string::String>,
        wandb: ::std::result::Result<super::FineTuningIntegrationWandb, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningIntegration {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                wandb: Err("no value supplied for wandb".to_string()),
            }
        }
    }
    impl FineTuningIntegration {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningIntegrationType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn wandb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningIntegrationWandb>,
            T::Error: ::std::fmt::Display,
        {
            self.wandb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wandb: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningIntegration> for super::FineTuningIntegration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningIntegration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                wandb: value.wandb?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningIntegration> for FineTuningIntegration {
        fn from(value: super::FineTuningIntegration) -> Self {
            Self {
                type_: Ok(value.type_),
                wandb: Ok(value.wandb),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningIntegrationWandb {
        entity: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project: ::std::result::Result<::std::string::String, ::std::string::String>,
        tags: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningIntegrationWandb {
        fn default() -> Self {
            Self {
                entity: Ok(Default::default()),
                name: Ok(Default::default()),
                project: Err("no value supplied for project".to_string()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl FineTuningIntegrationWandb {
        pub fn entity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.entity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity: {e}"));
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
        pub fn project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.project = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningIntegrationWandb> for super::FineTuningIntegrationWandb {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningIntegrationWandb,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                entity: value.entity?,
                name: value.name?,
                project: value.project?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningIntegrationWandb> for FineTuningIntegrationWandb {
        fn from(value: super::FineTuningIntegrationWandb) -> Self {
            Self {
                entity: Ok(value.entity),
                name: Ok(value.name),
                project: Ok(value.project),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJob {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        error: ::std::result::Result<super::FineTuningJobError, ::std::string::String>,
        estimated_finish: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        fine_tuned_model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        finished_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        hyperparameters: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobHyperparameters>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        integrations: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::FineTuningIntegration>>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::std::option::Option<super::Metadata>, ::std::string::String>,
        method: ::std::result::Result<
            ::std::option::Option<super::FineTuneMethod>,
            ::std::string::String,
        >,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::FineTuningJobObject, ::std::string::String>,
        organization_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        result_files:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        status: ::std::result::Result<super::FineTuningJobStatus, ::std::string::String>,
        trained_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        training_file: ::std::result::Result<::std::string::String, ::std::string::String>,
        validation_file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuningJob {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                error: Err("no value supplied for error".to_string()),
                estimated_finish: Ok(Default::default()),
                fine_tuned_model: Err("no value supplied for fine_tuned_model".to_string()),
                finished_at: Err("no value supplied for finished_at".to_string()),
                hyperparameters: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                integrations: Ok(Default::default()),
                metadata: Ok(Default::default()),
                method: Ok(Default::default()),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
                organization_id: Err("no value supplied for organization_id".to_string()),
                result_files: Err("no value supplied for result_files".to_string()),
                seed: Err("no value supplied for seed".to_string()),
                status: Err("no value supplied for status".to_string()),
                trained_tokens: Err("no value supplied for trained_tokens".to_string()),
                training_file: Err("no value supplied for training_file".to_string()),
                validation_file: Err("no value supplied for validation_file".to_string()),
            }
        }
    }
    impl FineTuningJob {
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
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn estimated_finish<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.estimated_finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for estimated_finish: {e}"));
            self
        }
        pub fn fine_tuned_model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fine_tuned_model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fine_tuned_model: {e}"));
            self
        }
        pub fn finished_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.finished_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finished_at: {e}"));
            self
        }
        pub fn hyperparameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuningJobHyperparameters>>,
            T::Error: ::std::fmt::Display,
        {
            self.hyperparameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hyperparameters: {e}"));
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
        pub fn integrations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::FineTuningIntegration>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.integrations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for integrations: {e}"));
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
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuneMethod>>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {e}"));
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
            T: ::std::convert::TryInto<super::FineTuningJobObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn organization_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organization_id: {e}"));
            self
        }
        pub fn result_files<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.result_files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result_files: {e}"));
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
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn trained_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.trained_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trained_tokens: {e}"));
            self
        }
        pub fn training_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.training_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for training_file: {e}"));
            self
        }
        pub fn validation_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.validation_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for validation_file: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJob> for super::FineTuningJob {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJob,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                error: value.error?,
                estimated_finish: value.estimated_finish?,
                fine_tuned_model: value.fine_tuned_model?,
                finished_at: value.finished_at?,
                hyperparameters: value.hyperparameters?,
                id: value.id?,
                integrations: value.integrations?,
                metadata: value.metadata?,
                method: value.method?,
                model: value.model?,
                object: value.object?,
                organization_id: value.organization_id?,
                result_files: value.result_files?,
                seed: value.seed?,
                status: value.status?,
                trained_tokens: value.trained_tokens?,
                training_file: value.training_file?,
                validation_file: value.validation_file?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJob> for FineTuningJob {
        fn from(value: super::FineTuningJob) -> Self {
            Self {
                created_at: Ok(value.created_at),
                error: Ok(value.error),
                estimated_finish: Ok(value.estimated_finish),
                fine_tuned_model: Ok(value.fine_tuned_model),
                finished_at: Ok(value.finished_at),
                hyperparameters: Ok(value.hyperparameters),
                id: Ok(value.id),
                integrations: Ok(value.integrations),
                metadata: Ok(value.metadata),
                method: Ok(value.method),
                model: Ok(value.model),
                object: Ok(value.object),
                organization_id: Ok(value.organization_id),
                result_files: Ok(value.result_files),
                seed: Ok(value.seed),
                status: Ok(value.status),
                trained_tokens: Ok(value.trained_tokens),
                training_file: Ok(value.training_file),
                validation_file: Ok(value.validation_file),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobCheckpoint {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        fine_tuned_model_checkpoint:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        fine_tuning_job_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metrics:
            ::std::result::Result<super::FineTuningJobCheckpointMetrics, ::std::string::String>,
        object: ::std::result::Result<super::FineTuningJobCheckpointObject, ::std::string::String>,
        step_number: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningJobCheckpoint {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                fine_tuned_model_checkpoint: Err(
                    "no value supplied for fine_tuned_model_checkpoint".to_string(),
                ),
                fine_tuning_job_id: Err("no value supplied for fine_tuning_job_id".to_string()),
                id: Err("no value supplied for id".to_string()),
                metrics: Err("no value supplied for metrics".to_string()),
                object: Err("no value supplied for object".to_string()),
                step_number: Err("no value supplied for step_number".to_string()),
            }
        }
    }
    impl FineTuningJobCheckpoint {
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
        pub fn fine_tuned_model_checkpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.fine_tuned_model_checkpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for fine_tuned_model_checkpoint: {e}")
            });
            self
        }
        pub fn fine_tuning_job_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.fine_tuning_job_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for fine_tuning_job_id: {e}")
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
        pub fn metrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobCheckpointMetrics>,
            T::Error: ::std::fmt::Display,
        {
            self.metrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metrics: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobCheckpointObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn step_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.step_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step_number: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJobCheckpoint> for super::FineTuningJobCheckpoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobCheckpoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                fine_tuned_model_checkpoint: value.fine_tuned_model_checkpoint?,
                fine_tuning_job_id: value.fine_tuning_job_id?,
                id: value.id?,
                metrics: value.metrics?,
                object: value.object?,
                step_number: value.step_number?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobCheckpoint> for FineTuningJobCheckpoint {
        fn from(value: super::FineTuningJobCheckpoint) -> Self {
            Self {
                created_at: Ok(value.created_at),
                fine_tuned_model_checkpoint: Ok(value.fine_tuned_model_checkpoint),
                fine_tuning_job_id: Ok(value.fine_tuning_job_id),
                id: Ok(value.id),
                metrics: Ok(value.metrics),
                object: Ok(value.object),
                step_number: Ok(value.step_number),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobCheckpointMetrics {
        full_valid_loss: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        full_valid_mean_token_accuracy:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        step: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        train_loss: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        train_mean_token_accuracy:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        valid_loss: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        valid_mean_token_accuracy:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FineTuningJobCheckpointMetrics {
        fn default() -> Self {
            Self {
                full_valid_loss: Ok(Default::default()),
                full_valid_mean_token_accuracy: Ok(Default::default()),
                step: Ok(Default::default()),
                train_loss: Ok(Default::default()),
                train_mean_token_accuracy: Ok(Default::default()),
                valid_loss: Ok(Default::default()),
                valid_mean_token_accuracy: Ok(Default::default()),
            }
        }
    }
    impl FineTuningJobCheckpointMetrics {
        pub fn full_valid_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_valid_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_valid_loss: {e}"));
            self
        }
        pub fn full_valid_mean_token_accuracy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_valid_mean_token_accuracy = value.try_into().map_err(|e| {
                format!("error converting supplied value for full_valid_mean_token_accuracy: {e}")
            });
            self
        }
        pub fn step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step: {e}"));
            self
        }
        pub fn train_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.train_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for train_loss: {e}"));
            self
        }
        pub fn train_mean_token_accuracy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.train_mean_token_accuracy = value.try_into().map_err(|e| {
                format!("error converting supplied value for train_mean_token_accuracy: {e}")
            });
            self
        }
        pub fn valid_loss<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_loss = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_loss: {e}"));
            self
        }
        pub fn valid_mean_token_accuracy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_mean_token_accuracy = value.try_into().map_err(|e| {
                format!("error converting supplied value for valid_mean_token_accuracy: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJobCheckpointMetrics>
        for super::FineTuningJobCheckpointMetrics
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobCheckpointMetrics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                full_valid_loss: value.full_valid_loss?,
                full_valid_mean_token_accuracy: value.full_valid_mean_token_accuracy?,
                step: value.step?,
                train_loss: value.train_loss?,
                train_mean_token_accuracy: value.train_mean_token_accuracy?,
                valid_loss: value.valid_loss?,
                valid_mean_token_accuracy: value.valid_mean_token_accuracy?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobCheckpointMetrics>
        for FineTuningJobCheckpointMetrics
    {
        fn from(value: super::FineTuningJobCheckpointMetrics) -> Self {
            Self {
                full_valid_loss: Ok(value.full_valid_loss),
                full_valid_mean_token_accuracy: Ok(value.full_valid_mean_token_accuracy),
                step: Ok(value.step),
                train_loss: Ok(value.train_loss),
                train_mean_token_accuracy: Ok(value.train_mean_token_accuracy),
                valid_loss: Ok(value.valid_loss),
                valid_mean_token_accuracy: Ok(value.valid_mean_token_accuracy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobError {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobErrorSubtype0>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<::std::option::Option<()>, ::std::string::String>,
        subtype_2: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuningJobError {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl FineTuningJobError {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuningJobErrorSubtype0>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {e}"));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<()>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {e}"));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJobError> for super::FineTuningJobError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobError> for FineTuningJobError {
        fn from(value: super::FineTuningJobError) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobErrorSubtype0 {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        param: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuningJobErrorSubtype0 {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
                param: Err("no value supplied for param".to_string()),
            }
        }
    }
    impl FineTuningJobErrorSubtype0 {
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
    }
    impl ::std::convert::TryFrom<FineTuningJobErrorSubtype0> for super::FineTuningJobErrorSubtype0 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobErrorSubtype0,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
                param: value.param?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobErrorSubtype0> for FineTuningJobErrorSubtype0 {
        fn from(value: super::FineTuningJobErrorSubtype0) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
                param: Ok(value.param),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobEvent {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        level: ::std::result::Result<super::FineTuningJobEventLevel, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::FineTuningJobEventObject, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobEventType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuningJobEvent {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                data: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                level: Err("no value supplied for level".to_string()),
                message: Err("no value supplied for message".to_string()),
                object: Err("no value supplied for object".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl FineTuningJobEvent {
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
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobEventLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FineTuningJobEventObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FineTuningJobEventType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJobEvent> for super::FineTuningJobEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                data: value.data?,
                id: value.id?,
                level: value.level?,
                message: value.message?,
                object: value.object?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobEvent> for FineTuningJobEvent {
        fn from(value: super::FineTuningJobEvent) -> Self {
            Self {
                created_at: Ok(value.created_at),
                data: Ok(value.data),
                id: Ok(value.id),
                level: Ok(value.level),
                message: Ok(value.message),
                object: Ok(value.object),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FineTuningJobHyperparameters {
        batch_size: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobHyperparametersBatchSize>,
            ::std::string::String,
        >,
        learning_rate_multiplier: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobHyperparametersLearningRateMultiplier>,
            ::std::string::String,
        >,
        n_epochs: ::std::result::Result<
            ::std::option::Option<super::FineTuningJobHyperparametersNEpochs>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FineTuningJobHyperparameters {
        fn default() -> Self {
            Self {
                batch_size: Ok(Default::default()),
                learning_rate_multiplier: Ok(Default::default()),
                n_epochs: Ok(Default::default()),
            }
        }
    }
    impl FineTuningJobHyperparameters {
        pub fn batch_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuningJobHyperparametersBatchSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.batch_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch_size: {e}"));
            self
        }
        pub fn learning_rate_multiplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuningJobHyperparametersLearningRateMultiplier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.learning_rate_multiplier = value.try_into().map_err(|e| {
                format!("error converting supplied value for learning_rate_multiplier: {e}")
            });
            self
        }
        pub fn n_epochs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FineTuningJobHyperparametersNEpochs>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.n_epochs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n_epochs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FineTuningJobHyperparameters> for super::FineTuningJobHyperparameters {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FineTuningJobHyperparameters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_size: value.batch_size?,
                learning_rate_multiplier: value.learning_rate_multiplier?,
                n_epochs: value.n_epochs?,
            })
        }
    }
    impl ::std::convert::From<super::FineTuningJobHyperparameters> for FineTuningJobHyperparameters {
        fn from(value: super::FineTuningJobHyperparameters) -> Self {
            Self {
                batch_size: Ok(value.batch_size),
                learning_rate_multiplier: Ok(value.learning_rate_multiplier),
                n_epochs: Ok(value.n_epochs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderLabelModel {
        input: ::std::result::Result<::std::vec::Vec<super::EvalItem>, ::std::string::String>,
        labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        passing_labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<super::GraderLabelModelType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderLabelModel {
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
    impl GraderLabelModel {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalItem>>,
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
            T: ::std::convert::TryInto<super::GraderLabelModelType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderLabelModel> for super::GraderLabelModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderLabelModel,
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
    impl ::std::convert::From<super::GraderLabelModel> for GraderLabelModel {
        fn from(value: super::GraderLabelModel) -> Self {
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
    pub struct GraderMulti {
        calculate_output: ::std::result::Result<::std::string::String, ::std::string::String>,
        graders: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::GraderMultiGradersValue>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::GraderMultiType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderMulti {
        fn default() -> Self {
            Self {
                calculate_output: Err("no value supplied for calculate_output".to_string()),
                graders: Err("no value supplied for graders".to_string()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GraderMulti {
        pub fn calculate_output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.calculate_output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for calculate_output: {e}"));
            self
        }
        pub fn graders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::GraderMultiGradersValue>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.graders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for graders: {e}"));
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderMultiType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderMulti> for super::GraderMulti {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderMulti,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                calculate_output: value.calculate_output?,
                graders: value.graders?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GraderMulti> for GraderMulti {
        fn from(value: super::GraderMulti) -> Self {
            Self {
                calculate_output: Ok(value.calculate_output),
                graders: Ok(value.graders),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderPython {
        image_tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::GraderPythonType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderPython {
        fn default() -> Self {
            Self {
                image_tag: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GraderPython {
        pub fn image_tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.image_tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_tag: {e}"));
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderPythonType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderPython> for super::GraderPython {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderPython,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_tag: value.image_tag?,
                name: value.name?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GraderPython> for GraderPython {
        fn from(value: super::GraderPython) -> Self {
            Self {
                image_tag: Ok(value.image_tag),
                name: Ok(value.name),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderScoreModel {
        input: ::std::result::Result<::std::vec::Vec<super::EvalItem>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        range: ::std::result::Result<::std::vec::Vec<f64>, ::std::string::String>,
        sampling_params: ::std::result::Result<
            ::std::option::Option<super::GraderScoreModelSamplingParams>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::GraderScoreModelType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderScoreModel {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                model: Err("no value supplied for model".to_string()),
                name: Err("no value supplied for name".to_string()),
                range: Ok(Default::default()),
                sampling_params: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GraderScoreModel {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EvalItem>>,
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
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range: {e}"));
            self
        }
        pub fn sampling_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::GraderScoreModelSamplingParams>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sampling_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sampling_params: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderScoreModelType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderScoreModel> for super::GraderScoreModel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderScoreModel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                model: value.model?,
                name: value.name?,
                range: value.range?,
                sampling_params: value.sampling_params?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GraderScoreModel> for GraderScoreModel {
        fn from(value: super::GraderScoreModel) -> Self {
            Self {
                input: Ok(value.input),
                model: Ok(value.model),
                name: Ok(value.name),
                range: Ok(value.range),
                sampling_params: Ok(value.sampling_params),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderScoreModelSamplingParams {
        max_completions_tokens: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        reasoning_effort: ::std::result::Result<
            ::std::option::Option<super::ReasoningEffort>,
            ::std::string::String,
        >,
        seed: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        temperature: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        top_p: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for GraderScoreModelSamplingParams {
        fn default() -> Self {
            Self {
                max_completions_tokens: Ok(Default::default()),
                reasoning_effort: Ok(Default::default()),
                seed: Ok(Default::default()),
                temperature: Ok(Default::default()),
                top_p: Ok(Default::default()),
            }
        }
    }
    impl GraderScoreModelSamplingParams {
        pub fn max_completions_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_completions_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_completions_tokens: {e}")
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
    impl ::std::convert::TryFrom<GraderScoreModelSamplingParams>
        for super::GraderScoreModelSamplingParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderScoreModelSamplingParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_completions_tokens: value.max_completions_tokens?,
                reasoning_effort: value.reasoning_effort?,
                seed: value.seed?,
                temperature: value.temperature?,
                top_p: value.top_p?,
            })
        }
    }
    impl ::std::convert::From<super::GraderScoreModelSamplingParams>
        for GraderScoreModelSamplingParams
    {
        fn from(value: super::GraderScoreModelSamplingParams) -> Self {
            Self {
                max_completions_tokens: Ok(value.max_completions_tokens),
                reasoning_effort: Ok(value.reasoning_effort),
                seed: Ok(value.seed),
                temperature: Ok(value.temperature),
                top_p: Ok(value.top_p),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderStringCheck {
        input: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        operation: ::std::result::Result<super::GraderStringCheckOperation, ::std::string::String>,
        reference: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::GraderStringCheckType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderStringCheck {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                name: Err("no value supplied for name".to_string()),
                operation: Err("no value supplied for operation".to_string()),
                reference: Err("no value supplied for reference".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GraderStringCheck {
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
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderStringCheckOperation>,
            T::Error: ::std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {e}"));
            self
        }
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderStringCheckType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderStringCheck> for super::GraderStringCheck {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderStringCheck,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                name: value.name?,
                operation: value.operation?,
                reference: value.reference?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GraderStringCheck> for GraderStringCheck {
        fn from(value: super::GraderStringCheck) -> Self {
            Self {
                input: Ok(value.input),
                name: Ok(value.name),
                operation: Ok(value.operation),
                reference: Ok(value.reference),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GraderTextSimilarity {
        evaluation_metric: ::std::result::Result<
            super::GraderTextSimilarityEvaluationMetric,
            ::std::string::String,
        >,
        input: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        reference: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::GraderTextSimilarityType, ::std::string::String>,
    }
    impl ::std::default::Default for GraderTextSimilarity {
        fn default() -> Self {
            Self {
                evaluation_metric: Err("no value supplied for evaluation_metric".to_string()),
                input: Err("no value supplied for input".to_string()),
                name: Err("no value supplied for name".to_string()),
                reference: Err("no value supplied for reference".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GraderTextSimilarity {
        pub fn evaluation_metric<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderTextSimilarityEvaluationMetric>,
            T::Error: ::std::fmt::Display,
        {
            self.evaluation_metric = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for evaluation_metric: {e}"));
            self
        }
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
        pub fn reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reference: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraderTextSimilarityType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GraderTextSimilarity> for super::GraderTextSimilarity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GraderTextSimilarity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                evaluation_metric: value.evaluation_metric?,
                input: value.input?,
                name: value.name?,
                reference: value.reference?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GraderTextSimilarity> for GraderTextSimilarity {
        fn from(value: super::GraderTextSimilarity) -> Self {
            Self {
                evaluation_metric: Ok(value.evaluation_metric),
                input: Ok(value.input),
                name: Ok(value.name),
                reference: Ok(value.reference),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputAudio {
        input_audio: ::std::result::Result<super::InputAudioInputAudio, ::std::string::String>,
        type_: ::std::result::Result<super::InputAudioType, ::std::string::String>,
    }
    impl ::std::default::Default for InputAudio {
        fn default() -> Self {
            Self {
                input_audio: Err("no value supplied for input_audio".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl InputAudio {
        pub fn input_audio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InputAudioInputAudio>,
            T::Error: ::std::fmt::Display,
        {
            self.input_audio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_audio: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InputAudioType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputAudio> for super::InputAudio {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_audio: value.input_audio?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::InputAudio> for InputAudio {
        fn from(value: super::InputAudio) -> Self {
            Self {
                input_audio: Ok(value.input_audio),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputAudioInputAudio {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
        format: ::std::result::Result<super::InputAudioInputAudioFormat, ::std::string::String>,
    }
    impl ::std::default::Default for InputAudioInputAudio {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                format: Err("no value supplied for format".to_string()),
            }
        }
    }
    impl InputAudioInputAudio {
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
            T: ::std::convert::TryInto<super::InputAudioInputAudioFormat>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputAudioInputAudio> for super::InputAudioInputAudio {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputAudioInputAudio,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                format: value.format?,
            })
        }
    }
    impl ::std::convert::From<super::InputAudioInputAudio> for InputAudioInputAudio {
        fn from(value: super::InputAudioInputAudio) -> Self {
            Self {
                data: Ok(value.data),
                format: Ok(value.format),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputTextContent {
        prompt_cache_breakpoint: ::std::result::Result<
            ::std::option::Option<super::PromptCacheBreakpointConfig>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::InputTextContentType, ::std::string::String>,
    }
    impl ::std::default::Default for InputTextContent {
        fn default() -> Self {
            Self {
                prompt_cache_breakpoint: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl InputTextContent {
        pub fn prompt_cache_breakpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PromptCacheBreakpointConfig>>,
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
            T: ::std::convert::TryInto<super::InputTextContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputTextContent> for super::InputTextContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputTextContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt_cache_breakpoint: value.prompt_cache_breakpoint?,
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::InputTextContent> for InputTextContent {
        fn from(value: super::InputTextContent) -> Self {
            Self {
                prompt_cache_breakpoint: Ok(value.prompt_cache_breakpoint),
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFineTuningCheckpointPermissionResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::FineTuningCheckpointPermission>,
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
        object: ::std::result::Result<
            super::ListFineTuningCheckpointPermissionResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListFineTuningCheckpointPermissionResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Ok(Default::default()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListFineTuningCheckpointPermissionResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FineTuningCheckpointPermission>>,
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
            T: ::std::convert::TryInto<super::ListFineTuningCheckpointPermissionResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFineTuningCheckpointPermissionResponse>
        for super::ListFineTuningCheckpointPermissionResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningCheckpointPermissionResponse,
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
    impl ::std::convert::From<super::ListFineTuningCheckpointPermissionResponse>
        for ListFineTuningCheckpointPermissionResponse
    {
        fn from(value: super::ListFineTuningCheckpointPermissionResponse) -> Self {
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
    pub struct ListFineTuningCheckpointPermissionsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListFineTuningCheckpointPermissionsParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListFineTuningCheckpointPermissionsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Ok(Default::default()),
            }
        }
    }
    impl ListFineTuningCheckpointPermissionsParams {
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
                ::std::option::Option<super::ListFineTuningCheckpointPermissionsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn project_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFineTuningCheckpointPermissionsParams>
        for super::ListFineTuningCheckpointPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningCheckpointPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListFineTuningCheckpointPermissionsParams>
        for ListFineTuningCheckpointPermissionsParams
    {
        fn from(value: super::ListFineTuningCheckpointPermissionsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFineTuningEventsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListFineTuningEventsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListFineTuningEventsParams {
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
    }
    impl ::std::convert::TryFrom<ListFineTuningEventsParams> for super::ListFineTuningEventsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningEventsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListFineTuningEventsParams> for ListFineTuningEventsParams {
        fn from(value: super::ListFineTuningEventsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFineTuningJobCheckpointsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListFineTuningJobCheckpointsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListFineTuningJobCheckpointsParams {
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
    }
    impl ::std::convert::TryFrom<ListFineTuningJobCheckpointsParams>
        for super::ListFineTuningJobCheckpointsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningJobCheckpointsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListFineTuningJobCheckpointsParams>
        for ListFineTuningJobCheckpointsParams
    {
        fn from(value: super::ListFineTuningJobCheckpointsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListFineTuningJobCheckpointsResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::FineTuningJobCheckpoint>,
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
        object: ::std::result::Result<
            super::ListFineTuningJobCheckpointsResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListFineTuningJobCheckpointsResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                first_id: Ok(Default::default()),
                has_more: Err("no value supplied for has_more".to_string()),
                last_id: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListFineTuningJobCheckpointsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FineTuningJobCheckpoint>>,
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
            T: ::std::convert::TryInto<super::ListFineTuningJobCheckpointsResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFineTuningJobCheckpointsResponse>
        for super::ListFineTuningJobCheckpointsResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningJobCheckpointsResponse,
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
    impl ::std::convert::From<super::ListFineTuningJobCheckpointsResponse>
        for ListFineTuningJobCheckpointsResponse
    {
        fn from(value: super::ListFineTuningJobCheckpointsResponse) -> Self {
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
    pub struct ListFineTuningJobEventsResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::FineTuningJobEvent>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<
            super::ListFineTuningJobEventsResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListFineTuningJobEventsResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListFineTuningJobEventsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FineTuningJobEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListFineTuningJobEventsResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListFineTuningJobEventsResponse>
        for super::ListFineTuningJobEventsResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListFineTuningJobEventsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ListFineTuningJobEventsResponse>
        for ListFineTuningJobEventsResponse
    {
        fn from(value: super::ListFineTuningJobEventsResponse) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPaginatedFineTuningJobsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListPaginatedFineTuningJobsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl ListPaginatedFineTuningJobsParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPaginatedFineTuningJobsParams>
        for super::ListPaginatedFineTuningJobsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPaginatedFineTuningJobsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::ListPaginatedFineTuningJobsParams>
        for ListPaginatedFineTuningJobsParams
    {
        fn from(value: super::ListPaginatedFineTuningJobsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListPaginatedFineTuningJobsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::FineTuningJob>, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<
            super::ListPaginatedFineTuningJobsResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListPaginatedFineTuningJobsResponse {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ListPaginatedFineTuningJobsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FineTuningJob>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListPaginatedFineTuningJobsResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListPaginatedFineTuningJobsResponse>
        for super::ListPaginatedFineTuningJobsResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListPaginatedFineTuningJobsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ListPaginatedFineTuningJobsResponse>
        for ListPaginatedFineTuningJobsResponse
    {
        fn from(value: super::ListPaginatedFineTuningJobsResponse) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PromptCacheBreakpointConfig {
        mode: ::std::result::Result<super::PromptCacheBreakpointConfigMode, ::std::string::String>,
    }
    impl ::std::default::Default for PromptCacheBreakpointConfig {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
            }
        }
    }
    impl PromptCacheBreakpointConfig {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PromptCacheBreakpointConfigMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PromptCacheBreakpointConfig> for super::PromptCacheBreakpointConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PromptCacheBreakpointConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { mode: value.mode? })
        }
    }
    impl ::std::convert::From<super::PromptCacheBreakpointConfig> for PromptCacheBreakpointConfig {
        fn from(value: super::PromptCacheBreakpointConfig) -> Self {
            Self {
                mode: Ok(value.mode),
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
