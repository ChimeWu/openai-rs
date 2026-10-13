#[doc = "`ContainerFileListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerFileListResource {
    #[doc = "A list of container files."]
    pub data: ::std::vec::Vec<ContainerFileResource>,
    #[doc = "The ID of the first file in the list."]
    pub first_id: ::std::string::String,
    #[doc = "Whether there are more files available."]
    pub has_more: bool,
    #[doc = "The ID of the last file in the list."]
    pub last_id: ::std::string::String,
    #[doc = "The type of object returned, must be 'list'."]
    pub object: ContainerFileListResourceObject,
}
impl ContainerFileListResource {
    pub fn builder() -> builder::ContainerFileListResource {
        Default::default()
    }
}
#[doc = "The type of object returned, must be 'list'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerFileListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ContainerFileListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ContainerFileListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerFileListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerFileListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainerFileResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerFileResource {
    #[doc = "Size of the file in bytes."]
    pub bytes: i64,
    #[doc = "The container this file belongs to."]
    pub container_id: ::std::string::String,
    #[doc = "Unix timestamp (in seconds) when the file was created."]
    pub created_at: i64,
    #[doc = "Unique identifier for the file."]
    pub id: ::std::string::String,
    #[doc = "The type of this object (`container.file`)."]
    pub object: ::std::string::String,
    #[doc = "Path of the file in the container."]
    pub path: ::std::string::String,
    #[doc = "Source of the file (e.g., `user`, `assistant`)."]
    pub source: ::std::string::String,
}
impl ContainerFileResource {
    pub fn builder() -> builder::ContainerFileResource {
        Default::default()
    }
}
#[doc = "`ContainerListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerListResource {
    #[doc = "A list of containers."]
    pub data: ::std::vec::Vec<ContainerResource>,
    #[doc = "The ID of the first container in the list."]
    pub first_id: ::std::string::String,
    #[doc = "Whether there are more containers available."]
    pub has_more: bool,
    #[doc = "The ID of the last container in the list."]
    pub last_id: ::std::string::String,
    #[doc = "The type of object returned, must be 'list'."]
    pub object: ContainerListResourceObject,
}
impl ContainerListResource {
    pub fn builder() -> builder::ContainerListResource {
        Default::default()
    }
}
#[doc = "The type of object returned, must be 'list'."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ContainerListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ContainerListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerListResourceObject {
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
impl ContainerNetworkPolicyAllowlistParam {
    pub fn builder() -> builder::ContainerNetworkPolicyAllowlistParam {
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
impl ContainerNetworkPolicyDisabledParam {
    pub fn builder() -> builder::ContainerNetworkPolicyDisabledParam {
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
impl ContainerNetworkPolicyDomainSecretParam {
    pub fn builder() -> builder::ContainerNetworkPolicyDomainSecretParam {
        Default::default()
    }
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
#[doc = "`ContainerResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerResource {
    #[doc = "Unix timestamp (in seconds) when the container was created."]
    pub created_at: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<ContainerResourceExpiresAfter>,
    #[doc = "Unique identifier for the container."]
    pub id: ::std::string::String,
    #[doc = "Unix timestamp (in seconds) when the container was last active."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_active_at: ::std::option::Option<i64>,
    #[doc = "The memory limit configured for the container."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_limit: ::std::option::Option<ContainerResourceMemoryLimit>,
    #[doc = "Name of the container."]
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_policy: ::std::option::Option<ContainerResourceNetworkPolicy>,
    #[doc = "The type of this object."]
    pub object: ::std::string::String,
    #[doc = "Status of the container (e.g., active, deleted)."]
    pub status: ::std::string::String,
}
impl ContainerResource {
    pub fn builder() -> builder::ContainerResource {
        Default::default()
    }
}
#[doc = "The container will expire after this time period.\nThe anchor is the reference point for the expiration.\nThe minutes is the number of minutes after the anchor before the container expires.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ContainerResourceExpiresAfter {
    #[doc = "The reference point for the expiration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub anchor: ::std::option::Option<ContainerResourceExpiresAfterAnchor>,
    #[doc = "The number of minutes after the anchor before the container expires."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minutes: ::std::option::Option<i64>,
}
impl ContainerResourceExpiresAfter {
    pub fn builder() -> builder::ContainerResourceExpiresAfter {
        Default::default()
    }
}
#[doc = "The reference point for the expiration."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerResourceExpiresAfterAnchor {
    #[serde(rename = "last_active_at")]
    LastActiveAt,
}
impl ::std::fmt::Display for ContainerResourceExpiresAfterAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LastActiveAt => f.write_str("last_active_at"),
        }
    }
}
impl ::std::str::FromStr for ContainerResourceExpiresAfterAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "last_active_at" => Ok(Self::LastActiveAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerResourceExpiresAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerResourceExpiresAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The memory limit configured for the container."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerResourceMemoryLimit {
    #[serde(rename = "1g")]
    X1g,
    #[serde(rename = "4g")]
    X4g,
    #[serde(rename = "16g")]
    X16g,
    #[serde(rename = "64g")]
    X64g,
}
impl ::std::fmt::Display for ContainerResourceMemoryLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1g => f.write_str("1g"),
            Self::X4g => f.write_str("4g"),
            Self::X16g => f.write_str("16g"),
            Self::X64g => f.write_str("64g"),
        }
    }
}
impl ::std::str::FromStr for ContainerResourceMemoryLimit {
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
impl ::std::convert::TryFrom<&str> for ContainerResourceMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerResourceMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Network access policy for the container."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainerResourceNetworkPolicy {
    #[doc = "Allowed outbound domains when `type` is `allowlist`."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[doc = "The network policy mode."]
    #[serde(rename = "type")]
    pub type_: ContainerResourceNetworkPolicyType,
}
impl ContainerResourceNetworkPolicy {
    pub fn builder() -> builder::ContainerResourceNetworkPolicy {
        Default::default()
    }
}
#[doc = "The network policy mode."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContainerResourceNetworkPolicyType {
    #[serde(rename = "allowlist")]
    Allowlist,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for ContainerResourceNetworkPolicyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allowlist => f.write_str("allowlist"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for ContainerResourceNetworkPolicyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allowlist" => Ok(Self::Allowlist),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContainerResourceNetworkPolicyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContainerResourceNetworkPolicyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContainersApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ContainersApiSchema {
    #[serde(rename = "CreateContainerFileMultipartFormDataRequest")]
    pub create_container_file_multipart_form_data_request: CreateContainerFileBody,
    #[serde(rename = "CreateContainerFileRequest")]
    pub create_container_file_request: CreateContainerFileBody,
    #[serde(rename = "CreateContainerFileResponse")]
    pub create_container_file_response: ContainerFileResource,
    #[serde(rename = "CreateContainerRequest")]
    pub create_container_request: CreateContainerBody,
    #[serde(rename = "CreateContainerResponse")]
    pub create_container_response: ContainerResource,
    #[serde(rename = "DeleteContainerFileResponse")]
    pub delete_container_file_response: DeleteContainerFileResponse,
    #[serde(rename = "DeleteContainerResponse")]
    pub delete_container_response: DeleteContainerResponse,
    #[serde(rename = "ListContainerFilesParams")]
    pub list_container_files_params: ListContainerFilesParams,
    #[serde(rename = "ListContainerFilesResponse")]
    pub list_container_files_response: ContainerFileListResource,
    #[serde(rename = "ListContainersParams")]
    pub list_containers_params: ListContainersParams,
    #[serde(rename = "ListContainersResponse")]
    pub list_containers_response: ContainerListResource,
    #[serde(rename = "RetrieveContainerFileResponse")]
    pub retrieve_container_file_response: ContainerFileResource,
    #[serde(rename = "RetrieveContainerResponse")]
    pub retrieve_container_response: ContainerResource,
}
impl ContainersApiSchema {
    pub fn builder() -> builder::ContainersApiSchema {
        Default::default()
    }
}
#[doc = "`CreateContainerBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateContainerBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_after: ::std::option::Option<CreateContainerBodyExpiresAfter>,
    #[doc = "IDs of files to copy to the container."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional memory limit for the container. Defaults to \"1g\"."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_limit: ::std::option::Option<CreateContainerBodyMemoryLimit>,
    #[doc = "Name of the container to create."]
    pub name: ::std::string::String,
    #[doc = "Network access policy for the container."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_policy: ::std::option::Option<CreateContainerBodyNetworkPolicy>,
    #[doc = "An optional list of skills referenced by id or inline data."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<CreateContainerBodySkillsItem>,
}
impl CreateContainerBody {
    pub fn builder() -> builder::CreateContainerBody {
        Default::default()
    }
}
#[doc = "Container expiration time in seconds relative to the 'anchor' time."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateContainerBodyExpiresAfter {
    #[doc = "Time anchor for the expiration time. Currently only 'last_active_at' is supported."]
    pub anchor: CreateContainerBodyExpiresAfterAnchor,
    pub minutes: i64,
}
impl CreateContainerBodyExpiresAfter {
    pub fn builder() -> builder::CreateContainerBodyExpiresAfter {
        Default::default()
    }
}
#[doc = "Time anchor for the expiration time. Currently only 'last_active_at' is supported."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateContainerBodyExpiresAfterAnchor {
    #[serde(rename = "last_active_at")]
    LastActiveAt,
}
impl ::std::fmt::Display for CreateContainerBodyExpiresAfterAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LastActiveAt => f.write_str("last_active_at"),
        }
    }
}
impl ::std::str::FromStr for CreateContainerBodyExpiresAfterAnchor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "last_active_at" => Ok(Self::LastActiveAt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateContainerBodyExpiresAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateContainerBodyExpiresAfterAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Optional memory limit for the container. Defaults to \"1g\"."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateContainerBodyMemoryLimit {
    #[serde(rename = "1g")]
    X1g,
    #[serde(rename = "4g")]
    X4g,
    #[serde(rename = "16g")]
    X16g,
    #[serde(rename = "64g")]
    X64g,
}
impl ::std::fmt::Display for CreateContainerBodyMemoryLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1g => f.write_str("1g"),
            Self::X4g => f.write_str("4g"),
            Self::X16g => f.write_str("16g"),
            Self::X64g => f.write_str("64g"),
        }
    }
}
impl ::std::str::FromStr for CreateContainerBodyMemoryLimit {
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
impl ::std::convert::TryFrom<&str> for CreateContainerBodyMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateContainerBodyMemoryLimit {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Network access policy for the container."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateContainerBodyNetworkPolicy {
    DisabledParam(ContainerNetworkPolicyDisabledParam),
    AllowlistParam(ContainerNetworkPolicyAllowlistParam),
}
impl ::std::convert::From<ContainerNetworkPolicyDisabledParam>
    for CreateContainerBodyNetworkPolicy
{
    fn from(value: ContainerNetworkPolicyDisabledParam) -> Self {
        Self::DisabledParam(value)
    }
}
impl ::std::convert::From<ContainerNetworkPolicyAllowlistParam>
    for CreateContainerBodyNetworkPolicy
{
    fn from(value: ContainerNetworkPolicyAllowlistParam) -> Self {
        Self::AllowlistParam(value)
    }
}
#[doc = "`CreateContainerBodySkillsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateContainerBodySkillsItem {
    SkillReferenceParam(SkillReferenceParam),
    InlineSkillParam(InlineSkillParam),
}
impl ::std::convert::From<SkillReferenceParam> for CreateContainerBodySkillsItem {
    fn from(value: SkillReferenceParam) -> Self {
        Self::SkillReferenceParam(value)
    }
}
impl ::std::convert::From<InlineSkillParam> for CreateContainerBodySkillsItem {
    fn from(value: InlineSkillParam) -> Self {
        Self::InlineSkillParam(value)
    }
}
#[doc = "`CreateContainerFileBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateContainerFileBody {
    #[doc = "The File object (not file name) to be uploaded.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<::std::string::String>,
    #[doc = "Name of the file to create."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
}
impl CreateContainerFileBody {
    pub fn builder() -> builder::CreateContainerFileBody {
        Default::default()
    }
}
#[doc = "`DeleteContainerFileResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteContainerFileResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteContainerFileResponseObject,
}
impl DeleteContainerFileResponse {
    pub fn builder() -> builder::DeleteContainerFileResponse {
        Default::default()
    }
}
#[doc = "`DeleteContainerFileResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeleteContainerFileResponseObject {
    #[serde(rename = "container.file.deleted")]
    ContainerFileDeleted,
}
impl ::std::fmt::Display for DeleteContainerFileResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerFileDeleted => f.write_str("container.file.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteContainerFileResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container.file.deleted" => Ok(Self::ContainerFileDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteContainerFileResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteContainerFileResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DeleteContainerResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteContainerResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: DeleteContainerResponseObject,
}
impl DeleteContainerResponse {
    pub fn builder() -> builder::DeleteContainerResponse {
        Default::default()
    }
}
#[doc = "`DeleteContainerResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeleteContainerResponseObject {
    #[serde(rename = "container.deleted")]
    ContainerDeleted,
}
impl ::std::fmt::Display for DeleteContainerResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContainerDeleted => f.write_str("container.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeleteContainerResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "container.deleted" => Ok(Self::ContainerDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteContainerResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteContainerResponseObject {
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
impl InlineSkillParam {
    pub fn builder() -> builder::InlineSkillParam {
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
impl InlineSkillSourceParam {
    pub fn builder() -> builder::InlineSkillSourceParam {
        Default::default()
    }
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
#[doc = "`ListContainerFilesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListContainerFilesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListContainerFilesParamsOrder>,
}
impl ListContainerFilesParams {
    pub fn builder() -> builder::ListContainerFilesParams {
        Default::default()
    }
}
#[doc = "`ListContainerFilesParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListContainerFilesParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListContainerFilesParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListContainerFilesParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListContainerFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListContainerFilesParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListContainersParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListContainersParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListContainersParamsOrder>,
}
impl ListContainersParams {
    pub fn builder() -> builder::ListContainersParams {
        Default::default()
    }
}
#[doc = "`ListContainersParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListContainersParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListContainersParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListContainersParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListContainersParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListContainersParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
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
impl SkillReferenceParam {
    pub fn builder() -> builder::SkillReferenceParam {
        Default::default()
    }
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
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ContainerFileListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::ContainerFileResource>,
            ::std::string::String,
        >,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::ContainerFileListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerFileListResource {
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
    impl ContainerFileListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ContainerFileResource>>,
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
            T: ::std::convert::TryInto<super::ContainerFileListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerFileListResource> for super::ContainerFileListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerFileListResource,
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
    impl ::std::convert::From<super::ContainerFileListResource> for ContainerFileListResource {
        fn from(value: super::ContainerFileListResource) -> Self {
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
    pub struct ContainerFileResource {
        bytes: ::std::result::Result<i64, ::std::string::String>,
        container_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerFileResource {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                container_id: Err("no value supplied for container_id".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                path: Err("no value supplied for path".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl ContainerFileResource {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    }
    impl ::std::convert::TryFrom<ContainerFileResource> for super::ContainerFileResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerFileResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                container_id: value.container_id?,
                created_at: value.created_at?,
                id: value.id?,
                object: value.object?,
                path: value.path?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerFileResource> for ContainerFileResource {
        fn from(value: super::ContainerFileResource) -> Self {
            Self {
                bytes: Ok(value.bytes),
                container_id: Ok(value.container_id),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                object: Ok(value.object),
                path: Ok(value.path),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerListResource {
        data:
            ::std::result::Result<::std::vec::Vec<super::ContainerResource>, ::std::string::String>,
        first_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::ContainerListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerListResource {
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
    impl ContainerListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ContainerResource>>,
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
            T: ::std::convert::TryInto<super::ContainerListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerListResource> for super::ContainerListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerListResource,
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
    impl ::std::convert::From<super::ContainerListResource> for ContainerListResource {
        fn from(value: super::ContainerListResource) -> Self {
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
    pub struct ContainerNetworkPolicyAllowlistParam {
        allowed_domains:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        domain_secrets: ::std::result::Result<
            ::std::vec::Vec<super::ContainerNetworkPolicyDomainSecretParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::ContainerNetworkPolicyAllowlistParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ContainerNetworkPolicyAllowlistParam {
        fn default() -> Self {
            Self {
                allowed_domains: Err("no value supplied for allowed_domains".to_string()),
                domain_secrets: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ContainerNetworkPolicyAllowlistParam {
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
        pub fn domain_secrets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::ContainerNetworkPolicyDomainSecretParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.domain_secrets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domain_secrets: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerNetworkPolicyAllowlistParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerNetworkPolicyAllowlistParam>
        for super::ContainerNetworkPolicyAllowlistParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerNetworkPolicyAllowlistParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_domains: value.allowed_domains?,
                domain_secrets: value.domain_secrets?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerNetworkPolicyAllowlistParam>
        for ContainerNetworkPolicyAllowlistParam
    {
        fn from(value: super::ContainerNetworkPolicyAllowlistParam) -> Self {
            Self {
                allowed_domains: Ok(value.allowed_domains),
                domain_secrets: Ok(value.domain_secrets),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerNetworkPolicyDisabledParam {
        type_: ::std::result::Result<
            super::ContainerNetworkPolicyDisabledParamType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ContainerNetworkPolicyDisabledParam {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ContainerNetworkPolicyDisabledParam {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerNetworkPolicyDisabledParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerNetworkPolicyDisabledParam>
        for super::ContainerNetworkPolicyDisabledParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerNetworkPolicyDisabledParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerNetworkPolicyDisabledParam>
        for ContainerNetworkPolicyDisabledParam
    {
        fn from(value: super::ContainerNetworkPolicyDisabledParam) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerNetworkPolicyDomainSecretParam {
        domain: ::std::result::Result<
            super::ContainerNetworkPolicyDomainSecretParamDomain,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::ContainerNetworkPolicyDomainSecretParamName,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            super::ContainerNetworkPolicyDomainSecretParamValue,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ContainerNetworkPolicyDomainSecretParam {
        fn default() -> Self {
            Self {
                domain: Err("no value supplied for domain".to_string()),
                name: Err("no value supplied for name".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ContainerNetworkPolicyDomainSecretParam {
        pub fn domain<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerNetworkPolicyDomainSecretParamDomain>,
            T::Error: ::std::fmt::Display,
        {
            self.domain = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domain: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerNetworkPolicyDomainSecretParamName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerNetworkPolicyDomainSecretParamValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerNetworkPolicyDomainSecretParam>
        for super::ContainerNetworkPolicyDomainSecretParam
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerNetworkPolicyDomainSecretParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                domain: value.domain?,
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerNetworkPolicyDomainSecretParam>
        for ContainerNetworkPolicyDomainSecretParam
    {
        fn from(value: super::ContainerNetworkPolicyDomainSecretParam) -> Self {
            Self {
                domain: Ok(value.domain),
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        expires_after: ::std::result::Result<
            ::std::option::Option<super::ContainerResourceExpiresAfter>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_active_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        memory_limit: ::std::result::Result<
            ::std::option::Option<super::ContainerResourceMemoryLimit>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        network_policy: ::std::result::Result<
            ::std::option::Option<super::ContainerResourceNetworkPolicy>,
            ::std::string::String,
        >,
        object: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                expires_after: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                last_active_at: Ok(Default::default()),
                memory_limit: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                network_policy: Ok(Default::default()),
                object: Err("no value supplied for object".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl ContainerResource {
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
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ContainerResourceExpiresAfter>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
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
        pub fn last_active_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_active_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_active_at: {e}"));
            self
        }
        pub fn memory_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ContainerResourceMemoryLimit>>,
            T::Error: ::std::fmt::Display,
        {
            self.memory_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memory_limit: {e}"));
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
        pub fn network_policy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ContainerResourceNetworkPolicy>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.network_policy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_policy: {e}"));
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
    impl ::std::convert::TryFrom<ContainerResource> for super::ContainerResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                expires_after: value.expires_after?,
                id: value.id?,
                last_active_at: value.last_active_at?,
                memory_limit: value.memory_limit?,
                name: value.name?,
                network_policy: value.network_policy?,
                object: value.object?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerResource> for ContainerResource {
        fn from(value: super::ContainerResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                expires_after: Ok(value.expires_after),
                id: Ok(value.id),
                last_active_at: Ok(value.last_active_at),
                memory_limit: Ok(value.memory_limit),
                name: Ok(value.name),
                network_policy: Ok(value.network_policy),
                object: Ok(value.object),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerResourceExpiresAfter {
        anchor: ::std::result::Result<
            ::std::option::Option<super::ContainerResourceExpiresAfterAnchor>,
            ::std::string::String,
        >,
        minutes: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerResourceExpiresAfter {
        fn default() -> Self {
            Self {
                anchor: Ok(Default::default()),
                minutes: Ok(Default::default()),
            }
        }
    }
    impl ContainerResourceExpiresAfter {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ContainerResourceExpiresAfterAnchor>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
            self
        }
        pub fn minutes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minutes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minutes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerResourceExpiresAfter>
        for super::ContainerResourceExpiresAfter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerResourceExpiresAfter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                minutes: value.minutes?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerResourceExpiresAfter> for ContainerResourceExpiresAfter {
        fn from(value: super::ContainerResourceExpiresAfter) -> Self {
            Self {
                anchor: Ok(value.anchor),
                minutes: Ok(value.minutes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerResourceNetworkPolicy {
        allowed_domains:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_:
            ::std::result::Result<super::ContainerResourceNetworkPolicyType, ::std::string::String>,
    }
    impl ::std::default::Default for ContainerResourceNetworkPolicy {
        fn default() -> Self {
            Self {
                allowed_domains: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ContainerResourceNetworkPolicy {
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
            T: ::std::convert::TryInto<super::ContainerResourceNetworkPolicyType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerResourceNetworkPolicy>
        for super::ContainerResourceNetworkPolicy
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerResourceNetworkPolicy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_domains: value.allowed_domains?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerResourceNetworkPolicy>
        for ContainerResourceNetworkPolicy
    {
        fn from(value: super::ContainerResourceNetworkPolicy) -> Self {
            Self {
                allowed_domains: Ok(value.allowed_domains),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainersApiSchema {
        create_container_file_multipart_form_data_request:
            ::std::result::Result<super::CreateContainerFileBody, ::std::string::String>,
        create_container_file_request:
            ::std::result::Result<super::CreateContainerFileBody, ::std::string::String>,
        create_container_file_response:
            ::std::result::Result<super::ContainerFileResource, ::std::string::String>,
        create_container_request:
            ::std::result::Result<super::CreateContainerBody, ::std::string::String>,
        create_container_response:
            ::std::result::Result<super::ContainerResource, ::std::string::String>,
        delete_container_file_response:
            ::std::result::Result<super::DeleteContainerFileResponse, ::std::string::String>,
        delete_container_response:
            ::std::result::Result<super::DeleteContainerResponse, ::std::string::String>,
        list_container_files_params:
            ::std::result::Result<super::ListContainerFilesParams, ::std::string::String>,
        list_container_files_response:
            ::std::result::Result<super::ContainerFileListResource, ::std::string::String>,
        list_containers_params:
            ::std::result::Result<super::ListContainersParams, ::std::string::String>,
        list_containers_response:
            ::std::result::Result<super::ContainerListResource, ::std::string::String>,
        retrieve_container_file_response:
            ::std::result::Result<super::ContainerFileResource, ::std::string::String>,
        retrieve_container_response:
            ::std::result::Result<super::ContainerResource, ::std::string::String>,
    }
    impl ::std::default::Default for ContainersApiSchema {
        fn default() -> Self {
            Self {
                create_container_file_multipart_form_data_request: Err(
                    "no value supplied for create_container_file_multipart_form_data_request"
                        .to_string(),
                ),
                create_container_file_request: Err(
                    "no value supplied for create_container_file_request".to_string(),
                ),
                create_container_file_response: Err(
                    "no value supplied for create_container_file_response".to_string(),
                ),
                create_container_request: Err(
                    "no value supplied for create_container_request".to_string()
                ),
                create_container_response: Err(
                    "no value supplied for create_container_response".to_string()
                ),
                delete_container_file_response: Err(
                    "no value supplied for delete_container_file_response".to_string(),
                ),
                delete_container_response: Err(
                    "no value supplied for delete_container_response".to_string()
                ),
                list_container_files_params: Err(
                    "no value supplied for list_container_files_params".to_string(),
                ),
                list_container_files_response: Err(
                    "no value supplied for list_container_files_response".to_string(),
                ),
                list_containers_params: Err(
                    "no value supplied for list_containers_params".to_string()
                ),
                list_containers_response: Err(
                    "no value supplied for list_containers_response".to_string()
                ),
                retrieve_container_file_response: Err(
                    "no value supplied for retrieve_container_file_response".to_string(),
                ),
                retrieve_container_response: Err(
                    "no value supplied for retrieve_container_response".to_string(),
                ),
            }
        }
    }
    impl ContainersApiSchema {
        pub fn create_container_file_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateContainerFileBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_container_file_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_container_file_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_container_file_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateContainerFileBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_container_file_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_container_file_request: {e}")
            });
            self
        }
        pub fn create_container_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerFileResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_container_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_container_file_response: {e}")
            });
            self
        }
        pub fn create_container_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateContainerBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_container_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_container_request: {e}")
            });
            self
        }
        pub fn create_container_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_container_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_container_response: {e}")
            });
            self
        }
        pub fn delete_container_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteContainerFileResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_container_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_container_file_response: {e}")
            });
            self
        }
        pub fn delete_container_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteContainerResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_container_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_container_response: {e}")
            });
            self
        }
        pub fn list_container_files_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListContainerFilesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_container_files_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_container_files_params: {e}")
            });
            self
        }
        pub fn list_container_files_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerFileListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_container_files_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_container_files_response: {e}")
            });
            self
        }
        pub fn list_containers_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListContainersParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_containers_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_containers_params: {e}")
            });
            self
        }
        pub fn list_containers_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_containers_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_containers_response: {e}")
            });
            self
        }
        pub fn retrieve_container_file_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerFileResource>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_container_file_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_container_file_response: {e}")
            });
            self
        }
        pub fn retrieve_container_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContainerResource>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_container_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_container_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ContainersApiSchema> for super::ContainersApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainersApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_container_file_multipart_form_data_request: value
                    .create_container_file_multipart_form_data_request?,
                create_container_file_request: value.create_container_file_request?,
                create_container_file_response: value.create_container_file_response?,
                create_container_request: value.create_container_request?,
                create_container_response: value.create_container_response?,
                delete_container_file_response: value.delete_container_file_response?,
                delete_container_response: value.delete_container_response?,
                list_container_files_params: value.list_container_files_params?,
                list_container_files_response: value.list_container_files_response?,
                list_containers_params: value.list_containers_params?,
                list_containers_response: value.list_containers_response?,
                retrieve_container_file_response: value.retrieve_container_file_response?,
                retrieve_container_response: value.retrieve_container_response?,
            })
        }
    }
    impl ::std::convert::From<super::ContainersApiSchema> for ContainersApiSchema {
        fn from(value: super::ContainersApiSchema) -> Self {
            Self {
                create_container_file_multipart_form_data_request: Ok(
                    value.create_container_file_multipart_form_data_request
                ),
                create_container_file_request: Ok(value.create_container_file_request),
                create_container_file_response: Ok(value.create_container_file_response),
                create_container_request: Ok(value.create_container_request),
                create_container_response: Ok(value.create_container_response),
                delete_container_file_response: Ok(value.delete_container_file_response),
                delete_container_response: Ok(value.delete_container_response),
                list_container_files_params: Ok(value.list_container_files_params),
                list_container_files_response: Ok(value.list_container_files_response),
                list_containers_params: Ok(value.list_containers_params),
                list_containers_response: Ok(value.list_containers_response),
                retrieve_container_file_response: Ok(value.retrieve_container_file_response),
                retrieve_container_response: Ok(value.retrieve_container_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateContainerBody {
        expires_after: ::std::result::Result<
            ::std::option::Option<super::CreateContainerBodyExpiresAfter>,
            ::std::string::String,
        >,
        file_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        memory_limit: ::std::result::Result<
            ::std::option::Option<super::CreateContainerBodyMemoryLimit>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        network_policy: ::std::result::Result<
            ::std::option::Option<super::CreateContainerBodyNetworkPolicy>,
            ::std::string::String,
        >,
        skills: ::std::result::Result<
            ::std::vec::Vec<super::CreateContainerBodySkillsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateContainerBody {
        fn default() -> Self {
            Self {
                expires_after: Ok(Default::default()),
                file_ids: Ok(Default::default()),
                memory_limit: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                network_policy: Ok(Default::default()),
                skills: Ok(Default::default()),
            }
        }
    }
    impl CreateContainerBody {
        pub fn expires_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateContainerBodyExpiresAfter>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_after: {e}"));
            self
        }
        pub fn file_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_ids: {e}"));
            self
        }
        pub fn memory_limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateContainerBodyMemoryLimit>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.memory_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memory_limit: {e}"));
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
        pub fn network_policy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateContainerBodyNetworkPolicy>,
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::CreateContainerBodySkillsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateContainerBody> for super::CreateContainerBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateContainerBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_after: value.expires_after?,
                file_ids: value.file_ids?,
                memory_limit: value.memory_limit?,
                name: value.name?,
                network_policy: value.network_policy?,
                skills: value.skills?,
            })
        }
    }
    impl ::std::convert::From<super::CreateContainerBody> for CreateContainerBody {
        fn from(value: super::CreateContainerBody) -> Self {
            Self {
                expires_after: Ok(value.expires_after),
                file_ids: Ok(value.file_ids),
                memory_limit: Ok(value.memory_limit),
                name: Ok(value.name),
                network_policy: Ok(value.network_policy),
                skills: Ok(value.skills),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateContainerBodyExpiresAfter {
        anchor: ::std::result::Result<
            super::CreateContainerBodyExpiresAfterAnchor,
            ::std::string::String,
        >,
        minutes: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for CreateContainerBodyExpiresAfter {
        fn default() -> Self {
            Self {
                anchor: Err("no value supplied for anchor".to_string()),
                minutes: Err("no value supplied for minutes".to_string()),
            }
        }
    }
    impl CreateContainerBodyExpiresAfter {
        pub fn anchor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateContainerBodyExpiresAfterAnchor>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor: {e}"));
            self
        }
        pub fn minutes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.minutes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minutes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateContainerBodyExpiresAfter>
        for super::CreateContainerBodyExpiresAfter
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateContainerBodyExpiresAfter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor: value.anchor?,
                minutes: value.minutes?,
            })
        }
    }
    impl ::std::convert::From<super::CreateContainerBodyExpiresAfter>
        for CreateContainerBodyExpiresAfter
    {
        fn from(value: super::CreateContainerBodyExpiresAfter) -> Self {
            Self {
                anchor: Ok(value.anchor),
                minutes: Ok(value.minutes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateContainerFileBody {
        file: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateContainerFileBody {
        fn default() -> Self {
            Self {
                file: Ok(Default::default()),
                file_id: Ok(Default::default()),
            }
        }
    }
    impl CreateContainerFileBody {
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateContainerFileBody> for super::CreateContainerFileBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateContainerFileBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file: value.file?,
                file_id: value.file_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateContainerFileBody> for CreateContainerFileBody {
        fn from(value: super::CreateContainerFileBody) -> Self {
            Self {
                file: Ok(value.file),
                file_id: Ok(value.file_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteContainerFileResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object:
            ::std::result::Result<super::DeleteContainerFileResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteContainerFileResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteContainerFileResponse {
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
            T: ::std::convert::TryInto<super::DeleteContainerFileResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteContainerFileResponse> for super::DeleteContainerFileResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteContainerFileResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteContainerFileResponse> for DeleteContainerFileResponse {
        fn from(value: super::DeleteContainerFileResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteContainerResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeleteContainerResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteContainerResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeleteContainerResponse {
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
            T: ::std::convert::TryInto<super::DeleteContainerResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteContainerResponse> for super::DeleteContainerResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteContainerResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteContainerResponse> for DeleteContainerResponse {
        fn from(value: super::DeleteContainerResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineSkillParam {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::InlineSkillSourceParam, ::std::string::String>,
        type_: ::std::result::Result<super::InlineSkillParamType, ::std::string::String>,
    }
    impl ::std::default::Default for InlineSkillParam {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl InlineSkillParam {
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
            T: ::std::convert::TryInto<super::InlineSkillSourceParam>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InlineSkillParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InlineSkillParam> for super::InlineSkillParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InlineSkillParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::InlineSkillParam> for InlineSkillParam {
        fn from(value: super::InlineSkillParam) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InlineSkillSourceParam {
        data: ::std::result::Result<super::InlineSkillSourceParamData, ::std::string::String>,
        media_type:
            ::std::result::Result<super::InlineSkillSourceParamMediaType, ::std::string::String>,
        type_: ::std::result::Result<super::InlineSkillSourceParamType, ::std::string::String>,
    }
    impl ::std::default::Default for InlineSkillSourceParam {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                media_type: Err("no value supplied for media_type".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl InlineSkillSourceParam {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InlineSkillSourceParamData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InlineSkillSourceParamMediaType>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InlineSkillSourceParamType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InlineSkillSourceParam> for super::InlineSkillSourceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InlineSkillSourceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                media_type: value.media_type?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::InlineSkillSourceParam> for InlineSkillSourceParam {
        fn from(value: super::InlineSkillSourceParam) -> Self {
            Self {
                data: Ok(value.data),
                media_type: Ok(value.media_type),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListContainerFilesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListContainerFilesParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListContainerFilesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListContainerFilesParams {
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListContainerFilesParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListContainerFilesParams> for super::ListContainerFilesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListContainerFilesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListContainerFilesParams> for ListContainerFilesParams {
        fn from(value: super::ListContainerFilesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListContainersParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<super::ListContainersParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListContainersParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                name: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListContainersParams {
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
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListContainersParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListContainersParams> for super::ListContainersParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListContainersParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                name: value.name?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListContainersParams> for ListContainersParams {
        fn from(value: super::ListContainersParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                name: Ok(value.name),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillReferenceParam {
        skill_id: ::std::result::Result<super::SkillReferenceParamSkillId, ::std::string::String>,
        type_: ::std::result::Result<super::SkillReferenceParamType, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SkillReferenceParam {
        fn default() -> Self {
            Self {
                skill_id: Err("no value supplied for skill_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl SkillReferenceParam {
        pub fn skill_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillReferenceParamSkillId>,
            T::Error: ::std::fmt::Display,
        {
            self.skill_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SkillReferenceParamType>,
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
    impl ::std::convert::TryFrom<SkillReferenceParam> for super::SkillReferenceParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SkillReferenceParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                skill_id: value.skill_id?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::SkillReferenceParam> for SkillReferenceParam {
        fn from(value: super::SkillReferenceParam) -> Self {
            Self {
                skill_id: Ok(value.skill_id),
                type_: Ok(value.type_),
                version: Ok(value.version),
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
