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
#[doc = "An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CoordParam {
    #[doc = "The x-coordinate."]
    pub x: i64,
    #[doc = "The y-coordinate."]
    pub y: i64,
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
    pub model: ::std::option::Option<::std::string::String>,
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
    pub size: ::std::option::Option<::std::string::String>,
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
    pub size: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "AcceptLiveSessionRequest")]
    pub accept_live_session_request: LiveCallAcceptRequest,
    #[serde(rename = "CreateLiveRequest")]
    pub create_live_request: LiveSessionCreateRequest,
    #[serde(rename = "CreateLiveResponse")]
    pub create_live_response: LiveSessionCreateResponse,
    #[serde(rename = "ForkLiveSessionRequest")]
    pub fork_live_session_request: LiveForkRequest,
    #[serde(rename = "ForkLiveSessionResponse")]
    pub fork_live_session_response: LiveCreateResponse,
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
    #[serde(rename = "ReferLiveSessionRequest")]
    pub refer_live_session_request: LiveCallReferRequest,
    #[serde(rename = "RejectLiveSessionRequest")]
    pub reject_live_session_request: LiveCallRejectRequest,
}
#[doc = "`LiveApplyPatchToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveApplyPatchToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveApplyPatchToolInputParamType,
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
#[doc = "Startup-only capabilities for an untrusted frontend attached to a unified WebRTC session. Trusted sideband connections are unaffected."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveClientConfigParam {
    #[doc = "Client and server event permissions for the WebRTC frontend data channel."]
    pub data_channel: LiveDataChannelConfigParam,
}
#[doc = "Delegate tasks to your application. The Live session emits delegation events that your backend handles."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveClientDelegationParam {
    #[doc = "The delegation owner. Always `client` for tasks handled by your application."]
    #[serde(rename = "type")]
    pub type_: LiveClientDelegationParamType,
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
#[doc = "The created Live session identifier and WebRTC answer. Apply transport.sdp as the peer's remote answer and wait for session.started on the data channel before sending commands."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCreateResponse {
    pub session: LiveCreateResponseSession,
    #[doc = "WebRTC transport with the SDP answer."]
    pub transport: LiveWebRtcTransport,
}
#[doc = "The newly created Live session. Use its ID for session controls and sideband connections."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCreateResponseSession {
    #[doc = "Opaque session identifier. Preserve the returned value unchanged, including its prefix."]
    pub id: ::std::string::String,
}
#[doc = "`LiveCustomToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveCustomToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveCustomToolInputParamType,
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
#[doc = "Text generation options for the Live session’s Responses backend."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct LiveDelegationTextInputParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verbosity: ::std::option::Option<LiveTextVerbosity>,
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
#[doc = "`LiveLocalEnvironmentParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveLocalEnvironmentParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub skills: ::std::option::Option<::std::vec::Vec<LiveLocalSkillParam>>,
    #[doc = "Use a local computer environment."]
    #[serde(rename = "type")]
    pub type_: LiveLocalEnvironmentParamType,
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
#[doc = "`LiveNamespaceToolInputParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LiveNamespaceToolInputParam {
    #[serde(rename = "type")]
    pub type_: LiveNamespaceToolInputParamType,
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
    #[doc = "An input item to append to the Responses backend conversation, such as a user message or a function tool result."]
    pub item: InputItem,
    #[doc = "The Live client event type. Always `response.item.create`."]
    #[serde(rename = "type")]
    pub type_: LiveResponseItemCreateParamType,
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
#[doc = "The newly created Live session. Use its ID for session controls and sideband connections."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Session {
    #[doc = "Opaque session identifier. Preserve the returned value unchanged, including its prefix."]
    pub id: ::std::string::String,
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
#[doc = "The top log probability of a token."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TopLogProb {
    pub bytes: ::std::vec::Vec<i64>,
    pub logprob: f64,
    pub token: ::std::string::String,
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
