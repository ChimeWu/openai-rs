pub use crate::common::{AwsExternalStorageProviderResponse, AzureExternalStorageProviderResponse, Error, ErrorResponse, GcpExternalStorageProviderResponse, MisalignmentErrorDetailsResource, OciExternalStorageProviderResponse, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Represents an individual Admin API key in an org."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKey {
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_used_at: ::std::option::Option<i64>,
    #[doc = "The name of the API key"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.admin_api_key`"]
    pub object: AdminApiKeyObject,
    pub owner: AdminApiKeyOwner,
    #[doc = "The redacted value of the API key"]
    pub redacted_value: ::std::string::String,
}
impl AdminApiKey {
    pub fn builder() -> builder::AdminApiKey {
        Default::default()
    }
}
#[doc = "`AdminApiKeyCreateResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKeyCreateResponse {
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_used_at: ::std::option::Option<i64>,
    #[doc = "The name of the API key"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `organization.admin_api_key`"]
    pub object: AdminApiKeyCreateResponseObject,
    pub owner: AdminApiKeyCreateResponseOwner,
    #[doc = "The redacted value of the API key"]
    pub redacted_value: ::std::string::String,
    #[doc = "The value of the API key. Only shown on create."]
    pub value: ::std::string::String,
}
impl AdminApiKeyCreateResponse {
    pub fn builder() -> builder::AdminApiKeyCreateResponse {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.admin_api_key`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdminApiKeyCreateResponseObject {
    #[serde(rename = "organization.admin_api_key")]
    OrganizationAdminApiKey,
}
impl ::std::fmt::Display for AdminApiKeyCreateResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationAdminApiKey => f.write_str("organization.admin_api_key"),
        }
    }
}
impl ::std::str::FromStr for AdminApiKeyCreateResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.admin_api_key" => Ok(Self::OrganizationAdminApiKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdminApiKeyCreateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdminApiKeyCreateResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AdminApiKeyCreateResponseOwner`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AdminApiKeyCreateResponseOwner {
    #[doc = "The Unix timestamp (in seconds) of when the user was created"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always organization.user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
    #[doc = "Always `owner`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[doc = "Always `user`"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl AdminApiKeyCreateResponseOwner {
    pub fn builder() -> builder::AdminApiKeyCreateResponseOwner {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.admin_api_key`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdminApiKeyObject {
    #[serde(rename = "organization.admin_api_key")]
    OrganizationAdminApiKey,
}
impl ::std::fmt::Display for AdminApiKeyObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationAdminApiKey => f.write_str("organization.admin_api_key"),
        }
    }
}
impl ::std::str::FromStr for AdminApiKeyObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.admin_api_key" => Ok(Self::OrganizationAdminApiKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdminApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdminApiKeyObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AdminApiKeyOwner`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AdminApiKeyOwner {
    #[doc = "The Unix timestamp (in seconds) of when the user was created"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always organization.user"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub object: ::std::option::Option<::std::string::String>,
    #[doc = "Always `owner`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[doc = "Always `user`"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl AdminApiKeyOwner {
    pub fn builder() -> builder::AdminApiKeyOwner {
        Default::default()
    }
}
#[doc = "`AdminApiKeysCreateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKeysCreateRequest {
    #[doc = "The number of seconds until the API key expires. Omit this field for a key that does not expire."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_in_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    pub name: ::std::string::String,
}
impl AdminApiKeysCreateRequest {
    pub fn builder() -> builder::AdminApiKeysCreateRequest {
        Default::default()
    }
}
#[doc = "`AdminApiKeysDeleteParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKeysDeleteParams {
    #[doc = "The ID of the API key to be deleted."]
    pub key_id: ::std::string::String,
}
impl AdminApiKeysDeleteParams {
    pub fn builder() -> builder::AdminApiKeysDeleteParams {
        Default::default()
    }
}
#[doc = "`AdminApiKeysDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKeysDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: AdminApiKeysDeleteResponseObject,
}
impl AdminApiKeysDeleteResponse {
    pub fn builder() -> builder::AdminApiKeysDeleteResponse {
        Default::default()
    }
}
#[doc = "`AdminApiKeysDeleteResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdminApiKeysDeleteResponseObject {
    #[serde(rename = "organization.admin_api_key.deleted")]
    OrganizationAdminApiKeyDeleted,
}
impl ::std::fmt::Display for AdminApiKeysDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationAdminApiKeyDeleted => {
                f.write_str("organization.admin_api_key.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for AdminApiKeysDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.admin_api_key.deleted" => Ok(Self::OrganizationAdminApiKeyDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdminApiKeysDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdminApiKeysDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AdminApiKeysGetParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AdminApiKeysGetParams {
    #[doc = "The ID of the API key."]
    pub key_id: ::std::string::String,
}
impl AdminApiKeysGetParams {
    pub fn builder() -> builder::AdminApiKeysGetParams {
        Default::default()
    }
}
#[doc = "`AdminApiKeysListParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AdminApiKeysListParams {
    #[doc = "Return keys with IDs that come after this ID in the pagination order."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[doc = "Maximum number of keys to return."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[doc = "Order results by creation time, ascending or descending."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<AdminApiKeysListParamsOrder>,
}
impl AdminApiKeysListParams {
    pub fn builder() -> builder::AdminApiKeysListParams {
        Default::default()
    }
}
#[doc = "Order results by creation time, ascending or descending."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AdminApiKeysListParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for AdminApiKeysListParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for AdminApiKeysListParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AdminApiKeysListParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AdminApiKeysListParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ApiKeyList`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ApiKeyList {
    pub data: ::std::vec::Vec<AdminApiKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ApiKeyListObject,
}
impl ApiKeyList {
    pub fn builder() -> builder::ApiKeyList {
        Default::default()
    }
}
#[doc = "`ApiKeyListObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApiKeyListObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ApiKeyListObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ApiKeyListObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApiKeyListObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApiKeyListObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AwsExternalStorageProviderParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AwsExternalStorageProviderParams {
    pub bucket: ::std::string::String,
    pub role_arn: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: AwsExternalStorageProviderParamsType,
}
impl AwsExternalStorageProviderParams {
    pub fn builder() -> builder::AwsExternalStorageProviderParams {
        Default::default()
    }
}
#[doc = "`AwsExternalStorageProviderParamsType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AwsExternalStorageProviderParamsType {
    #[serde(rename = "aws")]
    Aws,
}
impl ::std::fmt::Display for AwsExternalStorageProviderParamsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Aws => f.write_str("aws"),
        }
    }
}
impl ::std::str::FromStr for AwsExternalStorageProviderParamsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "aws" => Ok(Self::Aws),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AwsExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AwsExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AzureExternalStorageProviderParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AzureExternalStorageProviderParams {
    pub account_name: ::std::string::String,
    pub container: ::std::string::String,
    pub resource_group: ::std::string::String,
    pub subscription_id: ::std::string::String,
    pub tenant_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: AzureExternalStorageProviderParamsType,
}
impl AzureExternalStorageProviderParams {
    pub fn builder() -> builder::AzureExternalStorageProviderParams {
        Default::default()
    }
}
#[doc = "`AzureExternalStorageProviderParamsType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AzureExternalStorageProviderParamsType {
    #[serde(rename = "azure")]
    Azure,
}
impl ::std::fmt::Display for AzureExternalStorageProviderParamsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Azure => f.write_str("azure"),
        }
    }
}
impl ::std::str::FromStr for AzureExternalStorageProviderParamsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "azure" => Ok(Self::Azure),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AzureExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AzureExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateExternalStorageBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateExternalStorageBody {
    pub project_id: ::std::string::String,
    pub provider: CreateExternalStorageBodyProvider,
}
impl CreateExternalStorageBody {
    pub fn builder() -> builder::CreateExternalStorageBody {
        Default::default()
    }
}
#[doc = "`CreateExternalStorageBodyProvider`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateExternalStorageBodyProvider {
    AwsExternalStorageProviderParams(AwsExternalStorageProviderParams),
    AzureExternalStorageProviderParams(AzureExternalStorageProviderParams),
    GcpExternalStorageProviderParams(GcpExternalStorageProviderParams),
    OciExternalStorageProviderParams(OciExternalStorageProviderParams),
}
impl ::std::convert::From<AwsExternalStorageProviderParams> for CreateExternalStorageBodyProvider {
    fn from(value: AwsExternalStorageProviderParams) -> Self {
        Self::AwsExternalStorageProviderParams(value)
    }
}
impl ::std::convert::From<AzureExternalStorageProviderParams>
    for CreateExternalStorageBodyProvider
{
    fn from(value: AzureExternalStorageProviderParams) -> Self {
        Self::AzureExternalStorageProviderParams(value)
    }
}
impl ::std::convert::From<GcpExternalStorageProviderParams> for CreateExternalStorageBodyProvider {
    fn from(value: GcpExternalStorageProviderParams) -> Self {
        Self::GcpExternalStorageProviderParams(value)
    }
}
impl ::std::convert::From<OciExternalStorageProviderParams> for CreateExternalStorageBodyProvider {
    fn from(value: OciExternalStorageProviderParams) -> Self {
        Self::OciExternalStorageProviderParams(value)
    }
}
#[doc = "The service account API key create request payload."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateProjectServiceAccountApiKeyBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_in_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "API key name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "API key scopes."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
impl CreateProjectServiceAccountApiKeyBody {
    pub fn builder() -> builder::CreateProjectServiceAccountApiKeyBody {
        Default::default()
    }
}
#[doc = "`CreateanApIkeyforaserviceaccountParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateanApIkeyforaserviceaccountParams {
    #[doc = "The ID of the project."]
    pub project_id: ::std::string::String,
    #[doc = "The ID of the service account."]
    pub service_account_id: ::std::string::String,
}
impl CreateanApIkeyforaserviceaccountParams {
    pub fn builder() -> builder::CreateanApIkeyforaserviceaccountParams {
        Default::default()
    }
}
#[doc = "`DecisionsUsageApiSource`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DecisionsUsageApiSource {
    #[serde(rename = "agents_api")]
    AgentsApi,
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ::std::fmt::Display for DecisionsUsageApiSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentsApi => f.write_str("agents_api"),
            Self::Unlabeled => f.write_str("unlabeled"),
        }
    }
}
impl ::std::str::FromStr for DecisionsUsageApiSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agents_api" => Ok(Self::AgentsApi),
            "unlabeled" => Ok(Self::Unlabeled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionsUsageApiSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionsUsageApiSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionsUsageBucket`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsUsageBucket {
    pub end_time: i64,
    pub object: DecisionsUsageBucketObject,
    pub results: ::std::vec::Vec<DecisionsUsageResult>,
    pub start_time: i64,
}
impl DecisionsUsageBucket {
    pub fn builder() -> builder::DecisionsUsageBucket {
        Default::default()
    }
}
#[doc = "`DecisionsUsageBucketObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DecisionsUsageBucketObject {
    #[serde(rename = "bucket")]
    Bucket,
}
impl ::std::fmt::Display for DecisionsUsageBucketObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bucket => f.write_str("bucket"),
        }
    }
}
impl ::std::str::FromStr for DecisionsUsageBucketObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bucket" => Ok(Self::Bucket),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionsUsageBucketObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionsUsageBucketObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionsUsageBucketWidth`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DecisionsUsageBucketWidth {
    #[serde(rename = "1m")]
    X1m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "1d")]
    X1d,
}
impl ::std::fmt::Display for DecisionsUsageBucketWidth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1m => f.write_str("1m"),
            Self::X1h => f.write_str("1h"),
            Self::X1d => f.write_str("1d"),
        }
    }
}
impl ::std::str::FromStr for DecisionsUsageBucketWidth {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1m" => Ok(Self::X1m),
            "1h" => Ok(Self::X1h),
            "1d" => Ok(Self::X1d),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionsUsageBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionsUsageBucketWidth {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionsUsageErrorDetails`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsUsageErrorDetails {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub code: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub param: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl DecisionsUsageErrorDetails {
    pub fn builder() -> builder::DecisionsUsageErrorDetails {
        Default::default()
    }
}
#[doc = "`DecisionsUsageErrorResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsUsageErrorResponse {
    pub error: DecisionsUsageErrorDetails,
}
impl DecisionsUsageErrorResponse {
    pub fn builder() -> builder::DecisionsUsageErrorResponse {
        Default::default()
    }
}
#[doc = "`DecisionsUsageGroupBy`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DecisionsUsageGroupBy {
    #[serde(rename = "project_id")]
    ProjectId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "api_key_id")]
    ApiKeyId,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "service_tier")]
    ServiceTier,
    #[serde(rename = "api_source")]
    ApiSource,
}
impl ::std::fmt::Display for DecisionsUsageGroupBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectId => f.write_str("project_id"),
            Self::UserId => f.write_str("user_id"),
            Self::ApiKeyId => f.write_str("api_key_id"),
            Self::Model => f.write_str("model"),
            Self::Batch => f.write_str("batch"),
            Self::ServiceTier => f.write_str("service_tier"),
            Self::ApiSource => f.write_str("api_source"),
        }
    }
}
impl ::std::str::FromStr for DecisionsUsageGroupBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project_id" => Ok(Self::ProjectId),
            "user_id" => Ok(Self::UserId),
            "api_key_id" => Ok(Self::ApiKeyId),
            "model" => Ok(Self::Model),
            "batch" => Ok(Self::Batch),
            "service_tier" => Ok(Self::ServiceTier),
            "api_source" => Ok(Self::ApiSource),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionsUsageGroupBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionsUsageGroupBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionsUsagePermissionErrorResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsUsagePermissionErrorResponse {
    #[doc = "Details about the permission error."]
    pub error: DecisionsUsagePermissionErrorResponseError,
}
impl DecisionsUsagePermissionErrorResponse {
    pub fn builder() -> builder::DecisionsUsagePermissionErrorResponse {
        Default::default()
    }
}
#[doc = "Details about the permission error."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DecisionsUsagePermissionErrorResponseError {
    DecisionsUsageErrorDetails(DecisionsUsageErrorDetails),
    String(::std::string::String),
}
impl ::std::convert::From<DecisionsUsageErrorDetails>
    for DecisionsUsagePermissionErrorResponseError
{
    fn from(value: DecisionsUsageErrorDetails) -> Self {
        Self::DecisionsUsageErrorDetails(value)
    }
}
#[doc = "The aggregated Decisions API usage details of the specific time bucket."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsUsageResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_source: ::std::option::Option<DecisionsUsageApiSource>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_audio_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cache_write_12h_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cache_write_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_audio_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_image_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_text_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_cached_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_image_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_text_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of input tokens used, including cached and cache-write tokens. This includes text, audio, and image tokens. For customers subscribed to Scale Tier, this includes Scale Tier tokens."]
    pub input_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_uncached_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<::std::string::String>,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: i64,
    pub object: DecisionsUsageResultObject,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_audio_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_image_tokens: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_text_tokens: ::std::option::Option<i64>,
    #[doc = "The aggregated number of output tokens used across text, audio, and image outputs. For customers subscribed to Scale Tier, this includes Scale Tier tokens."]
    pub output_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_tier: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
impl DecisionsUsageResult {
    pub fn builder() -> builder::DecisionsUsageResult {
        Default::default()
    }
}
#[doc = "`DecisionsUsageResultObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DecisionsUsageResultObject {
    #[serde(rename = "organization.usage.decisions.result")]
    OrganizationUsageDecisionsResult,
}
impl ::std::fmt::Display for DecisionsUsageResultObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUsageDecisionsResult => {
                f.write_str("organization.usage.decisions.result")
            }
        }
    }
}
impl ::std::str::FromStr for DecisionsUsageResultObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.usage.decisions.result" => Ok(Self::OrganizationUsageDecisionsResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionsUsageResultObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionsUsageResultObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteanexternalstorageconfigurationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteanexternalstorageconfigurationParams {
    pub external_storage_id: ::std::string::String,
}
impl DeleteanexternalstorageconfigurationParams {
    pub fn builder() -> builder::DeleteanexternalstorageconfigurationParams {
        Default::default()
    }
}
#[doc = "`DeleteprojectspendlimitParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteprojectspendlimitParams {
    pub project_id: ::std::string::String,
}
impl DeleteprojectspendlimitParams {
    pub fn builder() -> builder::DeleteprojectspendlimitParams {
        Default::default()
    }
}
#[doc = "`ExternalStorageDeletedResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ExternalStorageDeletedResource {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: ExternalStorageDeletedResourceObject,
}
impl ExternalStorageDeletedResource {
    pub fn builder() -> builder::ExternalStorageDeletedResource {
        Default::default()
    }
}
#[doc = "`ExternalStorageDeletedResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalStorageDeletedResourceObject {
    #[serde(rename = "organization.external_storage.deleted")]
    OrganizationExternalStorageDeleted,
}
impl ::std::fmt::Display for ExternalStorageDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationExternalStorageDeleted => {
                f.write_str("organization.external_storage.deleted")
            }
        }
    }
}
impl ::std::str::FromStr for ExternalStorageDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.external_storage.deleted" => Ok(Self::OrganizationExternalStorageDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalStorageDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalStorageDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExternalStorageListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ExternalStorageListResource {
    pub data: ::std::vec::Vec<ExternalStorageResponse>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ExternalStorageListResourceObject,
}
impl ExternalStorageListResource {
    pub fn builder() -> builder::ExternalStorageListResource {
        Default::default()
    }
}
#[doc = "`ExternalStorageListResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalStorageListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ExternalStorageListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ExternalStorageListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalStorageListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalStorageListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExternalStorageOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalStorageOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ExternalStorageOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ExternalStorageOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalStorageOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalStorageOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExternalStorageResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ExternalStorageResponse {
    pub created_at: i64,
    pub geography: ::std::string::String,
    pub id: ::std::string::String,
    pub object: ExternalStorageResponseObject,
    pub project_id: ::std::string::String,
    pub provider: ExternalStorageResponseProvider,
    pub status: ExternalStorageStatus,
}
impl ExternalStorageResponse {
    pub fn builder() -> builder::ExternalStorageResponse {
        Default::default()
    }
}
#[doc = "`ExternalStorageResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalStorageResponseObject {
    #[serde(rename = "organization.external_storage")]
    OrganizationExternalStorage,
}
impl ::std::fmt::Display for ExternalStorageResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationExternalStorage => f.write_str("organization.external_storage"),
        }
    }
}
impl ::std::str::FromStr for ExternalStorageResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.external_storage" => Ok(Self::OrganizationExternalStorage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalStorageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalStorageResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ExternalStorageResponseProvider`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ExternalStorageResponseProvider {
    Variant0(::openai_api_types::common::AwsExternalStorageProviderResponse),
    Variant1(::openai_api_types::common::AzureExternalStorageProviderResponse),
    Variant2(::openai_api_types::common::GcpExternalStorageProviderResponse),
    Variant3(::openai_api_types::common::OciExternalStorageProviderResponse),
}
impl ::std::convert::From<::openai_api_types::common::AwsExternalStorageProviderResponse>
    for ExternalStorageResponseProvider
{
    fn from(value: ::openai_api_types::common::AwsExternalStorageProviderResponse) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::AzureExternalStorageProviderResponse>
    for ExternalStorageResponseProvider
{
    fn from(value: ::openai_api_types::common::AzureExternalStorageProviderResponse) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::GcpExternalStorageProviderResponse>
    for ExternalStorageResponseProvider
{
    fn from(value: ::openai_api_types::common::GcpExternalStorageProviderResponse) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::OciExternalStorageProviderResponse>
    for ExternalStorageResponseProvider
{
    fn from(value: ::openai_api_types::common::OciExternalStorageProviderResponse) -> Self {
        Self::Variant3(value)
    }
}
#[doc = "`ExternalStorageStatus`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExternalStorageStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "validated")]
    Validated,
    #[serde(rename = "unhealthy")]
    Unhealthy,
}
impl ::std::fmt::Display for ExternalStorageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Validated => f.write_str("validated"),
            Self::Unhealthy => f.write_str("unhealthy"),
        }
    }
}
impl ::std::str::FromStr for ExternalStorageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "validated" => Ok(Self::Validated),
            "unhealthy" => Ok(Self::Unhealthy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExternalStorageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExternalStorageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GcpExternalStorageProviderParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GcpExternalStorageProviderParams {
    pub bucket: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: GcpExternalStorageProviderParamsType,
    pub workload_identity_pool_id: ::std::string::String,
    pub workload_identity_project_number: ::std::string::String,
    pub workload_identity_provider_id: ::std::string::String,
}
impl GcpExternalStorageProviderParams {
    pub fn builder() -> builder::GcpExternalStorageProviderParams {
        Default::default()
    }
}
#[doc = "`GcpExternalStorageProviderParamsType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GcpExternalStorageProviderParamsType {
    #[serde(rename = "gcp")]
    Gcp,
}
impl ::std::fmt::Display for GcpExternalStorageProviderParamsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gcp => f.write_str("gcp"),
        }
    }
}
impl ::std::str::FromStr for GcpExternalStorageProviderParamsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gcp" => Ok(Self::Gcp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GcpExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GcpExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GetanexternalstorageconfigurationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetanexternalstorageconfigurationParams {
    pub external_storage_id: ::std::string::String,
}
impl GetanexternalstorageconfigurationParams {
    pub fn builder() -> builder::GetanexternalstorageconfigurationParams {
        Default::default()
    }
}
#[doc = "`GetprojectspendlimitParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetprojectspendlimitParams {
    pub project_id: ::std::string::String,
}
impl GetprojectspendlimitParams {
    pub fn builder() -> builder::GetprojectspendlimitParams {
        Default::default()
    }
}
#[doc = "`ListexternalstorageconfigurationsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListexternalstorageconfigurationsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ExternalStorageOrder>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
}
impl ListexternalstorageconfigurationsParams {
    pub fn builder() -> builder::ListexternalstorageconfigurationsParams {
        Default::default()
    }
}
#[doc = "`OciExternalStorageProviderParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OciExternalStorageProviderParams {
    pub bucket: ::std::string::String,
    pub region: ::std::string::String,
    pub tenancy_ocid: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: OciExternalStorageProviderParamsType,
}
impl OciExternalStorageProviderParams {
    pub fn builder() -> builder::OciExternalStorageProviderParams {
        Default::default()
    }
}
#[doc = "`OciExternalStorageProviderParamsType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OciExternalStorageProviderParamsType {
    #[serde(rename = "oci")]
    Oci,
}
impl ::std::fmt::Display for OciExternalStorageProviderParamsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Oci => f.write_str("oci"),
        }
    }
}
impl ::std::str::FromStr for OciExternalStorageProviderParamsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "oci" => Ok(Self::Oci),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OciExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OciExternalStorageProviderParamsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`OrganizationDecisionsusageParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationDecisionsusageParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub batch: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bucket_width: ::std::option::Option<DecisionsUsageBucketWidth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_time: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_by: ::std::option::Option<::std::vec::Vec<DecisionsUsageGroupBy>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub models: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub start_time: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl OrganizationDecisionsusageParams {
    pub fn builder() -> builder::OrganizationDecisionsusageParams {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting an organization hard spend limit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationSpendLimitDeletedResource {
    #[doc = "Whether the hard spend limit was deleted."]
    pub deleted: bool,
    #[doc = "The object type, which is always `organization.spend_limit.deleted`."]
    pub object: OrganizationSpendLimitDeletedResourceObject,
}
impl OrganizationSpendLimitDeletedResource {
    pub fn builder() -> builder::OrganizationSpendLimitDeletedResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.spend_limit.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OrganizationSpendLimitDeletedResourceObject {
    #[serde(rename = "organization.spend_limit.deleted")]
    OrganizationSpendLimitDeleted,
}
impl ::std::fmt::Display for OrganizationSpendLimitDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationSpendLimitDeleted => f.write_str("organization.spend_limit.deleted"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendLimitDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.spend_limit.deleted" => Ok(Self::OrganizationSpendLimitDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendLimitDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for OrganizationSpendLimitDeletedResourceObject
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a hard spend limit configured at the organization level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationSpendLimitResource {
    #[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
    pub currency: SpendLimitCurrency,
    #[doc = "The current enforcement state of the hard spend limit."]
    pub enforcement: SpendLimitEnforcement,
    #[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
    pub interval: SpendLimitInterval,
    #[doc = "The object type, which is always `organization.spend_limit`."]
    pub object: OrganizationSpendLimitResourceObject,
    #[doc = "The hard spend limit amount, in cents."]
    pub threshold_amount: i64,
}
impl OrganizationSpendLimitResource {
    pub fn builder() -> builder::OrganizationSpendLimitResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.spend_limit`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OrganizationSpendLimitResourceObject {
    #[serde(rename = "organization.spend_limit")]
    OrganizationSpendLimit,
}
impl ::std::fmt::Display for OrganizationSpendLimitResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationSpendLimit => f.write_str("organization.spend_limit"),
        }
    }
}
impl ::std::str::FromStr for OrganizationSpendLimitResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.spend_limit" => Ok(Self::OrganizationSpendLimit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationSpendLimitResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationSpendLimitResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Confirmation payload returned after deleting a project hard spend limit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectSpendLimitDeletedResource {
    #[doc = "Whether the hard spend limit was deleted."]
    pub deleted: bool,
    #[doc = "The object type, which is always `project.spend_limit.deleted`."]
    pub object: ProjectSpendLimitDeletedResourceObject,
}
impl ProjectSpendLimitDeletedResource {
    pub fn builder() -> builder::ProjectSpendLimitDeletedResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `project.spend_limit.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectSpendLimitDeletedResourceObject {
    #[serde(rename = "project.spend_limit.deleted")]
    ProjectSpendLimitDeleted,
}
impl ::std::fmt::Display for ProjectSpendLimitDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectSpendLimitDeleted => f.write_str("project.spend_limit.deleted"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendLimitDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.spend_limit.deleted" => Ok(Self::ProjectSpendLimitDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendLimitDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendLimitDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a hard spend limit configured at the project level."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectSpendLimitResource {
    #[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
    pub currency: SpendLimitCurrency,
    #[doc = "The current enforcement state of the hard spend limit."]
    pub enforcement: SpendLimitEnforcement,
    #[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
    pub interval: SpendLimitInterval,
    #[doc = "The object type, which is always `project.spend_limit`."]
    pub object: ProjectSpendLimitResourceObject,
    #[doc = "The hard spend limit amount, in cents."]
    pub threshold_amount: i64,
}
impl ProjectSpendLimitResource {
    pub fn builder() -> builder::ProjectSpendLimitResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `project.spend_limit`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectSpendLimitResourceObject {
    #[serde(rename = "project.spend_limit")]
    ProjectSpendLimit,
}
impl ::std::fmt::Display for ProjectSpendLimitResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProjectSpendLimit => f.write_str("project.spend_limit"),
        }
    }
}
impl ::std::str::FromStr for ProjectSpendLimitResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "project.spend_limit" => Ok(Self::ProjectSpendLimit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectSpendLimitResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectSpendLimitResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PublicDecisionsUsageResponseResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicDecisionsUsageResponseResource {
    pub data: ::std::vec::Vec<DecisionsUsageBucket>,
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next_page: ::std::option::Option<::std::string::String>,
    pub object: PublicDecisionsUsageResponseResourceObject,
}
impl PublicDecisionsUsageResponseResource {
    pub fn builder() -> builder::PublicDecisionsUsageResponseResource {
        Default::default()
    }
}
#[doc = "`PublicDecisionsUsageResponseResourceObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PublicDecisionsUsageResponseResourceObject {
    #[serde(rename = "page")]
    Page,
}
impl ::std::fmt::Display for PublicDecisionsUsageResponseResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Page => f.write_str("page"),
        }
    }
}
impl ::std::str::FromStr for PublicDecisionsUsageResponseResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "page" => Ok(Self::Page),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PublicDecisionsUsageResponseResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicDecisionsUsageResponseResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ServiceAccountApiKeyBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ServiceAccountApiKeyBody {
    #[doc = "The Unix timestamp (in seconds) when the API key was created."]
    pub created_at: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The identifier of the API key."]
    pub id: ::std::string::String,
    #[doc = "The name of the API key."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always `organization.project.service_account.api_key`"]
    pub object: ServiceAccountApiKeyBodyObject,
    #[doc = "The unredacted API key value."]
    pub value: ::std::string::String,
}
impl ServiceAccountApiKeyBody {
    pub fn builder() -> builder::ServiceAccountApiKeyBody {
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
pub enum ServiceAccountApiKeyBodyObject {
    #[serde(rename = "organization.project.service_account.api_key")]
    OrganizationProjectServiceAccountApiKey,
}
impl ::std::fmt::Display for ServiceAccountApiKeyBodyObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationProjectServiceAccountApiKey => {
                f.write_str("organization.project.service_account.api_key")
            }
        }
    }
}
impl ::std::str::FromStr for ServiceAccountApiKeyBodyObject {
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
impl ::std::convert::TryFrom<&str> for ServiceAccountApiKeyBodyObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ServiceAccountApiKeyBodyObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SpendLimitCurrency`"]
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
pub struct SpendLimitCurrency(pub ::std::string::String);
impl ::std::ops::Deref for SpendLimitCurrency {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SpendLimitCurrency> for ::std::string::String {
    fn from(value: SpendLimitCurrency) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SpendLimitCurrency {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SpendLimitCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SpendLimitCurrency {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "The current enforcement state of a hard spend limit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct SpendLimitEnforcement {
    #[doc = "Whether the hard spend limit is currently enforcing."]
    pub status: SpendLimitEnforcementStatus,
}
impl SpendLimitEnforcement {
    pub fn builder() -> builder::SpendLimitEnforcement {
        Default::default()
    }
}
#[doc = "`SpendLimitEnforcementStatus`"]
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
pub struct SpendLimitEnforcementStatus(pub ::std::string::String);
impl ::std::ops::Deref for SpendLimitEnforcementStatus {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SpendLimitEnforcementStatus> for ::std::string::String {
    fn from(value: SpendLimitEnforcementStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SpendLimitEnforcementStatus {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SpendLimitEnforcementStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SpendLimitEnforcementStatus {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "`SpendLimitInterval`"]
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
pub struct SpendLimitInterval(pub ::std::string::String);
impl ::std::ops::Deref for SpendLimitInterval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SpendLimitInterval> for ::std::string::String {
    fn from(value: SpendLimitInterval) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SpendLimitInterval {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SpendLimitInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SpendLimitInterval {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "`UncategorizedApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UncategorizedApiSchema {
    #[serde(rename = "AdminApiKeysCreateError400")]
    pub admin_api_keys_create_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AdminApiKeysCreateError403")]
    pub admin_api_keys_create_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AdminApiKeysCreateError404")]
    pub admin_api_keys_create_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AdminApiKeysCreateRequest")]
    pub admin_api_keys_create_request: AdminApiKeysCreateRequest,
    #[serde(rename = "AdminApiKeysCreateResponse")]
    pub admin_api_keys_create_response: AdminApiKeyCreateResponse,
    #[serde(rename = "AdminApiKeysDeleteError403")]
    pub admin_api_keys_delete_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AdminApiKeysDeleteError404")]
    pub admin_api_keys_delete_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AdminApiKeysDeleteParams")]
    pub admin_api_keys_delete_params: AdminApiKeysDeleteParams,
    #[serde(rename = "AdminApiKeysDeleteResponse")]
    pub admin_api_keys_delete_response: AdminApiKeysDeleteResponse,
    #[serde(rename = "AdminApiKeysGetError403")]
    pub admin_api_keys_get_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AdminApiKeysGetError404")]
    pub admin_api_keys_get_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AdminApiKeysGetParams")]
    pub admin_api_keys_get_params: AdminApiKeysGetParams,
    #[serde(rename = "AdminApiKeysGetResponse")]
    pub admin_api_keys_get_response: AdminApiKey,
    #[serde(rename = "AdminApiKeysListError400")]
    pub admin_api_keys_list_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AdminApiKeysListError403")]
    pub admin_api_keys_list_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AdminApiKeysListParams")]
    pub admin_api_keys_list_params: AdminApiKeysListParams,
    #[serde(rename = "AdminApiKeysListResponse")]
    pub admin_api_keys_list_response: ApiKeyList,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountError400")]
    pub createan_ap_ikeyforaserviceaccount_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountError403")]
    pub createan_ap_ikeyforaserviceaccount_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountError404")]
    pub createan_ap_ikeyforaserviceaccount_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountError409")]
    pub createan_ap_ikeyforaserviceaccount_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountError429")]
    pub createan_ap_ikeyforaserviceaccount_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountParams")]
    pub createan_ap_ikeyforaserviceaccount_params: CreateanApIkeyforaserviceaccountParams,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountRequest")]
    pub createan_ap_ikeyforaserviceaccount_request: CreateProjectServiceAccountApiKeyBody,
    #[serde(rename = "CreateanAPIkeyforaserviceaccountResponse")]
    pub createan_ap_ikeyforaserviceaccount_response: ServiceAccountApiKeyBody,
    #[serde(rename = "CreateanexternalstorageconfigurationError400")]
    pub createanexternalstorageconfiguration_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanexternalstorageconfigurationError403")]
    pub createanexternalstorageconfiguration_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanexternalstorageconfigurationError404")]
    pub createanexternalstorageconfiguration_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanexternalstorageconfigurationError503")]
    pub createanexternalstorageconfiguration_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateanexternalstorageconfigurationRequest")]
    pub createanexternalstorageconfiguration_request: CreateExternalStorageBody,
    #[serde(rename = "CreateanexternalstorageconfigurationResponse")]
    pub createanexternalstorageconfiguration_response: ExternalStorageResponse,
    #[serde(rename = "DeleteanexternalstorageconfigurationError400")]
    pub deleteanexternalstorageconfiguration_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteanexternalstorageconfigurationError403")]
    pub deleteanexternalstorageconfiguration_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteanexternalstorageconfigurationError404")]
    pub deleteanexternalstorageconfiguration_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteanexternalstorageconfigurationParams")]
    pub deleteanexternalstorageconfiguration_params: DeleteanexternalstorageconfigurationParams,
    #[serde(rename = "DeleteanexternalstorageconfigurationResponse")]
    pub deleteanexternalstorageconfiguration_response: ExternalStorageDeletedResource,
    #[serde(rename = "DeleteorganizationspendlimitError403")]
    pub deleteorganizationspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteorganizationspendlimitError404")]
    pub deleteorganizationspendlimit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteorganizationspendlimitResponse")]
    pub deleteorganizationspendlimit_response: OrganizationSpendLimitDeletedResource,
    #[serde(rename = "DeleteprojectspendlimitError400")]
    pub deleteprojectspendlimit_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteprojectspendlimitError403")]
    pub deleteprojectspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteprojectspendlimitError404")]
    pub deleteprojectspendlimit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteprojectspendlimitParams")]
    pub deleteprojectspendlimit_params: DeleteprojectspendlimitParams,
    #[serde(rename = "DeleteprojectspendlimitResponse")]
    pub deleteprojectspendlimit_response: ProjectSpendLimitDeletedResource,
    #[serde(rename = "GetanexternalstorageconfigurationError400")]
    pub getanexternalstorageconfiguration_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetanexternalstorageconfigurationError403")]
    pub getanexternalstorageconfiguration_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetanexternalstorageconfigurationError404")]
    pub getanexternalstorageconfiguration_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetanexternalstorageconfigurationParams")]
    pub getanexternalstorageconfiguration_params: GetanexternalstorageconfigurationParams,
    #[serde(rename = "GetanexternalstorageconfigurationResponse")]
    pub getanexternalstorageconfiguration_response: ExternalStorageResponse,
    #[serde(rename = "GetorganizationspendlimitError403")]
    pub getorganizationspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "GetorganizationspendlimitError404")]
    pub getorganizationspendlimit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetorganizationspendlimitResponse")]
    pub getorganizationspendlimit_response: OrganizationSpendLimitResource,
    #[serde(rename = "GetprojectspendlimitError400")]
    pub getprojectspendlimit_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectspendlimitError403")]
    pub getprojectspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "GetprojectspendlimitError404")]
    pub getprojectspendlimit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetprojectspendlimitParams")]
    pub getprojectspendlimit_params: GetprojectspendlimitParams,
    #[serde(rename = "GetprojectspendlimitResponse")]
    pub getprojectspendlimit_response: ProjectSpendLimitResource,
    #[serde(rename = "ListexternalstorageconfigurationsError400")]
    pub listexternalstorageconfigurations_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListexternalstorageconfigurationsError403")]
    pub listexternalstorageconfigurations_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListexternalstorageconfigurationsError404")]
    pub listexternalstorageconfigurations_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListexternalstorageconfigurationsParams")]
    pub listexternalstorageconfigurations_params: ListexternalstorageconfigurationsParams,
    #[serde(rename = "ListexternalstorageconfigurationsResponse")]
    pub listexternalstorageconfigurations_response: ExternalStorageListResource,
    #[serde(rename = "OrganizationDecisionsusageError400")]
    pub organization_decisionsusage_error400: DecisionsUsageErrorResponse,
    #[serde(rename = "OrganizationDecisionsusageError403")]
    pub organization_decisionsusage_error403: DecisionsUsagePermissionErrorResponse,
    #[serde(rename = "OrganizationDecisionsusageError429")]
    pub organization_decisionsusage_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "OrganizationDecisionsusageParams")]
    pub organization_decisionsusage_params: OrganizationDecisionsusageParams,
    #[serde(rename = "OrganizationDecisionsusageResponse")]
    pub organization_decisionsusage_response: PublicDecisionsUsageResponseResource,
    #[serde(rename = "UpdateorganizationspendlimitError400")]
    pub updateorganizationspendlimit_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateorganizationspendlimitError403")]
    pub updateorganizationspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateorganizationspendlimitRequest")]
    pub updateorganizationspendlimit_request: UpdateOrganizationSpendLimitBody,
    #[serde(rename = "UpdateorganizationspendlimitResponse")]
    pub updateorganizationspendlimit_response: OrganizationSpendLimitResource,
    #[serde(rename = "UpdateprojectspendlimitError400")]
    pub updateprojectspendlimit_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateprojectspendlimitError403")]
    pub updateprojectspendlimit_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UpdateprojectspendlimitError404")]
    pub updateprojectspendlimit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UpdateprojectspendlimitParams")]
    pub updateprojectspendlimit_params: UpdateprojectspendlimitParams,
    #[serde(rename = "UpdateprojectspendlimitRequest")]
    pub updateprojectspendlimit_request: UpdateProjectSpendLimitBody,
    #[serde(rename = "UpdateprojectspendlimitResponse")]
    pub updateprojectspendlimit_response: ProjectSpendLimitResource,
    #[serde(rename = "ValidateanexternalstorageconfigurationError400")]
    pub validateanexternalstorageconfiguration_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ValidateanexternalstorageconfigurationError403")]
    pub validateanexternalstorageconfiguration_error403: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ValidateanexternalstorageconfigurationError404")]
    pub validateanexternalstorageconfiguration_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ValidateanexternalstorageconfigurationError503")]
    pub validateanexternalstorageconfiguration_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ValidateanexternalstorageconfigurationParams")]
    pub validateanexternalstorageconfiguration_params: ValidateanexternalstorageconfigurationParams,
    #[serde(rename = "ValidateanexternalstorageconfigurationResponse")]
    pub validateanexternalstorageconfiguration_response: ExternalStorageResponse,
}
impl UncategorizedApiSchema {
    pub fn builder() -> builder::UncategorizedApiSchema {
        Default::default()
    }
}
#[doc = "Parameters for the hard spend limit you want to create or replace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateOrganizationSpendLimitBody {
    #[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
    pub currency: UpdateOrganizationSpendLimitBodyCurrency,
    #[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
    pub interval: UpdateOrganizationSpendLimitBodyInterval,
    #[doc = "The hard spend limit amount, in cents."]
    pub threshold_amount: ::std::num::NonZeroU64,
}
impl UpdateOrganizationSpendLimitBody {
    pub fn builder() -> builder::UpdateOrganizationSpendLimitBody {
        Default::default()
    }
}
#[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateOrganizationSpendLimitBodyCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl ::std::fmt::Display for UpdateOrganizationSpendLimitBodyCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Usd => f.write_str("USD"),
        }
    }
}
impl ::std::str::FromStr for UpdateOrganizationSpendLimitBodyCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateOrganizationSpendLimitBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateOrganizationSpendLimitBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateOrganizationSpendLimitBodyInterval {
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for UpdateOrganizationSpendLimitBodyInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for UpdateOrganizationSpendLimitBodyInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateOrganizationSpendLimitBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateOrganizationSpendLimitBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Parameters for the hard spend limit you want to create or replace."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateProjectSpendLimitBody {
    #[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
    pub currency: UpdateProjectSpendLimitBodyCurrency,
    #[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
    pub interval: UpdateProjectSpendLimitBodyInterval,
    #[doc = "The hard spend limit amount, in cents."]
    pub threshold_amount: ::std::num::NonZeroU64,
}
impl UpdateProjectSpendLimitBody {
    pub fn builder() -> builder::UpdateProjectSpendLimitBody {
        Default::default()
    }
}
#[doc = "The currency for the threshold amount. Currently, only `USD` is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateProjectSpendLimitBodyCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl ::std::fmt::Display for UpdateProjectSpendLimitBodyCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Usd => f.write_str("USD"),
        }
    }
}
impl ::std::str::FromStr for UpdateProjectSpendLimitBodyCurrency {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "USD" => Ok(Self::Usd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateProjectSpendLimitBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateProjectSpendLimitBodyCurrency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The time interval for evaluating spend against the threshold. Currently, only `month` is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UpdateProjectSpendLimitBodyInterval {
    #[serde(rename = "month")]
    Month,
}
impl ::std::fmt::Display for UpdateProjectSpendLimitBodyInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Month => f.write_str("month"),
        }
    }
}
impl ::std::str::FromStr for UpdateProjectSpendLimitBodyInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "month" => Ok(Self::Month),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpdateProjectSpendLimitBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateProjectSpendLimitBodyInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UpdateprojectspendlimitParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UpdateprojectspendlimitParams {
    pub project_id: ::std::string::String,
}
impl UpdateprojectspendlimitParams {
    pub fn builder() -> builder::UpdateprojectspendlimitParams {
        Default::default()
    }
}
#[doc = "`ValidateanexternalstorageconfigurationParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ValidateanexternalstorageconfigurationParams {
    pub external_storage_id: ::std::string::String,
}
impl ValidateanexternalstorageconfigurationParams {
    pub fn builder() -> builder::ValidateanexternalstorageconfigurationParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AdminApiKey {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_used_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::AdminApiKeyObject, ::std::string::String>,
        owner: ::std::result::Result<super::AdminApiKeyOwner, ::std::string::String>,
        redacted_value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKey {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                last_used_at: Ok(Default::default()),
                name: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                owner: Err("no value supplied for owner".to_string()),
                redacted_value: Err("no value supplied for redacted_value".to_string()),
            }
        }
    }
    impl AdminApiKey {
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
            T: ::std::convert::TryInto<super::AdminApiKeyObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeyOwner>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
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
    impl ::std::convert::TryFrom<AdminApiKey> for super::AdminApiKey {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKey,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                id: value.id?,
                last_used_at: value.last_used_at?,
                name: value.name?,
                object: value.object?,
                owner: value.owner?,
                redacted_value: value.redacted_value?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKey> for AdminApiKey {
        fn from(value: super::AdminApiKey) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                last_used_at: Ok(value.last_used_at),
                name: Ok(value.name),
                object: Ok(value.object),
                owner: Ok(value.owner),
                redacted_value: Ok(value.redacted_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeyCreateResponse {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_used_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object:
            ::std::result::Result<super::AdminApiKeyCreateResponseObject, ::std::string::String>,
        owner: ::std::result::Result<super::AdminApiKeyCreateResponseOwner, ::std::string::String>,
        redacted_value: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKeyCreateResponse {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                last_used_at: Ok(Default::default()),
                name: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                owner: Err("no value supplied for owner".to_string()),
                redacted_value: Err("no value supplied for redacted_value".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AdminApiKeyCreateResponse {
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
            T: ::std::convert::TryInto<super::AdminApiKeyCreateResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeyCreateResponseOwner>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
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
    impl ::std::convert::TryFrom<AdminApiKeyCreateResponse> for super::AdminApiKeyCreateResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeyCreateResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_at: value.expires_at?,
                id: value.id?,
                last_used_at: value.last_used_at?,
                name: value.name?,
                object: value.object?,
                owner: value.owner?,
                redacted_value: value.redacted_value?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeyCreateResponse> for AdminApiKeyCreateResponse {
        fn from(value: super::AdminApiKeyCreateResponse) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                last_used_at: Ok(value.last_used_at),
                name: Ok(value.name),
                object: Ok(value.object),
                owner: Ok(value.owner),
                redacted_value: Ok(value.redacted_value),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeyCreateResponseOwner {
        created_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AdminApiKeyCreateResponseOwner {
        fn default() -> Self {
            Self {
                created_at: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                object: Ok(Default::default()),
                role: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AdminApiKeyCreateResponseOwner {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    impl ::std::convert::TryFrom<AdminApiKeyCreateResponseOwner>
        for super::AdminApiKeyCreateResponseOwner
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeyCreateResponseOwner,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                role: value.role?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeyCreateResponseOwner>
        for AdminApiKeyCreateResponseOwner
    {
        fn from(value: super::AdminApiKeyCreateResponseOwner) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                role: Ok(value.role),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeyOwner {
        created_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AdminApiKeyOwner {
        fn default() -> Self {
            Self {
                created_at: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                object: Ok(Default::default()),
                role: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AdminApiKeyOwner {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    impl ::std::convert::TryFrom<AdminApiKeyOwner> for super::AdminApiKeyOwner {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeyOwner,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                role: value.role?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeyOwner> for AdminApiKeyOwner {
        fn from(value: super::AdminApiKeyOwner) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                role: Ok(value.role),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeysCreateRequest {
        expires_in_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKeysCreateRequest {
        fn default() -> Self {
            Self {
                expires_in_seconds: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl AdminApiKeysCreateRequest {
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
    impl ::std::convert::TryFrom<AdminApiKeysCreateRequest> for super::AdminApiKeysCreateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeysCreateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_in_seconds: value.expires_in_seconds?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeysCreateRequest> for AdminApiKeysCreateRequest {
        fn from(value: super::AdminApiKeysCreateRequest) -> Self {
            Self {
                expires_in_seconds: Ok(value.expires_in_seconds),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeysDeleteParams {
        key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKeysDeleteParams {
        fn default() -> Self {
            Self {
                key_id: Err("no value supplied for key_id".to_string()),
            }
        }
    }
    impl AdminApiKeysDeleteParams {
        pub fn key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AdminApiKeysDeleteParams> for super::AdminApiKeysDeleteParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeysDeleteParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key_id: value.key_id?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeysDeleteParams> for AdminApiKeysDeleteParams {
        fn from(value: super::AdminApiKeysDeleteParams) -> Self {
            Self {
                key_id: Ok(value.key_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeysDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::AdminApiKeysDeleteResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKeysDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl AdminApiKeysDeleteResponse {
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
            T: ::std::convert::TryInto<super::AdminApiKeysDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AdminApiKeysDeleteResponse> for super::AdminApiKeysDeleteResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeysDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeysDeleteResponse> for AdminApiKeysDeleteResponse {
        fn from(value: super::AdminApiKeysDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeysGetParams {
        key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AdminApiKeysGetParams {
        fn default() -> Self {
            Self {
                key_id: Err("no value supplied for key_id".to_string()),
            }
        }
    }
    impl AdminApiKeysGetParams {
        pub fn key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AdminApiKeysGetParams> for super::AdminApiKeysGetParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeysGetParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key_id: value.key_id?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeysGetParams> for AdminApiKeysGetParams {
        fn from(value: super::AdminApiKeysGetParams) -> Self {
            Self {
                key_id: Ok(value.key_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdminApiKeysListParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::AdminApiKeysListParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AdminApiKeysListParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl AdminApiKeysListParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::AdminApiKeysListParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AdminApiKeysListParams> for super::AdminApiKeysListParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AdminApiKeysListParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::AdminApiKeysListParams> for AdminApiKeysListParams {
        fn from(value: super::AdminApiKeysListParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ApiKeyList {
        data: ::std::result::Result<::std::vec::Vec<super::AdminApiKey>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ApiKeyListObject, ::std::string::String>,
    }
    impl ::std::default::Default for ApiKeyList {
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
    impl ApiKeyList {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AdminApiKey>>,
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
            T: ::std::convert::TryInto<super::ApiKeyListObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ApiKeyList> for super::ApiKeyList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ApiKeyList,
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
    impl ::std::convert::From<super::ApiKeyList> for ApiKeyList {
        fn from(value: super::ApiKeyList) -> Self {
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
    pub struct AwsExternalStorageProviderParams {
        bucket: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_arn: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::AwsExternalStorageProviderParamsType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AwsExternalStorageProviderParams {
        fn default() -> Self {
            Self {
                bucket: Err("no value supplied for bucket".to_string()),
                role_arn: Err("no value supplied for role_arn".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AwsExternalStorageProviderParams {
        pub fn bucket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket: {e}"));
            self
        }
        pub fn role_arn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_arn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_arn: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AwsExternalStorageProviderParamsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AwsExternalStorageProviderParams>
        for super::AwsExternalStorageProviderParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AwsExternalStorageProviderParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bucket: value.bucket?,
                role_arn: value.role_arn?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AwsExternalStorageProviderParams>
        for AwsExternalStorageProviderParams
    {
        fn from(value: super::AwsExternalStorageProviderParams) -> Self {
            Self {
                bucket: Ok(value.bucket),
                role_arn: Ok(value.role_arn),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AzureExternalStorageProviderParams {
        account_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        container: ::std::result::Result<::std::string::String, ::std::string::String>,
        resource_group: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscription_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::AzureExternalStorageProviderParamsType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AzureExternalStorageProviderParams {
        fn default() -> Self {
            Self {
                account_name: Err("no value supplied for account_name".to_string()),
                container: Err("no value supplied for container".to_string()),
                resource_group: Err("no value supplied for resource_group".to_string()),
                subscription_id: Err("no value supplied for subscription_id".to_string()),
                tenant_id: Err("no value supplied for tenant_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AzureExternalStorageProviderParams {
        pub fn account_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account_name: {e}"));
            self
        }
        pub fn container<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.container = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for container: {e}"));
            self
        }
        pub fn resource_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_group = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_group: {e}"));
            self
        }
        pub fn subscription_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscription_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscription_id: {e}"));
            self
        }
        pub fn tenant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AzureExternalStorageProviderParamsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AzureExternalStorageProviderParams>
        for super::AzureExternalStorageProviderParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AzureExternalStorageProviderParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account_name: value.account_name?,
                container: value.container?,
                resource_group: value.resource_group?,
                subscription_id: value.subscription_id?,
                tenant_id: value.tenant_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AzureExternalStorageProviderParams>
        for AzureExternalStorageProviderParams
    {
        fn from(value: super::AzureExternalStorageProviderParams) -> Self {
            Self {
                account_name: Ok(value.account_name),
                container: Ok(value.container),
                resource_group: Ok(value.resource_group),
                subscription_id: Ok(value.subscription_id),
                tenant_id: Ok(value.tenant_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateExternalStorageBody {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        provider:
            ::std::result::Result<super::CreateExternalStorageBodyProvider, ::std::string::String>,
    }
    impl ::std::default::Default for CreateExternalStorageBody {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                provider: Err("no value supplied for provider".to_string()),
            }
        }
    }
    impl CreateExternalStorageBody {
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
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateExternalStorageBodyProvider>,
            T::Error: ::std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateExternalStorageBody> for super::CreateExternalStorageBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateExternalStorageBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                provider: value.provider?,
            })
        }
    }
    impl ::std::convert::From<super::CreateExternalStorageBody> for CreateExternalStorageBody {
        fn from(value: super::CreateExternalStorageBody) -> Self {
            Self {
                project_id: Ok(value.project_id),
                provider: Ok(value.provider),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateProjectServiceAccountApiKeyBody {
        expires_in_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scopes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateProjectServiceAccountApiKeyBody {
        fn default() -> Self {
            Self {
                expires_in_seconds: Ok(Default::default()),
                name: Ok(Default::default()),
                scopes: Ok(Default::default()),
            }
        }
    }
    impl CreateProjectServiceAccountApiKeyBody {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateProjectServiceAccountApiKeyBody>
        for super::CreateProjectServiceAccountApiKeyBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateProjectServiceAccountApiKeyBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_in_seconds: value.expires_in_seconds?,
                name: value.name?,
                scopes: value.scopes?,
            })
        }
    }
    impl ::std::convert::From<super::CreateProjectServiceAccountApiKeyBody>
        for CreateProjectServiceAccountApiKeyBody
    {
        fn from(value: super::CreateProjectServiceAccountApiKeyBody) -> Self {
            Self {
                expires_in_seconds: Ok(value.expires_in_seconds),
                name: Ok(value.name),
                scopes: Ok(value.scopes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateanApIkeyforaserviceaccountParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        service_account_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateanApIkeyforaserviceaccountParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                service_account_id: Err("no value supplied for service_account_id".to_string()),
            }
        }
    }
    impl CreateanApIkeyforaserviceaccountParams {
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
    impl ::std::convert::TryFrom<CreateanApIkeyforaserviceaccountParams>
        for super::CreateanApIkeyforaserviceaccountParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateanApIkeyforaserviceaccountParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                service_account_id: value.service_account_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateanApIkeyforaserviceaccountParams>
        for CreateanApIkeyforaserviceaccountParams
    {
        fn from(value: super::CreateanApIkeyforaserviceaccountParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                service_account_id: Ok(value.service_account_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsUsageBucket {
        end_time: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::DecisionsUsageBucketObject, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::DecisionsUsageResult>,
            ::std::string::String,
        >,
        start_time: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionsUsageBucket {
        fn default() -> Self {
            Self {
                end_time: Err("no value supplied for end_time".to_string()),
                object: Err("no value supplied for object".to_string()),
                results: Err("no value supplied for results".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl DecisionsUsageBucket {
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsageBucketObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DecisionsUsageResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionsUsageBucket> for super::DecisionsUsageBucket {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsUsageBucket,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_time: value.end_time?,
                object: value.object?,
                results: value.results?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsUsageBucket> for DecisionsUsageBucket {
        fn from(value: super::DecisionsUsageBucket) -> Self {
            Self {
                end_time: Ok(value.end_time),
                object: Ok(value.object),
                results: Ok(value.results),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsUsageErrorDetails {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        param: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionsUsageErrorDetails {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
                param: Err("no value supplied for param".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DecisionsUsageErrorDetails {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    impl ::std::convert::TryFrom<DecisionsUsageErrorDetails> for super::DecisionsUsageErrorDetails {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsUsageErrorDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
                param: value.param?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsUsageErrorDetails> for DecisionsUsageErrorDetails {
        fn from(value: super::DecisionsUsageErrorDetails) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
                param: Ok(value.param),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsUsageErrorResponse {
        error: ::std::result::Result<super::DecisionsUsageErrorDetails, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionsUsageErrorResponse {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
            }
        }
    }
    impl DecisionsUsageErrorResponse {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsageErrorDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionsUsageErrorResponse> for super::DecisionsUsageErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsUsageErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsUsageErrorResponse> for DecisionsUsageErrorResponse {
        fn from(value: super::DecisionsUsageErrorResponse) -> Self {
            Self {
                error: Ok(value.error),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsUsagePermissionErrorResponse {
        error: ::std::result::Result<
            super::DecisionsUsagePermissionErrorResponseError,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionsUsagePermissionErrorResponse {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
            }
        }
    }
    impl DecisionsUsagePermissionErrorResponse {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsagePermissionErrorResponseError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionsUsagePermissionErrorResponse>
        for super::DecisionsUsagePermissionErrorResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsUsagePermissionErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsUsagePermissionErrorResponse>
        for DecisionsUsagePermissionErrorResponse
    {
        fn from(value: super::DecisionsUsagePermissionErrorResponse) -> Self {
            Self {
                error: Ok(value.error),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsUsageResult {
        api_key_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        api_source: ::std::result::Result<
            ::std::option::Option<super::DecisionsUsageApiSource>,
            ::std::string::String,
        >,
        batch: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        input_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cache_write_12h_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cache_write_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_text_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_cached_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_text_tokens: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        input_uncached_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        num_model_requests: ::std::result::Result<i64, ::std::string::String>,
        object: ::std::result::Result<super::DecisionsUsageResultObject, ::std::string::String>,
        output_audio_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_image_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_text_tokens:
            ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        output_tokens: ::std::result::Result<i64, ::std::string::String>,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        service_tier: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionsUsageResult {
        fn default() -> Self {
            Self {
                api_key_id: Ok(Default::default()),
                api_source: Ok(Default::default()),
                batch: Ok(Default::default()),
                input_audio_tokens: Ok(Default::default()),
                input_cache_write_12h_tokens: Ok(Default::default()),
                input_cache_write_tokens: Ok(Default::default()),
                input_cached_audio_tokens: Ok(Default::default()),
                input_cached_image_tokens: Ok(Default::default()),
                input_cached_text_tokens: Ok(Default::default()),
                input_cached_tokens: Ok(Default::default()),
                input_image_tokens: Ok(Default::default()),
                input_text_tokens: Ok(Default::default()),
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                input_uncached_tokens: Ok(Default::default()),
                model: Ok(Default::default()),
                num_model_requests: Err("no value supplied for num_model_requests".to_string()),
                object: Err("no value supplied for object".to_string()),
                output_audio_tokens: Ok(Default::default()),
                output_image_tokens: Ok(Default::default()),
                output_text_tokens: Ok(Default::default()),
                output_tokens: Err("no value supplied for output_tokens".to_string()),
                project_id: Ok(Default::default()),
                service_tier: Ok(Default::default()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl DecisionsUsageResult {
        pub fn api_key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_id: {e}"));
            self
        }
        pub fn api_source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DecisionsUsageApiSource>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_source: {e}"));
            self
        }
        pub fn batch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch: {e}"));
            self
        }
        pub fn input_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_audio_tokens: {e}")
            });
            self
        }
        pub fn input_cache_write_12h_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cache_write_12h_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cache_write_12h_tokens: {e}")
            });
            self
        }
        pub fn input_cache_write_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cache_write_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cache_write_tokens: {e}")
            });
            self
        }
        pub fn input_cached_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_audio_tokens: {e}")
            });
            self
        }
        pub fn input_cached_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_image_tokens: {e}")
            });
            self
        }
        pub fn input_cached_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_text_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_text_tokens: {e}")
            });
            self
        }
        pub fn input_cached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_cached_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_cached_tokens: {e}")
            });
            self
        }
        pub fn input_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_image_tokens: {e}")
            });
            self
        }
        pub fn input_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_text_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_text_tokens: {e}"));
            self
        }
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn input_uncached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_uncached_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_uncached_tokens: {e}")
            });
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
        pub fn num_model_requests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_model_requests = value.try_into().map_err(|e| {
                format!("error converting supplied value for num_model_requests: {e}")
            });
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsageResultObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn output_audio_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_audio_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_audio_tokens: {e}")
            });
            self
        }
        pub fn output_image_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_image_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_image_tokens: {e}")
            });
            self
        }
        pub fn output_text_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_text_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_text_tokens: {e}")
            });
            self
        }
        pub fn output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_tokens: {e}"));
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
        pub fn service_tier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_tier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_tier: {e}"));
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
    impl ::std::convert::TryFrom<DecisionsUsageResult> for super::DecisionsUsageResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsUsageResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_id: value.api_key_id?,
                api_source: value.api_source?,
                batch: value.batch?,
                input_audio_tokens: value.input_audio_tokens?,
                input_cache_write_12h_tokens: value.input_cache_write_12h_tokens?,
                input_cache_write_tokens: value.input_cache_write_tokens?,
                input_cached_audio_tokens: value.input_cached_audio_tokens?,
                input_cached_image_tokens: value.input_cached_image_tokens?,
                input_cached_text_tokens: value.input_cached_text_tokens?,
                input_cached_tokens: value.input_cached_tokens?,
                input_image_tokens: value.input_image_tokens?,
                input_text_tokens: value.input_text_tokens?,
                input_tokens: value.input_tokens?,
                input_uncached_tokens: value.input_uncached_tokens?,
                model: value.model?,
                num_model_requests: value.num_model_requests?,
                object: value.object?,
                output_audio_tokens: value.output_audio_tokens?,
                output_image_tokens: value.output_image_tokens?,
                output_text_tokens: value.output_text_tokens?,
                output_tokens: value.output_tokens?,
                project_id: value.project_id?,
                service_tier: value.service_tier?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsUsageResult> for DecisionsUsageResult {
        fn from(value: super::DecisionsUsageResult) -> Self {
            Self {
                api_key_id: Ok(value.api_key_id),
                api_source: Ok(value.api_source),
                batch: Ok(value.batch),
                input_audio_tokens: Ok(value.input_audio_tokens),
                input_cache_write_12h_tokens: Ok(value.input_cache_write_12h_tokens),
                input_cache_write_tokens: Ok(value.input_cache_write_tokens),
                input_cached_audio_tokens: Ok(value.input_cached_audio_tokens),
                input_cached_image_tokens: Ok(value.input_cached_image_tokens),
                input_cached_text_tokens: Ok(value.input_cached_text_tokens),
                input_cached_tokens: Ok(value.input_cached_tokens),
                input_image_tokens: Ok(value.input_image_tokens),
                input_text_tokens: Ok(value.input_text_tokens),
                input_tokens: Ok(value.input_tokens),
                input_uncached_tokens: Ok(value.input_uncached_tokens),
                model: Ok(value.model),
                num_model_requests: Ok(value.num_model_requests),
                object: Ok(value.object),
                output_audio_tokens: Ok(value.output_audio_tokens),
                output_image_tokens: Ok(value.output_image_tokens),
                output_text_tokens: Ok(value.output_text_tokens),
                output_tokens: Ok(value.output_tokens),
                project_id: Ok(value.project_id),
                service_tier: Ok(value.service_tier),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteanexternalstorageconfigurationParams {
        external_storage_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteanexternalstorageconfigurationParams {
        fn default() -> Self {
            Self {
                external_storage_id: Err("no value supplied for external_storage_id".to_string()),
            }
        }
    }
    impl DeleteanexternalstorageconfigurationParams {
        pub fn external_storage_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.external_storage_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_storage_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteanexternalstorageconfigurationParams>
        for super::DeleteanexternalstorageconfigurationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteanexternalstorageconfigurationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                external_storage_id: value.external_storage_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteanexternalstorageconfigurationParams>
        for DeleteanexternalstorageconfigurationParams
    {
        fn from(value: super::DeleteanexternalstorageconfigurationParams) -> Self {
            Self {
                external_storage_id: Ok(value.external_storage_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteprojectspendlimitParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteprojectspendlimitParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl DeleteprojectspendlimitParams {
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
    impl ::std::convert::TryFrom<DeleteprojectspendlimitParams>
        for super::DeleteprojectspendlimitParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteprojectspendlimitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteprojectspendlimitParams> for DeleteprojectspendlimitParams {
        fn from(value: super::DeleteprojectspendlimitParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalStorageDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::ExternalStorageDeletedResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ExternalStorageDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ExternalStorageDeletedResource {
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
            T: ::std::convert::TryInto<super::ExternalStorageDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalStorageDeletedResource>
        for super::ExternalStorageDeletedResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalStorageDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalStorageDeletedResource>
        for ExternalStorageDeletedResource
    {
        fn from(value: super::ExternalStorageDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalStorageListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::ExternalStorageResponse>,
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
        object:
            ::std::result::Result<super::ExternalStorageListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ExternalStorageListResource {
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
    impl ExternalStorageListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalStorageResponse>>,
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
            T: ::std::convert::TryInto<super::ExternalStorageListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalStorageListResource> for super::ExternalStorageListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalStorageListResource,
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
    impl ::std::convert::From<super::ExternalStorageListResource> for ExternalStorageListResource {
        fn from(value: super::ExternalStorageListResource) -> Self {
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
    pub struct ExternalStorageResponse {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        geography: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ExternalStorageResponseObject, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        provider:
            ::std::result::Result<super::ExternalStorageResponseProvider, ::std::string::String>,
        status: ::std::result::Result<super::ExternalStorageStatus, ::std::string::String>,
    }
    impl ::std::default::Default for ExternalStorageResponse {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                geography: Err("no value supplied for geography".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
                provider: Err("no value supplied for provider".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl ExternalStorageResponse {
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
        pub fn geography<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.geography = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geography: {e}"));
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
            T: ::std::convert::TryInto<super::ExternalStorageResponseObject>,
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
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageResponseProvider>,
            T::Error: ::std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalStorageResponse> for super::ExternalStorageResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalStorageResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                geography: value.geography?,
                id: value.id?,
                object: value.object?,
                project_id: value.project_id?,
                provider: value.provider?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalStorageResponse> for ExternalStorageResponse {
        fn from(value: super::ExternalStorageResponse) -> Self {
            Self {
                created_at: Ok(value.created_at),
                geography: Ok(value.geography),
                id: Ok(value.id),
                object: Ok(value.object),
                project_id: Ok(value.project_id),
                provider: Ok(value.provider),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GcpExternalStorageProviderParams {
        bucket: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::GcpExternalStorageProviderParamsType,
            ::std::string::String,
        >,
        workload_identity_pool_id:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        workload_identity_project_number:
            ::std::result::Result<::std::string::String, ::std::string::String>,
        workload_identity_provider_id:
            ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GcpExternalStorageProviderParams {
        fn default() -> Self {
            Self {
                bucket: Err("no value supplied for bucket".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                workload_identity_pool_id: Err(
                    "no value supplied for workload_identity_pool_id".to_string()
                ),
                workload_identity_project_number: Err(
                    "no value supplied for workload_identity_project_number".to_string(),
                ),
                workload_identity_provider_id: Err(
                    "no value supplied for workload_identity_provider_id".to_string(),
                ),
            }
        }
    }
    impl GcpExternalStorageProviderParams {
        pub fn bucket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GcpExternalStorageProviderParamsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn workload_identity_pool_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_pool_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for workload_identity_pool_id: {e}")
            });
            self
        }
        pub fn workload_identity_project_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_project_number = value.try_into().map_err(|e| {
                format!("error converting supplied value for workload_identity_project_number: {e}")
            });
            self
        }
        pub fn workload_identity_provider_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_provider_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for workload_identity_provider_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GcpExternalStorageProviderParams>
        for super::GcpExternalStorageProviderParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GcpExternalStorageProviderParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bucket: value.bucket?,
                type_: value.type_?,
                workload_identity_pool_id: value.workload_identity_pool_id?,
                workload_identity_project_number: value.workload_identity_project_number?,
                workload_identity_provider_id: value.workload_identity_provider_id?,
            })
        }
    }
    impl ::std::convert::From<super::GcpExternalStorageProviderParams>
        for GcpExternalStorageProviderParams
    {
        fn from(value: super::GcpExternalStorageProviderParams) -> Self {
            Self {
                bucket: Ok(value.bucket),
                type_: Ok(value.type_),
                workload_identity_pool_id: Ok(value.workload_identity_pool_id),
                workload_identity_project_number: Ok(value.workload_identity_project_number),
                workload_identity_provider_id: Ok(value.workload_identity_provider_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetanexternalstorageconfigurationParams {
        external_storage_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetanexternalstorageconfigurationParams {
        fn default() -> Self {
            Self {
                external_storage_id: Err("no value supplied for external_storage_id".to_string()),
            }
        }
    }
    impl GetanexternalstorageconfigurationParams {
        pub fn external_storage_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.external_storage_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_storage_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GetanexternalstorageconfigurationParams>
        for super::GetanexternalstorageconfigurationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetanexternalstorageconfigurationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                external_storage_id: value.external_storage_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetanexternalstorageconfigurationParams>
        for GetanexternalstorageconfigurationParams
    {
        fn from(value: super::GetanexternalstorageconfigurationParams) -> Self {
            Self {
                external_storage_id: Ok(value.external_storage_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetprojectspendlimitParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetprojectspendlimitParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl GetprojectspendlimitParams {
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
    impl ::std::convert::TryFrom<GetprojectspendlimitParams> for super::GetprojectspendlimitParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetprojectspendlimitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetprojectspendlimitParams> for GetprojectspendlimitParams {
        fn from(value: super::GetprojectspendlimitParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListexternalstorageconfigurationsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<super::ExternalStorageOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListexternalstorageconfigurationsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Ok(Default::default()),
            }
        }
    }
    impl ListexternalstorageconfigurationsParams {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalStorageOrder>>,
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
    impl ::std::convert::TryFrom<ListexternalstorageconfigurationsParams>
        for super::ListexternalstorageconfigurationsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListexternalstorageconfigurationsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListexternalstorageconfigurationsParams>
        for ListexternalstorageconfigurationsParams
    {
        fn from(value: super::ListexternalstorageconfigurationsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OciExternalStorageProviderParams {
        bucket: ::std::result::Result<::std::string::String, ::std::string::String>,
        region: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenancy_ocid: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::OciExternalStorageProviderParamsType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OciExternalStorageProviderParams {
        fn default() -> Self {
            Self {
                bucket: Err("no value supplied for bucket".to_string()),
                region: Err("no value supplied for region".to_string()),
                tenancy_ocid: Err("no value supplied for tenancy_ocid".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OciExternalStorageProviderParams {
        pub fn bucket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket: {e}"));
            self
        }
        pub fn region<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.region = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for region: {e}"));
            self
        }
        pub fn tenancy_ocid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenancy_ocid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenancy_ocid: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OciExternalStorageProviderParamsType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OciExternalStorageProviderParams>
        for super::OciExternalStorageProviderParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OciExternalStorageProviderParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bucket: value.bucket?,
                region: value.region?,
                tenancy_ocid: value.tenancy_ocid?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OciExternalStorageProviderParams>
        for OciExternalStorageProviderParams
    {
        fn from(value: super::OciExternalStorageProviderParams) -> Self {
            Self {
                bucket: Ok(value.bucket),
                region: Ok(value.region),
                tenancy_ocid: Ok(value.tenancy_ocid),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationDecisionsusageParams {
        api_key_ids: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        batch: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        bucket_width: ::std::result::Result<
            ::std::option::Option<super::DecisionsUsageBucketWidth>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        group_by: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::DecisionsUsageGroupBy>>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        models: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_ids: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        start_time: ::std::result::Result<i64, ::std::string::String>,
        user_ids: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationDecisionsusageParams {
        fn default() -> Self {
            Self {
                api_key_ids: Ok(Default::default()),
                batch: Ok(Default::default()),
                bucket_width: Ok(Default::default()),
                end_time: Ok(Default::default()),
                group_by: Ok(Default::default()),
                limit: Ok(Default::default()),
                models: Ok(Default::default()),
                page: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                start_time: Err("no value supplied for start_time".to_string()),
                user_ids: Ok(Default::default()),
            }
        }
    }
    impl OrganizationDecisionsusageParams {
        pub fn api_key_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_ids: {e}"));
            self
        }
        pub fn batch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.batch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for batch: {e}"));
            self
        }
        pub fn bucket_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DecisionsUsageBucketWidth>>,
            T::Error: ::std::fmt::Display,
        {
            self.bucket_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bucket_width: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn group_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<super::DecisionsUsageGroupBy>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.group_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_by: {e}"));
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
        pub fn models<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.models = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for models: {e}"));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {e}"));
            self
        }
        pub fn project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.project_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_ids: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
        pub fn user_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.user_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_ids: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationDecisionsusageParams>
        for super::OrganizationDecisionsusageParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationDecisionsusageParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_ids: value.api_key_ids?,
                batch: value.batch?,
                bucket_width: value.bucket_width?,
                end_time: value.end_time?,
                group_by: value.group_by?,
                limit: value.limit?,
                models: value.models?,
                page: value.page?,
                project_ids: value.project_ids?,
                start_time: value.start_time?,
                user_ids: value.user_ids?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationDecisionsusageParams>
        for OrganizationDecisionsusageParams
    {
        fn from(value: super::OrganizationDecisionsusageParams) -> Self {
            Self {
                api_key_ids: Ok(value.api_key_ids),
                batch: Ok(value.batch),
                bucket_width: Ok(value.bucket_width),
                end_time: Ok(value.end_time),
                group_by: Ok(value.group_by),
                limit: Ok(value.limit),
                models: Ok(value.models),
                page: Ok(value.page),
                project_ids: Ok(value.project_ids),
                start_time: Ok(value.start_time),
                user_ids: Ok(value.user_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationSpendLimitDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<
            super::OrganizationSpendLimitDeletedResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OrganizationSpendLimitDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl OrganizationSpendLimitDeletedResource {
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
            T: ::std::convert::TryInto<super::OrganizationSpendLimitDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationSpendLimitDeletedResource>
        for super::OrganizationSpendLimitDeletedResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationSpendLimitDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationSpendLimitDeletedResource>
        for OrganizationSpendLimitDeletedResource
    {
        fn from(value: super::OrganizationSpendLimitDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganizationSpendLimitResource {
        currency: ::std::result::Result<super::SpendLimitCurrency, ::std::string::String>,
        enforcement: ::std::result::Result<super::SpendLimitEnforcement, ::std::string::String>,
        interval: ::std::result::Result<super::SpendLimitInterval, ::std::string::String>,
        object: ::std::result::Result<
            super::OrganizationSpendLimitResourceObject,
            ::std::string::String,
        >,
        threshold_amount: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for OrganizationSpendLimitResource {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                enforcement: Err("no value supplied for enforcement".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                object: Err("no value supplied for object".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl OrganizationSpendLimitResource {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn enforcement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitEnforcement>,
            T::Error: ::std::fmt::Display,
        {
            self.enforcement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enforcement: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendLimitResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganizationSpendLimitResource>
        for super::OrganizationSpendLimitResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganizationSpendLimitResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                enforcement: value.enforcement?,
                interval: value.interval?,
                object: value.object?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::OrganizationSpendLimitResource>
        for OrganizationSpendLimitResource
    {
        fn from(value: super::OrganizationSpendLimitResource) -> Self {
            Self {
                currency: Ok(value.currency),
                enforcement: Ok(value.enforcement),
                interval: Ok(value.interval),
                object: Ok(value.object),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectSpendLimitDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<
            super::ProjectSpendLimitDeletedResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectSpendLimitDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl ProjectSpendLimitDeletedResource {
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
            T: ::std::convert::TryInto<super::ProjectSpendLimitDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectSpendLimitDeletedResource>
        for super::ProjectSpendLimitDeletedResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectSpendLimitDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectSpendLimitDeletedResource>
        for ProjectSpendLimitDeletedResource
    {
        fn from(value: super::ProjectSpendLimitDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectSpendLimitResource {
        currency: ::std::result::Result<super::SpendLimitCurrency, ::std::string::String>,
        enforcement: ::std::result::Result<super::SpendLimitEnforcement, ::std::string::String>,
        interval: ::std::result::Result<super::SpendLimitInterval, ::std::string::String>,
        object:
            ::std::result::Result<super::ProjectSpendLimitResourceObject, ::std::string::String>,
        threshold_amount: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectSpendLimitResource {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                enforcement: Err("no value supplied for enforcement".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                object: Err("no value supplied for object".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl ProjectSpendLimitResource {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn enforcement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitEnforcement>,
            T::Error: ::std::fmt::Display,
        {
            self.enforcement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enforcement: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendLimitResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectSpendLimitResource> for super::ProjectSpendLimitResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectSpendLimitResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                enforcement: value.enforcement?,
                interval: value.interval?,
                object: value.object?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectSpendLimitResource> for ProjectSpendLimitResource {
        fn from(value: super::ProjectSpendLimitResource) -> Self {
            Self {
                currency: Ok(value.currency),
                enforcement: Ok(value.enforcement),
                interval: Ok(value.interval),
                object: Ok(value.object),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicDecisionsUsageResponseResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::DecisionsUsageBucket>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next_page: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<
            super::PublicDecisionsUsageResponseResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PublicDecisionsUsageResponseResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next_page: Err("no value supplied for next_page".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl PublicDecisionsUsageResponseResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DecisionsUsageBucket>>,
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
        pub fn next_page<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_page: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicDecisionsUsageResponseResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicDecisionsUsageResponseResource>
        for super::PublicDecisionsUsageResponseResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicDecisionsUsageResponseResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next_page: value.next_page?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::PublicDecisionsUsageResponseResource>
        for PublicDecisionsUsageResponseResource
    {
        fn from(value: super::PublicDecisionsUsageResponseResource) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next_page: Ok(value.next_page),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServiceAccountApiKeyBody {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ServiceAccountApiKeyBodyObject, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ServiceAccountApiKeyBody {
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
    impl ServiceAccountApiKeyBody {
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
            T: ::std::convert::TryInto<super::ServiceAccountApiKeyBodyObject>,
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
    impl ::std::convert::TryFrom<ServiceAccountApiKeyBody> for super::ServiceAccountApiKeyBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ServiceAccountApiKeyBody,
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
    impl ::std::convert::From<super::ServiceAccountApiKeyBody> for ServiceAccountApiKeyBody {
        fn from(value: super::ServiceAccountApiKeyBody) -> Self {
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
    pub struct SpendLimitEnforcement {
        status: ::std::result::Result<super::SpendLimitEnforcementStatus, ::std::string::String>,
    }
    impl ::std::default::Default for SpendLimitEnforcement {
        fn default() -> Self {
            Self {
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl SpendLimitEnforcement {
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SpendLimitEnforcementStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SpendLimitEnforcement> for super::SpendLimitEnforcement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpendLimitEnforcement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::SpendLimitEnforcement> for SpendLimitEnforcement {
        fn from(value: super::SpendLimitEnforcement) -> Self {
            Self {
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UncategorizedApiSchema {
        admin_api_keys_create_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        admin_api_keys_create_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        admin_api_keys_create_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        admin_api_keys_create_request:
            ::std::result::Result<super::AdminApiKeysCreateRequest, ::std::string::String>,
        admin_api_keys_create_response:
            ::std::result::Result<super::AdminApiKeyCreateResponse, ::std::string::String>,
        admin_api_keys_delete_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        admin_api_keys_delete_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        admin_api_keys_delete_params:
            ::std::result::Result<super::AdminApiKeysDeleteParams, ::std::string::String>,
        admin_api_keys_delete_response:
            ::std::result::Result<super::AdminApiKeysDeleteResponse, ::std::string::String>,
        admin_api_keys_get_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        admin_api_keys_get_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        admin_api_keys_get_params:
            ::std::result::Result<super::AdminApiKeysGetParams, ::std::string::String>,
        admin_api_keys_get_response:
            ::std::result::Result<super::AdminApiKey, ::std::string::String>,
        admin_api_keys_list_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        admin_api_keys_list_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        admin_api_keys_list_params:
            ::std::result::Result<super::AdminApiKeysListParams, ::std::string::String>,
        admin_api_keys_list_response:
            ::std::result::Result<super::ApiKeyList, ::std::string::String>,
        createan_ap_ikeyforaserviceaccount_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createan_ap_ikeyforaserviceaccount_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        createan_ap_ikeyforaserviceaccount_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createan_ap_ikeyforaserviceaccount_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createan_ap_ikeyforaserviceaccount_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createan_ap_ikeyforaserviceaccount_params: ::std::result::Result<
            super::CreateanApIkeyforaserviceaccountParams,
            ::std::string::String,
        >,
        createan_ap_ikeyforaserviceaccount_request: ::std::result::Result<
            super::CreateProjectServiceAccountApiKeyBody,
            ::std::string::String,
        >,
        createan_ap_ikeyforaserviceaccount_response:
            ::std::result::Result<super::ServiceAccountApiKeyBody, ::std::string::String>,
        createanexternalstorageconfiguration_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createanexternalstorageconfiguration_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createanexternalstorageconfiguration_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createanexternalstorageconfiguration_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        createanexternalstorageconfiguration_request:
            ::std::result::Result<super::CreateExternalStorageBody, ::std::string::String>,
        createanexternalstorageconfiguration_response:
            ::std::result::Result<super::ExternalStorageResponse, ::std::string::String>,
        deleteanexternalstorageconfiguration_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteanexternalstorageconfiguration_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteanexternalstorageconfiguration_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteanexternalstorageconfiguration_params: ::std::result::Result<
            super::DeleteanexternalstorageconfigurationParams,
            ::std::string::String,
        >,
        deleteanexternalstorageconfiguration_response:
            ::std::result::Result<super::ExternalStorageDeletedResource, ::std::string::String>,
        deleteorganizationspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        deleteorganizationspendlimit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteorganizationspendlimit_response: ::std::result::Result<
            super::OrganizationSpendLimitDeletedResource,
            ::std::string::String,
        >,
        deleteprojectspendlimit_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteprojectspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        deleteprojectspendlimit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        deleteprojectspendlimit_params:
            ::std::result::Result<super::DeleteprojectspendlimitParams, ::std::string::String>,
        deleteprojectspendlimit_response:
            ::std::result::Result<super::ProjectSpendLimitDeletedResource, ::std::string::String>,
        getanexternalstorageconfiguration_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getanexternalstorageconfiguration_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getanexternalstorageconfiguration_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getanexternalstorageconfiguration_params: ::std::result::Result<
            super::GetanexternalstorageconfigurationParams,
            ::std::string::String,
        >,
        getanexternalstorageconfiguration_response:
            ::std::result::Result<super::ExternalStorageResponse, ::std::string::String>,
        getorganizationspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        getorganizationspendlimit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getorganizationspendlimit_response:
            ::std::result::Result<super::OrganizationSpendLimitResource, ::std::string::String>,
        getprojectspendlimit_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        getprojectspendlimit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        getprojectspendlimit_params:
            ::std::result::Result<super::GetprojectspendlimitParams, ::std::string::String>,
        getprojectspendlimit_response:
            ::std::result::Result<super::ProjectSpendLimitResource, ::std::string::String>,
        listexternalstorageconfigurations_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        listexternalstorageconfigurations_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        listexternalstorageconfigurations_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        listexternalstorageconfigurations_params: ::std::result::Result<
            super::ListexternalstorageconfigurationsParams,
            ::std::string::String,
        >,
        listexternalstorageconfigurations_response:
            ::std::result::Result<super::ExternalStorageListResource, ::std::string::String>,
        organization_decisionsusage_error400:
            ::std::result::Result<super::DecisionsUsageErrorResponse, ::std::string::String>,
        organization_decisionsusage_error403: ::std::result::Result<
            super::DecisionsUsagePermissionErrorResponse,
            ::std::string::String,
        >,
        organization_decisionsusage_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        organization_decisionsusage_params:
            ::std::result::Result<super::OrganizationDecisionsusageParams, ::std::string::String>,
        organization_decisionsusage_response: ::std::result::Result<
            super::PublicDecisionsUsageResponseResource,
            ::std::string::String,
        >,
        updateorganizationspendlimit_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        updateorganizationspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        updateorganizationspendlimit_request:
            ::std::result::Result<super::UpdateOrganizationSpendLimitBody, ::std::string::String>,
        updateorganizationspendlimit_response:
            ::std::result::Result<super::OrganizationSpendLimitResource, ::std::string::String>,
        updateprojectspendlimit_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        updateprojectspendlimit_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        updateprojectspendlimit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        updateprojectspendlimit_params:
            ::std::result::Result<super::UpdateprojectspendlimitParams, ::std::string::String>,
        updateprojectspendlimit_request:
            ::std::result::Result<super::UpdateProjectSpendLimitBody, ::std::string::String>,
        updateprojectspendlimit_response:
            ::std::result::Result<super::ProjectSpendLimitResource, ::std::string::String>,
        validateanexternalstorageconfiguration_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        validateanexternalstorageconfiguration_error403:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        validateanexternalstorageconfiguration_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        validateanexternalstorageconfiguration_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        validateanexternalstorageconfiguration_params: ::std::result::Result<
            super::ValidateanexternalstorageconfigurationParams,
            ::std::string::String,
        >,
        validateanexternalstorageconfiguration_response:
            ::std::result::Result<super::ExternalStorageResponse, ::std::string::String>,
    }
    impl ::std::default::Default for UncategorizedApiSchema {
        fn default() -> Self {
            Self {
                admin_api_keys_create_error400: Err(
                    "no value supplied for admin_api_keys_create_error400".to_string(),
                ),
                admin_api_keys_create_error403: Err(
                    "no value supplied for admin_api_keys_create_error403".to_string(),
                ),
                admin_api_keys_create_error404: Err(
                    "no value supplied for admin_api_keys_create_error404".to_string(),
                ),
                admin_api_keys_create_request: Err(
                    "no value supplied for admin_api_keys_create_request".to_string(),
                ),
                admin_api_keys_create_response: Err(
                    "no value supplied for admin_api_keys_create_response".to_string(),
                ),
                admin_api_keys_delete_error403: Err(
                    "no value supplied for admin_api_keys_delete_error403".to_string(),
                ),
                admin_api_keys_delete_error404: Err(
                    "no value supplied for admin_api_keys_delete_error404".to_string(),
                ),
                admin_api_keys_delete_params: Err(
                    "no value supplied for admin_api_keys_delete_params".to_string(),
                ),
                admin_api_keys_delete_response: Err(
                    "no value supplied for admin_api_keys_delete_response".to_string(),
                ),
                admin_api_keys_get_error403: Err(
                    "no value supplied for admin_api_keys_get_error403".to_string(),
                ),
                admin_api_keys_get_error404: Err(
                    "no value supplied for admin_api_keys_get_error404".to_string(),
                ),
                admin_api_keys_get_params: Err(
                    "no value supplied for admin_api_keys_get_params".to_string()
                ),
                admin_api_keys_get_response: Err(
                    "no value supplied for admin_api_keys_get_response".to_string(),
                ),
                admin_api_keys_list_error400: Err(
                    "no value supplied for admin_api_keys_list_error400".to_string(),
                ),
                admin_api_keys_list_error403: Err(
                    "no value supplied for admin_api_keys_list_error403".to_string(),
                ),
                admin_api_keys_list_params: Err(
                    "no value supplied for admin_api_keys_list_params".to_string()
                ),
                admin_api_keys_list_response: Err(
                    "no value supplied for admin_api_keys_list_response".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_error400: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_error400".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_error403: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_error403".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_error404: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_error404".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_error409: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_error409".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_error429: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_error429".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_params: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_params".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_request: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_request".to_string(),
                ),
                createan_ap_ikeyforaserviceaccount_response: Err(
                    "no value supplied for createan_ap_ikeyforaserviceaccount_response".to_string(),
                ),
                createanexternalstorageconfiguration_error400: Err(
                    "no value supplied for createanexternalstorageconfiguration_error400"
                        .to_string(),
                ),
                createanexternalstorageconfiguration_error403: Err(
                    "no value supplied for createanexternalstorageconfiguration_error403"
                        .to_string(),
                ),
                createanexternalstorageconfiguration_error404: Err(
                    "no value supplied for createanexternalstorageconfiguration_error404"
                        .to_string(),
                ),
                createanexternalstorageconfiguration_error503: Err(
                    "no value supplied for createanexternalstorageconfiguration_error503"
                        .to_string(),
                ),
                createanexternalstorageconfiguration_request: Err(
                    "no value supplied for createanexternalstorageconfiguration_request"
                        .to_string(),
                ),
                createanexternalstorageconfiguration_response: Err(
                    "no value supplied for createanexternalstorageconfiguration_response"
                        .to_string(),
                ),
                deleteanexternalstorageconfiguration_error400: Err(
                    "no value supplied for deleteanexternalstorageconfiguration_error400"
                        .to_string(),
                ),
                deleteanexternalstorageconfiguration_error403: Err(
                    "no value supplied for deleteanexternalstorageconfiguration_error403"
                        .to_string(),
                ),
                deleteanexternalstorageconfiguration_error404: Err(
                    "no value supplied for deleteanexternalstorageconfiguration_error404"
                        .to_string(),
                ),
                deleteanexternalstorageconfiguration_params: Err(
                    "no value supplied for deleteanexternalstorageconfiguration_params".to_string(),
                ),
                deleteanexternalstorageconfiguration_response: Err(
                    "no value supplied for deleteanexternalstorageconfiguration_response"
                        .to_string(),
                ),
                deleteorganizationspendlimit_error403: Err(
                    "no value supplied for deleteorganizationspendlimit_error403".to_string(),
                ),
                deleteorganizationspendlimit_error404: Err(
                    "no value supplied for deleteorganizationspendlimit_error404".to_string(),
                ),
                deleteorganizationspendlimit_response: Err(
                    "no value supplied for deleteorganizationspendlimit_response".to_string(),
                ),
                deleteprojectspendlimit_error400: Err(
                    "no value supplied for deleteprojectspendlimit_error400".to_string(),
                ),
                deleteprojectspendlimit_error403: Err(
                    "no value supplied for deleteprojectspendlimit_error403".to_string(),
                ),
                deleteprojectspendlimit_error404: Err(
                    "no value supplied for deleteprojectspendlimit_error404".to_string(),
                ),
                deleteprojectspendlimit_params: Err(
                    "no value supplied for deleteprojectspendlimit_params".to_string(),
                ),
                deleteprojectspendlimit_response: Err(
                    "no value supplied for deleteprojectspendlimit_response".to_string(),
                ),
                getanexternalstorageconfiguration_error400: Err(
                    "no value supplied for getanexternalstorageconfiguration_error400".to_string(),
                ),
                getanexternalstorageconfiguration_error403: Err(
                    "no value supplied for getanexternalstorageconfiguration_error403".to_string(),
                ),
                getanexternalstorageconfiguration_error404: Err(
                    "no value supplied for getanexternalstorageconfiguration_error404".to_string(),
                ),
                getanexternalstorageconfiguration_params: Err(
                    "no value supplied for getanexternalstorageconfiguration_params".to_string(),
                ),
                getanexternalstorageconfiguration_response: Err(
                    "no value supplied for getanexternalstorageconfiguration_response".to_string(),
                ),
                getorganizationspendlimit_error403: Err(
                    "no value supplied for getorganizationspendlimit_error403".to_string(),
                ),
                getorganizationspendlimit_error404: Err(
                    "no value supplied for getorganizationspendlimit_error404".to_string(),
                ),
                getorganizationspendlimit_response: Err(
                    "no value supplied for getorganizationspendlimit_response".to_string(),
                ),
                getprojectspendlimit_error400: Err(
                    "no value supplied for getprojectspendlimit_error400".to_string(),
                ),
                getprojectspendlimit_error403: Err(
                    "no value supplied for getprojectspendlimit_error403".to_string(),
                ),
                getprojectspendlimit_error404: Err(
                    "no value supplied for getprojectspendlimit_error404".to_string(),
                ),
                getprojectspendlimit_params: Err(
                    "no value supplied for getprojectspendlimit_params".to_string(),
                ),
                getprojectspendlimit_response: Err(
                    "no value supplied for getprojectspendlimit_response".to_string(),
                ),
                listexternalstorageconfigurations_error400: Err(
                    "no value supplied for listexternalstorageconfigurations_error400".to_string(),
                ),
                listexternalstorageconfigurations_error403: Err(
                    "no value supplied for listexternalstorageconfigurations_error403".to_string(),
                ),
                listexternalstorageconfigurations_error404: Err(
                    "no value supplied for listexternalstorageconfigurations_error404".to_string(),
                ),
                listexternalstorageconfigurations_params: Err(
                    "no value supplied for listexternalstorageconfigurations_params".to_string(),
                ),
                listexternalstorageconfigurations_response: Err(
                    "no value supplied for listexternalstorageconfigurations_response".to_string(),
                ),
                organization_decisionsusage_error400: Err(
                    "no value supplied for organization_decisionsusage_error400".to_string(),
                ),
                organization_decisionsusage_error403: Err(
                    "no value supplied for organization_decisionsusage_error403".to_string(),
                ),
                organization_decisionsusage_error429: Err(
                    "no value supplied for organization_decisionsusage_error429".to_string(),
                ),
                organization_decisionsusage_params: Err(
                    "no value supplied for organization_decisionsusage_params".to_string(),
                ),
                organization_decisionsusage_response: Err(
                    "no value supplied for organization_decisionsusage_response".to_string(),
                ),
                updateorganizationspendlimit_error400: Err(
                    "no value supplied for updateorganizationspendlimit_error400".to_string(),
                ),
                updateorganizationspendlimit_error403: Err(
                    "no value supplied for updateorganizationspendlimit_error403".to_string(),
                ),
                updateorganizationspendlimit_request: Err(
                    "no value supplied for updateorganizationspendlimit_request".to_string(),
                ),
                updateorganizationspendlimit_response: Err(
                    "no value supplied for updateorganizationspendlimit_response".to_string(),
                ),
                updateprojectspendlimit_error400: Err(
                    "no value supplied for updateprojectspendlimit_error400".to_string(),
                ),
                updateprojectspendlimit_error403: Err(
                    "no value supplied for updateprojectspendlimit_error403".to_string(),
                ),
                updateprojectspendlimit_error404: Err(
                    "no value supplied for updateprojectspendlimit_error404".to_string(),
                ),
                updateprojectspendlimit_params: Err(
                    "no value supplied for updateprojectspendlimit_params".to_string(),
                ),
                updateprojectspendlimit_request: Err(
                    "no value supplied for updateprojectspendlimit_request".to_string(),
                ),
                updateprojectspendlimit_response: Err(
                    "no value supplied for updateprojectspendlimit_response".to_string(),
                ),
                validateanexternalstorageconfiguration_error400: Err(
                    "no value supplied for validateanexternalstorageconfiguration_error400"
                        .to_string(),
                ),
                validateanexternalstorageconfiguration_error403: Err(
                    "no value supplied for validateanexternalstorageconfiguration_error403"
                        .to_string(),
                ),
                validateanexternalstorageconfiguration_error404: Err(
                    "no value supplied for validateanexternalstorageconfiguration_error404"
                        .to_string(),
                ),
                validateanexternalstorageconfiguration_error503: Err(
                    "no value supplied for validateanexternalstorageconfiguration_error503"
                        .to_string(),
                ),
                validateanexternalstorageconfiguration_params: Err(
                    "no value supplied for validateanexternalstorageconfiguration_params"
                        .to_string(),
                ),
                validateanexternalstorageconfiguration_response: Err(
                    "no value supplied for validateanexternalstorageconfiguration_response"
                        .to_string(),
                ),
            }
        }
    }
    impl UncategorizedApiSchema {
        pub fn admin_api_keys_create_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_create_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_create_error400: {e}")
            });
            self
        }
        pub fn admin_api_keys_create_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_create_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_create_error403: {e}")
            });
            self
        }
        pub fn admin_api_keys_create_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_create_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_create_error404: {e}")
            });
            self
        }
        pub fn admin_api_keys_create_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeysCreateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_create_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_create_request: {e}")
            });
            self
        }
        pub fn admin_api_keys_create_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeyCreateResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_create_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_create_response: {e}")
            });
            self
        }
        pub fn admin_api_keys_delete_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_delete_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_delete_error403: {e}")
            });
            self
        }
        pub fn admin_api_keys_delete_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_delete_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_delete_error404: {e}")
            });
            self
        }
        pub fn admin_api_keys_delete_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeysDeleteParams>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_delete_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_delete_params: {e}")
            });
            self
        }
        pub fn admin_api_keys_delete_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeysDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_delete_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_delete_response: {e}")
            });
            self
        }
        pub fn admin_api_keys_get_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_get_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_get_error403: {e}")
            });
            self
        }
        pub fn admin_api_keys_get_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_get_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_get_error404: {e}")
            });
            self
        }
        pub fn admin_api_keys_get_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeysGetParams>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_get_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_get_params: {e}")
            });
            self
        }
        pub fn admin_api_keys_get_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKey>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_get_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_get_response: {e}")
            });
            self
        }
        pub fn admin_api_keys_list_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_list_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_list_error400: {e}")
            });
            self
        }
        pub fn admin_api_keys_list_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_list_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_list_error403: {e}")
            });
            self
        }
        pub fn admin_api_keys_list_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AdminApiKeysListParams>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_list_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_list_params: {e}")
            });
            self
        }
        pub fn admin_api_keys_list_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ApiKeyList>,
            T::Error: ::std::fmt::Display,
        {
            self.admin_api_keys_list_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for admin_api_keys_list_response: {e}")
            });
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_error400: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_error403: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_error404: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_error409 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_error409: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_error429: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateanApIkeyforaserviceaccountParams>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_params: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateProjectServiceAccountApiKeyBody>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_request: {e}")) ;
            self
        }
        pub fn createan_ap_ikeyforaserviceaccount_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ServiceAccountApiKeyBody>,
            T::Error: ::std::fmt::Display,
        {
            self . createan_ap_ikeyforaserviceaccount_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for createan_ap_ikeyforaserviceaccount_response: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_error400: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_error403: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_error404: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_error503 = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_error503: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateExternalStorageBody>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_request: {e}")) ;
            self
        }
        pub fn createanexternalstorageconfiguration_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . createanexternalstorageconfiguration_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for createanexternalstorageconfiguration_response: {e}")) ;
            self
        }
        pub fn deleteanexternalstorageconfiguration_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteanexternalstorageconfiguration_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteanexternalstorageconfiguration_error400: {e}")) ;
            self
        }
        pub fn deleteanexternalstorageconfiguration_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteanexternalstorageconfiguration_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteanexternalstorageconfiguration_error403: {e}")) ;
            self
        }
        pub fn deleteanexternalstorageconfiguration_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteanexternalstorageconfiguration_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteanexternalstorageconfiguration_error404: {e}")) ;
            self
        }
        pub fn deleteanexternalstorageconfiguration_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteanexternalstorageconfigurationParams>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteanexternalstorageconfiguration_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteanexternalstorageconfiguration_params: {e}")) ;
            self
        }
        pub fn deleteanexternalstorageconfiguration_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteanexternalstorageconfiguration_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteanexternalstorageconfiguration_response: {e}")) ;
            self
        }
        pub fn deleteorganizationspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteorganizationspendlimit_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteorganizationspendlimit_error403: {e}")) ;
            self
        }
        pub fn deleteorganizationspendlimit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteorganizationspendlimit_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteorganizationspendlimit_error404: {e}")) ;
            self
        }
        pub fn deleteorganizationspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendLimitDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self . deleteorganizationspendlimit_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for deleteorganizationspendlimit_response: {e}")) ;
            self
        }
        pub fn deleteprojectspendlimit_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.deleteprojectspendlimit_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleteprojectspendlimit_error400: {e}")
            });
            self
        }
        pub fn deleteprojectspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.deleteprojectspendlimit_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleteprojectspendlimit_error403: {e}")
            });
            self
        }
        pub fn deleteprojectspendlimit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.deleteprojectspendlimit_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleteprojectspendlimit_error404: {e}")
            });
            self
        }
        pub fn deleteprojectspendlimit_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteprojectspendlimitParams>,
            T::Error: ::std::fmt::Display,
        {
            self.deleteprojectspendlimit_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleteprojectspendlimit_params: {e}")
            });
            self
        }
        pub fn deleteprojectspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendLimitDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.deleteprojectspendlimit_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleteprojectspendlimit_response: {e}")
            });
            self
        }
        pub fn getanexternalstorageconfiguration_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . getanexternalstorageconfiguration_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for getanexternalstorageconfiguration_error400: {e}")) ;
            self
        }
        pub fn getanexternalstorageconfiguration_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . getanexternalstorageconfiguration_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for getanexternalstorageconfiguration_error403: {e}")) ;
            self
        }
        pub fn getanexternalstorageconfiguration_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . getanexternalstorageconfiguration_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for getanexternalstorageconfiguration_error404: {e}")) ;
            self
        }
        pub fn getanexternalstorageconfiguration_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetanexternalstorageconfigurationParams>,
            T::Error: ::std::fmt::Display,
        {
            self . getanexternalstorageconfiguration_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for getanexternalstorageconfiguration_params: {e}")) ;
            self
        }
        pub fn getanexternalstorageconfiguration_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . getanexternalstorageconfiguration_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for getanexternalstorageconfiguration_response: {e}")) ;
            self
        }
        pub fn getorganizationspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getorganizationspendlimit_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for getorganizationspendlimit_error403: {e}"
                )
            });
            self
        }
        pub fn getorganizationspendlimit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getorganizationspendlimit_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for getorganizationspendlimit_error404: {e}"
                )
            });
            self
        }
        pub fn getorganizationspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendLimitResource>,
            T::Error: ::std::fmt::Display,
        {
            self.getorganizationspendlimit_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for getorganizationspendlimit_response: {e}"
                )
            });
            self
        }
        pub fn getprojectspendlimit_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectspendlimit_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectspendlimit_error400: {e}")
            });
            self
        }
        pub fn getprojectspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectspendlimit_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectspendlimit_error403: {e}")
            });
            self
        }
        pub fn getprojectspendlimit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectspendlimit_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectspendlimit_error404: {e}")
            });
            self
        }
        pub fn getprojectspendlimit_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetprojectspendlimitParams>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectspendlimit_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectspendlimit_params: {e}")
            });
            self
        }
        pub fn getprojectspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendLimitResource>,
            T::Error: ::std::fmt::Display,
        {
            self.getprojectspendlimit_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for getprojectspendlimit_response: {e}")
            });
            self
        }
        pub fn listexternalstorageconfigurations_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . listexternalstorageconfigurations_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for listexternalstorageconfigurations_error400: {e}")) ;
            self
        }
        pub fn listexternalstorageconfigurations_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . listexternalstorageconfigurations_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for listexternalstorageconfigurations_error403: {e}")) ;
            self
        }
        pub fn listexternalstorageconfigurations_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . listexternalstorageconfigurations_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for listexternalstorageconfigurations_error404: {e}")) ;
            self
        }
        pub fn listexternalstorageconfigurations_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListexternalstorageconfigurationsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . listexternalstorageconfigurations_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for listexternalstorageconfigurations_params: {e}")) ;
            self
        }
        pub fn listexternalstorageconfigurations_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageListResource>,
            T::Error: ::std::fmt::Display,
        {
            self . listexternalstorageconfigurations_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for listexternalstorageconfigurations_response: {e}")) ;
            self
        }
        pub fn organization_decisionsusage_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsageErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_decisionsusage_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organization_decisionsusage_error400: {e}"
                )
            });
            self
        }
        pub fn organization_decisionsusage_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionsUsagePermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_decisionsusage_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organization_decisionsusage_error403: {e}"
                )
            });
            self
        }
        pub fn organization_decisionsusage_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_decisionsusage_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organization_decisionsusage_error429: {e}"
                )
            });
            self
        }
        pub fn organization_decisionsusage_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationDecisionsusageParams>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_decisionsusage_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organization_decisionsusage_params: {e}"
                )
            });
            self
        }
        pub fn organization_decisionsusage_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicDecisionsUsageResponseResource>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_decisionsusage_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organization_decisionsusage_response: {e}"
                )
            });
            self
        }
        pub fn updateorganizationspendlimit_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . updateorganizationspendlimit_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for updateorganizationspendlimit_error400: {e}")) ;
            self
        }
        pub fn updateorganizationspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . updateorganizationspendlimit_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for updateorganizationspendlimit_error403: {e}")) ;
            self
        }
        pub fn updateorganizationspendlimit_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationSpendLimitBody>,
            T::Error: ::std::fmt::Display,
        {
            self.updateorganizationspendlimit_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for updateorganizationspendlimit_request: {e}"
                )
            });
            self
        }
        pub fn updateorganizationspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganizationSpendLimitResource>,
            T::Error: ::std::fmt::Display,
        {
            self . updateorganizationspendlimit_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for updateorganizationspendlimit_response: {e}")) ;
            self
        }
        pub fn updateprojectspendlimit_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_error400: {e}")
            });
            self
        }
        pub fn updateprojectspendlimit_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_error403: {e}")
            });
            self
        }
        pub fn updateprojectspendlimit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_error404: {e}")
            });
            self
        }
        pub fn updateprojectspendlimit_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateprojectspendlimitParams>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_params: {e}")
            });
            self
        }
        pub fn updateprojectspendlimit_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectSpendLimitBody>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_request: {e}")
            });
            self
        }
        pub fn updateprojectspendlimit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectSpendLimitResource>,
            T::Error: ::std::fmt::Display,
        {
            self.updateprojectspendlimit_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for updateprojectspendlimit_response: {e}")
            });
            self
        }
        pub fn validateanexternalstorageconfiguration_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_error400: {e}")) ;
            self
        }
        pub fn validateanexternalstorageconfiguration_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_error403: {e}")) ;
            self
        }
        pub fn validateanexternalstorageconfiguration_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_error404: {e}")) ;
            self
        }
        pub fn validateanexternalstorageconfiguration_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_error503 = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_error503: {e}")) ;
            self
        }
        pub fn validateanexternalstorageconfiguration_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ValidateanexternalstorageconfigurationParams>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_params: {e}")) ;
            self
        }
        pub fn validateanexternalstorageconfiguration_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalStorageResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . validateanexternalstorageconfiguration_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for validateanexternalstorageconfiguration_response: {e}")) ;
            self
        }
    }
    impl ::std::convert::TryFrom<UncategorizedApiSchema> for super::UncategorizedApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UncategorizedApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                admin_api_keys_create_error400: value.admin_api_keys_create_error400?,
                admin_api_keys_create_error403: value.admin_api_keys_create_error403?,
                admin_api_keys_create_error404: value.admin_api_keys_create_error404?,
                admin_api_keys_create_request: value.admin_api_keys_create_request?,
                admin_api_keys_create_response: value.admin_api_keys_create_response?,
                admin_api_keys_delete_error403: value.admin_api_keys_delete_error403?,
                admin_api_keys_delete_error404: value.admin_api_keys_delete_error404?,
                admin_api_keys_delete_params: value.admin_api_keys_delete_params?,
                admin_api_keys_delete_response: value.admin_api_keys_delete_response?,
                admin_api_keys_get_error403: value.admin_api_keys_get_error403?,
                admin_api_keys_get_error404: value.admin_api_keys_get_error404?,
                admin_api_keys_get_params: value.admin_api_keys_get_params?,
                admin_api_keys_get_response: value.admin_api_keys_get_response?,
                admin_api_keys_list_error400: value.admin_api_keys_list_error400?,
                admin_api_keys_list_error403: value.admin_api_keys_list_error403?,
                admin_api_keys_list_params: value.admin_api_keys_list_params?,
                admin_api_keys_list_response: value.admin_api_keys_list_response?,
                createan_ap_ikeyforaserviceaccount_error400: value
                    .createan_ap_ikeyforaserviceaccount_error400?,
                createan_ap_ikeyforaserviceaccount_error403: value
                    .createan_ap_ikeyforaserviceaccount_error403?,
                createan_ap_ikeyforaserviceaccount_error404: value
                    .createan_ap_ikeyforaserviceaccount_error404?,
                createan_ap_ikeyforaserviceaccount_error409: value
                    .createan_ap_ikeyforaserviceaccount_error409?,
                createan_ap_ikeyforaserviceaccount_error429: value
                    .createan_ap_ikeyforaserviceaccount_error429?,
                createan_ap_ikeyforaserviceaccount_params: value
                    .createan_ap_ikeyforaserviceaccount_params?,
                createan_ap_ikeyforaserviceaccount_request: value
                    .createan_ap_ikeyforaserviceaccount_request?,
                createan_ap_ikeyforaserviceaccount_response: value
                    .createan_ap_ikeyforaserviceaccount_response?,
                createanexternalstorageconfiguration_error400: value
                    .createanexternalstorageconfiguration_error400?,
                createanexternalstorageconfiguration_error403: value
                    .createanexternalstorageconfiguration_error403?,
                createanexternalstorageconfiguration_error404: value
                    .createanexternalstorageconfiguration_error404?,
                createanexternalstorageconfiguration_error503: value
                    .createanexternalstorageconfiguration_error503?,
                createanexternalstorageconfiguration_request: value
                    .createanexternalstorageconfiguration_request?,
                createanexternalstorageconfiguration_response: value
                    .createanexternalstorageconfiguration_response?,
                deleteanexternalstorageconfiguration_error400: value
                    .deleteanexternalstorageconfiguration_error400?,
                deleteanexternalstorageconfiguration_error403: value
                    .deleteanexternalstorageconfiguration_error403?,
                deleteanexternalstorageconfiguration_error404: value
                    .deleteanexternalstorageconfiguration_error404?,
                deleteanexternalstorageconfiguration_params: value
                    .deleteanexternalstorageconfiguration_params?,
                deleteanexternalstorageconfiguration_response: value
                    .deleteanexternalstorageconfiguration_response?,
                deleteorganizationspendlimit_error403: value
                    .deleteorganizationspendlimit_error403?,
                deleteorganizationspendlimit_error404: value
                    .deleteorganizationspendlimit_error404?,
                deleteorganizationspendlimit_response: value
                    .deleteorganizationspendlimit_response?,
                deleteprojectspendlimit_error400: value.deleteprojectspendlimit_error400?,
                deleteprojectspendlimit_error403: value.deleteprojectspendlimit_error403?,
                deleteprojectspendlimit_error404: value.deleteprojectspendlimit_error404?,
                deleteprojectspendlimit_params: value.deleteprojectspendlimit_params?,
                deleteprojectspendlimit_response: value.deleteprojectspendlimit_response?,
                getanexternalstorageconfiguration_error400: value
                    .getanexternalstorageconfiguration_error400?,
                getanexternalstorageconfiguration_error403: value
                    .getanexternalstorageconfiguration_error403?,
                getanexternalstorageconfiguration_error404: value
                    .getanexternalstorageconfiguration_error404?,
                getanexternalstorageconfiguration_params: value
                    .getanexternalstorageconfiguration_params?,
                getanexternalstorageconfiguration_response: value
                    .getanexternalstorageconfiguration_response?,
                getorganizationspendlimit_error403: value.getorganizationspendlimit_error403?,
                getorganizationspendlimit_error404: value.getorganizationspendlimit_error404?,
                getorganizationspendlimit_response: value.getorganizationspendlimit_response?,
                getprojectspendlimit_error400: value.getprojectspendlimit_error400?,
                getprojectspendlimit_error403: value.getprojectspendlimit_error403?,
                getprojectspendlimit_error404: value.getprojectspendlimit_error404?,
                getprojectspendlimit_params: value.getprojectspendlimit_params?,
                getprojectspendlimit_response: value.getprojectspendlimit_response?,
                listexternalstorageconfigurations_error400: value
                    .listexternalstorageconfigurations_error400?,
                listexternalstorageconfigurations_error403: value
                    .listexternalstorageconfigurations_error403?,
                listexternalstorageconfigurations_error404: value
                    .listexternalstorageconfigurations_error404?,
                listexternalstorageconfigurations_params: value
                    .listexternalstorageconfigurations_params?,
                listexternalstorageconfigurations_response: value
                    .listexternalstorageconfigurations_response?,
                organization_decisionsusage_error400: value.organization_decisionsusage_error400?,
                organization_decisionsusage_error403: value.organization_decisionsusage_error403?,
                organization_decisionsusage_error429: value.organization_decisionsusage_error429?,
                organization_decisionsusage_params: value.organization_decisionsusage_params?,
                organization_decisionsusage_response: value.organization_decisionsusage_response?,
                updateorganizationspendlimit_error400: value
                    .updateorganizationspendlimit_error400?,
                updateorganizationspendlimit_error403: value
                    .updateorganizationspendlimit_error403?,
                updateorganizationspendlimit_request: value.updateorganizationspendlimit_request?,
                updateorganizationspendlimit_response: value
                    .updateorganizationspendlimit_response?,
                updateprojectspendlimit_error400: value.updateprojectspendlimit_error400?,
                updateprojectspendlimit_error403: value.updateprojectspendlimit_error403?,
                updateprojectspendlimit_error404: value.updateprojectspendlimit_error404?,
                updateprojectspendlimit_params: value.updateprojectspendlimit_params?,
                updateprojectspendlimit_request: value.updateprojectspendlimit_request?,
                updateprojectspendlimit_response: value.updateprojectspendlimit_response?,
                validateanexternalstorageconfiguration_error400: value
                    .validateanexternalstorageconfiguration_error400?,
                validateanexternalstorageconfiguration_error403: value
                    .validateanexternalstorageconfiguration_error403?,
                validateanexternalstorageconfiguration_error404: value
                    .validateanexternalstorageconfiguration_error404?,
                validateanexternalstorageconfiguration_error503: value
                    .validateanexternalstorageconfiguration_error503?,
                validateanexternalstorageconfiguration_params: value
                    .validateanexternalstorageconfiguration_params?,
                validateanexternalstorageconfiguration_response: value
                    .validateanexternalstorageconfiguration_response?,
            })
        }
    }
    impl ::std::convert::From<super::UncategorizedApiSchema> for UncategorizedApiSchema {
        fn from(value: super::UncategorizedApiSchema) -> Self {
            Self {
                admin_api_keys_create_error400: Ok(value.admin_api_keys_create_error400),
                admin_api_keys_create_error403: Ok(value.admin_api_keys_create_error403),
                admin_api_keys_create_error404: Ok(value.admin_api_keys_create_error404),
                admin_api_keys_create_request: Ok(value.admin_api_keys_create_request),
                admin_api_keys_create_response: Ok(value.admin_api_keys_create_response),
                admin_api_keys_delete_error403: Ok(value.admin_api_keys_delete_error403),
                admin_api_keys_delete_error404: Ok(value.admin_api_keys_delete_error404),
                admin_api_keys_delete_params: Ok(value.admin_api_keys_delete_params),
                admin_api_keys_delete_response: Ok(value.admin_api_keys_delete_response),
                admin_api_keys_get_error403: Ok(value.admin_api_keys_get_error403),
                admin_api_keys_get_error404: Ok(value.admin_api_keys_get_error404),
                admin_api_keys_get_params: Ok(value.admin_api_keys_get_params),
                admin_api_keys_get_response: Ok(value.admin_api_keys_get_response),
                admin_api_keys_list_error400: Ok(value.admin_api_keys_list_error400),
                admin_api_keys_list_error403: Ok(value.admin_api_keys_list_error403),
                admin_api_keys_list_params: Ok(value.admin_api_keys_list_params),
                admin_api_keys_list_response: Ok(value.admin_api_keys_list_response),
                createan_ap_ikeyforaserviceaccount_error400: Ok(
                    value.createan_ap_ikeyforaserviceaccount_error400
                ),
                createan_ap_ikeyforaserviceaccount_error403: Ok(
                    value.createan_ap_ikeyforaserviceaccount_error403
                ),
                createan_ap_ikeyforaserviceaccount_error404: Ok(
                    value.createan_ap_ikeyforaserviceaccount_error404
                ),
                createan_ap_ikeyforaserviceaccount_error409: Ok(
                    value.createan_ap_ikeyforaserviceaccount_error409
                ),
                createan_ap_ikeyforaserviceaccount_error429: Ok(
                    value.createan_ap_ikeyforaserviceaccount_error429
                ),
                createan_ap_ikeyforaserviceaccount_params: Ok(
                    value.createan_ap_ikeyforaserviceaccount_params
                ),
                createan_ap_ikeyforaserviceaccount_request: Ok(
                    value.createan_ap_ikeyforaserviceaccount_request
                ),
                createan_ap_ikeyforaserviceaccount_response: Ok(
                    value.createan_ap_ikeyforaserviceaccount_response
                ),
                createanexternalstorageconfiguration_error400: Ok(
                    value.createanexternalstorageconfiguration_error400
                ),
                createanexternalstorageconfiguration_error403: Ok(
                    value.createanexternalstorageconfiguration_error403
                ),
                createanexternalstorageconfiguration_error404: Ok(
                    value.createanexternalstorageconfiguration_error404
                ),
                createanexternalstorageconfiguration_error503: Ok(
                    value.createanexternalstorageconfiguration_error503
                ),
                createanexternalstorageconfiguration_request: Ok(
                    value.createanexternalstorageconfiguration_request
                ),
                createanexternalstorageconfiguration_response: Ok(
                    value.createanexternalstorageconfiguration_response
                ),
                deleteanexternalstorageconfiguration_error400: Ok(
                    value.deleteanexternalstorageconfiguration_error400
                ),
                deleteanexternalstorageconfiguration_error403: Ok(
                    value.deleteanexternalstorageconfiguration_error403
                ),
                deleteanexternalstorageconfiguration_error404: Ok(
                    value.deleteanexternalstorageconfiguration_error404
                ),
                deleteanexternalstorageconfiguration_params: Ok(
                    value.deleteanexternalstorageconfiguration_params
                ),
                deleteanexternalstorageconfiguration_response: Ok(
                    value.deleteanexternalstorageconfiguration_response
                ),
                deleteorganizationspendlimit_error403: Ok(
                    value.deleteorganizationspendlimit_error403
                ),
                deleteorganizationspendlimit_error404: Ok(
                    value.deleteorganizationspendlimit_error404
                ),
                deleteorganizationspendlimit_response: Ok(
                    value.deleteorganizationspendlimit_response
                ),
                deleteprojectspendlimit_error400: Ok(value.deleteprojectspendlimit_error400),
                deleteprojectspendlimit_error403: Ok(value.deleteprojectspendlimit_error403),
                deleteprojectspendlimit_error404: Ok(value.deleteprojectspendlimit_error404),
                deleteprojectspendlimit_params: Ok(value.deleteprojectspendlimit_params),
                deleteprojectspendlimit_response: Ok(value.deleteprojectspendlimit_response),
                getanexternalstorageconfiguration_error400: Ok(
                    value.getanexternalstorageconfiguration_error400
                ),
                getanexternalstorageconfiguration_error403: Ok(
                    value.getanexternalstorageconfiguration_error403
                ),
                getanexternalstorageconfiguration_error404: Ok(
                    value.getanexternalstorageconfiguration_error404
                ),
                getanexternalstorageconfiguration_params: Ok(
                    value.getanexternalstorageconfiguration_params
                ),
                getanexternalstorageconfiguration_response: Ok(
                    value.getanexternalstorageconfiguration_response
                ),
                getorganizationspendlimit_error403: Ok(value.getorganizationspendlimit_error403),
                getorganizationspendlimit_error404: Ok(value.getorganizationspendlimit_error404),
                getorganizationspendlimit_response: Ok(value.getorganizationspendlimit_response),
                getprojectspendlimit_error400: Ok(value.getprojectspendlimit_error400),
                getprojectspendlimit_error403: Ok(value.getprojectspendlimit_error403),
                getprojectspendlimit_error404: Ok(value.getprojectspendlimit_error404),
                getprojectspendlimit_params: Ok(value.getprojectspendlimit_params),
                getprojectspendlimit_response: Ok(value.getprojectspendlimit_response),
                listexternalstorageconfigurations_error400: Ok(
                    value.listexternalstorageconfigurations_error400
                ),
                listexternalstorageconfigurations_error403: Ok(
                    value.listexternalstorageconfigurations_error403
                ),
                listexternalstorageconfigurations_error404: Ok(
                    value.listexternalstorageconfigurations_error404
                ),
                listexternalstorageconfigurations_params: Ok(
                    value.listexternalstorageconfigurations_params
                ),
                listexternalstorageconfigurations_response: Ok(
                    value.listexternalstorageconfigurations_response
                ),
                organization_decisionsusage_error400: Ok(value.organization_decisionsusage_error400),
                organization_decisionsusage_error403: Ok(value.organization_decisionsusage_error403),
                organization_decisionsusage_error429: Ok(value.organization_decisionsusage_error429),
                organization_decisionsusage_params: Ok(value.organization_decisionsusage_params),
                organization_decisionsusage_response: Ok(value.organization_decisionsusage_response),
                updateorganizationspendlimit_error400: Ok(
                    value.updateorganizationspendlimit_error400
                ),
                updateorganizationspendlimit_error403: Ok(
                    value.updateorganizationspendlimit_error403
                ),
                updateorganizationspendlimit_request: Ok(value.updateorganizationspendlimit_request),
                updateorganizationspendlimit_response: Ok(
                    value.updateorganizationspendlimit_response
                ),
                updateprojectspendlimit_error400: Ok(value.updateprojectspendlimit_error400),
                updateprojectspendlimit_error403: Ok(value.updateprojectspendlimit_error403),
                updateprojectspendlimit_error404: Ok(value.updateprojectspendlimit_error404),
                updateprojectspendlimit_params: Ok(value.updateprojectspendlimit_params),
                updateprojectspendlimit_request: Ok(value.updateprojectspendlimit_request),
                updateprojectspendlimit_response: Ok(value.updateprojectspendlimit_response),
                validateanexternalstorageconfiguration_error400: Ok(
                    value.validateanexternalstorageconfiguration_error400
                ),
                validateanexternalstorageconfiguration_error403: Ok(
                    value.validateanexternalstorageconfiguration_error403
                ),
                validateanexternalstorageconfiguration_error404: Ok(
                    value.validateanexternalstorageconfiguration_error404
                ),
                validateanexternalstorageconfiguration_error503: Ok(
                    value.validateanexternalstorageconfiguration_error503
                ),
                validateanexternalstorageconfiguration_params: Ok(
                    value.validateanexternalstorageconfiguration_params
                ),
                validateanexternalstorageconfiguration_response: Ok(
                    value.validateanexternalstorageconfiguration_response
                ),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateOrganizationSpendLimitBody {
        currency: ::std::result::Result<
            super::UpdateOrganizationSpendLimitBodyCurrency,
            ::std::string::String,
        >,
        interval: ::std::result::Result<
            super::UpdateOrganizationSpendLimitBodyInterval,
            ::std::string::String,
        >,
        threshold_amount: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateOrganizationSpendLimitBody {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl UpdateOrganizationSpendLimitBody {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationSpendLimitBodyCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateOrganizationSpendLimitBodyInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateOrganizationSpendLimitBody>
        for super::UpdateOrganizationSpendLimitBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateOrganizationSpendLimitBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                interval: value.interval?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateOrganizationSpendLimitBody>
        for UpdateOrganizationSpendLimitBody
    {
        fn from(value: super::UpdateOrganizationSpendLimitBody) -> Self {
            Self {
                currency: Ok(value.currency),
                interval: Ok(value.interval),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateProjectSpendLimitBody {
        currency: ::std::result::Result<
            super::UpdateProjectSpendLimitBodyCurrency,
            ::std::string::String,
        >,
        interval: ::std::result::Result<
            super::UpdateProjectSpendLimitBodyInterval,
            ::std::string::String,
        >,
        threshold_amount: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateProjectSpendLimitBody {
        fn default() -> Self {
            Self {
                currency: Err("no value supplied for currency".to_string()),
                interval: Err("no value supplied for interval".to_string()),
                threshold_amount: Err("no value supplied for threshold_amount".to_string()),
            }
        }
    }
    impl UpdateProjectSpendLimitBody {
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectSpendLimitBodyCurrency>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for currency: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateProjectSpendLimitBodyInterval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn threshold_amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_amount: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateProjectSpendLimitBody> for super::UpdateProjectSpendLimitBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateProjectSpendLimitBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                currency: value.currency?,
                interval: value.interval?,
                threshold_amount: value.threshold_amount?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateProjectSpendLimitBody> for UpdateProjectSpendLimitBody {
        fn from(value: super::UpdateProjectSpendLimitBody) -> Self {
            Self {
                currency: Ok(value.currency),
                interval: Ok(value.interval),
                threshold_amount: Ok(value.threshold_amount),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateprojectspendlimitParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateprojectspendlimitParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl UpdateprojectspendlimitParams {
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
    impl ::std::convert::TryFrom<UpdateprojectspendlimitParams>
        for super::UpdateprojectspendlimitParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateprojectspendlimitParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateprojectspendlimitParams> for UpdateprojectspendlimitParams {
        fn from(value: super::UpdateprojectspendlimitParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ValidateanexternalstorageconfigurationParams {
        external_storage_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ValidateanexternalstorageconfigurationParams {
        fn default() -> Self {
            Self {
                external_storage_id: Err("no value supplied for external_storage_id".to_string()),
            }
        }
    }
    impl ValidateanexternalstorageconfigurationParams {
        pub fn external_storage_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.external_storage_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_storage_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ValidateanexternalstorageconfigurationParams>
        for super::ValidateanexternalstorageconfigurationParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ValidateanexternalstorageconfigurationParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                external_storage_id: value.external_storage_id?,
            })
        }
    }
    impl ::std::convert::From<super::ValidateanexternalstorageconfigurationParams>
        for ValidateanexternalstorageconfigurationParams
    {
        fn from(value: super::ValidateanexternalstorageconfigurationParams) -> Self {
            Self {
                external_storage_id: Ok(value.external_storage_id),
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
