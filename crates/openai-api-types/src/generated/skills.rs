pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, OrderEnum, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Uploads a skill either as a directory (multipart `files[]`) or as a single zip file."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateSkillBody {
    pub files: CreateSkillBodyFiles,
}
impl CreateSkillBody {
    pub fn builder() -> builder::CreateSkillBody {
        Default::default()
    }
}
#[doc = "`CreateSkillBodyFiles`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateSkillBodyFiles {
    Array(::std::vec::Vec<::std::string::String>),
    Binary(::std::string::String),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CreateSkillBodyFiles {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "Uploads a new immutable version of a skill."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateSkillVersionBody {
    pub files: CreateSkillVersionBodyFiles,
}
impl CreateSkillVersionBody {
    pub fn builder() -> builder::CreateSkillVersionBody {
        Default::default()
    }
}
#[doc = "`CreateSkillVersionBodyFiles`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateSkillVersionBodyFiles {
    Array(::std::vec::Vec<::std::string::String>),
    Binary(::std::string::String),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CreateSkillVersionBodyFiles {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`CreateSkillVersionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateSkillVersionParams {
    pub skill_id: ::std::string::String,
}
impl CreateSkillVersionParams {
    pub fn builder() -> builder::CreateSkillVersionParams {
        Default::default()
    }
}
#[doc = "`DeleteSkillParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteSkillParams {
    pub skill_id: ::std::string::String,
}
impl DeleteSkillParams {
    pub fn builder() -> builder::DeleteSkillParams {
        Default::default()
    }
}
#[doc = "`DeleteSkillVersionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteSkillVersionParams {
    pub skill_id: ::std::string::String,
    #[doc = "The skill version number."]
    pub version: ::std::string::String,
}
impl DeleteSkillVersionParams {
    pub fn builder() -> builder::DeleteSkillVersionParams {
        Default::default()
    }
}
#[doc = "`DeletedSkillResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedSkillResource {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeletedSkillResourceObject,
}
impl DeletedSkillResource {
    pub fn builder() -> builder::DeletedSkillResource {
        Default::default()
    }
}
#[doc = "`DeletedSkillResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedSkillResourceObject {
    #[serde(rename = "skill.deleted")]
    SkillDeleted,
}
impl ::std::fmt::Display for DeletedSkillResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SkillDeleted => f.write_str("skill.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedSkillResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill.deleted" => Ok(Self::SkillDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedSkillResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedSkillResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeletedSkillVersionResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedSkillVersionResource {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeletedSkillVersionResourceObject,
    #[doc = "The deleted skill version."]
    pub version: ::std::string::String,
}
impl DeletedSkillVersionResource {
    pub fn builder() -> builder::DeletedSkillVersionResource {
        Default::default()
    }
}
#[doc = "`DeletedSkillVersionResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedSkillVersionResourceObject {
    #[serde(rename = "skill.version.deleted")]
    SkillVersionDeleted,
}
impl ::std::fmt::Display for DeletedSkillVersionResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SkillVersionDeleted => f.write_str("skill.version.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedSkillVersionResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill.version.deleted" => Ok(Self::SkillVersionDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedSkillVersionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedSkillVersionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetSkillContentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetSkillContentParams {
    pub skill_id: ::std::string::String,
}
impl GetSkillContentParams {
    pub fn builder() -> builder::GetSkillContentParams {
        Default::default()
    }
}
#[doc = "`GetSkillParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetSkillParams {
    pub skill_id: ::std::string::String,
}
impl GetSkillParams {
    pub fn builder() -> builder::GetSkillParams {
        Default::default()
    }
}
#[doc = "`GetSkillVersionContentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetSkillVersionContentParams {
    pub skill_id: ::std::string::String,
    #[doc = "The skill version number."]
    pub version: ::std::string::String,
}
impl GetSkillVersionContentParams {
    pub fn builder() -> builder::GetSkillVersionContentParams {
        Default::default()
    }
}
#[doc = "`GetSkillVersionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetSkillVersionParams {
    pub skill_id: ::std::string::String,
    #[doc = "The version number to retrieve."]
    pub version: ::std::string::String,
}
impl GetSkillVersionParams {
    pub fn builder() -> builder::GetSkillVersionParams {
        Default::default()
    }
}
#[doc = "`ListSkillVersionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListSkillVersionsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::OrderEnum>,
    pub skill_id: ::std::string::String,
}
impl ListSkillVersionsParams {
    pub fn builder() -> builder::ListSkillVersionsParams {
        Default::default()
    }
}
#[doc = "`ListSkillsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListSkillsParams {
    #[doc = "Identifier for the last item from the previous pagination request"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::OrderEnum>,
}
impl ListSkillsParams {
    pub fn builder() -> builder::ListSkillsParams {
        Default::default()
    }
}
#[doc = "Updates the default version pointer for a skill."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SetDefaultSkillVersionBody {
    #[doc = "The skill version number to set as default."]
    pub default_version: ::std::string::String,
}
impl SetDefaultSkillVersionBody {
    pub fn builder() -> builder::SetDefaultSkillVersionBody {
        Default::default()
    }
}
#[doc = "`SkillListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillListResource {
    #[doc = "A list of items"]
    pub data: ::std::vec::Vec<SkillResource>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of object returned, must be `list`."]
    pub object: SkillListResourceObject,
}
impl SkillListResource {
    pub fn builder() -> builder::SkillListResource {
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
pub enum SkillListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for SkillListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for SkillListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SkillResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillResource {
    #[doc = "Unix timestamp (seconds) for when the skill was created."]
    pub created_at: i64,
    #[doc = "Default version for the skill."]
    pub default_version: ::std::string::String,
    #[doc = "Description of the skill."]
    pub description: ::std::string::String,
    #[doc = "Unique identifier for the skill."]
    pub id: ::std::string::String,
    #[doc = "Latest version for the skill."]
    pub latest_version: ::std::string::String,
    #[doc = "Name of the skill."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is `skill`."]
    pub object: SkillResourceObject,
}
impl SkillResource {
    pub fn builder() -> builder::SkillResource {
        Default::default()
    }
}
#[doc = "The object type, which is `skill`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SkillResourceObject {
    #[serde(rename = "skill")]
    Skill,
}
impl ::std::fmt::Display for SkillResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skill => f.write_str("skill"),
        }
    }
}
impl ::std::str::FromStr for SkillResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill" => Ok(Self::Skill),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SkillVersionListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillVersionListResource {
    #[doc = "A list of items"]
    pub data: ::std::vec::Vec<SkillVersionResource>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of object returned, must be `list`."]
    pub object: SkillVersionListResourceObject,
}
impl SkillVersionListResource {
    pub fn builder() -> builder::SkillVersionListResource {
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
pub enum SkillVersionListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for SkillVersionListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for SkillVersionListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillVersionListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillVersionListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SkillVersionResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillVersionResource {
    #[doc = "Unix timestamp (seconds) for when the version was created."]
    pub created_at: i64,
    #[doc = "Description of the skill version."]
    pub description: ::std::string::String,
    #[doc = "Unique identifier for the skill version."]
    pub id: ::std::string::String,
    #[doc = "Name of the skill version."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is `skill.version`."]
    pub object: SkillVersionResourceObject,
    #[doc = "Identifier of the skill for this version."]
    pub skill_id: ::std::string::String,
    #[doc = "Version number for this skill."]
    pub version: ::std::string::String,
}
impl SkillVersionResource {
    pub fn builder() -> builder::SkillVersionResource {
        Default::default()
    }
}
#[doc = "The object type, which is `skill.version`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SkillVersionResourceObject {
    #[serde(rename = "skill.version")]
    SkillVersion,
}
impl ::std::fmt::Display for SkillVersionResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SkillVersion => f.write_str("skill.version"),
        }
    }
}
impl ::std::str::FromStr for SkillVersionResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill.version" => Ok(Self::SkillVersion),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SkillVersionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SkillVersionResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SkillsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SkillsApiSchema {
    #[serde(rename = "CreateSkillError400")]
    pub create_skill_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillError429")]
    pub create_skill_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillError500")]
    pub create_skill_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillMultipartFormDataRequest")]
    pub create_skill_multipart_form_data_request: CreateSkillBody,
    #[serde(rename = "CreateSkillRequest")]
    pub create_skill_request: CreateSkillBody,
    #[serde(rename = "CreateSkillResponse")]
    pub create_skill_response: SkillResource,
    #[serde(rename = "CreateSkillVersionError400")]
    pub create_skill_version_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillVersionError404")]
    pub create_skill_version_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillVersionError429")]
    pub create_skill_version_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillVersionError500")]
    pub create_skill_version_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateSkillVersionMultipartFormDataRequest")]
    pub create_skill_version_multipart_form_data_request: CreateSkillVersionBody,
    #[serde(rename = "CreateSkillVersionParams")]
    pub create_skill_version_params: CreateSkillVersionParams,
    #[serde(rename = "CreateSkillVersionRequest")]
    pub create_skill_version_request: CreateSkillVersionBody,
    #[serde(rename = "CreateSkillVersionResponse")]
    pub create_skill_version_response: SkillVersionResource,
    #[serde(rename = "DeleteSkillError404")]
    pub delete_skill_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteSkillError429")]
    pub delete_skill_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteSkillParams")]
    pub delete_skill_params: DeleteSkillParams,
    #[serde(rename = "DeleteSkillResponse")]
    pub delete_skill_response: DeletedSkillResource,
    #[serde(rename = "DeleteSkillVersionError400")]
    pub delete_skill_version_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteSkillVersionError404")]
    pub delete_skill_version_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteSkillVersionError429")]
    pub delete_skill_version_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteSkillVersionParams")]
    pub delete_skill_version_params: DeleteSkillVersionParams,
    #[serde(rename = "DeleteSkillVersionResponse")]
    pub delete_skill_version_response: DeletedSkillVersionResource,
    #[serde(rename = "GetSkillContentError404")]
    pub get_skill_content_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillContentError429")]
    pub get_skill_content_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillContentParams")]
    pub get_skill_content_params: GetSkillContentParams,
    #[serde(rename = "GetSkillContentResponse")]
    pub get_skill_content_response: ::std::string::String,
    #[serde(rename = "GetSkillError404")]
    pub get_skill_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillError429")]
    pub get_skill_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillParams")]
    pub get_skill_params: GetSkillParams,
    #[serde(rename = "GetSkillResponse")]
    pub get_skill_response: SkillResource,
    #[serde(rename = "GetSkillVersionContentError400")]
    pub get_skill_version_content_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionContentError404")]
    pub get_skill_version_content_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionContentError429")]
    pub get_skill_version_content_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionContentParams")]
    pub get_skill_version_content_params: GetSkillVersionContentParams,
    #[serde(rename = "GetSkillVersionContentResponse")]
    pub get_skill_version_content_response: ::std::string::String,
    #[serde(rename = "GetSkillVersionError400")]
    pub get_skill_version_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionError404")]
    pub get_skill_version_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionError429")]
    pub get_skill_version_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetSkillVersionParams")]
    pub get_skill_version_params: GetSkillVersionParams,
    #[serde(rename = "GetSkillVersionResponse")]
    pub get_skill_version_response: SkillVersionResource,
    #[serde(rename = "ListSkillVersionsError400")]
    pub list_skill_versions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListSkillVersionsError404")]
    pub list_skill_versions_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListSkillVersionsError429")]
    pub list_skill_versions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListSkillVersionsParams")]
    pub list_skill_versions_params: ListSkillVersionsParams,
    #[serde(rename = "ListSkillVersionsResponse")]
    pub list_skill_versions_response: SkillVersionListResource,
    #[serde(rename = "ListSkillsError404")]
    pub list_skills_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListSkillsError429")]
    pub list_skills_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListSkillsParams")]
    pub list_skills_params: ListSkillsParams,
    #[serde(rename = "ListSkillsResponse")]
    pub list_skills_response: SkillListResource,
    #[serde(rename = "UpdateSkillDefaultVersionError400")]
    pub update_skill_default_version_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateSkillDefaultVersionError404")]
    pub update_skill_default_version_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateSkillDefaultVersionError429")]
    pub update_skill_default_version_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateSkillDefaultVersionParams")]
    pub update_skill_default_version_params: UpdateSkillDefaultVersionParams,
    #[serde(rename = "UpdateSkillDefaultVersionRequest")]
    pub update_skill_default_version_request: SetDefaultSkillVersionBody,
    #[serde(rename = "UpdateSkillDefaultVersionResponse")]
    pub update_skill_default_version_response: SkillResource,
}
impl SkillsApiSchema {
    pub fn builder() -> builder::SkillsApiSchema {
        Default::default()
    }
}
#[doc = "`UpdateSkillDefaultVersionParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateSkillDefaultVersionParams {
    pub skill_id: ::std::string::String,
}
impl UpdateSkillDefaultVersionParams {
    pub fn builder() -> builder::UpdateSkillDefaultVersionParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateSkillBody {
        files: ::std::result::Result<super::CreateSkillBodyFiles, ::std::string::String>,
    }
    impl ::std::default::Default for CreateSkillBody {
        fn default() -> Self {
            Self {
                files: Err("no value supplied for files".to_string()),
            }
        }
    }
    impl CreateSkillBody {
        pub fn files<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillBodyFiles>,
            T::Error: ::std::fmt::Display,
        {
            self.files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for files: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateSkillBody> for super::CreateSkillBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateSkillBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                files: value.files?,
            })
        }
    }
    impl ::std::convert::From<super::CreateSkillBody> for CreateSkillBody {
        fn from(value: super::CreateSkillBody) -> Self {
            Self {
                files: Ok(value.files),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSkillVersionBody {
        files: ::std::result::Result<super::CreateSkillVersionBodyFiles, ::std::string::String>,
    }
    impl ::std::default::Default for CreateSkillVersionBody {
        fn default() -> Self {
            Self {
                files: Err("no value supplied for files".to_string()),
            }
        }
    }
    impl CreateSkillVersionBody {
        pub fn files<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillVersionBodyFiles>,
            T::Error: ::std::fmt::Display,
        {
            self.files = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for files: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateSkillVersionBody> for super::CreateSkillVersionBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateSkillVersionBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                files: value.files?,
            })
        }
    }
    impl ::std::convert::From<super::CreateSkillVersionBody> for CreateSkillVersionBody {
        fn from(value: super::CreateSkillVersionBody) -> Self {
            Self {
                files: Ok(value.files),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSkillVersionParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateSkillVersionParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl CreateSkillVersionParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateSkillVersionParams> for super::CreateSkillVersionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateSkillVersionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateSkillVersionParams> for CreateSkillVersionParams {
        fn from(value: super::CreateSkillVersionParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteSkillParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteSkillParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl DeleteSkillParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteSkillParams> for super::DeleteSkillParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteSkillParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteSkillParams> for DeleteSkillParams {
        fn from(value: super::DeleteSkillParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteSkillVersionParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteSkillVersionParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl DeleteSkillVersionParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteSkillVersionParams> for super::DeleteSkillVersionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteSkillVersionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteSkillVersionParams> for DeleteSkillVersionParams {
        fn from(value: super::DeleteSkillVersionParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedSkillResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeletedSkillResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedSkillResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedSkillResource {
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
            T: ::std::convert::TryInto<super::DeletedSkillResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedSkillResource> for super::DeletedSkillResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedSkillResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedSkillResource> for DeletedSkillResource {
        fn from(value: super::DeletedSkillResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedSkillVersionResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::DeletedSkillVersionResourceObject, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedSkillVersionResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl DeletedSkillVersionResource {
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
            T: ::std::convert::TryInto<super::DeletedSkillVersionResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedSkillVersionResource> for super::DeletedSkillVersionResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedSkillVersionResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedSkillVersionResource> for DeletedSkillVersionResource {
        fn from(value: super::DeletedSkillVersionResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetSkillContentParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetSkillContentParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl GetSkillContentParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetSkillContentParams> for super::GetSkillContentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetSkillContentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetSkillContentParams> for GetSkillContentParams {
        fn from(value: super::GetSkillContentParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetSkillParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetSkillParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl GetSkillParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetSkillParams> for super::GetSkillParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetSkillParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetSkillParams> for GetSkillParams {
        fn from(value: super::GetSkillParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetSkillVersionContentParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetSkillVersionContentParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl GetSkillVersionContentParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetSkillVersionContentParams> for super::GetSkillVersionContentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetSkillVersionContentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::GetSkillVersionContentParams> for GetSkillVersionContentParams {
        fn from(value: super::GetSkillVersionContentParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetSkillVersionParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetSkillVersionParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl GetSkillVersionParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetSkillVersionParams> for super::GetSkillVersionParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetSkillVersionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::GetSkillVersionParams> for GetSkillVersionParams {
        fn from(value: super::GetSkillVersionParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListSkillVersionsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::OrderEnum>,
            ::std::string::String,
        >,
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListSkillVersionsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl ListSkillVersionsParams {
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
                ::std::option::Option<::openai_api_types::common::OrderEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListSkillVersionsParams> for super::ListSkillVersionsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListSkillVersionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListSkillVersionsParams> for ListSkillVersionsParams {
        fn from(value: super::ListSkillVersionsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                skill_id: Ok(value.skill_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListSkillsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::OrderEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListSkillsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListSkillsParams {
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
                ::std::option::Option<::openai_api_types::common::OrderEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListSkillsParams> for super::ListSkillsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListSkillsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListSkillsParams> for ListSkillsParams {
        fn from(value: super::ListSkillsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetDefaultSkillVersionBody {
        default_version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SetDefaultSkillVersionBody {
        fn default() -> Self {
            Self {
                default_version: Err("no value supplied for default_version".to_string()),
            }
        }
    }
    impl SetDefaultSkillVersionBody {
        pub fn default_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.default_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetDefaultSkillVersionBody> for super::SetDefaultSkillVersionBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetDefaultSkillVersionBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_version: value.default_version?,
            })
        }
    }
    impl ::std::convert::From<super::SetDefaultSkillVersionBody> for SetDefaultSkillVersionBody {
        fn from(value: super::SetDefaultSkillVersionBody) -> Self {
            Self {
                default_version: Ok(value.default_version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillListResource {
        data: ::std::result::Result<::std::vec::Vec<super::SkillResource>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::SkillListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for SkillListResource {
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
    impl SkillListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SkillResource>>,
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
            T: ::std::convert::TryInto<super::SkillListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SkillListResource> for super::SkillListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillListResource,
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
    impl ::std::convert::From<super::SkillListResource> for SkillListResource {
        fn from(value: super::SkillListResource) -> Self {
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
    pub struct SkillResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        default_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        latest_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::SkillResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for SkillResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                default_version: Err("no value supplied for default_version".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                latest_version: Err("no value supplied for latest_version".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl SkillResource {
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
        pub fn default_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.default_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_version: {e}"));
            self
        }
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
        pub fn latest_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.latest_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for latest_version: {e}"));
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
            T: ::std::convert::TryInto<super::SkillResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SkillResource> for super::SkillResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                default_version: value.default_version?,
                description: value.description?,
                id: value.id?,
                latest_version: value.latest_version?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::SkillResource> for SkillResource {
        fn from(value: super::SkillResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                default_version: Ok(value.default_version),
                description: Ok(value.description),
                id: Ok(value.id),
                latest_version: Ok(value.latest_version),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillVersionListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::SkillVersionResource>,
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
        object: ::std::result::Result<super::SkillVersionListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for SkillVersionListResource {
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
    impl SkillVersionListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SkillVersionResource>>,
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
            T: ::std::convert::TryInto<super::SkillVersionListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SkillVersionListResource> for super::SkillVersionListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillVersionListResource,
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
    impl ::std::convert::From<super::SkillVersionListResource> for SkillVersionListResource {
        fn from(value: super::SkillVersionListResource) -> Self {
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
    pub struct SkillVersionResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::SkillVersionResourceObject, ::std::string::String>,
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SkillVersionResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                skill_id: Err("no value supplied for skill_id".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl SkillVersionResource {
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
            T: ::std::convert::TryInto<super::SkillVersionResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SkillVersionResource> for super::SkillVersionResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillVersionResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                description: value.description?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                skill_id: value.skill_id?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::SkillVersionResource> for SkillVersionResource {
        fn from(value: super::SkillVersionResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                skill_id: Ok(value.skill_id),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillsApiSchema {
        create_skill_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_multipart_form_data_request:
            ::std::result::Result<super::CreateSkillBody, ::std::string::String>,
        create_skill_request: ::std::result::Result<super::CreateSkillBody, ::std::string::String>,
        create_skill_response: ::std::result::Result<super::SkillResource, ::std::string::String>,
        create_skill_version_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_version_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_version_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_version_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_skill_version_multipart_form_data_request:
            ::std::result::Result<super::CreateSkillVersionBody, ::std::string::String>,
        create_skill_version_params:
            ::std::result::Result<super::CreateSkillVersionParams, ::std::string::String>,
        create_skill_version_request:
            ::std::result::Result<super::CreateSkillVersionBody, ::std::string::String>,
        create_skill_version_response:
            ::std::result::Result<super::SkillVersionResource, ::std::string::String>,
        delete_skill_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_skill_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_skill_params: ::std::result::Result<super::DeleteSkillParams, ::std::string::String>,
        delete_skill_response:
            ::std::result::Result<super::DeletedSkillResource, ::std::string::String>,
        delete_skill_version_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_skill_version_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_skill_version_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_skill_version_params:
            ::std::result::Result<super::DeleteSkillVersionParams, ::std::string::String>,
        delete_skill_version_response:
            ::std::result::Result<super::DeletedSkillVersionResource, ::std::string::String>,
        get_skill_content_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_content_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_content_params:
            ::std::result::Result<super::GetSkillContentParams, ::std::string::String>,
        get_skill_content_response:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        get_skill_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_params: ::std::result::Result<super::GetSkillParams, ::std::string::String>,
        get_skill_response: ::std::result::Result<super::SkillResource, ::std::string::String>,
        get_skill_version_content_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_content_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_content_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_content_params:
            ::std::result::Result<super::GetSkillVersionContentParams, ::std::string::String>,
        get_skill_version_content_response:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        get_skill_version_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_skill_version_params:
            ::std::result::Result<super::GetSkillVersionParams, ::std::string::String>,
        get_skill_version_response:
            ::std::result::Result<super::SkillVersionResource, ::std::string::String>,
        list_skill_versions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_skill_versions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_skill_versions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_skill_versions_params:
            ::std::result::Result<super::ListSkillVersionsParams, ::std::string::String>,
        list_skill_versions_response:
            ::std::result::Result<super::SkillVersionListResource, ::std::string::String>,
        list_skills_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_skills_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_skills_params: ::std::result::Result<super::ListSkillsParams, ::std::string::String>,
        list_skills_response:
            ::std::result::Result<super::SkillListResource, ::std::string::String>,
        update_skill_default_version_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_skill_default_version_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_skill_default_version_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_skill_default_version_params:
            ::std::result::Result<super::UpdateSkillDefaultVersionParams, ::std::string::String>,
        update_skill_default_version_request:
            ::std::result::Result<super::SetDefaultSkillVersionBody, ::std::string::String>,
        update_skill_default_version_response:
            ::std::result::Result<super::SkillResource, ::std::string::String>,
    }
    impl ::std::default::Default for SkillsApiSchema {
        fn default() -> Self {
            Self {
                create_skill_error400: Err(
                    "no value supplied for create_skill_error400".to_string()
                ),
                create_skill_error429: Err(
                    "no value supplied for create_skill_error429".to_string()
                ),
                create_skill_error500: Err(
                    "no value supplied for create_skill_error500".to_string()
                ),
                create_skill_multipart_form_data_request: Err(
                    "no value supplied for create_skill_multipart_form_data_request".to_string(),
                ),
                create_skill_request: Err("no value supplied for create_skill_request".to_string()),
                create_skill_response: Err(
                    "no value supplied for create_skill_response".to_string()
                ),
                create_skill_version_error400: Err(
                    "no value supplied for create_skill_version_error400".to_string(),
                ),
                create_skill_version_error404: Err(
                    "no value supplied for create_skill_version_error404".to_string(),
                ),
                create_skill_version_error429: Err(
                    "no value supplied for create_skill_version_error429".to_string(),
                ),
                create_skill_version_error500: Err(
                    "no value supplied for create_skill_version_error500".to_string(),
                ),
                create_skill_version_multipart_form_data_request: Err(
                    "no value supplied for create_skill_version_multipart_form_data_request"
                        .to_string(),
                ),
                create_skill_version_params: Err(
                    "no value supplied for create_skill_version_params".to_string(),
                ),
                create_skill_version_request: Err(
                    "no value supplied for create_skill_version_request".to_string(),
                ),
                create_skill_version_response: Err(
                    "no value supplied for create_skill_version_response".to_string(),
                ),
                delete_skill_error404: Err(
                    "no value supplied for delete_skill_error404".to_string()
                ),
                delete_skill_error429: Err(
                    "no value supplied for delete_skill_error429".to_string()
                ),
                delete_skill_params: Err("no value supplied for delete_skill_params".to_string()),
                delete_skill_response: Err(
                    "no value supplied for delete_skill_response".to_string()
                ),
                delete_skill_version_error400: Err(
                    "no value supplied for delete_skill_version_error400".to_string(),
                ),
                delete_skill_version_error404: Err(
                    "no value supplied for delete_skill_version_error404".to_string(),
                ),
                delete_skill_version_error429: Err(
                    "no value supplied for delete_skill_version_error429".to_string(),
                ),
                delete_skill_version_params: Err(
                    "no value supplied for delete_skill_version_params".to_string(),
                ),
                delete_skill_version_response: Err(
                    "no value supplied for delete_skill_version_response".to_string(),
                ),
                get_skill_content_error404: Err(
                    "no value supplied for get_skill_content_error404".to_string()
                ),
                get_skill_content_error429: Err(
                    "no value supplied for get_skill_content_error429".to_string()
                ),
                get_skill_content_params: Err(
                    "no value supplied for get_skill_content_params".to_string()
                ),
                get_skill_content_response: Err(
                    "no value supplied for get_skill_content_response".to_string()
                ),
                get_skill_error404: Err("no value supplied for get_skill_error404".to_string()),
                get_skill_error429: Err("no value supplied for get_skill_error429".to_string()),
                get_skill_params: Err("no value supplied for get_skill_params".to_string()),
                get_skill_response: Err("no value supplied for get_skill_response".to_string()),
                get_skill_version_content_error400: Err(
                    "no value supplied for get_skill_version_content_error400".to_string(),
                ),
                get_skill_version_content_error404: Err(
                    "no value supplied for get_skill_version_content_error404".to_string(),
                ),
                get_skill_version_content_error429: Err(
                    "no value supplied for get_skill_version_content_error429".to_string(),
                ),
                get_skill_version_content_params: Err(
                    "no value supplied for get_skill_version_content_params".to_string(),
                ),
                get_skill_version_content_response: Err(
                    "no value supplied for get_skill_version_content_response".to_string(),
                ),
                get_skill_version_error400: Err(
                    "no value supplied for get_skill_version_error400".to_string()
                ),
                get_skill_version_error404: Err(
                    "no value supplied for get_skill_version_error404".to_string()
                ),
                get_skill_version_error429: Err(
                    "no value supplied for get_skill_version_error429".to_string()
                ),
                get_skill_version_params: Err(
                    "no value supplied for get_skill_version_params".to_string()
                ),
                get_skill_version_response: Err(
                    "no value supplied for get_skill_version_response".to_string()
                ),
                list_skill_versions_error400: Err(
                    "no value supplied for list_skill_versions_error400".to_string(),
                ),
                list_skill_versions_error404: Err(
                    "no value supplied for list_skill_versions_error404".to_string(),
                ),
                list_skill_versions_error429: Err(
                    "no value supplied for list_skill_versions_error429".to_string(),
                ),
                list_skill_versions_params: Err(
                    "no value supplied for list_skill_versions_params".to_string()
                ),
                list_skill_versions_response: Err(
                    "no value supplied for list_skill_versions_response".to_string(),
                ),
                list_skills_error404: Err("no value supplied for list_skills_error404".to_string()),
                list_skills_error429: Err("no value supplied for list_skills_error429".to_string()),
                list_skills_params: Err("no value supplied for list_skills_params".to_string()),
                list_skills_response: Err("no value supplied for list_skills_response".to_string()),
                update_skill_default_version_error400: Err(
                    "no value supplied for update_skill_default_version_error400".to_string(),
                ),
                update_skill_default_version_error404: Err(
                    "no value supplied for update_skill_default_version_error404".to_string(),
                ),
                update_skill_default_version_error429: Err(
                    "no value supplied for update_skill_default_version_error429".to_string(),
                ),
                update_skill_default_version_params: Err(
                    "no value supplied for update_skill_default_version_params".to_string(),
                ),
                update_skill_default_version_request: Err(
                    "no value supplied for update_skill_default_version_request".to_string(),
                ),
                update_skill_default_version_response: Err(
                    "no value supplied for update_skill_default_version_response".to_string(),
                ),
            }
        }
    }
    impl SkillsApiSchema {
        pub fn create_skill_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_error400: {e}")
            });
            self
        }
        pub fn create_skill_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_error429: {e}")
            });
            self
        }
        pub fn create_skill_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_error500: {e}")
            });
            self
        }
        pub fn create_skill_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_skill_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_skill_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_skill_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_request: {e}")
            });
            self
        }
        pub fn create_skill_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_response: {e}")
            });
            self
        }
        pub fn create_skill_version_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_error400: {e}")
            });
            self
        }
        pub fn create_skill_version_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_error404: {e}")
            });
            self
        }
        pub fn create_skill_version_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_error429: {e}")
            });
            self
        }
        pub fn create_skill_version_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_error500: {e}")
            });
            self
        }
        pub fn create_skill_version_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillVersionBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_skill_version_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_skill_version_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_skill_version_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillVersionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_params: {e}")
            });
            self
        }
        pub fn create_skill_version_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateSkillVersionBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_request: {e}")
            });
            self
        }
        pub fn create_skill_version_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillVersionResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_skill_version_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_skill_version_response: {e}")
            });
            self
        }
        pub fn delete_skill_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_error404: {e}")
            });
            self
        }
        pub fn delete_skill_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_error429: {e}")
            });
            self
        }
        pub fn delete_skill_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteSkillParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_params: {e}")
            });
            self
        }
        pub fn delete_skill_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedSkillResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_response: {e}")
            });
            self
        }
        pub fn delete_skill_version_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_version_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_version_error400: {e}")
            });
            self
        }
        pub fn delete_skill_version_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_version_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_version_error404: {e}")
            });
            self
        }
        pub fn delete_skill_version_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_version_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_version_error429: {e}")
            });
            self
        }
        pub fn delete_skill_version_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteSkillVersionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_version_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_version_params: {e}")
            });
            self
        }
        pub fn delete_skill_version_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedSkillVersionResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_skill_version_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_skill_version_response: {e}")
            });
            self
        }
        pub fn get_skill_content_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_content_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_content_error404: {e}")
            });
            self
        }
        pub fn get_skill_content_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_content_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_content_error429: {e}")
            });
            self
        }
        pub fn get_skill_content_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetSkillContentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_content_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_content_params: {e}")
            });
            self
        }
        pub fn get_skill_content_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_content_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_content_response: {e}")
            });
            self
        }
        pub fn get_skill_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_error404: {e}")
            });
            self
        }
        pub fn get_skill_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_error429: {e}")
            });
            self
        }
        pub fn get_skill_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetSkillParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_skill_params: {e}"));
            self
        }
        pub fn get_skill_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_response: {e}")
            });
            self
        }
        pub fn get_skill_version_content_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_content_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_skill_version_content_error400: {e}"
                )
            });
            self
        }
        pub fn get_skill_version_content_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_content_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_skill_version_content_error404: {e}"
                )
            });
            self
        }
        pub fn get_skill_version_content_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_content_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_skill_version_content_error429: {e}"
                )
            });
            self
        }
        pub fn get_skill_version_content_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetSkillVersionContentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_content_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_content_params: {e}")
            });
            self
        }
        pub fn get_skill_version_content_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_content_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for get_skill_version_content_response: {e}"
                )
            });
            self
        }
        pub fn get_skill_version_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_error400: {e}")
            });
            self
        }
        pub fn get_skill_version_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_error404: {e}")
            });
            self
        }
        pub fn get_skill_version_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_error429: {e}")
            });
            self
        }
        pub fn get_skill_version_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetSkillVersionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_params: {e}")
            });
            self
        }
        pub fn get_skill_version_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillVersionResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_skill_version_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_skill_version_response: {e}")
            });
            self
        }
        pub fn list_skill_versions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skill_versions_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skill_versions_error400: {e}")
            });
            self
        }
        pub fn list_skill_versions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skill_versions_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skill_versions_error404: {e}")
            });
            self
        }
        pub fn list_skill_versions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skill_versions_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skill_versions_error429: {e}")
            });
            self
        }
        pub fn list_skill_versions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListSkillVersionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skill_versions_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skill_versions_params: {e}")
            });
            self
        }
        pub fn list_skill_versions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillVersionListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skill_versions_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skill_versions_response: {e}")
            });
            self
        }
        pub fn list_skills_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skills_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skills_error404: {e}")
            });
            self
        }
        pub fn list_skills_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skills_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skills_error429: {e}")
            });
            self
        }
        pub fn list_skills_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListSkillsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skills_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skills_params: {e}")
            });
            self
        }
        pub fn list_skills_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_skills_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_skills_response: {e}")
            });
            self
        }
        pub fn update_skill_default_version_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_skill_default_version_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_skill_default_version_error400: {e}")) ;
            self
        }
        pub fn update_skill_default_version_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_skill_default_version_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_skill_default_version_error404: {e}")) ;
            self
        }
        pub fn update_skill_default_version_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_skill_default_version_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_skill_default_version_error429: {e}")) ;
            self
        }
        pub fn update_skill_default_version_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateSkillDefaultVersionParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_skill_default_version_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_skill_default_version_params: {e}"
                )
            });
            self
        }
        pub fn update_skill_default_version_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SetDefaultSkillVersionBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_skill_default_version_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_skill_default_version_request: {e}"
                )
            });
            self
        }
        pub fn update_skill_default_version_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillResource>,
            T::Error: ::std::fmt::Display,
        {
            self . update_skill_default_version_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_skill_default_version_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<SkillsApiSchema> for super::SkillsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_skill_error400: value.create_skill_error400?,
                create_skill_error429: value.create_skill_error429?,
                create_skill_error500: value.create_skill_error500?,
                create_skill_multipart_form_data_request: value
                    .create_skill_multipart_form_data_request?,
                create_skill_request: value.create_skill_request?,
                create_skill_response: value.create_skill_response?,
                create_skill_version_error400: value.create_skill_version_error400?,
                create_skill_version_error404: value.create_skill_version_error404?,
                create_skill_version_error429: value.create_skill_version_error429?,
                create_skill_version_error500: value.create_skill_version_error500?,
                create_skill_version_multipart_form_data_request: value
                    .create_skill_version_multipart_form_data_request?,
                create_skill_version_params: value.create_skill_version_params?,
                create_skill_version_request: value.create_skill_version_request?,
                create_skill_version_response: value.create_skill_version_response?,
                delete_skill_error404: value.delete_skill_error404?,
                delete_skill_error429: value.delete_skill_error429?,
                delete_skill_params: value.delete_skill_params?,
                delete_skill_response: value.delete_skill_response?,
                delete_skill_version_error400: value.delete_skill_version_error400?,
                delete_skill_version_error404: value.delete_skill_version_error404?,
                delete_skill_version_error429: value.delete_skill_version_error429?,
                delete_skill_version_params: value.delete_skill_version_params?,
                delete_skill_version_response: value.delete_skill_version_response?,
                get_skill_content_error404: value.get_skill_content_error404?,
                get_skill_content_error429: value.get_skill_content_error429?,
                get_skill_content_params: value.get_skill_content_params?,
                get_skill_content_response: value.get_skill_content_response?,
                get_skill_error404: value.get_skill_error404?,
                get_skill_error429: value.get_skill_error429?,
                get_skill_params: value.get_skill_params?,
                get_skill_response: value.get_skill_response?,
                get_skill_version_content_error400: value.get_skill_version_content_error400?,
                get_skill_version_content_error404: value.get_skill_version_content_error404?,
                get_skill_version_content_error429: value.get_skill_version_content_error429?,
                get_skill_version_content_params: value.get_skill_version_content_params?,
                get_skill_version_content_response: value.get_skill_version_content_response?,
                get_skill_version_error400: value.get_skill_version_error400?,
                get_skill_version_error404: value.get_skill_version_error404?,
                get_skill_version_error429: value.get_skill_version_error429?,
                get_skill_version_params: value.get_skill_version_params?,
                get_skill_version_response: value.get_skill_version_response?,
                list_skill_versions_error400: value.list_skill_versions_error400?,
                list_skill_versions_error404: value.list_skill_versions_error404?,
                list_skill_versions_error429: value.list_skill_versions_error429?,
                list_skill_versions_params: value.list_skill_versions_params?,
                list_skill_versions_response: value.list_skill_versions_response?,
                list_skills_error404: value.list_skills_error404?,
                list_skills_error429: value.list_skills_error429?,
                list_skills_params: value.list_skills_params?,
                list_skills_response: value.list_skills_response?,
                update_skill_default_version_error400: value
                    .update_skill_default_version_error400?,
                update_skill_default_version_error404: value
                    .update_skill_default_version_error404?,
                update_skill_default_version_error429: value
                    .update_skill_default_version_error429?,
                update_skill_default_version_params: value.update_skill_default_version_params?,
                update_skill_default_version_request: value.update_skill_default_version_request?,
                update_skill_default_version_response: value
                    .update_skill_default_version_response?,
            })
        }
    }
    impl ::std::convert::From<super::SkillsApiSchema> for SkillsApiSchema {
        fn from(value: super::SkillsApiSchema) -> Self {
            Self {
                create_skill_error400: Ok(value.create_skill_error400),
                create_skill_error429: Ok(value.create_skill_error429),
                create_skill_error500: Ok(value.create_skill_error500),
                create_skill_multipart_form_data_request: Ok(
                    value.create_skill_multipart_form_data_request
                ),
                create_skill_request: Ok(value.create_skill_request),
                create_skill_response: Ok(value.create_skill_response),
                create_skill_version_error400: Ok(value.create_skill_version_error400),
                create_skill_version_error404: Ok(value.create_skill_version_error404),
                create_skill_version_error429: Ok(value.create_skill_version_error429),
                create_skill_version_error500: Ok(value.create_skill_version_error500),
                create_skill_version_multipart_form_data_request: Ok(
                    value.create_skill_version_multipart_form_data_request
                ),
                create_skill_version_params: Ok(value.create_skill_version_params),
                create_skill_version_request: Ok(value.create_skill_version_request),
                create_skill_version_response: Ok(value.create_skill_version_response),
                delete_skill_error404: Ok(value.delete_skill_error404),
                delete_skill_error429: Ok(value.delete_skill_error429),
                delete_skill_params: Ok(value.delete_skill_params),
                delete_skill_response: Ok(value.delete_skill_response),
                delete_skill_version_error400: Ok(value.delete_skill_version_error400),
                delete_skill_version_error404: Ok(value.delete_skill_version_error404),
                delete_skill_version_error429: Ok(value.delete_skill_version_error429),
                delete_skill_version_params: Ok(value.delete_skill_version_params),
                delete_skill_version_response: Ok(value.delete_skill_version_response),
                get_skill_content_error404: Ok(value.get_skill_content_error404),
                get_skill_content_error429: Ok(value.get_skill_content_error429),
                get_skill_content_params: Ok(value.get_skill_content_params),
                get_skill_content_response: Ok(value.get_skill_content_response),
                get_skill_error404: Ok(value.get_skill_error404),
                get_skill_error429: Ok(value.get_skill_error429),
                get_skill_params: Ok(value.get_skill_params),
                get_skill_response: Ok(value.get_skill_response),
                get_skill_version_content_error400: Ok(value.get_skill_version_content_error400),
                get_skill_version_content_error404: Ok(value.get_skill_version_content_error404),
                get_skill_version_content_error429: Ok(value.get_skill_version_content_error429),
                get_skill_version_content_params: Ok(value.get_skill_version_content_params),
                get_skill_version_content_response: Ok(value.get_skill_version_content_response),
                get_skill_version_error400: Ok(value.get_skill_version_error400),
                get_skill_version_error404: Ok(value.get_skill_version_error404),
                get_skill_version_error429: Ok(value.get_skill_version_error429),
                get_skill_version_params: Ok(value.get_skill_version_params),
                get_skill_version_response: Ok(value.get_skill_version_response),
                list_skill_versions_error400: Ok(value.list_skill_versions_error400),
                list_skill_versions_error404: Ok(value.list_skill_versions_error404),
                list_skill_versions_error429: Ok(value.list_skill_versions_error429),
                list_skill_versions_params: Ok(value.list_skill_versions_params),
                list_skill_versions_response: Ok(value.list_skill_versions_response),
                list_skills_error404: Ok(value.list_skills_error404),
                list_skills_error429: Ok(value.list_skills_error429),
                list_skills_params: Ok(value.list_skills_params),
                list_skills_response: Ok(value.list_skills_response),
                update_skill_default_version_error400: Ok(
                    value.update_skill_default_version_error400
                ),
                update_skill_default_version_error404: Ok(
                    value.update_skill_default_version_error404
                ),
                update_skill_default_version_error429: Ok(
                    value.update_skill_default_version_error429
                ),
                update_skill_default_version_params: Ok(value.update_skill_default_version_params),
                update_skill_default_version_request: Ok(value.update_skill_default_version_request),
                update_skill_default_version_response: Ok(
                    value.update_skill_default_version_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateSkillDefaultVersionParams {
        skill_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateSkillDefaultVersionParams {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
            }
        }
    }
    impl UpdateSkillDefaultVersionParams {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateSkillDefaultVersionParams>
        for super::UpdateSkillDefaultVersionParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateSkillDefaultVersionParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateSkillDefaultVersionParams>
        for UpdateSkillDefaultVersionParams
    {
        fn from(value: super::UpdateSkillDefaultVersionParams) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
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
