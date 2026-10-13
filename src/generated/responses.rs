#[doc = "`AccessProgramsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AccessProgramsBody {
    #[doc = "The effective Cyber access program used for this response."]
    pub cyber: CyberAccessProgramEnum,
}
#[doc = "Domain-specific access programs to use for this request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AccessProgramsParam {
    #[doc = "The Cyber access program to use for this request. Supported values are `standard`, `daybreak_blue`, and `daybreak_red`. If omitted, the API resolves the program from the model's Cyber tier and your organization and project access, subject to model-specific eligibility restrictions. By default, models without a Cyber tier use Standard. Blue-tier models use Daybreak Blue when authorized; otherwise they fall back to Standard unless the model requires Daybreak access. Red-tier models use Daybreak Red and require authorization. Requests that require unavailable Daybreak access return 403. An implicit Standard fallback is represented by null in the response's access_programs field, rather than an explicit Standard selection."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cyber: ::std::option::Option<CyberAccessProgramEnum>,
}
#[doc = "`AdditionalTools`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdditionalTools {
    #[doc = "The unique ID of the additional tools item."]
    pub id: ::std::string::String,
    #[doc = "The role that provided the additional tools."]
    pub role: MessageRole,
    #[doc = "The additional tool definitions made available at this item."]
    pub tools: ::std::vec::Vec<Tool>,
    #[doc = "The type of the item. Always `additional_tools`."]
    #[serde(rename = "type")]
    pub type_: AdditionalToolsType,
}
#[doc = "`AdditionalToolsItemParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdditionalToolsItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The role that provided the additional tools. Only `developer` is supported."]
    pub role: AdditionalToolsItemParamRole,
    #[doc = "A list of additional tools made available at this item."]
    pub tools: ::std::vec::Vec<Tool>,
    #[doc = "The item type. Always `additional_tools`."]
    #[serde(rename = "type")]
    pub type_: AdditionalToolsItemParamType,
}
#[doc = "The role that provided the additional tools. Only `developer` is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdditionalToolsItemParamRole {
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for AdditionalToolsItemParamRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for AdditionalToolsItemParamRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdditionalToolsItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdditionalToolsItemParamRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The item type. Always `additional_tools`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdditionalToolsItemParamType {
    #[serde(rename = "additional_tools")]
    AdditionalTools,
}
impl ::std::fmt::Display for AdditionalToolsItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AdditionalTools => f.write_str("additional_tools"),
        }
    }
}
impl ::std::str::FromStr for AdditionalToolsItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "additional_tools" => Ok(Self::AdditionalTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdditionalToolsItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdditionalToolsItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `additional_tools`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdditionalToolsType {
    #[serde(rename = "additional_tools")]
    AdditionalTools,
}
impl ::std::fmt::Display for AdditionalToolsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AdditionalTools => f.write_str("additional_tools"),
        }
    }
}
impl ::std::str::FromStr for AdditionalToolsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "additional_tools" => Ok(Self::AdditionalTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdditionalToolsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdditionalToolsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An annotation that applies to a span of output text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Annotation {
    FileCitationBody(FileCitationBody),
    UrlCitationBody(UrlCitationBody),
    ContainerFileCitationBody(ContainerFileCitationBody),
    FilePath(FilePath),
}
impl ::std::convert::From<FileCitationBody> for Annotation {
    fn from(value: FileCitationBody) -> Self {
        Self::FileCitationBody(value)
    }
}
impl ::std::convert::From<UrlCitationBody> for Annotation {
    fn from(value: UrlCitationBody) -> Self {
        Self::UrlCitationBody(value)
    }
}
impl ::std::convert::From<ContainerFileCitationBody> for Annotation {
    fn from(value: ContainerFileCitationBody) -> Self {
        Self::ContainerFileCitationBody(value)
    }
}
impl ::std::convert::From<FilePath> for Annotation {
    fn from(value: FilePath) -> Self {
        Self::FilePath(value)
    }
}
#[doc = "`ApplyPatchCallOutputStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCallOutputStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ApplyPatchCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Outcome values reported for apply_patch tool call outputs."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCallOutputStatusParam {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ApplyPatchCallOutputStatusParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCallOutputStatusParam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCallOutputStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCallOutputStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ApplyPatchCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for ApplyPatchCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Status values reported for apply_patch tool calls."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCallStatusParam {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for ApplyPatchCallStatusParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCallStatusParam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCallStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCallStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Instruction describing how to create a file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchCreateFileOperation {
    #[doc = "Diff to apply."]
    pub diff: ::std::string::String,
    #[doc = "Path of the file to create."]
    pub path: ::std::string::String,
    #[doc = "Create a new file with the provided diff."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchCreateFileOperationType,
}
#[doc = "Instruction for creating a new file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchCreateFileOperationParam {
    #[doc = "Unified diff content to apply when creating the file."]
    pub diff: ApplyPatchCreateFileOperationParamDiff,
    #[doc = "Path of the file to create relative to the workspace root."]
    pub path: ApplyPatchCreateFileOperationParamPath,
    #[doc = "The operation type. Always `create_file`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchCreateFileOperationParamType,
}
#[doc = "Unified diff content to apply when creating the file."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchCreateFileOperationParamDiff(::std::string::String);
impl ::std::ops::Deref for ApplyPatchCreateFileOperationParamDiff {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchCreateFileOperationParamDiff> for ::std::string::String {
    fn from(value: ApplyPatchCreateFileOperationParamDiff) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchCreateFileOperationParamDiff {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCreateFileOperationParamDiff {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCreateFileOperationParamDiff {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchCreateFileOperationParamDiff {
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
#[doc = "Path of the file to create relative to the workspace root."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchCreateFileOperationParamPath(::std::string::String);
impl ::std::ops::Deref for ApplyPatchCreateFileOperationParamPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchCreateFileOperationParamPath> for ::std::string::String {
    fn from(value: ApplyPatchCreateFileOperationParamPath) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchCreateFileOperationParamPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCreateFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCreateFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchCreateFileOperationParamPath {
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
#[doc = "The operation type. Always `create_file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCreateFileOperationParamType {
    #[serde(rename = "create_file")]
    CreateFile,
}
impl ::std::fmt::Display for ApplyPatchCreateFileOperationParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreateFile => f.write_str("create_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCreateFileOperationParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "create_file" => Ok(Self::CreateFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCreateFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCreateFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Create a new file with the provided diff."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchCreateFileOperationType {
    #[serde(rename = "create_file")]
    CreateFile,
}
impl ::std::fmt::Display for ApplyPatchCreateFileOperationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreateFile => f.write_str("create_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchCreateFileOperationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "create_file" => Ok(Self::CreateFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchCreateFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchCreateFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Instruction describing how to delete a file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchDeleteFileOperation {
    #[doc = "Path of the file to delete."]
    pub path: ::std::string::String,
    #[doc = "Delete the specified file."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchDeleteFileOperationType,
}
#[doc = "Instruction for deleting an existing file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchDeleteFileOperationParam {
    #[doc = "Path of the file to delete relative to the workspace root."]
    pub path: ApplyPatchDeleteFileOperationParamPath,
    #[doc = "The operation type. Always `delete_file`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchDeleteFileOperationParamType,
}
#[doc = "Path of the file to delete relative to the workspace root."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchDeleteFileOperationParamPath(::std::string::String);
impl ::std::ops::Deref for ApplyPatchDeleteFileOperationParamPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchDeleteFileOperationParamPath> for ::std::string::String {
    fn from(value: ApplyPatchDeleteFileOperationParamPath) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchDeleteFileOperationParamPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchDeleteFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchDeleteFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchDeleteFileOperationParamPath {
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
#[doc = "The operation type. Always `delete_file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchDeleteFileOperationParamType {
    #[serde(rename = "delete_file")]
    DeleteFile,
}
impl ::std::fmt::Display for ApplyPatchDeleteFileOperationParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DeleteFile => f.write_str("delete_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchDeleteFileOperationParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "delete_file" => Ok(Self::DeleteFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchDeleteFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchDeleteFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Delete the specified file."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchDeleteFileOperationType {
    #[serde(rename = "delete_file")]
    DeleteFile,
}
impl ::std::fmt::Display for ApplyPatchDeleteFileOperationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DeleteFile => f.write_str("delete_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchDeleteFileOperationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "delete_file" => Ok(Self::DeleteFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchDeleteFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchDeleteFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "One of the create_file, delete_file, or update_file operations applied via apply_patch."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ApplyPatchOperation {
    CreateFileOperation(ApplyPatchCreateFileOperation),
    DeleteFileOperation(ApplyPatchDeleteFileOperation),
    UpdateFileOperation(ApplyPatchUpdateFileOperation),
}
impl ::std::convert::From<ApplyPatchCreateFileOperation> for ApplyPatchOperation {
    fn from(value: ApplyPatchCreateFileOperation) -> Self {
        Self::CreateFileOperation(value)
    }
}
impl ::std::convert::From<ApplyPatchDeleteFileOperation> for ApplyPatchOperation {
    fn from(value: ApplyPatchDeleteFileOperation) -> Self {
        Self::DeleteFileOperation(value)
    }
}
impl ::std::convert::From<ApplyPatchUpdateFileOperation> for ApplyPatchOperation {
    fn from(value: ApplyPatchUpdateFileOperation) -> Self {
        Self::UpdateFileOperation(value)
    }
}
#[doc = "One of the create_file, delete_file, or update_file operations supplied to the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ApplyPatchOperationParam {
    CreateFileOperationParam(ApplyPatchCreateFileOperationParam),
    DeleteFileOperationParam(ApplyPatchDeleteFileOperationParam),
    UpdateFileOperationParam(ApplyPatchUpdateFileOperationParam),
}
impl ::std::convert::From<ApplyPatchCreateFileOperationParam> for ApplyPatchOperationParam {
    fn from(value: ApplyPatchCreateFileOperationParam) -> Self {
        Self::CreateFileOperationParam(value)
    }
}
impl ::std::convert::From<ApplyPatchDeleteFileOperationParam> for ApplyPatchOperationParam {
    fn from(value: ApplyPatchDeleteFileOperationParam) -> Self {
        Self::DeleteFileOperationParam(value)
    }
}
impl ::std::convert::From<ApplyPatchUpdateFileOperationParam> for ApplyPatchOperationParam {
    fn from(value: ApplyPatchUpdateFileOperationParam) -> Self {
        Self::UpdateFileOperationParam(value)
    }
}
#[doc = "A tool call that applies file diffs by creating, deleting, or updating files."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchToolCall {
    #[doc = "The unique ID of the apply patch tool call generated by the model."]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The ID of the entity that created this tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the apply patch tool call. Populated when this item is returned via API."]
    pub id: ::std::string::String,
    #[doc = "One of the create_file, delete_file, or update_file operations applied via apply_patch."]
    pub operation: ApplyPatchOperation,
    #[doc = "The status of the apply patch tool call. One of `in_progress` or `completed`."]
    pub status: ApplyPatchCallStatus,
    #[doc = "The type of the item. Always `apply_patch_call`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchToolCallType,
}
#[doc = "A tool call representing a request to create, delete, or update files using diff patches."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchToolCallItemParam {
    #[doc = "The unique ID of the apply patch tool call generated by the model."]
    pub call_id: ApplyPatchToolCallItemParamCallId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The specific create, delete, or update instruction for the apply_patch tool call."]
    pub operation: ApplyPatchOperationParam,
    #[doc = "The status of the apply patch tool call. One of `in_progress` or `completed`."]
    pub status: ApplyPatchCallStatusParam,
    #[doc = "The type of the item. Always `apply_patch_call`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchToolCallItemParamType,
}
#[doc = "The unique ID of the apply patch tool call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchToolCallItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ApplyPatchToolCallItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchToolCallItemParamCallId> for ::std::string::String {
    fn from(value: ApplyPatchToolCallItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchToolCallItemParamCallId {
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
#[doc = "The type of the item. Always `apply_patch_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchToolCallItemParamType {
    #[serde(rename = "apply_patch_call")]
    ApplyPatchCall,
}
impl ::std::fmt::Display for ApplyPatchToolCallItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatchCall => f.write_str("apply_patch_call"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch_call" => Ok(Self::ApplyPatchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output emitted by an apply patch tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchToolCallOutput {
    #[doc = "The unique ID of the apply patch tool call generated by the model."]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The ID of the entity that created this tool call output."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the apply patch tool call output. Populated when this item is returned via API."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
    #[doc = "The status of the apply patch tool call output. One of `completed` or `failed`."]
    pub status: ApplyPatchCallOutputStatus,
    #[doc = "The type of the item. Always `apply_patch_call_output`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchToolCallOutputType,
}
#[doc = "The streamed output emitted by an apply patch tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchToolCallOutputItemParam {
    #[doc = "The unique ID of the apply patch tool call generated by the model."]
    pub call_id: ApplyPatchToolCallOutputItemParamCallId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<ApplyPatchToolCallOutputItemParamOutput>,
    #[doc = "The status of the apply patch tool call output. One of `completed` or `failed`."]
    pub status: ApplyPatchCallOutputStatusParam,
    #[doc = "The type of the item. Always `apply_patch_call_output`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchToolCallOutputItemParamType,
}
#[doc = "The unique ID of the apply patch tool call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchToolCallOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ApplyPatchToolCallOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutputItemParamCallId> for ::std::string::String {
    fn from(value: ApplyPatchToolCallOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchToolCallOutputItemParamCallId {
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
#[doc = "Optional human-readable log text from the apply patch tool (e.g., patch results or errors)."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchToolCallOutputItemParamOutput(::std::string::String);
impl ::std::ops::Deref for ApplyPatchToolCallOutputItemParamOutput {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutputItemParamOutput> for ::std::string::String {
    fn from(value: ApplyPatchToolCallOutputItemParamOutput) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallOutputItemParamOutput {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallOutputItemParamOutput {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallOutputItemParamOutput {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchToolCallOutputItemParamOutput {
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
#[doc = "The type of the item. Always `apply_patch_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchToolCallOutputItemParamType {
    #[serde(rename = "apply_patch_call_output")]
    ApplyPatchCallOutput,
}
impl ::std::fmt::Display for ApplyPatchToolCallOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatchCallOutput => f.write_str("apply_patch_call_output"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch_call_output" => Ok(Self::ApplyPatchCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `apply_patch_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchToolCallOutputType {
    #[serde(rename = "apply_patch_call_output")]
    ApplyPatchCallOutput,
}
impl ::std::fmt::Display for ApplyPatchToolCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatchCallOutput => f.write_str("apply_patch_call_output"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch_call_output" => Ok(Self::ApplyPatchCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `apply_patch_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchToolCallType {
    #[serde(rename = "apply_patch_call")]
    ApplyPatchCall,
}
impl ::std::fmt::Display for ApplyPatchToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatchCall => f.write_str("apply_patch_call"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch_call" => Ok(Self::ApplyPatchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Allows the assistant to create, delete, or update files using unified diffs."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[doc = "The type of the tool. Always `apply_patch`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchToolParamType,
}
#[doc = "The type of the tool. Always `apply_patch`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchToolParamType {
    #[serde(rename = "apply_patch")]
    ApplyPatch,
}
impl ::std::fmt::Display for ApplyPatchToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatch => f.write_str("apply_patch"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch" => Ok(Self::ApplyPatch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Instruction describing how to update a file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchUpdateFileOperation {
    #[doc = "Diff to apply."]
    pub diff: ::std::string::String,
    #[doc = "Path of the file to update."]
    pub path: ::std::string::String,
    #[doc = "Update an existing file with the provided diff."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchUpdateFileOperationType,
}
#[doc = "Instruction for updating an existing file via the apply_patch tool."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApplyPatchUpdateFileOperationParam {
    #[doc = "Unified diff content to apply to the existing file."]
    pub diff: ApplyPatchUpdateFileOperationParamDiff,
    #[doc = "Path of the file to update relative to the workspace root."]
    pub path: ApplyPatchUpdateFileOperationParamPath,
    #[doc = "The operation type. Always `update_file`."]
    #[serde(rename = "type")]
    pub type_: ApplyPatchUpdateFileOperationParamType,
}
#[doc = "Unified diff content to apply to the existing file."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchUpdateFileOperationParamDiff(::std::string::String);
impl ::std::ops::Deref for ApplyPatchUpdateFileOperationParamDiff {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchUpdateFileOperationParamDiff> for ::std::string::String {
    fn from(value: ApplyPatchUpdateFileOperationParamDiff) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchUpdateFileOperationParamDiff {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchUpdateFileOperationParamDiff {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchUpdateFileOperationParamDiff {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchUpdateFileOperationParamDiff {
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
#[doc = "Path of the file to update relative to the workspace root."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ApplyPatchUpdateFileOperationParamPath(::std::string::String);
impl ::std::ops::Deref for ApplyPatchUpdateFileOperationParamPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ApplyPatchUpdateFileOperationParamPath> for ::std::string::String {
    fn from(value: ApplyPatchUpdateFileOperationParamPath) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ApplyPatchUpdateFileOperationParamPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchUpdateFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchUpdateFileOperationParamPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ApplyPatchUpdateFileOperationParamPath {
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
#[doc = "The operation type. Always `update_file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchUpdateFileOperationParamType {
    #[serde(rename = "update_file")]
    UpdateFile,
}
impl ::std::fmt::Display for ApplyPatchUpdateFileOperationParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UpdateFile => f.write_str("update_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchUpdateFileOperationParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "update_file" => Ok(Self::UpdateFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchUpdateFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchUpdateFileOperationParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Update an existing file with the provided diff."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApplyPatchUpdateFileOperationType {
    #[serde(rename = "update_file")]
    UpdateFile,
}
impl ::std::fmt::Display for ApplyPatchUpdateFileOperationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UpdateFile => f.write_str("update_file"),
        }
    }
}
impl ::std::str::FromStr for ApplyPatchUpdateFileOperationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "update_file" => Ok(Self::UpdateFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApplyPatchUpdateFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApplyPatchUpdateFileOperationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ApproximateLocation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApproximateLocation {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
    #[doc = "The type of location approximation. Always `approximate`."]
    #[serde(rename = "type")]
    pub type_: ApproximateLocationType,
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
pub enum ApproximateLocationType {
    #[serde(rename = "approximate")]
    Approximate,
}
impl ::std::fmt::Display for ApproximateLocationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Approximate => f.write_str("approximate"),
        }
    }
}
impl ::std::str::FromStr for ApproximateLocationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "approximate" => Ok(Self::Approximate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApproximateLocationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApproximateLocationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration for a code interpreter container. Optionally specify the IDs of the files to run the code on."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AutoCodeInterpreterToolParam {
    #[doc = "An optional list of uploaded files to make available to your code."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_limit: ::std::option::Option<ContainerMemoryLimit>,
    #[doc = "Network access policy for the container."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_policy: ::std::option::Option<AutoCodeInterpreterToolParamNetworkPolicy>,
    #[doc = "Always `auto`."]
    #[serde(rename = "type")]
    pub type_: AutoCodeInterpreterToolParamType,
}
#[doc = "Network access policy for the container."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AutoCodeInterpreterToolParamNetworkPolicy {
    DisabledParam(ContainerNetworkPolicyDisabledParam),
    AllowlistParam(ContainerNetworkPolicyAllowlistParam),
}
impl ::std::convert::From<ContainerNetworkPolicyDisabledParam>
    for AutoCodeInterpreterToolParamNetworkPolicy
{
    fn from(value: ContainerNetworkPolicyDisabledParam) -> Self {
        Self::DisabledParam(value)
    }
}
impl ::std::convert::From<ContainerNetworkPolicyAllowlistParam>
    for AutoCodeInterpreterToolParamNetworkPolicy
{
    fn from(value: ContainerNetworkPolicyAllowlistParam) -> Self {
        Self::AllowlistParam(value)
    }
}
#[doc = "Always `auto`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AutoCodeInterpreterToolParamType {
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for AutoCodeInterpreterToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for AutoCodeInterpreterToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AutoCodeInterpreterToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AutoCodeInterpreterToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CacheMissReasonTypeEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CacheMissReasonTypeEnum {
    #[serde(rename = "model_changed")]
    ModelChanged,
    #[serde(rename = "prompt_cache_key_changed")]
    PromptCacheKeyChanged,
    #[serde(rename = "tools_changed")]
    ToolsChanged,
    #[serde(rename = "text_format_changed")]
    TextFormatChanged,
    #[serde(rename = "reasoning_effort_changed")]
    ReasoningEffortChanged,
    #[serde(rename = "verbosity_changed")]
    VerbosityChanged,
    #[serde(rename = "context_compacted")]
    ContextCompacted,
    #[serde(rename = "input_changed")]
    InputChanged,
    #[serde(rename = "service_tier_changed")]
    ServiceTierChanged,
}
impl ::std::fmt::Display for CacheMissReasonTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ModelChanged => f.write_str("model_changed"),
            Self::PromptCacheKeyChanged => f.write_str("prompt_cache_key_changed"),
            Self::ToolsChanged => f.write_str("tools_changed"),
            Self::TextFormatChanged => f.write_str("text_format_changed"),
            Self::ReasoningEffortChanged => f.write_str("reasoning_effort_changed"),
            Self::VerbosityChanged => f.write_str("verbosity_changed"),
            Self::ContextCompacted => f.write_str("context_compacted"),
            Self::InputChanged => f.write_str("input_changed"),
            Self::ServiceTierChanged => f.write_str("service_tier_changed"),
        }
    }
}
impl ::std::str::FromStr for CacheMissReasonTypeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "model_changed" => Ok(Self::ModelChanged),
            "prompt_cache_key_changed" => Ok(Self::PromptCacheKeyChanged),
            "tools_changed" => Ok(Self::ToolsChanged),
            "text_format_changed" => Ok(Self::TextFormatChanged),
            "reasoning_effort_changed" => Ok(Self::ReasoningEffortChanged),
            "verbosity_changed" => Ok(Self::VerbosityChanged),
            "context_compacted" => Ok(Self::ContextCompacted),
            "input_changed" => Ok(Self::InputChanged),
            "service_tier_changed" => Ok(Self::ServiceTierChanged),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CacheMissReasonTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CacheMissReasonTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CallableToolAllowedCaller`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CallableToolAllowedCaller {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "programmatic")]
    Programmatic,
}
impl ::std::fmt::Display for CallableToolAllowedCaller {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Direct => f.write_str("direct"),
            Self::Programmatic => f.write_str("programmatic"),
        }
    }
}
impl ::std::str::FromStr for CallableToolAllowedCaller {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "direct" => Ok(Self::Direct),
            "programmatic" => Ok(Self::Programmatic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallableToolAllowedCaller {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallableToolAllowedCaller {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ClickButtonType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClickButtonType {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}
impl ::std::fmt::Display for ClickButtonType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Wheel => f.write_str("wheel"),
            Self::Back => f.write_str("back"),
            Self::Forward => f.write_str("forward"),
        }
    }
}
impl ::std::str::FromStr for ClickButtonType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "wheel" => Ok(Self::Wheel),
            "back" => Ok(Self::Back),
            "forward" => Ok(Self::Forward),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClickButtonType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClickButtonType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A click action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ClickParam {
    #[doc = "Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`."]
    pub button: ClickButtonType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Specifies the event type. For a click action, this property is always `click`."]
    #[serde(rename = "type")]
    pub type_: ClickParamType,
    #[doc = "The x-coordinate where the click occurred."]
    pub x: i64,
    #[doc = "The y-coordinate where the click occurred."]
    pub y: i64,
}
#[doc = "Specifies the event type. For a click action, this property is always `click`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ClickParamType {
    #[serde(rename = "click")]
    Click,
}
impl ::std::fmt::Display for ClickParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Click => f.write_str("click"),
        }
    }
}
impl ::std::str::FromStr for ClickParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "click" => Ok(Self::Click),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClickParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClickParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The image output from the code interpreter."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CodeInterpreterOutputImage {
    #[doc = "The type of the output. Always `image`."]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterOutputImageType,
    #[doc = "The URL of the image output from the code interpreter."]
    pub url: ::std::string::String,
}
#[doc = "The type of the output. Always `image`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeInterpreterOutputImageType {
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for CodeInterpreterOutputImageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for CodeInterpreterOutputImageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeInterpreterOutputImageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeInterpreterOutputImageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The logs output from the code interpreter."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CodeInterpreterOutputLogs {
    #[doc = "The logs output from the code interpreter."]
    pub logs: ::std::string::String,
    #[doc = "The type of the output. Always `logs`."]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterOutputLogsType,
}
#[doc = "The type of the output. Always `logs`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeInterpreterOutputLogsType {
    #[serde(rename = "logs")]
    Logs,
}
impl ::std::fmt::Display for CodeInterpreterOutputLogsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Logs => f.write_str("logs"),
        }
    }
}
impl ::std::str::FromStr for CodeInterpreterOutputLogsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "logs" => Ok(Self::Logs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeInterpreterOutputLogsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeInterpreterOutputLogsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that runs Python code to help generate a response to a prompt.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CodeInterpreterTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[doc = "The code interpreter container. Can be a container ID or an object that\nspecifies uploaded file IDs to make available to your code, along with an\noptional `memory_limit` setting.\n"]
    pub container: CodeInterpreterToolContainer,
    #[doc = "The type of the code interpreter tool. Always `code_interpreter`.\n"]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterToolType,
}
#[doc = "A tool call to run code.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CodeInterpreterToolCall {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the container used to run the code.\n"]
    pub container_id: ::std::string::String,
    #[doc = "The unique ID of the code interpreter tool call.\n"]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub outputs: ::std::option::Option<::std::vec::Vec<CodeInterpreterToolCallOutputsItem>>,
    #[doc = "The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.\n"]
    pub status: CodeInterpreterToolCallStatus,
    #[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterToolCallType,
}
#[doc = "`CodeInterpreterToolCallOutputsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CodeInterpreterToolCallOutputsItem {
    Logs(CodeInterpreterOutputLogs),
    Image(CodeInterpreterOutputImage),
}
impl ::std::convert::From<CodeInterpreterOutputLogs> for CodeInterpreterToolCallOutputsItem {
    fn from(value: CodeInterpreterOutputLogs) -> Self {
        Self::Logs(value)
    }
}
impl ::std::convert::From<CodeInterpreterOutputImage> for CodeInterpreterToolCallOutputsItem {
    fn from(value: CodeInterpreterOutputImage) -> Self {
        Self::Image(value)
    }
}
#[doc = "The status of the code interpreter tool call. Valid values are `in_progress`, `completed`, `incomplete`, `interpreting`, and `failed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeInterpreterToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "interpreting")]
    Interpreting,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for CodeInterpreterToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Interpreting => f.write_str("interpreting"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for CodeInterpreterToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "interpreting" => Ok(Self::Interpreting),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeInterpreterToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeInterpreterToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeInterpreterToolCallType {
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterCall,
}
impl ::std::fmt::Display for CodeInterpreterToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreterCall => f.write_str("code_interpreter_call"),
        }
    }
}
impl ::std::str::FromStr for CodeInterpreterToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter_call" => Ok(Self::CodeInterpreterCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeInterpreterToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeInterpreterToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The code interpreter container. Can be a container ID or an object that\nspecifies uploaded file IDs to make available to your code, along with an\noptional `memory_limit` setting.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CodeInterpreterToolContainer {
    String(::std::string::String),
    AutoCodeInterpreterToolParam(AutoCodeInterpreterToolParam),
}
impl ::std::convert::From<AutoCodeInterpreterToolParam> for CodeInterpreterToolContainer {
    fn from(value: AutoCodeInterpreterToolParam) -> Self {
        Self::AutoCodeInterpreterToolParam(value)
    }
}
#[doc = "The type of the code interpreter tool. Always `code_interpreter`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeInterpreterToolType {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for CodeInterpreterToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for CodeInterpreterToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeInterpreterToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeInterpreterToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CompactResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompactResource {
    #[doc = "Unix timestamp (in seconds) when the compacted conversation was created."]
    pub created_at: i64,
    #[doc = "The unique identifier for the compacted response."]
    pub id: ::std::string::String,
    #[doc = "The object type. Always `response.compaction`."]
    pub object: CompactResourceObject,
    #[doc = "The compacted list of output items."]
    pub output: ::std::vec::Vec<ItemField>,
    #[doc = "Token accounting for the compaction pass, including cached, reasoning, and total tokens."]
    pub usage: ResponseUsage,
}
#[doc = "The object type. Always `response.compaction`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CompactResourceObject {
    #[serde(rename = "response.compaction")]
    ResponseCompaction,
}
impl ::std::fmt::Display for CompactResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCompaction => f.write_str("response.compaction"),
        }
    }
}
impl ::std::str::FromStr for CompactResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.compaction" => Ok(Self::ResponseCompaction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompactResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CompactResponseMethodPublicBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompactResponseMethodPublicBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<CompactResponseMethodPublicBodyInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    pub model: ModelIdsCompaction,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_response_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<CompactResponseMethodPublicBodyPromptCacheKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_options: ::std::option::Option<PromptCacheOptionsParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention: ::std::option::Option<PromptCacheRetentionEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTierEnum>,
}
#[doc = "Text, image, or file inputs to the model, used to generate a response"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CompactResponseMethodPublicBodyInput {
    String(CompactResponseMethodPublicBodyInputString),
    Array(::std::vec::Vec<InputItem>),
}
impl ::std::convert::From<CompactResponseMethodPublicBodyInputString>
    for CompactResponseMethodPublicBodyInput
{
    fn from(value: CompactResponseMethodPublicBodyInputString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<InputItem>> for CompactResponseMethodPublicBodyInput {
    fn from(value: ::std::vec::Vec<InputItem>) -> Self {
        Self::Array(value)
    }
}
#[doc = "A text input to the model, equivalent to a text input with the `user` role."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CompactResponseMethodPublicBodyInputString(::std::string::String);
impl ::std::ops::Deref for CompactResponseMethodPublicBodyInputString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CompactResponseMethodPublicBodyInputString> for ::std::string::String {
    fn from(value: CompactResponseMethodPublicBodyInputString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CompactResponseMethodPublicBodyInputString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CompactResponseMethodPublicBodyInputString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactResponseMethodPublicBodyInputString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CompactResponseMethodPublicBodyInputString {
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
#[doc = "A key to use when reading from or writing to the prompt cache."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CompactResponseMethodPublicBodyPromptCacheKey(::std::string::String);
impl ::std::ops::Deref for CompactResponseMethodPublicBodyPromptCacheKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CompactResponseMethodPublicBodyPromptCacheKey> for ::std::string::String {
    fn from(value: CompactResponseMethodPublicBodyPromptCacheKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CompactResponseMethodPublicBodyPromptCacheKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CompactResponseMethodPublicBodyPromptCacheKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CompactResponseMethodPublicBodyPromptCacheKey
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CompactResponseMethodPublicBodyPromptCacheKey {
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
#[doc = "A compaction item generated by the [`v1/responses/compact` API](https://developers.openai.com/api/reference/resources/responses/methods/compact)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompactionBody {
    #[doc = "The identifier of the actor that created the item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "The encrypted content that was produced by compaction."]
    pub encrypted_content: ::std::string::String,
    #[doc = "The unique ID of the compaction item."]
    pub id: ::std::string::String,
    #[doc = "The type of the item. Always `compaction`."]
    #[serde(rename = "type")]
    pub type_: CompactionBodyType,
}
#[doc = "The type of the item. Always `compaction`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CompactionBodyType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for CompactionBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for CompactionBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compaction" => Ok(Self::Compaction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompactionBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactionBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A compaction item generated by the [`v1/responses/compact` API](https://developers.openai.com/api/reference/resources/responses/methods/compact)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompactionSummaryItemParam {
    #[doc = "The encrypted content of the compaction summary."]
    pub encrypted_content: CompactionSummaryItemParamEncryptedContent,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the item. Always `compaction`."]
    #[serde(rename = "type")]
    pub type_: CompactionSummaryItemParamType,
}
#[doc = "The encrypted content of the compaction summary."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CompactionSummaryItemParamEncryptedContent(::std::string::String);
impl ::std::ops::Deref for CompactionSummaryItemParamEncryptedContent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CompactionSummaryItemParamEncryptedContent> for ::std::string::String {
    fn from(value: CompactionSummaryItemParamEncryptedContent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CompactionSummaryItemParamEncryptedContent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 104857600usize {
            return Err("longer than 104857600 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CompactionSummaryItemParamEncryptedContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactionSummaryItemParamEncryptedContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CompactionSummaryItemParamEncryptedContent {
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
#[doc = "The type of the item. Always `compaction`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CompactionSummaryItemParamType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for CompactionSummaryItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for CompactionSummaryItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compaction" => Ok(Self::Compaction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompactionSummaryItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactionSummaryItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Compacts the current context. Must be the final input item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CompactionTriggerItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the item. Always `compaction_trigger`."]
    #[serde(rename = "type")]
    pub type_: CompactionTriggerItemParamType,
}
#[doc = "The type of the item. Always `compaction_trigger`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CompactionTriggerItemParamType {
    #[serde(rename = "compaction_trigger")]
    CompactionTrigger,
}
impl ::std::fmt::Display for CompactionTriggerItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CompactionTrigger => f.write_str("compaction_trigger"),
        }
    }
}
impl ::std::str::FromStr for CompactionTriggerItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compaction_trigger" => Ok(Self::CompactionTrigger),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompactionTriggerItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompactionTriggerItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A filter used to compare a specified attribute key to a given value using a defined comparison operation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ComparisonFilter {
    #[doc = "The key to compare against the value."]
    pub key: ::std::string::String,
    #[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `nin`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n- `in`: in\n- `nin`: not in\n"]
    #[serde(rename = "type")]
    pub type_: ComparisonFilterType,
    #[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
    pub value: ComparisonFilterValue,
}
#[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `in`, `nin`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n- `in`: in\n- `nin`: not in\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComparisonFilterType {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "nin")]
    Nin,
}
impl ::std::fmt::Display for ComparisonFilterType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Eq => f.write_str("eq"),
            Self::Ne => f.write_str("ne"),
            Self::Gt => f.write_str("gt"),
            Self::Gte => f.write_str("gte"),
            Self::Lt => f.write_str("lt"),
            Self::Lte => f.write_str("lte"),
            Self::In => f.write_str("in"),
            Self::Nin => f.write_str("nin"),
        }
    }
}
impl ::std::str::FromStr for ComparisonFilterType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "in" => Ok(Self::In),
            "nin" => Ok(Self::Nin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComparisonFilterType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComparisonFilterType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
    String(::std::string::String),
    Number(f64),
    Boolean(bool),
    Array(::std::vec::Vec<ComparisonFilterValueArrayItem>),
}
impl ::std::convert::From<f64> for ComparisonFilterValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<bool> for ComparisonFilterValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<ComparisonFilterValueArrayItem>>
    for ComparisonFilterValue
{
    fn from(value: ::std::vec::Vec<ComparisonFilterValueArrayItem>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`ComparisonFilterValueArrayItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ComparisonFilterValueArrayItem {
    String(::std::string::String),
    Number(f64),
}
impl ::std::fmt::Display for ComparisonFilterValueArrayItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<f64> for ComparisonFilterValueArrayItem {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "Combine multiple filters using `and` or `or`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompoundFilter {
    #[doc = "Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`."]
    pub filters: ::std::vec::Vec<CompoundFilterFiltersItem>,
    #[doc = "Type of operation: `and` or `or`."]
    #[serde(rename = "type")]
    pub type_: CompoundFilterType,
}
#[doc = "`CompoundFilterFiltersItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CompoundFilterFiltersItem {
    ComparisonFilter(ComparisonFilter),
    CompoundFilter(CompoundFilter),
}
impl ::std::convert::From<ComparisonFilter> for CompoundFilterFiltersItem {
    fn from(value: ComparisonFilter) -> Self {
        Self::ComparisonFilter(value)
    }
}
impl ::std::convert::From<CompoundFilter> for CompoundFilterFiltersItem {
    fn from(value: CompoundFilter) -> Self {
        Self::CompoundFilter(value)
    }
}
#[doc = "Type of operation: `and` or `or`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CompoundFilterType {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
impl ::std::fmt::Display for CompoundFilterType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::And => f.write_str("and"),
            Self::Or => f.write_str("or"),
        }
    }
}
impl ::std::str::FromStr for CompoundFilterType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CompoundFilterType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CompoundFilterType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ComputerAction`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ComputerAction {
    ClickParam(ClickParam),
    DoubleClickAction(DoubleClickAction),
    DragParam(DragParam),
    KeyPressAction(KeyPressAction),
    MoveParam(MoveParam),
    ScreenshotParam(ScreenshotParam),
    ScrollParam(ScrollParam),
    TypeParam(TypeParam),
    WaitParam(WaitParam),
}
impl ::std::convert::From<ClickParam> for ComputerAction {
    fn from(value: ClickParam) -> Self {
        Self::ClickParam(value)
    }
}
impl ::std::convert::From<DoubleClickAction> for ComputerAction {
    fn from(value: DoubleClickAction) -> Self {
        Self::DoubleClickAction(value)
    }
}
impl ::std::convert::From<DragParam> for ComputerAction {
    fn from(value: DragParam) -> Self {
        Self::DragParam(value)
    }
}
impl ::std::convert::From<KeyPressAction> for ComputerAction {
    fn from(value: KeyPressAction) -> Self {
        Self::KeyPressAction(value)
    }
}
impl ::std::convert::From<MoveParam> for ComputerAction {
    fn from(value: MoveParam) -> Self {
        Self::MoveParam(value)
    }
}
impl ::std::convert::From<ScreenshotParam> for ComputerAction {
    fn from(value: ScreenshotParam) -> Self {
        Self::ScreenshotParam(value)
    }
}
impl ::std::convert::From<ScrollParam> for ComputerAction {
    fn from(value: ScrollParam) -> Self {
        Self::ScrollParam(value)
    }
}
impl ::std::convert::From<TypeParam> for ComputerAction {
    fn from(value: TypeParam) -> Self {
        Self::TypeParam(value)
    }
}
impl ::std::convert::From<WaitParam> for ComputerAction {
    fn from(value: WaitParam) -> Self {
        Self::WaitParam(value)
    }
}
#[doc = "Flattened batched actions for `computer_use`. Each action includes an\n`type` discriminator and action-specific fields.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ComputerActionList(pub ::std::vec::Vec<ComputerAction>);
impl ::std::ops::Deref for ComputerActionList {
    type Target = ::std::vec::Vec<ComputerAction>;
    fn deref(&self) -> &::std::vec::Vec<ComputerAction> {
        &self.0
    }
}
impl ::std::convert::From<ComputerActionList> for ::std::vec::Vec<ComputerAction> {
    fn from(value: ComputerActionList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ComputerAction>> for ComputerActionList {
    fn from(value: ::std::vec::Vec<ComputerAction>) -> Self {
        Self(value)
    }
}
#[doc = "The output of a computer tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerCallOutputItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub acknowledged_safety_checks:
        ::std::option::Option<::std::vec::Vec<ComputerCallSafetyCheckParam>>,
    #[doc = "The ID of the computer tool call that produced the output."]
    pub call_id: ComputerCallOutputItemParamCallId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    pub output: ComputerScreenshotImage,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionCallItemStatus>,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`."]
    #[serde(rename = "type")]
    pub type_: ComputerCallOutputItemParamType,
}
#[doc = "The ID of the computer tool call that produced the output."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ComputerCallOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ComputerCallOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ComputerCallOutputItemParamCallId> for ::std::string::String {
    fn from(value: ComputerCallOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ComputerCallOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ComputerCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ComputerCallOutputItemParamCallId {
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
#[doc = "The type of the computer tool call output. Always `computer_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerCallOutputItemParamType {
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}
impl ::std::fmt::Display for ComputerCallOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerCallOutput => f.write_str("computer_call_output"),
        }
    }
}
impl ::std::str::FromStr for ComputerCallOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_call_output" => Ok(Self::ComputerCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ComputerCallOutputStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerCallOutputStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ComputerCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ComputerCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A pending safety check for the computer call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerCallSafetyCheckParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the pending safety check."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
}
#[doc = "`ComputerEnvironment`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerEnvironment {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "ubuntu")]
    Ubuntu,
    #[serde(rename = "browser")]
    Browser,
}
impl ::std::fmt::Display for ComputerEnvironment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Windows => f.write_str("windows"),
            Self::Mac => f.write_str("mac"),
            Self::Linux => f.write_str("linux"),
            Self::Ubuntu => f.write_str("ubuntu"),
            Self::Browser => f.write_str("browser"),
        }
    }
}
impl ::std::str::FromStr for ComputerEnvironment {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "windows" => Ok(Self::Windows),
            "mac" => Ok(Self::Mac),
            "linux" => Ok(Self::Linux),
            "ubuntu" => Ok(Self::Ubuntu),
            "browser" => Ok(Self::Browser),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A screenshot of a computer."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerScreenshotContent {
    #[doc = "The detail level of the screenshot image to be sent to the model. One of `high`, `low`, `auto`, or `original`. Defaults to `auto`."]
    pub detail: ImageDetail,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "Specifies the event type. For a computer screenshot, this property is always set to `computer_screenshot`."]
    #[serde(rename = "type")]
    pub type_: ComputerScreenshotContentType,
}
#[doc = "Specifies the event type. For a computer screenshot, this property is always set to `computer_screenshot`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerScreenshotContentType {
    #[serde(rename = "computer_screenshot")]
    ComputerScreenshot,
}
impl ::std::fmt::Display for ComputerScreenshotContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerScreenshot => f.write_str("computer_screenshot"),
        }
    }
}
impl ::std::str::FromStr for ComputerScreenshotContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_screenshot" => Ok(Self::ComputerScreenshot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerScreenshotContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerScreenshotContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A computer screenshot image used with the computer use tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerScreenshotImage {
    #[doc = "The identifier of an uploaded file that contains the screenshot."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the screenshot image."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
    #[serde(rename = "type")]
    pub type_: ComputerScreenshotImageType,
}
#[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerScreenshotImageType {
    #[serde(rename = "computer_screenshot")]
    ComputerScreenshot,
}
impl ::std::fmt::Display for ComputerScreenshotImageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerScreenshot => f.write_str("computer_screenshot"),
        }
    }
}
impl ::std::str::FromStr for ComputerScreenshotImageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_screenshot" => Ok(Self::ComputerScreenshot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerScreenshotImageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerScreenshotImageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that controls a virtual computer. Learn more about the [computer tool](https://developers.openai.com/api/docs/guides/tools-computer-use)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerTool {
    #[doc = "The type of the computer tool. Always `computer`."]
    #[serde(rename = "type")]
    pub type_: ComputerToolType,
}
#[doc = "A tool call to a computer use tool. See the\n[computer use guide](https://developers.openai.com/api/docs/guides/tools-computer-use) for more information.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerToolCall {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<ComputerAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub actions: ::std::option::Option<ComputerActionList>,
    #[doc = "An identifier used when responding to the tool call with output.\n"]
    pub call_id: ::std::string::String,
    #[doc = "The unique ID of the computer call."]
    pub id: ::std::string::String,
    #[doc = "The pending safety checks for the computer call.\n"]
    pub pending_safety_checks: ::std::vec::Vec<ComputerCallSafetyCheckParam>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: ComputerToolCallStatus,
    #[doc = "The type of the computer call. Always `computer_call`."]
    #[serde(rename = "type")]
    pub type_: ComputerToolCallType,
}
#[doc = "The output of a computer tool call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerToolCallOutput {
    #[doc = "The safety checks reported by the API that have been acknowledged by the\ndeveloper.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub acknowledged_safety_checks: ::std::vec::Vec<ComputerCallSafetyCheckParam>,
    #[doc = "The ID of the computer tool call that produced the output.\n"]
    pub call_id: ::std::string::String,
    #[doc = "The ID of the computer tool call output.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    pub output: ComputerScreenshotImage,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ComputerToolCallOutputStatus>,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: ComputerToolCallOutputType,
}
#[doc = "`ComputerToolCallOutputResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerToolCallOutputResource {
    #[doc = "The safety checks reported by the API that have been acknowledged by the\ndeveloper.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub acknowledged_safety_checks: ::std::vec::Vec<ComputerCallSafetyCheckParam>,
    #[doc = "The ID of the computer tool call that produced the output.\n"]
    pub call_id: ::std::string::String,
    #[doc = "The identifier of the actor that created the item.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the computer tool call output.\n"]
    pub id: ::std::string::String,
    pub output: ComputerScreenshotImage,
    pub status: ComputerToolCallOutputResourceStatus,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: ComputerToolCallOutputResourceType,
}
#[doc = "`ComputerToolCallOutputResourceStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallOutputResourceStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ComputerToolCallOutputResourceStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallOutputResourceStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallOutputResourceStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallOutputResourceStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallOutputResourceType {
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}
impl ::std::fmt::Display for ComputerToolCallOutputResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerCallOutput => f.write_str("computer_call_output"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallOutputResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_call_output" => Ok(Self::ComputerCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallOutputStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ComputerToolCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallOutputType {
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}
impl ::std::fmt::Display for ComputerToolCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerCallOutput => f.write_str("computer_call_output"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_call_output" => Ok(Self::ComputerCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ComputerToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the computer call. Always `computer_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolCallType {
    #[serde(rename = "computer_call")]
    ComputerCall,
}
impl ::std::fmt::Display for ComputerToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerCall => f.write_str("computer_call"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_call" => Ok(Self::ComputerCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the computer tool. Always `computer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerToolType {
    #[serde(rename = "computer")]
    Computer,
}
impl ::std::fmt::Display for ComputerToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Computer => f.write_str("computer"),
        }
    }
}
impl ::std::str::FromStr for ComputerToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer" => Ok(Self::Computer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that controls a virtual computer. Learn more about the [computer tool](https://developers.openai.com/api/docs/guides/tools-computer-use)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ComputerUsePreviewTool {
    #[doc = "The height of the computer display."]
    pub display_height: i64,
    #[doc = "The width of the computer display."]
    pub display_width: i64,
    #[doc = "The type of computer environment to control."]
    pub environment: ComputerEnvironment,
    #[doc = "The type of the computer use tool. Always `computer_use_preview`."]
    #[serde(rename = "type")]
    pub type_: ComputerUsePreviewToolType,
}
#[doc = "The type of the computer use tool. Always `computer_use_preview`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ComputerUsePreviewToolType {
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
}
impl ::std::fmt::Display for ComputerUsePreviewToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComputerUsePreview => f.write_str("computer_use_preview"),
        }
    }
}
impl ::std::str::FromStr for ComputerUsePreviewToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "computer_use_preview" => Ok(Self::ComputerUsePreview),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComputerUsePreviewToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComputerUsePreviewToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerAutoParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerAutoParam {
    #[doc = "An optional list of uploaded files to make available to your code."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_limit: ::std::option::Option<ContainerMemoryLimit>,
    #[doc = "Network access policy for the container."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_policy: ::std::option::Option<ContainerAutoParamNetworkPolicy>,
    #[doc = "An optional list of skills referenced by id or inline data."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<ContainerAutoParamSkillsItem>,
    #[doc = "Automatically creates a container for this request"]
    #[serde(rename = "type")]
    pub type_: ContainerAutoParamType,
}
#[doc = "Network access policy for the container."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ContainerAutoParamNetworkPolicy {
    DisabledParam(ContainerNetworkPolicyDisabledParam),
    AllowlistParam(ContainerNetworkPolicyAllowlistParam),
}
impl ::std::convert::From<ContainerNetworkPolicyDisabledParam> for ContainerAutoParamNetworkPolicy {
    fn from(value: ContainerNetworkPolicyDisabledParam) -> Self {
        Self::DisabledParam(value)
    }
}
impl ::std::convert::From<ContainerNetworkPolicyAllowlistParam>
    for ContainerAutoParamNetworkPolicy
{
    fn from(value: ContainerNetworkPolicyAllowlistParam) -> Self {
        Self::AllowlistParam(value)
    }
}
#[doc = "`ContainerAutoParamSkillsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ContainerAutoParamSkillsItem {
    SkillReferenceParam(SkillReferenceParam),
    InlineSkillParam(InlineSkillParam),
}
impl ::std::convert::From<SkillReferenceParam> for ContainerAutoParamSkillsItem {
    fn from(value: SkillReferenceParam) -> Self {
        Self::SkillReferenceParam(value)
    }
}
impl ::std::convert::From<InlineSkillParam> for ContainerAutoParamSkillsItem {
    fn from(value: InlineSkillParam) -> Self {
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
pub enum ContainerAutoParamType {
    #[serde(rename = "container_auto")]
    ContainerAuto,
}
impl ::std::fmt::Display for ContainerAutoParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerAuto => f.write_str("container_auto"),
        }
    }
}
impl ::std::str::FromStr for ContainerAutoParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_auto" => Ok(Self::ContainerAuto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A citation for a container file used to generate a model response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerFileCitationBody {
    #[doc = "The ID of the container file."]
    pub container_id: ::std::string::String,
    #[doc = "The index of the last character of the container file citation in the message."]
    pub end_index: i64,
    #[doc = "The ID of the file."]
    pub file_id: ::std::string::String,
    #[doc = "The filename of the container file cited."]
    pub filename: ::std::string::String,
    #[doc = "The index of the first character of the container file citation in the message."]
    pub start_index: i64,
    #[doc = "The type of the container file citation. Always `container_file_citation`."]
    #[serde(rename = "type")]
    pub type_: ContainerFileCitationBodyType,
}
#[doc = "The type of the container file citation. Always `container_file_citation`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerFileCitationBodyType {
    #[serde(rename = "container_file_citation")]
    ContainerFileCitation,
}
impl ::std::fmt::Display for ContainerFileCitationBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerFileCitation => f.write_str("container_file_citation"),
        }
    }
}
impl ::std::str::FromStr for ContainerFileCitationBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_file_citation" => Ok(Self::ContainerFileCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerFileCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerFileCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerMemoryLimit`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerMemoryLimit {
    #[serde(rename = "1g")]
    X1g,
    #[serde(rename = "4g")]
    X4g,
    #[serde(rename = "16g")]
    X16g,
    #[serde(rename = "64g")]
    X64g,
}
impl ::std::fmt::Display for ContainerMemoryLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1g => f.write_str("1g"),
            Self::X4g => f.write_str("4g"),
            Self::X16g => f.write_str("16g"),
            Self::X64g => f.write_str("64g"),
        }
    }
}
impl ::std::str::FromStr for ContainerMemoryLimit {
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
impl ::std::convert::TryFrom<&str> for ContainerMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerNetworkPolicyAllowlistParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerNetworkPolicyAllowlistParam {
    #[doc = "A list of allowed domains when type is `allowlist`."]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional domain-scoped secrets for allowlisted domains."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub domain_secrets: ::std::vec::Vec<ContainerNetworkPolicyDomainSecretParam>,
    #[doc = "Allow outbound network access only to specified domains. Always `allowlist`."]
    #[serde(rename = "type")]
    pub type_: ContainerNetworkPolicyAllowlistParamType,
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
pub enum ContainerNetworkPolicyAllowlistParamType {
    #[serde(rename = "allowlist")]
    Allowlist,
}
impl ::std::fmt::Display for ContainerNetworkPolicyAllowlistParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allowlist => f.write_str("allowlist"),
        }
    }
}
impl ::std::str::FromStr for ContainerNetworkPolicyAllowlistParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowlist" => Ok(Self::Allowlist),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerNetworkPolicyAllowlistParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerNetworkPolicyAllowlistParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerNetworkPolicyDisabledParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerNetworkPolicyDisabledParam {
    #[doc = "Disable outbound network access. Always `disabled`."]
    #[serde(rename = "type")]
    pub type_: ContainerNetworkPolicyDisabledParamType,
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
pub enum ContainerNetworkPolicyDisabledParamType {
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for ContainerNetworkPolicyDisabledParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for ContainerNetworkPolicyDisabledParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerNetworkPolicyDisabledParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerNetworkPolicyDisabledParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerNetworkPolicyDomainSecretParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerNetworkPolicyDomainSecretParam {
    #[doc = "The domain associated with the secret."]
    pub domain: ContainerNetworkPolicyDomainSecretParamDomain,
    #[doc = "The name of the secret to inject for the domain."]
    pub name: ContainerNetworkPolicyDomainSecretParamName,
    #[doc = "The secret value to inject for the domain."]
    pub value: ContainerNetworkPolicyDomainSecretParamValue,
}
#[doc = "The domain associated with the secret."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ContainerNetworkPolicyDomainSecretParamDomain(::std::string::String);
impl ::std::ops::Deref for ContainerNetworkPolicyDomainSecretParamDomain {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ContainerNetworkPolicyDomainSecretParamDomain> for ::std::string::String {
    fn from(value: ContainerNetworkPolicyDomainSecretParamDomain) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ContainerNetworkPolicyDomainSecretParamDomain {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContainerNetworkPolicyDomainSecretParamDomain {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ContainerNetworkPolicyDomainSecretParamDomain
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContainerNetworkPolicyDomainSecretParamDomain {
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
#[doc = "The name of the secret to inject for the domain."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ContainerNetworkPolicyDomainSecretParamName(::std::string::String);
impl ::std::ops::Deref for ContainerNetworkPolicyDomainSecretParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ContainerNetworkPolicyDomainSecretParamName> for ::std::string::String {
    fn from(value: ContainerNetworkPolicyDomainSecretParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ContainerNetworkPolicyDomainSecretParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContainerNetworkPolicyDomainSecretParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ContainerNetworkPolicyDomainSecretParamName
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContainerNetworkPolicyDomainSecretParamName {
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
#[doc = "The secret value to inject for the domain."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ContainerNetworkPolicyDomainSecretParamValue(::std::string::String);
impl ::std::ops::Deref for ContainerNetworkPolicyDomainSecretParamValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ContainerNetworkPolicyDomainSecretParamValue> for ::std::string::String {
    fn from(value: ContainerNetworkPolicyDomainSecretParamValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ContainerNetworkPolicyDomainSecretParamValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContainerNetworkPolicyDomainSecretParamValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ContainerNetworkPolicyDomainSecretParamValue
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContainerNetworkPolicyDomainSecretParamValue {
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
#[doc = "`ContainerReferenceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerReferenceParam {
    #[doc = "The ID of the referenced container."]
    pub container_id: ::std::string::String,
    #[doc = "References a container created with the /v1/containers endpoint"]
    #[serde(rename = "type")]
    pub type_: ContainerReferenceParamType,
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
pub enum ContainerReferenceParamType {
    #[serde(rename = "container_reference")]
    ContainerReference,
}
impl ::std::fmt::Display for ContainerReferenceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerReference => f.write_str("container_reference"),
        }
    }
}
impl ::std::str::FromStr for ContainerReferenceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_reference" => Ok(Self::ContainerReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a container created with /v1/containers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerReferenceResource {
    pub container_id: ::std::string::String,
    #[doc = "The environment type. Always `container_reference`."]
    #[serde(rename = "type")]
    pub type_: ContainerReferenceResourceType,
}
#[doc = "The environment type. Always `container_reference`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerReferenceResourceType {
    #[serde(rename = "container_reference")]
    ContainerReference,
}
impl ::std::fmt::Display for ContainerReferenceResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerReference => f.write_str("container_reference"),
        }
    }
}
impl ::std::str::FromStr for ContainerReferenceResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container_reference" => Ok(Self::ContainerReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerReferenceResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerReferenceResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContextManagementParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContextManagementParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compact_threshold: ::std::option::Option<i64>,
    #[doc = "The context management entry type. Currently only 'compaction' is supported."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "The conversation that this response belongs to. Items from this conversation are prepended to `input_items` for this response request.\nInput items and output items from this response are automatically added to this conversation after this response completes.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ConversationParam {
    Id(::std::string::String),
    Param2(ConversationParam2),
}
impl ::std::convert::From<ConversationParam2> for ConversationParam {
    fn from(value: ConversationParam2) -> Self {
        Self::Param2(value)
    }
}
#[doc = "The conversation that this response belongs to."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ConversationParam2 {
    #[doc = "The unique ID of the conversation."]
    pub id: ::std::string::String,
}
#[doc = "An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CoordParam {
    #[doc = "The x-coordinate."]
    pub x: i64,
    #[doc = "The y-coordinate."]
    pub y: i64,
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
#[doc = "`CreateResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub access_programs: ::std::option::Option<AccessProgramsParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context_management: ::std::option::Option<::std::vec::Vec<ContextManagementParam>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conversation: ::std::option::Option<ConversationParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<::std::vec::Vec<IncludeEnum>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<InputParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tool_calls: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Metadata>,
    #[doc = "Model ID used to generate the response, like `gpt-6-astra`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models)\nto browse and compare available models.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ModelIdsResponses>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ModerationParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_response_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_options: ::std::option::Option<ResponsePromptCacheOptionsParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention: ::std::option::Option<CreateResponsePromptCacheRetention>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<Reasoning>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<CreateResponseSafetyIdentifier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTierResponses>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream_options: ::std::option::Option<ResponseStreamOptions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<ResponseTextParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<ToolChoiceParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ToolsArray>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_p: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<CreateResponseTruncation>,
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
pub enum CreateResponsePromptCacheRetention {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for CreateResponsePromptCacheRetention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for CreateResponsePromptCacheRetention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateResponsePromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateResponsePromptCacheRetention {
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
pub struct CreateResponseSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for CreateResponseSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateResponseSafetyIdentifier> for ::std::string::String {
    fn from(value: CreateResponseSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateResponseSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateResponseSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateResponseSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateResponseSafetyIdentifier {
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
#[doc = "The truncation strategy to use for the model response.\n- `auto`: If the input to this Response exceeds\n  the model's context window size, the model will truncate the\n  response to fit the context window by dropping items from the beginning of the conversation.\n- `disabled` (default): If the input size will exceed the context window\n  size for a model, the request will fail with a 400 error.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateResponseTruncation {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for CreateResponseTruncation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for CreateResponseTruncation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateResponseTruncation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateResponseTruncation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A grammar defined by the user."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomGrammarFormatParam {
    #[doc = "The grammar definition."]
    pub definition: ::std::string::String,
    #[doc = "The syntax of the grammar definition. One of `lark` or `regex`."]
    pub syntax: GrammarSyntax1,
    #[doc = "Grammar format. Always `grammar`."]
    #[serde(rename = "type")]
    pub type_: CustomGrammarFormatParamType,
}
#[doc = "Grammar format. Always `grammar`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomGrammarFormatParamType {
    #[serde(rename = "grammar")]
    Grammar,
}
impl ::std::fmt::Display for CustomGrammarFormatParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Grammar => f.write_str("grammar"),
        }
    }
}
impl ::std::str::FromStr for CustomGrammarFormatParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "grammar" => Ok(Self::Grammar),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomGrammarFormatParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomGrammarFormatParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Unconstrained free-form text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomTextFormatParam {
    #[doc = "Unconstrained text format. Always `text`."]
    #[serde(rename = "type")]
    pub type_: CustomTextFormatParamType,
}
#[doc = "Unconstrained text format. Always `text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomTextFormatParamType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for CustomTextFormatParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for CustomTextFormatParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomTextFormatParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomTextFormatParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A call to a custom tool created by the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolCall {
    #[doc = "Whether the custom tool call runs asynchronously.\n"]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "An identifier used to map this custom tool call to a tool call output.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The unique ID of the custom tool call in the OpenAI platform.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The input for the custom tool call generated by the model.\n"]
    pub input: ::std::string::String,
    #[doc = "The name of the custom tool being called.\n"]
    pub name: ::std::string::String,
    #[doc = "The namespace of the custom tool being called.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the custom tool call. Always `custom_tool_call`.\n"]
    #[serde(rename = "type")]
    pub type_: CustomToolCallType,
}
#[doc = "The output of a custom tool call from your code, being sent back to the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolCallOutput {
    #[doc = "The call ID, used to map this custom tool call output to a custom tool call.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[doc = "The unique ID of the custom tool call output in the OpenAI platform.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The output from the custom tool call generated by your code.\nCan be a string or an list of output content.\n"]
    pub output: CustomToolCallOutputOutput,
    #[doc = "The type of the custom tool call output. Always `custom_tool_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: CustomToolCallOutputType,
}
#[doc = "The output from the custom tool call generated by your code.\nCan be a string or an list of output content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomToolCallOutputOutput {
    StringOutput(::std::string::String),
    OutputContentList(::std::vec::Vec<FunctionAndCustomToolCallOutput>),
}
impl ::std::convert::From<::std::vec::Vec<FunctionAndCustomToolCallOutput>>
    for CustomToolCallOutputOutput
{
    fn from(value: ::std::vec::Vec<FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(value)
    }
}
#[doc = "`CustomToolCallOutputResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolCallOutputResource {
    #[doc = "The call ID, used to map this custom tool call output to a custom tool call.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[doc = "The identifier of the actor that created the item.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[doc = "The output from the custom tool call generated by your code.\nCan be a string or an list of output content.\n"]
    pub output: CustomToolCallOutputResourceOutput,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionCallOutputStatusEnum,
    #[doc = "The type of the custom tool call output. Always `custom_tool_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: CustomToolCallOutputResourceType,
}
#[doc = "The output from the custom tool call generated by your code.\nCan be a string or an list of output content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomToolCallOutputResourceOutput {
    StringOutput(::std::string::String),
    OutputContentList(::std::vec::Vec<FunctionAndCustomToolCallOutput>),
}
impl ::std::convert::From<::std::vec::Vec<FunctionAndCustomToolCallOutput>>
    for CustomToolCallOutputResourceOutput
{
    fn from(value: ::std::vec::Vec<FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(value)
    }
}
#[doc = "The type of the custom tool call output. Always `custom_tool_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomToolCallOutputResourceType {
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput,
}
impl ::std::fmt::Display for CustomToolCallOutputResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CustomToolCallOutput => f.write_str("custom_tool_call_output"),
        }
    }
}
impl ::std::str::FromStr for CustomToolCallOutputResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom_tool_call_output" => Ok(Self::CustomToolCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the custom tool call output. Always `custom_tool_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomToolCallOutputType {
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput,
}
impl ::std::fmt::Display for CustomToolCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CustomToolCallOutput => f.write_str("custom_tool_call_output"),
        }
    }
}
impl ::std::str::FromStr for CustomToolCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom_tool_call_output" => Ok(Self::CustomToolCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CustomToolCallResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolCallResource {
    #[doc = "Whether the custom tool call runs asynchronously.\n"]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "An identifier used to map this custom tool call to a tool call output.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The identifier of the actor that created the item.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[doc = "The input for the custom tool call generated by the model.\n"]
    pub input: ::std::string::String,
    #[doc = "The name of the custom tool being called.\n"]
    pub name: ::std::string::String,
    #[doc = "The namespace of the custom tool being called.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionCallStatus,
    #[doc = "The type of the custom tool call. Always `custom_tool_call`.\n"]
    #[serde(rename = "type")]
    pub type_: CustomToolCallResourceType,
}
#[doc = "The type of the custom tool call. Always `custom_tool_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomToolCallResourceType {
    #[serde(rename = "custom_tool_call")]
    CustomToolCall,
}
impl ::std::fmt::Display for CustomToolCallResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CustomToolCall => f.write_str("custom_tool_call"),
        }
    }
}
impl ::std::str::FromStr for CustomToolCallResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom_tool_call" => Ok(Self::CustomToolCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolCallResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolCallResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the custom tool call. Always `custom_tool_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CustomToolCallType {
    #[serde(rename = "custom_tool_call")]
    CustomToolCall,
}
impl ::std::fmt::Display for CustomToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CustomToolCall => f.write_str("custom_tool_call"),
        }
    }
}
impl ::std::str::FromStr for CustomToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom_tool_call" => Ok(Self::CustomToolCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A custom tool that processes input using a specified format. Learn more about   [custom tools](https://developers.openai.com/api/docs/guides/function-calling#custom-tools)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CustomToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[doc = "Whether the tool response can be returned asynchronously versus immediately returned on next response creation."]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Whether this tool should be deferred and discovered via tool search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[doc = "Optional description of the custom tool, used to provide more context."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The input format for the custom tool. Default is unconstrained text."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<CustomToolParamFormat>,
    #[doc = "The name of the custom tool, used to identify it in tool calls."]
    pub name: ::std::string::String,
    #[doc = "The type of the custom tool. Always `custom`."]
    #[serde(rename = "type")]
    pub type_: CustomToolParamType,
}
#[doc = "The input format for the custom tool. Default is unconstrained text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomToolParamFormat {
    TextFormatParam(CustomTextFormatParam),
    GrammarFormatParam(CustomGrammarFormatParam),
}
impl ::std::convert::From<CustomTextFormatParam> for CustomToolParamFormat {
    fn from(value: CustomTextFormatParam) -> Self {
        Self::TextFormatParam(value)
    }
}
impl ::std::convert::From<CustomGrammarFormatParam> for CustomToolParamFormat {
    fn from(value: CustomGrammarFormatParam) -> Self {
        Self::GrammarFormatParam(value)
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
pub enum CustomToolParamType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for CustomToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for CustomToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CyberAccessProgramEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CyberAccessProgramEnum {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "daybreak_blue")]
    DaybreakBlue,
    #[serde(rename = "daybreak_red")]
    DaybreakRed,
}
impl ::std::fmt::Display for CyberAccessProgramEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::DaybreakBlue => f.write_str("daybreak_blue"),
            Self::DaybreakRed => f.write_str("daybreak_red"),
        }
    }
}
impl ::std::str::FromStr for CyberAccessProgramEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "daybreak_blue" => Ok(Self::DaybreakBlue),
            "daybreak_red" => Ok(Self::DaybreakRed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CyberAccessProgramEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CyberAccessProgramEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteResponseObject,
}
#[doc = "`DeleteResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeleteResponseObject {
    #[serde(rename = "response.deleted")]
    ResponseDeleted,
}
impl ::std::fmt::Display for DeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseDeleted => f.write_str("response.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.deleted" => Ok(Self::ResponseDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DetailEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DetailEnum {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for DetailEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Auto => f.write_str("auto"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for DetailEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "auto" => Ok(Self::Auto),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DetailEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DetailEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DirectToolCallCaller`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DirectToolCallCaller {
    #[serde(rename = "type")]
    pub type_: DirectToolCallCallerType,
}
#[doc = "`DirectToolCallCallerParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DirectToolCallCallerParam {
    #[doc = "The caller type. Always `direct`."]
    #[serde(rename = "type")]
    pub type_: DirectToolCallCallerParamType,
}
#[doc = "The caller type. Always `direct`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DirectToolCallCallerParamType {
    #[serde(rename = "direct")]
    Direct,
}
impl ::std::fmt::Display for DirectToolCallCallerParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Direct => f.write_str("direct"),
        }
    }
}
impl ::std::str::FromStr for DirectToolCallCallerParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "direct" => Ok(Self::Direct),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DirectToolCallCallerParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DirectToolCallCallerParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DirectToolCallCallerType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DirectToolCallCallerType {
    #[serde(rename = "direct")]
    Direct,
}
impl ::std::fmt::Display for DirectToolCallCallerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Direct => f.write_str("direct"),
        }
    }
}
impl ::std::str::FromStr for DirectToolCallCallerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "direct" => Ok(Self::Direct),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DirectToolCallCallerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DirectToolCallCallerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A double click action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DoubleClickAction {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Specifies the event type. For a double click action, this property is always set to `double_click`."]
    #[serde(rename = "type")]
    pub type_: DoubleClickActionType,
    #[doc = "The x-coordinate where the double click occurred."]
    pub x: i64,
    #[doc = "The y-coordinate where the double click occurred."]
    pub y: i64,
}
#[doc = "Specifies the event type. For a double click action, this property is always set to `double_click`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DoubleClickActionType {
    #[serde(rename = "double_click")]
    DoubleClick,
}
impl ::std::fmt::Display for DoubleClickActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DoubleClick => f.write_str("double_click"),
        }
    }
}
impl ::std::str::FromStr for DoubleClickActionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "double_click" => Ok(Self::DoubleClick),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DoubleClickActionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DoubleClickActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A drag action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DragParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "An array of coordinates representing the path of the drag action. Coordinates will appear as an array of objects, eg\n```\n[\n  { x: 100, y: 200 },\n  { x: 200, y: 300 }\n]\n```"]
    pub path: ::std::vec::Vec<CoordParam>,
    #[doc = "Specifies the event type. For a drag action, this property is always set to `drag`."]
    #[serde(rename = "type")]
    pub type_: DragParamType,
}
#[doc = "Specifies the event type. For a drag action, this property is always set to `drag`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DragParamType {
    #[serde(rename = "drag")]
    Drag,
}
impl ::std::fmt::Display for DragParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Drag => f.write_str("drag"),
        }
    }
}
impl ::std::str::FromStr for DragParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "drag" => Ok(Self::Drag),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DragParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DragParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EasyInputMessage {
    #[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
    pub content: EasyInputMessageContent,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<MessagePhase>,
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    pub role: EasyInputMessageRole,
    #[doc = "The type of the message input. Always `message`.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EasyInputMessageType>,
}
#[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
    TextInput(::std::string::String),
    InputMessageContentList(InputMessageContentList),
}
impl ::std::convert::From<InputMessageContentList> for EasyInputMessageContent {
    fn from(value: InputMessageContentList) -> Self {
        Self::InputMessageContentList(value)
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
pub enum EasyInputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for EasyInputMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::System => f.write_str("system"),
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for EasyInputMessageRole {
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
impl ::std::convert::TryFrom<&str> for EasyInputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EasyInputMessageRole {
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
pub enum EasyInputMessageType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for EasyInputMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for EasyInputMessageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EasyInputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EasyInputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EmptyModelParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct EmptyModelParam(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for EmptyModelParam {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<EmptyModelParam>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: EmptyModelParam) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for EmptyModelParam
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`Error`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Error {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub code: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub misalignment: ::std::option::Option<MisalignmentErrorDetailsResource>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub param: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
#[doc = "`ErrorResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ErrorResponse {
    pub error: Error,
}
#[doc = "A citation to a file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileCitationBody {
    #[doc = "The ID of the file."]
    pub file_id: ::std::string::String,
    #[doc = "The filename of the file cited."]
    pub filename: ::std::string::String,
    #[doc = "The index in the output text at which to insert the file citation."]
    pub index: i64,
    #[doc = "The type of the file citation. Always `file_citation`."]
    #[serde(rename = "type")]
    pub type_: FileCitationBodyType,
}
#[doc = "The type of the file citation. Always `file_citation`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileCitationBodyType {
    #[serde(rename = "file_citation")]
    FileCitation,
}
impl ::std::fmt::Display for FileCitationBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileCitation => f.write_str("file_citation"),
        }
    }
}
impl ::std::str::FromStr for FileCitationBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_citation" => Ok(Self::FileCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FileDetailEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileDetailEnum {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for FileDetailEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for FileDetailEnum {
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
impl ::std::convert::TryFrom<&str> for FileDetailEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileDetailEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FileInputDetail`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileInputDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for FileInputDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for FileInputDetail {
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
impl ::std::convert::TryFrom<&str> for FileInputDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileInputDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A path to a file.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FilePath {
    #[doc = "The ID of the file.\n"]
    pub file_id: ::std::string::String,
    #[doc = "The index of the file in the list of files.\n"]
    pub index: i64,
    #[doc = "The type of the file path. Always `file_path`.\n"]
    #[serde(rename = "type")]
    pub type_: FilePathType,
}
#[doc = "The type of the file path. Always `file_path`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FilePathType {
    #[serde(rename = "file_path")]
    FilePath,
}
impl ::std::fmt::Display for FilePathType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FilePath => f.write_str("file_path"),
        }
    }
}
impl ::std::str::FromStr for FilePathType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_path" => Ok(Self::FilePath),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FilePathType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FilePathType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://developers.openai.com/api/docs/guides/tools-file-search)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileSearchTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filters: ::std::option::Option<Filters>,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_num_results: ::std::option::Option<i64>,
    #[doc = "Ranking options for search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranking_options: ::std::option::Option<RankingOptions>,
    #[doc = "The type of the file search tool. Always `file_search`."]
    #[serde(rename = "type")]
    pub type_: FileSearchToolType,
    #[doc = "The IDs of the vector stores to search."]
    pub vector_store_ids: ::std::vec::Vec<::std::string::String>,
}
#[doc = "The results of a file search tool call. See the\n[file search guide](https://developers.openai.com/api/docs/guides/tools-file-search) for more information.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FileSearchToolCall {
    #[doc = "The unique ID of the file search tool call.\n"]
    pub id: ::std::string::String,
    #[doc = "The queries used to search for files.\n"]
    pub queries: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub results: ::std::option::Option<::std::vec::Vec<FileSearchToolCallResultsItem>>,
    #[doc = "The status of the file search tool call. One of `in_progress`,\n`searching`, `incomplete` or `failed`,\n"]
    pub status: FileSearchToolCallStatus,
    #[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
    #[serde(rename = "type")]
    pub type_: FileSearchToolCallType,
}
#[doc = "`FileSearchToolCallResultsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct FileSearchToolCallResultsItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<VectorStoreFileAttributes>,
    #[doc = "The unique ID of the file.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the file.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
    #[doc = "The relevance score of the file - a value between 0 and 1.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score: ::std::option::Option<f32>,
    #[doc = "The text that was retrieved from the file.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
#[doc = "The status of the file search tool call. One of `in_progress`,\n`searching`, `incomplete` or `failed`,\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileSearchToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for FileSearchToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Searching => f.write_str("searching"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for FileSearchToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "searching" => Ok(Self::Searching),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileSearchToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileSearchToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileSearchToolCallType {
    #[serde(rename = "file_search_call")]
    FileSearchCall,
}
impl ::std::fmt::Display for FileSearchToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearchCall => f.write_str("file_search_call"),
        }
    }
}
impl ::std::str::FromStr for FileSearchToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search_call" => Ok(Self::FileSearchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileSearchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileSearchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the file search tool. Always `file_search`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FileSearchToolType {
    #[serde(rename = "file_search")]
    FileSearch,
}
impl ::std::fmt::Display for FileSearchToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
        }
    }
}
impl ::std::str::FromStr for FileSearchToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileSearchToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileSearchToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Filters`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Filters {
    ComparisonFilter(ComparisonFilter),
    CompoundFilter(CompoundFilter),
}
impl ::std::convert::From<ComparisonFilter> for Filters {
    fn from(value: ComparisonFilter) -> Self {
        Self::ComparisonFilter(value)
    }
}
impl ::std::convert::From<CompoundFilter> for Filters {
    fn from(value: CompoundFilter) -> Self {
        Self::CompoundFilter(value)
    }
}
#[doc = "`FunctionAndCustomToolCallOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionAndCustomToolCallOutput {
    TextContent(InputTextContent),
    ImageContent(InputImageContent),
    FileContent(InputFileContent),
}
impl ::std::convert::From<InputTextContent> for FunctionAndCustomToolCallOutput {
    fn from(value: InputTextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<InputImageContent> for FunctionAndCustomToolCallOutput {
    fn from(value: InputImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<InputFileContent> for FunctionAndCustomToolCallOutput {
    fn from(value: InputFileContent) -> Self {
        Self::FileContent(value)
    }
}
#[doc = "`FunctionCallItemStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionCallItemStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionCallItemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionCallItemStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output of a function tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionCallOutputItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<FunctionCallOutputItemParamCallId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<FunctionCallOutputItemParamName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<FunctionCallOutputItemParamNamespace>,
    #[doc = "Text, image, or file output of the function tool call."]
    pub output: FunctionCallOutputItemParamOutput,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionCallItemStatus>,
    #[doc = "The type of the function tool call output. Always `function_call_output`."]
    #[serde(rename = "type")]
    pub type_: FunctionCallOutputItemParamType,
}
#[doc = "The unique ID of the function tool call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionCallOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for FunctionCallOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionCallOutputItemParamCallId> for ::std::string::String {
    fn from(value: FunctionCallOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionCallOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for FunctionCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionCallOutputItemParamCallId {
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
#[doc = "The name of the tool that produced the output."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionCallOutputItemParamName(::std::string::String);
impl ::std::ops::Deref for FunctionCallOutputItemParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionCallOutputItemParamName> for ::std::string::String {
    fn from(value: FunctionCallOutputItemParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionCallOutputItemParamName {
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
impl ::std::convert::TryFrom<&str> for FunctionCallOutputItemParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputItemParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionCallOutputItemParamName {
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
#[doc = "The namespace of the tool that produced the output."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionCallOutputItemParamNamespace(::std::string::String);
impl ::std::ops::Deref for FunctionCallOutputItemParamNamespace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionCallOutputItemParamNamespace> for ::std::string::String {
    fn from(value: FunctionCallOutputItemParamNamespace) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionCallOutputItemParamNamespace {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-zA-Z0-9_-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallOutputItemParamNamespace {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputItemParamNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionCallOutputItemParamNamespace {
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
#[doc = "Text, image, or file output of the function tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionCallOutputItemParamOutput {
    String(FunctionCallOutputItemParamOutputString),
    Array(::std::vec::Vec<FunctionCallOutputItemParamOutputArrayItem>),
}
impl ::std::convert::From<FunctionCallOutputItemParamOutputString>
    for FunctionCallOutputItemParamOutput
{
    fn from(value: FunctionCallOutputItemParamOutputString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<FunctionCallOutputItemParamOutputArrayItem>>
    for FunctionCallOutputItemParamOutput
{
    fn from(value: ::std::vec::Vec<FunctionCallOutputItemParamOutputArrayItem>) -> Self {
        Self::Array(value)
    }
}
#[doc = "A piece of message content, such as text, an image, or a file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionCallOutputItemParamOutputArrayItem {
    TextContentParam(InputTextContentParam),
    ImageContentParamAutoParam(InputImageContentParamAutoParam),
    FileContentParam(InputFileContentParam),
}
impl ::std::convert::From<InputTextContentParam> for FunctionCallOutputItemParamOutputArrayItem {
    fn from(value: InputTextContentParam) -> Self {
        Self::TextContentParam(value)
    }
}
impl ::std::convert::From<InputImageContentParamAutoParam>
    for FunctionCallOutputItemParamOutputArrayItem
{
    fn from(value: InputImageContentParamAutoParam) -> Self {
        Self::ImageContentParamAutoParam(value)
    }
}
impl ::std::convert::From<InputFileContentParam> for FunctionCallOutputItemParamOutputArrayItem {
    fn from(value: InputFileContentParam) -> Self {
        Self::FileContentParam(value)
    }
}
#[doc = "A JSON string of the output of the function tool call."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionCallOutputItemParamOutputString(::std::string::String);
impl ::std::ops::Deref for FunctionCallOutputItemParamOutputString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionCallOutputItemParamOutputString> for ::std::string::String {
    fn from(value: FunctionCallOutputItemParamOutputString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionCallOutputItemParamOutputString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallOutputItemParamOutputString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputItemParamOutputString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionCallOutputItemParamOutputString {
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
#[doc = "The type of the function tool call output. Always `function_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionCallOutputItemParamType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
impl ::std::fmt::Display for FunctionCallOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCallOutput => f.write_str("function_call_output"),
        }
    }
}
impl ::std::str::FromStr for FunctionCallOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call_output" => Ok(Self::FunctionCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionCallOutputStatusEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionCallOutputStatusEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionCallOutputStatusEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionCallOutputStatusEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallOutputStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallOutputStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Execute a shell command."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellAction {
    pub commands: ::std::vec::Vec<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_output_length: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub timeout_ms: ::std::option::Option<i64>,
}
#[doc = "Commands and limits describing how to run the shell tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellActionParam {
    #[doc = "Ordered shell commands for the execution environment to run."]
    pub commands: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_length: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<i64>,
}
#[doc = "A tool call that executes one or more shell commands in a managed environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCall {
    #[doc = "The shell commands and limits that describe how to run the tool call."]
    pub action: FunctionShellAction,
    #[doc = "The unique ID of the shell tool call generated by the model."]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The ID of the entity that created this tool call."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub environment: ::std::option::Option<FunctionShellCallEnvironment>,
    #[doc = "The unique ID of the shell tool call. Populated when this item is returned via API."]
    pub id: ::std::string::String,
    #[doc = "The status of the shell call. One of `in_progress`, `completed`, or `incomplete`."]
    pub status: FunctionShellCallStatus,
    #[doc = "The type of the item. Always `shell_call`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallType,
}
#[doc = "`FunctionShellCallEnvironment`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionShellCallEnvironment {
    LocalEnvironmentResource(LocalEnvironmentResource),
    ContainerReferenceResource(ContainerReferenceResource),
}
impl ::std::convert::From<LocalEnvironmentResource> for FunctionShellCallEnvironment {
    fn from(value: LocalEnvironmentResource) -> Self {
        Self::LocalEnvironmentResource(value)
    }
}
impl ::std::convert::From<ContainerReferenceResource> for FunctionShellCallEnvironment {
    fn from(value: ContainerReferenceResource) -> Self {
        Self::ContainerReferenceResource(value)
    }
}
#[doc = "A tool representing a request to execute one or more shell commands."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallItemParam {
    #[doc = "The shell commands and limits that describe how to run the tool call."]
    pub action: FunctionShellActionParam,
    #[doc = "The unique ID of the shell tool call generated by the model."]
    pub call_id: FunctionShellCallItemParamCallId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<FunctionShellCallItemParamEnvironment>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionShellCallItemStatus>,
    #[doc = "The type of the item. Always `shell_call`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallItemParamType,
}
#[doc = "The unique ID of the shell tool call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionShellCallItemParamCallId(::std::string::String);
impl ::std::ops::Deref for FunctionShellCallItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionShellCallItemParamCallId> for ::std::string::String {
    fn from(value: FunctionShellCallItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionShellCallItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for FunctionShellCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionShellCallItemParamCallId {
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
#[doc = "The environment to execute the shell commands in."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionShellCallItemParamEnvironment {
    LocalEnvironmentParam(LocalEnvironmentParam),
    ContainerReferenceParam(ContainerReferenceParam),
}
impl ::std::convert::From<LocalEnvironmentParam> for FunctionShellCallItemParamEnvironment {
    fn from(value: LocalEnvironmentParam) -> Self {
        Self::LocalEnvironmentParam(value)
    }
}
impl ::std::convert::From<ContainerReferenceParam> for FunctionShellCallItemParamEnvironment {
    fn from(value: ContainerReferenceParam) -> Self {
        Self::ContainerReferenceParam(value)
    }
}
#[doc = "The type of the item. Always `shell_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallItemParamType {
    #[serde(rename = "shell_call")]
    ShellCall,
}
impl ::std::fmt::Display for FunctionShellCallItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ShellCall => f.write_str("shell_call"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell_call" => Ok(Self::ShellCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Status values reported for shell tool calls."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallItemStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionShellCallItemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallItemStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output of a shell tool call that was emitted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutput {
    #[doc = "The unique ID of the shell tool call generated by the model."]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The identifier of the actor that created the item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the shell call output. Populated when this item is returned via API."]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_output_length: ::std::option::Option<i64>,
    #[doc = "An array of shell call output contents"]
    pub output: ::std::vec::Vec<FunctionShellCallOutputContent>,
    #[doc = "The status of the shell call output. One of `in_progress`, `completed`, or `incomplete`."]
    pub status: FunctionShellCallOutputStatusEnum,
    #[doc = "The type of the shell call output. Always `shell_call_output`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputType,
}
#[doc = "The content of a shell tool call output that was emitted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputContent {
    #[doc = "The identifier of the actor that created the item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "Represents either an exit outcome (with an exit code) or a timeout outcome for a shell call output chunk."]
    pub outcome: ShellCallOutcome,
    #[doc = "The standard error output that was captured."]
    pub stderr: ::std::string::String,
    #[doc = "The standard output that was captured."]
    pub stdout: ::std::string::String,
}
#[doc = "Captured stdout and stderr for a portion of a shell tool call output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputContentParam {
    #[doc = "The exit or timeout outcome associated with this shell call."]
    pub outcome: FunctionShellCallOutputOutcomeParam,
    #[doc = "Captured stderr output for the shell call."]
    pub stderr: FunctionShellCallOutputContentParamStderr,
    #[doc = "Captured stdout output for the shell call."]
    pub stdout: FunctionShellCallOutputContentParamStdout,
}
#[doc = "Captured stderr output for the shell call."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionShellCallOutputContentParamStderr(::std::string::String);
impl ::std::ops::Deref for FunctionShellCallOutputContentParamStderr {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionShellCallOutputContentParamStderr> for ::std::string::String {
    fn from(value: FunctionShellCallOutputContentParamStderr) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputContentParamStderr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputContentParamStderr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputContentParamStderr {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionShellCallOutputContentParamStderr {
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
#[doc = "Captured stdout output for the shell call."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionShellCallOutputContentParamStdout(::std::string::String);
impl ::std::ops::Deref for FunctionShellCallOutputContentParamStdout {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionShellCallOutputContentParamStdout> for ::std::string::String {
    fn from(value: FunctionShellCallOutputContentParamStdout) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputContentParamStdout {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputContentParamStdout {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputContentParamStdout {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionShellCallOutputContentParamStdout {
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
#[doc = "Indicates that the shell commands finished and returned an exit code."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputExitOutcome {
    #[doc = "Exit code from the shell process."]
    pub exit_code: i64,
    #[doc = "The outcome type. Always `exit`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputExitOutcomeType,
}
#[doc = "Indicates that the shell commands finished and returned an exit code."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputExitOutcomeParam {
    #[doc = "The exit code returned by the shell process."]
    pub exit_code: i64,
    #[doc = "The outcome type. Always `exit`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputExitOutcomeParamType,
}
#[doc = "The outcome type. Always `exit`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputExitOutcomeParamType {
    #[serde(rename = "exit")]
    Exit,
}
impl ::std::fmt::Display for FunctionShellCallOutputExitOutcomeParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exit => f.write_str("exit"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputExitOutcomeParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exit" => Ok(Self::Exit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputExitOutcomeParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FunctionShellCallOutputExitOutcomeParamType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The outcome type. Always `exit`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputExitOutcomeType {
    #[serde(rename = "exit")]
    Exit,
}
impl ::std::fmt::Display for FunctionShellCallOutputExitOutcomeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exit => f.write_str("exit"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputExitOutcomeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exit" => Ok(Self::Exit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputExitOutcomeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputExitOutcomeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The streamed output items emitted by a shell tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputItemParam {
    #[doc = "The unique ID of the shell tool call generated by the model."]
    pub call_id: FunctionShellCallOutputItemParamCallId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_length: ::std::option::Option<i64>,
    #[doc = "Captured chunks of stdout and stderr output, along with their associated outcomes."]
    pub output: ::std::vec::Vec<FunctionShellCallOutputContentParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionShellCallItemStatus>,
    #[doc = "The type of the item. Always `shell_call_output`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputItemParamType,
}
#[doc = "The unique ID of the shell tool call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionShellCallOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for FunctionShellCallOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionShellCallOutputItemParamCallId> for ::std::string::String {
    fn from(value: FunctionShellCallOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionShellCallOutputItemParamCallId {
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
#[doc = "The type of the item. Always `shell_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputItemParamType {
    #[serde(rename = "shell_call_output")]
    ShellCallOutput,
}
impl ::std::fmt::Display for FunctionShellCallOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ShellCallOutput => f.write_str("shell_call_output"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell_call_output" => Ok(Self::ShellCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The exit or timeout outcome associated with this shell call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionShellCallOutputOutcomeParam {
    TimeoutOutcomeParam(FunctionShellCallOutputTimeoutOutcomeParam),
    ExitOutcomeParam(FunctionShellCallOutputExitOutcomeParam),
}
impl ::std::convert::From<FunctionShellCallOutputTimeoutOutcomeParam>
    for FunctionShellCallOutputOutcomeParam
{
    fn from(value: FunctionShellCallOutputTimeoutOutcomeParam) -> Self {
        Self::TimeoutOutcomeParam(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutputExitOutcomeParam>
    for FunctionShellCallOutputOutcomeParam
{
    fn from(value: FunctionShellCallOutputExitOutcomeParam) -> Self {
        Self::ExitOutcomeParam(value)
    }
}
#[doc = "`FunctionShellCallOutputStatusEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputStatusEnum {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionShellCallOutputStatusEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputStatusEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Indicates that the shell call exceeded its configured time limit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputTimeoutOutcome {
    #[doc = "The outcome type. Always `timeout`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputTimeoutOutcomeType,
}
#[doc = "Indicates that the shell call exceeded its configured time limit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellCallOutputTimeoutOutcomeParam {
    #[doc = "The outcome type. Always `timeout`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellCallOutputTimeoutOutcomeParamType,
}
#[doc = "The outcome type. Always `timeout`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputTimeoutOutcomeParamType {
    #[serde(rename = "timeout")]
    Timeout,
}
impl ::std::fmt::Display for FunctionShellCallOutputTimeoutOutcomeParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Timeout => f.write_str("timeout"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputTimeoutOutcomeParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "timeout" => Ok(Self::Timeout),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputTimeoutOutcomeParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for FunctionShellCallOutputTimeoutOutcomeParamType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The outcome type. Always `timeout`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputTimeoutOutcomeType {
    #[serde(rename = "timeout")]
    Timeout,
}
impl ::std::fmt::Display for FunctionShellCallOutputTimeoutOutcomeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Timeout => f.write_str("timeout"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputTimeoutOutcomeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "timeout" => Ok(Self::Timeout),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputTimeoutOutcomeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputTimeoutOutcomeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the shell call output. Always `shell_call_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallOutputType {
    #[serde(rename = "shell_call_output")]
    ShellCallOutput,
}
impl ::std::fmt::Display for FunctionShellCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ShellCallOutput => f.write_str("shell_call_output"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell_call_output" => Ok(Self::ShellCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionShellCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionShellCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `shell_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellCallType {
    #[serde(rename = "shell_call")]
    ShellCall,
}
impl ::std::fmt::Display for FunctionShellCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ShellCall => f.write_str("shell_call"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell_call" => Ok(Self::ShellCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that allows the model to execute shell commands."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionShellToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<FunctionShellToolParamEnvironment>,
    #[doc = "The type of the shell tool. Always `shell`."]
    #[serde(rename = "type")]
    pub type_: FunctionShellToolParamType,
}
#[doc = "`FunctionShellToolParamEnvironment`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionShellToolParamEnvironment {
    ContainerAutoParam(ContainerAutoParam),
    LocalEnvironmentParam(LocalEnvironmentParam),
    ContainerReferenceParam(ContainerReferenceParam),
}
impl ::std::convert::From<ContainerAutoParam> for FunctionShellToolParamEnvironment {
    fn from(value: ContainerAutoParam) -> Self {
        Self::ContainerAutoParam(value)
    }
}
impl ::std::convert::From<LocalEnvironmentParam> for FunctionShellToolParamEnvironment {
    fn from(value: LocalEnvironmentParam) -> Self {
        Self::LocalEnvironmentParam(value)
    }
}
impl ::std::convert::From<ContainerReferenceParam> for FunctionShellToolParamEnvironment {
    fn from(value: ContainerReferenceParam) -> Self {
        Self::ContainerReferenceParam(value)
    }
}
#[doc = "The type of the shell tool. Always `shell`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionShellToolParamType {
    #[serde(rename = "shell")]
    Shell,
}
impl ::std::fmt::Display for FunctionShellToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
        }
    }
}
impl ::std::str::FromStr for FunctionShellToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionShellToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionShellToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Defines a function in your own code the model can choose to call. Learn more about [function calling](https://developers.openai.com/api/docs/guides/function-calling)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Whether this function is deferred and loaded via tool search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_schema:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub parameters:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub strict: ::std::option::Option<bool>,
    #[doc = "The type of the function tool. Always `function`."]
    #[serde(rename = "type")]
    pub type_: FunctionToolType,
}
#[doc = "A tool call to run a function. See the\n[function calling guide](https://developers.openai.com/api/docs/guides/function-calling) for more information.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionToolCall {
    #[doc = "A JSON string of the arguments to pass to the function.\n"]
    pub arguments: ::std::string::String,
    #[doc = "Whether the function tool call runs asynchronously.\n"]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The unique ID of the function tool call.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the function to run.\n"]
    pub name: ::std::string::String,
    #[doc = "The namespace of the function to run.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionToolCallStatus>,
    #[doc = "The type of the function tool call. Always `function_call`.\n"]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallType,
}
#[doc = "The output of a function tool call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionToolCallOutput {
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[doc = "The unique ID of the function tool call output. Populated when this item\nis returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the tool that produced the output.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The namespace of the tool that produced the output.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The output from the function call generated by your code.\nCan be a string or an list of output content.\n"]
    pub output: FunctionToolCallOutputOutput,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionToolCallOutputStatus>,
    #[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallOutputType,
}
#[doc = "The output from the function call generated by your code.\nCan be a string or an list of output content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionToolCallOutputOutput {
    StringOutput(::std::string::String),
    OutputContentList(::std::vec::Vec<FunctionAndCustomToolCallOutput>),
}
impl ::std::convert::From<::std::vec::Vec<FunctionAndCustomToolCallOutput>>
    for FunctionToolCallOutputOutput
{
    fn from(value: ::std::vec::Vec<FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(value)
    }
}
#[doc = "`FunctionToolCallOutputResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionToolCallOutputResource {
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCallerParam>,
    #[doc = "The identifier of the actor that created the item.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[doc = "The name of the tool that produced the output.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The namespace of the tool that produced the output.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The output from the function call generated by your code.\nCan be a string or an list of output content.\n"]
    pub output: FunctionToolCallOutputResourceOutput,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionCallOutputStatusEnum,
    #[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallOutputResourceType,
}
#[doc = "The output from the function call generated by your code.\nCan be a string or an list of output content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FunctionToolCallOutputResourceOutput {
    StringOutput(::std::string::String),
    OutputContentList(::std::vec::Vec<FunctionAndCustomToolCallOutput>),
}
impl ::std::convert::From<::std::vec::Vec<FunctionAndCustomToolCallOutput>>
    for FunctionToolCallOutputResourceOutput
{
    fn from(value: ::std::vec::Vec<FunctionAndCustomToolCallOutput>) -> Self {
        Self::OutputContentList(value)
    }
}
#[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallOutputResourceType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
impl ::std::fmt::Display for FunctionToolCallOutputResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCallOutput => f.write_str("function_call_output"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallOutputResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call_output" => Ok(Self::FunctionCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallOutputResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallOutputStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionToolCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallOutputType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
impl ::std::fmt::Display for FunctionToolCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCallOutput => f.write_str("function_call_output"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call_output" => Ok(Self::FunctionCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionToolCallResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionToolCallResource {
    #[doc = "A JSON string of the arguments to pass to the function.\n"]
    pub arguments: ::std::string::String,
    #[doc = "Whether the function tool call runs asynchronously.\n"]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    pub call_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub caller: ::std::option::Option<ToolCallCaller>,
    #[doc = "The identifier of the actor that created the item.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[doc = "The name of the function to run.\n"]
    pub name: ::std::string::String,
    #[doc = "The namespace of the function to run.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub namespace: ::std::option::Option<::std::string::String>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionCallStatus,
    #[doc = "The type of the function tool call. Always `function_call`.\n"]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallResourceType,
}
#[doc = "The type of the function tool call. Always `function_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallResourceType {
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for FunctionToolCallResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for FunctionToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the function tool call. Always `function_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolCallType {
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for FunctionToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FunctionToolParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FunctionToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[doc = "Whether the tool response can be returned asynchronously versus immediately returned on next response creation."]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Whether this function should be deferred and discovered via tool search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub name: FunctionToolParamName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_schema:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<EmptyModelParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
    #[serde(rename = "type")]
    pub type_: FunctionToolParamType,
}
#[doc = "`FunctionToolParamName`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionToolParamName(::std::string::String);
impl ::std::ops::Deref for FunctionToolParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionToolParamName> for ::std::string::String {
    fn from(value: FunctionToolParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FunctionToolParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-zA-Z0-9_-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionToolParamName {
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
#[doc = "`FunctionToolParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolParamType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for FunctionToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the function tool. Always `function`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FunctionToolType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for FunctionToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for FunctionToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FunctionToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GrammarSyntax1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GrammarSyntax1 {
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "regex")]
    Regex,
}
impl ::std::fmt::Display for GrammarSyntax1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Lark => f.write_str("lark"),
            Self::Regex => f.write_str("regex"),
        }
    }
}
impl ::std::str::FromStr for GrammarSyntax1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "lark" => Ok(Self::Lark),
            "regex" => Ok(Self::Regex),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GrammarSyntax1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GrammarSyntax1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HttpError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HttpError {
    pub code: i64,
    pub message: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: HttpErrorType,
}
#[doc = "`HttpErrorType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HttpErrorType {
    #[serde(rename = "http_error")]
    HttpError,
}
impl ::std::fmt::Display for HttpErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HttpError => f.write_str("http_error"),
        }
    }
}
impl ::std::str::FromStr for HttpErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "http_error" => Ok(Self::HttpError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HttpErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HttpErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HybridSearchOptions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct HybridSearchOptions {
    #[doc = "The weight of the embedding in the reciprocal ranking fusion."]
    pub embedding_weight: f64,
    #[doc = "The weight of the text in the reciprocal ranking fusion."]
    pub text_weight: f64,
}
#[doc = "`ImageBackground`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageDetail`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageDetail {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for ImageDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Auto => f.write_str("auto"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for ImageDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "auto" => Ok(Self::Auto),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageGenActionEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenActionEnum {
    #[serde(rename = "generate")]
    Generate,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenActionEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Generate => f.write_str("generate"),
            Self::Edit => f.write_str("edit"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenActionEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "generate" => Ok(Self::Generate),
            "edit" => Ok(Self::Edit),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenActionEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenActionEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that generates images using the GPT image models.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenTool {
    #[doc = "Whether to generate a new image or edit an existing image. Default: `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<ImageGenActionEnum>,
    #[doc = "Set the background of the generated image. One of `transparent`, `opaque`,\nor `auto`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including\ntheir `2026-09-08` snapshots, support `opaque` and `transparent`\nbackgrounds. Transparent backgrounds are available for supported GPT Image\nmodels. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in\npreview. When using `transparent`, set the output format to `png` or `webp`.\nDefault: `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<ImageGenToolBackground>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_fidelity: ::std::option::Option<InputFidelity>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_image_mask: ::std::option::Option<ImageGenToolInputImageMask>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ImageGenToolModel>,
    #[doc = "Moderation level for the generated image. Default: `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<ImageGenToolModeration>,
    #[doc = "Compression level for the output image. Default: 100.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_compression: ::std::option::Option<i64>,
    #[doc = "The output format of the generated image. One of `png`, `webp`, or\n`jpeg`. Default: `png`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<ImageGenToolOutputFormat>,
    #[doc = "Number of partial images to generate in streaming mode, from 0 (default value) to 3.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub partial_images: ::std::option::Option<i64>,
    #[doc = "The quality of the generated image. The GPT image models support `low`,\n`medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`,\nincluding their `2026-09-08` snapshots, also support `xhigh` and `max`.\nDefault: `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<ImageGenToolQuality>,
    #[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<ImageGenToolSize>,
    #[doc = "The type of the image generation tool. Always `image_generation`.\n"]
    #[serde(rename = "type")]
    pub type_: ImageGenToolType,
}
#[doc = "Set the background of the generated image. One of `transparent`, `opaque`,\nor `auto`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including\ntheir `2026-09-08` snapshots, support `opaque` and `transparent`\nbackgrounds. Transparent backgrounds are available for supported GPT Image\nmodels. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in\npreview. When using `transparent`, set the output format to `png` or `webp`.\nDefault: `auto`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenToolBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An image generation request made by the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenToolCall {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<ImageGenActionEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<ImageBackground>,
    #[doc = "The unique ID of the image generation call."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<ImageOutputFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<ImageGenToolCallQuality>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub result: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub revised_prompt: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<ImageGenToolCallSize>,
    #[doc = "The status of the image generation call."]
    pub status: ImageGenToolCallStatus,
    #[doc = "The type of the image generation call. Always `image_generation_call`."]
    #[serde(rename = "type")]
    pub type_: ImageGenToolCallType,
}
#[doc = "The quality of the image generated by the image generation tool call. One of `low`, `medium`, `high`, `xhigh`, `max`, or `auto`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolCallQuality {
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
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenToolCallQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolCallQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolCallQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolCallQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ImageGenToolCallSize {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ImageGenToolCallSizeSubtype1>,
}
#[doc = "`ImageGenToolCallSizeSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolCallSizeSubtype1 {
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1024x1536")]
    X1024x1536,
    #[serde(rename = "1536x1024")]
    X1536x1024,
}
impl ::std::fmt::Display for ImageGenToolCallSizeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1024x1536 => f.write_str("1024x1536"),
            Self::X1536x1024 => f.write_str("1536x1024"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolCallSizeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1024x1024" => Ok(Self::X1024x1024),
            "1024x1536" => Ok(Self::X1024x1536),
            "1536x1024" => Ok(Self::X1536x1024),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolCallSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolCallSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the image generation call."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "generating")]
    Generating,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for ImageGenToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Generating => f.write_str("generating"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "generating" => Ok(Self::Generating),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the image generation call. Always `image_generation_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolCallType {
    #[serde(rename = "image_generation_call")]
    ImageGenerationCall,
}
impl ::std::fmt::Display for ImageGenToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGenerationCall => f.write_str("image_generation_call"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation_call" => Ok(Self::ImageGenerationCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Optional mask for inpainting. Contains `image_url`\n(string, optional) and `file_id` (string, optional).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImageGenToolInputImageMask {
    #[doc = "File ID for the mask image.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "Base64-encoded mask image.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
}
#[doc = "`ImageGenToolModel`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ImageGenToolModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ImageGenToolModelSubtype1>,
}
#[doc = "The image generation model to use. One of `gpt-image-1`,\n`gpt-image-1-mini`, `gpt-image-1.5`, `gpt-image-2`,\n`gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,\n`gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`,\n`gpt-image-2.5-flare-2026-09-08`, or `chatgpt-image-latest`. Default:\n`gpt-image-1`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolModelSubtype1 {
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
    #[serde(rename = "gpt-image-1.5")]
    GptImage15,
    #[serde(rename = "gpt-image-2")]
    GptImage2,
    #[serde(rename = "gpt-image-2-2026-04-21")]
    GptImage220260421,
    #[serde(rename = "gpt-image-2.5-sunburst")]
    GptImage25Sunburst,
    #[serde(rename = "gpt-image-2.5-sunburst-2026-09-08")]
    GptImage25Sunburst20260908,
    #[serde(rename = "gpt-image-2.5-flare")]
    GptImage25Flare,
    #[serde(rename = "gpt-image-2.5-flare-2026-09-08")]
    GptImage25Flare20260908,
}
impl ::std::fmt::Display for ImageGenToolModelSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GptImage1 => f.write_str("gpt-image-1"),
            Self::GptImage1Mini => f.write_str("gpt-image-1-mini"),
            Self::GptImage15 => f.write_str("gpt-image-1.5"),
            Self::GptImage2 => f.write_str("gpt-image-2"),
            Self::GptImage220260421 => f.write_str("gpt-image-2-2026-04-21"),
            Self::GptImage25Sunburst => f.write_str("gpt-image-2.5-sunburst"),
            Self::GptImage25Sunburst20260908 => f.write_str("gpt-image-2.5-sunburst-2026-09-08"),
            Self::GptImage25Flare => f.write_str("gpt-image-2.5-flare"),
            Self::GptImage25Flare20260908 => f.write_str("gpt-image-2.5-flare-2026-09-08"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolModelSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gpt-image-1" => Ok(Self::GptImage1),
            "gpt-image-1-mini" => Ok(Self::GptImage1Mini),
            "gpt-image-1.5" => Ok(Self::GptImage15),
            "gpt-image-2" => Ok(Self::GptImage2),
            "gpt-image-2-2026-04-21" => Ok(Self::GptImage220260421),
            "gpt-image-2.5-sunburst" => Ok(Self::GptImage25Sunburst),
            "gpt-image-2.5-sunburst-2026-09-08" => Ok(Self::GptImage25Sunburst20260908),
            "gpt-image-2.5-flare" => Ok(Self::GptImage25Flare),
            "gpt-image-2.5-flare-2026-09-08" => Ok(Self::GptImage25Flare20260908),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Moderation level for the generated image. Default: `auto`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolModeration {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
}
impl ::std::fmt::Display for ImageGenToolModeration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolModeration {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolModeration {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolModeration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format of the generated image. One of `png`, `webp`, or\n`jpeg`. Default: `png`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageGenToolOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality of the generated image. The GPT image models support `low`,\n`medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`,\nincluding their `2026-09-08` snapshots, also support `xhigh` and `max`.\nDefault: `auto`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolQuality {
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
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenToolQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ImageGenToolSize {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ImageGenToolSizeSubtype1>,
}
#[doc = "`ImageGenToolSizeSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolSizeSubtype1 {
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1024x1536")]
    X1024x1536,
    #[serde(rename = "1536x1024")]
    X1536x1024,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenToolSizeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1024x1536 => f.write_str("1024x1536"),
            Self::X1536x1024 => f.write_str("1536x1024"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolSizeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1024x1024" => Ok(Self::X1024x1024),
            "1024x1536" => Ok(Self::X1024x1536),
            "1536x1024" => Ok(Self::X1536x1024),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the image generation tool. Always `image_generation`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageGenToolType {
    #[serde(rename = "image_generation")]
    ImageGeneration,
}
impl ::std::fmt::Display for ImageGenToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGeneration => f.write_str("image_generation"),
        }
    }
}
impl ::std::str::FromStr for ImageGenToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation" => Ok(Self::ImageGeneration),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageOutputFormat`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Specify additional output data to include in the model response. Currently supported values are:\n- `web_search_call.results`: Include the search results of the web search tool call.\n- `web_search_call.action.sources`: Include the sources of the web search tool call.\n- `code_interpreter_call.outputs`: Includes the outputs of python code execution in code interpreter tool call items.\n- `computer_call_output.output.image_url`: Include image urls from the computer call output.\n- `file_search_call.results`: Include the search results of the file search tool call.\n- `message.input_image.image_url`: Include image urls from the input message.\n- `message.output_text.logprobs`: Include logprobs with assistant messages.\n- `reasoning.encrypted_content`: Includes an encrypted version of reasoning tokens in reasoning item outputs. This enables reasoning items to be used in multi-turn conversations when using the Responses API statelessly (like when the `store` parameter is set to `false`, or when an organization is enrolled in the zero data retention program)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum IncludeEnum {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "web_search_call.results")]
    WebSearchCallResults,
    #[serde(rename = "web_search_call.action.sources")]
    WebSearchCallActionSources,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}
impl ::std::fmt::Display for IncludeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearchCallResults => f.write_str("file_search_call.results"),
            Self::WebSearchCallResults => f.write_str("web_search_call.results"),
            Self::WebSearchCallActionSources => f.write_str("web_search_call.action.sources"),
            Self::MessageInputImageImageUrl => f.write_str("message.input_image.image_url"),
            Self::ComputerCallOutputOutputImageUrl => {
                f.write_str("computer_call_output.output.image_url")
            }
            Self::CodeInterpreterCallOutputs => f.write_str("code_interpreter_call.outputs"),
            Self::ReasoningEncryptedContent => f.write_str("reasoning.encrypted_content"),
            Self::MessageOutputTextLogprobs => f.write_str("message.output_text.logprobs"),
        }
    }
}
impl ::std::str::FromStr for IncludeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search_call.results" => Ok(Self::FileSearchCallResults),
            "web_search_call.results" => Ok(Self::WebSearchCallResults),
            "web_search_call.action.sources" => Ok(Self::WebSearchCallActionSources),
            "message.input_image.image_url" => Ok(Self::MessageInputImageImageUrl),
            "computer_call_output.output.image_url" => Ok(Self::ComputerCallOutputOutputImageUrl),
            "code_interpreter_call.outputs" => Ok(Self::CodeInterpreterCallOutputs),
            "reasoning.encrypted_content" => Ok(Self::ReasoningEncryptedContent),
            "message.output_text.logprobs" => Ok(Self::MessageOutputTextLogprobs),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IncludeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IncludeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InlineSkillParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InlineSkillParam {
    #[doc = "The description of the skill."]
    pub description: ::std::string::String,
    #[doc = "The name of the skill."]
    pub name: ::std::string::String,
    #[doc = "Inline skill payload"]
    pub source: InlineSkillSourceParam,
    #[doc = "Defines an inline skill for this request."]
    #[serde(rename = "type")]
    pub type_: InlineSkillParamType,
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
pub enum InlineSkillParamType {
    #[serde(rename = "inline")]
    Inline,
}
impl ::std::fmt::Display for InlineSkillParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inline => f.write_str("inline"),
        }
    }
}
impl ::std::str::FromStr for InlineSkillParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InlineSkillParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InlineSkillParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Inline skill payload"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InlineSkillSourceParam {
    #[doc = "Base64-encoded skill zip bundle."]
    pub data: InlineSkillSourceParamData,
    #[doc = "The media type of the inline skill payload. Must be `application/zip`."]
    pub media_type: InlineSkillSourceParamMediaType,
    #[doc = "The type of the inline skill source. Must be `base64`."]
    #[serde(rename = "type")]
    pub type_: InlineSkillSourceParamType,
}
#[doc = "Base64-encoded skill zip bundle."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InlineSkillSourceParamData(::std::string::String);
impl ::std::ops::Deref for InlineSkillSourceParamData {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InlineSkillSourceParamData> for ::std::string::String {
    fn from(value: InlineSkillSourceParamData) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InlineSkillSourceParamData {
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
impl ::std::convert::TryFrom<&str> for InlineSkillSourceParamData {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InlineSkillSourceParamData {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InlineSkillSourceParamData {
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
pub enum InlineSkillSourceParamMediaType {
    #[serde(rename = "application/zip")]
    ApplicationZip,
}
impl ::std::fmt::Display for InlineSkillSourceParamMediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplicationZip => f.write_str("application/zip"),
        }
    }
}
impl ::std::str::FromStr for InlineSkillSourceParamMediaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "application/zip" => Ok(Self::ApplicationZip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InlineSkillSourceParamMediaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InlineSkillSourceParamMediaType {
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
pub enum InlineSkillSourceParamType {
    #[serde(rename = "base64")]
    Base64,
}
impl ::std::fmt::Display for InlineSkillSourceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Base64 => f.write_str("base64"),
        }
    }
}
impl ::std::str::FromStr for InlineSkillSourceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "base64" => Ok(Self::Base64),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InlineSkillSourceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InlineSkillSourceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InputContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum InputContent {
    TextContent(InputTextContent),
    ImageContent(InputImageContent),
    FileContent(InputFileContent),
}
impl ::std::convert::From<InputTextContent> for InputContent {
    fn from(value: InputTextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<InputImageContent> for InputContent {
    fn from(value: InputImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl ::std::convert::From<InputFileContent> for InputContent {
    fn from(value: InputFileContent) -> Self {
        Self::FileContent(value)
    }
}
#[doc = "Control how much effort the model will exert to match the style and features, especially facial features, of input images. Supports `high` and `low` on `gpt-image-1` and `gpt-image-1.5`; `gpt-image-1-mini` supports only `low`. For `gpt-image-2`, omit this parameter. Defaults to `low` on supported models."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputFidelity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}
impl ::std::fmt::Display for InputFidelity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::High => f.write_str("high"),
            Self::Low => f.write_str("low"),
        }
    }
}
impl ::std::str::FromStr for InputFidelity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A file input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputFileContent {
    #[doc = "The detail level of the file to be sent to the model. Use `auto` to let the system select the detail level; for GPT-5.6 and later models, `auto` uses high-quality rendering, which may increase input token usage. Use `low` for lower-cost rendering, or `high` to render the file at higher quality. Defaults to `auto`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<FileInputDetail>,
    #[doc = "The content of the file to be sent to the model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_data: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the file to be sent to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_url: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the file to be sent to the model."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The type of the input item. Always `input_file`."]
    #[serde(rename = "type")]
    pub type_: InputFileContentType,
}
#[doc = "A file input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputFileContentParam {
    #[doc = "The detail level of the file to be sent to the model. Use `auto` to let the system select the detail level; for GPT-5.6 and later models, `auto` uses high-quality rendering, which may increase input token usage. Use `low` for lower-cost rendering, or `high` to render the file at higher quality. Defaults to `auto`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<FileDetailEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_data: ::std::option::Option<InputFileContentParamFileData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The type of the input item. Always `input_file`."]
    #[serde(rename = "type")]
    pub type_: InputFileContentParamType,
}
#[doc = "The base64-encoded data of the file to be sent to the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InputFileContentParamFileData(::std::string::String);
impl ::std::ops::Deref for InputFileContentParamFileData {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InputFileContentParamFileData> for ::std::string::String {
    fn from(value: InputFileContentParamFileData) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InputFileContentParamFileData {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 73400320usize {
            return Err("longer than 73400320 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InputFileContentParamFileData {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFileContentParamFileData {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InputFileContentParamFileData {
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
#[doc = "The type of the input item. Always `input_file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputFileContentParamType {
    #[serde(rename = "input_file")]
    InputFile,
}
impl ::std::fmt::Display for InputFileContentParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputFile => f.write_str("input_file"),
        }
    }
}
impl ::std::str::FromStr for InputFileContentParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_file" => Ok(Self::InputFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputFileContentParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFileContentParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the input item. Always `input_file`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputFileContentType {
    #[serde(rename = "input_file")]
    InputFile,
}
impl ::std::fmt::Display for InputFileContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputFile => f.write_str("input_file"),
        }
    }
}
impl ::std::str::FromStr for InputFileContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_file" => Ok(Self::InputFile),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An image input to the model. Learn about [image inputs](https://developers.openai.com/api/docs/guides/images-vision)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputImageContent {
    #[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, `auto`, or `original`. Defaults to `auto`."]
    pub detail: ImageDetail,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointConfig>,
    #[doc = "The type of the input item. Always `input_image`."]
    #[serde(rename = "type")]
    pub type_: InputImageContentType,
}
#[doc = "An image input to the model. Learn about [image inputs](https://developers.openai.com/api/docs/guides/images-vision)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputImageContentParamAutoParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<DetailEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<InputImageContentParamAutoParamImageUrl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The type of the input item. Always `input_image`."]
    #[serde(rename = "type")]
    pub type_: InputImageContentParamAutoParamType,
}
#[doc = "The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InputImageContentParamAutoParamImageUrl(::std::string::String);
impl ::std::ops::Deref for InputImageContentParamAutoParamImageUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InputImageContentParamAutoParamImageUrl> for ::std::string::String {
    fn from(value: InputImageContentParamAutoParamImageUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InputImageContentParamAutoParamImageUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 20971520usize {
            return Err("longer than 20971520 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InputImageContentParamAutoParamImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputImageContentParamAutoParamImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InputImageContentParamAutoParamImageUrl {
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
#[doc = "The type of the input item. Always `input_image`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputImageContentParamAutoParamType {
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for InputImageContentParamAutoParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for InputImageContentParamAutoParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputImageContentParamAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputImageContentParamAutoParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the input item. Always `input_image`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputImageContentType {
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for InputImageContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for InputImageContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputImageContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputImageContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InputItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum InputItem {
    EasyInputMessage(EasyInputMessage),
    Item(Item),
    CompactionTriggerItemParam(CompactionTriggerItemParam),
    ItemReferenceParam(ItemReferenceParam),
    ProgramItemParam(ProgramItemParam),
    ProgramOutputItemParam(ProgramOutputItemParam),
}
impl ::std::convert::From<EasyInputMessage> for InputItem {
    fn from(value: EasyInputMessage) -> Self {
        Self::EasyInputMessage(value)
    }
}
impl ::std::convert::From<Item> for InputItem {
    fn from(value: Item) -> Self {
        Self::Item(value)
    }
}
impl ::std::convert::From<CompactionTriggerItemParam> for InputItem {
    fn from(value: CompactionTriggerItemParam) -> Self {
        Self::CompactionTriggerItemParam(value)
    }
}
impl ::std::convert::From<ItemReferenceParam> for InputItem {
    fn from(value: ItemReferenceParam) -> Self {
        Self::ItemReferenceParam(value)
    }
}
impl ::std::convert::From<ProgramItemParam> for InputItem {
    fn from(value: ProgramItemParam) -> Self {
        Self::ProgramItemParam(value)
    }
}
impl ::std::convert::From<ProgramOutputItemParam> for InputItem {
    fn from(value: ProgramOutputItemParam) -> Self {
        Self::ProgramOutputItemParam(value)
    }
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputMessage {
    pub content: InputMessageContentList,
    #[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
    pub role: InputMessageRole,
    #[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<InputMessageStatus>,
    #[doc = "The type of the message input. Always set to `message`.\n"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<InputMessageType>,
}
#[doc = "A list of one or many input items to the model, containing different content \ntypes.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct InputMessageContentList(pub ::std::vec::Vec<InputContent>);
impl ::std::ops::Deref for InputMessageContentList {
    type Target = ::std::vec::Vec<InputContent>;
    fn deref(&self) -> &::std::vec::Vec<InputContent> {
        &self.0
    }
}
impl ::std::convert::From<InputMessageContentList> for ::std::vec::Vec<InputContent> {
    fn from(value: InputMessageContentList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<InputContent>> for InputMessageContentList {
    fn from(value: ::std::vec::Vec<InputContent>) -> Self {
        Self(value)
    }
}
#[doc = "`InputMessageResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputMessageResource {
    pub content: InputMessageContentList,
    #[doc = "The unique ID of the message input.\n"]
    pub id: ::std::string::String,
    #[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
    pub role: InputMessageResourceRole,
    #[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<InputMessageResourceStatus>,
    #[doc = "The type of the message input. Always set to `message`.\n"]
    #[serde(rename = "type")]
    pub type_: InputMessageResourceType,
}
#[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageResourceRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for InputMessageResourceRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::System => f.write_str("system"),
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for InputMessageResourceRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "system" => Ok(Self::System),
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageResourceRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageResourceRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageResourceStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for InputMessageResourceStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for InputMessageResourceStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageResourceStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageResourceStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the message input. Always set to `message`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageResourceType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for InputMessageResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for InputMessageResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
impl ::std::fmt::Display for InputMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::System => f.write_str("system"),
            Self::Developer => f.write_str("developer"),
        }
    }
}
impl ::std::str::FromStr for InputMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "system" => Ok(Self::System),
            "developer" => Ok(Self::Developer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for InputMessageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for InputMessageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the message input. Always set to `message`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InputMessageType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for InputMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for InputMessageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Text, image, or file inputs to the model, used to generate a response.\n\nLearn more:\n- [Text inputs and outputs](https://developers.openai.com/api/docs/guides/text)\n- [Image inputs](https://developers.openai.com/api/docs/guides/images-vision)\n- [File inputs](https://developers.openai.com/api/docs/guides/file-inputs)\n- [Conversation state](https://developers.openai.com/api/docs/guides/conversation-state)\n- [Function calling](https://developers.openai.com/api/docs/guides/function-calling)\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum InputParam {
    TextInput(::std::string::String),
    InputItemList(::std::vec::Vec<InputItem>),
}
impl ::std::convert::From<::std::vec::Vec<InputItem>> for InputParam {
    fn from(value: ::std::vec::Vec<InputItem>) -> Self {
        Self::InputItemList(value)
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
#[doc = "A text input to the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InputTextContentParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_breakpoint: ::std::option::Option<PromptCacheBreakpointParam>,
    #[doc = "The text input to the model."]
    pub text: InputTextContentParamText,
    #[doc = "The type of the input item. Always `input_text`."]
    #[serde(rename = "type")]
    pub type_: InputTextContentParamType,
}
#[doc = "The text input to the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InputTextContentParamText(::std::string::String);
impl ::std::ops::Deref for InputTextContentParamText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InputTextContentParamText> for ::std::string::String {
    fn from(value: InputTextContentParamText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InputTextContentParamText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InputTextContentParamText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputTextContentParamText {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InputTextContentParamText {
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
pub enum InputTextContentParamType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for InputTextContentParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for InputTextContentParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputTextContentParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputTextContentParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "Content item used to generate a response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Item {
    InputMessage(InputMessage),
    OutputMessage(OutputMessage),
    FileSearchToolCall(FileSearchToolCall),
    ComputerToolCall(ComputerToolCall),
    ComputerCallOutputItemParam(ComputerCallOutputItemParam),
    WebSearchToolCall(WebSearchToolCall),
    FunctionToolCall(FunctionToolCall),
    FunctionCallOutputItemParam(FunctionCallOutputItemParam),
    ToolSearchCallItemParam(ToolSearchCallItemParam),
    ToolSearchOutputItemParam(ToolSearchOutputItemParam),
    AdditionalToolsItemParam(AdditionalToolsItemParam),
    ResponseConfigurationUpdateItemParam(ResponseConfigurationUpdateItemParam),
    ReasoningItem(ReasoningItem),
    CompactionSummaryItemParam(CompactionSummaryItemParam),
    ImageGenToolCall(ImageGenToolCall),
    CodeInterpreterToolCall(CodeInterpreterToolCall),
    LocalShellToolCall(LocalShellToolCall),
    LocalShellToolCallOutput(LocalShellToolCallOutput),
    FunctionShellCallItemParam(FunctionShellCallItemParam),
    FunctionShellCallOutputItemParam(FunctionShellCallOutputItemParam),
    ApplyPatchToolCallItemParam(ApplyPatchToolCallItemParam),
    ApplyPatchToolCallOutputItemParam(ApplyPatchToolCallOutputItemParam),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponse(McpApprovalResponse),
    McpToolCall(McpToolCall),
    CustomToolCallOutput(CustomToolCallOutput),
    CustomToolCall(CustomToolCall),
}
impl ::std::convert::From<InputMessage> for Item {
    fn from(value: InputMessage) -> Self {
        Self::InputMessage(value)
    }
}
impl ::std::convert::From<OutputMessage> for Item {
    fn from(value: OutputMessage) -> Self {
        Self::OutputMessage(value)
    }
}
impl ::std::convert::From<FileSearchToolCall> for Item {
    fn from(value: FileSearchToolCall) -> Self {
        Self::FileSearchToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCall> for Item {
    fn from(value: ComputerToolCall) -> Self {
        Self::ComputerToolCall(value)
    }
}
impl ::std::convert::From<ComputerCallOutputItemParam> for Item {
    fn from(value: ComputerCallOutputItemParam) -> Self {
        Self::ComputerCallOutputItemParam(value)
    }
}
impl ::std::convert::From<WebSearchToolCall> for Item {
    fn from(value: WebSearchToolCall) -> Self {
        Self::WebSearchToolCall(value)
    }
}
impl ::std::convert::From<FunctionToolCall> for Item {
    fn from(value: FunctionToolCall) -> Self {
        Self::FunctionToolCall(value)
    }
}
impl ::std::convert::From<FunctionCallOutputItemParam> for Item {
    fn from(value: FunctionCallOutputItemParam) -> Self {
        Self::FunctionCallOutputItemParam(value)
    }
}
impl ::std::convert::From<ToolSearchCallItemParam> for Item {
    fn from(value: ToolSearchCallItemParam) -> Self {
        Self::ToolSearchCallItemParam(value)
    }
}
impl ::std::convert::From<ToolSearchOutputItemParam> for Item {
    fn from(value: ToolSearchOutputItemParam) -> Self {
        Self::ToolSearchOutputItemParam(value)
    }
}
impl ::std::convert::From<AdditionalToolsItemParam> for Item {
    fn from(value: AdditionalToolsItemParam) -> Self {
        Self::AdditionalToolsItemParam(value)
    }
}
impl ::std::convert::From<ResponseConfigurationUpdateItemParam> for Item {
    fn from(value: ResponseConfigurationUpdateItemParam) -> Self {
        Self::ResponseConfigurationUpdateItemParam(value)
    }
}
impl ::std::convert::From<ReasoningItem> for Item {
    fn from(value: ReasoningItem) -> Self {
        Self::ReasoningItem(value)
    }
}
impl ::std::convert::From<CompactionSummaryItemParam> for Item {
    fn from(value: CompactionSummaryItemParam) -> Self {
        Self::CompactionSummaryItemParam(value)
    }
}
impl ::std::convert::From<ImageGenToolCall> for Item {
    fn from(value: ImageGenToolCall) -> Self {
        Self::ImageGenToolCall(value)
    }
}
impl ::std::convert::From<CodeInterpreterToolCall> for Item {
    fn from(value: CodeInterpreterToolCall) -> Self {
        Self::CodeInterpreterToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCall> for Item {
    fn from(value: LocalShellToolCall) -> Self {
        Self::LocalShellToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCallOutput> for Item {
    fn from(value: LocalShellToolCallOutput) -> Self {
        Self::LocalShellToolCallOutput(value)
    }
}
impl ::std::convert::From<FunctionShellCallItemParam> for Item {
    fn from(value: FunctionShellCallItemParam) -> Self {
        Self::FunctionShellCallItemParam(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutputItemParam> for Item {
    fn from(value: FunctionShellCallOutputItemParam) -> Self {
        Self::FunctionShellCallOutputItemParam(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCallItemParam> for Item {
    fn from(value: ApplyPatchToolCallItemParam) -> Self {
        Self::ApplyPatchToolCallItemParam(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutputItemParam> for Item {
    fn from(value: ApplyPatchToolCallOutputItemParam) -> Self {
        Self::ApplyPatchToolCallOutputItemParam(value)
    }
}
impl ::std::convert::From<McpListTools> for Item {
    fn from(value: McpListTools) -> Self {
        Self::McpListTools(value)
    }
}
impl ::std::convert::From<McpApprovalRequest> for Item {
    fn from(value: McpApprovalRequest) -> Self {
        Self::McpApprovalRequest(value)
    }
}
impl ::std::convert::From<McpApprovalResponse> for Item {
    fn from(value: McpApprovalResponse) -> Self {
        Self::McpApprovalResponse(value)
    }
}
impl ::std::convert::From<McpToolCall> for Item {
    fn from(value: McpToolCall) -> Self {
        Self::McpToolCall(value)
    }
}
impl ::std::convert::From<CustomToolCallOutput> for Item {
    fn from(value: CustomToolCallOutput) -> Self {
        Self::CustomToolCallOutput(value)
    }
}
impl ::std::convert::From<CustomToolCall> for Item {
    fn from(value: CustomToolCall) -> Self {
        Self::CustomToolCall(value)
    }
}
#[doc = "An item representing a message, tool call, tool output, reasoning, or other response element."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ItemField {
    Message(Message),
    Program(Program),
    ProgramOutput(ProgramOutput),
    FunctionToolCall(FunctionToolCall),
    ToolSearchCall(ToolSearchCall),
    ToolSearchOutput(ToolSearchOutput),
    AdditionalTools(AdditionalTools),
    FunctionToolCallOutput(FunctionToolCallOutput),
    FileSearchToolCall(FileSearchToolCall),
    WebSearchToolCall(WebSearchToolCall),
    ImageGenToolCall(ImageGenToolCall),
    ComputerToolCall(ComputerToolCall),
    ComputerToolCallOutputResource(ComputerToolCallOutputResource),
    ReasoningItem(ReasoningItem),
    CompactionBody(CompactionBody),
    CodeInterpreterToolCall(CodeInterpreterToolCall),
    LocalShellToolCall(LocalShellToolCall),
    LocalShellToolCallOutput(LocalShellToolCallOutput),
    FunctionShellCall(FunctionShellCall),
    FunctionShellCallOutput(FunctionShellCallOutput),
    ApplyPatchToolCall(ApplyPatchToolCall),
    ApplyPatchToolCallOutput(ApplyPatchToolCallOutput),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponseResource(McpApprovalResponseResource),
    McpToolCall(McpToolCall),
    CustomToolCall(CustomToolCall),
    CustomToolCallOutput(CustomToolCallOutput),
}
impl ::std::convert::From<Message> for ItemField {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}
impl ::std::convert::From<Program> for ItemField {
    fn from(value: Program) -> Self {
        Self::Program(value)
    }
}
impl ::std::convert::From<ProgramOutput> for ItemField {
    fn from(value: ProgramOutput) -> Self {
        Self::ProgramOutput(value)
    }
}
impl ::std::convert::From<FunctionToolCall> for ItemField {
    fn from(value: FunctionToolCall) -> Self {
        Self::FunctionToolCall(value)
    }
}
impl ::std::convert::From<ToolSearchCall> for ItemField {
    fn from(value: ToolSearchCall) -> Self {
        Self::ToolSearchCall(value)
    }
}
impl ::std::convert::From<ToolSearchOutput> for ItemField {
    fn from(value: ToolSearchOutput) -> Self {
        Self::ToolSearchOutput(value)
    }
}
impl ::std::convert::From<AdditionalTools> for ItemField {
    fn from(value: AdditionalTools) -> Self {
        Self::AdditionalTools(value)
    }
}
impl ::std::convert::From<FunctionToolCallOutput> for ItemField {
    fn from(value: FunctionToolCallOutput) -> Self {
        Self::FunctionToolCallOutput(value)
    }
}
impl ::std::convert::From<FileSearchToolCall> for ItemField {
    fn from(value: FileSearchToolCall) -> Self {
        Self::FileSearchToolCall(value)
    }
}
impl ::std::convert::From<WebSearchToolCall> for ItemField {
    fn from(value: WebSearchToolCall) -> Self {
        Self::WebSearchToolCall(value)
    }
}
impl ::std::convert::From<ImageGenToolCall> for ItemField {
    fn from(value: ImageGenToolCall) -> Self {
        Self::ImageGenToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCall> for ItemField {
    fn from(value: ComputerToolCall) -> Self {
        Self::ComputerToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCallOutputResource> for ItemField {
    fn from(value: ComputerToolCallOutputResource) -> Self {
        Self::ComputerToolCallOutputResource(value)
    }
}
impl ::std::convert::From<ReasoningItem> for ItemField {
    fn from(value: ReasoningItem) -> Self {
        Self::ReasoningItem(value)
    }
}
impl ::std::convert::From<CompactionBody> for ItemField {
    fn from(value: CompactionBody) -> Self {
        Self::CompactionBody(value)
    }
}
impl ::std::convert::From<CodeInterpreterToolCall> for ItemField {
    fn from(value: CodeInterpreterToolCall) -> Self {
        Self::CodeInterpreterToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCall> for ItemField {
    fn from(value: LocalShellToolCall) -> Self {
        Self::LocalShellToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCallOutput> for ItemField {
    fn from(value: LocalShellToolCallOutput) -> Self {
        Self::LocalShellToolCallOutput(value)
    }
}
impl ::std::convert::From<FunctionShellCall> for ItemField {
    fn from(value: FunctionShellCall) -> Self {
        Self::FunctionShellCall(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutput> for ItemField {
    fn from(value: FunctionShellCallOutput) -> Self {
        Self::FunctionShellCallOutput(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCall> for ItemField {
    fn from(value: ApplyPatchToolCall) -> Self {
        Self::ApplyPatchToolCall(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutput> for ItemField {
    fn from(value: ApplyPatchToolCallOutput) -> Self {
        Self::ApplyPatchToolCallOutput(value)
    }
}
impl ::std::convert::From<McpListTools> for ItemField {
    fn from(value: McpListTools) -> Self {
        Self::McpListTools(value)
    }
}
impl ::std::convert::From<McpApprovalRequest> for ItemField {
    fn from(value: McpApprovalRequest) -> Self {
        Self::McpApprovalRequest(value)
    }
}
impl ::std::convert::From<McpApprovalResponseResource> for ItemField {
    fn from(value: McpApprovalResponseResource) -> Self {
        Self::McpApprovalResponseResource(value)
    }
}
impl ::std::convert::From<McpToolCall> for ItemField {
    fn from(value: McpToolCall) -> Self {
        Self::McpToolCall(value)
    }
}
impl ::std::convert::From<CustomToolCall> for ItemField {
    fn from(value: CustomToolCall) -> Self {
        Self::CustomToolCall(value)
    }
}
impl ::std::convert::From<CustomToolCallOutput> for ItemField {
    fn from(value: CustomToolCallOutput) -> Self {
        Self::CustomToolCallOutput(value)
    }
}
#[doc = "An internal identifier for an item to reference."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ItemReferenceParam {
    #[doc = "The ID of the item to reference."]
    pub id: ::std::string::String,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ItemReferenceParamType>,
}
#[doc = "The type of item to reference. Always `item_reference`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ItemReferenceParamType {
    #[serde(rename = "item_reference")]
    ItemReference,
}
impl ::std::fmt::Display for ItemReferenceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ItemReference => f.write_str("item_reference"),
        }
    }
}
impl ::std::str::FromStr for ItemReferenceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "item_reference" => Ok(Self::ItemReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ItemReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ItemReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Content item used to generate a response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ItemResource {
    InputMessageResource(InputMessageResource),
    OutputMessage(OutputMessage),
    FileSearchToolCall(FileSearchToolCall),
    ComputerToolCall(ComputerToolCall),
    ComputerToolCallOutputResource(ComputerToolCallOutputResource),
    WebSearchToolCall(WebSearchToolCall),
    FunctionToolCallResource(FunctionToolCallResource),
    FunctionToolCallOutputResource(FunctionToolCallOutputResource),
    ToolSearchCall(ToolSearchCall),
    ToolSearchOutput(ToolSearchOutput),
    AdditionalTools(AdditionalTools),
    ResponseConfigurationUpdate(ResponseConfigurationUpdate),
    ReasoningItem(ReasoningItem),
    Program(Program),
    ProgramOutput(ProgramOutput),
    CompactionBody(CompactionBody),
    ImageGenToolCall(ImageGenToolCall),
    CodeInterpreterToolCall(CodeInterpreterToolCall),
    LocalShellToolCall(LocalShellToolCall),
    LocalShellToolCallOutput(LocalShellToolCallOutput),
    FunctionShellCall(FunctionShellCall),
    FunctionShellCallOutput(FunctionShellCallOutput),
    ApplyPatchToolCall(ApplyPatchToolCall),
    ApplyPatchToolCallOutput(ApplyPatchToolCallOutput),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponseResource(McpApprovalResponseResource),
    McpToolCall(McpToolCall),
    CustomToolCallResource(CustomToolCallResource),
    CustomToolCallOutputResource(CustomToolCallOutputResource),
}
impl ::std::convert::From<InputMessageResource> for ItemResource {
    fn from(value: InputMessageResource) -> Self {
        Self::InputMessageResource(value)
    }
}
impl ::std::convert::From<OutputMessage> for ItemResource {
    fn from(value: OutputMessage) -> Self {
        Self::OutputMessage(value)
    }
}
impl ::std::convert::From<FileSearchToolCall> for ItemResource {
    fn from(value: FileSearchToolCall) -> Self {
        Self::FileSearchToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCall> for ItemResource {
    fn from(value: ComputerToolCall) -> Self {
        Self::ComputerToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCallOutputResource> for ItemResource {
    fn from(value: ComputerToolCallOutputResource) -> Self {
        Self::ComputerToolCallOutputResource(value)
    }
}
impl ::std::convert::From<WebSearchToolCall> for ItemResource {
    fn from(value: WebSearchToolCall) -> Self {
        Self::WebSearchToolCall(value)
    }
}
impl ::std::convert::From<FunctionToolCallResource> for ItemResource {
    fn from(value: FunctionToolCallResource) -> Self {
        Self::FunctionToolCallResource(value)
    }
}
impl ::std::convert::From<FunctionToolCallOutputResource> for ItemResource {
    fn from(value: FunctionToolCallOutputResource) -> Self {
        Self::FunctionToolCallOutputResource(value)
    }
}
impl ::std::convert::From<ToolSearchCall> for ItemResource {
    fn from(value: ToolSearchCall) -> Self {
        Self::ToolSearchCall(value)
    }
}
impl ::std::convert::From<ToolSearchOutput> for ItemResource {
    fn from(value: ToolSearchOutput) -> Self {
        Self::ToolSearchOutput(value)
    }
}
impl ::std::convert::From<AdditionalTools> for ItemResource {
    fn from(value: AdditionalTools) -> Self {
        Self::AdditionalTools(value)
    }
}
impl ::std::convert::From<ResponseConfigurationUpdate> for ItemResource {
    fn from(value: ResponseConfigurationUpdate) -> Self {
        Self::ResponseConfigurationUpdate(value)
    }
}
impl ::std::convert::From<ReasoningItem> for ItemResource {
    fn from(value: ReasoningItem) -> Self {
        Self::ReasoningItem(value)
    }
}
impl ::std::convert::From<Program> for ItemResource {
    fn from(value: Program) -> Self {
        Self::Program(value)
    }
}
impl ::std::convert::From<ProgramOutput> for ItemResource {
    fn from(value: ProgramOutput) -> Self {
        Self::ProgramOutput(value)
    }
}
impl ::std::convert::From<CompactionBody> for ItemResource {
    fn from(value: CompactionBody) -> Self {
        Self::CompactionBody(value)
    }
}
impl ::std::convert::From<ImageGenToolCall> for ItemResource {
    fn from(value: ImageGenToolCall) -> Self {
        Self::ImageGenToolCall(value)
    }
}
impl ::std::convert::From<CodeInterpreterToolCall> for ItemResource {
    fn from(value: CodeInterpreterToolCall) -> Self {
        Self::CodeInterpreterToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCall> for ItemResource {
    fn from(value: LocalShellToolCall) -> Self {
        Self::LocalShellToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCallOutput> for ItemResource {
    fn from(value: LocalShellToolCallOutput) -> Self {
        Self::LocalShellToolCallOutput(value)
    }
}
impl ::std::convert::From<FunctionShellCall> for ItemResource {
    fn from(value: FunctionShellCall) -> Self {
        Self::FunctionShellCall(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutput> for ItemResource {
    fn from(value: FunctionShellCallOutput) -> Self {
        Self::FunctionShellCallOutput(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCall> for ItemResource {
    fn from(value: ApplyPatchToolCall) -> Self {
        Self::ApplyPatchToolCall(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutput> for ItemResource {
    fn from(value: ApplyPatchToolCallOutput) -> Self {
        Self::ApplyPatchToolCallOutput(value)
    }
}
impl ::std::convert::From<McpListTools> for ItemResource {
    fn from(value: McpListTools) -> Self {
        Self::McpListTools(value)
    }
}
impl ::std::convert::From<McpApprovalRequest> for ItemResource {
    fn from(value: McpApprovalRequest) -> Self {
        Self::McpApprovalRequest(value)
    }
}
impl ::std::convert::From<McpApprovalResponseResource> for ItemResource {
    fn from(value: McpApprovalResponseResource) -> Self {
        Self::McpApprovalResponseResource(value)
    }
}
impl ::std::convert::From<McpToolCall> for ItemResource {
    fn from(value: McpToolCall) -> Self {
        Self::McpToolCall(value)
    }
}
impl ::std::convert::From<CustomToolCallResource> for ItemResource {
    fn from(value: CustomToolCallResource) -> Self {
        Self::CustomToolCallResource(value)
    }
}
impl ::std::convert::From<CustomToolCallOutputResource> for ItemResource {
    fn from(value: CustomToolCallOutputResource) -> Self {
        Self::CustomToolCallOutputResource(value)
    }
}
#[doc = "A collection of keypresses the model would like to perform."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct KeyPressAction {
    #[doc = "The combination of keys the model is requesting to be pressed. This is an array of strings, each representing a key."]
    pub keys: ::std::vec::Vec<::std::string::String>,
    #[doc = "Specifies the event type. For a keypress action, this property is always set to `keypress`."]
    #[serde(rename = "type")]
    pub type_: KeyPressActionType,
}
#[doc = "Specifies the event type. For a keypress action, this property is always set to `keypress`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum KeyPressActionType {
    #[serde(rename = "keypress")]
    Keypress,
}
impl ::std::fmt::Display for KeyPressActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Keypress => f.write_str("keypress"),
        }
    }
}
impl ::std::str::FromStr for KeyPressActionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "keypress" => Ok(Self::Keypress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for KeyPressActionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for KeyPressActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListInputItemsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListInputItemsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<IncludeEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListInputItemsParamsOrder>,
}
#[doc = "`ListInputItemsParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListInputItemsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListInputItemsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListInputItemsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListInputItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListInputItemsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LocalEnvironmentParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalEnvironmentParam {
    #[doc = "An optional list of skills."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<LocalSkillParam>,
    #[doc = "Use a local computer environment."]
    #[serde(rename = "type")]
    pub type_: LocalEnvironmentParamType,
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
pub enum LocalEnvironmentParamType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for LocalEnvironmentParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for LocalEnvironmentParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local" => Ok(Self::Local),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalEnvironmentParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalEnvironmentParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents the use of a local environment to perform shell actions."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalEnvironmentResource {
    #[doc = "The environment type. Always `local`."]
    #[serde(rename = "type")]
    pub type_: LocalEnvironmentResourceType,
}
#[doc = "The environment type. Always `local`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalEnvironmentResourceType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for LocalEnvironmentResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for LocalEnvironmentResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local" => Ok(Self::Local),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalEnvironmentResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalEnvironmentResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Execute a shell command on the server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalShellExecAction {
    #[doc = "The command to run."]
    pub command: ::std::vec::Vec<::std::string::String>,
    #[doc = "Environment variables to set for the command."]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_ms: ::std::option::Option<i64>,
    #[doc = "The type of the local shell action. Always `exec`."]
    #[serde(rename = "type")]
    pub type_: LocalShellExecActionType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub working_directory: ::std::option::Option<::std::string::String>,
}
#[doc = "The type of the local shell action. Always `exec`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellExecActionType {
    #[serde(rename = "exec")]
    Exec,
}
impl ::std::fmt::Display for LocalShellExecActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exec => f.write_str("exec"),
        }
    }
}
impl ::std::str::FromStr for LocalShellExecActionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exec" => Ok(Self::Exec),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellExecActionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellExecActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool call to run a command on the local shell.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalShellToolCall {
    pub action: LocalShellExecAction,
    #[doc = "The unique ID of the local shell tool call generated by the model.\n"]
    pub call_id: ::std::string::String,
    #[doc = "The unique ID of the local shell call.\n"]
    pub id: ::std::string::String,
    #[doc = "The status of the local shell call.\n"]
    pub status: LocalShellToolCallStatus,
    #[doc = "The type of the local shell call. Always `local_shell_call`.\n"]
    #[serde(rename = "type")]
    pub type_: LocalShellToolCallType,
}
#[doc = "The output of a local shell tool call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalShellToolCallOutput {
    pub call_id: ::serde_json::Value,
    #[doc = "The unique ID of the local shell tool call generated by the model.\n"]
    pub id: ::std::string::String,
    #[doc = "A JSON string of the output of the local shell tool call.\n"]
    pub output: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<LocalShellToolCallOutputStatus>,
    #[doc = "The type of the local shell tool call output. Always `local_shell_call_output`.\n"]
    #[serde(rename = "type")]
    pub type_: LocalShellToolCallOutputType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or `incomplete`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellToolCallOutputStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for LocalShellToolCallOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for LocalShellToolCallOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellToolCallOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the local shell tool call output. Always `local_shell_call_output`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellToolCallOutputType {
    #[serde(rename = "local_shell_call_output")]
    LocalShellCallOutput,
}
impl ::std::fmt::Display for LocalShellToolCallOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LocalShellCallOutput => f.write_str("local_shell_call_output"),
        }
    }
}
impl ::std::str::FromStr for LocalShellToolCallOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local_shell_call_output" => Ok(Self::LocalShellCallOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellToolCallOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the local shell call.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for LocalShellToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for LocalShellToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the local shell call. Always `local_shell_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellToolCallType {
    #[serde(rename = "local_shell_call")]
    LocalShellCall,
}
impl ::std::fmt::Display for LocalShellToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LocalShellCall => f.write_str("local_shell_call"),
        }
    }
}
impl ::std::str::FromStr for LocalShellToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local_shell_call" => Ok(Self::LocalShellCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that allows the model to execute shell commands in a local environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalShellToolParam {
    #[doc = "The type of the local shell tool. Always `local_shell`."]
    #[serde(rename = "type")]
    pub type_: LocalShellToolParamType,
}
#[doc = "The type of the local shell tool. Always `local_shell`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LocalShellToolParamType {
    #[serde(rename = "local_shell")]
    LocalShell,
}
impl ::std::fmt::Display for LocalShellToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LocalShell => f.write_str("local_shell"),
        }
    }
}
impl ::std::str::FromStr for LocalShellToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local_shell" => Ok(Self::LocalShell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LocalShellToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LocalShellToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LocalSkillParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LocalSkillParam {
    #[doc = "The description of the skill."]
    pub description: ::std::string::String,
    #[doc = "The name of the skill."]
    pub name: ::std::string::String,
    #[doc = "The path to the directory containing the skill."]
    pub path: ::std::string::String,
}
#[doc = "The log probability of a token."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LogProb {
    pub bytes: ::std::vec::Vec<i64>,
    pub logprob: f64,
    pub token: ::std::string::String,
    pub top_logprobs: ::std::vec::Vec<TopLogProb>,
}
#[doc = "A request for human approval of a tool invocation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpApprovalRequest {
    #[doc = "A JSON string of arguments for the tool.\n"]
    pub arguments: ::std::string::String,
    #[doc = "The unique ID of the approval request.\n"]
    pub id: ::std::string::String,
    #[doc = "The name of the tool to run.\n"]
    pub name: ::std::string::String,
    #[doc = "The label of the MCP server making the request.\n"]
    pub server_label: ::std::string::String,
    #[doc = "The type of the item. Always `mcp_approval_request`.\n"]
    #[serde(rename = "type")]
    pub type_: McpApprovalRequestType,
}
#[doc = "The type of the item. Always `mcp_approval_request`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpApprovalRequestType {
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest,
}
impl ::std::fmt::Display for McpApprovalRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpApprovalRequest => f.write_str("mcp_approval_request"),
        }
    }
}
impl ::std::str::FromStr for McpApprovalRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_approval_request" => Ok(Self::McpApprovalRequest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpApprovalRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpApprovalRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A response to an MCP approval request.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpApprovalResponse {
    #[doc = "The ID of the approval request being answered.\n"]
    pub approval_request_id: ::std::string::String,
    #[doc = "Whether the request was approved.\n"]
    pub approve: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    pub request_id: ::serde_json::Value,
    #[doc = "The type of the item. Always `mcp_approval_response`.\n"]
    #[serde(rename = "type")]
    pub type_: McpApprovalResponseType,
}
#[doc = "A response to an MCP approval request.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpApprovalResponseResource {
    #[doc = "The ID of the approval request being answered.\n"]
    pub approval_request_id: ::std::string::String,
    #[doc = "Whether the request was approved.\n"]
    pub approve: bool,
    #[doc = "The unique ID of the approval response\n"]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    pub request_id: ::serde_json::Value,
    #[doc = "The type of the item. Always `mcp_approval_response`.\n"]
    #[serde(rename = "type")]
    pub type_: McpApprovalResponseResourceType,
}
#[doc = "The type of the item. Always `mcp_approval_response`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpApprovalResponseResourceType {
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse,
}
impl ::std::fmt::Display for McpApprovalResponseResourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpApprovalResponse => f.write_str("mcp_approval_response"),
        }
    }
}
impl ::std::str::FromStr for McpApprovalResponseResourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_approval_response" => Ok(Self::McpApprovalResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpApprovalResponseResourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpApprovalResponseResourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `mcp_approval_response`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpApprovalResponseType {
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse,
}
impl ::std::fmt::Display for McpApprovalResponseType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpApprovalResponse => f.write_str("mcp_approval_response"),
        }
    }
}
impl ::std::str::FromStr for McpApprovalResponseType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_approval_response" => Ok(Self::McpApprovalResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpApprovalResponseType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpApprovalResponseType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A list of tools available on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpListTools {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID of the list.\n"]
    pub id: ::std::string::String,
    #[doc = "The label of the MCP server.\n"]
    pub server_label: ::std::string::String,
    #[doc = "The tools available on the server.\n"]
    pub tools: ::std::vec::Vec<McpListToolsTool>,
    #[doc = "The type of the item. Always `mcp_list_tools`.\n"]
    #[serde(rename = "type")]
    pub type_: McpListToolsType,
}
#[doc = "A tool available on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpListToolsTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The JSON schema describing the tool's input.\n"]
    pub input_schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The name of the tool.\n"]
    pub name: ::std::string::String,
}
#[doc = "The type of the item. Always `mcp_list_tools`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpListToolsType {
    #[serde(rename = "mcp_list_tools")]
    McpListTools,
}
impl ::std::fmt::Display for McpListToolsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpListTools => f.write_str("mcp_list_tools"),
        }
    }
}
impl ::std::str::FromStr for McpListToolsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_list_tools" => Ok(Self::McpListTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpListToolsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpListToolsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`McpProtocolError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpProtocolError {
    pub code: i64,
    pub message: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: McpProtocolErrorType,
}
#[doc = "`McpProtocolErrorType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpProtocolErrorType {
    #[serde(rename = "mcp_protocol_error")]
    McpProtocolError,
}
impl ::std::fmt::Display for McpProtocolErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpProtocolError => f.write_str("mcp_protocol_error"),
        }
    }
}
impl ::std::str::FromStr for McpProtocolErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_protocol_error" => Ok(Self::McpProtocolError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpProtocolErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpProtocolErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Give the model access to additional tools via remote Model Context Protocol\n(MCP) servers. [Learn more about MCP](https://developers.openai.com/api/docs/guides/tools-connectors-mcp).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_tools: ::std::option::Option<McpToolAllowedTools>,
    #[doc = "An OAuth access token that can be used with a remote MCP server, either\nwith a custom MCP server URL or a service connector. Your application\nmust handle the OAuth authorization flow and provide the token here.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub authorization: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for service connectors, like those available in ChatGPT. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided. Learn more\nabout service connectors [here](https://developers.openai.com/api/docs/guides/tools-connectors-mcp#connectors).\n\nThis field is deprecated for models released after September 1, 2026.\nUse `server_url` to connect to a remote MCP server, or `tunnel_id` to\nconnect through a Secure MCP Tunnel.\n\nCurrently supported `connector_id` values are:\n\n- Dropbox: `connector_dropbox`\n- Gmail: `connector_gmail`\n- Google Calendar: `connector_googlecalendar`\n- Google Drive: `connector_googledrive`\n- Microsoft Teams: `connector_microsoftteams`\n- Outlook Calendar: `connector_outlookcalendar`\n- Outlook Email: `connector_outlookemail`\n- SharePoint: `connector_sharepoint`\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_id: ::std::option::Option<McpToolConnectorId>,
    #[doc = "Whether this MCP tool is deferred and discovered via tool search.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub require_approval: ::std::option::Option<McpToolRequireApproval>,
    #[doc = "Optional description of the MCP server, used to provide more context.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_description: ::std::option::Option<::std::string::String>,
    #[doc = "A label for this MCP server, used to identify it in tool calls.\n"]
    pub server_label: ::std::string::String,
    #[doc = "The URL for the MCP server. One of `server_url`, `connector_id`, or\n`tunnel_id` must be provided.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_url: ::std::option::Option<::std::string::String>,
    #[doc = "The Secure MCP Tunnel ID to use instead of a direct server URL. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tunnel_id: ::std::option::Option<McpToolTunnelId>,
    #[doc = "The type of the MCP tool. Always `mcp`."]
    #[serde(rename = "type")]
    pub type_: McpToolType,
}
#[doc = "List of allowed tool names or a filter object.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum McpToolAllowedTools {
    AllowedTools(::std::vec::Vec<::std::string::String>),
    ToolFilter(McpToolFilter),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for McpToolAllowedTools {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::AllowedTools(value)
    }
}
impl ::std::convert::From<McpToolFilter> for McpToolAllowedTools {
    fn from(value: McpToolFilter) -> Self {
        Self::ToolFilter(value)
    }
}
#[doc = "Specify a single approval policy for all tools. One of `always` or\n`never`. When set to `always`, all tools will require approval. When\nset to `never`, all tools will not require approval.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolApprovalSetting {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
}
impl ::std::fmt::Display for McpToolApprovalSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Always => f.write_str("always"),
            Self::Never => f.write_str("never"),
        }
    }
}
impl ::std::str::FromStr for McpToolApprovalSetting {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolApprovalSetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolApprovalSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An invocation of a tool on an MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpToolCall {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub approval_request_id: ::std::option::Option<::std::string::String>,
    #[doc = "A JSON string of the arguments passed to the tool.\n"]
    pub arguments: ::std::string::String,
    #[doc = "The error from the tool call, if any."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<McpToolCallError>,
    #[doc = "The unique ID of the tool call.\n"]
    pub id: ::std::string::String,
    #[doc = "The name of the tool that was run.\n"]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server running the tool.\n"]
    pub server_label: ::std::string::String,
    #[doc = "The status of the tool call. One of `in_progress`, `completed`, `incomplete`, `calling`, or `failed`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<McpToolCallStatus>,
    #[doc = "The type of the item. Always `mcp_call`.\n"]
    #[serde(rename = "type")]
    pub type_: McpToolCallType,
}
#[doc = "`McpToolCallError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum McpToolCallError {
    McpProtocolError(McpProtocolError),
    McpToolExecutionError(McpToolExecutionError),
    HttpError(HttpError),
}
impl ::std::convert::From<McpProtocolError> for McpToolCallError {
    fn from(value: McpProtocolError) -> Self {
        Self::McpProtocolError(value)
    }
}
impl ::std::convert::From<McpToolExecutionError> for McpToolCallError {
    fn from(value: McpToolExecutionError) -> Self {
        Self::McpToolExecutionError(value)
    }
}
impl ::std::convert::From<HttpError> for McpToolCallError {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
    }
}
#[doc = "`McpToolCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "calling")]
    Calling,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for McpToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Calling => f.write_str("calling"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for McpToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            "calling" => Ok(Self::Calling),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `mcp_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolCallType {
    #[serde(rename = "mcp_call")]
    McpCall,
}
impl ::std::fmt::Display for McpToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpCall => f.write_str("mcp_call"),
        }
    }
}
impl ::std::str::FromStr for McpToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_call" => Ok(Self::McpCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier for service connectors, like those available in ChatGPT. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided. Learn more\nabout service connectors [here](https://developers.openai.com/api/docs/guides/tools-connectors-mcp#connectors).\n\nThis field is deprecated for models released after September 1, 2026.\nUse `server_url` to connect to a remote MCP server, or `tunnel_id` to\nconnect through a Secure MCP Tunnel.\n\nCurrently supported `connector_id` values are:\n\n- Dropbox: `connector_dropbox`\n- Gmail: `connector_gmail`\n- Google Calendar: `connector_googlecalendar`\n- Google Drive: `connector_googledrive`\n- Microsoft Teams: `connector_microsoftteams`\n- Outlook Calendar: `connector_outlookcalendar`\n- Outlook Email: `connector_outlookemail`\n- SharePoint: `connector_sharepoint`\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolConnectorId {
    #[serde(rename = "connector_dropbox")]
    ConnectorDropbox,
    #[serde(rename = "connector_gmail")]
    ConnectorGmail,
    #[serde(rename = "connector_googlecalendar")]
    ConnectorGooglecalendar,
    #[serde(rename = "connector_googledrive")]
    ConnectorGoogledrive,
    #[serde(rename = "connector_microsoftteams")]
    ConnectorMicrosoftteams,
    #[serde(rename = "connector_outlookcalendar")]
    ConnectorOutlookcalendar,
    #[serde(rename = "connector_outlookemail")]
    ConnectorOutlookemail,
    #[serde(rename = "connector_sharepoint")]
    ConnectorSharepoint,
}
impl ::std::fmt::Display for McpToolConnectorId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConnectorDropbox => f.write_str("connector_dropbox"),
            Self::ConnectorGmail => f.write_str("connector_gmail"),
            Self::ConnectorGooglecalendar => f.write_str("connector_googlecalendar"),
            Self::ConnectorGoogledrive => f.write_str("connector_googledrive"),
            Self::ConnectorMicrosoftteams => f.write_str("connector_microsoftteams"),
            Self::ConnectorOutlookcalendar => f.write_str("connector_outlookcalendar"),
            Self::ConnectorOutlookemail => f.write_str("connector_outlookemail"),
            Self::ConnectorSharepoint => f.write_str("connector_sharepoint"),
        }
    }
}
impl ::std::str::FromStr for McpToolConnectorId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "connector_dropbox" => Ok(Self::ConnectorDropbox),
            "connector_gmail" => Ok(Self::ConnectorGmail),
            "connector_googlecalendar" => Ok(Self::ConnectorGooglecalendar),
            "connector_googledrive" => Ok(Self::ConnectorGoogledrive),
            "connector_microsoftteams" => Ok(Self::ConnectorMicrosoftteams),
            "connector_outlookcalendar" => Ok(Self::ConnectorOutlookcalendar),
            "connector_outlookemail" => Ok(Self::ConnectorOutlookemail),
            "connector_sharepoint" => Ok(Self::ConnectorSharepoint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolConnectorId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolConnectorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`McpToolExecutionError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct McpToolExecutionError {
    pub content: ::serde_json::Value,
    #[serde(rename = "type")]
    pub type_: McpToolExecutionErrorType,
}
#[doc = "`McpToolExecutionErrorType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolExecutionErrorType {
    #[serde(rename = "mcp_tool_execution_error")]
    McpToolExecutionError,
}
impl ::std::fmt::Display for McpToolExecutionErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpToolExecutionError => f.write_str("mcp_tool_execution_error"),
        }
    }
}
impl ::std::str::FromStr for McpToolExecutionErrorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_tool_execution_error" => Ok(Self::McpToolExecutionError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolExecutionErrorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolExecutionErrorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A filter object to specify which tools are allowed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpToolFilter {
    #[doc = "Indicates whether or not a tool modifies data or is read-only. If an\nMCP server is [annotated with `readOnlyHint`](https://modelcontextprotocol.io/specification/2025-06-18/schema#toolannotations-readonlyhint),\nit will match this filter.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub read_only: ::std::option::Option<bool>,
    #[doc = "List of allowed tool names."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_names: ::std::vec::Vec<::std::string::String>,
}
#[doc = "Specify which of the MCP server's tools require approval."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, deny_unknown_fields)]
pub enum McpToolRequireApproval {
    Filter {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        always: ::std::option::Option<McpToolFilter>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        never: ::std::option::Option<McpToolFilter>,
    },
    Setting(McpToolApprovalSetting),
}
impl ::std::convert::From<McpToolApprovalSetting> for McpToolRequireApproval {
    fn from(value: McpToolApprovalSetting) -> Self {
        Self::Setting(value)
    }
}
#[doc = "The Secure MCP Tunnel ID to use instead of a direct server URL. One of\n`server_url`, `connector_id`, or `tunnel_id` must be provided.\n"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct McpToolTunnelId(::std::string::String);
impl ::std::ops::Deref for McpToolTunnelId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<McpToolTunnelId> for ::std::string::String {
    fn from(value: McpToolTunnelId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for McpToolTunnelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^tunnel_[a-z0-9]{32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^tunnel_[a-z0-9]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for McpToolTunnelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolTunnelId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for McpToolTunnelId {
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
#[doc = "The type of the MCP tool. Always `mcp`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpToolType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for McpToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for McpToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A message to or from the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Message {
    #[doc = "The content of the message"]
    pub content: ::std::vec::Vec<MessageContentItem>,
    #[doc = "The unique ID of the message."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<MessagePhase2>,
    #[doc = "The role of the message. One of `unknown`, `user`, `assistant`, `system`, `critic`, `discriminator`, `developer`, or `tool`."]
    pub role: MessageRole,
    #[doc = "The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API."]
    pub status: MessageStatus,
    #[doc = "The type of the message. Always set to `message`."]
    #[serde(rename = "type")]
    pub type_: MessageType,
}
#[doc = "A content part that makes up an input or output item."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MessageContentItem {
    InputTextContent(InputTextContent),
    OutputTextContent(OutputTextContent),
    TextContent(TextContent),
    SummaryTextContent(SummaryTextContent),
    ReasoningTextContent(ReasoningTextContent),
    RefusalContent(RefusalContent),
    InputImageContent(InputImageContent),
    ComputerScreenshotContent(ComputerScreenshotContent),
    InputFileContent(InputFileContent),
}
impl ::std::convert::From<InputTextContent> for MessageContentItem {
    fn from(value: InputTextContent) -> Self {
        Self::InputTextContent(value)
    }
}
impl ::std::convert::From<OutputTextContent> for MessageContentItem {
    fn from(value: OutputTextContent) -> Self {
        Self::OutputTextContent(value)
    }
}
impl ::std::convert::From<TextContent> for MessageContentItem {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<SummaryTextContent> for MessageContentItem {
    fn from(value: SummaryTextContent) -> Self {
        Self::SummaryTextContent(value)
    }
}
impl ::std::convert::From<ReasoningTextContent> for MessageContentItem {
    fn from(value: ReasoningTextContent) -> Self {
        Self::ReasoningTextContent(value)
    }
}
impl ::std::convert::From<RefusalContent> for MessageContentItem {
    fn from(value: RefusalContent) -> Self {
        Self::RefusalContent(value)
    }
}
impl ::std::convert::From<InputImageContent> for MessageContentItem {
    fn from(value: InputImageContent) -> Self {
        Self::InputImageContent(value)
    }
}
impl ::std::convert::From<ComputerScreenshotContent> for MessageContentItem {
    fn from(value: ComputerScreenshotContent) -> Self {
        Self::ComputerScreenshotContent(value)
    }
}
impl ::std::convert::From<InputFileContent> for MessageContentItem {
    fn from(value: InputFileContent) -> Self {
        Self::InputFileContent(value)
    }
}
#[doc = "Labels an `assistant` message as intermediate commentary (`commentary`) or the final answer (`final_answer`).\nFor models like `gpt-5.3-codex` and beyond, when sending follow-up requests, preserve and resend\nphase on all assistant messages — dropping it can degrade performance. Not used for user messages.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessagePhase {
    #[serde(rename = "commentary")]
    Commentary,
    #[serde(rename = "final_answer")]
    FinalAnswer,
}
impl ::std::fmt::Display for MessagePhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Commentary => f.write_str("commentary"),
            Self::FinalAnswer => f.write_str("final_answer"),
        }
    }
}
impl ::std::str::FromStr for MessagePhase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "commentary" => Ok(Self::Commentary),
            "final_answer" => Ok(Self::FinalAnswer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessagePhase {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessagePhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MessagePhase2`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessagePhase2 {
    #[serde(rename = "commentary")]
    Commentary,
    #[serde(rename = "final_answer")]
    FinalAnswer,
}
impl ::std::fmt::Display for MessagePhase2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Commentary => f.write_str("commentary"),
            Self::FinalAnswer => f.write_str("final_answer"),
        }
    }
}
impl ::std::str::FromStr for MessagePhase2 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "commentary" => Ok(Self::Commentary),
            "final_answer" => Ok(Self::FinalAnswer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessagePhase2 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessagePhase2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MessageRole`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageRole {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "critic")]
    Critic,
    #[serde(rename = "discriminator")]
    Discriminator,
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::System => f.write_str("system"),
            Self::Critic => f.write_str("critic"),
            Self::Discriminator => f.write_str("discriminator"),
            Self::Developer => f.write_str("developer"),
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for MessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "critic" => Ok(Self::Critic),
            "discriminator" => Ok(Self::Discriminator),
            "developer" => Ok(Self::Developer),
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MessageStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for MessageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the message. Always set to `message`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for MessageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageType {
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
#[doc = "`MisalignmentErrorDetailsResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MisalignmentErrorDetailsResource {
    #[doc = "The public explanation for this block."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detailed_explanation: ::std::option::Option<::std::string::String>,
    #[doc = "An optional classification; clients must accept additional values."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_type: ::std::option::Option<MisalignmentErrorType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub review_target: ::std::option::Option<MisalignmentErrorDetailsResourceReviewTarget>,
    #[doc = "An optional public continuation instruction."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub steer: ::std::option::Option<MisalignmentSteer>,
}
#[doc = "An opaque target for explicitly continuing this review, or null when unavailable."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MisalignmentErrorDetailsResourceReviewTarget(::std::string::String);
impl ::std::ops::Deref for MisalignmentErrorDetailsResourceReviewTarget {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MisalignmentErrorDetailsResourceReviewTarget> for ::std::string::String {
    fn from(value: MisalignmentErrorDetailsResourceReviewTarget) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MisalignmentErrorDetailsResourceReviewTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 96usize {
            return Err("longer than 96 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._~:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._~:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MisalignmentErrorDetailsResourceReviewTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for MisalignmentErrorDetailsResourceReviewTarget
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MisalignmentErrorDetailsResourceReviewTarget {
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
#[doc = "`MisalignmentErrorType`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct MisalignmentErrorType {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<MisalignmentErrorTypeSubtype1>,
}
#[doc = "`MisalignmentErrorTypeSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MisalignmentErrorTypeSubtype1 {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for MisalignmentErrorTypeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PotentiallyUnintendedDataTransfer => {
                f.write_str("potentially_unintended_data_transfer")
            }
            Self::PotentiallyUnintendedDataAccess => {
                f.write_str("potentially_unintended_data_access")
            }
            Self::PotentiallyUnintendedDestructiveActivity => {
                f.write_str("potentially_unintended_destructive_activity")
            }
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for MisalignmentErrorTypeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "potentially_unintended_data_transfer" => Ok(Self::PotentiallyUnintendedDataTransfer),
            "potentially_unintended_data_access" => Ok(Self::PotentiallyUnintendedDataAccess),
            "potentially_unintended_destructive_activity" => {
                Ok(Self::PotentiallyUnintendedDestructiveActivity)
            }
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MisalignmentErrorTypeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MisalignmentErrorTypeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MisalignmentSteer`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MisalignmentSteer {
    #[doc = "The public continuation instruction."]
    pub message: ::std::string::String,
}
#[doc = "Model ID used to generate the response, like `gpt-6-astra`. OpenAI offers a wide range of models with different capabilities, performance characteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models) to browse and compare available models."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModelIdsCompaction {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<ModelIdsResponses>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
#[doc = "`ModelIdsResponses`"]
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
pub struct ModelIdsResponses(pub ::std::string::String);
impl ::std::ops::Deref for ModelIdsResponses {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelIdsResponses> for ::std::string::String {
    fn from(value: ModelIdsResponses) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ModelIdsResponses {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ModelIdsResponses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ModelIdsResponses {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "`ModelIdsShared`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ModelIdsShared {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ModelIdsSharedSubtype1>,
}
#[doc = "`ModelIdsSharedSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ModelIdsSharedSubtype1 {
    #[serde(rename = "gpt-6-astra")]
    Gpt6Astra,
    #[serde(rename = "gpt-6.1-sol")]
    Gpt61Sol,
    #[serde(rename = "gpt-6-sol")]
    Gpt6Sol,
    #[serde(rename = "gpt-6-luna")]
    Gpt6Luna,
    #[serde(rename = "gpt-5.6-sol")]
    Gpt56Sol,
    #[serde(rename = "gpt-5.6-terra")]
    Gpt56Terra,
    #[serde(rename = "gpt-5.6-luna")]
    Gpt56Luna,
    #[serde(rename = "gpt-5.5")]
    Gpt55,
    #[serde(rename = "gpt-5.5-2026-04-23")]
    Gpt5520260423,
    #[serde(rename = "gpt-5.4")]
    Gpt54,
    #[serde(rename = "gpt-5.4-mini")]
    Gpt54Mini,
    #[serde(rename = "gpt-5.4-nano")]
    Gpt54Nano,
    #[serde(rename = "gpt-5.4-mini-2026-03-17")]
    Gpt54Mini20260317,
    #[serde(rename = "gpt-5.4-nano-2026-03-17")]
    Gpt54Nano20260317,
    #[serde(rename = "gpt-5.3-chat-latest")]
    Gpt53ChatLatest,
    #[serde(rename = "gpt-5.2")]
    Gpt52,
    #[serde(rename = "gpt-5.2-2025-12-11")]
    Gpt5220251211,
    #[serde(rename = "gpt-5.2-chat-latest")]
    Gpt52ChatLatest,
    #[serde(rename = "gpt-5.2-pro")]
    Gpt52Pro,
    #[serde(rename = "gpt-5.2-pro-2025-12-11")]
    Gpt52Pro20251211,
    #[serde(rename = "gpt-5.1")]
    Gpt51,
    #[serde(rename = "gpt-5.1-2025-11-13")]
    Gpt5120251113,
    #[serde(rename = "gpt-5.1-codex")]
    Gpt51Codex,
    #[serde(rename = "gpt-5.1-mini")]
    Gpt51Mini,
    #[serde(rename = "gpt-5.1-chat-latest")]
    Gpt51ChatLatest,
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
    #[serde(rename = "gpt-5-chat-latest")]
    Gpt5ChatLatest,
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
    #[serde(rename = "o4-mini")]
    O4Mini,
    #[serde(rename = "o4-mini-2025-04-16")]
    O4Mini20250416,
    #[serde(rename = "o3")]
    O3,
    #[serde(rename = "o3-2025-04-16")]
    O320250416,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3-mini-2025-01-31")]
    O3Mini20250131,
    #[serde(rename = "o1")]
    O1,
    #[serde(rename = "o1-2024-12-17")]
    O120241217,
    #[serde(rename = "o1-preview")]
    O1Preview,
    #[serde(rename = "o1-preview-2024-09-12")]
    O1Preview20240912,
    #[serde(rename = "o1-mini")]
    O1Mini,
    #[serde(rename = "o1-mini-2024-09-12")]
    O1Mini20240912,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-audio-mini")]
    GptAudioMini,
    #[serde(rename = "gpt-audio-mini-2025-12-15")]
    GptAudioMini20251215,
    #[serde(rename = "gpt-4o-audio-preview")]
    Gpt4oAudioPreview,
    #[serde(rename = "gpt-4o-audio-preview-2024-10-01")]
    Gpt4oAudioPreview20241001,
    #[serde(rename = "gpt-4o-audio-preview-2024-12-17")]
    Gpt4oAudioPreview20241217,
    #[serde(rename = "gpt-4o-audio-preview-2025-06-03")]
    Gpt4oAudioPreview20250603,
    #[serde(rename = "gpt-4o-mini-audio-preview")]
    Gpt4oMiniAudioPreview,
    #[serde(rename = "gpt-4o-mini-audio-preview-2024-12-17")]
    Gpt4oMiniAudioPreview20241217,
    #[serde(rename = "gpt-4o-search-preview")]
    Gpt4oSearchPreview,
    #[serde(rename = "gpt-4o-mini-search-preview")]
    Gpt4oMiniSearchPreview,
    #[serde(rename = "gpt-4o-search-preview-2025-03-11")]
    Gpt4oSearchPreview20250311,
    #[serde(rename = "gpt-4o-mini-search-preview-2025-03-11")]
    Gpt4oMiniSearchPreview20250311,
    #[serde(rename = "chatgpt-4o-latest")]
    Chatgpt4oLatest,
    #[serde(rename = "codex-mini-latest")]
    CodexMiniLatest,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
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
    #[serde(rename = "gpt-3.5-turbo-0301")]
    Gpt35Turbo0301,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt35Turbo16k0613,
}
impl ::std::fmt::Display for ModelIdsSharedSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gpt6Astra => f.write_str("gpt-6-astra"),
            Self::Gpt61Sol => f.write_str("gpt-6.1-sol"),
            Self::Gpt6Sol => f.write_str("gpt-6-sol"),
            Self::Gpt6Luna => f.write_str("gpt-6-luna"),
            Self::Gpt56Sol => f.write_str("gpt-5.6-sol"),
            Self::Gpt56Terra => f.write_str("gpt-5.6-terra"),
            Self::Gpt56Luna => f.write_str("gpt-5.6-luna"),
            Self::Gpt55 => f.write_str("gpt-5.5"),
            Self::Gpt5520260423 => f.write_str("gpt-5.5-2026-04-23"),
            Self::Gpt54 => f.write_str("gpt-5.4"),
            Self::Gpt54Mini => f.write_str("gpt-5.4-mini"),
            Self::Gpt54Nano => f.write_str("gpt-5.4-nano"),
            Self::Gpt54Mini20260317 => f.write_str("gpt-5.4-mini-2026-03-17"),
            Self::Gpt54Nano20260317 => f.write_str("gpt-5.4-nano-2026-03-17"),
            Self::Gpt53ChatLatest => f.write_str("gpt-5.3-chat-latest"),
            Self::Gpt52 => f.write_str("gpt-5.2"),
            Self::Gpt5220251211 => f.write_str("gpt-5.2-2025-12-11"),
            Self::Gpt52ChatLatest => f.write_str("gpt-5.2-chat-latest"),
            Self::Gpt52Pro => f.write_str("gpt-5.2-pro"),
            Self::Gpt52Pro20251211 => f.write_str("gpt-5.2-pro-2025-12-11"),
            Self::Gpt51 => f.write_str("gpt-5.1"),
            Self::Gpt5120251113 => f.write_str("gpt-5.1-2025-11-13"),
            Self::Gpt51Codex => f.write_str("gpt-5.1-codex"),
            Self::Gpt51Mini => f.write_str("gpt-5.1-mini"),
            Self::Gpt51ChatLatest => f.write_str("gpt-5.1-chat-latest"),
            Self::Gpt5 => f.write_str("gpt-5"),
            Self::Gpt5Mini => f.write_str("gpt-5-mini"),
            Self::Gpt5Nano => f.write_str("gpt-5-nano"),
            Self::Gpt520250807 => f.write_str("gpt-5-2025-08-07"),
            Self::Gpt5Mini20250807 => f.write_str("gpt-5-mini-2025-08-07"),
            Self::Gpt5Nano20250807 => f.write_str("gpt-5-nano-2025-08-07"),
            Self::Gpt5ChatLatest => f.write_str("gpt-5-chat-latest"),
            Self::Gpt41 => f.write_str("gpt-4.1"),
            Self::Gpt41Mini => f.write_str("gpt-4.1-mini"),
            Self::Gpt41Nano => f.write_str("gpt-4.1-nano"),
            Self::Gpt4120250414 => f.write_str("gpt-4.1-2025-04-14"),
            Self::Gpt41Mini20250414 => f.write_str("gpt-4.1-mini-2025-04-14"),
            Self::Gpt41Nano20250414 => f.write_str("gpt-4.1-nano-2025-04-14"),
            Self::O4Mini => f.write_str("o4-mini"),
            Self::O4Mini20250416 => f.write_str("o4-mini-2025-04-16"),
            Self::O3 => f.write_str("o3"),
            Self::O320250416 => f.write_str("o3-2025-04-16"),
            Self::O3Mini => f.write_str("o3-mini"),
            Self::O3Mini20250131 => f.write_str("o3-mini-2025-01-31"),
            Self::O1 => f.write_str("o1"),
            Self::O120241217 => f.write_str("o1-2024-12-17"),
            Self::O1Preview => f.write_str("o1-preview"),
            Self::O1Preview20240912 => f.write_str("o1-preview-2024-09-12"),
            Self::O1Mini => f.write_str("o1-mini"),
            Self::O1Mini20240912 => f.write_str("o1-mini-2024-09-12"),
            Self::Gpt4o => f.write_str("gpt-4o"),
            Self::Gpt4o20241120 => f.write_str("gpt-4o-2024-11-20"),
            Self::Gpt4o20240806 => f.write_str("gpt-4o-2024-08-06"),
            Self::Gpt4o20240513 => f.write_str("gpt-4o-2024-05-13"),
            Self::GptAudioMini => f.write_str("gpt-audio-mini"),
            Self::GptAudioMini20251215 => f.write_str("gpt-audio-mini-2025-12-15"),
            Self::Gpt4oAudioPreview => f.write_str("gpt-4o-audio-preview"),
            Self::Gpt4oAudioPreview20241001 => f.write_str("gpt-4o-audio-preview-2024-10-01"),
            Self::Gpt4oAudioPreview20241217 => f.write_str("gpt-4o-audio-preview-2024-12-17"),
            Self::Gpt4oAudioPreview20250603 => f.write_str("gpt-4o-audio-preview-2025-06-03"),
            Self::Gpt4oMiniAudioPreview => f.write_str("gpt-4o-mini-audio-preview"),
            Self::Gpt4oMiniAudioPreview20241217 => {
                f.write_str("gpt-4o-mini-audio-preview-2024-12-17")
            }
            Self::Gpt4oSearchPreview => f.write_str("gpt-4o-search-preview"),
            Self::Gpt4oMiniSearchPreview => f.write_str("gpt-4o-mini-search-preview"),
            Self::Gpt4oSearchPreview20250311 => f.write_str("gpt-4o-search-preview-2025-03-11"),
            Self::Gpt4oMiniSearchPreview20250311 => {
                f.write_str("gpt-4o-mini-search-preview-2025-03-11")
            }
            Self::Chatgpt4oLatest => f.write_str("chatgpt-4o-latest"),
            Self::CodexMiniLatest => f.write_str("codex-mini-latest"),
            Self::Gpt4oMini => f.write_str("gpt-4o-mini"),
            Self::Gpt4oMini20240718 => f.write_str("gpt-4o-mini-2024-07-18"),
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
            Self::Gpt35Turbo0301 => f.write_str("gpt-3.5-turbo-0301"),
            Self::Gpt35Turbo0613 => f.write_str("gpt-3.5-turbo-0613"),
            Self::Gpt35Turbo1106 => f.write_str("gpt-3.5-turbo-1106"),
            Self::Gpt35Turbo0125 => f.write_str("gpt-3.5-turbo-0125"),
            Self::Gpt35Turbo16k0613 => f.write_str("gpt-3.5-turbo-16k-0613"),
        }
    }
}
impl ::std::str::FromStr for ModelIdsSharedSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gpt-6-astra" => Ok(Self::Gpt6Astra),
            "gpt-6.1-sol" => Ok(Self::Gpt61Sol),
            "gpt-6-sol" => Ok(Self::Gpt6Sol),
            "gpt-6-luna" => Ok(Self::Gpt6Luna),
            "gpt-5.6-sol" => Ok(Self::Gpt56Sol),
            "gpt-5.6-terra" => Ok(Self::Gpt56Terra),
            "gpt-5.6-luna" => Ok(Self::Gpt56Luna),
            "gpt-5.5" => Ok(Self::Gpt55),
            "gpt-5.5-2026-04-23" => Ok(Self::Gpt5520260423),
            "gpt-5.4" => Ok(Self::Gpt54),
            "gpt-5.4-mini" => Ok(Self::Gpt54Mini),
            "gpt-5.4-nano" => Ok(Self::Gpt54Nano),
            "gpt-5.4-mini-2026-03-17" => Ok(Self::Gpt54Mini20260317),
            "gpt-5.4-nano-2026-03-17" => Ok(Self::Gpt54Nano20260317),
            "gpt-5.3-chat-latest" => Ok(Self::Gpt53ChatLatest),
            "gpt-5.2" => Ok(Self::Gpt52),
            "gpt-5.2-2025-12-11" => Ok(Self::Gpt5220251211),
            "gpt-5.2-chat-latest" => Ok(Self::Gpt52ChatLatest),
            "gpt-5.2-pro" => Ok(Self::Gpt52Pro),
            "gpt-5.2-pro-2025-12-11" => Ok(Self::Gpt52Pro20251211),
            "gpt-5.1" => Ok(Self::Gpt51),
            "gpt-5.1-2025-11-13" => Ok(Self::Gpt5120251113),
            "gpt-5.1-codex" => Ok(Self::Gpt51Codex),
            "gpt-5.1-mini" => Ok(Self::Gpt51Mini),
            "gpt-5.1-chat-latest" => Ok(Self::Gpt51ChatLatest),
            "gpt-5" => Ok(Self::Gpt5),
            "gpt-5-mini" => Ok(Self::Gpt5Mini),
            "gpt-5-nano" => Ok(Self::Gpt5Nano),
            "gpt-5-2025-08-07" => Ok(Self::Gpt520250807),
            "gpt-5-mini-2025-08-07" => Ok(Self::Gpt5Mini20250807),
            "gpt-5-nano-2025-08-07" => Ok(Self::Gpt5Nano20250807),
            "gpt-5-chat-latest" => Ok(Self::Gpt5ChatLatest),
            "gpt-4.1" => Ok(Self::Gpt41),
            "gpt-4.1-mini" => Ok(Self::Gpt41Mini),
            "gpt-4.1-nano" => Ok(Self::Gpt41Nano),
            "gpt-4.1-2025-04-14" => Ok(Self::Gpt4120250414),
            "gpt-4.1-mini-2025-04-14" => Ok(Self::Gpt41Mini20250414),
            "gpt-4.1-nano-2025-04-14" => Ok(Self::Gpt41Nano20250414),
            "o4-mini" => Ok(Self::O4Mini),
            "o4-mini-2025-04-16" => Ok(Self::O4Mini20250416),
            "o3" => Ok(Self::O3),
            "o3-2025-04-16" => Ok(Self::O320250416),
            "o3-mini" => Ok(Self::O3Mini),
            "o3-mini-2025-01-31" => Ok(Self::O3Mini20250131),
            "o1" => Ok(Self::O1),
            "o1-2024-12-17" => Ok(Self::O120241217),
            "o1-preview" => Ok(Self::O1Preview),
            "o1-preview-2024-09-12" => Ok(Self::O1Preview20240912),
            "o1-mini" => Ok(Self::O1Mini),
            "o1-mini-2024-09-12" => Ok(Self::O1Mini20240912),
            "gpt-4o" => Ok(Self::Gpt4o),
            "gpt-4o-2024-11-20" => Ok(Self::Gpt4o20241120),
            "gpt-4o-2024-08-06" => Ok(Self::Gpt4o20240806),
            "gpt-4o-2024-05-13" => Ok(Self::Gpt4o20240513),
            "gpt-audio-mini" => Ok(Self::GptAudioMini),
            "gpt-audio-mini-2025-12-15" => Ok(Self::GptAudioMini20251215),
            "gpt-4o-audio-preview" => Ok(Self::Gpt4oAudioPreview),
            "gpt-4o-audio-preview-2024-10-01" => Ok(Self::Gpt4oAudioPreview20241001),
            "gpt-4o-audio-preview-2024-12-17" => Ok(Self::Gpt4oAudioPreview20241217),
            "gpt-4o-audio-preview-2025-06-03" => Ok(Self::Gpt4oAudioPreview20250603),
            "gpt-4o-mini-audio-preview" => Ok(Self::Gpt4oMiniAudioPreview),
            "gpt-4o-mini-audio-preview-2024-12-17" => Ok(Self::Gpt4oMiniAudioPreview20241217),
            "gpt-4o-search-preview" => Ok(Self::Gpt4oSearchPreview),
            "gpt-4o-mini-search-preview" => Ok(Self::Gpt4oMiniSearchPreview),
            "gpt-4o-search-preview-2025-03-11" => Ok(Self::Gpt4oSearchPreview20250311),
            "gpt-4o-mini-search-preview-2025-03-11" => Ok(Self::Gpt4oMiniSearchPreview20250311),
            "chatgpt-4o-latest" => Ok(Self::Chatgpt4oLatest),
            "codex-mini-latest" => Ok(Self::CodexMiniLatest),
            "gpt-4o-mini" => Ok(Self::Gpt4oMini),
            "gpt-4o-mini-2024-07-18" => Ok(Self::Gpt4oMini20240718),
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
            "gpt-3.5-turbo-0301" => Ok(Self::Gpt35Turbo0301),
            "gpt-3.5-turbo-0613" => Ok(Self::Gpt35Turbo0613),
            "gpt-3.5-turbo-1106" => Ok(Self::Gpt35Turbo1106),
            "gpt-3.5-turbo-0125" => Ok(Self::Gpt35Turbo0125),
            "gpt-3.5-turbo-16k-0613" => Ok(Self::Gpt35Turbo16k0613),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelIdsSharedSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelIdsSharedSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "Moderation results or errors for the response input and output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Moderation {
    #[doc = "Moderation for the response input."]
    pub input: ModerationInput,
    #[doc = "Moderation for the response output."]
    pub output: ModerationOutput,
}
#[doc = "The moderation policy for the response input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationConfigParam {
    pub mode: ModerationMode,
}
#[doc = "An error produced while attempting moderation for the response input or output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModerationErrorBody {
    #[doc = "The error code."]
    pub code: ::std::string::String,
    #[doc = "The error message."]
    pub message: ::std::string::String,
    #[doc = "The object type, which was always `error` for moderation failures."]
    #[serde(rename = "type")]
    pub type_: ModerationErrorBodyType,
}
#[doc = "The object type, which was always `error` for moderation failures."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ModerationErrorBodyType {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for ModerationErrorBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ModerationErrorBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModerationErrorBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModerationErrorBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Moderation for the response input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ModerationInput {
    ResultBody(ModerationResultBody),
    ErrorBody(ModerationErrorBody),
}
impl ::std::convert::From<ModerationResultBody> for ModerationInput {
    fn from(value: ModerationResultBody) -> Self {
        Self::ResultBody(value)
    }
}
impl ::std::convert::From<ModerationErrorBody> for ModerationInput {
    fn from(value: ModerationErrorBody) -> Self {
        Self::ErrorBody(value)
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
#[doc = "Moderation for the response output."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ModerationOutput {
    ResultBody(ModerationResultBody),
    ErrorBody(ModerationErrorBody),
}
impl ::std::convert::From<ModerationResultBody> for ModerationOutput {
    fn from(value: ModerationResultBody) -> Self {
        Self::ResultBody(value)
    }
}
impl ::std::convert::From<ModerationErrorBody> for ModerationOutput {
    fn from(value: ModerationErrorBody) -> Self {
        Self::ErrorBody(value)
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
#[doc = "A mouse move action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct MoveParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Specifies the event type. For a move action, this property is always set to `move`."]
    #[serde(rename = "type")]
    pub type_: MoveParamType,
    #[doc = "The x-coordinate to move to."]
    pub x: i64,
    #[doc = "The y-coordinate to move to."]
    pub y: i64,
}
#[doc = "Specifies the event type. For a move action, this property is always set to `move`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MoveParamType {
    #[serde(rename = "move")]
    Move,
}
impl ::std::fmt::Display for MoveParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Move => f.write_str("move"),
        }
    }
}
impl ::std::str::FromStr for MoveParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "move" => Ok(Self::Move),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MoveParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MoveParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Groups function/custom tools under a shared namespace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct NamespaceToolParam {
    #[doc = "A description of the namespace shown to the model."]
    pub description: ::std::string::String,
    #[doc = "The namespace name used in tool calls (for example, `crm`)."]
    pub name: NamespaceToolParamName,
    #[doc = "The function/custom tools available inside this namespace."]
    pub tools: ::std::vec::Vec<NamespaceToolParamToolsItem>,
    #[doc = "The type of the tool. Always `namespace`."]
    #[serde(rename = "type")]
    pub type_: NamespaceToolParamType,
}
#[doc = "The namespace name used in tool calls (for example, `crm`)."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NamespaceToolParamName(::std::string::String);
impl ::std::ops::Deref for NamespaceToolParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<NamespaceToolParamName> for ::std::string::String {
    fn from(value: NamespaceToolParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for NamespaceToolParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for NamespaceToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NamespaceToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for NamespaceToolParamName {
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
#[doc = "A function or custom tool that belongs to a namespace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum NamespaceToolParamToolsItem {
    FunctionToolParam(FunctionToolParam),
    CustomToolParam(CustomToolParam),
}
impl ::std::convert::From<FunctionToolParam> for NamespaceToolParamToolsItem {
    fn from(value: FunctionToolParam) -> Self {
        Self::FunctionToolParam(value)
    }
}
impl ::std::convert::From<CustomToolParam> for NamespaceToolParamToolsItem {
    fn from(value: CustomToolParam) -> Self {
        Self::CustomToolParam(value)
    }
}
#[doc = "The type of the tool. Always `namespace`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NamespaceToolParamType {
    #[serde(rename = "namespace")]
    Namespace,
}
impl ::std::fmt::Display for NamespaceToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Namespace => f.write_str("namespace"),
        }
    }
}
impl ::std::str::FromStr for NamespaceToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "namespace" => Ok(Self::Namespace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NamespaceToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NamespaceToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OutputContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum OutputContent {
    OutputTextContent(OutputTextContent),
    RefusalContent(RefusalContent),
    ReasoningTextContent(ReasoningTextContent),
}
impl ::std::convert::From<OutputTextContent> for OutputContent {
    fn from(value: OutputTextContent) -> Self {
        Self::OutputTextContent(value)
    }
}
impl ::std::convert::From<RefusalContent> for OutputContent {
    fn from(value: RefusalContent) -> Self {
        Self::RefusalContent(value)
    }
}
impl ::std::convert::From<ReasoningTextContent> for OutputContent {
    fn from(value: ReasoningTextContent) -> Self {
        Self::ReasoningTextContent(value)
    }
}
#[doc = "`OutputItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum OutputItem {
    OutputMessage(OutputMessage),
    FileSearchToolCall(FileSearchToolCall),
    FunctionToolCall(FunctionToolCall),
    FunctionToolCallOutputResource(FunctionToolCallOutputResource),
    WebSearchToolCall(WebSearchToolCall),
    ComputerToolCall(ComputerToolCall),
    ComputerToolCallOutputResource(ComputerToolCallOutputResource),
    ReasoningItem(ReasoningItem),
    Program(Program),
    ProgramOutput(ProgramOutput),
    ToolSearchCall(ToolSearchCall),
    ToolSearchOutput(ToolSearchOutput),
    AdditionalTools(AdditionalTools),
    CompactionBody(CompactionBody),
    ImageGenToolCall(ImageGenToolCall),
    CodeInterpreterToolCall(CodeInterpreterToolCall),
    LocalShellToolCall(LocalShellToolCall),
    LocalShellToolCallOutput(LocalShellToolCallOutput),
    FunctionShellCall(FunctionShellCall),
    FunctionShellCallOutput(FunctionShellCallOutput),
    ApplyPatchToolCall(ApplyPatchToolCall),
    ApplyPatchToolCallOutput(ApplyPatchToolCallOutput),
    McpToolCall(McpToolCall),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponseResource(McpApprovalResponseResource),
    CustomToolCall(CustomToolCall),
    CustomToolCallOutputResource(CustomToolCallOutputResource),
}
impl ::std::convert::From<OutputMessage> for OutputItem {
    fn from(value: OutputMessage) -> Self {
        Self::OutputMessage(value)
    }
}
impl ::std::convert::From<FileSearchToolCall> for OutputItem {
    fn from(value: FileSearchToolCall) -> Self {
        Self::FileSearchToolCall(value)
    }
}
impl ::std::convert::From<FunctionToolCall> for OutputItem {
    fn from(value: FunctionToolCall) -> Self {
        Self::FunctionToolCall(value)
    }
}
impl ::std::convert::From<FunctionToolCallOutputResource> for OutputItem {
    fn from(value: FunctionToolCallOutputResource) -> Self {
        Self::FunctionToolCallOutputResource(value)
    }
}
impl ::std::convert::From<WebSearchToolCall> for OutputItem {
    fn from(value: WebSearchToolCall) -> Self {
        Self::WebSearchToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCall> for OutputItem {
    fn from(value: ComputerToolCall) -> Self {
        Self::ComputerToolCall(value)
    }
}
impl ::std::convert::From<ComputerToolCallOutputResource> for OutputItem {
    fn from(value: ComputerToolCallOutputResource) -> Self {
        Self::ComputerToolCallOutputResource(value)
    }
}
impl ::std::convert::From<ReasoningItem> for OutputItem {
    fn from(value: ReasoningItem) -> Self {
        Self::ReasoningItem(value)
    }
}
impl ::std::convert::From<Program> for OutputItem {
    fn from(value: Program) -> Self {
        Self::Program(value)
    }
}
impl ::std::convert::From<ProgramOutput> for OutputItem {
    fn from(value: ProgramOutput) -> Self {
        Self::ProgramOutput(value)
    }
}
impl ::std::convert::From<ToolSearchCall> for OutputItem {
    fn from(value: ToolSearchCall) -> Self {
        Self::ToolSearchCall(value)
    }
}
impl ::std::convert::From<ToolSearchOutput> for OutputItem {
    fn from(value: ToolSearchOutput) -> Self {
        Self::ToolSearchOutput(value)
    }
}
impl ::std::convert::From<AdditionalTools> for OutputItem {
    fn from(value: AdditionalTools) -> Self {
        Self::AdditionalTools(value)
    }
}
impl ::std::convert::From<CompactionBody> for OutputItem {
    fn from(value: CompactionBody) -> Self {
        Self::CompactionBody(value)
    }
}
impl ::std::convert::From<ImageGenToolCall> for OutputItem {
    fn from(value: ImageGenToolCall) -> Self {
        Self::ImageGenToolCall(value)
    }
}
impl ::std::convert::From<CodeInterpreterToolCall> for OutputItem {
    fn from(value: CodeInterpreterToolCall) -> Self {
        Self::CodeInterpreterToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCall> for OutputItem {
    fn from(value: LocalShellToolCall) -> Self {
        Self::LocalShellToolCall(value)
    }
}
impl ::std::convert::From<LocalShellToolCallOutput> for OutputItem {
    fn from(value: LocalShellToolCallOutput) -> Self {
        Self::LocalShellToolCallOutput(value)
    }
}
impl ::std::convert::From<FunctionShellCall> for OutputItem {
    fn from(value: FunctionShellCall) -> Self {
        Self::FunctionShellCall(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutput> for OutputItem {
    fn from(value: FunctionShellCallOutput) -> Self {
        Self::FunctionShellCallOutput(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCall> for OutputItem {
    fn from(value: ApplyPatchToolCall) -> Self {
        Self::ApplyPatchToolCall(value)
    }
}
impl ::std::convert::From<ApplyPatchToolCallOutput> for OutputItem {
    fn from(value: ApplyPatchToolCallOutput) -> Self {
        Self::ApplyPatchToolCallOutput(value)
    }
}
impl ::std::convert::From<McpToolCall> for OutputItem {
    fn from(value: McpToolCall) -> Self {
        Self::McpToolCall(value)
    }
}
impl ::std::convert::From<McpListTools> for OutputItem {
    fn from(value: McpListTools) -> Self {
        Self::McpListTools(value)
    }
}
impl ::std::convert::From<McpApprovalRequest> for OutputItem {
    fn from(value: McpApprovalRequest) -> Self {
        Self::McpApprovalRequest(value)
    }
}
impl ::std::convert::From<McpApprovalResponseResource> for OutputItem {
    fn from(value: McpApprovalResponseResource) -> Self {
        Self::McpApprovalResponseResource(value)
    }
}
impl ::std::convert::From<CustomToolCall> for OutputItem {
    fn from(value: CustomToolCall) -> Self {
        Self::CustomToolCall(value)
    }
}
impl ::std::convert::From<CustomToolCallOutputResource> for OutputItem {
    fn from(value: CustomToolCallOutputResource) -> Self {
        Self::CustomToolCallOutputResource(value)
    }
}
#[doc = "An output message from the model.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OutputMessage {
    #[doc = "The content of the output message.\n"]
    pub content: ::std::vec::Vec<OutputMessageContent>,
    #[doc = "The unique ID of the output message.\n"]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<MessagePhase>,
    #[doc = "The role of the output message. Always `assistant`.\n"]
    pub role: OutputMessageRole,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    pub status: OutputMessageStatus,
    #[doc = "The type of the output message. Always `message`.\n"]
    #[serde(rename = "type")]
    pub type_: OutputMessageType,
}
#[doc = "`OutputMessageContent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum OutputMessageContent {
    OutputTextContent(OutputTextContent),
    RefusalContent(RefusalContent),
}
impl ::std::convert::From<OutputTextContent> for OutputMessageContent {
    fn from(value: OutputTextContent) -> Self {
        Self::OutputTextContent(value)
    }
}
impl ::std::convert::From<RefusalContent> for OutputMessageContent {
    fn from(value: RefusalContent) -> Self {
        Self::RefusalContent(value)
    }
}
#[doc = "The role of the output message. Always `assistant`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OutputMessageRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for OutputMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for OutputMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OutputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OutputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OutputMessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for OutputMessageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for OutputMessageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OutputMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OutputMessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the output message. Always `message`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OutputMessageType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for OutputMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for OutputMessageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OutputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OutputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text output from the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OutputTextContent {
    #[doc = "The annotations of the text output."]
    pub annotations: ::std::vec::Vec<Annotation>,
    pub logprobs: ::std::vec::Vec<LogProb>,
    #[doc = "The text output from the model."]
    pub text: ::std::string::String,
    #[doc = "The type of the output text. Always `output_text`."]
    #[serde(rename = "type")]
    pub type_: OutputTextContentType,
}
#[doc = "The type of the output text. Always `output_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OutputTextContentType {
    #[serde(rename = "output_text")]
    OutputText,
}
impl ::std::fmt::Display for OutputTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OutputText => f.write_str("output_text"),
        }
    }
}
impl ::std::str::FromStr for OutputTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "output_text" => Ok(Self::OutputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OutputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OutputTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PersonalityEnum`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PersonalityEnum {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<PersonalityEnumSubtype1>,
}
#[doc = "`PersonalityEnumSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PersonalityEnumSubtype1 {
    #[serde(rename = "friendly")]
    Friendly,
    #[serde(rename = "pragmatic")]
    Pragmatic,
}
impl ::std::fmt::Display for PersonalityEnumSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Friendly => f.write_str("friendly"),
            Self::Pragmatic => f.write_str("pragmatic"),
        }
    }
}
impl ::std::str::FromStr for PersonalityEnumSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "friendly" => Ok(Self::Friendly),
            "pragmatic" => Ok(Self::Pragmatic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PersonalityEnumSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PersonalityEnumSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Program`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Program {
    #[doc = "The stable call ID of the program item."]
    pub call_id: ::std::string::String,
    #[doc = "The JavaScript source executed by programmatic tool calling."]
    pub code: ::std::string::String,
    #[doc = "Opaque program replay fingerprint that must be round-tripped."]
    pub fingerprint: ::std::string::String,
    #[doc = "The unique ID of the program item."]
    pub id: ::std::string::String,
    #[doc = "The type of the item. Always `program`."]
    #[serde(rename = "type")]
    pub type_: ProgramType,
}
#[doc = "`ProgramItemParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgramItemParam {
    #[doc = "The stable call ID of the program item."]
    pub call_id: ProgramItemParamCallId,
    #[doc = "The JavaScript source executed by programmatic tool calling."]
    pub code: ProgramItemParamCode,
    #[doc = "Opaque program replay fingerprint that must be round-tripped."]
    pub fingerprint: ProgramItemParamFingerprint,
    #[doc = "The unique ID of this program item."]
    pub id: ::std::string::String,
    #[doc = "The item type. Always `program`."]
    #[serde(rename = "type")]
    pub type_: ProgramItemParamType,
}
#[doc = "The stable call ID of the program item."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ProgramItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramItemParamCallId> for ::std::string::String {
    fn from(value: ProgramItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ProgramItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramItemParamCallId {
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
#[doc = "The JavaScript source executed by programmatic tool calling."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramItemParamCode(::std::string::String);
impl ::std::ops::Deref for ProgramItemParamCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramItemParamCode> for ::std::string::String {
    fn from(value: ProgramItemParamCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramItemParamCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProgramItemParamCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramItemParamCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramItemParamCode {
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
#[doc = "Opaque program replay fingerprint that must be round-tripped."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramItemParamFingerprint(::std::string::String);
impl ::std::ops::Deref for ProgramItemParamFingerprint {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramItemParamFingerprint> for ::std::string::String {
    fn from(value: ProgramItemParamFingerprint) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramItemParamFingerprint {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProgramItemParamFingerprint {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramItemParamFingerprint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramItemParamFingerprint {
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
#[doc = "The item type. Always `program`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramItemParamType {
    #[serde(rename = "program")]
    Program,
}
impl ::std::fmt::Display for ProgramItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Program => f.write_str("program"),
        }
    }
}
impl ::std::str::FromStr for ProgramItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program" => Ok(Self::Program),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgramOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgramOutput {
    #[doc = "The call ID of the program item."]
    pub call_id: ::std::string::String,
    #[doc = "The unique ID of the program output item."]
    pub id: ::std::string::String,
    #[doc = "The result produced by the program item."]
    pub result: ::std::string::String,
    #[doc = "The terminal status of the program output item."]
    pub status: ProgramOutputStatus,
    #[doc = "The type of the item. Always `program_output`."]
    #[serde(rename = "type")]
    pub type_: ProgramOutputType,
}
#[doc = "`ProgramOutputItemParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgramOutputItemParam {
    #[doc = "The call ID of the program item."]
    pub call_id: ProgramOutputItemParamCallId,
    #[doc = "The unique ID of this program output item."]
    pub id: ::std::string::String,
    #[doc = "The result produced by the program item."]
    pub result: ProgramOutputItemParamResult,
    #[doc = "The terminal status of the program output."]
    pub status: ProgramOutputItemStatus,
    #[doc = "The item type. Always `program_output`."]
    #[serde(rename = "type")]
    pub type_: ProgramOutputItemParamType,
}
#[doc = "The call ID of the program item."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ProgramOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramOutputItemParamCallId> for ::std::string::String {
    fn from(value: ProgramOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ProgramOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramOutputItemParamCallId {
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
#[doc = "The result produced by the program item."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramOutputItemParamResult(::std::string::String);
impl ::std::ops::Deref for ProgramOutputItemParamResult {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramOutputItemParamResult> for ::std::string::String {
    fn from(value: ProgramOutputItemParamResult) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramOutputItemParamResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProgramOutputItemParamResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputItemParamResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramOutputItemParamResult {
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
#[doc = "The item type. Always `program_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramOutputItemParamType {
    #[serde(rename = "program_output")]
    ProgramOutput,
}
impl ::std::fmt::Display for ProgramOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgramOutput => f.write_str("program_output"),
        }
    }
}
impl ::std::str::FromStr for ProgramOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program_output" => Ok(Self::ProgramOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgramOutputItemStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramOutputItemStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ProgramOutputItemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ProgramOutputItemStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramOutputItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgramOutputStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramOutputStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ProgramOutputStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ProgramOutputStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `program_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramOutputType {
    #[serde(rename = "program_output")]
    ProgramOutput,
}
impl ::std::fmt::Display for ProgramOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgramOutput => f.write_str("program_output"),
        }
    }
}
impl ::std::str::FromStr for ProgramOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program_output" => Ok(Self::ProgramOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgramToolCallCaller`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgramToolCallCaller {
    #[doc = "The call ID of the program item that produced this tool call."]
    pub caller_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ProgramToolCallCallerType,
}
#[doc = "`ProgramToolCallCallerParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgramToolCallCallerParam {
    #[doc = "The call ID of the program item that produced this tool call."]
    pub caller_id: ProgramToolCallCallerParamCallerId,
    #[doc = "The caller type. Always `program`."]
    #[serde(rename = "type")]
    pub type_: ProgramToolCallCallerParamType,
}
#[doc = "The call ID of the program item that produced this tool call."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProgramToolCallCallerParamCallerId(::std::string::String);
impl ::std::ops::Deref for ProgramToolCallCallerParamCallerId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProgramToolCallCallerParamCallerId> for ::std::string::String {
    fn from(value: ProgramToolCallCallerParamCallerId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProgramToolCallCallerParamCallerId {
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
impl ::std::convert::TryFrom<&str> for ProgramToolCallCallerParamCallerId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramToolCallCallerParamCallerId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProgramToolCallCallerParamCallerId {
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
#[doc = "The caller type. Always `program`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramToolCallCallerParamType {
    #[serde(rename = "program")]
    Program,
}
impl ::std::fmt::Display for ProgramToolCallCallerParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Program => f.write_str("program"),
        }
    }
}
impl ::std::str::FromStr for ProgramToolCallCallerParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program" => Ok(Self::Program),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramToolCallCallerParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramToolCallCallerParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgramToolCallCallerType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramToolCallCallerType {
    #[serde(rename = "program")]
    Program,
}
impl ::std::fmt::Display for ProgramToolCallCallerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Program => f.write_str("program"),
        }
    }
}
impl ::std::str::FromStr for ProgramToolCallCallerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program" => Ok(Self::Program),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramToolCallCallerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramToolCallCallerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `program`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgramType {
    #[serde(rename = "program")]
    Program,
}
impl ::std::fmt::Display for ProgramType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Program => f.write_str("program"),
        }
    }
}
impl ::std::str::FromStr for ProgramType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "program" => Ok(Self::Program),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgramType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgramType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProgrammaticToolCallingParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProgrammaticToolCallingParam {
    #[doc = "The type of the tool. Always `programmatic_tool_calling`."]
    #[serde(rename = "type")]
    pub type_: ProgrammaticToolCallingParamType,
}
#[doc = "The type of the tool. Always `programmatic_tool_calling`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProgrammaticToolCallingParamType {
    #[serde(rename = "programmatic_tool_calling")]
    ProgrammaticToolCalling,
}
impl ::std::fmt::Display for ProgrammaticToolCallingParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgrammaticToolCalling => f.write_str("programmatic_tool_calling"),
        }
    }
}
impl ::std::str::FromStr for ProgrammaticToolCallingParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "programmatic_tool_calling" => Ok(Self::ProgrammaticToolCalling),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProgrammaticToolCallingParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProgrammaticToolCallingParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Prompt`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Prompt(pub ::std::option::Option<PromptValue>);
impl ::std::ops::Deref for Prompt {
    type Target = ::std::option::Option<PromptValue>;
    fn deref(&self) -> &::std::option::Option<PromptValue> {
        &self.0
    }
}
impl ::std::convert::From<Prompt> for ::std::option::Option<PromptValue> {
    fn from(value: Prompt) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PromptValue>> for Prompt {
    fn from(value: ::std::option::Option<PromptValue>) -> Self {
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
#[doc = "`PromptCacheComparisonResponseNotFoundDiagnosticsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheComparisonResponseNotFoundDiagnosticsBody {
    #[serde(rename = "type")]
    pub type_: PromptCacheComparisonResponseNotFoundDiagnosticsBodyType,
}
#[doc = "`PromptCacheComparisonResponseNotFoundDiagnosticsBodyType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PromptCacheComparisonResponseNotFoundDiagnosticsBodyType {
    #[serde(rename = "comparison_response_not_found")]
    ComparisonResponseNotFound,
}
impl ::std::fmt::Display for PromptCacheComparisonResponseNotFoundDiagnosticsBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ComparisonResponseNotFound => f.write_str("comparison_response_not_found"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheComparisonResponseNotFoundDiagnosticsBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "comparison_response_not_found" => Ok(Self::ComparisonResponseNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheComparisonResponseNotFoundDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for PromptCacheComparisonResponseNotFoundDiagnosticsBodyType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Prompt cache diagnostics requested for this response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PromptCacheDiagnostics {
    MissDiagnosticsBody(PromptCacheMissDiagnosticsBody),
    HitDiagnosticsBody(PromptCacheHitDiagnosticsBody),
    ComparisonResponseNotFoundDiagnosticsBody(PromptCacheComparisonResponseNotFoundDiagnosticsBody),
    UnavailableDiagnosticsBody(PromptCacheUnavailableDiagnosticsBody),
}
impl ::std::convert::From<PromptCacheMissDiagnosticsBody> for PromptCacheDiagnostics {
    fn from(value: PromptCacheMissDiagnosticsBody) -> Self {
        Self::MissDiagnosticsBody(value)
    }
}
impl ::std::convert::From<PromptCacheHitDiagnosticsBody> for PromptCacheDiagnostics {
    fn from(value: PromptCacheHitDiagnosticsBody) -> Self {
        Self::HitDiagnosticsBody(value)
    }
}
impl ::std::convert::From<PromptCacheComparisonResponseNotFoundDiagnosticsBody>
    for PromptCacheDiagnostics
{
    fn from(value: PromptCacheComparisonResponseNotFoundDiagnosticsBody) -> Self {
        Self::ComparisonResponseNotFoundDiagnosticsBody(value)
    }
}
impl ::std::convert::From<PromptCacheUnavailableDiagnosticsBody> for PromptCacheDiagnostics {
    fn from(value: PromptCacheUnavailableDiagnosticsBody) -> Self {
        Self::UnavailableDiagnosticsBody(value)
    }
}
#[doc = "`PromptCacheHitDiagnosticsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheHitDiagnosticsBody {
    #[serde(rename = "type")]
    pub type_: PromptCacheHitDiagnosticsBodyType,
}
#[doc = "`PromptCacheHitDiagnosticsBodyType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PromptCacheHitDiagnosticsBodyType {
    #[serde(rename = "cache_hit")]
    CacheHit,
}
impl ::std::fmt::Display for PromptCacheHitDiagnosticsBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CacheHit => f.write_str("cache_hit"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheHitDiagnosticsBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cache_hit" => Ok(Self::CacheHit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheHitDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheHitDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PromptCacheMissDiagnosticsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheMissDiagnosticsBody {
    #[doc = "The estimated number of input tokens affected after the first detected divergence."]
    pub cache_missed_tokens: i64,
    #[doc = "The raw token count of the reusable prefix in the compared response."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comparison_reusable_tokens: ::std::option::Option<i64>,
    #[doc = "The reason prompt cache reuse did not occur."]
    pub reason: CacheMissReasonTypeEnum,
    #[serde(rename = "type")]
    pub type_: PromptCacheMissDiagnosticsBodyType,
}
#[doc = "`PromptCacheMissDiagnosticsBodyType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PromptCacheMissDiagnosticsBodyType {
    #[serde(rename = "cache_miss")]
    CacheMiss,
}
impl ::std::fmt::Display for PromptCacheMissDiagnosticsBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CacheMiss => f.write_str("cache_miss"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheMissDiagnosticsBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cache_miss" => Ok(Self::CacheMiss),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheMissDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheMissDiagnosticsBodyType {
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
#[doc = "The prompt-caching options that were applied to the response. Supported for `gpt-5.6` and later models."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheOptions {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comparison_response_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether implicit prompt-cache breakpoints were enabled."]
    pub mode: PromptCacheModeEnum,
    #[doc = "The minimum lifetime applied to each cache breakpoint."]
    pub ttl: PromptCacheTtlEnum,
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
#[doc = "`PromptCacheRetentionEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PromptCacheRetentionEnum {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for PromptCacheRetentionEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheRetentionEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheRetentionEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheRetentionEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "`PromptCacheUnavailableDiagnosticsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptCacheUnavailableDiagnosticsBody {
    #[serde(rename = "type")]
    pub type_: PromptCacheUnavailableDiagnosticsBodyType,
}
#[doc = "`PromptCacheUnavailableDiagnosticsBodyType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PromptCacheUnavailableDiagnosticsBodyType {
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::fmt::Display for PromptCacheUnavailableDiagnosticsBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unavailable => f.write_str("unavailable"),
        }
    }
}
impl ::std::str::FromStr for PromptCacheUnavailableDiagnosticsBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unavailable" => Ok(Self::Unavailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptCacheUnavailableDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptCacheUnavailableDiagnosticsBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reference to a prompt template and its variables.\n[Learn more](https://developers.openai.com/api/docs/guides/text?api-mode=responses#version-prompts-in-code).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PromptValue {
    #[doc = "The unique identifier of the prompt template to use."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub variables: ::std::option::Option<ResponsePromptVariables>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
#[doc = "`RankerVersionType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RankerVersionType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}
impl ::std::fmt::Display for RankerVersionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default20241115 => f.write_str("default-2024-11-15"),
        }
    }
}
impl ::std::str::FromStr for RankerVersionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default-2024-11-15" => Ok(Self::Default20241115),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RankerVersionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RankerVersionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RankingOptions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RankingOptions {
    #[doc = "Weights that control how reciprocal rank fusion balances semantic embedding matches versus sparse keyword matches when hybrid search is enabled."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hybrid_search: ::std::option::Option<HybridSearchOptions>,
    #[doc = "The ranker to use for the file search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ranker: ::std::option::Option<RankerVersionType>,
    #[doc = "The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_threshold: ::std::option::Option<f64>,
}
#[doc = "Configuration options for\n[reasoning models](https://developers.openai.com/api/docs/guides/reasoning).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct Reasoning {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<ReasoningContext>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<ReasoningEffort>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub generate_summary: ::std::option::Option<ReasoningGenerateSummary>,
    #[doc = "Controls the reasoning execution mode for the request.\n\nWhen returned on a response, this is the effective execution mode.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<ReasoningModeEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<ReasoningSummary>,
}
#[doc = "Controls which reasoning items are rendered back to the model on later turns.\nIf omitted or set to `auto`, the model determines the context mode. The\n`gpt-5.6` model family defaults to `all_turns`; earlier models default to\n`current_turn`.\n\nWhen returned on a response, this is the effective reasoning context mode\nused for the response.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningContext {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "current_turn")]
    CurrentTurn,
    #[serde(rename = "all_turns")]
    AllTurns,
}
impl ::std::fmt::Display for ReasoningContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::CurrentTurn => f.write_str("current_turn"),
            Self::AllTurns => f.write_str("all_turns"),
        }
    }
}
impl ::std::str::FromStr for ReasoningContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "current_turn" => Ok(Self::CurrentTurn),
            "all_turns" => Ok(Self::AllTurns),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningContext {
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
#[doc = "**Deprecated:** use `summary` instead.\n\nA summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningGenerateSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}
impl ::std::fmt::Display for ReasoningGenerateSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Concise => f.write_str("concise"),
            Self::Detailed => f.write_str("detailed"),
        }
    }
}
impl ::std::str::FromStr for ReasoningGenerateSummary {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningGenerateSummary {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningGenerateSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A description of the chain of thought used by a reasoning model while generating\na response. Be sure to include these items in your `input` to the Responses API\nfor subsequent turns of a conversation if you are manually\n[managing context](https://developers.openai.com/api/docs/guides/conversation-state).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ReasoningItem {
    #[doc = "Reasoning text content.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub content: ::std::vec::Vec<ReasoningTextContent>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encrypted_content: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identifier of the reasoning content.\n"]
    pub id: ::std::string::String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ReasoningItemStatus>,
    #[doc = "Reasoning summary content.\n"]
    pub summary: ::std::vec::Vec<SummaryTextContent>,
    #[doc = "The type of the object. Always `reasoning`.\n"]
    #[serde(rename = "type")]
    pub type_: ReasoningItemType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningItemStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ReasoningItemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ReasoningItemStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the object. Always `reasoning`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningItemType {
    #[serde(rename = "reasoning")]
    Reasoning,
}
impl ::std::fmt::Display for ReasoningItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reasoning => f.write_str("reasoning"),
        }
    }
}
impl ::std::str::FromStr for ReasoningItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reasoning" => Ok(Self::Reasoning),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ReasoningModeEnum`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ReasoningModeEnum {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ReasoningModeEnumSubtype1>,
}
#[doc = "`ReasoningModeEnumSubtype1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningModeEnumSubtype1 {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "pro")]
    Pro,
}
impl ::std::fmt::Display for ReasoningModeEnumSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::Pro => f.write_str("pro"),
        }
    }
}
impl ::std::str::FromStr for ReasoningModeEnumSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "pro" => Ok(Self::Pro),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningModeEnumSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningModeEnumSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n\n`concise` is supported for `computer-use-preview` models and all reasoning models after `gpt-5`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}
impl ::std::fmt::Display for ReasoningSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Concise => f.write_str("concise"),
            Self::Detailed => f.write_str("detailed"),
        }
    }
}
impl ::std::str::FromStr for ReasoningSummary {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "concise" => Ok(Self::Concise),
            "detailed" => Ok(Self::Detailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningSummary {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reasoning text from the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ReasoningTextContent {
    #[doc = "The reasoning text from the model."]
    pub text: ::std::string::String,
    #[doc = "The type of the reasoning text. Always `reasoning_text`."]
    #[serde(rename = "type")]
    pub type_: ReasoningTextContentType,
}
#[doc = "The type of the reasoning text. Always `reasoning_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ReasoningTextContentType {
    #[serde(rename = "reasoning_text")]
    ReasoningText,
}
impl ::std::fmt::Display for ReasoningTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ReasoningText => f.write_str("reasoning_text"),
        }
    }
}
impl ::std::str::FromStr for ReasoningTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reasoning_text" => Ok(Self::ReasoningText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReasoningTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReasoningTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A refusal from the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RefusalContent {
    #[doc = "The refusal explanation from the model."]
    pub refusal: ::std::string::String,
    #[doc = "The type of the refusal. Always `refusal`."]
    #[serde(rename = "type")]
    pub type_: RefusalContentType,
}
#[doc = "The type of the refusal. Always `refusal`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RefusalContentType {
    #[serde(rename = "refusal")]
    Refusal,
}
impl ::std::fmt::Display for RefusalContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Refusal => f.write_str("refusal"),
        }
    }
}
impl ::std::str::FromStr for RefusalContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "refusal" => Ok(Self::Refusal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RefusalContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RefusalContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Response`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub access_programs: ::std::option::Option<AccessProgramsBody>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed_at: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conversation: ::std::option::Option<ResponseConversation>,
    #[doc = "Unix timestamp (in seconds) of when this Response was created.\n"]
    pub created_at: f64,
    pub error: ResponseError,
    #[doc = "Unique identifier for this Response.\n"]
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub incomplete_details: ::std::option::Option<ResponseIncompleteDetails>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub instructions: ::std::option::Option<ResponseInstructions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_output_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tool_calls: ::std::option::Option<i64>,
    pub metadata: Metadata,
    #[doc = "Model ID used to generate the response, like `gpt-6-astra`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models)\nto browse and compare available models.\n"]
    pub model: ModelIdsResponses,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<Moderation>,
    #[doc = "The object type of this resource - always set to `response`.\n"]
    pub object: ResponseObject,
    #[doc = "An array of content items generated by the model.\n\n- The length and order of items in the `output` array is dependent\n  on the model's response.\n- Rather than accessing the first item in the `output` array and\n  assuming it's an `assistant` message with the content generated by\n  the model, you might consider using the `output_text` property where\n  supported in SDKs.\n"]
    pub output: ::std::vec::Vec<OutputItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_text: ::std::option::Option<::std::string::String>,
    #[doc = "Whether to allow the model to run tool calls in parallel.\n"]
    pub parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_response_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_diagnostics: ::std::option::Option<PromptCacheDiagnostics>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_key: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_options: ::std::option::Option<PromptCacheOptions>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt_cache_retention: ::std::option::Option<ResponsePromptCacheRetention>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<Reasoning>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<ResponseSafetyIdentifier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<ServiceTierResponses>,
    #[doc = "The status of the response generation. One of `completed`, `failed`,\n`in_progress`, `cancelled`, `queued`, or `incomplete`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ResponseStatus>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub temperature: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<ResponseTextParam>,
    pub tool_choice: ToolChoiceParam,
    pub tools: ToolsArray,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub top_p: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<ResponseTruncation>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<ResponseUsage>,
    #[doc = "This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead to maintain caching optimizations.\nA stable identifier for your end-users.\nUsed to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "Emitted when there is a partial audio response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseAudioDeltaEvent {
    #[doc = "A chunk of Base64 encoded response audio bytes.\n"]
    pub delta: ::std::string::String,
    #[doc = "A sequence number for this chunk of the stream response.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.audio.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseAudioDeltaEventType,
}
#[doc = "The type of the event. Always `response.audio.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseAudioDeltaEventType {
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta,
}
impl ::std::fmt::Display for ResponseAudioDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseAudioDelta => f.write_str("response.audio.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseAudioDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.audio.delta" => Ok(Self::ResponseAudioDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseAudioDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseAudioDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the audio response is complete."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseAudioDoneEvent {
    pub response_id: ::serde_json::Value,
    #[doc = "The sequence number of the delta.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.audio.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseAudioDoneEventType,
}
#[doc = "The type of the event. Always `response.audio.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseAudioDoneEventType {
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone,
}
impl ::std::fmt::Display for ResponseAudioDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseAudioDone => f.write_str("response.audio.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseAudioDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.audio.done" => Ok(Self::ResponseAudioDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseAudioDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseAudioDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when there is a partial transcript of audio."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseAudioTranscriptDeltaEvent {
    #[doc = "The partial transcript of the audio response.\n"]
    pub delta: ::std::string::String,
    pub response_id: ::serde_json::Value,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseAudioTranscriptDeltaEventType,
}
#[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseAudioTranscriptDeltaEventType {
    #[serde(rename = "response.audio.transcript.delta")]
    ResponseAudioTranscriptDelta,
}
impl ::std::fmt::Display for ResponseAudioTranscriptDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseAudioTranscriptDelta => f.write_str("response.audio.transcript.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseAudioTranscriptDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.audio.transcript.delta" => Ok(Self::ResponseAudioTranscriptDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseAudioTranscriptDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseAudioTranscriptDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the full audio transcript is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseAudioTranscriptDoneEvent {
    pub response_id: ::serde_json::Value,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseAudioTranscriptDoneEventType,
}
#[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseAudioTranscriptDoneEventType {
    #[serde(rename = "response.audio.transcript.done")]
    ResponseAudioTranscriptDone,
}
impl ::std::fmt::Display for ResponseAudioTranscriptDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseAudioTranscriptDone => f.write_str("response.audio.transcript.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseAudioTranscriptDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.audio.transcript.done" => Ok(Self::ResponseAudioTranscriptDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseAudioTranscriptDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseAudioTranscriptDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a partial code snippet is streamed by the code interpreter."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    #[doc = "The partial code snippet being streamed by the code interpreter."]
    pub delta: ::std::string::String,
    #[doc = "The unique identifier of the code interpreter tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response for which the code is being streamed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event, used to order streaming events."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.code_interpreter_call_code.delta`."]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCodeDeltaEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call_code.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCodeInterpreterCallCodeDeltaEventType {
    #[serde(rename = "response.code_interpreter_call_code.delta")]
    ResponseCodeInterpreterCallCodeDelta,
}
impl ::std::fmt::Display for ResponseCodeInterpreterCallCodeDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCodeInterpreterCallCodeDelta => {
                f.write_str("response.code_interpreter_call_code.delta")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCodeInterpreterCallCodeDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.code_interpreter_call_code.delta" => {
                Ok(Self::ResponseCodeInterpreterCallCodeDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCodeInterpreterCallCodeDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCodeInterpreterCallCodeDeltaEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the code snippet is finalized by the code interpreter."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    #[doc = "The final code snippet output by the code interpreter."]
    pub code: ::std::string::String,
    #[doc = "The unique identifier of the code interpreter tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response for which the code is finalized."]
    pub output_index: i64,
    #[doc = "The sequence number of this event, used to order streaming events."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.code_interpreter_call_code.done`."]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCodeDoneEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call_code.done`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCodeInterpreterCallCodeDoneEventType {
    #[serde(rename = "response.code_interpreter_call_code.done")]
    ResponseCodeInterpreterCallCodeDone,
}
impl ::std::fmt::Display for ResponseCodeInterpreterCallCodeDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCodeInterpreterCallCodeDone => {
                f.write_str("response.code_interpreter_call_code.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCodeInterpreterCallCodeDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.code_interpreter_call_code.done" => {
                Ok(Self::ResponseCodeInterpreterCallCodeDone)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCodeInterpreterCallCodeDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCodeInterpreterCallCodeDoneEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the code interpreter call is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    #[doc = "The unique identifier of the code interpreter tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response for which the code interpreter call is completed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event, used to order streaming events."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.completed`."]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.completed`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCodeInterpreterCallCompletedEventType {
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted,
}
impl ::std::fmt::Display for ResponseCodeInterpreterCallCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCodeInterpreterCallCompleted => {
                f.write_str("response.code_interpreter_call.completed")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCodeInterpreterCallCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.code_interpreter_call.completed" => {
                Ok(Self::ResponseCodeInterpreterCallCompleted)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCodeInterpreterCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCodeInterpreterCallCompletedEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a code interpreter call is in progress."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    #[doc = "The unique identifier of the code interpreter tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response for which the code interpreter call is in progress."]
    pub output_index: i64,
    #[doc = "The sequence number of this event, used to order streaming events."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`."]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCodeInterpreterCallInProgressEventType {
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress,
}
impl ::std::fmt::Display for ResponseCodeInterpreterCallInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCodeInterpreterCallInProgress => {
                f.write_str("response.code_interpreter_call.in_progress")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCodeInterpreterCallInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.code_interpreter_call.in_progress" => {
                Ok(Self::ResponseCodeInterpreterCallInProgress)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCodeInterpreterCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCodeInterpreterCallInProgressEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the code interpreter is actively interpreting the code snippet."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    #[doc = "The unique identifier of the code interpreter tool call item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response for which the code interpreter is interpreting code."]
    pub output_index: i64,
    #[doc = "The sequence number of this event, used to order streaming events."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`."]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallInterpretingEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCodeInterpreterCallInterpretingEventType {
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting,
}
impl ::std::fmt::Display for ResponseCodeInterpreterCallInterpretingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCodeInterpreterCallInterpreting => {
                f.write_str("response.code_interpreter_call.interpreting")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCodeInterpreterCallInterpretingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.code_interpreter_call.interpreting" => {
                Ok(Self::ResponseCodeInterpreterCallInterpreting)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCodeInterpreterCallInterpretingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCodeInterpreterCallInterpretingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when new summary content is sampled for a compaction trigger. Contains no summary content."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCompactionCompactingStreamingEvent {
    #[doc = "The ID of the compaction output item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the compaction output item."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.compaction.compacting`."]
    #[serde(rename = "type")]
    pub type_: ResponseCompactionCompactingStreamingEventType,
}
#[doc = "The type of the event, always `response.compaction.compacting`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCompactionCompactingStreamingEventType {
    #[serde(rename = "response.compaction.compacting")]
    ResponseCompactionCompacting,
}
impl ::std::fmt::Display for ResponseCompactionCompactingStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCompactionCompacting => f.write_str("response.compaction.compacting"),
        }
    }
}
impl ::std::str::FromStr for ResponseCompactionCompactingStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.compaction.compacting" => Ok(Self::ResponseCompactionCompacting),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCompactionCompactingStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseCompactionCompactingStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the model response is complete."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCompletedEvent {
    #[doc = "Properties of the completed response.\n"]
    pub response: Response,
    #[doc = "The sequence number for this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseCompletedEventType,
}
#[doc = "The type of the event. Always `response.completed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCompletedEventType {
    #[serde(rename = "response.completed")]
    ResponseCompleted,
}
impl ::std::fmt::Display for ResponseCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCompleted => f.write_str("response.completed"),
        }
    }
}
impl ::std::str::FromStr for ResponseCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.completed" => Ok(Self::ResponseCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A configuration update that applies to subsequent responses until it is\nreplaced by another configuration update.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseConfigurationUpdate {
    #[doc = "The unique ID of the configuration update item."]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<ResponseConfigurationUpdateReasoning>,
    #[doc = "The item type. Always `configuration_update`."]
    #[serde(rename = "type")]
    pub type_: ResponseConfigurationUpdateType,
}
#[doc = "An update to the conversation's response configuration. The configuration\nremains in effect for subsequent responses until it is replaced by another\nconfiguration update.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseConfigurationUpdateItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<ResponseConfigurationUpdateItemParamReasoning>,
    #[doc = "The item type. Always `configuration_update`."]
    #[serde(rename = "type")]
    pub type_: ResponseConfigurationUpdateItemParamType,
}
#[doc = "Updates to reasoning configuration. Only effort is supported."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseConfigurationUpdateItemParamReasoning {
    #[doc = "The reasoning effort to use for subsequent responses until another\nconfiguration update replaces it.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<ReasoningEffort>,
}
#[doc = "The item type. Always `configuration_update`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseConfigurationUpdateItemParamType {
    #[serde(rename = "configuration_update")]
    ConfigurationUpdate,
}
impl ::std::fmt::Display for ResponseConfigurationUpdateItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConfigurationUpdate => f.write_str("configuration_update"),
        }
    }
}
impl ::std::str::FromStr for ResponseConfigurationUpdateItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "configuration_update" => Ok(Self::ConfigurationUpdate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseConfigurationUpdateItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseConfigurationUpdateItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The reasoning configuration applied by this update."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseConfigurationUpdateReasoning {
    #[doc = "The reasoning effort used for subsequent responses until another\nconfiguration update replaces it.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effort: ::std::option::Option<ReasoningEffort>,
}
#[doc = "The item type. Always `configuration_update`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseConfigurationUpdateType {
    #[serde(rename = "configuration_update")]
    ConfigurationUpdate,
}
impl ::std::fmt::Display for ResponseConfigurationUpdateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ConfigurationUpdate => f.write_str("configuration_update"),
        }
    }
}
impl ::std::str::FromStr for ResponseConfigurationUpdateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "configuration_update" => Ok(Self::ConfigurationUpdate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseConfigurationUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseConfigurationUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a new content part is added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseContentPartAddedEvent {
    #[doc = "The index of the content part that was added.\n"]
    pub content_index: i64,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    pub output_index: i64,
    #[doc = "The content part that was added.\n"]
    pub part: OutputContent,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.content_part.added`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseContentPartAddedEventType,
}
#[doc = "The type of the event. Always `response.content_part.added`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseContentPartAddedEventType {
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}
impl ::std::fmt::Display for ResponseContentPartAddedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseContentPartAdded => f.write_str("response.content_part.added"),
        }
    }
}
impl ::std::str::FromStr for ResponseContentPartAddedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.content_part.added" => Ok(Self::ResponseContentPartAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseContentPartAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseContentPartAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a content part is done."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseContentPartDoneEvent {
    #[doc = "The index of the content part that is done.\n"]
    pub content_index: i64,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    pub output_index: i64,
    #[doc = "The content part that is done.\n"]
    pub part: OutputContent,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.content_part.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseContentPartDoneEventType,
}
#[doc = "The type of the event. Always `response.content_part.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseContentPartDoneEventType {
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}
impl ::std::fmt::Display for ResponseContentPartDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseContentPartDone => f.write_str("response.content_part.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseContentPartDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.content_part.done" => Ok(Self::ResponseContentPartDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseContentPartDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseContentPartDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The conversation that this response belonged to. Input items and output items from this response were automatically added to this conversation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseConversation {
    #[doc = "The unique ID of the conversation that this response was associated with."]
    pub id: ::std::string::String,
}
#[doc = "An event that is emitted when a response is created.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCreatedEvent {
    #[doc = "The response that was created.\n"]
    pub response: Response,
    #[doc = "The sequence number for this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.created`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseCreatedEventType,
}
#[doc = "The type of the event. Always `response.created`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCreatedEventType {
    #[serde(rename = "response.created")]
    ResponseCreated,
}
impl ::std::fmt::Display for ResponseCreatedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCreated => f.write_str("response.created"),
        }
    }
}
impl ::std::str::FromStr for ResponseCreatedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.created" => Ok(Self::ResponseCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCreatedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseCreatedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Event representing a delta (partial update) to the input of a custom tool call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCustomToolCallInputDeltaEvent {
    #[doc = "The incremental input data (delta) for the custom tool call."]
    pub delta: ::std::string::String,
    #[doc = "Unique identifier for the API item associated with this event."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output this delta applies to."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The event type identifier."]
    #[serde(rename = "type")]
    pub type_: ResponseCustomToolCallInputDeltaEventType,
}
#[doc = "The event type identifier."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCustomToolCallInputDeltaEventType {
    #[serde(rename = "response.custom_tool_call_input.delta")]
    ResponseCustomToolCallInputDelta,
}
impl ::std::fmt::Display for ResponseCustomToolCallInputDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCustomToolCallInputDelta => {
                f.write_str("response.custom_tool_call_input.delta")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCustomToolCallInputDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.custom_tool_call_input.delta" => Ok(Self::ResponseCustomToolCallInputDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCustomToolCallInputDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseCustomToolCallInputDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Event indicating that input for a custom tool call is complete.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseCustomToolCallInputDoneEvent {
    #[doc = "The complete input data for the custom tool call."]
    pub input: ::std::string::String,
    #[doc = "Unique identifier for the API item associated with this event."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output this event applies to."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The event type identifier."]
    #[serde(rename = "type")]
    pub type_: ResponseCustomToolCallInputDoneEventType,
}
#[doc = "The event type identifier."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseCustomToolCallInputDoneEventType {
    #[serde(rename = "response.custom_tool_call_input.done")]
    ResponseCustomToolCallInputDone,
}
impl ::std::fmt::Display for ResponseCustomToolCallInputDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseCustomToolCallInputDone => {
                f.write_str("response.custom_tool_call_input.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseCustomToolCallInputDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.custom_tool_call_input.done" => Ok(Self::ResponseCustomToolCallInputDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseCustomToolCallInputDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseCustomToolCallInputDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponseError`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponseError(pub ::std::option::Option<ResponseErrorValue>);
impl ::std::ops::Deref for ResponseError {
    type Target = ::std::option::Option<ResponseErrorValue>;
    fn deref(&self) -> &::std::option::Option<ResponseErrorValue> {
        &self.0
    }
}
impl ::std::convert::From<ResponseError> for ::std::option::Option<ResponseErrorValue> {
    fn from(value: ResponseError) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ResponseErrorValue>> for ResponseError {
    fn from(value: ::std::option::Option<ResponseErrorValue>) -> Self {
        Self(value)
    }
}
#[doc = "The error code for the response.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
    #[serde(rename = "data_residency_mismatch")]
    DataResidencyMismatch,
    #[serde(rename = "bio_policy")]
    BioPolicy,
    #[serde(rename = "misalignment_policy_violation")]
    MisalignmentPolicyViolation,
    #[serde(rename = "vector_store_timeout")]
    VectorStoreTimeout,
    #[serde(rename = "invalid_image")]
    InvalidImage,
    #[serde(rename = "invalid_image_format")]
    InvalidImageFormat,
    #[serde(rename = "invalid_base64_image")]
    InvalidBase64Image,
    #[serde(rename = "invalid_image_url")]
    InvalidImageUrl,
    #[serde(rename = "image_too_large")]
    ImageTooLarge,
    #[serde(rename = "image_too_small")]
    ImageTooSmall,
    #[serde(rename = "image_parse_error")]
    ImageParseError,
    #[serde(rename = "image_content_policy_violation")]
    ImageContentPolicyViolation,
    #[serde(rename = "invalid_image_mode")]
    InvalidImageMode,
    #[serde(rename = "image_file_too_large")]
    ImageFileTooLarge,
    #[serde(rename = "unsupported_image_media_type")]
    UnsupportedImageMediaType,
    #[serde(rename = "empty_image_file")]
    EmptyImageFile,
    #[serde(rename = "failed_to_download_image")]
    FailedToDownloadImage,
    #[serde(rename = "image_file_not_found")]
    ImageFileNotFound,
}
impl ::std::fmt::Display for ResponseErrorCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerError => f.write_str("server_error"),
            Self::RateLimitExceeded => f.write_str("rate_limit_exceeded"),
            Self::InvalidPrompt => f.write_str("invalid_prompt"),
            Self::DataResidencyMismatch => f.write_str("data_residency_mismatch"),
            Self::BioPolicy => f.write_str("bio_policy"),
            Self::MisalignmentPolicyViolation => f.write_str("misalignment_policy_violation"),
            Self::VectorStoreTimeout => f.write_str("vector_store_timeout"),
            Self::InvalidImage => f.write_str("invalid_image"),
            Self::InvalidImageFormat => f.write_str("invalid_image_format"),
            Self::InvalidBase64Image => f.write_str("invalid_base64_image"),
            Self::InvalidImageUrl => f.write_str("invalid_image_url"),
            Self::ImageTooLarge => f.write_str("image_too_large"),
            Self::ImageTooSmall => f.write_str("image_too_small"),
            Self::ImageParseError => f.write_str("image_parse_error"),
            Self::ImageContentPolicyViolation => f.write_str("image_content_policy_violation"),
            Self::InvalidImageMode => f.write_str("invalid_image_mode"),
            Self::ImageFileTooLarge => f.write_str("image_file_too_large"),
            Self::UnsupportedImageMediaType => f.write_str("unsupported_image_media_type"),
            Self::EmptyImageFile => f.write_str("empty_image_file"),
            Self::FailedToDownloadImage => f.write_str("failed_to_download_image"),
            Self::ImageFileNotFound => f.write_str("image_file_not_found"),
        }
    }
}
impl ::std::str::FromStr for ResponseErrorCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server_error" => Ok(Self::ServerError),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            "invalid_prompt" => Ok(Self::InvalidPrompt),
            "data_residency_mismatch" => Ok(Self::DataResidencyMismatch),
            "bio_policy" => Ok(Self::BioPolicy),
            "misalignment_policy_violation" => Ok(Self::MisalignmentPolicyViolation),
            "vector_store_timeout" => Ok(Self::VectorStoreTimeout),
            "invalid_image" => Ok(Self::InvalidImage),
            "invalid_image_format" => Ok(Self::InvalidImageFormat),
            "invalid_base64_image" => Ok(Self::InvalidBase64Image),
            "invalid_image_url" => Ok(Self::InvalidImageUrl),
            "image_too_large" => Ok(Self::ImageTooLarge),
            "image_too_small" => Ok(Self::ImageTooSmall),
            "image_parse_error" => Ok(Self::ImageParseError),
            "image_content_policy_violation" => Ok(Self::ImageContentPolicyViolation),
            "invalid_image_mode" => Ok(Self::InvalidImageMode),
            "image_file_too_large" => Ok(Self::ImageFileTooLarge),
            "unsupported_image_media_type" => Ok(Self::UnsupportedImageMediaType),
            "empty_image_file" => Ok(Self::EmptyImageFile),
            "failed_to_download_image" => Ok(Self::FailedToDownloadImage),
            "image_file_not_found" => Ok(Self::ImageFileNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseErrorCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an error occurs."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseErrorEvent {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "The error message.\n"]
    pub message: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub param: ::std::option::Option<::std::string::String>,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `error`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseErrorEventType,
}
#[doc = "The type of the event. Always `error`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseErrorEventType {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for ResponseErrorEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ResponseErrorEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseErrorEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseErrorEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An error object returned when the model fails to generate a Response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseErrorValue {
    pub code: ResponseErrorCode,
    #[doc = "A human-readable description of the error.\n"]
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub misalignment: ::std::option::Option<MisalignmentErrorDetailsResource>,
}
#[doc = "An event that is emitted when a response fails.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFailedEvent {
    #[doc = "The response that failed.\n"]
    pub response: Response,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.failed`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseFailedEventType,
}
#[doc = "The type of the event. Always `response.failed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFailedEventType {
    #[serde(rename = "response.failed")]
    ResponseFailed,
}
impl ::std::fmt::Display for ResponseFailedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFailed => f.write_str("response.failed"),
        }
    }
}
impl ::std::str::FromStr for ResponseFailedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.failed" => Ok(Self::ResponseFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a file search call is completed (results found)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFileSearchCallCompletedEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFileSearchCallCompletedEventType {
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted,
}
impl ::std::fmt::Display for ResponseFileSearchCallCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFileSearchCallCompleted => {
                f.write_str("response.file_search_call.completed")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseFileSearchCallCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.file_search_call.completed" => Ok(Self::ResponseFileSearchCallCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFileSearchCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFileSearchCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a file search call is initiated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFileSearchCallInProgressEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFileSearchCallInProgressEventType {
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress,
}
impl ::std::fmt::Display for ResponseFileSearchCallInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFileSearchCallInProgress => {
                f.write_str("response.file_search_call.in_progress")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseFileSearchCallInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.file_search_call.in_progress" => Ok(Self::ResponseFileSearchCallInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFileSearchCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFileSearchCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a file search is currently searching."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFileSearchCallSearchingEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the file search call is searching.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallSearchingEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFileSearchCallSearchingEventType {
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching,
}
impl ::std::fmt::Display for ResponseFileSearchCallSearchingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFileSearchCallSearching => {
                f.write_str("response.file_search_call.searching")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseFileSearchCallSearchingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.file_search_call.searching" => Ok(Self::ResponseFileSearchCallSearching),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFileSearchCallSearchingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFileSearchCallSearchingEventType {
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
#[doc = "Emitted when there is a partial function-call arguments delta."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    #[doc = "The function-call arguments delta that is added.\n"]
    pub delta: ::std::string::String,
    #[doc = "The ID of the output item that the function-call arguments delta is added to.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the function-call arguments delta is added to.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseFunctionCallArgumentsDeltaEventType,
}
#[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFunctionCallArgumentsDeltaEventType {
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}
impl ::std::fmt::Display for ResponseFunctionCallArgumentsDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFunctionCallArgumentsDelta => {
                f.write_str("response.function_call_arguments.delta")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseFunctionCallArgumentsDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.function_call_arguments.delta" => {
                Ok(Self::ResponseFunctionCallArgumentsDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFunctionCallArgumentsDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseFunctionCallArgumentsDeltaEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when function-call arguments are finalized."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    #[doc = "The function-call arguments."]
    pub arguments: ::std::string::String,
    #[doc = "The ID of the item."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[serde(rename = "type")]
    pub type_: ResponseFunctionCallArgumentsDoneEventType,
}
#[doc = "`ResponseFunctionCallArgumentsDoneEventType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFunctionCallArgumentsDoneEventType {
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}
impl ::std::fmt::Display for ResponseFunctionCallArgumentsDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseFunctionCallArgumentsDone => {
                f.write_str("response.function_call_arguments.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseFunctionCallArgumentsDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.function_call_arguments.done" => Ok(Self::ResponseFunctionCallArgumentsDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFunctionCallArgumentsDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFunctionCallArgumentsDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an image generation tool call has completed and the final image is available.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseImageGenCallCompletedEvent {
    #[doc = "The unique identifier of the image generation item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.image_generation_call.completed'."]
    #[serde(rename = "type")]
    pub type_: ResponseImageGenCallCompletedEventType,
}
#[doc = "The type of the event. Always 'response.image_generation_call.completed'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseImageGenCallCompletedEventType {
    #[serde(rename = "response.image_generation_call.completed")]
    ResponseImageGenerationCallCompleted,
}
impl ::std::fmt::Display for ResponseImageGenCallCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseImageGenerationCallCompleted => {
                f.write_str("response.image_generation_call.completed")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseImageGenCallCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.image_generation_call.completed" => {
                Ok(Self::ResponseImageGenerationCallCompleted)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseImageGenCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseImageGenCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an image generation tool call is actively generating an image (intermediate state).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseImageGenCallGeneratingEvent {
    #[doc = "The unique identifier of the image generation item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of the image generation item being processed."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.image_generation_call.generating'."]
    #[serde(rename = "type")]
    pub type_: ResponseImageGenCallGeneratingEventType,
}
#[doc = "The type of the event. Always 'response.image_generation_call.generating'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseImageGenCallGeneratingEventType {
    #[serde(rename = "response.image_generation_call.generating")]
    ResponseImageGenerationCallGenerating,
}
impl ::std::fmt::Display for ResponseImageGenCallGeneratingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseImageGenerationCallGenerating => {
                f.write_str("response.image_generation_call.generating")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseImageGenCallGeneratingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.image_generation_call.generating" => {
                Ok(Self::ResponseImageGenerationCallGenerating)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseImageGenCallGeneratingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseImageGenCallGeneratingEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an image generation tool call is in progress.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseImageGenCallInProgressEvent {
    #[doc = "The unique identifier of the image generation item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of the image generation item being processed."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.image_generation_call.in_progress'."]
    #[serde(rename = "type")]
    pub type_: ResponseImageGenCallInProgressEventType,
}
#[doc = "The type of the event. Always 'response.image_generation_call.in_progress'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseImageGenCallInProgressEventType {
    #[serde(rename = "response.image_generation_call.in_progress")]
    ResponseImageGenerationCallInProgress,
}
impl ::std::fmt::Display for ResponseImageGenCallInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseImageGenerationCallInProgress => {
                f.write_str("response.image_generation_call.in_progress")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseImageGenCallInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.image_generation_call.in_progress" => {
                Ok(Self::ResponseImageGenerationCallInProgress)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseImageGenCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseImageGenCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a partial image is available during image generation streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseImageGenCallPartialImageEvent {
    #[doc = "The background setting that was used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identifier of the image generation item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The output format that was used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "Base64-encoded partial image data, suitable for rendering as an image."]
    pub partial_image_b64: ::std::string::String,
    #[doc = "0-based index for the partial image (backend is 1-based, but this is 0-based for the user)."]
    pub partial_image_index: i64,
    #[doc = "The image quality that was used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<::std::string::String>,
    #[doc = "The sequence number of the image generation item being processed."]
    pub sequence_number: i64,
    #[doc = "The image size that was used."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the event. Always 'response.image_generation_call.partial_image'."]
    #[serde(rename = "type")]
    pub type_: ResponseImageGenCallPartialImageEventType,
}
#[doc = "The type of the event. Always 'response.image_generation_call.partial_image'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseImageGenCallPartialImageEventType {
    #[serde(rename = "response.image_generation_call.partial_image")]
    ResponseImageGenerationCallPartialImage,
}
impl ::std::fmt::Display for ResponseImageGenCallPartialImageEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseImageGenerationCallPartialImage => {
                f.write_str("response.image_generation_call.partial_image")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseImageGenCallPartialImageEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.image_generation_call.partial_image" => {
                Ok(Self::ResponseImageGenerationCallPartialImage)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseImageGenCallPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseImageGenCallPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the response is in progress."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseInProgressEvent {
    #[doc = "The response that is in progress.\n"]
    pub response: Response,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.in_progress`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseInProgressEventType,
}
#[doc = "The type of the event. Always `response.in_progress`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseInProgressEventType {
    #[serde(rename = "response.in_progress")]
    ResponseInProgress,
}
impl ::std::fmt::Display for ResponseInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseInProgress => f.write_str("response.in_progress"),
        }
    }
}
impl ::std::str::FromStr for ResponseInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.in_progress" => Ok(Self::ResponseInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details about why the response is incomplete.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseIncompleteDetails {
    #[doc = "The reason why the response is incomplete. `steered` means\nthe response stopped at a safe output boundary after a\nWebSocket `response.steer` event. The server can then create\na successor response automatically with the queued input.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<ResponseIncompleteDetailsReason>,
}
#[doc = "The reason why the response is incomplete. `steered` means\nthe response stopped at a safe output boundary after a\nWebSocket `response.steer` event. The server can then create\na successor response automatically with the queued input.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseIncompleteDetailsReason {
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "max_messages")]
    MaxMessages,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "steered")]
    Steered,
}
impl ::std::fmt::Display for ResponseIncompleteDetailsReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MaxOutputTokens => f.write_str("max_output_tokens"),
            Self::MaxMessages => f.write_str("max_messages"),
            Self::ContentFilter => f.write_str("content_filter"),
            Self::Steered => f.write_str("steered"),
        }
    }
}
impl ::std::str::FromStr for ResponseIncompleteDetailsReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "max_output_tokens" => Ok(Self::MaxOutputTokens),
            "max_messages" => Ok(Self::MaxMessages),
            "content_filter" => Ok(Self::ContentFilter),
            "steered" => Ok(Self::Steered),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseIncompleteDetailsReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An event that is emitted when a response finishes as incomplete.\n\nOver WebSocket, steering can finish a response with\n`response.incomplete_details.reason` set to `steered`, followed automatically\nby a successor `response.created` that commits the queued steering input.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseIncompleteEvent {
    #[doc = "The response that was incomplete.\n"]
    pub response: Response,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.incomplete`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseIncompleteEventType,
}
#[doc = "The type of the event. Always `response.incomplete`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseIncompleteEventType {
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete,
}
impl ::std::fmt::Display for ResponseIncompleteEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseIncomplete => f.write_str("response.incomplete"),
        }
    }
}
impl ::std::str::FromStr for ResponseIncompleteEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.incomplete" => Ok(Self::ResponseIncomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseIncompleteEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseIncompleteEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A system (or developer) message inserted into the model's context.\n\nWhen using along with `previous_response_id`, the instructions from a previous\nresponse will not be carried over to the next response. This makes it simple\nto swap out system (or developer) messages in new responses.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponseInstructions {
    String(::std::string::String),
    InputItemList(::std::vec::Vec<InputItem>),
}
impl ::std::convert::From<::std::vec::Vec<InputItem>> for ResponseInstructions {
    fn from(value: ::std::vec::Vec<InputItem>) -> Self {
        Self::InputItemList(value)
    }
}
#[doc = "A list of Response items."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseItemList {
    #[doc = "A list of items used to generate this response."]
    pub data: ::std::vec::Vec<ItemResource>,
    #[doc = "The ID of the first item in the list."]
    pub first_id: ::std::string::String,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[doc = "The ID of the last item in the list."]
    pub last_id: ::std::string::String,
    #[doc = "The type of object returned, must be `list`."]
    pub object: ResponseItemListObject,
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
pub enum ResponseItemListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ResponseItemListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ResponseItemListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseItemListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A logprob is the logarithmic probability that the model assigns to producing \na particular token at a given position in the sequence. Less-negative (higher) \nlogprob values indicate greater model confidence in that token choice.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseLogProb {
    #[doc = "The log probability of this token.\n"]
    pub logprob: f64,
    #[doc = "A possible text token."]
    pub token: ::std::string::String,
    #[doc = "The log probabilities of up to 20 of the most likely tokens.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub top_logprobs: ::std::vec::Vec<ResponseLogProbTopLogprobsItem>,
}
#[doc = "`ResponseLogProbTopLogprobsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseLogProbTopLogprobsItem {
    #[doc = "The log probability of this token."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logprob: ::std::option::Option<f64>,
    #[doc = "A possible text token."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
}
#[doc = "Emitted when there is a delta (partial update) to the arguments of an MCP tool call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpCallArgumentsDeltaEvent {
    #[doc = "A JSON string containing the partial update to the arguments for the MCP tool call.\n"]
    pub delta: ::std::string::String,
    #[doc = "The unique identifier of the MCP tool call item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_call_arguments.delta'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpCallArgumentsDeltaEventType,
}
#[doc = "The type of the event. Always 'response.mcp_call_arguments.delta'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpCallArgumentsDeltaEventType {
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMcpCallArgumentsDelta,
}
impl ::std::fmt::Display for ResponseMcpCallArgumentsDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallArgumentsDelta => f.write_str("response.mcp_call_arguments.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpCallArgumentsDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call_arguments.delta" => Ok(Self::ResponseMcpCallArgumentsDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpCallArgumentsDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpCallArgumentsDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the arguments for an MCP tool call are finalized.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpCallArgumentsDoneEvent {
    #[doc = "A JSON string containing the finalized arguments for the MCP tool call.\n"]
    pub arguments: ::std::string::String,
    #[doc = "The unique identifier of the MCP tool call item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_call_arguments.done'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpCallArgumentsDoneEventType,
}
#[doc = "The type of the event. Always 'response.mcp_call_arguments.done'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpCallArgumentsDoneEventType {
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMcpCallArgumentsDone,
}
impl ::std::fmt::Display for ResponseMcpCallArgumentsDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallArgumentsDone => f.write_str("response.mcp_call_arguments.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpCallArgumentsDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call_arguments.done" => Ok(Self::ResponseMcpCallArgumentsDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpCallArgumentsDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpCallArgumentsDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an MCP  tool call has completed successfully.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpCallCompletedEvent {
    #[doc = "The ID of the MCP tool call item that completed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that completed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_call.completed'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpCallCompletedEventType,
}
#[doc = "The type of the event. Always 'response.mcp_call.completed'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpCallCompletedEventType {
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMcpCallCompleted,
}
impl ::std::fmt::Display for ResponseMcpCallCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallCompleted => f.write_str("response.mcp_call.completed"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpCallCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.completed" => Ok(Self::ResponseMcpCallCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an MCP  tool call has failed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpCallFailedEvent {
    #[doc = "The ID of the MCP tool call item that failed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that failed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_call.failed'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpCallFailedEventType,
}
#[doc = "The type of the event. Always 'response.mcp_call.failed'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpCallFailedEventType {
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMcpCallFailed,
}
impl ::std::fmt::Display for ResponseMcpCallFailedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallFailed => f.write_str("response.mcp_call.failed"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpCallFailedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.failed" => Ok(Self::ResponseMcpCallFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpCallFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpCallFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an MCP  tool call is in progress.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpCallInProgressEvent {
    #[doc = "The unique identifier of the MCP tool call item being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_call.in_progress'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpCallInProgressEventType,
}
#[doc = "The type of the event. Always 'response.mcp_call.in_progress'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpCallInProgressEventType {
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMcpCallInProgress,
}
impl ::std::fmt::Display for ResponseMcpCallInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpCallInProgress => f.write_str("response.mcp_call.in_progress"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpCallInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_call.in_progress" => Ok(Self::ResponseMcpCallInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the list of available MCP tools has been successfully retrieved.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpListToolsCompletedEvent {
    #[doc = "The ID of the MCP tool call item that produced this output."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that was processed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_list_tools.completed'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpListToolsCompletedEventType,
}
#[doc = "The type of the event. Always 'response.mcp_list_tools.completed'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpListToolsCompletedEventType {
    #[serde(rename = "response.mcp_list_tools.completed")]
    ResponseMcpListToolsCompleted,
}
impl ::std::fmt::Display for ResponseMcpListToolsCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpListToolsCompleted => f.write_str("response.mcp_list_tools.completed"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpListToolsCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_list_tools.completed" => Ok(Self::ResponseMcpListToolsCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpListToolsCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpListToolsCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the attempt to list available MCP tools has failed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpListToolsFailedEvent {
    #[doc = "The ID of the MCP tool call item that failed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that failed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_list_tools.failed'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpListToolsFailedEventType,
}
#[doc = "The type of the event. Always 'response.mcp_list_tools.failed'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpListToolsFailedEventType {
    #[serde(rename = "response.mcp_list_tools.failed")]
    ResponseMcpListToolsFailed,
}
impl ::std::fmt::Display for ResponseMcpListToolsFailedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpListToolsFailed => f.write_str("response.mcp_list_tools.failed"),
        }
    }
}
impl ::std::str::FromStr for ResponseMcpListToolsFailedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_list_tools.failed" => Ok(Self::ResponseMcpListToolsFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpListToolsFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpListToolsFailedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when the system is in the process of retrieving the list of available MCP tools.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseMcpListToolsInProgressEvent {
    #[doc = "The ID of the MCP tool call item that is being processed."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that is being processed."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.mcp_list_tools.in_progress'."]
    #[serde(rename = "type")]
    pub type_: ResponseMcpListToolsInProgressEventType,
}
#[doc = "The type of the event. Always 'response.mcp_list_tools.in_progress'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseMcpListToolsInProgressEventType {
    #[serde(rename = "response.mcp_list_tools.in_progress")]
    ResponseMcpListToolsInProgress,
}
impl ::std::fmt::Display for ResponseMcpListToolsInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseMcpListToolsInProgress => {
                f.write_str("response.mcp_list_tools.in_progress")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseMcpListToolsInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.mcp_list_tools.in_progress" => Ok(Self::ResponseMcpListToolsInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseMcpListToolsInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseMcpListToolsInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type of this resource - always set to `response`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseObject {
    #[serde(rename = "response")]
    Response,
}
impl ::std::fmt::Display for ResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Response => f.write_str("response"),
        }
    }
}
impl ::std::str::FromStr for ResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response" => Ok(Self::Response),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a new output item is added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseOutputItemAddedEvent {
    #[doc = "The output item that was added. For reasoning items, `encrypted_content`\nmay be incomplete while the item is in progress. Use the reasoning item\nfrom the corresponding `response.output_item.done` event when passing it\nas input to a subsequent request.\n"]
    pub item: OutputItem,
    #[doc = "The index of the output item that was added.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.output_item.added`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseOutputItemAddedEventType,
}
#[doc = "The type of the event. Always `response.output_item.added`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseOutputItemAddedEventType {
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}
impl ::std::fmt::Display for ResponseOutputItemAddedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputItemAdded => f.write_str("response.output_item.added"),
        }
    }
}
impl ::std::str::FromStr for ResponseOutputItemAddedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_item.added" => Ok(Self::ResponseOutputItemAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseOutputItemAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseOutputItemAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an output item is marked done."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseOutputItemDoneEvent {
    #[doc = "The output item that was marked done.\n"]
    pub item: OutputItem,
    #[doc = "The index of the output item that was marked done.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.output_item.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseOutputItemDoneEventType,
}
#[doc = "The type of the event. Always `response.output_item.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseOutputItemDoneEventType {
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}
impl ::std::fmt::Display for ResponseOutputItemDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputItemDone => f.write_str("response.output_item.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseOutputItemDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_item.done" => Ok(Self::ResponseOutputItemDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseOutputItemDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseOutputItemDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when an annotation is added to output text content.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseOutputTextAnnotationAddedEvent {
    #[doc = "The annotation object being added. (See annotation schema for details.)"]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub annotation: ::std::option::Option<Annotation>,
    #[doc = "The index of the annotation within the content part."]
    pub annotation_index: i64,
    #[doc = "The index of the content part within the output item."]
    pub content_index: i64,
    #[doc = "The unique identifier of the item to which the annotation is being added."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item in the response's output array."]
    pub output_index: i64,
    #[doc = "The sequence number of this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.output_text.annotation.added'."]
    #[serde(rename = "type")]
    pub type_: ResponseOutputTextAnnotationAddedEventType,
}
#[doc = "The type of the event. Always 'response.output_text.annotation.added'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseOutputTextAnnotationAddedEventType {
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded,
}
impl ::std::fmt::Display for ResponseOutputTextAnnotationAddedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputTextAnnotationAdded => {
                f.write_str("response.output_text.annotation.added")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseOutputTextAnnotationAddedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_text.annotation.added" => Ok(Self::ResponseOutputTextAnnotationAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseOutputTextAnnotationAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseOutputTextAnnotationAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Options for prompt caching. Supported for `gpt-5.6` and later models. By default, OpenAI automatically chooses one implicit cache breakpoint. You can add explicit breakpoints to content blocks with `prompt_cache_breakpoint`. Each request can write up to four breakpoints. For cache matching, OpenAI considers up to the latest 80 breakpoints in the conversation, without a content-block lookback limit. Set `mode` to `explicit` to disable the implicit breakpoint. The `ttl` defaults to `30m`, which is currently the only supported value. See the [prompt caching guide](https://developers.openai.com/api/docs/guides/prompt-caching) for current details."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponsePromptCacheOptionsParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comparison_response_id: ::std::option::Option<::std::string::String>,
    #[doc = "Controls whether OpenAI automatically creates an implicit cache breakpoint. Defaults to `implicit`. With `implicit`, OpenAI creates one implicit breakpoint and writes up to the latest three explicit breakpoints in the request. With `explicit`, OpenAI does not create an implicit breakpoint and writes up to the latest four explicit breakpoints. If there are no explicit breakpoints, the request does not use prompt caching."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<PromptCacheModeEnum>,
    #[doc = "Prepares the prompt cache without generating output. Defaults to `false`. When set to `true`, overrides the `generate` field to `false`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prewarm: ::std::option::Option<bool>,
    #[doc = "The minimum lifetime applied to every implicit and explicit cache breakpoint written by the request. Defaults to `30m`, which is currently the only supported value. The backend may retain cache entries for longer."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<PromptCacheTtlEnum>,
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
pub enum ResponsePromptCacheRetention {
    #[serde(rename = "in_memory")]
    InMemory,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for ResponsePromptCacheRetention {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InMemory => f.write_str("in_memory"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for ResponsePromptCacheRetention {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_memory" => Ok(Self::InMemory),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponsePromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponsePromptCacheRetention {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponsePromptVariables`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponsePromptVariables(pub ::std::option::Option<ResponsePromptVariablesValue>);
impl ::std::ops::Deref for ResponsePromptVariables {
    type Target = ::std::option::Option<ResponsePromptVariablesValue>;
    fn deref(&self) -> &::std::option::Option<ResponsePromptVariablesValue> {
        &self.0
    }
}
impl ::std::convert::From<ResponsePromptVariables>
    for ::std::option::Option<ResponsePromptVariablesValue>
{
    fn from(value: ResponsePromptVariables) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ResponsePromptVariablesValue>>
    for ResponsePromptVariables
{
    fn from(value: ::std::option::Option<ResponsePromptVariablesValue>) -> Self {
        Self(value)
    }
}
#[doc = "Optional map of values to substitute in for variables in your\nprompt. The substitution values can either be strings, or other\nResponse input types like images or files.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponsePromptVariablesValue(
    pub ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>,
);
impl ::std::ops::Deref for ResponsePromptVariablesValue {
    type Target =
        ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>
    {
        &self.0
    }
}
impl ::std::convert::From<ResponsePromptVariablesValue>
    for ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>
{
    fn from(value: ResponsePromptVariablesValue) -> Self {
        value.0
    }
}
impl
    ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ResponsePromptVariablesValueValue>,
    > for ResponsePromptVariablesValue
{
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            ResponsePromptVariablesValueValue,
        >,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`ResponsePromptVariablesValueValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponsePromptVariablesValueValue {
    String(::std::string::String),
    InputTextContent(InputTextContent),
    InputImageContent(InputImageContent),
    InputFileContent(InputFileContent),
}
impl ::std::convert::From<InputTextContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputTextContent) -> Self {
        Self::InputTextContent(value)
    }
}
impl ::std::convert::From<InputImageContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputImageContent) -> Self {
        Self::InputImageContent(value)
    }
}
impl ::std::convert::From<InputFileContent> for ResponsePromptVariablesValueValue {
    fn from(value: InputFileContent) -> Self {
        Self::InputFileContent(value)
    }
}
#[doc = "`ResponseProperties`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseProperties {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tool_calls: ::std::option::Option<i64>,
    #[doc = "Model ID used to generate the response, like `gpt-6-astra`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](https://developers.openai.com/api/docs/models)\nto browse and compare available models.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ModelIdsResponses>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_response_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<Prompt>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<ResponseTextParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<ToolChoiceParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ToolsArray>,
}
#[doc = "Emitted when a response is queued and waiting to be processed.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseQueuedEvent {
    #[doc = "The full response object that is queued."]
    pub response: Response,
    #[doc = "The sequence number for this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always 'response.queued'."]
    #[serde(rename = "type")]
    pub type_: ResponseQueuedEventType,
}
#[doc = "The type of the event. Always 'response.queued'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseQueuedEventType {
    #[serde(rename = "response.queued")]
    ResponseQueued,
}
impl ::std::fmt::Display for ResponseQueuedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseQueued => f.write_str("response.queued"),
        }
    }
}
impl ::std::str::FromStr for ResponseQueuedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.queued" => Ok(Self::ResponseQueued),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseQueuedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseQueuedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a new reasoning summary part is added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryPartAddedEvent {
    #[doc = "The ID of the item this summary part is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    pub output_index: i64,
    pub part: ResponseReasoningSummaryPartAddedEventPart,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: i64,
    #[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartAddedEventType,
}
#[doc = "The summary part that was added.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryPartAddedEventPart {
    #[doc = "The text of the summary part."]
    pub text: ::std::string::String,
    #[doc = "The type of the summary part. Always `summary_text`."]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartAddedEventPartType,
}
#[doc = "The type of the summary part. Always `summary_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryPartAddedEventPartType {
    #[serde(rename = "summary_text")]
    SummaryText,
}
impl ::std::fmt::Display for ResponseReasoningSummaryPartAddedEventPartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SummaryText => f.write_str("summary_text"),
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryPartAddedEventPartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "summary_text" => Ok(Self::SummaryText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryPartAddedEventPartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseReasoningSummaryPartAddedEventPartType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryPartAddedEventType {
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded,
}
impl ::std::fmt::Display for ResponseReasoningSummaryPartAddedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningSummaryPartAdded => {
                f.write_str("response.reasoning_summary_part.added")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryPartAddedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_summary_part.added" => Ok(Self::ResponseReasoningSummaryPartAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryPartAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningSummaryPartAddedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a reasoning summary part is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryPartDoneEvent {
    #[doc = "The ID of the item this summary part is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    pub output_index: i64,
    pub part: ResponseReasoningSummaryPartDoneEventPart,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The completion status of the summary part. Omitted when the part completed\nnormally and set to `incomplete` when generation was interrupted.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ResponseReasoningSummaryPartDoneEventStatus>,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: i64,
    #[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartDoneEventType,
}
#[doc = "The completed summary part.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryPartDoneEventPart {
    #[doc = "The text of the summary part."]
    pub text: ::std::string::String,
    #[doc = "The type of the summary part. Always `summary_text`."]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartDoneEventPartType,
}
#[doc = "The type of the summary part. Always `summary_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryPartDoneEventPartType {
    #[serde(rename = "summary_text")]
    SummaryText,
}
impl ::std::fmt::Display for ResponseReasoningSummaryPartDoneEventPartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SummaryText => f.write_str("summary_text"),
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryPartDoneEventPartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "summary_text" => Ok(Self::SummaryText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryPartDoneEventPartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseReasoningSummaryPartDoneEventPartType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The completion status of the summary part. Omitted when the part completed\nnormally and set to `incomplete` when generation was interrupted.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryPartDoneEventStatus {
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ResponseReasoningSummaryPartDoneEventStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryPartDoneEventStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryPartDoneEventStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseReasoningSummaryPartDoneEventStatus
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryPartDoneEventType {
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone,
}
impl ::std::fmt::Display for ResponseReasoningSummaryPartDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningSummaryPartDone => {
                f.write_str("response.reasoning_summary_part.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryPartDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_summary_part.done" => Ok(Self::ResponseReasoningSummaryPartDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryPartDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningSummaryPartDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a delta is added to a reasoning summary text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    #[doc = "The text delta that was added to the summary.\n"]
    pub delta: ::std::string::String,
    #[doc = "The ID of the item this summary text delta is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this summary text delta is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: i64,
    #[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryTextDeltaEventType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryTextDeltaEventType {
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta,
}
impl ::std::fmt::Display for ResponseReasoningSummaryTextDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningSummaryTextDelta => {
                f.write_str("response.reasoning_summary_text.delta")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryTextDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_summary_text.delta" => Ok(Self::ResponseReasoningSummaryTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningSummaryTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a reasoning summary text is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningSummaryTextDoneEvent {
    #[doc = "The ID of the item this summary text is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this summary text is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: i64,
    #[doc = "The full text of the completed reasoning summary.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryTextDoneEventType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningSummaryTextDoneEventType {
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone,
}
impl ::std::fmt::Display for ResponseReasoningSummaryTextDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningSummaryTextDone => {
                f.write_str("response.reasoning_summary_text.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningSummaryTextDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_summary_text.done" => Ok(Self::ResponseReasoningSummaryTextDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningSummaryTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningSummaryTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a delta is added to a reasoning text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningTextDeltaEvent {
    #[doc = "The index of the reasoning content part this delta is associated with.\n"]
    pub content_index: i64,
    #[doc = "The text delta that was added to the reasoning content.\n"]
    pub delta: ::std::string::String,
    #[doc = "The ID of the item this reasoning text delta is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this reasoning text delta is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.reasoning_text.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningTextDeltaEventType,
}
#[doc = "The type of the event. Always `response.reasoning_text.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningTextDeltaEventType {
    #[serde(rename = "response.reasoning_text.delta")]
    ResponseReasoningTextDelta,
}
impl ::std::fmt::Display for ResponseReasoningTextDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningTextDelta => f.write_str("response.reasoning_text.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningTextDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_text.delta" => Ok(Self::ResponseReasoningTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a reasoning text is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseReasoningTextDoneEvent {
    #[doc = "The index of the reasoning content part.\n"]
    pub content_index: i64,
    #[doc = "The ID of the item this reasoning text is associated with.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item this reasoning text is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The full text of the completed reasoning content.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `response.reasoning_text.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningTextDoneEventType,
}
#[doc = "The type of the event. Always `response.reasoning_text.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseReasoningTextDoneEventType {
    #[serde(rename = "response.reasoning_text.done")]
    ResponseReasoningTextDone,
}
impl ::std::fmt::Display for ResponseReasoningTextDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseReasoningTextDone => f.write_str("response.reasoning_text.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseReasoningTextDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.reasoning_text.done" => Ok(Self::ResponseReasoningTextDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseReasoningTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseReasoningTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when there is a partial refusal text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseRefusalDeltaEvent {
    #[doc = "The index of the content part that the refusal text is added to.\n"]
    pub content_index: i64,
    #[doc = "The refusal text that is added.\n"]
    pub delta: ::std::string::String,
    #[doc = "The ID of the output item that the refusal text is added to.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the refusal text is added to.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.refusal.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseRefusalDeltaEventType,
}
#[doc = "The type of the event. Always `response.refusal.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseRefusalDeltaEventType {
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta,
}
impl ::std::fmt::Display for ResponseRefusalDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseRefusalDelta => f.write_str("response.refusal.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseRefusalDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.refusal.delta" => Ok(Self::ResponseRefusalDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseRefusalDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseRefusalDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when refusal text is finalized."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseRefusalDoneEvent {
    #[doc = "The index of the content part that the refusal text is finalized.\n"]
    pub content_index: i64,
    #[doc = "The ID of the output item that the refusal text is finalized.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the refusal text is finalized.\n"]
    pub output_index: i64,
    #[doc = "The refusal text that is finalized.\n"]
    pub refusal: ::std::string::String,
    #[doc = "The sequence number of this event.\n"]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.refusal.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseRefusalDoneEventType,
}
#[doc = "The type of the event. Always `response.refusal.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseRefusalDoneEventType {
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone,
}
impl ::std::fmt::Display for ResponseRefusalDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseRefusalDone => f.write_str("response.refusal.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseRefusalDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.refusal.done" => Ok(Self::ResponseRefusalDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseRefusalDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseRefusalDoneEventType {
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
pub struct ResponseSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for ResponseSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ResponseSafetyIdentifier> for ::std::string::String {
    fn from(value: ResponseSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ResponseSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ResponseSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ResponseSafetyIdentifier {
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
#[doc = "A streaming event that indicated a shell command was added to a tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseShellCallCommandAddedStreamingEvent {
    #[doc = "The shell command that was added."]
    pub command: ::std::string::String,
    #[doc = "The index of the shell command that was added."]
    pub command_index: i64,
    #[doc = "The index of the output item that was updated."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.shell_call_command.added`."]
    #[serde(rename = "type")]
    pub type_: ResponseShellCallCommandAddedStreamingEventType,
}
#[doc = "The type of the event, always `response.shell_call_command.added`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseShellCallCommandAddedStreamingEventType {
    #[serde(rename = "response.shell_call_command.added")]
    ResponseShellCallCommandAdded,
}
impl ::std::fmt::Display for ResponseShellCallCommandAddedStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseShellCallCommandAdded => f.write_str("response.shell_call_command.added"),
        }
    }
}
impl ::std::str::FromStr for ResponseShellCallCommandAddedStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.shell_call_command.added" => Ok(Self::ResponseShellCallCommandAdded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseShellCallCommandAddedStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseShellCallCommandAddedStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A streaming event that indicated a shell command was incrementally updated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseShellCallCommandDeltaStreamingEvent {
    #[doc = "The index of the shell command that was updated."]
    pub command_index: i64,
    #[doc = "The shell command delta that was appended."]
    pub delta: ::std::string::String,
    #[doc = "An obfuscation string that was added to pad the event payload."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub obfuscation: ::std::option::Option<::std::string::String>,
    #[doc = "The index of the output item that was updated."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.shell_call_command.delta`."]
    #[serde(rename = "type")]
    pub type_: ResponseShellCallCommandDeltaStreamingEventType,
}
#[doc = "The type of the event, always `response.shell_call_command.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseShellCallCommandDeltaStreamingEventType {
    #[serde(rename = "response.shell_call_command.delta")]
    ResponseShellCallCommandDelta,
}
impl ::std::fmt::Display for ResponseShellCallCommandDeltaStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseShellCallCommandDelta => f.write_str("response.shell_call_command.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseShellCallCommandDeltaStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.shell_call_command.delta" => Ok(Self::ResponseShellCallCommandDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseShellCallCommandDeltaStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseShellCallCommandDeltaStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A streaming event that indicated a shell command was completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseShellCallCommandDoneStreamingEvent {
    #[doc = "The final shell command that was emitted."]
    pub command: ::std::string::String,
    #[doc = "The index of the shell command that was completed."]
    pub command_index: i64,
    #[doc = "The index of the output item that was updated."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.shell_call_command.done`."]
    #[serde(rename = "type")]
    pub type_: ResponseShellCallCommandDoneStreamingEventType,
}
#[doc = "The type of the event, always `response.shell_call_command.done`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseShellCallCommandDoneStreamingEventType {
    #[serde(rename = "response.shell_call_command.done")]
    ResponseShellCallCommandDone,
}
impl ::std::fmt::Display for ResponseShellCallCommandDoneStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseShellCallCommandDone => f.write_str("response.shell_call_command.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseShellCallCommandDoneStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.shell_call_command.done" => Ok(Self::ResponseShellCallCommandDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseShellCallCommandDoneStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseShellCallCommandDoneStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A streaming event that indicated shell call output was incrementally added."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseShellCallOutputContentDeltaStreamingEvent {
    #[doc = "The index of the shell command that produced output."]
    pub command_index: i64,
    #[doc = "The stdout/stderr delta that was emitted."]
    pub delta: ShellCallOutputDelta,
    #[doc = "The ID of the output item that was updated."]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that was updated."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.shell_call_output_content.delta`."]
    #[serde(rename = "type")]
    pub type_: ResponseShellCallOutputContentDeltaStreamingEventType,
}
#[doc = "The type of the event, always `response.shell_call_output_content.delta`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseShellCallOutputContentDeltaStreamingEventType {
    #[serde(rename = "response.shell_call_output_content.delta")]
    ResponseShellCallOutputContentDelta,
}
impl ::std::fmt::Display for ResponseShellCallOutputContentDeltaStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseShellCallOutputContentDelta => {
                f.write_str("response.shell_call_output_content.delta")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseShellCallOutputContentDeltaStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.shell_call_output_content.delta" => {
                Ok(Self::ResponseShellCallOutputContentDelta)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseShellCallOutputContentDeltaStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseShellCallOutputContentDeltaStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A streaming event that indicated shell call output was completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseShellCallOutputContentDoneStreamingEvent {
    #[doc = "The index of the shell command that produced output."]
    pub command_index: i64,
    #[doc = "The ID of the output item that was updated."]
    pub item_id: ::std::string::String,
    #[doc = "The output contents emitted for the shell command."]
    pub output: ::std::vec::Vec<FunctionShellCallOutputContent>,
    #[doc = "The index of the output item that was updated."]
    pub output_index: i64,
    #[doc = "The sequence number of the event that was emitted."]
    pub sequence_number: i64,
    #[doc = "The type of the event, always `response.shell_call_output_content.done`."]
    #[serde(rename = "type")]
    pub type_: ResponseShellCallOutputContentDoneStreamingEventType,
}
#[doc = "The type of the event, always `response.shell_call_output_content.done`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseShellCallOutputContentDoneStreamingEventType {
    #[serde(rename = "response.shell_call_output_content.done")]
    ResponseShellCallOutputContentDone,
}
impl ::std::fmt::Display for ResponseShellCallOutputContentDoneStreamingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseShellCallOutputContentDone => {
                f.write_str("response.shell_call_output_content.done")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseShellCallOutputContentDoneStreamingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.shell_call_output_content.done" => {
                Ok(Self::ResponseShellCallOutputContentDone)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseShellCallOutputContentDoneStreamingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ResponseShellCallOutputContentDoneStreamingEventType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The status of the response generation. One of `completed`, `failed`,\n`in_progress`, `cancelled`, `queued`, or `incomplete`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Queued => f.write_str("queued"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for ResponseStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "in_progress" => Ok(Self::InProgress),
            "cancelled" => Ok(Self::Cancelled),
            "queued" => Ok(Self::Queued),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Event emitted while a response is streamed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponseStreamEvent {
    AudioDeltaEvent(ResponseAudioDeltaEvent),
    AudioDoneEvent(ResponseAudioDoneEvent),
    AudioTranscriptDeltaEvent(ResponseAudioTranscriptDeltaEvent),
    AudioTranscriptDoneEvent(ResponseAudioTranscriptDoneEvent),
    CodeInterpreterCallCodeDeltaEvent(ResponseCodeInterpreterCallCodeDeltaEvent),
    CodeInterpreterCallCodeDoneEvent(ResponseCodeInterpreterCallCodeDoneEvent),
    CodeInterpreterCallCompletedEvent(ResponseCodeInterpreterCallCompletedEvent),
    CodeInterpreterCallInProgressEvent(ResponseCodeInterpreterCallInProgressEvent),
    CodeInterpreterCallInterpretingEvent(ResponseCodeInterpreterCallInterpretingEvent),
    CompactionCompactingStreamingEvent(ResponseCompactionCompactingStreamingEvent),
    CompletedEvent(ResponseCompletedEvent),
    ContentPartAddedEvent(ResponseContentPartAddedEvent),
    ContentPartDoneEvent(ResponseContentPartDoneEvent),
    CreatedEvent(ResponseCreatedEvent),
    ErrorEvent(ResponseErrorEvent),
    FileSearchCallCompletedEvent(ResponseFileSearchCallCompletedEvent),
    FileSearchCallInProgressEvent(ResponseFileSearchCallInProgressEvent),
    FileSearchCallSearchingEvent(ResponseFileSearchCallSearchingEvent),
    FunctionCallArgumentsDeltaEvent(ResponseFunctionCallArgumentsDeltaEvent),
    FunctionCallArgumentsDoneEvent(ResponseFunctionCallArgumentsDoneEvent),
    ShellCallCommandAddedStreamingEvent(ResponseShellCallCommandAddedStreamingEvent),
    ShellCallCommandDeltaStreamingEvent(ResponseShellCallCommandDeltaStreamingEvent),
    ShellCallCommandDoneStreamingEvent(ResponseShellCallCommandDoneStreamingEvent),
    ShellCallOutputContentDeltaStreamingEvent(ResponseShellCallOutputContentDeltaStreamingEvent),
    ShellCallOutputContentDoneStreamingEvent(ResponseShellCallOutputContentDoneStreamingEvent),
    InProgressEvent(ResponseInProgressEvent),
    FailedEvent(ResponseFailedEvent),
    IncompleteEvent(ResponseIncompleteEvent),
    OutputItemAddedEvent(ResponseOutputItemAddedEvent),
    OutputItemDoneEvent(ResponseOutputItemDoneEvent),
    ReasoningSummaryPartAddedEvent(ResponseReasoningSummaryPartAddedEvent),
    ReasoningSummaryPartDoneEvent(ResponseReasoningSummaryPartDoneEvent),
    ReasoningSummaryTextDeltaEvent(ResponseReasoningSummaryTextDeltaEvent),
    ReasoningSummaryTextDoneEvent(ResponseReasoningSummaryTextDoneEvent),
    ReasoningTextDeltaEvent(ResponseReasoningTextDeltaEvent),
    ReasoningTextDoneEvent(ResponseReasoningTextDoneEvent),
    RefusalDeltaEvent(ResponseRefusalDeltaEvent),
    RefusalDoneEvent(ResponseRefusalDoneEvent),
    TextDeltaEvent(ResponseTextDeltaEvent),
    TextDoneEvent(ResponseTextDoneEvent),
    WebSearchCallCompletedEvent(ResponseWebSearchCallCompletedEvent),
    WebSearchCallInProgressEvent(ResponseWebSearchCallInProgressEvent),
    WebSearchCallSearchingEvent(ResponseWebSearchCallSearchingEvent),
    ImageGenCallCompletedEvent(ResponseImageGenCallCompletedEvent),
    ImageGenCallGeneratingEvent(ResponseImageGenCallGeneratingEvent),
    ImageGenCallInProgressEvent(ResponseImageGenCallInProgressEvent),
    ImageGenCallPartialImageEvent(ResponseImageGenCallPartialImageEvent),
    McpCallArgumentsDeltaEvent(ResponseMcpCallArgumentsDeltaEvent),
    McpCallArgumentsDoneEvent(ResponseMcpCallArgumentsDoneEvent),
    McpCallCompletedEvent(ResponseMcpCallCompletedEvent),
    McpCallFailedEvent(ResponseMcpCallFailedEvent),
    McpCallInProgressEvent(ResponseMcpCallInProgressEvent),
    McpListToolsCompletedEvent(ResponseMcpListToolsCompletedEvent),
    McpListToolsFailedEvent(ResponseMcpListToolsFailedEvent),
    McpListToolsInProgressEvent(ResponseMcpListToolsInProgressEvent),
    OutputTextAnnotationAddedEvent(ResponseOutputTextAnnotationAddedEvent),
    QueuedEvent(ResponseQueuedEvent),
    CustomToolCallInputDeltaEvent(ResponseCustomToolCallInputDeltaEvent),
    CustomToolCallInputDoneEvent(ResponseCustomToolCallInputDoneEvent),
}
impl ::std::convert::From<ResponseAudioDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseAudioDeltaEvent) -> Self {
        Self::AudioDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseAudioDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseAudioDoneEvent) -> Self {
        Self::AudioDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseAudioTranscriptDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseAudioTranscriptDeltaEvent) -> Self {
        Self::AudioTranscriptDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseAudioTranscriptDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseAudioTranscriptDoneEvent) -> Self {
        Self::AudioTranscriptDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseCodeInterpreterCallCodeDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseCodeInterpreterCallCodeDeltaEvent) -> Self {
        Self::CodeInterpreterCallCodeDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseCodeInterpreterCallCodeDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseCodeInterpreterCallCodeDoneEvent) -> Self {
        Self::CodeInterpreterCallCodeDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseCodeInterpreterCallCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseCodeInterpreterCallCompletedEvent) -> Self {
        Self::CodeInterpreterCallCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseCodeInterpreterCallInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseCodeInterpreterCallInProgressEvent) -> Self {
        Self::CodeInterpreterCallInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseCodeInterpreterCallInterpretingEvent> for ResponseStreamEvent {
    fn from(value: ResponseCodeInterpreterCallInterpretingEvent) -> Self {
        Self::CodeInterpreterCallInterpretingEvent(value)
    }
}
impl ::std::convert::From<ResponseCompactionCompactingStreamingEvent> for ResponseStreamEvent {
    fn from(value: ResponseCompactionCompactingStreamingEvent) -> Self {
        Self::CompactionCompactingStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseCompletedEvent) -> Self {
        Self::CompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseContentPartAddedEvent> for ResponseStreamEvent {
    fn from(value: ResponseContentPartAddedEvent) -> Self {
        Self::ContentPartAddedEvent(value)
    }
}
impl ::std::convert::From<ResponseContentPartDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseContentPartDoneEvent) -> Self {
        Self::ContentPartDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseCreatedEvent> for ResponseStreamEvent {
    fn from(value: ResponseCreatedEvent) -> Self {
        Self::CreatedEvent(value)
    }
}
impl ::std::convert::From<ResponseErrorEvent> for ResponseStreamEvent {
    fn from(value: ResponseErrorEvent) -> Self {
        Self::ErrorEvent(value)
    }
}
impl ::std::convert::From<ResponseFileSearchCallCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseFileSearchCallCompletedEvent) -> Self {
        Self::FileSearchCallCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseFileSearchCallInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseFileSearchCallInProgressEvent) -> Self {
        Self::FileSearchCallInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseFileSearchCallSearchingEvent> for ResponseStreamEvent {
    fn from(value: ResponseFileSearchCallSearchingEvent) -> Self {
        Self::FileSearchCallSearchingEvent(value)
    }
}
impl ::std::convert::From<ResponseFunctionCallArgumentsDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseFunctionCallArgumentsDeltaEvent) -> Self {
        Self::FunctionCallArgumentsDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseFunctionCallArgumentsDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseFunctionCallArgumentsDoneEvent) -> Self {
        Self::FunctionCallArgumentsDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseShellCallCommandAddedStreamingEvent> for ResponseStreamEvent {
    fn from(value: ResponseShellCallCommandAddedStreamingEvent) -> Self {
        Self::ShellCallCommandAddedStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseShellCallCommandDeltaStreamingEvent> for ResponseStreamEvent {
    fn from(value: ResponseShellCallCommandDeltaStreamingEvent) -> Self {
        Self::ShellCallCommandDeltaStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseShellCallCommandDoneStreamingEvent> for ResponseStreamEvent {
    fn from(value: ResponseShellCallCommandDoneStreamingEvent) -> Self {
        Self::ShellCallCommandDoneStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseShellCallOutputContentDeltaStreamingEvent>
    for ResponseStreamEvent
{
    fn from(value: ResponseShellCallOutputContentDeltaStreamingEvent) -> Self {
        Self::ShellCallOutputContentDeltaStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseShellCallOutputContentDoneStreamingEvent>
    for ResponseStreamEvent
{
    fn from(value: ResponseShellCallOutputContentDoneStreamingEvent) -> Self {
        Self::ShellCallOutputContentDoneStreamingEvent(value)
    }
}
impl ::std::convert::From<ResponseInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseInProgressEvent) -> Self {
        Self::InProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseFailedEvent> for ResponseStreamEvent {
    fn from(value: ResponseFailedEvent) -> Self {
        Self::FailedEvent(value)
    }
}
impl ::std::convert::From<ResponseIncompleteEvent> for ResponseStreamEvent {
    fn from(value: ResponseIncompleteEvent) -> Self {
        Self::IncompleteEvent(value)
    }
}
impl ::std::convert::From<ResponseOutputItemAddedEvent> for ResponseStreamEvent {
    fn from(value: ResponseOutputItemAddedEvent) -> Self {
        Self::OutputItemAddedEvent(value)
    }
}
impl ::std::convert::From<ResponseOutputItemDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseOutputItemDoneEvent) -> Self {
        Self::OutputItemDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningSummaryPartAddedEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningSummaryPartAddedEvent) -> Self {
        Self::ReasoningSummaryPartAddedEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningSummaryPartDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningSummaryPartDoneEvent) -> Self {
        Self::ReasoningSummaryPartDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningSummaryTextDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningSummaryTextDeltaEvent) -> Self {
        Self::ReasoningSummaryTextDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningSummaryTextDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningSummaryTextDoneEvent) -> Self {
        Self::ReasoningSummaryTextDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningTextDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningTextDeltaEvent) -> Self {
        Self::ReasoningTextDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseReasoningTextDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseReasoningTextDoneEvent) -> Self {
        Self::ReasoningTextDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseRefusalDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseRefusalDeltaEvent) -> Self {
        Self::RefusalDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseRefusalDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseRefusalDoneEvent) -> Self {
        Self::RefusalDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseTextDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseTextDeltaEvent) -> Self {
        Self::TextDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseTextDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseTextDoneEvent) -> Self {
        Self::TextDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseWebSearchCallCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseWebSearchCallCompletedEvent) -> Self {
        Self::WebSearchCallCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseWebSearchCallInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseWebSearchCallInProgressEvent) -> Self {
        Self::WebSearchCallInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseWebSearchCallSearchingEvent> for ResponseStreamEvent {
    fn from(value: ResponseWebSearchCallSearchingEvent) -> Self {
        Self::WebSearchCallSearchingEvent(value)
    }
}
impl ::std::convert::From<ResponseImageGenCallCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseImageGenCallCompletedEvent) -> Self {
        Self::ImageGenCallCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseImageGenCallGeneratingEvent> for ResponseStreamEvent {
    fn from(value: ResponseImageGenCallGeneratingEvent) -> Self {
        Self::ImageGenCallGeneratingEvent(value)
    }
}
impl ::std::convert::From<ResponseImageGenCallInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseImageGenCallInProgressEvent) -> Self {
        Self::ImageGenCallInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseImageGenCallPartialImageEvent> for ResponseStreamEvent {
    fn from(value: ResponseImageGenCallPartialImageEvent) -> Self {
        Self::ImageGenCallPartialImageEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpCallArgumentsDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpCallArgumentsDeltaEvent) -> Self {
        Self::McpCallArgumentsDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpCallArgumentsDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpCallArgumentsDoneEvent) -> Self {
        Self::McpCallArgumentsDoneEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpCallCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpCallCompletedEvent) -> Self {
        Self::McpCallCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpCallFailedEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpCallFailedEvent) -> Self {
        Self::McpCallFailedEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpCallInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpCallInProgressEvent) -> Self {
        Self::McpCallInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpListToolsCompletedEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpListToolsCompletedEvent) -> Self {
        Self::McpListToolsCompletedEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpListToolsFailedEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpListToolsFailedEvent) -> Self {
        Self::McpListToolsFailedEvent(value)
    }
}
impl ::std::convert::From<ResponseMcpListToolsInProgressEvent> for ResponseStreamEvent {
    fn from(value: ResponseMcpListToolsInProgressEvent) -> Self {
        Self::McpListToolsInProgressEvent(value)
    }
}
impl ::std::convert::From<ResponseOutputTextAnnotationAddedEvent> for ResponseStreamEvent {
    fn from(value: ResponseOutputTextAnnotationAddedEvent) -> Self {
        Self::OutputTextAnnotationAddedEvent(value)
    }
}
impl ::std::convert::From<ResponseQueuedEvent> for ResponseStreamEvent {
    fn from(value: ResponseQueuedEvent) -> Self {
        Self::QueuedEvent(value)
    }
}
impl ::std::convert::From<ResponseCustomToolCallInputDeltaEvent> for ResponseStreamEvent {
    fn from(value: ResponseCustomToolCallInputDeltaEvent) -> Self {
        Self::CustomToolCallInputDeltaEvent(value)
    }
}
impl ::std::convert::From<ResponseCustomToolCallInputDoneEvent> for ResponseStreamEvent {
    fn from(value: ResponseCustomToolCallInputDoneEvent) -> Self {
        Self::CustomToolCallInputDoneEvent(value)
    }
}
#[doc = "`ResponseStreamOptions`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ResponseStreamOptions(pub ::std::option::Option<ResponseStreamOptionsValue>);
impl ::std::ops::Deref for ResponseStreamOptions {
    type Target = ::std::option::Option<ResponseStreamOptionsValue>;
    fn deref(&self) -> &::std::option::Option<ResponseStreamOptionsValue> {
        &self.0
    }
}
impl ::std::convert::From<ResponseStreamOptions>
    for ::std::option::Option<ResponseStreamOptionsValue>
{
    fn from(value: ResponseStreamOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ResponseStreamOptionsValue>>
    for ResponseStreamOptions
{
    fn from(value: ::std::option::Option<ResponseStreamOptionsValue>) -> Self {
        Self(value)
    }
}
#[doc = "Options for streaming responses. Only set this when you set `stream: true`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseStreamOptionsValue {
    #[doc = "When true, stream obfuscation will be enabled. Stream obfuscation adds\nrandom characters to an `obfuscation` field on streaming delta events to\nnormalize payload sizes as a mitigation to certain side-channel attacks.\nThese obfuscation fields are included by default, but add a small amount\nof overhead to the data stream. You can set `include_obfuscation` to\nfalse to optimize for bandwidth if you trust the network links between\nyour application and the OpenAI API.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_obfuscation: ::std::option::Option<bool>,
}
#[doc = "Emitted when there is an additional text delta."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseTextDeltaEvent {
    #[doc = "The index of the content part that the text delta was added to.\n"]
    pub content_index: i64,
    #[doc = "The text delta that was added.\n"]
    pub delta: ::std::string::String,
    #[doc = "The ID of the output item that the text delta was added to.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The log probabilities of the tokens in the delta.\n"]
    pub logprobs: ::std::vec::Vec<ResponseLogProb>,
    #[doc = "The index of the output item that the text delta was added to.\n"]
    pub output_index: i64,
    #[doc = "The sequence number for this event."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.output_text.delta`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseTextDeltaEventType,
}
#[doc = "The type of the event. Always `response.output_text.delta`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseTextDeltaEventType {
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta,
}
impl ::std::fmt::Display for ResponseTextDeltaEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputTextDelta => f.write_str("response.output_text.delta"),
        }
    }
}
impl ::std::str::FromStr for ResponseTextDeltaEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_text.delta" => Ok(Self::ResponseOutputTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseTextDeltaEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when text content is finalized."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseTextDoneEvent {
    #[doc = "The index of the content part that the text content is finalized.\n"]
    pub content_index: i64,
    #[doc = "The ID of the output item that the text content is finalized.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The log probabilities of the tokens in the delta.\n"]
    pub logprobs: ::std::vec::Vec<ResponseLogProb>,
    #[doc = "The index of the output item that the text content is finalized.\n"]
    pub output_index: i64,
    #[doc = "The sequence number for this event."]
    pub sequence_number: i64,
    #[doc = "The text content that is finalized.\n"]
    pub text: ::std::string::String,
    #[doc = "The type of the event. Always `response.output_text.done`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseTextDoneEventType,
}
#[doc = "The type of the event. Always `response.output_text.done`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseTextDoneEventType {
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone,
}
impl ::std::fmt::Display for ResponseTextDoneEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseOutputTextDone => f.write_str("response.output_text.done"),
        }
    }
}
impl ::std::str::FromStr for ResponseTextDoneEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.output_text.done" => Ok(Self::ResponseOutputTextDone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseTextDoneEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](https://developers.openai.com/api/docs/guides/text)\n- [Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs)\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseTextParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<TextResponseFormatConfiguration>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verbosity: ::std::option::Option<Verbosity>,
}
#[doc = "The truncation strategy to use for the model response.\n- `auto`: If the input to this Response exceeds\n  the model's context window size, the model will truncate the\n  response to fit the context window by dropping items from the beginning of the conversation.\n- `disabled` (default): If the input size will exceed the context window\n  size for a model, the request will fail with a 400 error.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseTruncation {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for ResponseTruncation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for ResponseTruncation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseTruncation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseTruncation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents token usage details including input tokens, output tokens,\na breakdown of output tokens, and the total tokens used.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseUsage {
    #[doc = "The number of input tokens."]
    pub input_tokens: i64,
    pub input_tokens_details: ResponseUsageInputTokensDetails,
    #[doc = "The number of output tokens."]
    pub output_tokens: i64,
    pub output_tokens_details: ResponseUsageOutputTokensDetails,
    #[doc = "The total number of tokens used."]
    pub total_tokens: i64,
}
#[doc = "A detailed breakdown of the input tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseUsageInputTokensDetails {
    #[doc = "The number of input tokens that were written to the cache."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_write_tokens: ::std::option::Option<i64>,
    #[doc = "The number of tokens that were retrieved from the cache.\n[More on prompt caching](https://developers.openai.com/api/docs/guides/prompt-caching).\n"]
    pub cached_tokens: i64,
}
#[doc = "A detailed breakdown of the output tokens."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseUsageOutputTokensDetails {
    #[doc = "The number of reasoning tokens."]
    pub reasoning_tokens: i64,
}
#[doc = "Emitted when a web search call is completed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseWebSearchCallCompletedEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of the web search call being processed."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseWebSearchCallCompletedEventType {
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted,
}
impl ::std::fmt::Display for ResponseWebSearchCallCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseWebSearchCallCompleted => {
                f.write_str("response.web_search_call.completed")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseWebSearchCallCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.web_search_call.completed" => Ok(Self::ResponseWebSearchCallCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseWebSearchCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseWebSearchCallCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a web search call is initiated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseWebSearchCallInProgressEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of the web search call being processed."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseWebSearchCallInProgressEventType {
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress,
}
impl ::std::fmt::Display for ResponseWebSearchCallInProgressEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseWebSearchCallInProgress => {
                f.write_str("response.web_search_call.in_progress")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseWebSearchCallInProgressEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.web_search_call.in_progress" => Ok(Self::ResponseWebSearchCallInProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseWebSearchCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseWebSearchCallInProgressEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a web search call is executing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponseWebSearchCallSearchingEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: ::std::string::String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: i64,
    #[doc = "The sequence number of the web search call being processed."]
    pub sequence_number: i64,
    #[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallSearchingEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseWebSearchCallSearchingEventType {
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching,
}
impl ::std::fmt::Display for ResponseWebSearchCallSearchingEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseWebSearchCallSearching => {
                f.write_str("response.web_search_call.searching")
            }
        }
    }
}
impl ::std::str::FromStr for ResponseWebSearchCallSearchingEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.web_search_call.searching" => Ok(Self::ResponseWebSearchCallSearching),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseWebSearchCallSearchingEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseWebSearchCallSearchingEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponsesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ResponsesApiSchema {
    #[serde(rename = "CompactResource")]
    pub compact_resource: CompactResource,
    #[serde(rename = "CompactResponseBody")]
    pub compact_response_body: CompactResponseMethodPublicBody,
    #[serde(rename = "CreateResponse")]
    pub create_response: CreateResponse,
    #[serde(rename = "DeleteResponse")]
    pub delete_response: DeleteResponse,
    #[serde(rename = "ErrorResponse")]
    pub error_response: ErrorResponse,
    #[serde(rename = "ListInputItemsParams")]
    pub list_input_items_params: ListInputItemsParams,
    #[serde(rename = "Response")]
    pub response: Response,
    #[serde(rename = "ResponseItemList")]
    pub response_item_list: ResponseItemList,
    #[serde(rename = "ResponseStreamEvent")]
    pub response_stream_event: ResponseStreamEvent,
    #[serde(rename = "RetrieveResponseParams")]
    pub retrieve_response_params: RetrieveResponseParams,
    #[serde(rename = "RetrieveResponseStreamParams")]
    pub retrieve_response_stream_params: RetrieveResponseStreamParams,
    #[serde(rename = "TokenCountsBody")]
    pub token_counts_body: TokenCountsBody,
    #[serde(rename = "TokenCountsResource")]
    pub token_counts_resource: TokenCountsResource,
}
#[doc = "`RetrieveResponseParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RetrieveResponseParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<IncludeEnum>,
}
#[doc = "`RetrieveResponseStreamParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct RetrieveResponseStreamParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub include: ::std::vec::Vec<IncludeEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_obfuscation: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub starting_after: ::std::option::Option<i64>,
}
#[doc = "A screenshot action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ScreenshotParam {
    #[doc = "Specifies the event type. For a screenshot action, this property is always set to `screenshot`."]
    #[serde(rename = "type")]
    pub type_: ScreenshotParamType,
}
#[doc = "Specifies the event type. For a screenshot action, this property is always set to `screenshot`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScreenshotParamType {
    #[serde(rename = "screenshot")]
    Screenshot,
}
impl ::std::fmt::Display for ScreenshotParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Screenshot => f.write_str("screenshot"),
        }
    }
}
impl ::std::str::FromStr for ScreenshotParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "screenshot" => Ok(Self::Screenshot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScreenshotParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScreenshotParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A scroll action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ScrollParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "The horizontal scroll distance."]
    pub scroll_x: i64,
    #[doc = "The vertical scroll distance."]
    pub scroll_y: i64,
    #[doc = "Specifies the event type. For a scroll action, this property is always set to `scroll`."]
    #[serde(rename = "type")]
    pub type_: ScrollParamType,
    #[doc = "The x-coordinate where the scroll occurred."]
    pub x: i64,
    #[doc = "The y-coordinate where the scroll occurred."]
    pub y: i64,
}
#[doc = "Specifies the event type. For a scroll action, this property is always set to `scroll`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScrollParamType {
    #[serde(rename = "scroll")]
    Scroll,
}
impl ::std::fmt::Display for ScrollParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Scroll => f.write_str("scroll"),
        }
    }
}
impl ::std::str::FromStr for ScrollParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "scroll" => Ok(Self::Scroll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScrollParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScrollParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SearchContentType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SearchContentType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
impl ::std::fmt::Display for SearchContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
        }
    }
}
impl ::std::str::FromStr for SearchContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SearchContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SearchContextSize`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for SearchContextSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for SearchContextSize {
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
impl ::std::convert::TryFrom<&str> for SearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ServiceTierEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ServiceTierEnum {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "flex")]
    Flex,
    #[serde(rename = "priority")]
    Priority,
}
impl ::std::fmt::Display for ServiceTierEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::Fast => f.write_str("fast"),
            Self::Flex => f.write_str("flex"),
            Self::Priority => f.write_str("priority"),
        }
    }
}
impl ::std::str::FromStr for ServiceTierEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "fast" => Ok(Self::Fast),
            "flex" => Ok(Self::Flex),
            "priority" => Ok(Self::Priority),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ServiceTierEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ServiceTierEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ServiceTierResponses`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ServiceTierResponses(pub ::std::option::Option<ServiceTierResponsesValue>);
impl ::std::ops::Deref for ServiceTierResponses {
    type Target = ::std::option::Option<ServiceTierResponsesValue>;
    fn deref(&self) -> &::std::option::Option<ServiceTierResponsesValue> {
        &self.0
    }
}
impl ::std::convert::From<ServiceTierResponses>
    for ::std::option::Option<ServiceTierResponsesValue>
{
    fn from(value: ServiceTierResponses) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<ServiceTierResponsesValue>>
    for ServiceTierResponses
{
    fn from(value: ::std::option::Option<ServiceTierResponsesValue>) -> Self {
        Self(value)
    }
}
#[doc = "Specifies the processing type used for serving the request.\n  - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.\n  - If set to 'default', then the request will be processed with the standard pricing and performance for the selected model.\n  - If set to '[flex](https://developers.openai.com/api/docs/guides/flex-processing)', then the request will be processed with the Flex Processing service tier.\n  - To opt-in to [Fast mode](https://developers.openai.com/api/docs/guides/fast-mode) at the request level, include the `service_tier=fast` or `service_tier=priority` parameter for Responses or Chat Completions. The response will show `service_tier=priority` regardless of if you specify `service_tier=fast` or `priority` in your request.\n  - If set to 'ultrafast', then the request will be processed with the access-controlled Ultrafast Processing service tier. This tier is currently available for `gpt-5.6-sol`; a response served through it will show `service_tier=ultrafast`.\n  - When not set, the default behavior is 'auto'.\n\n  When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ServiceTierResponsesValue {
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
    #[serde(rename = "ultrafast")]
    Ultrafast,
}
impl ::std::fmt::Display for ServiceTierResponsesValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::Flex => f.write_str("flex"),
            Self::Scale => f.write_str("scale"),
            Self::Priority => f.write_str("priority"),
            Self::Fast => f.write_str("fast"),
            Self::Ultrafast => f.write_str("ultrafast"),
        }
    }
}
impl ::std::str::FromStr for ServiceTierResponsesValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "flex" => Ok(Self::Flex),
            "scale" => Ok(Self::Scale),
            "priority" => Ok(Self::Priority),
            "fast" => Ok(Self::Fast),
            "ultrafast" => Ok(Self::Ultrafast),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ServiceTierResponsesValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ServiceTierResponsesValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents either an exit outcome (with an exit code) or a timeout outcome for a shell call output chunk."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ShellCallOutcome {
    TimeoutOutcome(FunctionShellCallOutputTimeoutOutcome),
    ExitOutcome(FunctionShellCallOutputExitOutcome),
}
impl ::std::convert::From<FunctionShellCallOutputTimeoutOutcome> for ShellCallOutcome {
    fn from(value: FunctionShellCallOutputTimeoutOutcome) -> Self {
        Self::TimeoutOutcome(value)
    }
}
impl ::std::convert::From<FunctionShellCallOutputExitOutcome> for ShellCallOutcome {
    fn from(value: FunctionShellCallOutputExitOutcome) -> Self {
        Self::ExitOutcome(value)
    }
}
#[doc = "A delta of stdout/stderr emitted while a shell call was running."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ShellCallOutputDelta {
    #[doc = "The stderr delta that was emitted."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stderr: ::std::option::Option<::std::string::String>,
    #[doc = "The stdout delta that was emitted."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stdout: ::std::option::Option<::std::string::String>,
}
#[doc = "`SkillReferenceParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillReferenceParam {
    #[doc = "The ID of the referenced skill."]
    pub skill_id: SkillReferenceParamSkillId,
    #[doc = "References a skill created with the /v1/skills endpoint."]
    #[serde(rename = "type")]
    pub type_: SkillReferenceParamType,
    #[doc = "Optional skill version. Use a positive integer or 'latest'. Omit for default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
#[doc = "The ID of the referenced skill."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SkillReferenceParamSkillId(::std::string::String);
impl ::std::ops::Deref for SkillReferenceParamSkillId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SkillReferenceParamSkillId> for ::std::string::String {
    fn from(value: SkillReferenceParamSkillId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SkillReferenceParamSkillId {
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
impl ::std::convert::TryFrom<&str> for SkillReferenceParamSkillId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillReferenceParamSkillId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SkillReferenceParamSkillId {
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
pub enum SkillReferenceParamType {
    #[serde(rename = "skill_reference")]
    SkillReference,
}
impl ::std::fmt::Display for SkillReferenceParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SkillReference => f.write_str("skill_reference"),
        }
    }
}
impl ::std::str::FromStr for SkillReferenceParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill_reference" => Ok(Self::SkillReference),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillReferenceParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Forces the model to call the apply_patch tool when executing a tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpecificApplyPatchParam {
    #[doc = "The tool to call. Always `apply_patch`."]
    #[serde(rename = "type")]
    pub type_: SpecificApplyPatchParamType,
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
pub enum SpecificApplyPatchParamType {
    #[serde(rename = "apply_patch")]
    ApplyPatch,
}
impl ::std::fmt::Display for SpecificApplyPatchParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplyPatch => f.write_str("apply_patch"),
        }
    }
}
impl ::std::str::FromStr for SpecificApplyPatchParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "apply_patch" => Ok(Self::ApplyPatch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpecificApplyPatchParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpecificApplyPatchParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Forces the model to call the shell tool when a tool call is required."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpecificFunctionShellParam {
    #[doc = "The tool to call. Always `shell`."]
    #[serde(rename = "type")]
    pub type_: SpecificFunctionShellParamType,
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
pub enum SpecificFunctionShellParamType {
    #[serde(rename = "shell")]
    Shell,
}
impl ::std::fmt::Display for SpecificFunctionShellParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
        }
    }
}
impl ::std::str::FromStr for SpecificFunctionShellParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpecificFunctionShellParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpecificFunctionShellParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SpecificProgrammaticToolCallingParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpecificProgrammaticToolCallingParam {
    #[doc = "The tool to call. Always `programmatic_tool_calling`."]
    #[serde(rename = "type")]
    pub type_: SpecificProgrammaticToolCallingParamType,
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
pub enum SpecificProgrammaticToolCallingParamType {
    #[serde(rename = "programmatic_tool_calling")]
    ProgrammaticToolCalling,
}
impl ::std::fmt::Display for SpecificProgrammaticToolCallingParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProgrammaticToolCalling => f.write_str("programmatic_tool_calling"),
        }
    }
}
impl ::std::str::FromStr for SpecificProgrammaticToolCallingParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "programmatic_tool_calling" => Ok(Self::ProgrammaticToolCalling),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpecificProgrammaticToolCallingParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpecificProgrammaticToolCallingParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A summary text from the model."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SummaryTextContent {
    #[doc = "A summary of the reasoning output from the model so far."]
    pub text: ::std::string::String,
    #[doc = "The type of the object. Always `summary_text`."]
    #[serde(rename = "type")]
    pub type_: SummaryTextContentType,
}
#[doc = "The type of the object. Always `summary_text`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SummaryTextContentType {
    #[serde(rename = "summary_text")]
    SummaryText,
}
impl ::std::fmt::Display for SummaryTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SummaryText => f.write_str("summary_text"),
        }
    }
}
impl ::std::str::FromStr for SummaryTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "summary_text" => Ok(Self::SummaryText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SummaryTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SummaryTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text content."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TextContent {
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: TextContentType,
}
#[doc = "`TextContentType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TextContentType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for TextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for TextContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TextContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An object specifying the format that the model must output.\n\nConfiguring `{ \"type\": \"json_schema\" }` enables Structured Outputs,\nwhich ensures the model will match your supplied JSON schema. Learn more in the\n[Structured Outputs guide](https://developers.openai.com/api/docs/guides/structured-outputs).\n\nThe default format is `{ \"type\": \"text\" }` with no additional options.\n\n**Not recommended for gpt-4o and newer models:**\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TextResponseFormatConfiguration {
    ResponseFormatText(ResponseFormatText),
    TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),
    ResponseFormatJsonObject(ResponseFormatJsonObject),
}
impl ::std::convert::From<ResponseFormatText> for TextResponseFormatConfiguration {
    fn from(value: ResponseFormatText) -> Self {
        Self::ResponseFormatText(value)
    }
}
impl ::std::convert::From<TextResponseFormatJsonSchema> for TextResponseFormatConfiguration {
    fn from(value: TextResponseFormatJsonSchema) -> Self {
        Self::TextResponseFormatJsonSchema(value)
    }
}
impl ::std::convert::From<ResponseFormatJsonObject> for TextResponseFormatConfiguration {
    fn from(value: ResponseFormatJsonObject) -> Self {
        Self::ResponseFormatJsonObject(value)
    }
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TextResponseFormatJsonSchema {
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    pub name: ::std::string::String,
    pub schema: ResponseFormatJsonSchemaSchema,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
    #[doc = "The type of response format being defined. Always `json_schema`."]
    #[serde(rename = "type")]
    pub type_: TextResponseFormatJsonSchemaType,
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
pub enum TextResponseFormatJsonSchemaType {
    #[serde(rename = "json_schema")]
    JsonSchema,
}
impl ::std::fmt::Display for TextResponseFormatJsonSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonSchema => f.write_str("json_schema"),
        }
    }
}
impl ::std::str::FromStr for TextResponseFormatJsonSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json_schema" => Ok(Self::JsonSchema),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TextResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TextResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TokenCountsBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct TokenCountsBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conversation: ::std::option::Option<ConversationParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<TokenCountsBodyInput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parallel_tool_calls: ::std::option::Option<bool>,
    #[doc = "A model-owned style preset to apply to this request. Omit this parameter to use the model's default style. Supported values may expand over time. Values must be at most 64 characters."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub personality: ::std::option::Option<PersonalityEnum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous_response_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<Reasoning>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<ResponseTextParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<ToolChoiceParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<::std::vec::Vec<Tool>>,
    #[doc = "The truncation strategy to use for the model response. - `auto`: If the input to this Response exceeds the model's context window size, the model will truncate the response to fit the context window by dropping items from the beginning of the conversation. - `disabled` (default): If the input size will exceed the context window size for a model, the request will fail with a 400 error."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub truncation: ::std::option::Option<TruncationEnum>,
}
#[doc = "Text, image, or file inputs to the model, used to generate a response"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TokenCountsBodyInput {
    String(TokenCountsBodyInputString),
    Array(::std::vec::Vec<InputItem>),
}
impl ::std::convert::From<TokenCountsBodyInputString> for TokenCountsBodyInput {
    fn from(value: TokenCountsBodyInputString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<InputItem>> for TokenCountsBodyInput {
    fn from(value: ::std::vec::Vec<InputItem>) -> Self {
        Self::Array(value)
    }
}
#[doc = "A text input to the model, equivalent to a text input with the `user` role."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TokenCountsBodyInputString(::std::string::String);
impl ::std::ops::Deref for TokenCountsBodyInputString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TokenCountsBodyInputString> for ::std::string::String {
    fn from(value: TokenCountsBodyInputString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TokenCountsBodyInputString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TokenCountsBodyInputString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TokenCountsBodyInputString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TokenCountsBodyInputString {
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
#[doc = "`TokenCountsResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TokenCountsResource {
    pub input_tokens: i64,
    pub object: TokenCountsResourceObject,
}
#[doc = "`TokenCountsResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TokenCountsResourceObject {
    #[serde(rename = "response.input_tokens")]
    ResponseInputTokens,
}
impl ::std::fmt::Display for TokenCountsResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ResponseInputTokens => f.write_str("response.input_tokens"),
        }
    }
}
impl ::std::str::FromStr for TokenCountsResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "response.input_tokens" => Ok(Self::ResponseInputTokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TokenCountsResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TokenCountsResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool that can be used to generate a response.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Tool {
    FunctionTool(FunctionTool),
    FileSearchTool(FileSearchTool),
    ComputerTool(ComputerTool),
    ComputerUsePreviewTool(ComputerUsePreviewTool),
    WebSearchTool(WebSearchTool),
    McpTool(McpTool),
    CodeInterpreterTool(CodeInterpreterTool),
    ProgrammaticToolCallingParam(ProgrammaticToolCallingParam),
    ImageGenTool(ImageGenTool),
    LocalShellToolParam(LocalShellToolParam),
    FunctionShellToolParam(FunctionShellToolParam),
    CustomToolParam(CustomToolParam),
    NamespaceToolParam(NamespaceToolParam),
    ToolSearchToolParam(ToolSearchToolParam),
    WebSearchPreviewTool(WebSearchPreviewTool),
    ApplyPatchToolParam(ApplyPatchToolParam),
}
impl ::std::convert::From<FunctionTool> for Tool {
    fn from(value: FunctionTool) -> Self {
        Self::FunctionTool(value)
    }
}
impl ::std::convert::From<FileSearchTool> for Tool {
    fn from(value: FileSearchTool) -> Self {
        Self::FileSearchTool(value)
    }
}
impl ::std::convert::From<ComputerTool> for Tool {
    fn from(value: ComputerTool) -> Self {
        Self::ComputerTool(value)
    }
}
impl ::std::convert::From<ComputerUsePreviewTool> for Tool {
    fn from(value: ComputerUsePreviewTool) -> Self {
        Self::ComputerUsePreviewTool(value)
    }
}
impl ::std::convert::From<WebSearchTool> for Tool {
    fn from(value: WebSearchTool) -> Self {
        Self::WebSearchTool(value)
    }
}
impl ::std::convert::From<McpTool> for Tool {
    fn from(value: McpTool) -> Self {
        Self::McpTool(value)
    }
}
impl ::std::convert::From<CodeInterpreterTool> for Tool {
    fn from(value: CodeInterpreterTool) -> Self {
        Self::CodeInterpreterTool(value)
    }
}
impl ::std::convert::From<ProgrammaticToolCallingParam> for Tool {
    fn from(value: ProgrammaticToolCallingParam) -> Self {
        Self::ProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<ImageGenTool> for Tool {
    fn from(value: ImageGenTool) -> Self {
        Self::ImageGenTool(value)
    }
}
impl ::std::convert::From<LocalShellToolParam> for Tool {
    fn from(value: LocalShellToolParam) -> Self {
        Self::LocalShellToolParam(value)
    }
}
impl ::std::convert::From<FunctionShellToolParam> for Tool {
    fn from(value: FunctionShellToolParam) -> Self {
        Self::FunctionShellToolParam(value)
    }
}
impl ::std::convert::From<CustomToolParam> for Tool {
    fn from(value: CustomToolParam) -> Self {
        Self::CustomToolParam(value)
    }
}
impl ::std::convert::From<NamespaceToolParam> for Tool {
    fn from(value: NamespaceToolParam) -> Self {
        Self::NamespaceToolParam(value)
    }
}
impl ::std::convert::From<ToolSearchToolParam> for Tool {
    fn from(value: ToolSearchToolParam) -> Self {
        Self::ToolSearchToolParam(value)
    }
}
impl ::std::convert::From<WebSearchPreviewTool> for Tool {
    fn from(value: WebSearchPreviewTool) -> Self {
        Self::WebSearchPreviewTool(value)
    }
}
impl ::std::convert::From<ApplyPatchToolParam> for Tool {
    fn from(value: ApplyPatchToolParam) -> Self {
        Self::ApplyPatchToolParam(value)
    }
}
#[doc = "The execution context that produced this tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolCallCaller {
    DirectToolCallCaller(DirectToolCallCaller),
    ProgramToolCallCaller(ProgramToolCallCaller),
}
impl ::std::convert::From<DirectToolCallCaller> for ToolCallCaller {
    fn from(value: DirectToolCallCaller) -> Self {
        Self::DirectToolCallCaller(value)
    }
}
impl ::std::convert::From<ProgramToolCallCaller> for ToolCallCaller {
    fn from(value: ProgramToolCallCaller) -> Self {
        Self::ProgramToolCallCaller(value)
    }
}
#[doc = "The execution context that produced this tool call."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolCallCallerParam {
    DirectToolCallCallerParam(DirectToolCallCallerParam),
    ProgramToolCallCallerParam(ProgramToolCallCallerParam),
}
impl ::std::convert::From<DirectToolCallCallerParam> for ToolCallCallerParam {
    fn from(value: DirectToolCallCallerParam) -> Self {
        Self::DirectToolCallCallerParam(value)
    }
}
impl ::std::convert::From<ProgramToolCallCallerParam> for ToolCallCallerParam {
    fn from(value: ProgramToolCallCallerParam) -> Self {
        Self::ProgramToolCallCallerParam(value)
    }
}
#[doc = "Constrains the tools available to the model to a pre-defined set.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceAllowed {
    #[doc = "Constrains the tools available to the model to a pre-defined set.\n\n`auto` allows the model to pick from among the allowed tools and generate a\nmessage.\n\n`required` requires the model to call one or more of the allowed tools.\n"]
    pub mode: ToolChoiceAllowedMode,
    #[doc = "A list of tool definitions that the model should be allowed to call.\n\nFor the Responses API, the list of tool definitions might look like:\n```json\n[\n  { \"type\": \"function\", \"name\": \"get_weather\" },\n  { \"type\": \"mcp\", \"server_label\": \"deepwiki\" },\n  { \"type\": \"image_generation\" }\n]\n```\n"]
    pub tools: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[doc = "Allowed tool configuration type. Always `allowed_tools`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceAllowedType,
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
pub enum ToolChoiceAllowedMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ToolChoiceAllowedMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceAllowedMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceAllowedMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceAllowedMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub enum ToolChoiceAllowedType {
    #[serde(rename = "allowed_tools")]
    AllowedTools,
}
impl ::std::fmt::Display for ToolChoiceAllowedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllowedTools => f.write_str("allowed_tools"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceAllowedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowed_tools" => Ok(Self::AllowedTools),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceAllowedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceAllowedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Use this option to force the model to call a specific custom tool.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceCustom {
    #[doc = "The name of the custom tool to call."]
    pub name: ::std::string::String,
    #[doc = "For custom tool calling, the type is always `custom`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceCustomType,
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
pub enum ToolChoiceCustomType {
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for ToolChoiceCustomType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceCustomType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceCustomType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceCustomType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Use this option to force the model to call a specific function.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: ::std::string::String,
    #[doc = "For function calling, the type is always `function`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceFunctionType,
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
pub enum ToolChoiceFunctionType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ToolChoiceFunctionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceFunctionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceFunctionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Use this option to force the model to call a specific tool on a remote MCP server.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceMcp {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The label of the MCP server to use.\n"]
    pub server_label: ::std::string::String,
    #[doc = "For MCP tools, the type is always `mcp`."]
    #[serde(rename = "type")]
    pub type_: ToolChoiceMcpType,
}
#[doc = "For MCP tools, the type is always `mcp`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolChoiceMcpType {
    #[serde(rename = "mcp")]
    Mcp,
}
impl ::std::fmt::Display for ToolChoiceMcpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mcp => f.write_str("mcp"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceMcpType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp" => Ok(Self::Mcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceMcpType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceMcpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Controls which (if any) tool is called by the model.\n\n`none` means the model will not call any tool and instead generates a message.\n\n`auto` means the model can pick between generating a message or calling one or\nmore tools.\n\n`required` means the model must call one or more tools.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolChoiceOptions {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ToolChoiceOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceOptions {
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
impl ::std::convert::TryFrom<&str> for ToolChoiceOptions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceOptions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "How the model should select which tool (or tools) to use when generating\na response. See the `tools` parameter to see how to specify which tools\nthe model can call.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolChoiceParam {
    ToolChoiceOptions(ToolChoiceOptions),
    ToolChoiceAllowed(ToolChoiceAllowed),
    ToolChoiceTypes(ToolChoiceTypes),
    ToolChoiceFunction(ToolChoiceFunction),
    ToolChoiceMcp(ToolChoiceMcp),
    ToolChoiceCustom(ToolChoiceCustom),
    SpecificProgrammaticToolCallingParam(SpecificProgrammaticToolCallingParam),
    SpecificApplyPatchParam(SpecificApplyPatchParam),
    SpecificFunctionShellParam(SpecificFunctionShellParam),
}
impl ::std::convert::From<ToolChoiceOptions> for ToolChoiceParam {
    fn from(value: ToolChoiceOptions) -> Self {
        Self::ToolChoiceOptions(value)
    }
}
impl ::std::convert::From<ToolChoiceAllowed> for ToolChoiceParam {
    fn from(value: ToolChoiceAllowed) -> Self {
        Self::ToolChoiceAllowed(value)
    }
}
impl ::std::convert::From<ToolChoiceTypes> for ToolChoiceParam {
    fn from(value: ToolChoiceTypes) -> Self {
        Self::ToolChoiceTypes(value)
    }
}
impl ::std::convert::From<ToolChoiceFunction> for ToolChoiceParam {
    fn from(value: ToolChoiceFunction) -> Self {
        Self::ToolChoiceFunction(value)
    }
}
impl ::std::convert::From<ToolChoiceMcp> for ToolChoiceParam {
    fn from(value: ToolChoiceMcp) -> Self {
        Self::ToolChoiceMcp(value)
    }
}
impl ::std::convert::From<ToolChoiceCustom> for ToolChoiceParam {
    fn from(value: ToolChoiceCustom) -> Self {
        Self::ToolChoiceCustom(value)
    }
}
impl ::std::convert::From<SpecificProgrammaticToolCallingParam> for ToolChoiceParam {
    fn from(value: SpecificProgrammaticToolCallingParam) -> Self {
        Self::SpecificProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<SpecificApplyPatchParam> for ToolChoiceParam {
    fn from(value: SpecificApplyPatchParam) -> Self {
        Self::SpecificApplyPatchParam(value)
    }
}
impl ::std::convert::From<SpecificFunctionShellParam> for ToolChoiceParam {
    fn from(value: SpecificFunctionShellParam) -> Self {
        Self::SpecificFunctionShellParam(value)
    }
}
#[doc = "Indicates that the model should use a built-in tool to generate a response.\n[Learn more about built-in tools](https://developers.openai.com/api/docs/guides/tools).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolChoiceTypes {
    #[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](https://developers.openai.com/api/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer`\n- `computer_use_preview`\n- `computer_use`\n- `code_interpreter`\n- `image_generation`\n"]
    #[serde(rename = "type")]
    pub type_: ToolChoiceTypesType,
}
#[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](https://developers.openai.com/api/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer`\n- `computer_use_preview`\n- `computer_use`\n- `code_interpreter`\n- `image_generation`\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolChoiceTypesType {
    #[serde(rename = "file_search")]
    FileSearch,
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
    #[serde(rename = "computer_use")]
    ComputerUse,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
    #[serde(rename = "image_generation")]
    ImageGeneration,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
impl ::std::fmt::Display for ToolChoiceTypesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileSearch => f.write_str("file_search"),
            Self::WebSearchPreview => f.write_str("web_search_preview"),
            Self::Computer => f.write_str("computer"),
            Self::ComputerUsePreview => f.write_str("computer_use_preview"),
            Self::ComputerUse => f.write_str("computer_use"),
            Self::WebSearchPreview20250311 => f.write_str("web_search_preview_2025_03_11"),
            Self::ImageGeneration => f.write_str("image_generation"),
            Self::CodeInterpreter => f.write_str("code_interpreter"),
        }
    }
}
impl ::std::str::FromStr for ToolChoiceTypesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file_search" => Ok(Self::FileSearch),
            "web_search_preview" => Ok(Self::WebSearchPreview),
            "computer" => Ok(Self::Computer),
            "computer_use_preview" => Ok(Self::ComputerUsePreview),
            "computer_use" => Ok(Self::ComputerUse),
            "web_search_preview_2025_03_11" => Ok(Self::WebSearchPreview20250311),
            "image_generation" => Ok(Self::ImageGeneration),
            "code_interpreter" => Ok(Self::CodeInterpreter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolChoiceTypesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolChoiceTypesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ToolSearchCall`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchCall {
    #[doc = "Arguments used for the tool search call."]
    pub arguments: ::serde_json::Value,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub call_id: ::std::option::Option<::std::string::String>,
    #[doc = "The identifier of the actor that created the item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "Whether tool search was executed by the server or by the client."]
    pub execution: ToolSearchExecutionType,
    #[doc = "The unique ID of the tool search call item."]
    pub id: ::std::string::String,
    #[doc = "The status of the tool search call item that was recorded."]
    pub status: FunctionCallStatus,
    #[doc = "The type of the item. Always `tool_search_call`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchCallType,
}
#[doc = "`ToolSearchCallItemParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchCallItemParam {
    #[doc = "The arguments supplied to the tool search call."]
    pub arguments: EmptyModelParam,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<ToolSearchCallItemParamCallId>,
    #[doc = "Whether tool search was executed by the server or by the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<ToolSearchExecutionType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionCallItemStatus>,
    #[doc = "The item type. Always `tool_search_call`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchCallItemParamType,
}
#[doc = "The unique ID of the tool search call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ToolSearchCallItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ToolSearchCallItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ToolSearchCallItemParamCallId> for ::std::string::String {
    fn from(value: ToolSearchCallItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ToolSearchCallItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ToolSearchCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchCallItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ToolSearchCallItemParamCallId {
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
#[doc = "The item type. Always `tool_search_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchCallItemParamType {
    #[serde(rename = "tool_search_call")]
    ToolSearchCall,
}
impl ::std::fmt::Display for ToolSearchCallItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearchCall => f.write_str("tool_search_call"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchCallItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search_call" => Ok(Self::ToolSearchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchCallItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the item. Always `tool_search_call`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchCallType {
    #[serde(rename = "tool_search_call")]
    ToolSearchCall,
}
impl ::std::fmt::Display for ToolSearchCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearchCall => f.write_str("tool_search_call"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search_call" => Ok(Self::ToolSearchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ToolSearchExecutionType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchExecutionType {
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "client")]
    Client,
}
impl ::std::fmt::Display for ToolSearchExecutionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Server => f.write_str("server"),
            Self::Client => f.write_str("client"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchExecutionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server" => Ok(Self::Server),
            "client" => Ok(Self::Client),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchExecutionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ToolSearchOutput`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchOutput {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub call_id: ::std::option::Option<::std::string::String>,
    #[doc = "The identifier of the actor that created the item."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_by: ::std::option::Option<::std::string::String>,
    #[doc = "Whether tool search was executed by the server or by the client."]
    pub execution: ToolSearchExecutionType,
    #[doc = "The unique ID of the tool search output item."]
    pub id: ::std::string::String,
    #[doc = "The status of the tool search output item that was recorded."]
    pub status: FunctionCallOutputStatusEnum,
    #[doc = "The loaded tool definitions returned by tool search."]
    pub tools: ::std::vec::Vec<Tool>,
    #[doc = "The type of the item. Always `tool_search_output`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchOutputType,
}
#[doc = "`ToolSearchOutputFunctionToolParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchOutputFunctionToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_callers: ::std::option::Option<::std::vec::Vec<CallableToolAllowedCaller>>,
    #[doc = "Whether the tool response can be returned asynchronously versus immediately returned on next response creation."]
    #[serde(
        rename = "async",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub async_: ::std::option::Option<bool>,
    #[doc = "Whether this function should be deferred and discovered via tool search."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defer_loading: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the loaded function tool."]
    pub name: ToolSearchOutputFunctionToolParamName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_schema:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<EmptyModelParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strict: ::std::option::Option<bool>,
    #[serde(rename = "type")]
    pub type_: ToolSearchOutputFunctionToolParamType,
}
#[doc = "The name of the loaded function tool."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ToolSearchOutputFunctionToolParamName(::std::string::String);
impl ::std::ops::Deref for ToolSearchOutputFunctionToolParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ToolSearchOutputFunctionToolParamName> for ::std::string::String {
    fn from(value: ToolSearchOutputFunctionToolParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ToolSearchOutputFunctionToolParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^(?![\\s\\S]*[\\r\\n])[a-zA-Z0-9_.-]+$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![\\s\\S]*[\\r\\n])[a-zA-Z0-9_.-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputFunctionToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputFunctionToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ToolSearchOutputFunctionToolParamName {
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
#[doc = "`ToolSearchOutputFunctionToolParamType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchOutputFunctionToolParamType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ToolSearchOutputFunctionToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchOutputFunctionToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputFunctionToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputFunctionToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ToolSearchOutputItemParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchOutputItemParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub call_id: ::std::option::Option<ToolSearchOutputItemParamCallId>,
    #[doc = "Whether tool search was executed by the server or by the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<ToolSearchExecutionType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FunctionCallItemStatus>,
    #[doc = "The loaded tool definitions returned by the tool search output."]
    pub tools: ::std::vec::Vec<ToolSearchOutputTool>,
    #[doc = "The item type. Always `tool_search_output`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchOutputItemParamType,
}
#[doc = "The unique ID of the tool search call generated by the model."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ToolSearchOutputItemParamCallId(::std::string::String);
impl ::std::ops::Deref for ToolSearchOutputItemParamCallId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ToolSearchOutputItemParamCallId> for ::std::string::String {
    fn from(value: ToolSearchOutputItemParamCallId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ToolSearchOutputItemParamCallId {
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
impl ::std::convert::TryFrom<&str> for ToolSearchOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputItemParamCallId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ToolSearchOutputItemParamCallId {
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
#[doc = "The item type. Always `tool_search_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchOutputItemParamType {
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput,
}
impl ::std::fmt::Display for ToolSearchOutputItemParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearchOutput => f.write_str("tool_search_output"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchOutputItemParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search_output" => Ok(Self::ToolSearchOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputItemParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Groups function/custom tools under a shared namespace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchOutputNamespaceToolParam {
    #[doc = "A description of the namespace shown to the model."]
    pub description: ::std::string::String,
    #[doc = "The namespace name used in tool calls (for example, `crm`)."]
    pub name: ToolSearchOutputNamespaceToolParamName,
    #[doc = "The function/custom tools loaded inside this namespace."]
    pub tools: ::std::vec::Vec<ToolSearchOutputNamespaceToolParamToolsItem>,
    #[doc = "The type of the tool. Always `namespace`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchOutputNamespaceToolParamType,
}
#[doc = "The namespace name used in tool calls (for example, `crm`)."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ToolSearchOutputNamespaceToolParamName(::std::string::String);
impl ::std::ops::Deref for ToolSearchOutputNamespaceToolParamName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ToolSearchOutputNamespaceToolParamName> for ::std::string::String {
    fn from(value: ToolSearchOutputNamespaceToolParamName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ToolSearchOutputNamespaceToolParamName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputNamespaceToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputNamespaceToolParamName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ToolSearchOutputNamespaceToolParamName {
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
#[doc = "A function or custom tool loaded into a namespace by tool search."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolSearchOutputNamespaceToolParamToolsItem {
    ToolSearchOutputFunctionToolParam(ToolSearchOutputFunctionToolParam),
    CustomToolParam(CustomToolParam),
}
impl ::std::convert::From<ToolSearchOutputFunctionToolParam>
    for ToolSearchOutputNamespaceToolParamToolsItem
{
    fn from(value: ToolSearchOutputFunctionToolParam) -> Self {
        Self::ToolSearchOutputFunctionToolParam(value)
    }
}
impl ::std::convert::From<CustomToolParam> for ToolSearchOutputNamespaceToolParamToolsItem {
    fn from(value: CustomToolParam) -> Self {
        Self::CustomToolParam(value)
    }
}
#[doc = "The type of the tool. Always `namespace`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchOutputNamespaceToolParamType {
    #[serde(rename = "namespace")]
    Namespace,
}
impl ::std::fmt::Display for ToolSearchOutputNamespaceToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Namespace => f.write_str("namespace"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchOutputNamespaceToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "namespace" => Ok(Self::Namespace),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputNamespaceToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputNamespaceToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool definition returned by a tool search output.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ToolSearchOutputTool {
    FunctionTool(FunctionTool),
    FileSearchTool(FileSearchTool),
    ComputerTool(ComputerTool),
    ComputerUsePreviewTool(ComputerUsePreviewTool),
    WebSearchTool(WebSearchTool),
    McpTool(McpTool),
    CodeInterpreterTool(CodeInterpreterTool),
    ProgrammaticToolCallingParam(ProgrammaticToolCallingParam),
    ImageGenTool(ImageGenTool),
    LocalShellToolParam(LocalShellToolParam),
    FunctionShellToolParam(FunctionShellToolParam),
    CustomToolParam(CustomToolParam),
    ToolSearchOutputNamespaceToolParam(ToolSearchOutputNamespaceToolParam),
    ToolSearchToolParam(ToolSearchToolParam),
    WebSearchPreviewTool(WebSearchPreviewTool),
    ApplyPatchToolParam(ApplyPatchToolParam),
}
impl ::std::convert::From<FunctionTool> for ToolSearchOutputTool {
    fn from(value: FunctionTool) -> Self {
        Self::FunctionTool(value)
    }
}
impl ::std::convert::From<FileSearchTool> for ToolSearchOutputTool {
    fn from(value: FileSearchTool) -> Self {
        Self::FileSearchTool(value)
    }
}
impl ::std::convert::From<ComputerTool> for ToolSearchOutputTool {
    fn from(value: ComputerTool) -> Self {
        Self::ComputerTool(value)
    }
}
impl ::std::convert::From<ComputerUsePreviewTool> for ToolSearchOutputTool {
    fn from(value: ComputerUsePreviewTool) -> Self {
        Self::ComputerUsePreviewTool(value)
    }
}
impl ::std::convert::From<WebSearchTool> for ToolSearchOutputTool {
    fn from(value: WebSearchTool) -> Self {
        Self::WebSearchTool(value)
    }
}
impl ::std::convert::From<McpTool> for ToolSearchOutputTool {
    fn from(value: McpTool) -> Self {
        Self::McpTool(value)
    }
}
impl ::std::convert::From<CodeInterpreterTool> for ToolSearchOutputTool {
    fn from(value: CodeInterpreterTool) -> Self {
        Self::CodeInterpreterTool(value)
    }
}
impl ::std::convert::From<ProgrammaticToolCallingParam> for ToolSearchOutputTool {
    fn from(value: ProgrammaticToolCallingParam) -> Self {
        Self::ProgrammaticToolCallingParam(value)
    }
}
impl ::std::convert::From<ImageGenTool> for ToolSearchOutputTool {
    fn from(value: ImageGenTool) -> Self {
        Self::ImageGenTool(value)
    }
}
impl ::std::convert::From<LocalShellToolParam> for ToolSearchOutputTool {
    fn from(value: LocalShellToolParam) -> Self {
        Self::LocalShellToolParam(value)
    }
}
impl ::std::convert::From<FunctionShellToolParam> for ToolSearchOutputTool {
    fn from(value: FunctionShellToolParam) -> Self {
        Self::FunctionShellToolParam(value)
    }
}
impl ::std::convert::From<CustomToolParam> for ToolSearchOutputTool {
    fn from(value: CustomToolParam) -> Self {
        Self::CustomToolParam(value)
    }
}
impl ::std::convert::From<ToolSearchOutputNamespaceToolParam> for ToolSearchOutputTool {
    fn from(value: ToolSearchOutputNamespaceToolParam) -> Self {
        Self::ToolSearchOutputNamespaceToolParam(value)
    }
}
impl ::std::convert::From<ToolSearchToolParam> for ToolSearchOutputTool {
    fn from(value: ToolSearchToolParam) -> Self {
        Self::ToolSearchToolParam(value)
    }
}
impl ::std::convert::From<WebSearchPreviewTool> for ToolSearchOutputTool {
    fn from(value: WebSearchPreviewTool) -> Self {
        Self::WebSearchPreviewTool(value)
    }
}
impl ::std::convert::From<ApplyPatchToolParam> for ToolSearchOutputTool {
    fn from(value: ApplyPatchToolParam) -> Self {
        Self::ApplyPatchToolParam(value)
    }
}
#[doc = "The type of the item. Always `tool_search_output`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchOutputType {
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput,
}
impl ::std::fmt::Display for ToolSearchOutputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearchOutput => f.write_str("tool_search_output"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchOutputType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search_output" => Ok(Self::ToolSearchOutput),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchOutputType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchOutputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Hosted or BYOT tool search configuration for deferred tools."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ToolSearchToolParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Whether tool search is executed by the server or by the client."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<ToolSearchExecutionType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<EmptyModelParam>,
    #[doc = "The type of the tool. Always `tool_search`."]
    #[serde(rename = "type")]
    pub type_: ToolSearchToolParamType,
}
#[doc = "The type of the tool. Always `tool_search`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolSearchToolParamType {
    #[serde(rename = "tool_search")]
    ToolSearch,
}
impl ::std::fmt::Display for ToolSearchToolParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ToolSearch => f.write_str("tool_search"),
        }
    }
}
impl ::std::str::FromStr for ToolSearchToolParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool_search" => Ok(Self::ToolSearch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolSearchToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolSearchToolParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An array of tools the model may call while generating a response. You\ncan specify which tool to use by setting the `tool_choice` parameter.\n\nWe support the following categories of tools:\n- **Built-in tools**: Tools that are provided by OpenAI that extend the\n  model's capabilities, like [web search](https://developers.openai.com/api/docs/guides/tools-web-search)\n  or [file search](https://developers.openai.com/api/docs/guides/tools-file-search). Learn more about\n  [built-in tools](https://developers.openai.com/api/docs/guides/tools).\n- **MCP Tools**: Integrations with third-party systems via custom MCP servers\n  or predefined connectors such as Google Drive and SharePoint. Learn more about\n  [MCP Tools](https://developers.openai.com/api/docs/guides/tools-connectors-mcp).\n- **Function calls (custom tools)**: Functions that are defined by you,\n  enabling the model to call your own code with strongly typed arguments\n  and outputs. Learn more about\n  [function calling](https://developers.openai.com/api/docs/guides/function-calling). You can also use\n  custom tools to call your own code.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ToolsArray(pub ::std::vec::Vec<Tool>);
impl ::std::ops::Deref for ToolsArray {
    type Target = ::std::vec::Vec<Tool>;
    fn deref(&self) -> &::std::vec::Vec<Tool> {
        &self.0
    }
}
impl ::std::convert::From<ToolsArray> for ::std::vec::Vec<Tool> {
    fn from(value: ToolsArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Tool>> for ToolsArray {
    fn from(value: ::std::vec::Vec<Tool>) -> Self {
        Self(value)
    }
}
#[doc = "The top log probability of a token."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TopLogProb {
    pub bytes: ::std::vec::Vec<i64>,
    pub logprob: f64,
    pub token: ::std::string::String,
}
#[doc = "`TruncationEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TruncationEnum {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for TruncationEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for TruncationEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TruncationEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TruncationEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An action to type in text."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TypeParam {
    #[doc = "The text to type."]
    pub text: ::std::string::String,
    #[doc = "Specifies the event type. For a type action, this property is always set to `type`."]
    #[serde(rename = "type")]
    pub type_: TypeParamType,
}
#[doc = "Specifies the event type. For a type action, this property is always set to `type`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TypeParamType {
    #[serde(rename = "type")]
    Type,
}
impl ::std::fmt::Display for TypeParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Type => f.write_str("type"),
        }
    }
}
impl ::std::str::FromStr for TypeParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "type" => Ok(Self::Type),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TypeParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A citation for a web resource used to generate a model response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UrlCitationBody {
    #[doc = "The index of the last character of the URL citation in the message."]
    pub end_index: i64,
    #[doc = "The index of the first character of the URL citation in the message."]
    pub start_index: i64,
    #[doc = "The title of the web resource."]
    pub title: ::std::string::String,
    #[doc = "The type of the URL citation. Always `url_citation`."]
    #[serde(rename = "type")]
    pub type_: UrlCitationBodyType,
    #[doc = "The URL of the web resource."]
    pub url: ::std::string::String,
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
pub enum UrlCitationBodyType {
    #[serde(rename = "url_citation")]
    UrlCitation,
}
impl ::std::fmt::Display for UrlCitationBodyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UrlCitation => f.write_str("url_citation"),
        }
    }
}
impl ::std::str::FromStr for UrlCitationBodyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url_citation" => Ok(Self::UrlCitation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UrlCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UrlCitationBodyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VectorStoreFileAttributes`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct VectorStoreFileAttributes(pub ::std::option::Option<VectorStoreFileAttributesValue>);
impl ::std::ops::Deref for VectorStoreFileAttributes {
    type Target = ::std::option::Option<VectorStoreFileAttributesValue>;
    fn deref(&self) -> &::std::option::Option<VectorStoreFileAttributesValue> {
        &self.0
    }
}
impl ::std::convert::From<VectorStoreFileAttributes>
    for ::std::option::Option<VectorStoreFileAttributesValue>
{
    fn from(value: VectorStoreFileAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<VectorStoreFileAttributesValue>>
    for VectorStoreFileAttributes
{
    fn from(value: ::std::option::Option<VectorStoreFileAttributesValue>) -> Self {
        Self(value)
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
#[doc = "A wait action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WaitParam {
    #[doc = "Specifies the event type. For a wait action, this property is always set to `wait`."]
    #[serde(rename = "type")]
    pub type_: WaitParamType,
}
#[doc = "Specifies the event type. For a wait action, this property is always set to `wait`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WaitParamType {
    #[serde(rename = "wait")]
    Wait,
}
impl ::std::fmt::Display for WaitParamType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wait => f.write_str("wait"),
        }
    }
}
impl ::std::str::FromStr for WaitParamType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wait" => Ok(Self::Wait),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WaitParamType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WaitParamType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Action type \"find_in_page\": Searches for a pattern within a loaded page.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchActionFind {
    #[doc = "The pattern or text to search for within the page.\n"]
    pub pattern: ::std::string::String,
    #[doc = "The action type.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchActionFindType,
    #[doc = "The URL of the page searched for the pattern.\n"]
    pub url: ::std::string::String,
}
#[doc = "The action type.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchActionFindType {
    #[serde(rename = "find_in_page")]
    FindInPage,
}
impl ::std::fmt::Display for WebSearchActionFindType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FindInPage => f.write_str("find_in_page"),
        }
    }
}
impl ::std::str::FromStr for WebSearchActionFindType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "find_in_page" => Ok(Self::FindInPage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchActionFindType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchActionFindType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Action type \"open_page\" - Opens a specific URL from search results.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchActionOpenPage {
    #[doc = "The action type.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchActionOpenPageType,
    #[doc = "The URL opened by the model.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[doc = "The action type.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchActionOpenPageType {
    #[serde(rename = "open_page")]
    OpenPage,
}
impl ::std::fmt::Display for WebSearchActionOpenPageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OpenPage => f.write_str("open_page"),
        }
    }
}
impl ::std::str::FromStr for WebSearchActionOpenPageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "open_page" => Ok(Self::OpenPage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchActionOpenPageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchActionOpenPageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Action type \"search\" - Performs a web search query.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchActionSearch {
    #[doc = "The search queries.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub queries: ::std::vec::Vec<::std::string::String>,
    #[doc = "The search query.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub query: ::std::option::Option<::std::string::String>,
    #[doc = "The sources used in the search.\n"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sources: ::std::vec::Vec<WebSearchSource>,
    #[doc = "The action type.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchActionSearchType,
}
#[doc = "The action type.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchActionSearchType {
    #[serde(rename = "search")]
    Search,
}
impl ::std::fmt::Display for WebSearchActionSearchType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Search => f.write_str("search"),
        }
    }
}
impl ::std::str::FromStr for WebSearchActionSearchType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "search" => Ok(Self::Search),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchActionSearchType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchActionSearchType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebSearchApproximateLocation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct WebSearchApproximateLocation(
    pub ::std::option::Option<WebSearchApproximateLocationValue>,
);
impl ::std::ops::Deref for WebSearchApproximateLocation {
    type Target = ::std::option::Option<WebSearchApproximateLocationValue>;
    fn deref(&self) -> &::std::option::Option<WebSearchApproximateLocationValue> {
        &self.0
    }
}
impl ::std::convert::From<WebSearchApproximateLocation>
    for ::std::option::Option<WebSearchApproximateLocationValue>
{
    fn from(value: WebSearchApproximateLocation) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<WebSearchApproximateLocationValue>>
    for WebSearchApproximateLocation
{
    fn from(value: ::std::option::Option<WebSearchApproximateLocationValue>) -> Self {
        Self(value)
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
#[doc = "`WebSearchCallStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
impl ::std::fmt::Display for WebSearchCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Searching => f.write_str("searching"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
            Self::Incomplete => f.write_str("incomplete"),
        }
    }
}
impl ::std::str::FromStr for WebSearchCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "searching" => Ok(Self::Searching),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](https://developers.openai.com/api/docs/guides/tools-web-search)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchPreviewTool {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub search_content_types: ::std::vec::Vec<SearchContentType>,
    #[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub search_context_size: ::std::option::Option<SearchContextSize>,
    #[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
    #[serde(rename = "type")]
    pub type_: WebSearchPreviewToolType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_location: ::std::option::Option<ApproximateLocation>,
}
#[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchPreviewToolType {
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
}
impl ::std::fmt::Display for WebSearchPreviewToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearchPreview => f.write_str("web_search_preview"),
            Self::WebSearchPreview20250311 => f.write_str("web_search_preview_2025_03_11"),
        }
    }
}
impl ::std::str::FromStr for WebSearchPreviewToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search_preview" => Ok(Self::WebSearchPreview),
            "web_search_preview_2025_03_11" => Ok(Self::WebSearchPreview20250311),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchPreviewToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchPreviewToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A source used in the search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchSource {
    #[doc = "The type of source. Always `url`.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchSourceType,
    #[doc = "The URL of the source.\n"]
    pub url: ::std::string::String,
}
#[doc = "The type of source. Always `url`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchSourceType {
    #[serde(rename = "url")]
    Url,
}
impl ::std::fmt::Display for WebSearchSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Url => f.write_str("url"),
        }
    }
}
impl ::std::str::FromStr for WebSearchSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url" => Ok(Self::Url),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Search the Internet for sources related to the prompt. Learn more about the\n[web search tool](https://developers.openai.com/api/docs/guides/tools-web-search).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchTool {
    #[doc = "Allow live internet access for web search. Defaults to true when omitted. When false, the web search tool runs in offline/cache-only mode and will not fetch new external content."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub external_web_access: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filters: ::std::option::Option<WebSearchToolFilters>,
    #[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub search_context_size: ::std::option::Option<WebSearchToolSearchContextSize>,
    #[doc = "The type of the web search tool. One of `web_search` or `web_search_2025_08_26`."]
    #[serde(rename = "type")]
    pub type_: WebSearchToolType,
    #[doc = "The approximate location of the user. If omitted or null, defaults to the\nUnited States. To avoid this fallback, pass `{\"type\": \"approximate\"}` without\nlocation fields. To localize results, provide the relevant location fields.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_location: ::std::option::Option<WebSearchApproximateLocation>,
}
#[doc = "The results of a web search tool call. See the\n[web search guide](https://developers.openai.com/api/docs/guides/tools-web-search) for more information.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebSearchToolCall {
    #[doc = "An object describing the specific action taken in this web search call.\nIncludes details on how the model used the web (search, open_page, find_in_page).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<WebSearchToolCallAction>,
    #[doc = "The unique ID of the web search tool call.\n"]
    pub id: ::std::string::String,
    #[doc = "The status of the web search tool call.\n"]
    pub status: WebSearchCallStatus,
    #[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
    #[serde(rename = "type")]
    pub type_: WebSearchToolCallType,
}
#[doc = "An object describing the specific action taken in this web search call.\nIncludes details on how the model used the web (search, open_page, find_in_page).\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum WebSearchToolCallAction {
    Search(WebSearchActionSearch),
    OpenPage(WebSearchActionOpenPage),
    Find(WebSearchActionFind),
}
impl ::std::convert::From<WebSearchActionSearch> for WebSearchToolCallAction {
    fn from(value: WebSearchActionSearch) -> Self {
        Self::Search(value)
    }
}
impl ::std::convert::From<WebSearchActionOpenPage> for WebSearchToolCallAction {
    fn from(value: WebSearchActionOpenPage) -> Self {
        Self::OpenPage(value)
    }
}
impl ::std::convert::From<WebSearchActionFind> for WebSearchToolCallAction {
    fn from(value: WebSearchActionFind) -> Self {
        Self::Find(value)
    }
}
#[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchToolCallType {
    #[serde(rename = "web_search_call")]
    WebSearchCall,
}
impl ::std::fmt::Display for WebSearchToolCallType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearchCall => f.write_str("web_search_call"),
        }
    }
}
impl ::std::str::FromStr for WebSearchToolCallType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search_call" => Ok(Self::WebSearchCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchToolCallType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Filters for the search.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct WebSearchToolFilters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchToolSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for WebSearchToolSearchContextSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for WebSearchToolSearchContextSize {
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
impl ::std::convert::TryFrom<&str> for WebSearchToolSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchToolSearchContextSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the web search tool. One of `web_search` or `web_search_2025_08_26`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebSearchToolType {
    #[serde(rename = "web_search")]
    WebSearch,
    #[serde(rename = "web_search_2025_08_26")]
    WebSearch20250826,
}
impl ::std::fmt::Display for WebSearchToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebSearch => f.write_str("web_search"),
            Self::WebSearch20250826 => f.write_str("web_search_2025_08_26"),
        }
    }
}
impl ::std::str::FromStr for WebSearchToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "web_search" => Ok(Self::WebSearch),
            "web_search_2025_08_26" => Ok(Self::WebSearch20250826),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebSearchToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebSearchToolType {
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
