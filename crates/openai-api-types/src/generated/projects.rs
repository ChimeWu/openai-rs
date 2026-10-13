pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`ArchiveProjectParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ArchiveProjectParams {
    pub project_id: ::std::string::String,
}
impl ArchiveProjectParams {
    pub fn builder() -> builder::ArchiveProjectParams {
        Default::default()
    }
}
#[doc = "`CreateProjectServiceAccountParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateProjectServiceAccountParams {
    pub project_id: ::std::string::String,
}
impl CreateProjectServiceAccountParams {
    pub fn builder() -> builder::CreateProjectServiceAccountParams {
        Default::default()
    }
}
#[doc = "`CreateProjectUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateProjectUserParams {
    pub project_id: ::std::string::String,
}
impl CreateProjectUserParams {
    pub fn builder() -> builder::CreateProjectUserParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectApiKeyParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectApiKeyParams {
    pub api_key_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl DeleteProjectApiKeyParams {
    pub fn builder() -> builder::DeleteProjectApiKeyParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectModelPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectModelPermissionsParams {
    pub project_id: ::std::string::String,
}
impl DeleteProjectModelPermissionsParams {
    pub fn builder() -> builder::DeleteProjectModelPermissionsParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectServiceAccountParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectServiceAccountParams {
    pub project_id: ::std::string::String,
    pub service_account_id: ::std::string::String,
}
impl DeleteProjectServiceAccountParams {
    pub fn builder() -> builder::DeleteProjectServiceAccountParams {
        Default::default()
    }
}
#[doc = "`DeleteProjectUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteProjectUserParams {
    pub project_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl DeleteProjectUserParams {
    pub fn builder() -> builder::DeleteProjectUserParams {
        Default::default()
    }
}
#[doc = "`ListProjectApiKeysParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectApiKeysParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner_project_access: ::std::option::Option<ListProjectApiKeysParamsOwnerProjectAccess>,
    pub project_id: ::std::string::String,
}
impl ListProjectApiKeysParams {
    pub fn builder() -> builder::ListProjectApiKeysParams {
        Default::default()
    }
}
#[doc = "`ListProjectApiKeysParamsOwnerProjectAccess`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListProjectApiKeysParamsOwnerProjectAccess {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "any")]
    Any,
}
impl ::std::fmt::Display for ListProjectApiKeysParamsOwnerProjectAccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Inactive => f.write_str("inactive"),
            Self::Any => f.write_str("any"),
        }
    }
}
impl ::std::str::FromStr for ListProjectApiKeysParamsOwnerProjectAccess {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "any" => Ok(Self::Any),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectApiKeysParamsOwnerProjectAccess {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectApiKeysParamsOwnerProjectAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListProjectRateLimitsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectRateLimitsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    pub project_id: ::std::string::String,
}
impl ListProjectRateLimitsParams {
    pub fn builder() -> builder::ListProjectRateLimitsParams {
        Default::default()
    }
}
#[doc = "`ListProjectServiceAccountsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectServiceAccountsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    pub project_id: ::std::string::String,
}
impl ListProjectServiceAccountsParams {
    pub fn builder() -> builder::ListProjectServiceAccountsParams {
        Default::default()
    }
}
#[doc = "`ListProjectUsersParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectUsersParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    pub project_id: ::std::string::String,
}
impl ListProjectUsersParams {
    pub fn builder() -> builder::ListProjectUsersParams {
        Default::default()
    }
}
#[doc = "`ListProjectsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListProjectsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_archived: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
impl ListProjectsParams {
    pub fn builder() -> builder::ListProjectsParams {
        Default::default()
    }
}
#[doc = "`ModifyProjectParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyProjectParams {
    pub project_id: ::std::string::String,
}
impl ModifyProjectParams {
    pub fn builder() -> builder::ModifyProjectParams {
        Default::default()
    }
}
#[doc = "`ModifyProjectUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyProjectUserParams {
    pub project_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl ModifyProjectUserParams {
    pub fn builder() -> builder::ModifyProjectUserParams {
        Default::default()
    }
}
#[doc = "Represents an individual project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Project {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub archived_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) of when the project was created."]
    pub created_at: i64,
    #[doc = "The external key associated with the project."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub external_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the project. This appears in reporting."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.project`"]
    pub object: ProjectObject,
    #[doc = "The residency configuration for the project."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub residency: ::std::option::Option<PublicProjectResidency>,
    #[doc = "`active` or `archived`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
}
impl Project {
    pub fn builder() -> builder::Project {
        Default::default()
    }
}
#[doc = "Represents an individual API key in a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectApiKey {
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    pub created_at: i64,
    #[doc = "The Unix timestamp (in seconds) when the API key expires, or null if it does not expire."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) of when the API key was last used."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_used_at: ::std::option::Option<i64>,
    #[doc = "The name of the API key"]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `organization.project.api_key`"]
    pub object: ProjectApiKeyObject,
    pub owner: ProjectApiKeyOwner,
    #[doc = "Whether the API key's owner currently has effective access to the project."]
    pub owner_project_access: ProjectApiKeyOwnerProjectAccess,
    #[doc = "The redacted value of the API key"]
    pub redacted_value: ::std::string::String,
}
impl ProjectApiKey {
    pub fn builder() -> builder::ProjectApiKey {
        Default::default()
    }
}
#[doc = "`ProjectApiKeyDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectApiKeyDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: ProjectApiKeyDeleteResponseObject,
}
impl ProjectApiKeyDeleteResponse {
    pub fn builder() -> builder::ProjectApiKeyDeleteResponse {
        Default::default()
    }
}
#[doc = "`ProjectApiKeyDeleteResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectApiKeyDeleteResponseObject {
    #[serde(rename = "organization.project.api_key.deleted")]
    OrganizationProjectApiKeyDeleted,
}
impl ::std::fmt::Display for ProjectApiKeyDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectApiKeyDeleted => {
                f.write_str("organization.project.api_key.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectApiKeyDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.api_key.deleted" => Ok(Self::OrganizationProjectApiKeyDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectApiKeyDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectApiKeyDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectApiKeyListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectApiKeyListResponse {
    pub data: ::std::vec::Vec<ProjectApiKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ProjectApiKeyListResponseObject,
}
impl ProjectApiKeyListResponse {
    pub fn builder() -> builder::ProjectApiKeyListResponse {
        Default::default()
    }
}
#[doc = "`ProjectApiKeyListResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectApiKeyListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectApiKeyListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectApiKeyListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectApiKeyListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectApiKeyListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.project.api_key`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectApiKeyObject {
    #[serde(rename = "organization.project.api_key")]
    OrganizationProjectApiKey,
}
impl ::std::fmt::Display for ProjectApiKeyObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectApiKey => f.write_str("organization.project.api_key"),
        }
    }
}
impl ::std::str::FromStr for ProjectApiKeyObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.api_key" => Ok(Self::OrganizationProjectApiKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectApiKeyOwner`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ProjectApiKeyOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_account: ::std::option::Option<ProjectApiKeyOwnerServiceAccount>,
    #[doc = "`user` or `service_account`"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ProjectApiKeyOwnerType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<ProjectApiKeyOwnerUser>,
}
impl ProjectApiKeyOwner {
    pub fn builder() -> builder::ProjectApiKeyOwner {
        Default::default()
    }
}
#[doc = "Whether the API key's owner currently has effective access to the project."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectApiKeyOwnerProjectAccess {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}
impl ::std::fmt::Display for ProjectApiKeyOwnerProjectAccess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Inactive => f.write_str("inactive"),
        }
    }
}
impl ::std::str::FromStr for ProjectApiKeyOwnerProjectAccess {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectApiKeyOwnerProjectAccess {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectApiKeyOwnerProjectAccess {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The service account that owns a project API key."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectApiKeyOwnerServiceAccount {
    #[doc = "The Unix timestamp (in seconds) of when the service account was created."]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the service account."]
    pub name: ::std::string::String,
    #[doc = "The service account's project role."]
    pub role: ::std::string::String,
}
impl ProjectApiKeyOwnerServiceAccount {
    pub fn builder() -> builder::ProjectApiKeyOwnerServiceAccount {
        Default::default()
    }
}
#[doc = "`user` or `service_account`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectApiKeyOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "service_account")]
    ServiceAccount,
}
impl ::std::fmt::Display for ProjectApiKeyOwnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::ServiceAccount => f.write_str("service_account"),
        }
    }
}
impl ::std::str::FromStr for ProjectApiKeyOwnerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "service_account" => Ok(Self::ServiceAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectApiKeyOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectApiKeyOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The user that owns a project API key."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectApiKeyOwnerUser {
    #[doc = "The Unix timestamp (in seconds) of when the user was created."]
    pub created_at: i64,
    #[doc = "The email address of the user."]
    pub email: ::std::string::String,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the user."]
    pub name: ::std::string::String,
    #[doc = "The user's project role."]
    pub role: ::std::string::String,
}
impl ProjectApiKeyOwnerUser {
    pub fn builder() -> builder::ProjectApiKeyOwnerUser {
        Default::default()
    }
}
#[doc = "`ProjectCreateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCreateRequest {
    #[doc = "External key ID to associate with the project."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub external_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "Create the project with the specified data residency region. Your organization must have access to Data residency functionality in order to use. See [data residency controls](https://developers.openai.com/api/docs/guides/your-data#data-residency-controls) to review the functionality and limitations of setting this field.\nDeprecated: use `residency` instead. Do not provide both `geography` and `residency`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub geography: ::std::option::Option<::std::string::String>,
    #[doc = "The friendly name of the project, this name appears in reports."]
    pub name: ::std::string::String,
    #[doc = "Create the project with the specified residency configuration. Your organization must have access to the requested residency configuration in order to use it. See [data residency controls](https://developers.openai.com/api/docs/guides/your-data#data-residency-controls) to review the functionality and limitations of setting this field."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub residency: ::std::option::Option<PublicProjectResidency>,
}
impl ProjectCreateRequest {
    pub fn builder() -> builder::ProjectCreateRequest {
        Default::default()
    }
}
#[doc = "`ProjectListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectListResponse {
    pub data: ::std::vec::Vec<Project>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ProjectListResponseObject,
}
impl ProjectListResponse {
    pub fn builder() -> builder::ProjectListResponse {
        Default::default()
    }
}
#[doc = "`ProjectListResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents the model allowlist or denylist policy for a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectModelPermissions {
    #[doc = "Whether the project uses an allowlist or a denylist."]
    pub mode: ProjectModelPermissionsMode,
    #[doc = "The model IDs included in the model permissions policy."]
    pub model_ids: ::std::vec::Vec<::std::string::String>,
    #[doc = "The object type, which is always `project.model_permissions`."]
    pub object: ProjectModelPermissionsObject,
}
impl ProjectModelPermissions {
    pub fn builder() -> builder::ProjectModelPermissions {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting project model permissions."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectModelPermissionsDeleteResponse {
    #[doc = "Whether the project model permissions were deleted."]
    pub deleted: bool,
    #[doc = "The object type, which is always `project.model_permissions.deleted`."]
    pub object: ProjectModelPermissionsDeleteResponseObject,
}
impl ProjectModelPermissionsDeleteResponse {
    pub fn builder() -> builder::ProjectModelPermissionsDeleteResponse {
        Default::default()
    }
}
#[doc = "The object type, which is always `project.model_permissions.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectModelPermissionsDeleteResponseObject {
    #[serde(rename = "project.model_permissions.deleted")]
    ProjectModelPermissionsDeleted,
}
impl ::std::fmt::Display for ProjectModelPermissionsDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectModelPermissionsDeleted => {
                f.write_str("project.model_permissions.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectModelPermissionsDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.model_permissions.deleted" => Ok(Self::ProjectModelPermissionsDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectModelPermissionsDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ProjectModelPermissionsDeleteResponseObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Whether the project uses an allowlist or a denylist."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectModelPermissionsMode {
    #[serde(rename = "allow_list")]
    AllowList,
    #[serde(rename = "deny_list")]
    DenyList,
}
impl ::std::fmt::Display for ProjectModelPermissionsMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllowList => f.write_str("allow_list"),
            Self::DenyList => f.write_str("deny_list"),
        }
    }
}
impl ::std::str::FromStr for ProjectModelPermissionsMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow_list" => Ok(Self::AllowList),
            "deny_list" => Ok(Self::DenyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectModelPermissionsMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectModelPermissionsMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `project.model_permissions`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectModelPermissionsObject {
    #[serde(rename = "project.model_permissions")]
    ProjectModelPermissions,
}
impl ::std::fmt::Display for ProjectModelPermissionsObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectModelPermissions => f.write_str("project.model_permissions"),
        }
    }
}
impl ::std::str::FromStr for ProjectModelPermissionsObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.model_permissions" => Ok(Self::ProjectModelPermissions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectModelPermissionsObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectModelPermissionsObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectModelPermissionsUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectModelPermissionsUpdateRequest {
    #[doc = "The model permissions mode to apply."]
    pub mode: ProjectModelPermissionsUpdateRequestMode,
    #[doc = "The model IDs included in this permissions policy."]
    pub model_ids: ::std::vec::Vec<::std::string::String>,
}
impl ProjectModelPermissionsUpdateRequest {
    pub fn builder() -> builder::ProjectModelPermissionsUpdateRequest {
        Default::default()
    }
}
#[doc = "The model permissions mode to apply."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectModelPermissionsUpdateRequestMode {
    #[serde(rename = "allow_list")]
    AllowList,
    #[serde(rename = "deny_list")]
    DenyList,
}
impl ::std::fmt::Display for ProjectModelPermissionsUpdateRequestMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllowList => f.write_str("allow_list"),
            Self::DenyList => f.write_str("deny_list"),
        }
    }
}
impl ::std::str::FromStr for ProjectModelPermissionsUpdateRequestMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow_list" => Ok(Self::AllowList),
            "deny_list" => Ok(Self::DenyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectModelPermissionsUpdateRequestMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectModelPermissionsUpdateRequestMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.project`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectObject {
    #[serde(rename = "organization.project")]
    OrganizationProject,
}
impl ::std::fmt::Display for ProjectObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProject => f.write_str("organization.project"),
        }
    }
}
impl ::std::str::FromStr for ProjectObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project" => Ok(Self::OrganizationProject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a project rate limit config."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectRateLimit {
    #[doc = "The maximum batch input tokens per day. Only present for relevant models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_1_day_max_input_tokens: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: ::std::string::String,
    #[doc = "The maximum audio megabytes per minute. Only present for relevant models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_audio_megabytes_per_1_minute: ::std::option::Option<i64>,
    #[doc = "The maximum images per minute. Only present for relevant models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_images_per_1_minute: ::std::option::Option<i64>,
    #[doc = "The maximum requests per day. Only present for relevant models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_requests_per_1_day: ::std::option::Option<i64>,
    #[doc = "The maximum requests per minute."]
    pub max_requests_per_1_minute: i64,
    #[doc = "The maximum tokens per minute."]
    pub max_tokens_per_1_minute: i64,
    #[doc = "The model this rate limit applies to."]
    pub model: ::std::string::String,
    #[doc = "The object type, which is always `project.rate_limit`"]
    pub object: ProjectRateLimitObject,
}
impl ProjectRateLimit {
    pub fn builder() -> builder::ProjectRateLimit {
        Default::default()
    }
}
#[doc = "`ProjectRateLimitListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectRateLimitListResponse {
    pub data: ::std::vec::Vec<ProjectRateLimit>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ProjectRateLimitListResponseObject,
}
impl ProjectRateLimitListResponse {
    pub fn builder() -> builder::ProjectRateLimitListResponse {
        Default::default()
    }
}
#[doc = "`ProjectRateLimitListResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectRateLimitListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectRateLimitListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectRateLimitListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectRateLimitListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectRateLimitListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `project.rate_limit`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectRateLimitObject {
    #[serde(rename = "project.rate_limit")]
    ProjectRateLimit,
}
impl ::std::fmt::Display for ProjectRateLimitObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectRateLimit => f.write_str("project.rate_limit"),
        }
    }
}
impl ::std::str::FromStr for ProjectRateLimitObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.rate_limit" => Ok(Self::ProjectRateLimit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectRateLimitObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectRateLimitObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectRateLimitUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ProjectRateLimitUpdateRequest {
    #[doc = "The maximum batch input tokens per day. Only relevant for certain models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_1_day_max_input_tokens: ::std::option::Option<i64>,
    #[doc = "The maximum audio megabytes per minute. Only relevant for certain models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_audio_megabytes_per_1_minute: ::std::option::Option<i64>,
    #[doc = "The maximum images per minute. Only relevant for certain models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_images_per_1_minute: ::std::option::Option<i64>,
    #[doc = "The maximum requests per day. Only relevant for certain models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_requests_per_1_day: ::std::option::Option<i64>,
    #[doc = "The maximum requests per minute."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_requests_per_1_minute: ::std::option::Option<i64>,
    #[doc = "The maximum tokens per minute."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tokens_per_1_minute: ::std::option::Option<i64>,
}
impl ProjectRateLimitUpdateRequest {
    pub fn builder() -> builder::ProjectRateLimitUpdateRequest {
        Default::default()
    }
}
#[doc = "Represents an individual service account in a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccount {
    #[doc = "The Unix timestamp (in seconds) of when the service account was created"]
    pub created_at: i64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the service account"]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `organization.project.service_account`"]
    pub object: ProjectServiceAccountObject,
    #[doc = "`owner`, `member`, or `none`"]
    pub role: ProjectServiceAccountRole,
}
impl ProjectServiceAccount {
    pub fn builder() -> builder::ProjectServiceAccount {
        Default::default()
    }
}
#[doc = "`ProjectServiceAccountApiKey`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccountApiKey {
    pub created_at: i64,
    #[doc = "The Unix timestamp (in seconds) when the API key expires, or null if it does not expire."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `organization.project.service_account.api_key`"]
    pub object: ProjectServiceAccountApiKeyObject,
    pub value: ::std::string::String,
}
impl ProjectServiceAccountApiKey {
    pub fn builder() -> builder::ProjectServiceAccountApiKey {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.project.service_account.api_key`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountApiKeyObject {
    #[serde(rename = "organization.project.service_account.api_key")]
    OrganizationProjectServiceAccountApiKey,
}
impl ::std::fmt::Display for ProjectServiceAccountApiKeyObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectServiceAccountApiKey => {
                f.write_str("organization.project.service_account.api_key")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountApiKeyObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.service_account.api_key" => {
                Ok(Self::OrganizationProjectServiceAccountApiKey)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectServiceAccountCreateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccountCreateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub create_service_account_only: ::std::option::Option<bool>,
    #[doc = "Number of seconds until the initial API key expires. If omitted or null, the key does not expire unless the effective organization or project policy requires an expiration. When a policy sets a maximum lifetime, this value must be provided and must not exceed that limit. A non-null value cannot be used when `create_service_account_only` is true."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_in_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "The name of the service account being created."]
    pub name: ::std::string::String,
}
impl ProjectServiceAccountCreateRequest {
    pub fn builder() -> builder::ProjectServiceAccountCreateRequest {
        Default::default()
    }
}
#[doc = "`ProjectServiceAccountCreateResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccountCreateResponse {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub api_key: ::std::option::Option<ProjectServiceAccountApiKey>,
    pub created_at: i64,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub object: ProjectServiceAccountCreateResponseObject,
    #[doc = "Service accounts created with default project membership have role `member`. Accounts created with `create_service_account_only` have role `none`.\n"]
    pub role: ProjectServiceAccountCreateResponseRole,
}
impl ProjectServiceAccountCreateResponse {
    pub fn builder() -> builder::ProjectServiceAccountCreateResponse {
        Default::default()
    }
}
#[doc = "`ProjectServiceAccountCreateResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountCreateResponseObject {
    #[serde(rename = "organization.project.service_account")]
    OrganizationProjectServiceAccount,
}
impl ::std::fmt::Display for ProjectServiceAccountCreateResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectServiceAccount => {
                f.write_str("organization.project.service_account")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountCreateResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.service_account" => Ok(Self::OrganizationProjectServiceAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountCreateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountCreateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Service accounts created with default project membership have role `member`. Accounts created with `create_service_account_only` have role `none`.\n"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountCreateResponseRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for ProjectServiceAccountCreateResponseRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountCreateResponseRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountCreateResponseRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountCreateResponseRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectServiceAccountDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccountDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: ProjectServiceAccountDeleteResponseObject,
}
impl ProjectServiceAccountDeleteResponse {
    pub fn builder() -> builder::ProjectServiceAccountDeleteResponse {
        Default::default()
    }
}
#[doc = "`ProjectServiceAccountDeleteResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountDeleteResponseObject {
    #[serde(rename = "organization.project.service_account.deleted")]
    OrganizationProjectServiceAccountDeleted,
}
impl ::std::fmt::Display for ProjectServiceAccountDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectServiceAccountDeleted => {
                f.write_str("organization.project.service_account.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.service_account.deleted" => {
                Ok(Self::OrganizationProjectServiceAccountDeleted)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectServiceAccountListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectServiceAccountListResponse {
    pub data: ::std::vec::Vec<ProjectServiceAccount>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ProjectServiceAccountListResponseObject,
}
impl ProjectServiceAccountListResponse {
    pub fn builder() -> builder::ProjectServiceAccountListResponse {
        Default::default()
    }
}
#[doc = "`ProjectServiceAccountListResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ProjectServiceAccountListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.project.service_account`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountObject {
    #[serde(rename = "organization.project.service_account")]
    OrganizationProjectServiceAccount,
}
impl ::std::fmt::Display for ProjectServiceAccountObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectServiceAccount => {
                f.write_str("organization.project.service_account")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.service_account" => Ok(Self::OrganizationProjectServiceAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`owner`, `member`, or `none`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectServiceAccountRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for ProjectServiceAccountRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Owner => f.write_str("owner"),
            Self::Member => f.write_str("member"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for ProjectServiceAccountRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "owner" => Ok(Self::Owner),
            "member" => Ok(Self::Member),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectServiceAccountRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectServiceAccountRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ProjectUpdateRequest {
    #[doc = "External key ID to associate with the project."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub external_key_id: ::std::option::Option<::std::string::String>,
    #[doc = "Geography for the project.\nDeprecated: use `residency` when creating a project to configure data residency. This field is retained for backward compatibility."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub geography: ::std::option::Option<::std::string::String>,
    #[doc = "The updated name of the project, this name appears in reports."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ProjectUpdateRequest {
    pub fn builder() -> builder::ProjectUpdateRequest {
        Default::default()
    }
}
#[doc = "Represents an individual user in a project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectUser {
    #[doc = "The Unix timestamp (in seconds) of when the project was added."]
    pub added_at: i64,
    #[doc = "The email address of the user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.project.user`"]
    pub object: ProjectUserObject,
    #[doc = "`owner` or `member`"]
    pub role: ::std::string::String,
}
impl ProjectUser {
    pub fn builder() -> builder::ProjectUser {
        Default::default()
    }
}
#[doc = "`ProjectUserCreateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectUserCreateRequest {
    #[doc = "Email of the user to add."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "`owner` or `member`"]
    pub role: ::std::string::String,
    #[doc = "The ID of the user."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl ProjectUserCreateRequest {
    pub fn builder() -> builder::ProjectUserCreateRequest {
        Default::default()
    }
}
#[doc = "`ProjectUserDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectUserDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: ProjectUserDeleteResponseObject,
}
impl ProjectUserDeleteResponse {
    pub fn builder() -> builder::ProjectUserDeleteResponse {
        Default::default()
    }
}
#[doc = "`ProjectUserDeleteResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectUserDeleteResponseObject {
    #[serde(rename = "organization.project.user.deleted")]
    OrganizationProjectUserDeleted,
}
impl ::std::fmt::Display for ProjectUserDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectUserDeleted => {
                f.write_str("organization.project.user.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for ProjectUserDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.user.deleted" => Ok(Self::OrganizationProjectUserDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUserDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectUserDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectUserListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectUserListResponse {
    pub data: ::std::vec::Vec<ProjectUser>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ::std::string::String,
}
impl ProjectUserListResponse {
    pub fn builder() -> builder::ProjectUserListResponse {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.project.user`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectUserObject {
    #[serde(rename = "organization.project.user")]
    OrganizationProjectUser,
}
impl ::std::fmt::Display for ProjectUserObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectUser => f.write_str("organization.project.user"),
        }
    }
}
impl ::std::str::FromStr for ProjectUserObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.project.user" => Ok(Self::OrganizationProjectUser),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUserObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectUserObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectUserUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ProjectUserUpdateRequest {
    #[doc = "`owner` or `member`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl ProjectUserUpdateRequest {
    pub fn builder() -> builder::ProjectUserUpdateRequest {
        Default::default()
    }
}
#[doc = "`ProjectsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectsApiSchema {
    #[serde(rename = "ArchiveProjectError400")]
    pub archive_project_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ArchiveProjectError403")]
    pub archive_project_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ArchiveProjectError404")]
    pub archive_project_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ArchiveProjectError429")]
    pub archive_project_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ArchiveProjectParams")]
    pub archive_project_params: ArchiveProjectParams,
    #[serde(rename = "ArchiveProjectResponse")]
    pub archive_project_response: Project,
    #[serde(rename = "CreateProjectError400")]
    pub create_project_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectError403")]
    pub create_project_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateProjectError429")]
    pub create_project_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectRequest")]
    pub create_project_request: ProjectCreateRequest,
    #[serde(rename = "CreateProjectResponse")]
    pub create_project_response: Project,
    #[serde(rename = "CreateProjectServiceAccountError400")]
    pub create_project_service_account_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectServiceAccountError403")]
    pub create_project_service_account_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateProjectServiceAccountError404")]
    pub create_project_service_account_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectServiceAccountError429")]
    pub create_project_service_account_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectServiceAccountParams")]
    pub create_project_service_account_params: CreateProjectServiceAccountParams,
    #[serde(rename = "CreateProjectServiceAccountRequest")]
    pub create_project_service_account_request: ProjectServiceAccountCreateRequest,
    #[serde(rename = "CreateProjectServiceAccountResponse")]
    pub create_project_service_account_response: ProjectServiceAccountCreateResponse,
    #[serde(rename = "CreateProjectUserError400")]
    pub create_project_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectUserError403")]
    pub create_project_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateProjectUserError404")]
    pub create_project_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectUserError409")]
    pub create_project_user_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectUserError429")]
    pub create_project_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateProjectUserParams")]
    pub create_project_user_params: CreateProjectUserParams,
    #[serde(rename = "CreateProjectUserRequest")]
    pub create_project_user_request: ProjectUserCreateRequest,
    #[serde(rename = "CreateProjectUserResponse")]
    pub create_project_user_response: ProjectUser,
    #[serde(rename = "DeleteProjectApiKeyError400")]
    pub delete_project_api_key_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectApiKeyError403")]
    pub delete_project_api_key_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectApiKeyError404")]
    pub delete_project_api_key_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectApiKeyError429")]
    pub delete_project_api_key_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectApiKeyParams")]
    pub delete_project_api_key_params: DeleteProjectApiKeyParams,
    #[serde(rename = "DeleteProjectApiKeyResponse")]
    pub delete_project_api_key_response: ProjectApiKeyDeleteResponse,
    #[serde(rename = "DeleteProjectModelPermissionsError400")]
    pub delete_project_model_permissions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectModelPermissionsError403")]
    pub delete_project_model_permissions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectModelPermissionsError404")]
    pub delete_project_model_permissions_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectModelPermissionsError429")]
    pub delete_project_model_permissions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectModelPermissionsParams")]
    pub delete_project_model_permissions_params: DeleteProjectModelPermissionsParams,
    #[serde(rename = "DeleteProjectModelPermissionsResponse")]
    pub delete_project_model_permissions_response: ProjectModelPermissionsDeleteResponse,
    #[serde(rename = "DeleteProjectServiceAccountError400")]
    pub delete_project_service_account_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectServiceAccountError403")]
    pub delete_project_service_account_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectServiceAccountError404")]
    pub delete_project_service_account_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectServiceAccountError429")]
    pub delete_project_service_account_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectServiceAccountParams")]
    pub delete_project_service_account_params: DeleteProjectServiceAccountParams,
    #[serde(rename = "DeleteProjectServiceAccountResponse")]
    pub delete_project_service_account_response: ProjectServiceAccountDeleteResponse,
    #[serde(rename = "DeleteProjectUserError400")]
    pub delete_project_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectUserError403")]
    pub delete_project_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteProjectUserError404")]
    pub delete_project_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectUserError429")]
    pub delete_project_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteProjectUserParams")]
    pub delete_project_user_params: DeleteProjectUserParams,
    #[serde(rename = "DeleteProjectUserResponse")]
    pub delete_project_user_response: ProjectUserDeleteResponse,
    #[serde(rename = "ListProjectApiKeysError400")]
    pub list_project_api_keys_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectApiKeysError403")]
    pub list_project_api_keys_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectApiKeysError404")]
    pub list_project_api_keys_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectApiKeysError429")]
    pub list_project_api_keys_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectApiKeysParams")]
    pub list_project_api_keys_params: ListProjectApiKeysParams,
    #[serde(rename = "ListProjectApiKeysResponse")]
    pub list_project_api_keys_response: ProjectApiKeyListResponse,
    #[serde(rename = "ListProjectRateLimitsError400")]
    pub list_project_rate_limits_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectRateLimitsError403")]
    pub list_project_rate_limits_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectRateLimitsError404")]
    pub list_project_rate_limits_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectRateLimitsParams")]
    pub list_project_rate_limits_params: ListProjectRateLimitsParams,
    #[serde(rename = "ListProjectRateLimitsResponse")]
    pub list_project_rate_limits_response: ProjectRateLimitListResponse,
    #[serde(rename = "ListProjectServiceAccountsError400")]
    pub list_project_service_accounts_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectServiceAccountsError403")]
    pub list_project_service_accounts_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectServiceAccountsError404")]
    pub list_project_service_accounts_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectServiceAccountsError429")]
    pub list_project_service_accounts_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectServiceAccountsParams")]
    pub list_project_service_accounts_params: ListProjectServiceAccountsParams,
    #[serde(rename = "ListProjectServiceAccountsResponse")]
    pub list_project_service_accounts_response: ProjectServiceAccountListResponse,
    #[serde(rename = "ListProjectUsersError400")]
    pub list_project_users_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUsersError403")]
    pub list_project_users_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectUsersError404")]
    pub list_project_users_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUsersError429")]
    pub list_project_users_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUsersParams")]
    pub list_project_users_params: ListProjectUsersParams,
    #[serde(rename = "ListProjectUsersResponse")]
    pub list_project_users_response: ProjectUserListResponse,
    #[serde(rename = "ListProjectsError400")]
    pub list_projects_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectsError403")]
    pub list_projects_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectsError404")]
    pub list_projects_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectsError429")]
    pub list_projects_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectsParams")]
    pub list_projects_params: ListProjectsParams,
    #[serde(rename = "ListProjectsResponse")]
    pub list_projects_response: ProjectListResponse,
    #[serde(rename = "ModifyProjectError400")]
    pub modify_project_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectError403")]
    pub modify_project_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ModifyProjectError404")]
    pub modify_project_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectError429")]
    pub modify_project_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectParams")]
    pub modify_project_params: ModifyProjectParams,
    #[serde(rename = "ModifyProjectRequest")]
    pub modify_project_request: ProjectUpdateRequest,
    #[serde(rename = "ModifyProjectResponse")]
    pub modify_project_response: Project,
    #[serde(rename = "ModifyProjectUserError400")]
    pub modify_project_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectUserError403")]
    pub modify_project_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ModifyProjectUserError404")]
    pub modify_project_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectUserError429")]
    pub modify_project_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyProjectUserParams")]
    pub modify_project_user_params: ModifyProjectUserParams,
    #[serde(rename = "ModifyProjectUserRequest")]
    pub modify_project_user_request: ProjectUserUpdateRequest,
    #[serde(rename = "ModifyProjectUserResponse")]
    pub modify_project_user_response: ProjectUser,
    #[serde(rename = "RetrieveProjectApiKeyError400")]
    pub retrieve_project_api_key_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectApiKeyError403")]
    pub retrieve_project_api_key_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectApiKeyError404")]
    pub retrieve_project_api_key_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectApiKeyError429")]
    pub retrieve_project_api_key_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectApiKeyParams")]
    pub retrieve_project_api_key_params: RetrieveProjectApiKeyParams,
    #[serde(rename = "RetrieveProjectApiKeyResponse")]
    pub retrieve_project_api_key_response: ProjectApiKey,
    #[serde(rename = "RetrieveProjectError403")]
    pub retrieve_project_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectError404")]
    pub retrieve_project_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectError429")]
    pub retrieve_project_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectModelPermissionsError400")]
    pub retrieve_project_model_permissions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectModelPermissionsError403")]
    pub retrieve_project_model_permissions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectModelPermissionsError404")]
    pub retrieve_project_model_permissions_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectModelPermissionsError429")]
    pub retrieve_project_model_permissions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectModelPermissionsParams")]
    pub retrieve_project_model_permissions_params: RetrieveProjectModelPermissionsParams,
    #[serde(rename = "RetrieveProjectModelPermissionsResponse")]
    pub retrieve_project_model_permissions_response: ProjectModelPermissions,
    #[serde(rename = "RetrieveProjectParams")]
    pub retrieve_project_params: RetrieveProjectParams,
    #[serde(rename = "RetrieveProjectResponse")]
    pub retrieve_project_response: Project,
    #[serde(rename = "RetrieveProjectServiceAccountError400")]
    pub retrieve_project_service_account_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectServiceAccountError403")]
    pub retrieve_project_service_account_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectServiceAccountError404")]
    pub retrieve_project_service_account_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectServiceAccountError429")]
    pub retrieve_project_service_account_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectServiceAccountParams")]
    pub retrieve_project_service_account_params: RetrieveProjectServiceAccountParams,
    #[serde(rename = "RetrieveProjectServiceAccountResponse")]
    pub retrieve_project_service_account_response: ProjectServiceAccount,
    #[serde(rename = "RetrieveProjectUserError400")]
    pub retrieve_project_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserError403")]
    pub retrieve_project_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectUserError404")]
    pub retrieve_project_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserError429")]
    pub retrieve_project_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserParams")]
    pub retrieve_project_user_params: RetrieveProjectUserParams,
    #[serde(rename = "RetrieveProjectUserResponse")]
    pub retrieve_project_user_response: ProjectUser,
    #[serde(rename = "UpdateProjectApiKeyError400")]
    pub update_project_api_key_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectApiKeyError403")]
    pub update_project_api_key_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectApiKeyError404")]
    pub update_project_api_key_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectApiKeyError429")]
    pub update_project_api_key_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectApiKeyParams")]
    pub update_project_api_key_params: UpdateProjectApiKeyParams,
    #[serde(rename = "UpdateProjectApiKeyRequest")]
    pub update_project_api_key_request: UpdateProjectApiKeyBody,
    #[serde(rename = "UpdateProjectApiKeyResponse")]
    pub update_project_api_key_response: ProjectApiKey,
    #[serde(rename = "UpdateProjectModelPermissionsError400")]
    pub update_project_model_permissions_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectModelPermissionsError403")]
    pub update_project_model_permissions_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectModelPermissionsError404")]
    pub update_project_model_permissions_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectModelPermissionsError429")]
    pub update_project_model_permissions_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectModelPermissionsParams")]
    pub update_project_model_permissions_params: UpdateProjectModelPermissionsParams,
    #[serde(rename = "UpdateProjectModelPermissionsRequest")]
    pub update_project_model_permissions_request: ProjectModelPermissionsUpdateRequest,
    #[serde(rename = "UpdateProjectModelPermissionsResponse")]
    pub update_project_model_permissions_response: ProjectModelPermissions,
    #[serde(rename = "UpdateProjectRateLimitsError400")]
    pub update_project_rate_limits_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectRateLimitsError403")]
    pub update_project_rate_limits_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectRateLimitsError404")]
    pub update_project_rate_limits_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectRateLimitsParams")]
    pub update_project_rate_limits_params: UpdateProjectRateLimitsParams,
    #[serde(rename = "UpdateProjectRateLimitsRequest")]
    pub update_project_rate_limits_request: ProjectRateLimitUpdateRequest,
    #[serde(rename = "UpdateProjectRateLimitsResponse")]
    pub update_project_rate_limits_response: ProjectRateLimit,
    #[serde(rename = "UpdateProjectServiceAccountError400")]
    pub update_project_service_account_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectServiceAccountError403")]
    pub update_project_service_account_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateProjectServiceAccountError404")]
    pub update_project_service_account_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectServiceAccountError429")]
    pub update_project_service_account_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateProjectServiceAccountParams")]
    pub update_project_service_account_params: UpdateProjectServiceAccountParams,
    #[serde(rename = "UpdateProjectServiceAccountRequest")]
    pub update_project_service_account_request: UpdateProjectServiceAccountBody,
    #[serde(rename = "UpdateProjectServiceAccountResponse")]
    pub update_project_service_account_response: ProjectServiceAccount,
}
impl ProjectsApiSchema {
    pub fn builder() -> builder::ProjectsApiSchema {
        Default::default()
    }
}
#[doc = "`PublicProjectResidency`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PublicProjectResidency {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "US_STORAGE_PROCESSING")]
    UsStorageProcessing,
    #[serde(rename = "EU_STORAGE_PROCESSING")]
    EuStorageProcessing,
    #[serde(rename = "JP_STORAGE")]
    JpStorage,
    #[serde(rename = "KR_STORAGE")]
    KrStorage,
    #[serde(rename = "CA_STORAGE")]
    CaStorage,
    #[serde(rename = "SG_STORAGE")]
    SgStorage,
    #[serde(rename = "IN_STORAGE")]
    InStorage,
    #[serde(rename = "AU_STORAGE")]
    AuStorage,
    #[serde(rename = "GB_STORAGE")]
    GbStorage,
    #[serde(rename = "AE_STORAGE")]
    AeStorage,
    #[serde(rename = "AE_STORAGE_PROCESSING")]
    AeStorageProcessing,
}
impl ::std::fmt::Display for PublicProjectResidency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Global => f.write_str("GLOBAL"),
            Self::UsStorageProcessing => f.write_str("US_STORAGE_PROCESSING"),
            Self::EuStorageProcessing => f.write_str("EU_STORAGE_PROCESSING"),
            Self::JpStorage => f.write_str("JP_STORAGE"),
            Self::KrStorage => f.write_str("KR_STORAGE"),
            Self::CaStorage => f.write_str("CA_STORAGE"),
            Self::SgStorage => f.write_str("SG_STORAGE"),
            Self::InStorage => f.write_str("IN_STORAGE"),
            Self::AuStorage => f.write_str("AU_STORAGE"),
            Self::GbStorage => f.write_str("GB_STORAGE"),
            Self::AeStorage => f.write_str("AE_STORAGE"),
            Self::AeStorageProcessing => f.write_str("AE_STORAGE_PROCESSING"),
        }
    }
}
impl ::std::str::FromStr for PublicProjectResidency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GLOBAL" => Ok(Self::Global),
            "US_STORAGE_PROCESSING" => Ok(Self::UsStorageProcessing),
            "EU_STORAGE_PROCESSING" => Ok(Self::EuStorageProcessing),
            "JP_STORAGE" => Ok(Self::JpStorage),
            "KR_STORAGE" => Ok(Self::KrStorage),
            "CA_STORAGE" => Ok(Self::CaStorage),
            "SG_STORAGE" => Ok(Self::SgStorage),
            "IN_STORAGE" => Ok(Self::InStorage),
            "AU_STORAGE" => Ok(Self::AuStorage),
            "GB_STORAGE" => Ok(Self::GbStorage),
            "AE_STORAGE" => Ok(Self::AeStorage),
            "AE_STORAGE_PROCESSING" => Ok(Self::AeStorageProcessing),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PublicProjectResidency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicProjectResidency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveProjectApiKeyParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectApiKeyParams {
    pub api_key_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl RetrieveProjectApiKeyParams {
    pub fn builder() -> builder::RetrieveProjectApiKeyParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectModelPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectModelPermissionsParams {
    pub project_id: ::std::string::String,
}
impl RetrieveProjectModelPermissionsParams {
    pub fn builder() -> builder::RetrieveProjectModelPermissionsParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectParams {
    pub project_id: ::std::string::String,
}
impl RetrieveProjectParams {
    pub fn builder() -> builder::RetrieveProjectParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectServiceAccountParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectServiceAccountParams {
    pub project_id: ::std::string::String,
    pub service_account_id: ::std::string::String,
}
impl RetrieveProjectServiceAccountParams {
    pub fn builder() -> builder::RetrieveProjectServiceAccountParams {
        Default::default()
    }
}
#[doc = "`RetrieveProjectUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectUserParams {
    pub project_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl RetrieveProjectUserParams {
    pub fn builder() -> builder::RetrieveProjectUserParams {
        Default::default()
    }
}
#[doc = "`UpdateProjectApiKeyBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectApiKeyBody {
    #[doc = "Number of seconds from this request until the API key expires. Must comply with the organization and project expiry policies. Expired keys cannot be updated. Retrying resets the expiry relative to the new request."]
    pub expires_in_seconds: ::std::num::NonZeroU64,
}
impl UpdateProjectApiKeyBody {
    pub fn builder() -> builder::UpdateProjectApiKeyBody {
        Default::default()
    }
}
#[doc = "`UpdateProjectApiKeyParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectApiKeyParams {
    pub api_key_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl UpdateProjectApiKeyParams {
    pub fn builder() -> builder::UpdateProjectApiKeyParams {
        Default::default()
    }
}
#[doc = "`UpdateProjectModelPermissionsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectModelPermissionsParams {
    pub project_id: ::std::string::String,
}
impl UpdateProjectModelPermissionsParams {
    pub fn builder() -> builder::UpdateProjectModelPermissionsParams {
        Default::default()
    }
}
#[doc = "`UpdateProjectRateLimitsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectRateLimitsParams {
    pub project_id: ::std::string::String,
    pub rate_limit_id: ::std::string::String,
}
impl UpdateProjectRateLimitsParams {
    pub fn builder() -> builder::UpdateProjectRateLimitsParams {
        Default::default()
    }
}
#[doc = "Parameters for updating a project service account."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateProjectServiceAccountBody {
    #[doc = "The updated service account name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The updated service account role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<UpdateProjectServiceAccountBodyRole>,
}
impl UpdateProjectServiceAccountBody {
    pub fn builder() -> builder::UpdateProjectServiceAccountBody {
        Default::default()
    }
}
#[doc = "The updated service account role."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateProjectServiceAccountBodyRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for UpdateProjectServiceAccountBodyRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for UpdateProjectServiceAccountBodyRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateProjectServiceAccountBodyRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateProjectServiceAccountBodyRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateProjectServiceAccountParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectServiceAccountParams {
    pub project_id: ::std::string::String,
    pub service_account_id: ::std::string::String,
}
impl UpdateProjectServiceAccountParams {
    pub fn builder() -> builder::UpdateProjectServiceAccountParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ArchiveProjectParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ArchiveProjectParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ArchiveProjectParams {
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
    impl ::std::convert::TryFrom<ArchiveProjectParams> for super::ArchiveProjectParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ArchiveProjectParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ArchiveProjectParams> for ArchiveProjectParams {
        fn from(value: super::ArchiveProjectParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateProjectServiceAccountParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateProjectServiceAccountParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl CreateProjectServiceAccountParams {
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
    impl ::std::convert::TryFrom<CreateProjectServiceAccountParams>
        for super::CreateProjectServiceAccountParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateProjectServiceAccountParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateProjectServiceAccountParams>
        for CreateProjectServiceAccountParams
    {
        fn from(value: super::CreateProjectServiceAccountParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateProjectUserParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateProjectUserParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl CreateProjectUserParams {
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
    impl ::std::convert::TryFrom<CreateProjectUserParams> for super::CreateProjectUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateProjectUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateProjectUserParams> for CreateProjectUserParams {
        fn from(value: super::CreateProjectUserParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectApiKeyParams {
        api_key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectApiKeyParams {
        fn default() -> Self {
            Self {
                api_key_id: Err("no value supplied for api_key_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl DeleteProjectApiKeyParams {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
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
    impl ::std::convert::TryFrom<DeleteProjectApiKeyParams> for super::DeleteProjectApiKeyParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectApiKeyParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectApiKeyParams> for DeleteProjectApiKeyParams {
        fn from(value: super::DeleteProjectApiKeyParams) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectModelPermissionsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectModelPermissionsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl DeleteProjectModelPermissionsParams {
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
    impl ::std::convert::TryFrom<DeleteProjectModelPermissionsParams>
        for super::DeleteProjectModelPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectModelPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectModelPermissionsParams>
        for DeleteProjectModelPermissionsParams
    {
        fn from(value: super::DeleteProjectModelPermissionsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectServiceAccountParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        service_account_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectServiceAccountParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                service_account_id: Err("no value supplied for service_account_id".to_string()),
            }
        }
    }
    impl DeleteProjectServiceAccountParams {
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
        pub fn service_account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteProjectServiceAccountParams>
        for super::DeleteProjectServiceAccountParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectServiceAccountParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                service_account_id: value.service_account_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectServiceAccountParams>
        for DeleteProjectServiceAccountParams
    {
        fn from(value: super::DeleteProjectServiceAccountParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                service_account_id: Ok(value.service_account_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteProjectUserParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteProjectUserParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl DeleteProjectUserParams {
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
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteProjectUserParams> for super::DeleteProjectUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteProjectUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteProjectUserParams> for DeleteProjectUserParams {
        fn from(value: super::DeleteProjectUserParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectApiKeysParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        owner_project_access: ::std::result::Result<
            ::std::option::Option<super::ListProjectApiKeysParamsOwnerProjectAccess>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectApiKeysParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                owner_project_access: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectApiKeysParams {
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
        pub fn owner_project_access<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListProjectApiKeysParamsOwnerProjectAccess>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.owner_project_access = value.try_into().map_err(|e| {
                format!("error converting supplied value for owner_project_access: {e}")
            });
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
    impl ::std::convert::TryFrom<ListProjectApiKeysParams> for super::ListProjectApiKeysParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectApiKeysParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                owner_project_access: value.owner_project_access?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectApiKeysParams> for ListProjectApiKeysParams {
        fn from(value: super::ListProjectApiKeysParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                owner_project_access: Ok(value.owner_project_access),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectRateLimitsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectRateLimitsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                limit: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectRateLimitsParams {
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
    impl ::std::convert::TryFrom<ListProjectRateLimitsParams> for super::ListProjectRateLimitsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectRateLimitsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                before: value.before?,
                limit: value.limit?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectRateLimitsParams> for ListProjectRateLimitsParams {
        fn from(value: super::ListProjectRateLimitsParams) -> Self {
            Self {
                after: Ok(value.after),
                before: Ok(value.before),
                limit: Ok(value.limit),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectServiceAccountsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectServiceAccountsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectServiceAccountsParams {
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
    impl ::std::convert::TryFrom<ListProjectServiceAccountsParams>
        for super::ListProjectServiceAccountsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectServiceAccountsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectServiceAccountsParams>
        for ListProjectServiceAccountsParams
    {
        fn from(value: super::ListProjectServiceAccountsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectUsersParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectUsersParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectUsersParams {
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
    impl ::std::convert::TryFrom<ListProjectUsersParams> for super::ListProjectUsersParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectUsersParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectUsersParams> for ListProjectUsersParams {
        fn from(value: super::ListProjectUsersParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        include_archived: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                include_archived: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListProjectsParams {
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
        pub fn include_archived<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.include_archived = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include_archived: {e}"));
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
    impl ::std::convert::TryFrom<ListProjectsParams> for super::ListProjectsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                include_archived: value.include_archived?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectsParams> for ListProjectsParams {
        fn from(value: super::ListProjectsParams) -> Self {
            Self {
                after: Ok(value.after),
                include_archived: Ok(value.include_archived),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyProjectParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyProjectParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ModifyProjectParams {
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
    impl ::std::convert::TryFrom<ModifyProjectParams> for super::ModifyProjectParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyProjectParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyProjectParams> for ModifyProjectParams {
        fn from(value: super::ModifyProjectParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyProjectUserParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyProjectUserParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl ModifyProjectUserParams {
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
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ModifyProjectUserParams> for super::ModifyProjectUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyProjectUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyProjectUserParams> for ModifyProjectUserParams {
        fn from(value: super::ModifyProjectUserParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Project {
        archived_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        external_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ProjectObject, ::std::string::String>,
        residency: ::std::result::Result<
            ::std::option::Option<super::PublicProjectResidency>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Project {
        fn default() -> Self {
            Self {
                archived_at: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                external_key_id: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                residency: Ok(Default::default()),
                status: Ok(Default::default()),
            }
        }
    }
    impl Project {
        pub fn archived_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.archived_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for archived_at: {e}"));
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
        pub fn external_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_key_id: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn residency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PublicProjectResidency>>,
            T::Error: ::std::fmt::Display,
        {
            self.residency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for residency: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Project> for super::Project {
        type Error = super::error::ConversionError;
        fn try_from(value: Project) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                archived_at: value.archived_at?,
                created_at: value.created_at?,
                external_key_id: value.external_key_id?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                residency: value.residency?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::Project> for Project {
        fn from(value: super::Project) -> Self {
            Self {
                archived_at: Ok(value.archived_at),
                created_at: Ok(value.created_at),
                external_key_id: Ok(value.external_key_id),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                residency: Ok(value.residency),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectApiKey {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_used_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ProjectApiKeyObject, ::std::string::String>,
        owner: ::std::result::Result<super::ProjectApiKeyOwner, ::std::string::String>,
        owner_project_access:
            ::std::result::Result<super::ProjectApiKeyOwnerProjectAccess, ::std::string::String>,
        redacted_value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectApiKey {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                last_used_at: Err("no value supplied for last_used_at".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                owner: Err("no value supplied for owner".to_string()),
                owner_project_access: Err("no value supplied for owner_project_access".to_string()),
                redacted_value: Err("no value supplied for redacted_value".to_string()),
            }
        }
    }
    impl ProjectApiKey {
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn last_used_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_used_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_used_at: {e}"));
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
            T: ::std::convert::TryInto<super::ProjectApiKeyObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKeyOwner>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn owner_project_access<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKeyOwnerProjectAccess>,
            T::Error: ::std::fmt::Display,
        {
            self.owner_project_access = value.try_into().map_err(|e| {
                format!("error converting supplied value for owner_project_access: {e}")
            });
            self
        }
        pub fn redacted_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.redacted_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for redacted_value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectApiKey> for super::ProjectApiKey {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKey,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                id: value.id?,
                last_used_at: value.last_used_at?,
                name: value.name?,
                object: value.object?,
                owner: value.owner?,
                owner_project_access: value.owner_project_access?,
                redacted_value: value.redacted_value?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectApiKey> for ProjectApiKey {
        fn from(value: super::ProjectApiKey) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                last_used_at: Ok(value.last_used_at),
                name: Ok(value.name),
                object: Ok(value.object),
                owner: Ok(value.owner),
                owner_project_access: Ok(value.owner_project_access),
                redacted_value: Ok(value.redacted_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectApiKeyDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::ProjectApiKeyDeleteResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectApiKeyDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectApiKeyDeleteResponse {
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
            T: ::std::convert::TryInto<super::ProjectApiKeyDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectApiKeyDeleteResponse> for super::ProjectApiKeyDeleteResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKeyDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectApiKeyDeleteResponse> for ProjectApiKeyDeleteResponse {
        fn from(value: super::ProjectApiKeyDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectApiKeyListResponse {
        data: ::std::result::Result<::std::vec::Vec<super::ProjectApiKey>, ::std::string::String>,
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
            ::std::result::Result<super::ProjectApiKeyListResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectApiKeyListResponse {
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
    impl ProjectApiKeyListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectApiKey>>,
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
            T: ::std::convert::TryInto<super::ProjectApiKeyListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectApiKeyListResponse> for super::ProjectApiKeyListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKeyListResponse,
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
    impl ::std::convert::From<super::ProjectApiKeyListResponse> for ProjectApiKeyListResponse {
        fn from(value: super::ProjectApiKeyListResponse) -> Self {
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
    pub struct ProjectApiKeyOwner {
        service_account: ::std::result::Result<
            ::std::option::Option<super::ProjectApiKeyOwnerServiceAccount>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::ProjectApiKeyOwnerType>,
            ::std::string::String,
        >,
        user: ::std::result::Result<
            ::std::option::Option<super::ProjectApiKeyOwnerUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectApiKeyOwner {
        fn default() -> Self {
            Self {
                service_account: Ok(Default::default()),
                type_: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl ProjectApiKeyOwner {
        pub fn service_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ProjectApiKeyOwnerServiceAccount>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.service_account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_account: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProjectApiKeyOwnerType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProjectApiKeyOwnerUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectApiKeyOwner> for super::ProjectApiKeyOwner {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKeyOwner,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                service_account: value.service_account?,
                type_: value.type_?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectApiKeyOwner> for ProjectApiKeyOwner {
        fn from(value: super::ProjectApiKeyOwner) -> Self {
            Self {
                service_account: Ok(value.service_account),
                type_: Ok(value.type_),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectApiKeyOwnerServiceAccount {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectApiKeyOwnerServiceAccount {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ProjectApiKeyOwnerServiceAccount {
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
    impl ::std::convert::TryFrom<ProjectApiKeyOwnerServiceAccount>
        for super::ProjectApiKeyOwnerServiceAccount
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKeyOwnerServiceAccount,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectApiKeyOwnerServiceAccount>
        for ProjectApiKeyOwnerServiceAccount
    {
        fn from(value: super::ProjectApiKeyOwnerServiceAccount) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectApiKeyOwnerUser {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        email: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectApiKeyOwnerUser {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                email: Err("no value supplied for email".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ProjectApiKeyOwnerUser {
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
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {e}"));
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
    impl ::std::convert::TryFrom<ProjectApiKeyOwnerUser> for super::ProjectApiKeyOwnerUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectApiKeyOwnerUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                email: value.email?,
                id: value.id?,
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectApiKeyOwnerUser> for ProjectApiKeyOwnerUser {
        fn from(value: super::ProjectApiKeyOwnerUser) -> Self {
            Self {
                created_at: Ok(value.created_at),
                email: Ok(value.email),
                id: Ok(value.id),
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectCreateRequest {
        external_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        geography: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        residency: ::std::result::Result<
            ::std::option::Option<super::PublicProjectResidency>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectCreateRequest {
        fn default() -> Self {
            Self {
                external_key_id: Ok(Default::default()),
                geography: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                residency: Ok(Default::default()),
            }
        }
    }
    impl ProjectCreateRequest {
        pub fn external_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_key_id: {e}"));
            self
        }
        pub fn geography<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geography = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geography: {e}"));
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
        pub fn residency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PublicProjectResidency>>,
            T::Error: ::std::fmt::Display,
        {
            self.residency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for residency: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectCreateRequest> for super::ProjectCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                external_key_id: value.external_key_id?,
                geography: value.geography?,
                name: value.name?,
                residency: value.residency?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectCreateRequest> for ProjectCreateRequest {
        fn from(value: super::ProjectCreateRequest) -> Self {
            Self {
                external_key_id: Ok(value.external_key_id),
                geography: Ok(value.geography),
                name: Ok(value.name),
                residency: Ok(value.residency),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectListResponse {
        data: ::std::result::Result<::std::vec::Vec<super::Project>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ProjectListResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectListResponse {
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
    impl ProjectListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Project>>,
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
            T: ::std::convert::TryInto<super::ProjectListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectListResponse> for super::ProjectListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectListResponse,
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
    impl ::std::convert::From<super::ProjectListResponse> for ProjectListResponse {
        fn from(value: super::ProjectListResponse) -> Self {
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
    pub struct ProjectModelPermissions {
        mode: ::std::result::Result<super::ProjectModelPermissionsMode, ::std::string::String>,
        model_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        object: ::std::result::Result<super::ProjectModelPermissionsObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectModelPermissions {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
                model_ids: Err("no value supplied for model_ids".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectModelPermissions {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn model_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_ids: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectModelPermissions> for super::ProjectModelPermissions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectModelPermissions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                model_ids: value.model_ids?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectModelPermissions> for ProjectModelPermissions {
        fn from(value: super::ProjectModelPermissions) -> Self {
            Self {
                mode: Ok(value.mode),
                model_ids: Ok(value.model_ids),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectModelPermissionsDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<
            super::ProjectModelPermissionsDeleteResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectModelPermissionsDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectModelPermissionsDeleteResponse {
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectModelPermissionsDeleteResponse>
        for super::ProjectModelPermissionsDeleteResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectModelPermissionsDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectModelPermissionsDeleteResponse>
        for ProjectModelPermissionsDeleteResponse
    {
        fn from(value: super::ProjectModelPermissionsDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectModelPermissionsUpdateRequest {
        mode: ::std::result::Result<
            super::ProjectModelPermissionsUpdateRequestMode,
            ::std::string::String,
        >,
        model_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectModelPermissionsUpdateRequest {
        fn default() -> Self {
            Self {
                mode: Err("no value supplied for mode".to_string()),
                model_ids: Err("no value supplied for model_ids".to_string()),
            }
        }
    }
    impl ProjectModelPermissionsUpdateRequest {
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsUpdateRequestMode>,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {e}"));
            self
        }
        pub fn model_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.model_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectModelPermissionsUpdateRequest>
        for super::ProjectModelPermissionsUpdateRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectModelPermissionsUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mode: value.mode?,
                model_ids: value.model_ids?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectModelPermissionsUpdateRequest>
        for ProjectModelPermissionsUpdateRequest
    {
        fn from(value: super::ProjectModelPermissionsUpdateRequest) -> Self {
            Self {
                mode: Ok(value.mode),
                model_ids: Ok(value.model_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectRateLimit {
        batch_1_day_max_input_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        max_audio_megabytes_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_images_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_requests_per_1_day:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_requests_per_1_minute: ::std::result::Result<i64, ::std::string::String>,
        max_tokens_per_1_minute: ::std::result::Result<i64, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ProjectRateLimitObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectRateLimit {
        fn default() -> Self {
            Self {
                batch_1_day_max_input_tokens: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                max_audio_megabytes_per_1_minute: Ok(Default::default()),
                max_images_per_1_minute: Ok(Default::default()),
                max_requests_per_1_day: Ok(Default::default()),
                max_requests_per_1_minute: Err(
                    "no value supplied for max_requests_per_1_minute".to_string()
                ),
                max_tokens_per_1_minute: Err(
                    "no value supplied for max_tokens_per_1_minute".to_string()
                ),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectRateLimit {
        pub fn batch_1_day_max_input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_1_day_max_input_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for batch_1_day_max_input_tokens: {e}")
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
        pub fn max_audio_megabytes_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_audio_megabytes_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_audio_megabytes_per_1_minute: {e}")
            });
            self
        }
        pub fn max_images_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_images_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_images_per_1_minute: {e}")
            });
            self
        }
        pub fn max_requests_per_1_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_day = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_day: {e}")
            });
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
        pub fn max_tokens_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tokens_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_tokens_per_1_minute: {e}")
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectRateLimitObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectRateLimit> for super::ProjectRateLimit {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectRateLimit,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_1_day_max_input_tokens: value.batch_1_day_max_input_tokens?,
                id: value.id?,
                max_audio_megabytes_per_1_minute: value.max_audio_megabytes_per_1_minute?,
                max_images_per_1_minute: value.max_images_per_1_minute?,
                max_requests_per_1_day: value.max_requests_per_1_day?,
                max_requests_per_1_minute: value.max_requests_per_1_minute?,
                max_tokens_per_1_minute: value.max_tokens_per_1_minute?,
                model: value.model?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectRateLimit> for ProjectRateLimit {
        fn from(value: super::ProjectRateLimit) -> Self {
            Self {
                batch_1_day_max_input_tokens: Ok(value.batch_1_day_max_input_tokens),
                id: Ok(value.id),
                max_audio_megabytes_per_1_minute: Ok(value.max_audio_megabytes_per_1_minute),
                max_images_per_1_minute: Ok(value.max_images_per_1_minute),
                max_requests_per_1_day: Ok(value.max_requests_per_1_day),
                max_requests_per_1_minute: Ok(value.max_requests_per_1_minute),
                max_tokens_per_1_minute: Ok(value.max_tokens_per_1_minute),
                model: Ok(value.model),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectRateLimitListResponse {
        data:
            ::std::result::Result<::std::vec::Vec<super::ProjectRateLimit>, ::std::string::String>,
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
            ::std::result::Result<super::ProjectRateLimitListResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectRateLimitListResponse {
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
    impl ProjectRateLimitListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectRateLimit>>,
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
            T: ::std::convert::TryInto<super::ProjectRateLimitListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectRateLimitListResponse> for super::ProjectRateLimitListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectRateLimitListResponse,
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
    impl ::std::convert::From<super::ProjectRateLimitListResponse> for ProjectRateLimitListResponse {
        fn from(value: super::ProjectRateLimitListResponse) -> Self {
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
    pub struct ProjectRateLimitUpdateRequest {
        batch_1_day_max_input_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_audio_megabytes_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_images_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_requests_per_1_day:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_requests_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        max_tokens_per_1_minute:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectRateLimitUpdateRequest {
        fn default() -> Self {
            Self {
                batch_1_day_max_input_tokens: Ok(Default::default()),
                max_audio_megabytes_per_1_minute: Ok(Default::default()),
                max_images_per_1_minute: Ok(Default::default()),
                max_requests_per_1_day: Ok(Default::default()),
                max_requests_per_1_minute: Ok(Default::default()),
                max_tokens_per_1_minute: Ok(Default::default()),
            }
        }
    }
    impl ProjectRateLimitUpdateRequest {
        pub fn batch_1_day_max_input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch_1_day_max_input_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for batch_1_day_max_input_tokens: {e}")
            });
            self
        }
        pub fn max_audio_megabytes_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_audio_megabytes_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_audio_megabytes_per_1_minute: {e}")
            });
            self
        }
        pub fn max_images_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_images_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_images_per_1_minute: {e}")
            });
            self
        }
        pub fn max_requests_per_1_day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_day = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_day: {e}")
            });
            self
        }
        pub fn max_requests_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_requests_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_requests_per_1_minute: {e}")
            });
            self
        }
        pub fn max_tokens_per_1_minute<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tokens_per_1_minute = value.try_into().map_err(|e| {
                format!("error converting supplied value for max_tokens_per_1_minute: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectRateLimitUpdateRequest>
        for super::ProjectRateLimitUpdateRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectRateLimitUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                batch_1_day_max_input_tokens: value.batch_1_day_max_input_tokens?,
                max_audio_megabytes_per_1_minute: value.max_audio_megabytes_per_1_minute?,
                max_images_per_1_minute: value.max_images_per_1_minute?,
                max_requests_per_1_day: value.max_requests_per_1_day?,
                max_requests_per_1_minute: value.max_requests_per_1_minute?,
                max_tokens_per_1_minute: value.max_tokens_per_1_minute?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectRateLimitUpdateRequest> for ProjectRateLimitUpdateRequest {
        fn from(value: super::ProjectRateLimitUpdateRequest) -> Self {
            Self {
                batch_1_day_max_input_tokens: Ok(value.batch_1_day_max_input_tokens),
                max_audio_megabytes_per_1_minute: Ok(value.max_audio_megabytes_per_1_minute),
                max_images_per_1_minute: Ok(value.max_images_per_1_minute),
                max_requests_per_1_day: Ok(value.max_requests_per_1_day),
                max_requests_per_1_minute: Ok(value.max_requests_per_1_minute),
                max_tokens_per_1_minute: Ok(value.max_tokens_per_1_minute),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccount {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ProjectServiceAccountObject, ::std::string::String>,
        role: ::std::result::Result<super::ProjectServiceAccountRole, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectServiceAccount {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ProjectServiceAccount {
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
            T: ::std::convert::TryInto<super::ProjectServiceAccountObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectServiceAccount> for super::ProjectServiceAccount {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccount,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectServiceAccount> for ProjectServiceAccount {
        fn from(value: super::ProjectServiceAccount) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccountApiKey {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::ProjectServiceAccountApiKeyObject, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectServiceAccountApiKey {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ProjectServiceAccountApiKey {
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
            T: ::std::convert::TryInto<super::ProjectServiceAccountApiKeyObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectServiceAccountApiKey> for super::ProjectServiceAccountApiKey {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccountApiKey,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectServiceAccountApiKey> for ProjectServiceAccountApiKey {
        fn from(value: super::ProjectServiceAccountApiKey) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccountCreateRequest {
        create_service_account_only:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        expires_in_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectServiceAccountCreateRequest {
        fn default() -> Self {
            Self {
                create_service_account_only: Ok(Default::default()),
                expires_in_seconds: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ProjectServiceAccountCreateRequest {
        pub fn create_service_account_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.create_service_account_only = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_service_account_only: {e}")
            });
            self
        }
        pub fn expires_in_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_in_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for expires_in_seconds: {e}")
            });
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
    impl ::std::convert::TryFrom<ProjectServiceAccountCreateRequest>
        for super::ProjectServiceAccountCreateRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccountCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_service_account_only: value.create_service_account_only?,
                expires_in_seconds: value.expires_in_seconds?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectServiceAccountCreateRequest>
        for ProjectServiceAccountCreateRequest
    {
        fn from(value: super::ProjectServiceAccountCreateRequest) -> Self {
            Self {
                create_service_account_only: Ok(value.create_service_account_only),
                expires_in_seconds: Ok(value.expires_in_seconds),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccountCreateResponse {
        api_key: ::std::result::Result<
            ::std::option::Option<super::ProjectServiceAccountApiKey>,
            ::std::string::String,
        >,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::ProjectServiceAccountCreateResponseObject,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            super::ProjectServiceAccountCreateResponseRole,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectServiceAccountCreateResponse {
        fn default() -> Self {
            Self {
                api_key: Err("no value supplied for api_key".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ProjectServiceAccountCreateResponse {
        pub fn api_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ProjectServiceAccountApiKey>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key: {e}"));
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
            T: ::std::convert::TryInto<super::ProjectServiceAccountCreateResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountCreateResponseRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectServiceAccountCreateResponse>
        for super::ProjectServiceAccountCreateResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccountCreateResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key: value.api_key?,
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectServiceAccountCreateResponse>
        for ProjectServiceAccountCreateResponse
    {
        fn from(value: super::ProjectServiceAccountCreateResponse) -> Self {
            Self {
                api_key: Ok(value.api_key),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccountDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::ProjectServiceAccountDeleteResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectServiceAccountDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectServiceAccountDeleteResponse {
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
            T: ::std::convert::TryInto<super::ProjectServiceAccountDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectServiceAccountDeleteResponse>
        for super::ProjectServiceAccountDeleteResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccountDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectServiceAccountDeleteResponse>
        for ProjectServiceAccountDeleteResponse
    {
        fn from(value: super::ProjectServiceAccountDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectServiceAccountListResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<super::ProjectServiceAccount>,
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
            super::ProjectServiceAccountListResponseObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectServiceAccountListResponse {
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
    impl ProjectServiceAccountListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectServiceAccount>>,
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
            T: ::std::convert::TryInto<super::ProjectServiceAccountListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectServiceAccountListResponse>
        for super::ProjectServiceAccountListResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectServiceAccountListResponse,
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
    impl ::std::convert::From<super::ProjectServiceAccountListResponse>
        for ProjectServiceAccountListResponse
    {
        fn from(value: super::ProjectServiceAccountListResponse) -> Self {
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
    pub struct ProjectUpdateRequest {
        external_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        geography: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectUpdateRequest {
        fn default() -> Self {
            Self {
                external_key_id: Ok(Default::default()),
                geography: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl ProjectUpdateRequest {
        pub fn external_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_key_id: {e}"));
            self
        }
        pub fn geography<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.geography = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geography: {e}"));
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
    impl ::std::convert::TryFrom<ProjectUpdateRequest> for super::ProjectUpdateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                external_key_id: value.external_key_id?,
                geography: value.geography?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectUpdateRequest> for ProjectUpdateRequest {
        fn from(value: super::ProjectUpdateRequest) -> Self {
            Self {
                external_key_id: Ok(value.external_key_id),
                geography: Ok(value.geography),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectUser {
        added_at: ::std::result::Result<i64, ::std::string::String>,
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ProjectUserObject, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectUser {
        fn default() -> Self {
            Self {
                added_at: Err("no value supplied for added_at".to_string()),
                email: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ProjectUser {
        pub fn added_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.added_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for added_at: {e}"));
            self
        }
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUserObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    impl ::std::convert::TryFrom<ProjectUser> for super::ProjectUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                added_at: value.added_at?,
                email: value.email?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectUser> for ProjectUser {
        fn from(value: super::ProjectUser) -> Self {
            Self {
                added_at: Ok(value.added_at),
                email: Ok(value.email),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectUserCreateRequest {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectUserCreateRequest {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl ProjectUserCreateRequest {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {e}"));
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
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectUserCreateRequest> for super::ProjectUserCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUserCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                role: value.role?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectUserCreateRequest> for ProjectUserCreateRequest {
        fn from(value: super::ProjectUserCreateRequest) -> Self {
            Self {
                email: Ok(value.email),
                role: Ok(value.role),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectUserDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::ProjectUserDeleteResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectUserDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectUserDeleteResponse {
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
            T: ::std::convert::TryInto<super::ProjectUserDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectUserDeleteResponse> for super::ProjectUserDeleteResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUserDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectUserDeleteResponse> for ProjectUserDeleteResponse {
        fn from(value: super::ProjectUserDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectUserListResponse {
        data: ::std::result::Result<::std::vec::Vec<super::ProjectUser>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectUserListResponse {
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
    impl ProjectUserListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectUser>>,
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectUserListResponse> for super::ProjectUserListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUserListResponse,
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
    impl ::std::convert::From<super::ProjectUserListResponse> for ProjectUserListResponse {
        fn from(value: super::ProjectUserListResponse) -> Self {
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
    pub struct ProjectUserUpdateRequest {
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectUserUpdateRequest {
        fn default() -> Self {
            Self {
                role: Ok(Default::default()),
            }
        }
    }
    impl ProjectUserUpdateRequest {
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
    impl ::std::convert::TryFrom<ProjectUserUpdateRequest> for super::ProjectUserUpdateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUserUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { role: value.role? })
        }
    }
    impl ::std::convert::From<super::ProjectUserUpdateRequest> for ProjectUserUpdateRequest {
        fn from(value: super::ProjectUserUpdateRequest) -> Self {
            Self {
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectsApiSchema {
        archive_project_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        archive_project_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        archive_project_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        archive_project_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        archive_project_params:
            ::std::result::Result<super::ArchiveProjectParams, ::std::string::String>,
        archive_project_response: ::std::result::Result<super::Project, ::std::string::String>,
        create_project_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_project_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_request:
            ::std::result::Result<super::ProjectCreateRequest, ::std::string::String>,
        create_project_response: ::std::result::Result<super::Project, ::std::string::String>,
        create_project_service_account_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_service_account_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_project_service_account_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_service_account_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_service_account_params:
            ::std::result::Result<super::CreateProjectServiceAccountParams, ::std::string::String>,
        create_project_service_account_request:
            ::std::result::Result<super::ProjectServiceAccountCreateRequest, ::std::string::String>,
        create_project_service_account_response: ::std::result::Result<
            super::ProjectServiceAccountCreateResponse,
            ::std::string::String,
        >,
        create_project_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        create_project_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_user_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_project_user_params:
            ::std::result::Result<super::CreateProjectUserParams, ::std::string::String>,
        create_project_user_request:
            ::std::result::Result<super::ProjectUserCreateRequest, ::std::string::String>,
        create_project_user_response:
            ::std::result::Result<super::ProjectUser, ::std::string::String>,
        delete_project_api_key_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_api_key_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_api_key_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_api_key_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_api_key_params:
            ::std::result::Result<super::DeleteProjectApiKeyParams, ::std::string::String>,
        delete_project_api_key_response:
            ::std::result::Result<super::ProjectApiKeyDeleteResponse, ::std::string::String>,
        delete_project_model_permissions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_model_permissions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_model_permissions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_model_permissions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_model_permissions_params: ::std::result::Result<
            super::DeleteProjectModelPermissionsParams,
            ::std::string::String,
        >,
        delete_project_model_permissions_response: ::std::result::Result<
            super::ProjectModelPermissionsDeleteResponse,
            ::std::string::String,
        >,
        delete_project_service_account_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_service_account_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_service_account_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_service_account_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_service_account_params:
            ::std::result::Result<super::DeleteProjectServiceAccountParams, ::std::string::String>,
        delete_project_service_account_response: ::std::result::Result<
            super::ProjectServiceAccountDeleteResponse,
            ::std::string::String,
        >,
        delete_project_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_project_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_project_user_params:
            ::std::result::Result<super::DeleteProjectUserParams, ::std::string::String>,
        delete_project_user_response:
            ::std::result::Result<super::ProjectUserDeleteResponse, ::std::string::String>,
        list_project_api_keys_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_api_keys_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_api_keys_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_api_keys_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_api_keys_params:
            ::std::result::Result<super::ListProjectApiKeysParams, ::std::string::String>,
        list_project_api_keys_response:
            ::std::result::Result<super::ProjectApiKeyListResponse, ::std::string::String>,
        list_project_rate_limits_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_rate_limits_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_rate_limits_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_rate_limits_params:
            ::std::result::Result<super::ListProjectRateLimitsParams, ::std::string::String>,
        list_project_rate_limits_response:
            ::std::result::Result<super::ProjectRateLimitListResponse, ::std::string::String>,
        list_project_service_accounts_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_service_accounts_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_service_accounts_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_service_accounts_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_service_accounts_params:
            ::std::result::Result<super::ListProjectServiceAccountsParams, ::std::string::String>,
        list_project_service_accounts_response:
            ::std::result::Result<super::ProjectServiceAccountListResponse, ::std::string::String>,
        list_project_users_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_users_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_users_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_users_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_users_params:
            ::std::result::Result<super::ListProjectUsersParams, ::std::string::String>,
        list_project_users_response:
            ::std::result::Result<super::ProjectUserListResponse, ::std::string::String>,
        list_projects_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_projects_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_projects_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_projects_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_projects_params:
            ::std::result::Result<super::ListProjectsParams, ::std::string::String>,
        list_projects_response:
            ::std::result::Result<super::ProjectListResponse, ::std::string::String>,
        modify_project_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        modify_project_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_params:
            ::std::result::Result<super::ModifyProjectParams, ::std::string::String>,
        modify_project_request:
            ::std::result::Result<super::ProjectUpdateRequest, ::std::string::String>,
        modify_project_response: ::std::result::Result<super::Project, ::std::string::String>,
        modify_project_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        modify_project_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_project_user_params:
            ::std::result::Result<super::ModifyProjectUserParams, ::std::string::String>,
        modify_project_user_request:
            ::std::result::Result<super::ProjectUserUpdateRequest, ::std::string::String>,
        modify_project_user_response:
            ::std::result::Result<super::ProjectUser, ::std::string::String>,
        retrieve_project_api_key_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_api_key_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_api_key_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_api_key_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_api_key_params:
            ::std::result::Result<super::RetrieveProjectApiKeyParams, ::std::string::String>,
        retrieve_project_api_key_response:
            ::std::result::Result<super::ProjectApiKey, ::std::string::String>,
        retrieve_project_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_model_permissions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_model_permissions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_model_permissions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_model_permissions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_model_permissions_params: ::std::result::Result<
            super::RetrieveProjectModelPermissionsParams,
            ::std::string::String,
        >,
        retrieve_project_model_permissions_response:
            ::std::result::Result<super::ProjectModelPermissions, ::std::string::String>,
        retrieve_project_params:
            ::std::result::Result<super::RetrieveProjectParams, ::std::string::String>,
        retrieve_project_response: ::std::result::Result<super::Project, ::std::string::String>,
        retrieve_project_service_account_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_service_account_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_service_account_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_service_account_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_service_account_params: ::std::result::Result<
            super::RetrieveProjectServiceAccountParams,
            ::std::string::String,
        >,
        retrieve_project_service_account_response:
            ::std::result::Result<super::ProjectServiceAccount, ::std::string::String>,
        retrieve_project_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_params:
            ::std::result::Result<super::RetrieveProjectUserParams, ::std::string::String>,
        retrieve_project_user_response:
            ::std::result::Result<super::ProjectUser, ::std::string::String>,
        update_project_api_key_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_api_key_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_api_key_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_api_key_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_api_key_params:
            ::std::result::Result<super::UpdateProjectApiKeyParams, ::std::string::String>,
        update_project_api_key_request:
            ::std::result::Result<super::UpdateProjectApiKeyBody, ::std::string::String>,
        update_project_api_key_response:
            ::std::result::Result<super::ProjectApiKey, ::std::string::String>,
        update_project_model_permissions_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_model_permissions_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_model_permissions_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_model_permissions_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_model_permissions_params: ::std::result::Result<
            super::UpdateProjectModelPermissionsParams,
            ::std::string::String,
        >,
        update_project_model_permissions_request: ::std::result::Result<
            super::ProjectModelPermissionsUpdateRequest,
            ::std::string::String,
        >,
        update_project_model_permissions_response:
            ::std::result::Result<super::ProjectModelPermissions, ::std::string::String>,
        update_project_rate_limits_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_rate_limits_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_rate_limits_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_rate_limits_params:
            ::std::result::Result<super::UpdateProjectRateLimitsParams, ::std::string::String>,
        update_project_rate_limits_request:
            ::std::result::Result<super::ProjectRateLimitUpdateRequest, ::std::string::String>,
        update_project_rate_limits_response:
            ::std::result::Result<super::ProjectRateLimit, ::std::string::String>,
        update_project_service_account_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_service_account_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        update_project_service_account_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_service_account_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        update_project_service_account_params:
            ::std::result::Result<super::UpdateProjectServiceAccountParams, ::std::string::String>,
        update_project_service_account_request:
            ::std::result::Result<super::UpdateProjectServiceAccountBody, ::std::string::String>,
        update_project_service_account_response:
            ::std::result::Result<super::ProjectServiceAccount, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectsApiSchema {
        fn default() -> Self {
            Self {
                archive_project_error400: Err(
                    "no value supplied for archive_project_error400".to_string()
                ),
                archive_project_error403: Err(
                    "no value supplied for archive_project_error403".to_string()
                ),
                archive_project_error404: Err(
                    "no value supplied for archive_project_error404".to_string()
                ),
                archive_project_error429: Err(
                    "no value supplied for archive_project_error429".to_string()
                ),
                archive_project_params: Err(
                    "no value supplied for archive_project_params".to_string()
                ),
                archive_project_response: Err(
                    "no value supplied for archive_project_response".to_string()
                ),
                create_project_error400: Err(
                    "no value supplied for create_project_error400".to_string()
                ),
                create_project_error403: Err(
                    "no value supplied for create_project_error403".to_string()
                ),
                create_project_error429: Err(
                    "no value supplied for create_project_error429".to_string()
                ),
                create_project_request: Err(
                    "no value supplied for create_project_request".to_string()
                ),
                create_project_response: Err(
                    "no value supplied for create_project_response".to_string()
                ),
                create_project_service_account_error400: Err(
                    "no value supplied for create_project_service_account_error400".to_string(),
                ),
                create_project_service_account_error403: Err(
                    "no value supplied for create_project_service_account_error403".to_string(),
                ),
                create_project_service_account_error404: Err(
                    "no value supplied for create_project_service_account_error404".to_string(),
                ),
                create_project_service_account_error429: Err(
                    "no value supplied for create_project_service_account_error429".to_string(),
                ),
                create_project_service_account_params: Err(
                    "no value supplied for create_project_service_account_params".to_string(),
                ),
                create_project_service_account_request: Err(
                    "no value supplied for create_project_service_account_request".to_string(),
                ),
                create_project_service_account_response: Err(
                    "no value supplied for create_project_service_account_response".to_string(),
                ),
                create_project_user_error400: Err(
                    "no value supplied for create_project_user_error400".to_string(),
                ),
                create_project_user_error403: Err(
                    "no value supplied for create_project_user_error403".to_string(),
                ),
                create_project_user_error404: Err(
                    "no value supplied for create_project_user_error404".to_string(),
                ),
                create_project_user_error409: Err(
                    "no value supplied for create_project_user_error409".to_string(),
                ),
                create_project_user_error429: Err(
                    "no value supplied for create_project_user_error429".to_string(),
                ),
                create_project_user_params: Err(
                    "no value supplied for create_project_user_params".to_string()
                ),
                create_project_user_request: Err(
                    "no value supplied for create_project_user_request".to_string(),
                ),
                create_project_user_response: Err(
                    "no value supplied for create_project_user_response".to_string(),
                ),
                delete_project_api_key_error400: Err(
                    "no value supplied for delete_project_api_key_error400".to_string(),
                ),
                delete_project_api_key_error403: Err(
                    "no value supplied for delete_project_api_key_error403".to_string(),
                ),
                delete_project_api_key_error404: Err(
                    "no value supplied for delete_project_api_key_error404".to_string(),
                ),
                delete_project_api_key_error429: Err(
                    "no value supplied for delete_project_api_key_error429".to_string(),
                ),
                delete_project_api_key_params: Err(
                    "no value supplied for delete_project_api_key_params".to_string(),
                ),
                delete_project_api_key_response: Err(
                    "no value supplied for delete_project_api_key_response".to_string(),
                ),
                delete_project_model_permissions_error400: Err(
                    "no value supplied for delete_project_model_permissions_error400".to_string(),
                ),
                delete_project_model_permissions_error403: Err(
                    "no value supplied for delete_project_model_permissions_error403".to_string(),
                ),
                delete_project_model_permissions_error404: Err(
                    "no value supplied for delete_project_model_permissions_error404".to_string(),
                ),
                delete_project_model_permissions_error429: Err(
                    "no value supplied for delete_project_model_permissions_error429".to_string(),
                ),
                delete_project_model_permissions_params: Err(
                    "no value supplied for delete_project_model_permissions_params".to_string(),
                ),
                delete_project_model_permissions_response: Err(
                    "no value supplied for delete_project_model_permissions_response".to_string(),
                ),
                delete_project_service_account_error400: Err(
                    "no value supplied for delete_project_service_account_error400".to_string(),
                ),
                delete_project_service_account_error403: Err(
                    "no value supplied for delete_project_service_account_error403".to_string(),
                ),
                delete_project_service_account_error404: Err(
                    "no value supplied for delete_project_service_account_error404".to_string(),
                ),
                delete_project_service_account_error429: Err(
                    "no value supplied for delete_project_service_account_error429".to_string(),
                ),
                delete_project_service_account_params: Err(
                    "no value supplied for delete_project_service_account_params".to_string(),
                ),
                delete_project_service_account_response: Err(
                    "no value supplied for delete_project_service_account_response".to_string(),
                ),
                delete_project_user_error400: Err(
                    "no value supplied for delete_project_user_error400".to_string(),
                ),
                delete_project_user_error403: Err(
                    "no value supplied for delete_project_user_error403".to_string(),
                ),
                delete_project_user_error404: Err(
                    "no value supplied for delete_project_user_error404".to_string(),
                ),
                delete_project_user_error429: Err(
                    "no value supplied for delete_project_user_error429".to_string(),
                ),
                delete_project_user_params: Err(
                    "no value supplied for delete_project_user_params".to_string()
                ),
                delete_project_user_response: Err(
                    "no value supplied for delete_project_user_response".to_string(),
                ),
                list_project_api_keys_error400: Err(
                    "no value supplied for list_project_api_keys_error400".to_string(),
                ),
                list_project_api_keys_error403: Err(
                    "no value supplied for list_project_api_keys_error403".to_string(),
                ),
                list_project_api_keys_error404: Err(
                    "no value supplied for list_project_api_keys_error404".to_string(),
                ),
                list_project_api_keys_error429: Err(
                    "no value supplied for list_project_api_keys_error429".to_string(),
                ),
                list_project_api_keys_params: Err(
                    "no value supplied for list_project_api_keys_params".to_string(),
                ),
                list_project_api_keys_response: Err(
                    "no value supplied for list_project_api_keys_response".to_string(),
                ),
                list_project_rate_limits_error400: Err(
                    "no value supplied for list_project_rate_limits_error400".to_string(),
                ),
                list_project_rate_limits_error403: Err(
                    "no value supplied for list_project_rate_limits_error403".to_string(),
                ),
                list_project_rate_limits_error404: Err(
                    "no value supplied for list_project_rate_limits_error404".to_string(),
                ),
                list_project_rate_limits_params: Err(
                    "no value supplied for list_project_rate_limits_params".to_string(),
                ),
                list_project_rate_limits_response: Err(
                    "no value supplied for list_project_rate_limits_response".to_string(),
                ),
                list_project_service_accounts_error400: Err(
                    "no value supplied for list_project_service_accounts_error400".to_string(),
                ),
                list_project_service_accounts_error403: Err(
                    "no value supplied for list_project_service_accounts_error403".to_string(),
                ),
                list_project_service_accounts_error404: Err(
                    "no value supplied for list_project_service_accounts_error404".to_string(),
                ),
                list_project_service_accounts_error429: Err(
                    "no value supplied for list_project_service_accounts_error429".to_string(),
                ),
                list_project_service_accounts_params: Err(
                    "no value supplied for list_project_service_accounts_params".to_string(),
                ),
                list_project_service_accounts_response: Err(
                    "no value supplied for list_project_service_accounts_response".to_string(),
                ),
                list_project_users_error400: Err(
                    "no value supplied for list_project_users_error400".to_string(),
                ),
                list_project_users_error403: Err(
                    "no value supplied for list_project_users_error403".to_string(),
                ),
                list_project_users_error404: Err(
                    "no value supplied for list_project_users_error404".to_string(),
                ),
                list_project_users_error429: Err(
                    "no value supplied for list_project_users_error429".to_string(),
                ),
                list_project_users_params: Err(
                    "no value supplied for list_project_users_params".to_string()
                ),
                list_project_users_response: Err(
                    "no value supplied for list_project_users_response".to_string(),
                ),
                list_projects_error400: Err(
                    "no value supplied for list_projects_error400".to_string()
                ),
                list_projects_error403: Err(
                    "no value supplied for list_projects_error403".to_string()
                ),
                list_projects_error404: Err(
                    "no value supplied for list_projects_error404".to_string()
                ),
                list_projects_error429: Err(
                    "no value supplied for list_projects_error429".to_string()
                ),
                list_projects_params: Err("no value supplied for list_projects_params".to_string()),
                list_projects_response: Err(
                    "no value supplied for list_projects_response".to_string()
                ),
                modify_project_error400: Err(
                    "no value supplied for modify_project_error400".to_string()
                ),
                modify_project_error403: Err(
                    "no value supplied for modify_project_error403".to_string()
                ),
                modify_project_error404: Err(
                    "no value supplied for modify_project_error404".to_string()
                ),
                modify_project_error429: Err(
                    "no value supplied for modify_project_error429".to_string()
                ),
                modify_project_params: Err(
                    "no value supplied for modify_project_params".to_string()
                ),
                modify_project_request: Err(
                    "no value supplied for modify_project_request".to_string()
                ),
                modify_project_response: Err(
                    "no value supplied for modify_project_response".to_string()
                ),
                modify_project_user_error400: Err(
                    "no value supplied for modify_project_user_error400".to_string(),
                ),
                modify_project_user_error403: Err(
                    "no value supplied for modify_project_user_error403".to_string(),
                ),
                modify_project_user_error404: Err(
                    "no value supplied for modify_project_user_error404".to_string(),
                ),
                modify_project_user_error429: Err(
                    "no value supplied for modify_project_user_error429".to_string(),
                ),
                modify_project_user_params: Err(
                    "no value supplied for modify_project_user_params".to_string()
                ),
                modify_project_user_request: Err(
                    "no value supplied for modify_project_user_request".to_string(),
                ),
                modify_project_user_response: Err(
                    "no value supplied for modify_project_user_response".to_string(),
                ),
                retrieve_project_api_key_error400: Err(
                    "no value supplied for retrieve_project_api_key_error400".to_string(),
                ),
                retrieve_project_api_key_error403: Err(
                    "no value supplied for retrieve_project_api_key_error403".to_string(),
                ),
                retrieve_project_api_key_error404: Err(
                    "no value supplied for retrieve_project_api_key_error404".to_string(),
                ),
                retrieve_project_api_key_error429: Err(
                    "no value supplied for retrieve_project_api_key_error429".to_string(),
                ),
                retrieve_project_api_key_params: Err(
                    "no value supplied for retrieve_project_api_key_params".to_string(),
                ),
                retrieve_project_api_key_response: Err(
                    "no value supplied for retrieve_project_api_key_response".to_string(),
                ),
                retrieve_project_error403: Err(
                    "no value supplied for retrieve_project_error403".to_string()
                ),
                retrieve_project_error404: Err(
                    "no value supplied for retrieve_project_error404".to_string()
                ),
                retrieve_project_error429: Err(
                    "no value supplied for retrieve_project_error429".to_string()
                ),
                retrieve_project_model_permissions_error400: Err(
                    "no value supplied for retrieve_project_model_permissions_error400".to_string(),
                ),
                retrieve_project_model_permissions_error403: Err(
                    "no value supplied for retrieve_project_model_permissions_error403".to_string(),
                ),
                retrieve_project_model_permissions_error404: Err(
                    "no value supplied for retrieve_project_model_permissions_error404".to_string(),
                ),
                retrieve_project_model_permissions_error429: Err(
                    "no value supplied for retrieve_project_model_permissions_error429".to_string(),
                ),
                retrieve_project_model_permissions_params: Err(
                    "no value supplied for retrieve_project_model_permissions_params".to_string(),
                ),
                retrieve_project_model_permissions_response: Err(
                    "no value supplied for retrieve_project_model_permissions_response".to_string(),
                ),
                retrieve_project_params: Err(
                    "no value supplied for retrieve_project_params".to_string()
                ),
                retrieve_project_response: Err(
                    "no value supplied for retrieve_project_response".to_string()
                ),
                retrieve_project_service_account_error400: Err(
                    "no value supplied for retrieve_project_service_account_error400".to_string(),
                ),
                retrieve_project_service_account_error403: Err(
                    "no value supplied for retrieve_project_service_account_error403".to_string(),
                ),
                retrieve_project_service_account_error404: Err(
                    "no value supplied for retrieve_project_service_account_error404".to_string(),
                ),
                retrieve_project_service_account_error429: Err(
                    "no value supplied for retrieve_project_service_account_error429".to_string(),
                ),
                retrieve_project_service_account_params: Err(
                    "no value supplied for retrieve_project_service_account_params".to_string(),
                ),
                retrieve_project_service_account_response: Err(
                    "no value supplied for retrieve_project_service_account_response".to_string(),
                ),
                retrieve_project_user_error400: Err(
                    "no value supplied for retrieve_project_user_error400".to_string(),
                ),
                retrieve_project_user_error403: Err(
                    "no value supplied for retrieve_project_user_error403".to_string(),
                ),
                retrieve_project_user_error404: Err(
                    "no value supplied for retrieve_project_user_error404".to_string(),
                ),
                retrieve_project_user_error429: Err(
                    "no value supplied for retrieve_project_user_error429".to_string(),
                ),
                retrieve_project_user_params: Err(
                    "no value supplied for retrieve_project_user_params".to_string(),
                ),
                retrieve_project_user_response: Err(
                    "no value supplied for retrieve_project_user_response".to_string(),
                ),
                update_project_api_key_error400: Err(
                    "no value supplied for update_project_api_key_error400".to_string(),
                ),
                update_project_api_key_error403: Err(
                    "no value supplied for update_project_api_key_error403".to_string(),
                ),
                update_project_api_key_error404: Err(
                    "no value supplied for update_project_api_key_error404".to_string(),
                ),
                update_project_api_key_error429: Err(
                    "no value supplied for update_project_api_key_error429".to_string(),
                ),
                update_project_api_key_params: Err(
                    "no value supplied for update_project_api_key_params".to_string(),
                ),
                update_project_api_key_request: Err(
                    "no value supplied for update_project_api_key_request".to_string(),
                ),
                update_project_api_key_response: Err(
                    "no value supplied for update_project_api_key_response".to_string(),
                ),
                update_project_model_permissions_error400: Err(
                    "no value supplied for update_project_model_permissions_error400".to_string(),
                ),
                update_project_model_permissions_error403: Err(
                    "no value supplied for update_project_model_permissions_error403".to_string(),
                ),
                update_project_model_permissions_error404: Err(
                    "no value supplied for update_project_model_permissions_error404".to_string(),
                ),
                update_project_model_permissions_error429: Err(
                    "no value supplied for update_project_model_permissions_error429".to_string(),
                ),
                update_project_model_permissions_params: Err(
                    "no value supplied for update_project_model_permissions_params".to_string(),
                ),
                update_project_model_permissions_request: Err(
                    "no value supplied for update_project_model_permissions_request".to_string(),
                ),
                update_project_model_permissions_response: Err(
                    "no value supplied for update_project_model_permissions_response".to_string(),
                ),
                update_project_rate_limits_error400: Err(
                    "no value supplied for update_project_rate_limits_error400".to_string(),
                ),
                update_project_rate_limits_error403: Err(
                    "no value supplied for update_project_rate_limits_error403".to_string(),
                ),
                update_project_rate_limits_error404: Err(
                    "no value supplied for update_project_rate_limits_error404".to_string(),
                ),
                update_project_rate_limits_params: Err(
                    "no value supplied for update_project_rate_limits_params".to_string(),
                ),
                update_project_rate_limits_request: Err(
                    "no value supplied for update_project_rate_limits_request".to_string(),
                ),
                update_project_rate_limits_response: Err(
                    "no value supplied for update_project_rate_limits_response".to_string(),
                ),
                update_project_service_account_error400: Err(
                    "no value supplied for update_project_service_account_error400".to_string(),
                ),
                update_project_service_account_error403: Err(
                    "no value supplied for update_project_service_account_error403".to_string(),
                ),
                update_project_service_account_error404: Err(
                    "no value supplied for update_project_service_account_error404".to_string(),
                ),
                update_project_service_account_error429: Err(
                    "no value supplied for update_project_service_account_error429".to_string(),
                ),
                update_project_service_account_params: Err(
                    "no value supplied for update_project_service_account_params".to_string(),
                ),
                update_project_service_account_request: Err(
                    "no value supplied for update_project_service_account_request".to_string(),
                ),
                update_project_service_account_response: Err(
                    "no value supplied for update_project_service_account_response".to_string(),
                ),
            }
        }
    }
    impl ProjectsApiSchema {
        pub fn archive_project_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_error400: {e}")
            });
            self
        }
        pub fn archive_project_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_error403: {e}")
            });
            self
        }
        pub fn archive_project_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_error404: {e}")
            });
            self
        }
        pub fn archive_project_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_error429: {e}")
            });
            self
        }
        pub fn archive_project_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ArchiveProjectParams>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_params: {e}")
            });
            self
        }
        pub fn archive_project_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Project>,
            T::Error: ::std::fmt::Display,
        {
            self.archive_project_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for archive_project_response: {e}")
            });
            self
        }
        pub fn create_project_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_error400: {e}")
            });
            self
        }
        pub fn create_project_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_error403: {e}")
            });
            self
        }
        pub fn create_project_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_error429: {e}")
            });
            self
        }
        pub fn create_project_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectCreateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_request: {e}")
            });
            self
        }
        pub fn create_project_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Project>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_response: {e}")
            });
            self
        }
        pub fn create_project_service_account_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_error400: {e}")) ;
            self
        }
        pub fn create_project_service_account_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_error403: {e}")) ;
            self
        }
        pub fn create_project_service_account_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_error404: {e}")) ;
            self
        }
        pub fn create_project_service_account_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_error429: {e}")) ;
            self
        }
        pub fn create_project_service_account_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateProjectServiceAccountParams>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_params: {e}")) ;
            self
        }
        pub fn create_project_service_account_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountCreateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_request: {e}")) ;
            self
        }
        pub fn create_project_service_account_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountCreateResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . create_project_service_account_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_project_service_account_response: {e}")) ;
            self
        }
        pub fn create_project_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_error400: {e}")
            });
            self
        }
        pub fn create_project_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_error403: {e}")
            });
            self
        }
        pub fn create_project_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_error404: {e}")
            });
            self
        }
        pub fn create_project_user_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_error409: {e}")
            });
            self
        }
        pub fn create_project_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_error429: {e}")
            });
            self
        }
        pub fn create_project_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateProjectUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_params: {e}")
            });
            self
        }
        pub fn create_project_user_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUserCreateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_request: {e}")
            });
            self
        }
        pub fn create_project_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUser>,
            T::Error: ::std::fmt::Display,
        {
            self.create_project_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_project_user_response: {e}")
            });
            self
        }
        pub fn delete_project_api_key_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_error400: {e}")
            });
            self
        }
        pub fn delete_project_api_key_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_error403: {e}")
            });
            self
        }
        pub fn delete_project_api_key_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_error404: {e}")
            });
            self
        }
        pub fn delete_project_api_key_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_error429: {e}")
            });
            self
        }
        pub fn delete_project_api_key_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectApiKeyParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_params: {e}")
            });
            self
        }
        pub fn delete_project_api_key_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKeyDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_api_key_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_api_key_response: {e}")
            });
            self
        }
        pub fn delete_project_model_permissions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_error400: {e}")) ;
            self
        }
        pub fn delete_project_model_permissions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_error403: {e}")) ;
            self
        }
        pub fn delete_project_model_permissions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_error404: {e}")) ;
            self
        }
        pub fn delete_project_model_permissions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_error429: {e}")) ;
            self
        }
        pub fn delete_project_model_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectModelPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_params: {e}")) ;
            self
        }
        pub fn delete_project_model_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_model_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_model_permissions_response: {e}")) ;
            self
        }
        pub fn delete_project_service_account_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_error400: {e}")) ;
            self
        }
        pub fn delete_project_service_account_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_error403: {e}")) ;
            self
        }
        pub fn delete_project_service_account_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_error404: {e}")) ;
            self
        }
        pub fn delete_project_service_account_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_error429: {e}")) ;
            self
        }
        pub fn delete_project_service_account_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectServiceAccountParams>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_params: {e}")) ;
            self
        }
        pub fn delete_project_service_account_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . delete_project_service_account_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for delete_project_service_account_response: {e}")) ;
            self
        }
        pub fn delete_project_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_error400: {e}")
            });
            self
        }
        pub fn delete_project_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_error403: {e}")
            });
            self
        }
        pub fn delete_project_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_error404: {e}")
            });
            self
        }
        pub fn delete_project_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_error429: {e}")
            });
            self
        }
        pub fn delete_project_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteProjectUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_params: {e}")
            });
            self
        }
        pub fn delete_project_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUserDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_project_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_project_user_response: {e}")
            });
            self
        }
        pub fn list_project_api_keys_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_error400: {e}")
            });
            self
        }
        pub fn list_project_api_keys_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_error403: {e}")
            });
            self
        }
        pub fn list_project_api_keys_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_error404: {e}")
            });
            self
        }
        pub fn list_project_api_keys_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_error429: {e}")
            });
            self
        }
        pub fn list_project_api_keys_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectApiKeysParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_params: {e}")
            });
            self
        }
        pub fn list_project_api_keys_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKeyListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_api_keys_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_api_keys_response: {e}")
            });
            self
        }
        pub fn list_project_rate_limits_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_rate_limits_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_rate_limits_error400: {e}"
                )
            });
            self
        }
        pub fn list_project_rate_limits_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_rate_limits_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_rate_limits_error403: {e}"
                )
            });
            self
        }
        pub fn list_project_rate_limits_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_rate_limits_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_rate_limits_error404: {e}"
                )
            });
            self
        }
        pub fn list_project_rate_limits_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectRateLimitsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_rate_limits_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_rate_limits_params: {e}")
            });
            self
        }
        pub fn list_project_rate_limits_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectRateLimitListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_rate_limits_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_rate_limits_response: {e}"
                )
            });
            self
        }
        pub fn list_project_service_accounts_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_service_accounts_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_service_accounts_error400: {e}")) ;
            self
        }
        pub fn list_project_service_accounts_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_service_accounts_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_service_accounts_error403: {e}")) ;
            self
        }
        pub fn list_project_service_accounts_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_service_accounts_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_service_accounts_error404: {e}")) ;
            self
        }
        pub fn list_project_service_accounts_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_service_accounts_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_service_accounts_error429: {e}")) ;
            self
        }
        pub fn list_project_service_accounts_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectServiceAccountsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_service_accounts_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_project_service_accounts_params: {e}"
                )
            });
            self
        }
        pub fn list_project_service_accounts_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccountListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_service_accounts_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_service_accounts_response: {e}")) ;
            self
        }
        pub fn list_project_users_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_error400: {e}")
            });
            self
        }
        pub fn list_project_users_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_error403: {e}")
            });
            self
        }
        pub fn list_project_users_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_error404: {e}")
            });
            self
        }
        pub fn list_project_users_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_error429: {e}")
            });
            self
        }
        pub fn list_project_users_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectUsersParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_params: {e}")
            });
            self
        }
        pub fn list_project_users_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUserListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_project_users_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_project_users_response: {e}")
            });
            self
        }
        pub fn list_projects_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_error400: {e}")
            });
            self
        }
        pub fn list_projects_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_error403: {e}")
            });
            self
        }
        pub fn list_projects_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_error404: {e}")
            });
            self
        }
        pub fn list_projects_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_error429: {e}")
            });
            self
        }
        pub fn list_projects_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_params: {e}")
            });
            self
        }
        pub fn list_projects_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_projects_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_projects_response: {e}")
            });
            self
        }
        pub fn modify_project_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_error400: {e}")
            });
            self
        }
        pub fn modify_project_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_error403: {e}")
            });
            self
        }
        pub fn modify_project_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_error404: {e}")
            });
            self
        }
        pub fn modify_project_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_error429: {e}")
            });
            self
        }
        pub fn modify_project_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyProjectParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_params: {e}")
            });
            self
        }
        pub fn modify_project_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_request: {e}")
            });
            self
        }
        pub fn modify_project_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Project>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_response: {e}")
            });
            self
        }
        pub fn modify_project_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_error400: {e}")
            });
            self
        }
        pub fn modify_project_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_error403: {e}")
            });
            self
        }
        pub fn modify_project_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_error404: {e}")
            });
            self
        }
        pub fn modify_project_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_error429: {e}")
            });
            self
        }
        pub fn modify_project_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyProjectUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_params: {e}")
            });
            self
        }
        pub fn modify_project_user_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUserUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_request: {e}")
            });
            self
        }
        pub fn modify_project_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUser>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_project_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_project_user_response: {e}")
            });
            self
        }
        pub fn retrieve_project_api_key_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_api_key_error400: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_api_key_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_api_key_error403: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_api_key_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_api_key_error404: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_api_key_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_api_key_error429: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_api_key_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectApiKeyParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_api_key_params: {e}")
            });
            self
        }
        pub fn retrieve_project_api_key_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKey>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_api_key_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_api_key_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_error403: {e}")
            });
            self
        }
        pub fn retrieve_project_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_error404: {e}")
            });
            self
        }
        pub fn retrieve_project_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_error429: {e}")
            });
            self
        }
        pub fn retrieve_project_model_permissions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_error400: {e}")) ;
            self
        }
        pub fn retrieve_project_model_permissions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_error403: {e}")) ;
            self
        }
        pub fn retrieve_project_model_permissions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_error404: {e}")) ;
            self
        }
        pub fn retrieve_project_model_permissions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_error429: {e}")) ;
            self
        }
        pub fn retrieve_project_model_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectModelPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_params: {e}")) ;
            self
        }
        pub fn retrieve_project_model_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissions>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_model_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_model_permissions_response: {e}")) ;
            self
        }
        pub fn retrieve_project_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_params: {e}")
            });
            self
        }
        pub fn retrieve_project_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Project>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_response: {e}")
            });
            self
        }
        pub fn retrieve_project_service_account_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_error400: {e}")) ;
            self
        }
        pub fn retrieve_project_service_account_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_error403: {e}")) ;
            self
        }
        pub fn retrieve_project_service_account_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_error404: {e}")) ;
            self
        }
        pub fn retrieve_project_service_account_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_error429: {e}")) ;
            self
        }
        pub fn retrieve_project_service_account_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectServiceAccountParams>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_params: {e}")) ;
            self
        }
        pub fn retrieve_project_service_account_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccount>,
            T::Error: ::std::fmt::Display,
        {
            self . retrieve_project_service_account_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for retrieve_project_service_account_response: {e}")) ;
            self
        }
        pub fn retrieve_project_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_error400: {e}")
            });
            self
        }
        pub fn retrieve_project_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_error403: {e}")
            });
            self
        }
        pub fn retrieve_project_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_error404: {e}")
            });
            self
        }
        pub fn retrieve_project_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_error429: {e}")
            });
            self
        }
        pub fn retrieve_project_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_params: {e}")
            });
            self
        }
        pub fn retrieve_project_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectUser>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_project_user_response: {e}")
            });
            self
        }
        pub fn update_project_api_key_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_error400: {e}")
            });
            self
        }
        pub fn update_project_api_key_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_error403: {e}")
            });
            self
        }
        pub fn update_project_api_key_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_error404: {e}")
            });
            self
        }
        pub fn update_project_api_key_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_error429: {e}")
            });
            self
        }
        pub fn update_project_api_key_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectApiKeyParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_params: {e}")
            });
            self
        }
        pub fn update_project_api_key_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectApiKeyBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_request: {e}")
            });
            self
        }
        pub fn update_project_api_key_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectApiKey>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_api_key_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_project_api_key_response: {e}")
            });
            self
        }
        pub fn update_project_model_permissions_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_error400: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_error403: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_error404: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_error429: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectModelPermissionsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_params: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissionsUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_request: {e}")) ;
            self
        }
        pub fn update_project_model_permissions_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectModelPermissions>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_model_permissions_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_model_permissions_response: {e}")) ;
            self
        }
        pub fn update_project_rate_limits_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_error400: {e}"
                )
            });
            self
        }
        pub fn update_project_rate_limits_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_error403: {e}"
                )
            });
            self
        }
        pub fn update_project_rate_limits_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_error404: {e}"
                )
            });
            self
        }
        pub fn update_project_rate_limits_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectRateLimitsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_params: {e}"
                )
            });
            self
        }
        pub fn update_project_rate_limits_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectRateLimitUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_request: {e}"
                )
            });
            self
        }
        pub fn update_project_rate_limits_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectRateLimit>,
            T::Error: ::std::fmt::Display,
        {
            self.update_project_rate_limits_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for update_project_rate_limits_response: {e}"
                )
            });
            self
        }
        pub fn update_project_service_account_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_error400: {e}")) ;
            self
        }
        pub fn update_project_service_account_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_error403: {e}")) ;
            self
        }
        pub fn update_project_service_account_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_error404: {e}")) ;
            self
        }
        pub fn update_project_service_account_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_error429: {e}")) ;
            self
        }
        pub fn update_project_service_account_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectServiceAccountParams>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_params: {e}")) ;
            self
        }
        pub fn update_project_service_account_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectServiceAccountBody>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_request: {e}")) ;
            self
        }
        pub fn update_project_service_account_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectServiceAccount>,
            T::Error: ::std::fmt::Display,
        {
            self . update_project_service_account_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for update_project_service_account_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectsApiSchema> for super::ProjectsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                archive_project_error400: value.archive_project_error400?,
                archive_project_error403: value.archive_project_error403?,
                archive_project_error404: value.archive_project_error404?,
                archive_project_error429: value.archive_project_error429?,
                archive_project_params: value.archive_project_params?,
                archive_project_response: value.archive_project_response?,
                create_project_error400: value.create_project_error400?,
                create_project_error403: value.create_project_error403?,
                create_project_error429: value.create_project_error429?,
                create_project_request: value.create_project_request?,
                create_project_response: value.create_project_response?,
                create_project_service_account_error400: value
                    .create_project_service_account_error400?,
                create_project_service_account_error403: value
                    .create_project_service_account_error403?,
                create_project_service_account_error404: value
                    .create_project_service_account_error404?,
                create_project_service_account_error429: value
                    .create_project_service_account_error429?,
                create_project_service_account_params: value
                    .create_project_service_account_params?,
                create_project_service_account_request: value
                    .create_project_service_account_request?,
                create_project_service_account_response: value
                    .create_project_service_account_response?,
                create_project_user_error400: value.create_project_user_error400?,
                create_project_user_error403: value.create_project_user_error403?,
                create_project_user_error404: value.create_project_user_error404?,
                create_project_user_error409: value.create_project_user_error409?,
                create_project_user_error429: value.create_project_user_error429?,
                create_project_user_params: value.create_project_user_params?,
                create_project_user_request: value.create_project_user_request?,
                create_project_user_response: value.create_project_user_response?,
                delete_project_api_key_error400: value.delete_project_api_key_error400?,
                delete_project_api_key_error403: value.delete_project_api_key_error403?,
                delete_project_api_key_error404: value.delete_project_api_key_error404?,
                delete_project_api_key_error429: value.delete_project_api_key_error429?,
                delete_project_api_key_params: value.delete_project_api_key_params?,
                delete_project_api_key_response: value.delete_project_api_key_response?,
                delete_project_model_permissions_error400: value
                    .delete_project_model_permissions_error400?,
                delete_project_model_permissions_error403: value
                    .delete_project_model_permissions_error403?,
                delete_project_model_permissions_error404: value
                    .delete_project_model_permissions_error404?,
                delete_project_model_permissions_error429: value
                    .delete_project_model_permissions_error429?,
                delete_project_model_permissions_params: value
                    .delete_project_model_permissions_params?,
                delete_project_model_permissions_response: value
                    .delete_project_model_permissions_response?,
                delete_project_service_account_error400: value
                    .delete_project_service_account_error400?,
                delete_project_service_account_error403: value
                    .delete_project_service_account_error403?,
                delete_project_service_account_error404: value
                    .delete_project_service_account_error404?,
                delete_project_service_account_error429: value
                    .delete_project_service_account_error429?,
                delete_project_service_account_params: value
                    .delete_project_service_account_params?,
                delete_project_service_account_response: value
                    .delete_project_service_account_response?,
                delete_project_user_error400: value.delete_project_user_error400?,
                delete_project_user_error403: value.delete_project_user_error403?,
                delete_project_user_error404: value.delete_project_user_error404?,
                delete_project_user_error429: value.delete_project_user_error429?,
                delete_project_user_params: value.delete_project_user_params?,
                delete_project_user_response: value.delete_project_user_response?,
                list_project_api_keys_error400: value.list_project_api_keys_error400?,
                list_project_api_keys_error403: value.list_project_api_keys_error403?,
                list_project_api_keys_error404: value.list_project_api_keys_error404?,
                list_project_api_keys_error429: value.list_project_api_keys_error429?,
                list_project_api_keys_params: value.list_project_api_keys_params?,
                list_project_api_keys_response: value.list_project_api_keys_response?,
                list_project_rate_limits_error400: value.list_project_rate_limits_error400?,
                list_project_rate_limits_error403: value.list_project_rate_limits_error403?,
                list_project_rate_limits_error404: value.list_project_rate_limits_error404?,
                list_project_rate_limits_params: value.list_project_rate_limits_params?,
                list_project_rate_limits_response: value.list_project_rate_limits_response?,
                list_project_service_accounts_error400: value
                    .list_project_service_accounts_error400?,
                list_project_service_accounts_error403: value
                    .list_project_service_accounts_error403?,
                list_project_service_accounts_error404: value
                    .list_project_service_accounts_error404?,
                list_project_service_accounts_error429: value
                    .list_project_service_accounts_error429?,
                list_project_service_accounts_params: value.list_project_service_accounts_params?,
                list_project_service_accounts_response: value
                    .list_project_service_accounts_response?,
                list_project_users_error400: value.list_project_users_error400?,
                list_project_users_error403: value.list_project_users_error403?,
                list_project_users_error404: value.list_project_users_error404?,
                list_project_users_error429: value.list_project_users_error429?,
                list_project_users_params: value.list_project_users_params?,
                list_project_users_response: value.list_project_users_response?,
                list_projects_error400: value.list_projects_error400?,
                list_projects_error403: value.list_projects_error403?,
                list_projects_error404: value.list_projects_error404?,
                list_projects_error429: value.list_projects_error429?,
                list_projects_params: value.list_projects_params?,
                list_projects_response: value.list_projects_response?,
                modify_project_error400: value.modify_project_error400?,
                modify_project_error403: value.modify_project_error403?,
                modify_project_error404: value.modify_project_error404?,
                modify_project_error429: value.modify_project_error429?,
                modify_project_params: value.modify_project_params?,
                modify_project_request: value.modify_project_request?,
                modify_project_response: value.modify_project_response?,
                modify_project_user_error400: value.modify_project_user_error400?,
                modify_project_user_error403: value.modify_project_user_error403?,
                modify_project_user_error404: value.modify_project_user_error404?,
                modify_project_user_error429: value.modify_project_user_error429?,
                modify_project_user_params: value.modify_project_user_params?,
                modify_project_user_request: value.modify_project_user_request?,
                modify_project_user_response: value.modify_project_user_response?,
                retrieve_project_api_key_error400: value.retrieve_project_api_key_error400?,
                retrieve_project_api_key_error403: value.retrieve_project_api_key_error403?,
                retrieve_project_api_key_error404: value.retrieve_project_api_key_error404?,
                retrieve_project_api_key_error429: value.retrieve_project_api_key_error429?,
                retrieve_project_api_key_params: value.retrieve_project_api_key_params?,
                retrieve_project_api_key_response: value.retrieve_project_api_key_response?,
                retrieve_project_error403: value.retrieve_project_error403?,
                retrieve_project_error404: value.retrieve_project_error404?,
                retrieve_project_error429: value.retrieve_project_error429?,
                retrieve_project_model_permissions_error400: value
                    .retrieve_project_model_permissions_error400?,
                retrieve_project_model_permissions_error403: value
                    .retrieve_project_model_permissions_error403?,
                retrieve_project_model_permissions_error404: value
                    .retrieve_project_model_permissions_error404?,
                retrieve_project_model_permissions_error429: value
                    .retrieve_project_model_permissions_error429?,
                retrieve_project_model_permissions_params: value
                    .retrieve_project_model_permissions_params?,
                retrieve_project_model_permissions_response: value
                    .retrieve_project_model_permissions_response?,
                retrieve_project_params: value.retrieve_project_params?,
                retrieve_project_response: value.retrieve_project_response?,
                retrieve_project_service_account_error400: value
                    .retrieve_project_service_account_error400?,
                retrieve_project_service_account_error403: value
                    .retrieve_project_service_account_error403?,
                retrieve_project_service_account_error404: value
                    .retrieve_project_service_account_error404?,
                retrieve_project_service_account_error429: value
                    .retrieve_project_service_account_error429?,
                retrieve_project_service_account_params: value
                    .retrieve_project_service_account_params?,
                retrieve_project_service_account_response: value
                    .retrieve_project_service_account_response?,
                retrieve_project_user_error400: value.retrieve_project_user_error400?,
                retrieve_project_user_error403: value.retrieve_project_user_error403?,
                retrieve_project_user_error404: value.retrieve_project_user_error404?,
                retrieve_project_user_error429: value.retrieve_project_user_error429?,
                retrieve_project_user_params: value.retrieve_project_user_params?,
                retrieve_project_user_response: value.retrieve_project_user_response?,
                update_project_api_key_error400: value.update_project_api_key_error400?,
                update_project_api_key_error403: value.update_project_api_key_error403?,
                update_project_api_key_error404: value.update_project_api_key_error404?,
                update_project_api_key_error429: value.update_project_api_key_error429?,
                update_project_api_key_params: value.update_project_api_key_params?,
                update_project_api_key_request: value.update_project_api_key_request?,
                update_project_api_key_response: value.update_project_api_key_response?,
                update_project_model_permissions_error400: value
                    .update_project_model_permissions_error400?,
                update_project_model_permissions_error403: value
                    .update_project_model_permissions_error403?,
                update_project_model_permissions_error404: value
                    .update_project_model_permissions_error404?,
                update_project_model_permissions_error429: value
                    .update_project_model_permissions_error429?,
                update_project_model_permissions_params: value
                    .update_project_model_permissions_params?,
                update_project_model_permissions_request: value
                    .update_project_model_permissions_request?,
                update_project_model_permissions_response: value
                    .update_project_model_permissions_response?,
                update_project_rate_limits_error400: value.update_project_rate_limits_error400?,
                update_project_rate_limits_error403: value.update_project_rate_limits_error403?,
                update_project_rate_limits_error404: value.update_project_rate_limits_error404?,
                update_project_rate_limits_params: value.update_project_rate_limits_params?,
                update_project_rate_limits_request: value.update_project_rate_limits_request?,
                update_project_rate_limits_response: value.update_project_rate_limits_response?,
                update_project_service_account_error400: value
                    .update_project_service_account_error400?,
                update_project_service_account_error403: value
                    .update_project_service_account_error403?,
                update_project_service_account_error404: value
                    .update_project_service_account_error404?,
                update_project_service_account_error429: value
                    .update_project_service_account_error429?,
                update_project_service_account_params: value
                    .update_project_service_account_params?,
                update_project_service_account_request: value
                    .update_project_service_account_request?,
                update_project_service_account_response: value
                    .update_project_service_account_response?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectsApiSchema> for ProjectsApiSchema {
        fn from(value: super::ProjectsApiSchema) -> Self {
            Self {
                archive_project_error400: Ok(value.archive_project_error400),
                archive_project_error403: Ok(value.archive_project_error403),
                archive_project_error404: Ok(value.archive_project_error404),
                archive_project_error429: Ok(value.archive_project_error429),
                archive_project_params: Ok(value.archive_project_params),
                archive_project_response: Ok(value.archive_project_response),
                create_project_error400: Ok(value.create_project_error400),
                create_project_error403: Ok(value.create_project_error403),
                create_project_error429: Ok(value.create_project_error429),
                create_project_request: Ok(value.create_project_request),
                create_project_response: Ok(value.create_project_response),
                create_project_service_account_error400: Ok(
                    value.create_project_service_account_error400
                ),
                create_project_service_account_error403: Ok(
                    value.create_project_service_account_error403
                ),
                create_project_service_account_error404: Ok(
                    value.create_project_service_account_error404
                ),
                create_project_service_account_error429: Ok(
                    value.create_project_service_account_error429
                ),
                create_project_service_account_params: Ok(
                    value.create_project_service_account_params
                ),
                create_project_service_account_request: Ok(
                    value.create_project_service_account_request
                ),
                create_project_service_account_response: Ok(
                    value.create_project_service_account_response
                ),
                create_project_user_error400: Ok(value.create_project_user_error400),
                create_project_user_error403: Ok(value.create_project_user_error403),
                create_project_user_error404: Ok(value.create_project_user_error404),
                create_project_user_error409: Ok(value.create_project_user_error409),
                create_project_user_error429: Ok(value.create_project_user_error429),
                create_project_user_params: Ok(value.create_project_user_params),
                create_project_user_request: Ok(value.create_project_user_request),
                create_project_user_response: Ok(value.create_project_user_response),
                delete_project_api_key_error400: Ok(value.delete_project_api_key_error400),
                delete_project_api_key_error403: Ok(value.delete_project_api_key_error403),
                delete_project_api_key_error404: Ok(value.delete_project_api_key_error404),
                delete_project_api_key_error429: Ok(value.delete_project_api_key_error429),
                delete_project_api_key_params: Ok(value.delete_project_api_key_params),
                delete_project_api_key_response: Ok(value.delete_project_api_key_response),
                delete_project_model_permissions_error400: Ok(
                    value.delete_project_model_permissions_error400
                ),
                delete_project_model_permissions_error403: Ok(
                    value.delete_project_model_permissions_error403
                ),
                delete_project_model_permissions_error404: Ok(
                    value.delete_project_model_permissions_error404
                ),
                delete_project_model_permissions_error429: Ok(
                    value.delete_project_model_permissions_error429
                ),
                delete_project_model_permissions_params: Ok(
                    value.delete_project_model_permissions_params
                ),
                delete_project_model_permissions_response: Ok(
                    value.delete_project_model_permissions_response
                ),
                delete_project_service_account_error400: Ok(
                    value.delete_project_service_account_error400
                ),
                delete_project_service_account_error403: Ok(
                    value.delete_project_service_account_error403
                ),
                delete_project_service_account_error404: Ok(
                    value.delete_project_service_account_error404
                ),
                delete_project_service_account_error429: Ok(
                    value.delete_project_service_account_error429
                ),
                delete_project_service_account_params: Ok(
                    value.delete_project_service_account_params
                ),
                delete_project_service_account_response: Ok(
                    value.delete_project_service_account_response
                ),
                delete_project_user_error400: Ok(value.delete_project_user_error400),
                delete_project_user_error403: Ok(value.delete_project_user_error403),
                delete_project_user_error404: Ok(value.delete_project_user_error404),
                delete_project_user_error429: Ok(value.delete_project_user_error429),
                delete_project_user_params: Ok(value.delete_project_user_params),
                delete_project_user_response: Ok(value.delete_project_user_response),
                list_project_api_keys_error400: Ok(value.list_project_api_keys_error400),
                list_project_api_keys_error403: Ok(value.list_project_api_keys_error403),
                list_project_api_keys_error404: Ok(value.list_project_api_keys_error404),
                list_project_api_keys_error429: Ok(value.list_project_api_keys_error429),
                list_project_api_keys_params: Ok(value.list_project_api_keys_params),
                list_project_api_keys_response: Ok(value.list_project_api_keys_response),
                list_project_rate_limits_error400: Ok(value.list_project_rate_limits_error400),
                list_project_rate_limits_error403: Ok(value.list_project_rate_limits_error403),
                list_project_rate_limits_error404: Ok(value.list_project_rate_limits_error404),
                list_project_rate_limits_params: Ok(value.list_project_rate_limits_params),
                list_project_rate_limits_response: Ok(value.list_project_rate_limits_response),
                list_project_service_accounts_error400: Ok(
                    value.list_project_service_accounts_error400
                ),
                list_project_service_accounts_error403: Ok(
                    value.list_project_service_accounts_error403
                ),
                list_project_service_accounts_error404: Ok(
                    value.list_project_service_accounts_error404
                ),
                list_project_service_accounts_error429: Ok(
                    value.list_project_service_accounts_error429
                ),
                list_project_service_accounts_params: Ok(value.list_project_service_accounts_params),
                list_project_service_accounts_response: Ok(
                    value.list_project_service_accounts_response
                ),
                list_project_users_error400: Ok(value.list_project_users_error400),
                list_project_users_error403: Ok(value.list_project_users_error403),
                list_project_users_error404: Ok(value.list_project_users_error404),
                list_project_users_error429: Ok(value.list_project_users_error429),
                list_project_users_params: Ok(value.list_project_users_params),
                list_project_users_response: Ok(value.list_project_users_response),
                list_projects_error400: Ok(value.list_projects_error400),
                list_projects_error403: Ok(value.list_projects_error403),
                list_projects_error404: Ok(value.list_projects_error404),
                list_projects_error429: Ok(value.list_projects_error429),
                list_projects_params: Ok(value.list_projects_params),
                list_projects_response: Ok(value.list_projects_response),
                modify_project_error400: Ok(value.modify_project_error400),
                modify_project_error403: Ok(value.modify_project_error403),
                modify_project_error404: Ok(value.modify_project_error404),
                modify_project_error429: Ok(value.modify_project_error429),
                modify_project_params: Ok(value.modify_project_params),
                modify_project_request: Ok(value.modify_project_request),
                modify_project_response: Ok(value.modify_project_response),
                modify_project_user_error400: Ok(value.modify_project_user_error400),
                modify_project_user_error403: Ok(value.modify_project_user_error403),
                modify_project_user_error404: Ok(value.modify_project_user_error404),
                modify_project_user_error429: Ok(value.modify_project_user_error429),
                modify_project_user_params: Ok(value.modify_project_user_params),
                modify_project_user_request: Ok(value.modify_project_user_request),
                modify_project_user_response: Ok(value.modify_project_user_response),
                retrieve_project_api_key_error400: Ok(value.retrieve_project_api_key_error400),
                retrieve_project_api_key_error403: Ok(value.retrieve_project_api_key_error403),
                retrieve_project_api_key_error404: Ok(value.retrieve_project_api_key_error404),
                retrieve_project_api_key_error429: Ok(value.retrieve_project_api_key_error429),
                retrieve_project_api_key_params: Ok(value.retrieve_project_api_key_params),
                retrieve_project_api_key_response: Ok(value.retrieve_project_api_key_response),
                retrieve_project_error403: Ok(value.retrieve_project_error403),
                retrieve_project_error404: Ok(value.retrieve_project_error404),
                retrieve_project_error429: Ok(value.retrieve_project_error429),
                retrieve_project_model_permissions_error400: Ok(
                    value.retrieve_project_model_permissions_error400
                ),
                retrieve_project_model_permissions_error403: Ok(
                    value.retrieve_project_model_permissions_error403
                ),
                retrieve_project_model_permissions_error404: Ok(
                    value.retrieve_project_model_permissions_error404
                ),
                retrieve_project_model_permissions_error429: Ok(
                    value.retrieve_project_model_permissions_error429
                ),
                retrieve_project_model_permissions_params: Ok(
                    value.retrieve_project_model_permissions_params
                ),
                retrieve_project_model_permissions_response: Ok(
                    value.retrieve_project_model_permissions_response
                ),
                retrieve_project_params: Ok(value.retrieve_project_params),
                retrieve_project_response: Ok(value.retrieve_project_response),
                retrieve_project_service_account_error400: Ok(
                    value.retrieve_project_service_account_error400
                ),
                retrieve_project_service_account_error403: Ok(
                    value.retrieve_project_service_account_error403
                ),
                retrieve_project_service_account_error404: Ok(
                    value.retrieve_project_service_account_error404
                ),
                retrieve_project_service_account_error429: Ok(
                    value.retrieve_project_service_account_error429
                ),
                retrieve_project_service_account_params: Ok(
                    value.retrieve_project_service_account_params
                ),
                retrieve_project_service_account_response: Ok(
                    value.retrieve_project_service_account_response
                ),
                retrieve_project_user_error400: Ok(value.retrieve_project_user_error400),
                retrieve_project_user_error403: Ok(value.retrieve_project_user_error403),
                retrieve_project_user_error404: Ok(value.retrieve_project_user_error404),
                retrieve_project_user_error429: Ok(value.retrieve_project_user_error429),
                retrieve_project_user_params: Ok(value.retrieve_project_user_params),
                retrieve_project_user_response: Ok(value.retrieve_project_user_response),
                update_project_api_key_error400: Ok(value.update_project_api_key_error400),
                update_project_api_key_error403: Ok(value.update_project_api_key_error403),
                update_project_api_key_error404: Ok(value.update_project_api_key_error404),
                update_project_api_key_error429: Ok(value.update_project_api_key_error429),
                update_project_api_key_params: Ok(value.update_project_api_key_params),
                update_project_api_key_request: Ok(value.update_project_api_key_request),
                update_project_api_key_response: Ok(value.update_project_api_key_response),
                update_project_model_permissions_error400: Ok(
                    value.update_project_model_permissions_error400
                ),
                update_project_model_permissions_error403: Ok(
                    value.update_project_model_permissions_error403
                ),
                update_project_model_permissions_error404: Ok(
                    value.update_project_model_permissions_error404
                ),
                update_project_model_permissions_error429: Ok(
                    value.update_project_model_permissions_error429
                ),
                update_project_model_permissions_params: Ok(
                    value.update_project_model_permissions_params
                ),
                update_project_model_permissions_request: Ok(
                    value.update_project_model_permissions_request
                ),
                update_project_model_permissions_response: Ok(
                    value.update_project_model_permissions_response
                ),
                update_project_rate_limits_error400: Ok(value.update_project_rate_limits_error400),
                update_project_rate_limits_error403: Ok(value.update_project_rate_limits_error403),
                update_project_rate_limits_error404: Ok(value.update_project_rate_limits_error404),
                update_project_rate_limits_params: Ok(value.update_project_rate_limits_params),
                update_project_rate_limits_request: Ok(value.update_project_rate_limits_request),
                update_project_rate_limits_response: Ok(value.update_project_rate_limits_response),
                update_project_service_account_error400: Ok(
                    value.update_project_service_account_error400
                ),
                update_project_service_account_error403: Ok(
                    value.update_project_service_account_error403
                ),
                update_project_service_account_error404: Ok(
                    value.update_project_service_account_error404
                ),
                update_project_service_account_error429: Ok(
                    value.update_project_service_account_error429
                ),
                update_project_service_account_params: Ok(
                    value.update_project_service_account_params
                ),
                update_project_service_account_request: Ok(
                    value.update_project_service_account_request
                ),
                update_project_service_account_response: Ok(
                    value.update_project_service_account_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectApiKeyParams {
        api_key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectApiKeyParams {
        fn default() -> Self {
            Self {
                api_key_id: Err("no value supplied for api_key_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectApiKeyParams {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
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
    impl ::std::convert::TryFrom<RetrieveProjectApiKeyParams> for super::RetrieveProjectApiKeyParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectApiKeyParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectApiKeyParams> for RetrieveProjectApiKeyParams {
        fn from(value: super::RetrieveProjectApiKeyParams) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectModelPermissionsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectModelPermissionsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectModelPermissionsParams {
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
    impl ::std::convert::TryFrom<RetrieveProjectModelPermissionsParams>
        for super::RetrieveProjectModelPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectModelPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectModelPermissionsParams>
        for RetrieveProjectModelPermissionsParams
    {
        fn from(value: super::RetrieveProjectModelPermissionsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl RetrieveProjectParams {
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
    impl ::std::convert::TryFrom<RetrieveProjectParams> for super::RetrieveProjectParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectParams> for RetrieveProjectParams {
        fn from(value: super::RetrieveProjectParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectServiceAccountParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        service_account_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectServiceAccountParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                service_account_id: Err("no value supplied for service_account_id".to_string()),
            }
        }
    }
    impl RetrieveProjectServiceAccountParams {
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
        pub fn service_account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveProjectServiceAccountParams>
        for super::RetrieveProjectServiceAccountParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectServiceAccountParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                service_account_id: value.service_account_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectServiceAccountParams>
        for RetrieveProjectServiceAccountParams
    {
        fn from(value: super::RetrieveProjectServiceAccountParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                service_account_id: Ok(value.service_account_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectUserParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectUserParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RetrieveProjectUserParams {
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
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveProjectUserParams> for super::RetrieveProjectUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectUserParams> for RetrieveProjectUserParams {
        fn from(value: super::RetrieveProjectUserParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectApiKeyBody {
        expires_in_seconds: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectApiKeyBody {
        fn default() -> Self {
            Self {
                expires_in_seconds: Err("no value supplied for expires_in_seconds".to_string()),
            }
        }
    }
    impl UpdateProjectApiKeyBody {
        pub fn expires_in_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_in_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for expires_in_seconds: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectApiKeyBody> for super::UpdateProjectApiKeyBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectApiKeyBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_in_seconds: value.expires_in_seconds?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectApiKeyBody> for UpdateProjectApiKeyBody {
        fn from(value: super::UpdateProjectApiKeyBody) -> Self {
            Self {
                expires_in_seconds: Ok(value.expires_in_seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectApiKeyParams {
        api_key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectApiKeyParams {
        fn default() -> Self {
            Self {
                api_key_id: Err("no value supplied for api_key_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateProjectApiKeyParams {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
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
    impl ::std::convert::TryFrom<UpdateProjectApiKeyParams> for super::UpdateProjectApiKeyParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectApiKeyParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectApiKeyParams> for UpdateProjectApiKeyParams {
        fn from(value: super::UpdateProjectApiKeyParams) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectModelPermissionsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectModelPermissionsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateProjectModelPermissionsParams {
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
    impl ::std::convert::TryFrom<UpdateProjectModelPermissionsParams>
        for super::UpdateProjectModelPermissionsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectModelPermissionsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectModelPermissionsParams>
        for UpdateProjectModelPermissionsParams
    {
        fn from(value: super::UpdateProjectModelPermissionsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectRateLimitsParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        rate_limit_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectRateLimitsParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                rate_limit_id: Err("no value supplied for rate_limit_id".to_string()),
            }
        }
    }
    impl UpdateProjectRateLimitsParams {
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
        pub fn rate_limit_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.rate_limit_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rate_limit_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectRateLimitsParams>
        for super::UpdateProjectRateLimitsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectRateLimitsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                rate_limit_id: value.rate_limit_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectRateLimitsParams> for UpdateProjectRateLimitsParams {
        fn from(value: super::UpdateProjectRateLimitsParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                rate_limit_id: Ok(value.rate_limit_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectServiceAccountBody {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<super::UpdateProjectServiceAccountBodyRole>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateProjectServiceAccountBody {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                role: Ok(Default::default()),
            }
        }
    }
    impl UpdateProjectServiceAccountBody {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::UpdateProjectServiceAccountBodyRole>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectServiceAccountBody>
        for super::UpdateProjectServiceAccountBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectServiceAccountBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectServiceAccountBody>
        for UpdateProjectServiceAccountBody
    {
        fn from(value: super::UpdateProjectServiceAccountBody) -> Self {
            Self {
                name: Ok(value.name),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectServiceAccountParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        service_account_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectServiceAccountParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                service_account_id: Err("no value supplied for service_account_id".to_string()),
            }
        }
    }
    impl UpdateProjectServiceAccountParams {
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
        pub fn service_account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectServiceAccountParams>
        for super::UpdateProjectServiceAccountParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectServiceAccountParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                service_account_id: value.service_account_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectServiceAccountParams>
        for UpdateProjectServiceAccountParams
    {
        fn from(value: super::UpdateProjectServiceAccountParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                service_account_id: Ok(value.service_account_id),
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
