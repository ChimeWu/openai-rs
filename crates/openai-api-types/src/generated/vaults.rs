pub use crate::common::{ErrorBodyResource, ErrorResponse2, ListOrderParam};
#[doc = "Configuration for refreshing the access token of an MCP OAuth credential."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateMcpOauthRefreshParam {
    #[doc = "The OAuth client ID used when requesting a new access token."]
    pub client_id: CreateMcpOauthRefreshParamClientId,
    #[doc = "The refresh token to store. This secret is never returned in credential resources."]
    pub refresh_token: CreateMcpOauthRefreshParamRefreshToken,
    #[doc = "The resource URI to send to the OAuth token endpoint during refresh, if required."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource: ::std::option::Option<CreateMcpOauthRefreshParamResource>,
    #[doc = "Space-separated OAuth scopes to request during refresh, if required."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<CreateMcpOauthRefreshParamScope>,
    #[doc = "The HTTPS OAuth token endpoint used to exchange the refresh token for a new access token."]
    pub token_endpoint: CreateMcpOauthRefreshParamTokenEndpoint,
    #[doc = "How the OAuth client authenticates to the token endpoint."]
    pub token_endpoint_auth: CreateMcpOauthTokenEndpointAuthParam,
}
impl CreateMcpOauthRefreshParam {
    pub fn builder() -> builder::CreateMcpOauthRefreshParam {
        Default::default()
    }
}
#[doc = "The OAuth client ID used when requesting a new access token."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthRefreshParamClientId(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthRefreshParamClientId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthRefreshParamClientId> for ::std::string::String {
    fn from(value: CreateMcpOauthRefreshParamClientId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthRefreshParamClientId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthRefreshParamClientId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMcpOauthRefreshParamClientId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateMcpOauthRefreshParamClientId {
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
#[doc = "The refresh token to store. This secret is never returned in credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthRefreshParamRefreshToken(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthRefreshParamRefreshToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthRefreshParamRefreshToken> for ::std::string::String {
    fn from(value: CreateMcpOauthRefreshParamRefreshToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthRefreshParamRefreshToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthRefreshParamRefreshToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMcpOauthRefreshParamRefreshToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateMcpOauthRefreshParamRefreshToken {
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
#[doc = "The resource URI to send to the OAuth token endpoint during refresh, if required."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthRefreshParamResource(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthRefreshParamResource {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthRefreshParamResource> for ::std::string::String {
    fn from(value: CreateMcpOauthRefreshParamResource) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthRefreshParamResource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthRefreshParamResource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMcpOauthRefreshParamResource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateMcpOauthRefreshParamResource {
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
#[doc = "Space-separated OAuth scopes to request during refresh, if required."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthRefreshParamScope(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthRefreshParamScope {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthRefreshParamScope> for ::std::string::String {
    fn from(value: CreateMcpOauthRefreshParamScope) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthRefreshParamScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthRefreshParamScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMcpOauthRefreshParamScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateMcpOauthRefreshParamScope {
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
#[doc = "The HTTPS OAuth token endpoint used to exchange the refresh token for a new access token."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthRefreshParamTokenEndpoint(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthRefreshParamTokenEndpoint {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthRefreshParamTokenEndpoint> for ::std::string::String {
    fn from(value: CreateMcpOauthRefreshParamTokenEndpoint) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthRefreshParamTokenEndpoint {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthRefreshParamTokenEndpoint {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateMcpOauthRefreshParamTokenEndpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateMcpOauthRefreshParamTokenEndpoint {
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
#[doc = "Client authentication credentials for OAuth token refresh."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateMcpOauthTokenEndpointAuthParam {
    None(CreateMcpOauthTokenEndpointAuthParamNone),
    ClientSecretBasic(CreateMcpOauthTokenEndpointAuthParamClientSecretBasic),
    ClientSecretPost(CreateMcpOauthTokenEndpointAuthParamClientSecretPost),
}
impl ::std::convert::From<CreateMcpOauthTokenEndpointAuthParamNone>
    for CreateMcpOauthTokenEndpointAuthParam
{
    fn from(value: CreateMcpOauthTokenEndpointAuthParamNone) -> Self {
        Self::None(value)
    }
}
impl ::std::convert::From<CreateMcpOauthTokenEndpointAuthParamClientSecretBasic>
    for CreateMcpOauthTokenEndpointAuthParam
{
    fn from(value: CreateMcpOauthTokenEndpointAuthParamClientSecretBasic) -> Self {
        Self::ClientSecretBasic(value)
    }
}
impl ::std::convert::From<CreateMcpOauthTokenEndpointAuthParamClientSecretPost>
    for CreateMcpOauthTokenEndpointAuthParam
{
    fn from(value: CreateMcpOauthTokenEndpointAuthParamClientSecretPost) -> Self {
        Self::ClientSecretPost(value)
    }
}
#[doc = "Sends the client ID and secret using HTTP Basic authentication."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
    #[doc = "The OAuth client secret to store. Never returned in credential resources."]
    pub client_secret: CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret,
    #[doc = "The type of the object. Always `client_secret_basic`."]
    #[serde(rename = "type")]
    pub type_: CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
}
impl CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
    pub fn builder() -> builder::CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        Default::default()
    }
}
#[doc = "The OAuth client secret to store. Never returned in credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret>
    for ::std::string::String
{
    fn from(value: CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
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
#[doc = "The type of the object. Always `client_secret_basic`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
}
impl ::std::fmt::Display for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretBasic => f.write_str("client_secret_basic"),
        }
    }
}
impl ::std::str::FromStr for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_basic" => Ok(Self::ClientSecretBasic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sends the client ID and secret in the token request body."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
    #[doc = "The OAuth client secret to store. Never returned in credential resources."]
    pub client_secret: CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret,
    #[doc = "The type of the object. Always `client_secret_post`."]
    #[serde(rename = "type")]
    pub type_: CreateMcpOauthTokenEndpointAuthParamClientSecretPostType,
}
impl CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
    pub fn builder() -> builder::CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
        Default::default()
    }
}
#[doc = "The OAuth client secret to store. Never returned in credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret(::std::string::String);
impl ::std::ops::Deref for CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret>
    for ::std::string::String
{
    fn from(value: CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
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
#[doc = "The type of the object. Always `client_secret_post`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
}
impl ::std::fmt::Display for CreateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretPost => f.write_str("client_secret_post"),
        }
    }
}
impl ::std::str::FromStr for CreateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_post" => Ok(Self::ClientSecretPost),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateMcpOauthTokenEndpointAuthParamClientSecretPostType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sends the client ID without a client secret."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateMcpOauthTokenEndpointAuthParamNone {
    #[doc = "The type of the object. Always `none`."]
    #[serde(rename = "type")]
    pub type_: CreateMcpOauthTokenEndpointAuthParamNoneType,
}
impl CreateMcpOauthTokenEndpointAuthParamNone {
    pub fn builder() -> builder::CreateMcpOauthTokenEndpointAuthParamNone {
        Default::default()
    }
}
#[doc = "The type of the object. Always `none`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateMcpOauthTokenEndpointAuthParamNoneType {
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for CreateMcpOauthTokenEndpointAuthParamNoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for CreateMcpOauthTokenEndpointAuthParamNoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateMcpOauthTokenEndpointAuthParamNoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateMcpOauthTokenEndpointAuthParamNoneType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Authentication credentials for an MCP server or an OpenAI-hosted environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateVaultCredentialAuthParam {
    McpOauth(CreateVaultCredentialAuthParamMcpOauth),
    StaticBearer(CreateVaultCredentialAuthParamStaticBearer),
    EnvironmentVariable(CreateVaultCredentialAuthParamEnvironmentVariable),
}
impl ::std::convert::From<CreateVaultCredentialAuthParamMcpOauth>
    for CreateVaultCredentialAuthParam
{
    fn from(value: CreateVaultCredentialAuthParamMcpOauth) -> Self {
        Self::McpOauth(value)
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamStaticBearer>
    for CreateVaultCredentialAuthParam
{
    fn from(value: CreateVaultCredentialAuthParamStaticBearer) -> Self {
        Self::StaticBearer(value)
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamEnvironmentVariable>
    for CreateVaultCredentialAuthParam
{
    fn from(value: CreateVaultCredentialAuthParamEnvironmentVariable) -> Self {
        Self::EnvironmentVariable(value)
    }
}
#[doc = "An HTTP credential for OpenAI-hosted environments only. The sandbox receives an environment variable containing a placeholder, not the secret. Use the placeholder unchanged in outgoing requests. The egress proxy replaces the placeholder with the secret for allowed HTTPS destinations on ports 443 and 8443. Sandbox code cannot read the real secret or use it for local computation, such as signing a request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVaultCredentialAuthParamEnvironmentVariable {
    #[doc = "The destinations where the proxy can substitute this secret. The environment network policy must also allow them."]
    pub networking: VaultCredentialNetworkingParam,
    #[doc = "The environment variable name that receives the placeholder, such as `SERVICE_API_KEY`. Use ASCII letters, digits, and underscores, starting with a letter or underscore. Names starting with `CODEX_` and managed proxy or certificate variable names are reserved."]
    pub secret_name: CreateVaultCredentialAuthParamEnvironmentVariableSecretName,
    #[doc = "The write-only secret to store. Never returned in credential resources or supplied directly to sandbox code. Must be nonempty and must not contain carriage returns, newlines, or NUL bytes."]
    pub secret_value: CreateVaultCredentialAuthParamEnvironmentVariableSecretValue,
    #[doc = "The type of the object. Always `environment_variable`."]
    #[serde(rename = "type")]
    pub type_: CreateVaultCredentialAuthParamEnvironmentVariableType,
}
impl CreateVaultCredentialAuthParamEnvironmentVariable {
    pub fn builder() -> builder::CreateVaultCredentialAuthParamEnvironmentVariable {
        Default::default()
    }
}
#[doc = "The environment variable name that receives the placeholder, such as `SERVICE_API_KEY`. Use ASCII letters, digits, and underscores, starting with a letter or underscore. Names starting with `CODEX_` and managed proxy or certificate variable names are reserved."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamEnvironmentVariableSecretName(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamEnvironmentVariableSecretName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamEnvironmentVariableSecretName>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamEnvironmentVariableSecretName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamEnvironmentVariableSecretName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamEnvironmentVariableSecretName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamEnvironmentVariableSecretName
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for CreateVaultCredentialAuthParamEnvironmentVariableSecretName
{
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
#[doc = "The write-only secret to store. Never returned in credential resources or supplied directly to sandbox code. Must be nonempty and must not contain carriage returns, newlines, or NUL bytes."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamEnvironmentVariableSecretValue(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamEnvironmentVariableSecretValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamEnvironmentVariableSecretValue>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamEnvironmentVariableSecretValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamEnvironmentVariableSecretValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for CreateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for CreateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
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
#[doc = "The type of the object. Always `environment_variable`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateVaultCredentialAuthParamEnvironmentVariableType {
    #[serde(rename = "environment_variable")]
    EnvironmentVariable,
}
impl ::std::fmt::Display for CreateVaultCredentialAuthParamEnvironmentVariableType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EnvironmentVariable => f.write_str("environment_variable"),
        }
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamEnvironmentVariableType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "environment_variable" => Ok(Self::EnvironmentVariable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamEnvironmentVariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamEnvironmentVariableType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An OAuth credential for an HTTPS MCP destination."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVaultCredentialAuthParamMcpOauth {
    #[doc = "A write-only OAuth access token; never returned by credential resources."]
    pub access_token: CreateVaultCredentialAuthParamMcpOauthAccessToken,
    #[doc = "When the OAuth access token expires, as an RFC 3339 timestamp, if known."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<CreateVaultCredentialAuthParamMcpOauthExpiresAt>,
    #[doc = "The HTTPS MCP server URL authorized by this credential."]
    pub mcp_server_url: CreateVaultCredentialAuthParamMcpOauthMcpServerUrl,
    #[doc = "Optional refresh configuration for an HTTPS OAuth token endpoint."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<CreateMcpOauthRefreshParam>,
    #[doc = "The type of the object. Always `mcp_oauth`."]
    #[serde(rename = "type")]
    pub type_: CreateVaultCredentialAuthParamMcpOauthType,
}
impl CreateVaultCredentialAuthParamMcpOauth {
    pub fn builder() -> builder::CreateVaultCredentialAuthParamMcpOauth {
        Default::default()
    }
}
#[doc = "A write-only OAuth access token; never returned by credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamMcpOauthAccessToken(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamMcpOauthAccessToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamMcpOauthAccessToken>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamMcpOauthAccessToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamMcpOauthAccessToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamMcpOauthAccessToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamMcpOauthAccessToken
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialAuthParamMcpOauthAccessToken {
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
#[doc = "When the OAuth access token expires, as an RFC 3339 timestamp, if known."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamMcpOauthExpiresAt(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamMcpOauthExpiresAt>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamMcpOauthExpiresAt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamMcpOauthExpiresAt
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialAuthParamMcpOauthExpiresAt {
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
#[doc = "The HTTPS MCP server URL authorized by this credential."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamMcpOauthMcpServerUrl(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamMcpOauthMcpServerUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamMcpOauthMcpServerUrl>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamMcpOauthMcpServerUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamMcpOauthMcpServerUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamMcpOauthMcpServerUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamMcpOauthMcpServerUrl
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialAuthParamMcpOauthMcpServerUrl {
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
#[doc = "The type of the object. Always `mcp_oauth`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateVaultCredentialAuthParamMcpOauthType {
    #[serde(rename = "mcp_oauth")]
    McpOauth,
}
impl ::std::fmt::Display for CreateVaultCredentialAuthParamMcpOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpOauth => f.write_str("mcp_oauth"),
        }
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamMcpOauthType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_oauth" => Ok(Self::McpOauth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultCredentialAuthParamMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A bearer token for an MCP server, without automatic OAuth refresh."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVaultCredentialAuthParamStaticBearer {
    #[doc = "The HTTPS MCP server URL authorized by this credential."]
    pub mcp_server_url: CreateVaultCredentialAuthParamStaticBearerMcpServerUrl,
    #[doc = "The bearer token to store. This secret is never returned in credential resources."]
    pub token: CreateVaultCredentialAuthParamStaticBearerToken,
    #[doc = "The type of the object. Always `static_bearer`."]
    #[serde(rename = "type")]
    pub type_: CreateVaultCredentialAuthParamStaticBearerType,
}
impl CreateVaultCredentialAuthParamStaticBearer {
    pub fn builder() -> builder::CreateVaultCredentialAuthParamStaticBearer {
        Default::default()
    }
}
#[doc = "The HTTPS MCP server URL authorized by this credential."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamStaticBearerMcpServerUrl(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamStaticBearerMcpServerUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamStaticBearerMcpServerUrl>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamStaticBearerMcpServerUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamStaticBearerMcpServerUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamStaticBearerMcpServerUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamStaticBearerMcpServerUrl
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialAuthParamStaticBearerMcpServerUrl {
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
#[doc = "The bearer token to store. This secret is never returned in credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialAuthParamStaticBearerToken(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialAuthParamStaticBearerToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialAuthParamStaticBearerToken>
    for ::std::string::String
{
    fn from(value: CreateVaultCredentialAuthParamStaticBearerToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamStaticBearerToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamStaticBearerToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamStaticBearerToken
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialAuthParamStaticBearerToken {
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
#[doc = "The type of the object. Always `static_bearer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateVaultCredentialAuthParamStaticBearerType {
    #[serde(rename = "static_bearer")]
    StaticBearer,
}
impl ::std::fmt::Display for CreateVaultCredentialAuthParamStaticBearerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StaticBearer => f.write_str("static_bearer"),
        }
    }
}
impl ::std::str::FromStr for CreateVaultCredentialAuthParamStaticBearerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "static_bearer" => Ok(Self::StaticBearer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialAuthParamStaticBearerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for CreateVaultCredentialAuthParamStaticBearerType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Parameters for storing a credential for an MCP server or an OpenAI-hosted environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVaultCredentialParams {
    #[doc = "The authentication method and write-only secret values to store."]
    pub auth: CreateVaultCredentialAuthParam,
    #[doc = "Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters. Defaults to an empty map."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub metadata: ::std::collections::HashMap<
        CreateVaultCredentialParamsMetadataKey,
        CreateVaultCredentialParamsMetadataValue,
    >,
    #[doc = "The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
    pub name: CreateVaultCredentialParamsName,
}
impl CreateVaultCredentialParams {
    pub fn builder() -> builder::CreateVaultCredentialParams {
        Default::default()
    }
}
#[doc = "`CreateVaultCredentialParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialParamsMetadataKey> for ::std::string::String {
    fn from(value: CreateVaultCredentialParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultCredentialParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialParamsMetadataKey {
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
#[doc = "`CreateVaultCredentialParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialParamsMetadataValue> for ::std::string::String {
    fn from(value: CreateVaultCredentialParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultCredentialParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialParamsMetadataValue {
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
#[doc = "The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialParamsName(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialParamsName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialParamsName> for ::std::string::String {
    fn from(value: CreateVaultCredentialParamsName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialParamsName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialParamsName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultCredentialParamsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialParamsName {
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
#[doc = "`CreateVaultCredentialParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultCredentialParamsVaultId(::std::string::String);
impl ::std::ops::Deref for CreateVaultCredentialParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultCredentialParamsVaultId> for ::std::string::String {
    fn from(value: CreateVaultCredentialParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultCredentialParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultCredentialParamsVaultId {
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
#[doc = "Parameters for creating a vault to store credentials used by agent tools."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateVaultParams {
    #[doc = "Key-value pairs to associate with the vault, such as an application or team identifier."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<
        ::std::collections::HashMap<CreateVaultParamsMetadataKey, CreateVaultParamsMetadataValue>,
    >,
    #[doc = "The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<CreateVaultParamsName>,
}
impl CreateVaultParams {
    pub fn builder() -> builder::CreateVaultParams {
        Default::default()
    }
}
#[doc = "`CreateVaultParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for CreateVaultParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultParamsMetadataKey> for ::std::string::String {
    fn from(value: CreateVaultParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for CreateVaultParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultParamsMetadataKey {
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
#[doc = "`CreateVaultParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for CreateVaultParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultParamsMetadataValue> for ::std::string::String {
    fn from(value: CreateVaultParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultParamsMetadataValue {
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
#[doc = "The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVaultParamsName(::std::string::String);
impl ::std::ops::Deref for CreateVaultParamsName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVaultParamsName> for ::std::string::String {
    fn from(value: CreateVaultParamsName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVaultParamsName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVaultParamsName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVaultParamsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVaultParamsName {
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
#[doc = "`DeleteVaultCredentialParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVaultCredentialParams {
    pub credential_id: DeleteVaultCredentialParamsCredentialId,
    pub vault_id: DeleteVaultCredentialParamsVaultId,
}
impl DeleteVaultCredentialParams {
    pub fn builder() -> builder::DeleteVaultCredentialParams {
        Default::default()
    }
}
#[doc = "`DeleteVaultCredentialParamsCredentialId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DeleteVaultCredentialParamsCredentialId(::std::string::String);
impl ::std::ops::Deref for DeleteVaultCredentialParamsCredentialId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DeleteVaultCredentialParamsCredentialId> for ::std::string::String {
    fn from(value: DeleteVaultCredentialParamsCredentialId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DeleteVaultCredentialParamsCredentialId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DeleteVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteVaultCredentialParamsCredentialId {
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
#[doc = "`DeleteVaultCredentialParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DeleteVaultCredentialParamsVaultId(::std::string::String);
impl ::std::ops::Deref for DeleteVaultCredentialParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DeleteVaultCredentialParamsVaultId> for ::std::string::String {
    fn from(value: DeleteVaultCredentialParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DeleteVaultCredentialParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DeleteVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteVaultCredentialParamsVaultId {
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
#[doc = "`DeleteVaultParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVaultParams {
    pub vault_id: DeleteVaultParamsVaultId,
}
impl DeleteVaultParams {
    pub fn builder() -> builder::DeleteVaultParams {
        Default::default()
    }
}
#[doc = "`DeleteVaultParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DeleteVaultParamsVaultId(::std::string::String);
impl ::std::ops::Deref for DeleteVaultParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DeleteVaultParamsVaultId> for ::std::string::String {
    fn from(value: DeleteVaultParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DeleteVaultParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DeleteVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteVaultParamsVaultId {
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
#[doc = "Confirmation that a vault credential was deleted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeletedVaultCredentialResource {
    #[doc = "Whether the resource was deleted. Always `true`."]
    pub deleted: bool,
    #[doc = "The ID of the deleted credential."]
    pub id: ::std::string::String,
    #[doc = "The object type. Always `vault.credential.deleted`."]
    pub object: DeletedVaultCredentialResourceObject,
}
impl DeletedVaultCredentialResource {
    pub fn builder() -> builder::DeletedVaultCredentialResource {
        Default::default()
    }
}
#[doc = "The object type. Always `vault.credential.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedVaultCredentialResourceObject {
    #[serde(rename = "vault.credential.deleted")]
    VaultCredentialDeleted,
}
impl ::std::fmt::Display for DeletedVaultCredentialResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VaultCredentialDeleted => f.write_str("vault.credential.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedVaultCredentialResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vault.credential.deleted" => Ok(Self::VaultCredentialDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedVaultCredentialResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedVaultCredentialResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Confirmation that a vault was deleted."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeletedVaultResource {
    #[doc = "Whether the resource was deleted. Always `true`."]
    pub deleted: bool,
    #[doc = "The ID of the deleted vault."]
    pub id: ::std::string::String,
    #[doc = "The object type. Always `vault.deleted`."]
    pub object: DeletedVaultResourceObject,
}
impl DeletedVaultResource {
    pub fn builder() -> builder::DeletedVaultResource {
        Default::default()
    }
}
#[doc = "The object type. Always `vault.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedVaultResourceObject {
    #[serde(rename = "vault.deleted")]
    VaultDeleted,
}
impl ::std::fmt::Display for DeletedVaultResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VaultDeleted => f.write_str("vault.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedVaultResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vault.deleted" => Ok(Self::VaultDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedVaultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedVaultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListVaultCredentialsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListVaultCredentialsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<ListVaultCredentialsParamsAfter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub metadata: ::std::collections::HashMap<
        ListVaultCredentialsParamsMetadataKey,
        ListVaultCredentialsParamsMetadataValue,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::ListOrderParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<VaultStatusFilterParam>,
    pub vault_id: ListVaultCredentialsParamsVaultId,
}
impl ListVaultCredentialsParams {
    pub fn builder() -> builder::ListVaultCredentialsParams {
        Default::default()
    }
}
#[doc = "`ListVaultCredentialsParamsAfter`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultCredentialsParamsAfter(::std::string::String);
impl ::std::ops::Deref for ListVaultCredentialsParamsAfter {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultCredentialsParamsAfter> for ::std::string::String {
    fn from(value: ListVaultCredentialsParamsAfter) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultCredentialsParamsAfter {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListVaultCredentialsParamsAfter {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultCredentialsParamsAfter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultCredentialsParamsAfter {
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
#[doc = "`ListVaultCredentialsParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultCredentialsParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for ListVaultCredentialsParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultCredentialsParamsMetadataKey> for ::std::string::String {
    fn from(value: ListVaultCredentialsParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultCredentialsParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for ListVaultCredentialsParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultCredentialsParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultCredentialsParamsMetadataKey {
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
#[doc = "`ListVaultCredentialsParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultCredentialsParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for ListVaultCredentialsParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultCredentialsParamsMetadataValue> for ::std::string::String {
    fn from(value: ListVaultCredentialsParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultCredentialsParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListVaultCredentialsParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultCredentialsParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultCredentialsParamsMetadataValue {
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
#[doc = "`ListVaultCredentialsParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultCredentialsParamsVaultId(::std::string::String);
impl ::std::ops::Deref for ListVaultCredentialsParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultCredentialsParamsVaultId> for ::std::string::String {
    fn from(value: ListVaultCredentialsParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultCredentialsParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListVaultCredentialsParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultCredentialsParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultCredentialsParamsVaultId {
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
#[doc = "`ListVaultsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListVaultsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<ListVaultsParamsAfter>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub metadata:
        ::std::collections::HashMap<ListVaultsParamsMetadataKey, ListVaultsParamsMetadataValue>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::ListOrderParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<VaultStatusFilterParam>,
}
impl ListVaultsParams {
    pub fn builder() -> builder::ListVaultsParams {
        Default::default()
    }
}
#[doc = "`ListVaultsParamsAfter`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultsParamsAfter(::std::string::String);
impl ::std::ops::Deref for ListVaultsParamsAfter {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultsParamsAfter> for ::std::string::String {
    fn from(value: ListVaultsParamsAfter) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultsParamsAfter {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListVaultsParamsAfter {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultsParamsAfter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultsParamsAfter {
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
#[doc = "`ListVaultsParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultsParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for ListVaultsParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultsParamsMetadataKey> for ::std::string::String {
    fn from(value: ListVaultsParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultsParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for ListVaultsParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultsParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultsParamsMetadataKey {
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
#[doc = "`ListVaultsParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ListVaultsParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for ListVaultsParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListVaultsParamsMetadataValue> for ::std::string::String {
    fn from(value: ListVaultsParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListVaultsParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListVaultsParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListVaultsParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListVaultsParamsMetadataValue {
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
#[doc = "Configuration used to refresh an MCP OAuth access token, excluding secret values."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpOauthRefreshResource {
    #[doc = "The OAuth client ID used when requesting a new access token."]
    pub client_id: ::std::string::String,
    #[doc = "The resource URI sent to the OAuth token endpoint during refresh, if configured."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub resource: ::std::option::Option<::std::string::String>,
    #[doc = "Space-separated OAuth scopes requested during refresh, if configured."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub scope: ::std::option::Option<::std::string::String>,
    #[doc = "The HTTPS OAuth token endpoint used for refresh."]
    pub token_endpoint: ::std::string::String,
    #[doc = "How the OAuth client authenticates to the token endpoint, excluding its client secret."]
    pub token_endpoint_auth: McpOauthTokenEndpointAuthResource,
}
impl McpOauthRefreshResource {
    pub fn builder() -> builder::McpOauthRefreshResource {
        Default::default()
    }
}
#[doc = "The client authentication method used for OAuth token refresh."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum McpOauthTokenEndpointAuthResource {
    None(McpOauthTokenEndpointAuthResourceNone),
    ClientSecretBasic(McpOauthTokenEndpointAuthResourceClientSecretBasic),
    ClientSecretPost(McpOauthTokenEndpointAuthResourceClientSecretPost),
}
impl ::std::convert::From<McpOauthTokenEndpointAuthResourceNone>
    for McpOauthTokenEndpointAuthResource
{
    fn from(value: McpOauthTokenEndpointAuthResourceNone) -> Self {
        Self::None(value)
    }
}
impl ::std::convert::From<McpOauthTokenEndpointAuthResourceClientSecretBasic>
    for McpOauthTokenEndpointAuthResource
{
    fn from(value: McpOauthTokenEndpointAuthResourceClientSecretBasic) -> Self {
        Self::ClientSecretBasic(value)
    }
}
impl ::std::convert::From<McpOauthTokenEndpointAuthResourceClientSecretPost>
    for McpOauthTokenEndpointAuthResource
{
    fn from(value: McpOauthTokenEndpointAuthResourceClientSecretPost) -> Self {
        Self::ClientSecretPost(value)
    }
}
#[doc = "Sends the client ID and secret using HTTP Basic authentication."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpOauthTokenEndpointAuthResourceClientSecretBasic {
    #[doc = "The type of the object. Always `client_secret_basic`."]
    #[serde(rename = "type")]
    pub type_: McpOauthTokenEndpointAuthResourceClientSecretBasicType,
}
impl McpOauthTokenEndpointAuthResourceClientSecretBasic {
    pub fn builder() -> builder::McpOauthTokenEndpointAuthResourceClientSecretBasic {
        Default::default()
    }
}
#[doc = "The type of the object. Always `client_secret_basic`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpOauthTokenEndpointAuthResourceClientSecretBasicType {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
}
impl ::std::fmt::Display for McpOauthTokenEndpointAuthResourceClientSecretBasicType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretBasic => f.write_str("client_secret_basic"),
        }
    }
}
impl ::std::str::FromStr for McpOauthTokenEndpointAuthResourceClientSecretBasicType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_basic" => Ok(Self::ClientSecretBasic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpOauthTokenEndpointAuthResourceClientSecretBasicType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for McpOauthTokenEndpointAuthResourceClientSecretBasicType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sends the client ID and secret in the token request body."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpOauthTokenEndpointAuthResourceClientSecretPost {
    #[doc = "The type of the object. Always `client_secret_post`."]
    #[serde(rename = "type")]
    pub type_: McpOauthTokenEndpointAuthResourceClientSecretPostType,
}
impl McpOauthTokenEndpointAuthResourceClientSecretPost {
    pub fn builder() -> builder::McpOauthTokenEndpointAuthResourceClientSecretPost {
        Default::default()
    }
}
#[doc = "The type of the object. Always `client_secret_post`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpOauthTokenEndpointAuthResourceClientSecretPostType {
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
}
impl ::std::fmt::Display for McpOauthTokenEndpointAuthResourceClientSecretPostType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretPost => f.write_str("client_secret_post"),
        }
    }
}
impl ::std::str::FromStr for McpOauthTokenEndpointAuthResourceClientSecretPostType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_post" => Ok(Self::ClientSecretPost),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpOauthTokenEndpointAuthResourceClientSecretPostType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for McpOauthTokenEndpointAuthResourceClientSecretPostType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Sends the client ID without a client secret."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct McpOauthTokenEndpointAuthResourceNone {
    #[doc = "The type of the object. Always `none`."]
    #[serde(rename = "type")]
    pub type_: McpOauthTokenEndpointAuthResourceNoneType,
}
impl McpOauthTokenEndpointAuthResourceNone {
    pub fn builder() -> builder::McpOauthTokenEndpointAuthResourceNone {
        Default::default()
    }
}
#[doc = "The type of the object. Always `none`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum McpOauthTokenEndpointAuthResourceNoneType {
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for McpOauthTokenEndpointAuthResourceNoneType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for McpOauthTokenEndpointAuthResourceNoneType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpOauthTokenEndpointAuthResourceNoneType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpOauthTokenEndpointAuthResourceNoneType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveVaultCredentialParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveVaultCredentialParams {
    pub credential_id: RetrieveVaultCredentialParamsCredentialId,
    pub vault_id: RetrieveVaultCredentialParamsVaultId,
}
impl RetrieveVaultCredentialParams {
    pub fn builder() -> builder::RetrieveVaultCredentialParams {
        Default::default()
    }
}
#[doc = "`RetrieveVaultCredentialParamsCredentialId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RetrieveVaultCredentialParamsCredentialId(::std::string::String);
impl ::std::ops::Deref for RetrieveVaultCredentialParamsCredentialId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RetrieveVaultCredentialParamsCredentialId> for ::std::string::String {
    fn from(value: RetrieveVaultCredentialParamsCredentialId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RetrieveVaultCredentialParamsCredentialId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RetrieveVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RetrieveVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RetrieveVaultCredentialParamsCredentialId {
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
#[doc = "`RetrieveVaultCredentialParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RetrieveVaultCredentialParamsVaultId(::std::string::String);
impl ::std::ops::Deref for RetrieveVaultCredentialParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RetrieveVaultCredentialParamsVaultId> for ::std::string::String {
    fn from(value: RetrieveVaultCredentialParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RetrieveVaultCredentialParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RetrieveVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RetrieveVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RetrieveVaultCredentialParamsVaultId {
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
#[doc = "`RetrieveVaultParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveVaultParams {
    pub vault_id: RetrieveVaultParamsVaultId,
}
impl RetrieveVaultParams {
    pub fn builder() -> builder::RetrieveVaultParams {
        Default::default()
    }
}
#[doc = "`RetrieveVaultParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RetrieveVaultParamsVaultId(::std::string::String);
impl ::std::ops::Deref for RetrieveVaultParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RetrieveVaultParamsVaultId> for ::std::string::String {
    fn from(value: RetrieveVaultParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RetrieveVaultParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RetrieveVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RetrieveVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RetrieveVaultParamsVaultId {
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
#[doc = "Updates to an MCP credential's existing OAuth refresh configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateMcpOauthRefreshParam {
    #[doc = "The replacement refresh token. Omit or pass `null` to keep the stored token. This secret is never returned in resources."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh_token: ::std::option::Option<RotateMcpOauthRefreshParamRefreshToken>,
    #[doc = "Replacement space-separated OAuth scopes for refresh requests. Omit to keep the scopes, or pass `null` to stop sending a scope parameter."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<RotateMcpOauthRefreshParamScope>,
    #[doc = "Client-secret updates for the existing token endpoint authentication method."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token_endpoint_auth: ::std::option::Option<RotateMcpOauthTokenEndpointAuthParam>,
}
impl RotateMcpOauthRefreshParam {
    pub fn builder() -> builder::RotateMcpOauthRefreshParam {
        Default::default()
    }
}
#[doc = "The replacement refresh token. Omit or pass `null` to keep the stored token. This secret is never returned in resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateMcpOauthRefreshParamRefreshToken(::std::string::String);
impl ::std::ops::Deref for RotateMcpOauthRefreshParamRefreshToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateMcpOauthRefreshParamRefreshToken> for ::std::string::String {
    fn from(value: RotateMcpOauthRefreshParamRefreshToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateMcpOauthRefreshParamRefreshToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateMcpOauthRefreshParamRefreshToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateMcpOauthRefreshParamRefreshToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateMcpOauthRefreshParamRefreshToken {
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
#[doc = "Replacement space-separated OAuth scopes for refresh requests. Omit to keep the scopes, or pass `null` to stop sending a scope parameter."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateMcpOauthRefreshParamScope(::std::string::String);
impl ::std::ops::Deref for RotateMcpOauthRefreshParamScope {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateMcpOauthRefreshParamScope> for ::std::string::String {
    fn from(value: RotateMcpOauthRefreshParamScope) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateMcpOauthRefreshParamScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateMcpOauthRefreshParamScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateMcpOauthRefreshParamScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateMcpOauthRefreshParamScope {
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
#[doc = "Client-secret updates that preserve the credential's OAuth authentication method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RotateMcpOauthTokenEndpointAuthParam {
    Basic(RotateMcpOauthTokenEndpointAuthParamClientSecretBasic),
    Post(RotateMcpOauthTokenEndpointAuthParamClientSecretPost),
}
impl ::std::convert::From<RotateMcpOauthTokenEndpointAuthParamClientSecretBasic>
    for RotateMcpOauthTokenEndpointAuthParam
{
    fn from(value: RotateMcpOauthTokenEndpointAuthParamClientSecretBasic) -> Self {
        Self::Basic(value)
    }
}
impl ::std::convert::From<RotateMcpOauthTokenEndpointAuthParamClientSecretPost>
    for RotateMcpOauthTokenEndpointAuthParam
{
    fn from(value: RotateMcpOauthTokenEndpointAuthParamClientSecretPost) -> Self {
        Self::Post(value)
    }
}
#[doc = "Updates credentials sent using HTTP Basic authentication."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
    #[doc = "The replacement OAuth client secret. Omit or pass `null` to keep the stored secret. This secret is never returned in resources."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_secret:
        ::std::option::Option<RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret>,
    #[doc = "The type of the object. Always `client_secret_basic`."]
    #[serde(rename = "type")]
    pub type_: RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
}
impl RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
    pub fn builder() -> builder::RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        Default::default()
    }
}
#[doc = "The replacement OAuth client secret. Omit or pass `null` to keep the stored secret. This secret is never returned in resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret(::std::string::String);
impl ::std::ops::Deref for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret>
    for ::std::string::String
{
    fn from(value: RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret
{
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
#[doc = "The type of the object. Always `client_secret_basic`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
}
impl ::std::fmt::Display for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretBasic => f.write_str("client_secret_basic"),
        }
    }
}
impl ::std::str::FromStr for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_basic" => Ok(Self::ClientSecretBasic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Updates credentials sent in the token request body."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
    #[doc = "The replacement OAuth client secret. Omit or pass `null` to keep the stored secret. This secret is never returned in resources."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_secret:
        ::std::option::Option<RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret>,
    #[doc = "The type of the object. Always `client_secret_post`."]
    #[serde(rename = "type")]
    pub type_: RotateMcpOauthTokenEndpointAuthParamClientSecretPostType,
}
impl RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
    pub fn builder() -> builder::RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
        Default::default()
    }
}
#[doc = "The replacement OAuth client secret. Omit or pass `null` to keep the stored secret. This secret is never returned in resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret(::std::string::String);
impl ::std::ops::Deref for RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret>
    for ::std::string::String
{
    fn from(value: RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret
{
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
#[doc = "The type of the object. Always `client_secret_post`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RotateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
}
impl ::std::fmt::Display for RotateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientSecretPost => f.write_str("client_secret_post"),
        }
    }
}
impl ::std::str::FromStr for RotateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_secret_post" => Ok(Self::ClientSecretPost),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RotateMcpOauthTokenEndpointAuthParamClientSecretPostType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateMcpOauthTokenEndpointAuthParamClientSecretPostType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Updates to a vault credential without changing its authentication method or destination configuration."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RotateVaultCredentialAuthParam {
    McpOauth(RotateVaultCredentialAuthParamMcpOauth),
    StaticBearer(RotateVaultCredentialAuthParamStaticBearer),
    EnvironmentVariable(RotateVaultCredentialAuthParamEnvironmentVariable),
}
impl ::std::convert::From<RotateVaultCredentialAuthParamMcpOauth>
    for RotateVaultCredentialAuthParam
{
    fn from(value: RotateVaultCredentialAuthParamMcpOauth) -> Self {
        Self::McpOauth(value)
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamStaticBearer>
    for RotateVaultCredentialAuthParam
{
    fn from(value: RotateVaultCredentialAuthParamStaticBearer) -> Self {
        Self::StaticBearer(value)
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamEnvironmentVariable>
    for RotateVaultCredentialAuthParam
{
    fn from(value: RotateVaultCredentialAuthParamEnvironmentVariable) -> Self {
        Self::EnvironmentVariable(value)
    }
}
#[doc = "Replace the secret for an OpenAI-hosted environment credential. The environment variable name and networking configuration remain unchanged."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateVaultCredentialAuthParamEnvironmentVariable {
    #[doc = "The write-only replacement secret. Never returned in credential resources or supplied directly to sandbox code. Must be nonempty and must not contain carriage returns, newlines, or NUL bytes."]
    pub secret_value: RotateVaultCredentialAuthParamEnvironmentVariableSecretValue,
    #[doc = "The type of the object. Always `environment_variable`."]
    #[serde(rename = "type")]
    pub type_: RotateVaultCredentialAuthParamEnvironmentVariableType,
}
impl RotateVaultCredentialAuthParamEnvironmentVariable {
    pub fn builder() -> builder::RotateVaultCredentialAuthParamEnvironmentVariable {
        Default::default()
    }
}
#[doc = "The write-only replacement secret. Never returned in credential resources or supplied directly to sandbox code. Must be nonempty and must not contain carriage returns, newlines, or NUL bytes."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialAuthParamEnvironmentVariableSecretValue(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialAuthParamEnvironmentVariableSecretValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamEnvironmentVariableSecretValue>
    for ::std::string::String
{
    fn from(value: RotateVaultCredentialAuthParamEnvironmentVariableSecretValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamEnvironmentVariableSecretValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
    for RotateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
    for RotateVaultCredentialAuthParamEnvironmentVariableSecretValue
{
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
#[doc = "The type of the object. Always `environment_variable`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RotateVaultCredentialAuthParamEnvironmentVariableType {
    #[serde(rename = "environment_variable")]
    EnvironmentVariable,
}
impl ::std::fmt::Display for RotateVaultCredentialAuthParamEnvironmentVariableType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EnvironmentVariable => f.write_str("environment_variable"),
        }
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamEnvironmentVariableType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "environment_variable" => Ok(Self::EnvironmentVariable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamEnvironmentVariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamEnvironmentVariableType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Rotate an OAuth credential for an HTTPS MCP destination."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateVaultCredentialAuthParamMcpOauth {
    #[doc = "A write-only replacement OAuth access token."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub access_token: ::std::option::Option<RotateVaultCredentialAuthParamMcpOauthAccessToken>,
    #[doc = "The replacement expiry as an RFC 3339 timestamp, or `null` to clear it. Omitting this field preserves the expiry unless a new access token is supplied, in which case the expiry is cleared."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<RotateVaultCredentialAuthParamMcpOauthExpiresAt>,
    #[doc = "Optional write-only refresh-token and client-secret updates."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<RotateMcpOauthRefreshParam>,
    #[doc = "The type of the object. Always `mcp_oauth`."]
    #[serde(rename = "type")]
    pub type_: RotateVaultCredentialAuthParamMcpOauthType,
}
impl RotateVaultCredentialAuthParamMcpOauth {
    pub fn builder() -> builder::RotateVaultCredentialAuthParamMcpOauth {
        Default::default()
    }
}
#[doc = "A write-only replacement OAuth access token."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialAuthParamMcpOauthAccessToken(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialAuthParamMcpOauthAccessToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamMcpOauthAccessToken>
    for ::std::string::String
{
    fn from(value: RotateVaultCredentialAuthParamMcpOauthAccessToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamMcpOauthAccessToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamMcpOauthAccessToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamMcpOauthAccessToken
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialAuthParamMcpOauthAccessToken {
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
#[doc = "The replacement expiry as an RFC 3339 timestamp, or `null` to clear it. Omitting this field preserves the expiry unless a new access token is supplied, in which case the expiry is cleared."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialAuthParamMcpOauthExpiresAt(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamMcpOauthExpiresAt>
    for ::std::string::String
{
    fn from(value: RotateVaultCredentialAuthParamMcpOauthExpiresAt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamMcpOauthExpiresAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamMcpOauthExpiresAt
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialAuthParamMcpOauthExpiresAt {
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
#[doc = "The type of the object. Always `mcp_oauth`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RotateVaultCredentialAuthParamMcpOauthType {
    #[serde(rename = "mcp_oauth")]
    McpOauth,
}
impl ::std::fmt::Display for RotateVaultCredentialAuthParamMcpOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpOauth => f.write_str("mcp_oauth"),
        }
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamMcpOauthType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_oauth" => Ok(Self::McpOauth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateVaultCredentialAuthParamMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Replace the bearer token for the credential's MCP server."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateVaultCredentialAuthParamStaticBearer {
    #[doc = "The replacement bearer token. This secret is never returned in credential resources."]
    pub token: RotateVaultCredentialAuthParamStaticBearerToken,
    #[doc = "The type of the object. Always `static_bearer`."]
    #[serde(rename = "type")]
    pub type_: RotateVaultCredentialAuthParamStaticBearerType,
}
impl RotateVaultCredentialAuthParamStaticBearer {
    pub fn builder() -> builder::RotateVaultCredentialAuthParamStaticBearer {
        Default::default()
    }
}
#[doc = "The replacement bearer token. This secret is never returned in credential resources."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialAuthParamStaticBearerToken(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialAuthParamStaticBearerToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialAuthParamStaticBearerToken>
    for ::std::string::String
{
    fn from(value: RotateVaultCredentialAuthParamStaticBearerToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamStaticBearerToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamStaticBearerToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamStaticBearerToken
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialAuthParamStaticBearerToken {
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
#[doc = "The type of the object. Always `static_bearer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RotateVaultCredentialAuthParamStaticBearerType {
    #[serde(rename = "static_bearer")]
    StaticBearer,
}
impl ::std::fmt::Display for RotateVaultCredentialAuthParamStaticBearerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StaticBearer => f.write_str("static_bearer"),
        }
    }
}
impl ::std::str::FromStr for RotateVaultCredentialAuthParamStaticBearerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "static_bearer" => Ok(Self::StaticBearer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialAuthParamStaticBearerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for RotateVaultCredentialAuthParamStaticBearerType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Metadata, secret, expiry, and OAuth refresh scope updates for an existing vault credential. Supply at least one of `auth` or `metadata`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RotateVaultCredentialParams {
    #[doc = "Replacement values for the credential's existing authentication method."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auth: ::std::option::Option<RotateVaultCredentialAuthParam>,
    #[doc = "Replaces all metadata. Omit to preserve it, or pass {} to clear it. Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub metadata: ::std::collections::HashMap<
        RotateVaultCredentialParamsMetadataKey,
        RotateVaultCredentialParamsMetadataValue,
    >,
}
impl RotateVaultCredentialParams {
    pub fn builder() -> builder::RotateVaultCredentialParams {
        Default::default()
    }
}
#[doc = "`RotateVaultCredentialParamsCredentialId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialParamsCredentialId(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialParamsCredentialId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialParamsCredentialId> for ::std::string::String {
    fn from(value: RotateVaultCredentialParamsCredentialId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialParamsCredentialId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateVaultCredentialParamsCredentialId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialParamsCredentialId {
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
#[doc = "`RotateVaultCredentialParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialParamsMetadataKey> for ::std::string::String {
    fn from(value: RotateVaultCredentialParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateVaultCredentialParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialParamsMetadataKey {
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
#[doc = "`RotateVaultCredentialParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialParamsMetadataValue> for ::std::string::String {
    fn from(value: RotateVaultCredentialParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateVaultCredentialParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialParamsMetadataValue {
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
#[doc = "`RotateVaultCredentialParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RotateVaultCredentialParamsVaultId(::std::string::String);
impl ::std::ops::Deref for RotateVaultCredentialParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RotateVaultCredentialParamsVaultId> for ::std::string::String {
    fn from(value: RotateVaultCredentialParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RotateVaultCredentialParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RotateVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RotateVaultCredentialParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RotateVaultCredentialParamsVaultId {
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
#[doc = "Fields to replace on an active vault."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpdateVaultParams {
    #[doc = "Replaces all metadata. Omit to leave unchanged, or pass {} to clear it. Up to 16 string key-value pairs, with keys up to 64 and values up to 512 characters."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub metadata:
        ::std::collections::HashMap<UpdateVaultParamsMetadataKey, UpdateVaultParamsMetadataValue>,
    #[doc = "A replacement name. Omit to leave unchanged, or pass null to clear it. The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<UpdateVaultParamsName>,
}
impl UpdateVaultParams {
    pub fn builder() -> builder::UpdateVaultParams {
        Default::default()
    }
}
#[doc = "`UpdateVaultParamsMetadataKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateVaultParamsMetadataKey(::std::string::String);
impl ::std::ops::Deref for UpdateVaultParamsMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UpdateVaultParamsMetadataKey> for ::std::string::String {
    fn from(value: UpdateVaultParamsMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UpdateVaultParamsMetadataKey {
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
impl ::std::convert::TryFrom<&str> for UpdateVaultParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateVaultParamsMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UpdateVaultParamsMetadataKey {
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
#[doc = "`UpdateVaultParamsMetadataValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateVaultParamsMetadataValue(::std::string::String);
impl ::std::ops::Deref for UpdateVaultParamsMetadataValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UpdateVaultParamsMetadataValue> for ::std::string::String {
    fn from(value: UpdateVaultParamsMetadataValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UpdateVaultParamsMetadataValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UpdateVaultParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateVaultParamsMetadataValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UpdateVaultParamsMetadataValue {
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
#[doc = "A replacement name. Omit to leave unchanged, or pass null to clear it. The name is trimmed before storage. It must contain 1 to 256 UTF-8 bytes after trimming."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateVaultParamsName(::std::string::String);
impl ::std::ops::Deref for UpdateVaultParamsName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UpdateVaultParamsName> for ::std::string::String {
    fn from(value: UpdateVaultParamsName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UpdateVaultParamsName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UpdateVaultParamsName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateVaultParamsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UpdateVaultParamsName {
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
#[doc = "`UpdateVaultParamsVaultId`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateVaultParamsVaultId(::std::string::String);
impl ::std::ops::Deref for UpdateVaultParamsVaultId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UpdateVaultParamsVaultId> for ::std::string::String {
    fn from(value: UpdateVaultParamsVaultId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UpdateVaultParamsVaultId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UpdateVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpdateVaultParamsVaultId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UpdateVaultParamsVaultId {
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
#[doc = "The authentication configuration of a vault credential, excluding secrets."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VaultCredentialAuthResource {
    McpOauth(VaultCredentialAuthResourceMcpOauth),
    StaticBearer(VaultCredentialAuthResourceStaticBearer),
    EnvironmentVariable(VaultCredentialAuthResourceEnvironmentVariable),
}
impl ::std::convert::From<VaultCredentialAuthResourceMcpOauth> for VaultCredentialAuthResource {
    fn from(value: VaultCredentialAuthResourceMcpOauth) -> Self {
        Self::McpOauth(value)
    }
}
impl ::std::convert::From<VaultCredentialAuthResourceStaticBearer> for VaultCredentialAuthResource {
    fn from(value: VaultCredentialAuthResourceStaticBearer) -> Self {
        Self::StaticBearer(value)
    }
}
impl ::std::convert::From<VaultCredentialAuthResourceEnvironmentVariable>
    for VaultCredentialAuthResource
{
    fn from(value: VaultCredentialAuthResourceEnvironmentVariable) -> Self {
        Self::EnvironmentVariable(value)
    }
}
#[doc = "Metadata for an HTTP credential used only in OpenAI-hosted environments. Sandbox code receives a placeholder. The proxy substitutes the secret for allowed HTTPS destinations on ports 443 and 8443. The real secret is not available to sandbox code for local computation and is never returned in this resource."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialAuthResourceEnvironmentVariable {
    #[doc = "The destinations where the proxy can substitute the secret, subject to the environment network policy."]
    pub networking: VaultCredentialNetworkingResource,
    #[doc = "The environment variable name that receives the placeholder in the sandbox."]
    pub secret_name: ::std::string::String,
    #[doc = "The type of the object. Always `environment_variable`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialAuthResourceEnvironmentVariableType,
}
impl VaultCredentialAuthResourceEnvironmentVariable {
    pub fn builder() -> builder::VaultCredentialAuthResourceEnvironmentVariable {
        Default::default()
    }
}
#[doc = "The type of the object. Always `environment_variable`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialAuthResourceEnvironmentVariableType {
    #[serde(rename = "environment_variable")]
    EnvironmentVariable,
}
impl ::std::fmt::Display for VaultCredentialAuthResourceEnvironmentVariableType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EnvironmentVariable => f.write_str("environment_variable"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialAuthResourceEnvironmentVariableType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "environment_variable" => Ok(Self::EnvironmentVariable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialAuthResourceEnvironmentVariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialAuthResourceEnvironmentVariableType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Public metadata for an OAuth credential; tokens and client secrets are never returned."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialAuthResourceMcpOauth {
    #[doc = "When the OAuth access token expires, as an RFC 3339 timestamp, if known."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expires_at: ::std::option::Option<::std::string::String>,
    #[doc = "The HTTPS MCP server URL authorized by this credential."]
    pub mcp_server_url: ::std::string::String,
    #[doc = "Public refresh metadata without refresh tokens or OAuth client secrets."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub refresh: ::std::option::Option<McpOauthRefreshResource>,
    #[doc = "The type of the object. Always `mcp_oauth`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialAuthResourceMcpOauthType,
}
impl VaultCredentialAuthResourceMcpOauth {
    pub fn builder() -> builder::VaultCredentialAuthResourceMcpOauth {
        Default::default()
    }
}
#[doc = "The type of the object. Always `mcp_oauth`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialAuthResourceMcpOauthType {
    #[serde(rename = "mcp_oauth")]
    McpOauth,
}
impl ::std::fmt::Display for VaultCredentialAuthResourceMcpOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::McpOauth => f.write_str("mcp_oauth"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialAuthResourceMcpOauthType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mcp_oauth" => Ok(Self::McpOauth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialAuthResourceMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultCredentialAuthResourceMcpOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Metadata for a bearer-token credential, without automatic OAuth refresh."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialAuthResourceStaticBearer {
    #[doc = "The HTTPS MCP server URL authorized by this credential."]
    pub mcp_server_url: ::std::string::String,
    #[doc = "The type of the object. Always `static_bearer`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialAuthResourceStaticBearerType,
}
impl VaultCredentialAuthResourceStaticBearer {
    pub fn builder() -> builder::VaultCredentialAuthResourceStaticBearer {
        Default::default()
    }
}
#[doc = "The type of the object. Always `static_bearer`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialAuthResourceStaticBearerType {
    #[serde(rename = "static_bearer")]
    StaticBearer,
}
impl ::std::fmt::Display for VaultCredentialAuthResourceStaticBearerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::StaticBearer => f.write_str("static_bearer"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialAuthResourceStaticBearerType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "static_bearer" => Ok(Self::StaticBearer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialAuthResourceStaticBearerType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialAuthResourceStaticBearerType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A page of Agents API resources, with IDs for retrieving additional pages."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialListResource {
    #[doc = "The resources returned in this page, in the requested sort order."]
    pub data: ::std::vec::Vec<VaultCredentialResource>,
    #[doc = "The ID of the first resource in `data`, or `null` if the page is empty."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more resources to retrieve after this page."]
    pub has_more: bool,
    #[doc = "The ID of the last resource in `data`, or `null` if the page is empty. Pass this as `after` with the same order and filters."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `list`."]
    pub object: VaultCredentialListResourceObject,
}
impl VaultCredentialListResource {
    pub fn builder() -> builder::VaultCredentialListResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `list`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for VaultCredentialListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultCredentialListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Destination permissions for an environment-variable credential. These do not grant network access to the environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VaultCredentialNetworkingParam {
    Unrestricted(VaultCredentialNetworkingParamUnrestricted),
    Limited(VaultCredentialNetworkingParamLimited),
}
impl ::std::convert::From<VaultCredentialNetworkingParamUnrestricted>
    for VaultCredentialNetworkingParam
{
    fn from(value: VaultCredentialNetworkingParamUnrestricted) -> Self {
        Self::Unrestricted(value)
    }
}
impl ::std::convert::From<VaultCredentialNetworkingParamLimited>
    for VaultCredentialNetworkingParam
{
    fn from(value: VaultCredentialNetworkingParamLimited) -> Self {
        Self::Limited(value)
    }
}
#[doc = "Allows substitution only for the listed hosts. The environment network policy must also allow these hosts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialNetworkingParamLimited {
    #[doc = "The 1 to 16 distinct allowed hostnames or IPv4 addresses, normalized to lowercase. Entries contain no scheme, path, port, or wildcard. IPv6 addresses are not supported."]
    pub allowed_hosts: ::std::vec::Vec<VaultCredentialNetworkingParamLimitedAllowedHostsItem>,
    #[doc = "The type of the object. Always `limited`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialNetworkingParamLimitedType,
}
impl VaultCredentialNetworkingParamLimited {
    pub fn builder() -> builder::VaultCredentialNetworkingParamLimited {
        Default::default()
    }
}
#[doc = "`VaultCredentialNetworkingParamLimitedAllowedHostsItem`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VaultCredentialNetworkingParamLimitedAllowedHostsItem(::std::string::String);
impl ::std::ops::Deref for VaultCredentialNetworkingParamLimitedAllowedHostsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VaultCredentialNetworkingParamLimitedAllowedHostsItem>
    for ::std::string::String
{
    fn from(value: VaultCredentialNetworkingParamLimitedAllowedHostsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VaultCredentialNetworkingParamLimitedAllowedHostsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialNetworkingParamLimitedAllowedHostsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialNetworkingParamLimitedAllowedHostsItem
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VaultCredentialNetworkingParamLimitedAllowedHostsItem {
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
#[doc = "The type of the object. Always `limited`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialNetworkingParamLimitedType {
    #[serde(rename = "limited")]
    Limited,
}
impl ::std::fmt::Display for VaultCredentialNetworkingParamLimitedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Limited => f.write_str("limited"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialNetworkingParamLimitedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "limited" => Ok(Self::Limited),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialNetworkingParamLimitedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultCredentialNetworkingParamLimitedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Allows substitution for destinations permitted by the environment network policy. Requires `environment.network.access` to be `restricted`, with explicit `allowed_domains`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialNetworkingParamUnrestricted {
    #[doc = "The type of the object. Always `unrestricted`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialNetworkingParamUnrestrictedType,
}
impl VaultCredentialNetworkingParamUnrestricted {
    pub fn builder() -> builder::VaultCredentialNetworkingParamUnrestricted {
        Default::default()
    }
}
#[doc = "The type of the object. Always `unrestricted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialNetworkingParamUnrestrictedType {
    #[serde(rename = "unrestricted")]
    Unrestricted,
}
impl ::std::fmt::Display for VaultCredentialNetworkingParamUnrestrictedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unrestricted => f.write_str("unrestricted"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialNetworkingParamUnrestrictedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unrestricted" => Ok(Self::Unrestricted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialNetworkingParamUnrestrictedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialNetworkingParamUnrestrictedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Destination permissions for an environment-variable credential. These do not grant network access to the environment."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VaultCredentialNetworkingResource {
    Unrestricted(VaultCredentialNetworkingResourceUnrestricted),
    Limited(VaultCredentialNetworkingResourceLimited),
}
impl ::std::convert::From<VaultCredentialNetworkingResourceUnrestricted>
    for VaultCredentialNetworkingResource
{
    fn from(value: VaultCredentialNetworkingResourceUnrestricted) -> Self {
        Self::Unrestricted(value)
    }
}
impl ::std::convert::From<VaultCredentialNetworkingResourceLimited>
    for VaultCredentialNetworkingResource
{
    fn from(value: VaultCredentialNetworkingResourceLimited) -> Self {
        Self::Limited(value)
    }
}
#[doc = "Allows substitution only for the listed hosts. The environment network policy must also allow these hosts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialNetworkingResourceLimited {
    #[doc = "The 1 to 16 distinct allowed hostnames or IPv4 addresses, normalized to lowercase. Entries contain no scheme, path, port, or wildcard. IPv6 addresses are not supported."]
    pub allowed_hosts: ::std::vec::Vec<::std::string::String>,
    #[doc = "The type of the object. Always `limited`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialNetworkingResourceLimitedType,
}
impl VaultCredentialNetworkingResourceLimited {
    pub fn builder() -> builder::VaultCredentialNetworkingResourceLimited {
        Default::default()
    }
}
#[doc = "The type of the object. Always `limited`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialNetworkingResourceLimitedType {
    #[serde(rename = "limited")]
    Limited,
}
impl ::std::fmt::Display for VaultCredentialNetworkingResourceLimitedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Limited => f.write_str("limited"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialNetworkingResourceLimitedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "limited" => Ok(Self::Limited),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialNetworkingResourceLimitedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialNetworkingResourceLimitedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Allows substitution for destinations permitted by the environment network policy. Requires `environment.network.access` to be `restricted`, with explicit `allowed_domains`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialNetworkingResourceUnrestricted {
    #[doc = "The type of the object. Always `unrestricted`."]
    #[serde(rename = "type")]
    pub type_: VaultCredentialNetworkingResourceUnrestrictedType,
}
impl VaultCredentialNetworkingResourceUnrestricted {
    pub fn builder() -> builder::VaultCredentialNetworkingResourceUnrestricted {
        Default::default()
    }
}
#[doc = "The type of the object. Always `unrestricted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialNetworkingResourceUnrestrictedType {
    #[serde(rename = "unrestricted")]
    Unrestricted,
}
impl ::std::fmt::Display for VaultCredentialNetworkingResourceUnrestrictedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unrestricted => f.write_str("unrestricted"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialNetworkingResourceUnrestrictedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unrestricted" => Ok(Self::Unrestricted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialNetworkingResourceUnrestrictedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for VaultCredentialNetworkingResourceUnrestrictedType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Metadata for a stored credential. Secret values are never returned."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultCredentialResource {
    #[doc = "The authentication method and non-secret configuration of the credential."]
    pub auth: VaultCredentialAuthResource,
    #[doc = "The Unix timestamp, in seconds, when the credential was created."]
    pub created_at: i64,
    #[doc = "The ID of the credential."]
    pub id: ::std::string::String,
    #[doc = "Application-defined key-value pairs associated with this credential."]
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The human-readable name of the credential."]
    pub name: ::std::string::String,
    #[doc = "The object type. Always `vault.credential`."]
    pub object: VaultCredentialResourceObject,
    #[doc = "The Unix timestamp, in seconds, when the credential was last updated."]
    pub updated_at: i64,
    #[doc = "The ID of the vault containing this credential."]
    pub vault_id: ::std::string::String,
}
impl VaultCredentialResource {
    pub fn builder() -> builder::VaultCredentialResource {
        Default::default()
    }
}
#[doc = "The object type. Always `vault.credential`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultCredentialResourceObject {
    #[serde(rename = "vault.credential")]
    VaultCredential,
}
impl ::std::fmt::Display for VaultCredentialResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VaultCredential => f.write_str("vault.credential"),
        }
    }
}
impl ::std::str::FromStr for VaultCredentialResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vault.credential" => Ok(Self::VaultCredential),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultCredentialResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultCredentialResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A page of Agents API resources, with IDs for retrieving additional pages."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultListResource {
    #[doc = "The resources returned in this page, in the requested sort order."]
    pub data: ::std::vec::Vec<VaultResource>,
    #[doc = "The ID of the first resource in `data`, or `null` if the page is empty."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more resources to retrieve after this page."]
    pub has_more: bool,
    #[doc = "The ID of the last resource in `data`, or `null` if the page is empty. Pass this as `after` with the same order and filters."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `list`."]
    pub object: VaultListResourceObject,
}
impl VaultListResource {
    pub fn builder() -> builder::VaultListResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `list`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for VaultListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for VaultListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A collection of credentials for MCP servers and OpenAI-hosted environments."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VaultResource {
    #[doc = "The Unix timestamp, in seconds, when the vault was created."]
    pub created_at: i64,
    #[doc = "The ID of the vault."]
    pub id: ::std::string::String,
    #[doc = "Key-value pairs associated with the vault, such as an application or team identifier."]
    pub metadata: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The human-readable name of the vault, if set."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The object type. Always `vault`."]
    pub object: VaultResourceObject,
}
impl VaultResource {
    pub fn builder() -> builder::VaultResource {
        Default::default()
    }
}
#[doc = "The object type. Always `vault`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultResourceObject {
    #[serde(rename = "vault")]
    Vault,
}
impl ::std::fmt::Display for VaultResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Vault => f.write_str("vault"),
        }
    }
}
impl ::std::str::FromStr for VaultResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vault" => Ok(Self::Vault),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "One or more lifecycle statuses to include when listing vaults or credentials."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum VaultStatusFilterParam {
    VaultStatusParam(VaultStatusParam),
    Array(::std::vec::Vec<VaultStatusParam>),
}
impl ::std::convert::From<VaultStatusParam> for VaultStatusFilterParam {
    fn from(value: VaultStatusParam) -> Self {
        Self::VaultStatusParam(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<VaultStatusParam>> for VaultStatusFilterParam {
    fn from(value: ::std::vec::Vec<VaultStatusParam>) -> Self {
        Self::Array(value)
    }
}
#[doc = "Whether a vault or credential is active or archived."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VaultStatusParam {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
}
impl ::std::fmt::Display for VaultStatusParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Archived => f.write_str("archived"),
        }
    }
}
impl ::std::str::FromStr for VaultStatusParam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VaultStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VaultStatusParam {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VaultsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VaultsApiSchema {
    #[serde(rename = "CreateVaultCredentialError400")]
    pub create_vault_credential_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError401")]
    pub create_vault_credential_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError403")]
    pub create_vault_credential_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError404")]
    pub create_vault_credential_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError409")]
    pub create_vault_credential_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError500")]
    pub create_vault_credential_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialError503")]
    pub create_vault_credential_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultCredentialParams")]
    pub create_vault_credential_params: CreateVaultCredentialParams,
    #[serde(rename = "CreateVaultCredentialRequest")]
    pub create_vault_credential_request: CreateVaultCredentialParams,
    #[serde(rename = "CreateVaultCredentialResponse")]
    pub create_vault_credential_response: VaultCredentialResource,
    #[serde(rename = "CreateVaultError400")]
    pub create_vault_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError401")]
    pub create_vault_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError403")]
    pub create_vault_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError404")]
    pub create_vault_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError409")]
    pub create_vault_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError500")]
    pub create_vault_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultError503")]
    pub create_vault_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "CreateVaultRequest")]
    pub create_vault_request: CreateVaultParams,
    #[serde(rename = "CreateVaultResponse")]
    pub create_vault_response: VaultResource,
    #[serde(rename = "DeleteVaultCredentialError400")]
    pub delete_vault_credential_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError401")]
    pub delete_vault_credential_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError403")]
    pub delete_vault_credential_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError404")]
    pub delete_vault_credential_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError409")]
    pub delete_vault_credential_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError500")]
    pub delete_vault_credential_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialError503")]
    pub delete_vault_credential_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultCredentialParams")]
    pub delete_vault_credential_params: DeleteVaultCredentialParams,
    #[serde(rename = "DeleteVaultCredentialResponse")]
    pub delete_vault_credential_response: DeletedVaultCredentialResource,
    #[serde(rename = "DeleteVaultError400")]
    pub delete_vault_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError401")]
    pub delete_vault_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError403")]
    pub delete_vault_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError404")]
    pub delete_vault_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError409")]
    pub delete_vault_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError500")]
    pub delete_vault_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultError503")]
    pub delete_vault_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "DeleteVaultParams")]
    pub delete_vault_params: DeleteVaultParams,
    #[serde(rename = "DeleteVaultResponse")]
    pub delete_vault_response: DeletedVaultResource,
    #[serde(rename = "ListVaultCredentialsError400")]
    pub list_vault_credentials_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError401")]
    pub list_vault_credentials_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError403")]
    pub list_vault_credentials_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError404")]
    pub list_vault_credentials_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError409")]
    pub list_vault_credentials_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError500")]
    pub list_vault_credentials_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsError503")]
    pub list_vault_credentials_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultCredentialsParams")]
    pub list_vault_credentials_params: ListVaultCredentialsParams,
    #[doc = "A page of credentials in a vault."]
    #[serde(rename = "ListVaultCredentialsResponse")]
    pub list_vault_credentials_response: VaultCredentialListResource,
    #[serde(rename = "ListVaultsError400")]
    pub list_vaults_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError401")]
    pub list_vaults_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError403")]
    pub list_vaults_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError404")]
    pub list_vaults_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError409")]
    pub list_vaults_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError500")]
    pub list_vaults_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsError503")]
    pub list_vaults_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "ListVaultsParams")]
    pub list_vaults_params: ListVaultsParams,
    #[doc = "A page of vaults."]
    #[serde(rename = "ListVaultsResponse")]
    pub list_vaults_response: VaultListResource,
    #[serde(rename = "RetrieveVaultCredentialError400")]
    pub retrieve_vault_credential_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError401")]
    pub retrieve_vault_credential_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError403")]
    pub retrieve_vault_credential_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError404")]
    pub retrieve_vault_credential_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError409")]
    pub retrieve_vault_credential_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError500")]
    pub retrieve_vault_credential_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialError503")]
    pub retrieve_vault_credential_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultCredentialParams")]
    pub retrieve_vault_credential_params: RetrieveVaultCredentialParams,
    #[serde(rename = "RetrieveVaultCredentialResponse")]
    pub retrieve_vault_credential_response: VaultCredentialResource,
    #[serde(rename = "RetrieveVaultError400")]
    pub retrieve_vault_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError401")]
    pub retrieve_vault_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError403")]
    pub retrieve_vault_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError404")]
    pub retrieve_vault_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError409")]
    pub retrieve_vault_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError500")]
    pub retrieve_vault_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultError503")]
    pub retrieve_vault_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RetrieveVaultParams")]
    pub retrieve_vault_params: RetrieveVaultParams,
    #[serde(rename = "RetrieveVaultResponse")]
    pub retrieve_vault_response: VaultResource,
    #[serde(rename = "RotateVaultCredentialError400")]
    pub rotate_vault_credential_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError401")]
    pub rotate_vault_credential_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError403")]
    pub rotate_vault_credential_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError404")]
    pub rotate_vault_credential_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError409")]
    pub rotate_vault_credential_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError500")]
    pub rotate_vault_credential_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialError503")]
    pub rotate_vault_credential_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "RotateVaultCredentialParams")]
    pub rotate_vault_credential_params: RotateVaultCredentialParams,
    #[serde(rename = "RotateVaultCredentialRequest")]
    pub rotate_vault_credential_request: RotateVaultCredentialParams,
    #[serde(rename = "RotateVaultCredentialResponse")]
    pub rotate_vault_credential_response: VaultCredentialResource,
    #[serde(rename = "UpdateVaultError400")]
    pub update_vault_error400: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError401")]
    pub update_vault_error401: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError403")]
    pub update_vault_error403: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError404")]
    pub update_vault_error404: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError409")]
    pub update_vault_error409: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError500")]
    pub update_vault_error500: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultError503")]
    pub update_vault_error503: ::openai_api_types::common::ErrorResponse2,
    #[serde(rename = "UpdateVaultParams")]
    pub update_vault_params: UpdateVaultParams,
    #[serde(rename = "UpdateVaultRequest")]
    pub update_vault_request: UpdateVaultParams,
    #[serde(rename = "UpdateVaultResponse")]
    pub update_vault_response: VaultResource,
}
impl VaultsApiSchema {
    pub fn builder() -> builder::VaultsApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateMcpOauthRefreshParam {
        client_id:
            ::std::result::Result<super::CreateMcpOauthRefreshParamClientId, ::std::string::String>,
        refresh_token: ::std::result::Result<
            super::CreateMcpOauthRefreshParamRefreshToken,
            ::std::string::String,
        >,
        resource: ::std::result::Result<
            ::std::option::Option<super::CreateMcpOauthRefreshParamResource>,
            ::std::string::String,
        >,
        scope: ::std::result::Result<
            ::std::option::Option<super::CreateMcpOauthRefreshParamScope>,
            ::std::string::String,
        >,
        token_endpoint: ::std::result::Result<
            super::CreateMcpOauthRefreshParamTokenEndpoint,
            ::std::string::String,
        >,
        token_endpoint_auth: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParam,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMcpOauthRefreshParam {
        fn default() -> Self {
            Self {
                client_id: Err("no value supplied for client_id".to_string()),
                refresh_token: Err("no value supplied for refresh_token".to_string()),
                resource: Ok(Default::default()),
                scope: Ok(Default::default()),
                token_endpoint: Err("no value supplied for token_endpoint".to_string()),
                token_endpoint_auth: Err("no value supplied for token_endpoint_auth".to_string()),
            }
        }
    }
    impl CreateMcpOauthRefreshParam {
        pub fn client_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMcpOauthRefreshParamClientId>,
            T::Error: ::std::fmt::Display,
        {
            self.client_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_id: {e}"));
            self
        }
        pub fn refresh_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMcpOauthRefreshParamRefreshToken>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_token: {e}"));
            self
        }
        pub fn resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateMcpOauthRefreshParamResource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.resource = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource: {e}"));
            self
        }
        pub fn scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateMcpOauthRefreshParamScope>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope: {e}"));
            self
        }
        pub fn token_endpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMcpOauthRefreshParamTokenEndpoint>,
            T::Error: ::std::fmt::Display,
        {
            self.token_endpoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_endpoint: {e}"));
            self
        }
        pub fn token_endpoint_auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMcpOauthTokenEndpointAuthParam>,
            T::Error: ::std::fmt::Display,
        {
            self.token_endpoint_auth = value.try_into().map_err(|e| {
                format!("error converting supplied value for token_endpoint_auth: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMcpOauthRefreshParam> for super::CreateMcpOauthRefreshParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMcpOauthRefreshParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_id: value.client_id?,
                refresh_token: value.refresh_token?,
                resource: value.resource?,
                scope: value.scope?,
                token_endpoint: value.token_endpoint?,
                token_endpoint_auth: value.token_endpoint_auth?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMcpOauthRefreshParam> for CreateMcpOauthRefreshParam {
        fn from(value: super::CreateMcpOauthRefreshParam) -> Self {
            Self {
                client_id: Ok(value.client_id),
                refresh_token: Ok(value.refresh_token),
                resource: Ok(value.resource),
                scope: Ok(value.scope),
                token_endpoint: Ok(value.token_endpoint),
                token_endpoint_auth: Ok(value.token_endpoint_auth),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        client_secret: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        fn default() -> Self {
            Self {
                client_secret: Err("no value supplied for client_secret".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        pub fn client_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.client_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMcpOauthTokenEndpointAuthParamClientSecretBasic>
        for super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasic
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMcpOauthTokenEndpointAuthParamClientSecretBasic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_secret: value.client_secret?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasic>
        for CreateMcpOauthTokenEndpointAuthParamClientSecretBasic
    {
        fn from(value: super::CreateMcpOauthTokenEndpointAuthParamClientSecretBasic) -> Self {
            Self {
                client_secret: Ok(value.client_secret),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
        client_secret: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParamClientSecretPostType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
        fn default() -> Self {
            Self {
                client_secret: Err("no value supplied for client_secret".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateMcpOauthTokenEndpointAuthParamClientSecretPost {
        pub fn client_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.client_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateMcpOauthTokenEndpointAuthParamClientSecretPostType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMcpOauthTokenEndpointAuthParamClientSecretPost>
        for super::CreateMcpOauthTokenEndpointAuthParamClientSecretPost
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMcpOauthTokenEndpointAuthParamClientSecretPost,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_secret: value.client_secret?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMcpOauthTokenEndpointAuthParamClientSecretPost>
        for CreateMcpOauthTokenEndpointAuthParamClientSecretPost
    {
        fn from(value: super::CreateMcpOauthTokenEndpointAuthParamClientSecretPost) -> Self {
            Self {
                client_secret: Ok(value.client_secret),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateMcpOauthTokenEndpointAuthParamNone {
        type_: ::std::result::Result<
            super::CreateMcpOauthTokenEndpointAuthParamNoneType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateMcpOauthTokenEndpointAuthParamNone {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateMcpOauthTokenEndpointAuthParamNone {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateMcpOauthTokenEndpointAuthParamNoneType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateMcpOauthTokenEndpointAuthParamNone>
        for super::CreateMcpOauthTokenEndpointAuthParamNone
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateMcpOauthTokenEndpointAuthParamNone,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateMcpOauthTokenEndpointAuthParamNone>
        for CreateMcpOauthTokenEndpointAuthParamNone
    {
        fn from(value: super::CreateMcpOauthTokenEndpointAuthParamNone) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVaultCredentialAuthParamEnvironmentVariable {
        networking:
            ::std::result::Result<super::VaultCredentialNetworkingParam, ::std::string::String>,
        secret_name: ::std::result::Result<
            super::CreateVaultCredentialAuthParamEnvironmentVariableSecretName,
            ::std::string::String,
        >,
        secret_value: ::std::result::Result<
            super::CreateVaultCredentialAuthParamEnvironmentVariableSecretValue,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateVaultCredentialAuthParamEnvironmentVariableType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVaultCredentialAuthParamEnvironmentVariable {
        fn default() -> Self {
            Self {
                networking: Err("no value supplied for networking".to_string()),
                secret_name: Err("no value supplied for secret_name".to_string()),
                secret_value: Err("no value supplied for secret_value".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateVaultCredentialAuthParamEnvironmentVariable {
        pub fn networking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingParam>,
            T::Error: ::std::fmt::Display,
        {
            self.networking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for networking: {e}"));
            self
        }
        pub fn secret_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateVaultCredentialAuthParamEnvironmentVariableSecretName,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.secret_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secret_name: {e}"));
            self
        }
        pub fn secret_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateVaultCredentialAuthParamEnvironmentVariableSecretValue,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.secret_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secret_value: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateVaultCredentialAuthParamEnvironmentVariableType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVaultCredentialAuthParamEnvironmentVariable>
        for super::CreateVaultCredentialAuthParamEnvironmentVariable
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVaultCredentialAuthParamEnvironmentVariable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                networking: value.networking?,
                secret_name: value.secret_name?,
                secret_value: value.secret_value?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVaultCredentialAuthParamEnvironmentVariable>
        for CreateVaultCredentialAuthParamEnvironmentVariable
    {
        fn from(value: super::CreateVaultCredentialAuthParamEnvironmentVariable) -> Self {
            Self {
                networking: Ok(value.networking),
                secret_name: Ok(value.secret_name),
                secret_value: Ok(value.secret_value),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVaultCredentialAuthParamMcpOauth {
        access_token: ::std::result::Result<
            super::CreateVaultCredentialAuthParamMcpOauthAccessToken,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<
            ::std::option::Option<super::CreateVaultCredentialAuthParamMcpOauthExpiresAt>,
            ::std::string::String,
        >,
        mcp_server_url: ::std::result::Result<
            super::CreateVaultCredentialAuthParamMcpOauthMcpServerUrl,
            ::std::string::String,
        >,
        refresh: ::std::result::Result<
            ::std::option::Option<super::CreateMcpOauthRefreshParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateVaultCredentialAuthParamMcpOauthType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVaultCredentialAuthParamMcpOauth {
        fn default() -> Self {
            Self {
                access_token: Err("no value supplied for access_token".to_string()),
                expires_at: Ok(Default::default()),
                mcp_server_url: Err("no value supplied for mcp_server_url".to_string()),
                refresh: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateVaultCredentialAuthParamMcpOauth {
        pub fn access_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParamMcpOauthAccessToken>,
            T::Error: ::std::fmt::Display,
        {
            self.access_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access_token: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateVaultCredentialAuthParamMcpOauthExpiresAt>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn mcp_server_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParamMcpOauthMcpServerUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp_server_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp_server_url: {e}"));
            self
        }
        pub fn refresh<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateMcpOauthRefreshParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParamMcpOauthType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVaultCredentialAuthParamMcpOauth>
        for super::CreateVaultCredentialAuthParamMcpOauth
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVaultCredentialAuthParamMcpOauth,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                access_token: value.access_token?,
                expires_at: value.expires_at?,
                mcp_server_url: value.mcp_server_url?,
                refresh: value.refresh?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVaultCredentialAuthParamMcpOauth>
        for CreateVaultCredentialAuthParamMcpOauth
    {
        fn from(value: super::CreateVaultCredentialAuthParamMcpOauth) -> Self {
            Self {
                access_token: Ok(value.access_token),
                expires_at: Ok(value.expires_at),
                mcp_server_url: Ok(value.mcp_server_url),
                refresh: Ok(value.refresh),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVaultCredentialAuthParamStaticBearer {
        mcp_server_url: ::std::result::Result<
            super::CreateVaultCredentialAuthParamStaticBearerMcpServerUrl,
            ::std::string::String,
        >,
        token: ::std::result::Result<
            super::CreateVaultCredentialAuthParamStaticBearerToken,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::CreateVaultCredentialAuthParamStaticBearerType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVaultCredentialAuthParamStaticBearer {
        fn default() -> Self {
            Self {
                mcp_server_url: Err("no value supplied for mcp_server_url".to_string()),
                token: Err("no value supplied for token".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl CreateVaultCredentialAuthParamStaticBearer {
        pub fn mcp_server_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::CreateVaultCredentialAuthParamStaticBearerMcpServerUrl,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.mcp_server_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp_server_url: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParamStaticBearerToken>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParamStaticBearerType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVaultCredentialAuthParamStaticBearer>
        for super::CreateVaultCredentialAuthParamStaticBearer
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVaultCredentialAuthParamStaticBearer,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mcp_server_url: value.mcp_server_url?,
                token: value.token?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVaultCredentialAuthParamStaticBearer>
        for CreateVaultCredentialAuthParamStaticBearer
    {
        fn from(value: super::CreateVaultCredentialAuthParamStaticBearer) -> Self {
            Self {
                mcp_server_url: Ok(value.mcp_server_url),
                token: Ok(value.token),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVaultCredentialParams {
        auth: ::std::result::Result<super::CreateVaultCredentialAuthParam, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::CreateVaultCredentialParamsMetadataKey,
                super::CreateVaultCredentialParamsMetadataValue,
            >,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::CreateVaultCredentialParamsName, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVaultCredentialParams {
        fn default() -> Self {
            Self {
                auth: Err("no value supplied for auth".to_string()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl CreateVaultCredentialParams {
        pub fn auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialAuthParam>,
            T::Error: ::std::fmt::Display,
        {
            self.auth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::CreateVaultCredentialParamsMetadataKey,
                    super::CreateVaultCredentialParamsMetadataValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialParamsName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVaultCredentialParams> for super::CreateVaultCredentialParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVaultCredentialParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auth: value.auth?,
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVaultCredentialParams> for CreateVaultCredentialParams {
        fn from(value: super::CreateVaultCredentialParams) -> Self {
            Self {
                auth: Ok(value.auth),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVaultParams {
        metadata: ::std::result::Result<
            ::std::option::Option<
                ::std::collections::HashMap<
                    super::CreateVaultParamsMetadataKey,
                    super::CreateVaultParamsMetadataValue,
                >,
            >,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::CreateVaultParamsName>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVaultParams {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl CreateVaultParams {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    ::std::collections::HashMap<
                        super::CreateVaultParamsMetadataKey,
                        super::CreateVaultParamsMetadataValue,
                    >,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CreateVaultParamsName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVaultParams> for super::CreateVaultParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVaultParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVaultParams> for CreateVaultParams {
        fn from(value: super::CreateVaultParams) -> Self {
            Self {
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVaultCredentialParams {
        credential_id: ::std::result::Result<
            super::DeleteVaultCredentialParamsCredentialId,
            ::std::string::String,
        >,
        vault_id:
            ::std::result::Result<super::DeleteVaultCredentialParamsVaultId, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVaultCredentialParams {
        fn default() -> Self {
            Self {
                credential_id: Err("no value supplied for credential_id".to_string()),
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl DeleteVaultCredentialParams {
        pub fn credential_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVaultCredentialParamsCredentialId>,
            T::Error: ::std::fmt::Display,
        {
            self.credential_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credential_id: {e}"));
            self
        }
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVaultCredentialParamsVaultId>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVaultCredentialParams> for super::DeleteVaultCredentialParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVaultCredentialParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credential_id: value.credential_id?,
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVaultCredentialParams> for DeleteVaultCredentialParams {
        fn from(value: super::DeleteVaultCredentialParams) -> Self {
            Self {
                credential_id: Ok(value.credential_id),
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVaultParams {
        vault_id: ::std::result::Result<super::DeleteVaultParamsVaultId, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVaultParams {
        fn default() -> Self {
            Self {
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl DeleteVaultParams {
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVaultParamsVaultId>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVaultParams> for super::DeleteVaultParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVaultParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVaultParams> for DeleteVaultParams {
        fn from(value: super::DeleteVaultParams) -> Self {
            Self {
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedVaultCredentialResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::DeletedVaultCredentialResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeletedVaultCredentialResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedVaultCredentialResource {
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
            T: ::std::convert::TryInto<super::DeletedVaultCredentialResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedVaultCredentialResource>
        for super::DeletedVaultCredentialResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedVaultCredentialResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedVaultCredentialResource>
        for DeletedVaultCredentialResource
    {
        fn from(value: super::DeletedVaultCredentialResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedVaultResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeletedVaultResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedVaultResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedVaultResource {
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
            T: ::std::convert::TryInto<super::DeletedVaultResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedVaultResource> for super::DeletedVaultResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedVaultResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedVaultResource> for DeletedVaultResource {
        fn from(value: super::DeletedVaultResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVaultCredentialsParams {
        after: ::std::result::Result<
            ::std::option::Option<super::ListVaultCredentialsParamsAfter>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::ListVaultCredentialsParamsMetadataKey,
                super::ListVaultCredentialsParamsMetadataValue,
            >,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ListOrderParam>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::VaultStatusFilterParam>,
            ::std::string::String,
        >,
        vault_id:
            ::std::result::Result<super::ListVaultCredentialsParamsVaultId, ::std::string::String>,
    }
    impl ::std::default::Default for ListVaultCredentialsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                metadata: Ok(Default::default()),
                order: Ok(Default::default()),
                status: Ok(Default::default()),
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl ListVaultCredentialsParams {
        pub fn after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListVaultCredentialsParamsAfter>,
            >,
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
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ListVaultCredentialsParamsMetadataKey,
                    super::ListVaultCredentialsParamsMetadataValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ListOrderParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VaultStatusFilterParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVaultCredentialsParamsVaultId>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVaultCredentialsParams> for super::ListVaultCredentialsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVaultCredentialsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                metadata: value.metadata?,
                order: value.order?,
                status: value.status?,
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListVaultCredentialsParams> for ListVaultCredentialsParams {
        fn from(value: super::ListVaultCredentialsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                metadata: Ok(value.metadata),
                order: Ok(value.order),
                status: Ok(value.status),
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVaultsParams {
        after: ::std::result::Result<
            ::std::option::Option<super::ListVaultsParamsAfter>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::ListVaultsParamsMetadataKey,
                super::ListVaultsParamsMetadataValue,
            >,
            ::std::string::String,
        >,
        order: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::ListOrderParam>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<super::VaultStatusFilterParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListVaultsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                metadata: Ok(Default::default()),
                order: Ok(Default::default()),
                status: Ok(Default::default()),
            }
        }
    }
    impl ListVaultsParams {
        pub fn after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ListVaultsParamsAfter>>,
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
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::ListVaultsParamsMetadataKey,
                    super::ListVaultsParamsMetadataValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::ListOrderParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VaultStatusFilterParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVaultsParams> for super::ListVaultsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVaultsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                metadata: value.metadata?,
                order: value.order?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::ListVaultsParams> for ListVaultsParams {
        fn from(value: super::ListVaultsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                metadata: Ok(value.metadata),
                order: Ok(value.order),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpOauthRefreshResource {
        client_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        resource: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scope: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        token_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        token_endpoint_auth:
            ::std::result::Result<super::McpOauthTokenEndpointAuthResource, ::std::string::String>,
    }
    impl ::std::default::Default for McpOauthRefreshResource {
        fn default() -> Self {
            Self {
                client_id: Err("no value supplied for client_id".to_string()),
                resource: Err("no value supplied for resource".to_string()),
                scope: Err("no value supplied for scope".to_string()),
                token_endpoint: Err("no value supplied for token_endpoint".to_string()),
                token_endpoint_auth: Err("no value supplied for token_endpoint_auth".to_string()),
            }
        }
    }
    impl McpOauthRefreshResource {
        pub fn client_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.client_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_id: {e}"));
            self
        }
        pub fn resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource: {e}"));
            self
        }
        pub fn scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope: {e}"));
            self
        }
        pub fn token_endpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_endpoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_endpoint: {e}"));
            self
        }
        pub fn token_endpoint_auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::McpOauthTokenEndpointAuthResource>,
            T::Error: ::std::fmt::Display,
        {
            self.token_endpoint_auth = value.try_into().map_err(|e| {
                format!("error converting supplied value for token_endpoint_auth: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<McpOauthRefreshResource> for super::McpOauthRefreshResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpOauthRefreshResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_id: value.client_id?,
                resource: value.resource?,
                scope: value.scope?,
                token_endpoint: value.token_endpoint?,
                token_endpoint_auth: value.token_endpoint_auth?,
            })
        }
    }
    impl ::std::convert::From<super::McpOauthRefreshResource> for McpOauthRefreshResource {
        fn from(value: super::McpOauthRefreshResource) -> Self {
            Self {
                client_id: Ok(value.client_id),
                resource: Ok(value.resource),
                scope: Ok(value.scope),
                token_endpoint: Ok(value.token_endpoint),
                token_endpoint_auth: Ok(value.token_endpoint_auth),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpOauthTokenEndpointAuthResourceClientSecretBasic {
        type_: ::std::result::Result<
            super::McpOauthTokenEndpointAuthResourceClientSecretBasicType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for McpOauthTokenEndpointAuthResourceClientSecretBasic {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl McpOauthTokenEndpointAuthResourceClientSecretBasic {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::McpOauthTokenEndpointAuthResourceClientSecretBasicType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpOauthTokenEndpointAuthResourceClientSecretBasic>
        for super::McpOauthTokenEndpointAuthResourceClientSecretBasic
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpOauthTokenEndpointAuthResourceClientSecretBasic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::McpOauthTokenEndpointAuthResourceClientSecretBasic>
        for McpOauthTokenEndpointAuthResourceClientSecretBasic
    {
        fn from(value: super::McpOauthTokenEndpointAuthResourceClientSecretBasic) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpOauthTokenEndpointAuthResourceClientSecretPost {
        type_: ::std::result::Result<
            super::McpOauthTokenEndpointAuthResourceClientSecretPostType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for McpOauthTokenEndpointAuthResourceClientSecretPost {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl McpOauthTokenEndpointAuthResourceClientSecretPost {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::McpOauthTokenEndpointAuthResourceClientSecretPostType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpOauthTokenEndpointAuthResourceClientSecretPost>
        for super::McpOauthTokenEndpointAuthResourceClientSecretPost
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpOauthTokenEndpointAuthResourceClientSecretPost,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::McpOauthTokenEndpointAuthResourceClientSecretPost>
        for McpOauthTokenEndpointAuthResourceClientSecretPost
    {
        fn from(value: super::McpOauthTokenEndpointAuthResourceClientSecretPost) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct McpOauthTokenEndpointAuthResourceNone {
        type_: ::std::result::Result<
            super::McpOauthTokenEndpointAuthResourceNoneType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for McpOauthTokenEndpointAuthResourceNone {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl McpOauthTokenEndpointAuthResourceNone {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::McpOauthTokenEndpointAuthResourceNoneType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<McpOauthTokenEndpointAuthResourceNone>
        for super::McpOauthTokenEndpointAuthResourceNone
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: McpOauthTokenEndpointAuthResourceNone,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::McpOauthTokenEndpointAuthResourceNone>
        for McpOauthTokenEndpointAuthResourceNone
    {
        fn from(value: super::McpOauthTokenEndpointAuthResourceNone) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveVaultCredentialParams {
        credential_id: ::std::result::Result<
            super::RetrieveVaultCredentialParamsCredentialId,
            ::std::string::String,
        >,
        vault_id: ::std::result::Result<
            super::RetrieveVaultCredentialParamsVaultId,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RetrieveVaultCredentialParams {
        fn default() -> Self {
            Self {
                credential_id: Err("no value supplied for credential_id".to_string()),
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl RetrieveVaultCredentialParams {
        pub fn credential_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVaultCredentialParamsCredentialId>,
            T::Error: ::std::fmt::Display,
        {
            self.credential_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credential_id: {e}"));
            self
        }
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVaultCredentialParamsVaultId>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveVaultCredentialParams>
        for super::RetrieveVaultCredentialParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveVaultCredentialParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credential_id: value.credential_id?,
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveVaultCredentialParams> for RetrieveVaultCredentialParams {
        fn from(value: super::RetrieveVaultCredentialParams) -> Self {
            Self {
                credential_id: Ok(value.credential_id),
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveVaultParams {
        vault_id: ::std::result::Result<super::RetrieveVaultParamsVaultId, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveVaultParams {
        fn default() -> Self {
            Self {
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl RetrieveVaultParams {
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVaultParamsVaultId>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveVaultParams> for super::RetrieveVaultParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveVaultParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveVaultParams> for RetrieveVaultParams {
        fn from(value: super::RetrieveVaultParams) -> Self {
            Self {
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateMcpOauthRefreshParam {
        refresh_token: ::std::result::Result<
            ::std::option::Option<super::RotateMcpOauthRefreshParamRefreshToken>,
            ::std::string::String,
        >,
        scope: ::std::result::Result<
            ::std::option::Option<super::RotateMcpOauthRefreshParamScope>,
            ::std::string::String,
        >,
        token_endpoint_auth: ::std::result::Result<
            ::std::option::Option<super::RotateMcpOauthTokenEndpointAuthParam>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateMcpOauthRefreshParam {
        fn default() -> Self {
            Self {
                refresh_token: Ok(Default::default()),
                scope: Ok(Default::default()),
                token_endpoint_auth: Ok(Default::default()),
            }
        }
    }
    impl RotateMcpOauthRefreshParam {
        pub fn refresh_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateMcpOauthRefreshParamRefreshToken>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_token: {e}"));
            self
        }
        pub fn scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateMcpOauthRefreshParamScope>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.scope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope: {e}"));
            self
        }
        pub fn token_endpoint_auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateMcpOauthTokenEndpointAuthParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.token_endpoint_auth = value.try_into().map_err(|e| {
                format!("error converting supplied value for token_endpoint_auth: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<RotateMcpOauthRefreshParam> for super::RotateMcpOauthRefreshParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateMcpOauthRefreshParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                refresh_token: value.refresh_token?,
                scope: value.scope?,
                token_endpoint_auth: value.token_endpoint_auth?,
            })
        }
    }
    impl ::std::convert::From<super::RotateMcpOauthRefreshParam> for RotateMcpOauthRefreshParam {
        fn from(value: super::RotateMcpOauthRefreshParam) -> Self {
            Self {
                refresh_token: Ok(value.refresh_token),
                scope: Ok(value.scope),
                token_endpoint_auth: Ok(value.token_endpoint_auth),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        client_secret: ::std::result::Result<
            ::std::option::Option<
                super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret,
            >,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        fn default() -> Self {
            Self {
                client_secret: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RotateMcpOauthTokenEndpointAuthParamClientSecretBasic {
        pub fn client_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasicClientSecret,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.client_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasicType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RotateMcpOauthTokenEndpointAuthParamClientSecretBasic>
        for super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasic
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateMcpOauthTokenEndpointAuthParamClientSecretBasic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_secret: value.client_secret?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasic>
        for RotateMcpOauthTokenEndpointAuthParamClientSecretBasic
    {
        fn from(value: super::RotateMcpOauthTokenEndpointAuthParamClientSecretBasic) -> Self {
            Self {
                client_secret: Ok(value.client_secret),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
        client_secret: ::std::result::Result<
            ::std::option::Option<
                super::RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret,
            >,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RotateMcpOauthTokenEndpointAuthParamClientSecretPostType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
        fn default() -> Self {
            Self {
                client_secret: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RotateMcpOauthTokenEndpointAuthParamClientSecretPost {
        pub fn client_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::RotateMcpOauthTokenEndpointAuthParamClientSecretPostClientSecret,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.client_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for client_secret: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RotateMcpOauthTokenEndpointAuthParamClientSecretPostType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RotateMcpOauthTokenEndpointAuthParamClientSecretPost>
        for super::RotateMcpOauthTokenEndpointAuthParamClientSecretPost
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateMcpOauthTokenEndpointAuthParamClientSecretPost,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                client_secret: value.client_secret?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RotateMcpOauthTokenEndpointAuthParamClientSecretPost>
        for RotateMcpOauthTokenEndpointAuthParamClientSecretPost
    {
        fn from(value: super::RotateMcpOauthTokenEndpointAuthParamClientSecretPost) -> Self {
            Self {
                client_secret: Ok(value.client_secret),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateVaultCredentialAuthParamEnvironmentVariable {
        secret_value: ::std::result::Result<
            super::RotateVaultCredentialAuthParamEnvironmentVariableSecretValue,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RotateVaultCredentialAuthParamEnvironmentVariableType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateVaultCredentialAuthParamEnvironmentVariable {
        fn default() -> Self {
            Self {
                secret_value: Err("no value supplied for secret_value".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RotateVaultCredentialAuthParamEnvironmentVariable {
        pub fn secret_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RotateVaultCredentialAuthParamEnvironmentVariableSecretValue,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.secret_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secret_value: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::RotateVaultCredentialAuthParamEnvironmentVariableType,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RotateVaultCredentialAuthParamEnvironmentVariable>
        for super::RotateVaultCredentialAuthParamEnvironmentVariable
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateVaultCredentialAuthParamEnvironmentVariable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                secret_value: value.secret_value?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RotateVaultCredentialAuthParamEnvironmentVariable>
        for RotateVaultCredentialAuthParamEnvironmentVariable
    {
        fn from(value: super::RotateVaultCredentialAuthParamEnvironmentVariable) -> Self {
            Self {
                secret_value: Ok(value.secret_value),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateVaultCredentialAuthParamMcpOauth {
        access_token: ::std::result::Result<
            ::std::option::Option<super::RotateVaultCredentialAuthParamMcpOauthAccessToken>,
            ::std::string::String,
        >,
        expires_at: ::std::result::Result<
            ::std::option::Option<super::RotateVaultCredentialAuthParamMcpOauthExpiresAt>,
            ::std::string::String,
        >,
        refresh: ::std::result::Result<
            ::std::option::Option<super::RotateMcpOauthRefreshParam>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RotateVaultCredentialAuthParamMcpOauthType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateVaultCredentialAuthParamMcpOauth {
        fn default() -> Self {
            Self {
                access_token: Ok(Default::default()),
                expires_at: Ok(Default::default()),
                refresh: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RotateVaultCredentialAuthParamMcpOauth {
        pub fn access_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateVaultCredentialAuthParamMcpOauthAccessToken>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.access_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access_token: {e}"));
            self
        }
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateVaultCredentialAuthParamMcpOauthExpiresAt>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn refresh<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RotateMcpOauthRefreshParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RotateVaultCredentialAuthParamMcpOauthType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RotateVaultCredentialAuthParamMcpOauth>
        for super::RotateVaultCredentialAuthParamMcpOauth
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateVaultCredentialAuthParamMcpOauth,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                access_token: value.access_token?,
                expires_at: value.expires_at?,
                refresh: value.refresh?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RotateVaultCredentialAuthParamMcpOauth>
        for RotateVaultCredentialAuthParamMcpOauth
    {
        fn from(value: super::RotateVaultCredentialAuthParamMcpOauth) -> Self {
            Self {
                access_token: Ok(value.access_token),
                expires_at: Ok(value.expires_at),
                refresh: Ok(value.refresh),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateVaultCredentialAuthParamStaticBearer {
        token: ::std::result::Result<
            super::RotateVaultCredentialAuthParamStaticBearerToken,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::RotateVaultCredentialAuthParamStaticBearerType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateVaultCredentialAuthParamStaticBearer {
        fn default() -> Self {
            Self {
                token: Err("no value supplied for token".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RotateVaultCredentialAuthParamStaticBearer {
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RotateVaultCredentialAuthParamStaticBearerToken>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RotateVaultCredentialAuthParamStaticBearerType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RotateVaultCredentialAuthParamStaticBearer>
        for super::RotateVaultCredentialAuthParamStaticBearer
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateVaultCredentialAuthParamStaticBearer,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                token: value.token?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::RotateVaultCredentialAuthParamStaticBearer>
        for RotateVaultCredentialAuthParamStaticBearer
    {
        fn from(value: super::RotateVaultCredentialAuthParamStaticBearer) -> Self {
            Self {
                token: Ok(value.token),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RotateVaultCredentialParams {
        auth: ::std::result::Result<
            ::std::option::Option<super::RotateVaultCredentialAuthParam>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::RotateVaultCredentialParamsMetadataKey,
                super::RotateVaultCredentialParamsMetadataValue,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RotateVaultCredentialParams {
        fn default() -> Self {
            Self {
                auth: Ok(Default::default()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl RotateVaultCredentialParams {
        pub fn auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RotateVaultCredentialAuthParam>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.auth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::RotateVaultCredentialParamsMetadataKey,
                    super::RotateVaultCredentialParamsMetadataValue,
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
    impl ::std::convert::TryFrom<RotateVaultCredentialParams> for super::RotateVaultCredentialParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RotateVaultCredentialParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auth: value.auth?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::RotateVaultCredentialParams> for RotateVaultCredentialParams {
        fn from(value: super::RotateVaultCredentialParams) -> Self {
            Self {
                auth: Ok(value.auth),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateVaultParams {
        metadata: ::std::result::Result<
            ::std::collections::HashMap<
                super::UpdateVaultParamsMetadataKey,
                super::UpdateVaultParamsMetadataValue,
            >,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::UpdateVaultParamsName>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UpdateVaultParams {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl UpdateVaultParams {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::UpdateVaultParamsMetadataKey,
                    super::UpdateVaultParamsMetadataValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UpdateVaultParamsName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateVaultParams> for super::UpdateVaultParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateVaultParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::UpdateVaultParams> for UpdateVaultParams {
        fn from(value: super::UpdateVaultParams) -> Self {
            Self {
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialAuthResourceEnvironmentVariable {
        networking:
            ::std::result::Result<super::VaultCredentialNetworkingResource, ::std::string::String>,
        secret_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::VaultCredentialAuthResourceEnvironmentVariableType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialAuthResourceEnvironmentVariable {
        fn default() -> Self {
            Self {
                networking: Err("no value supplied for networking".to_string()),
                secret_name: Err("no value supplied for secret_name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialAuthResourceEnvironmentVariable {
        pub fn networking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingResource>,
            T::Error: ::std::fmt::Display,
        {
            self.networking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for networking: {e}"));
            self
        }
        pub fn secret_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.secret_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for secret_name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialAuthResourceEnvironmentVariableType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialAuthResourceEnvironmentVariable>
        for super::VaultCredentialAuthResourceEnvironmentVariable
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialAuthResourceEnvironmentVariable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                networking: value.networking?,
                secret_name: value.secret_name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialAuthResourceEnvironmentVariable>
        for VaultCredentialAuthResourceEnvironmentVariable
    {
        fn from(value: super::VaultCredentialAuthResourceEnvironmentVariable) -> Self {
            Self {
                networking: Ok(value.networking),
                secret_name: Ok(value.secret_name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialAuthResourceMcpOauth {
        expires_at: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        mcp_server_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        refresh: ::std::result::Result<
            ::std::option::Option<super::McpOauthRefreshResource>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::VaultCredentialAuthResourceMcpOauthType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialAuthResourceMcpOauth {
        fn default() -> Self {
            Self {
                expires_at: Err("no value supplied for expires_at".to_string()),
                mcp_server_url: Err("no value supplied for mcp_server_url".to_string()),
                refresh: Err("no value supplied for refresh".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialAuthResourceMcpOauth {
        pub fn expires_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.expires_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expires_at: {e}"));
            self
        }
        pub fn mcp_server_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp_server_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp_server_url: {e}"));
            self
        }
        pub fn refresh<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::McpOauthRefreshResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialAuthResourceMcpOauthType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialAuthResourceMcpOauth>
        for super::VaultCredentialAuthResourceMcpOauth
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialAuthResourceMcpOauth,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expires_at: value.expires_at?,
                mcp_server_url: value.mcp_server_url?,
                refresh: value.refresh?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialAuthResourceMcpOauth>
        for VaultCredentialAuthResourceMcpOauth
    {
        fn from(value: super::VaultCredentialAuthResourceMcpOauth) -> Self {
            Self {
                expires_at: Ok(value.expires_at),
                mcp_server_url: Ok(value.mcp_server_url),
                refresh: Ok(value.refresh),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialAuthResourceStaticBearer {
        mcp_server_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            super::VaultCredentialAuthResourceStaticBearerType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialAuthResourceStaticBearer {
        fn default() -> Self {
            Self {
                mcp_server_url: Err("no value supplied for mcp_server_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialAuthResourceStaticBearer {
        pub fn mcp_server_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mcp_server_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mcp_server_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialAuthResourceStaticBearerType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialAuthResourceStaticBearer>
        for super::VaultCredentialAuthResourceStaticBearer
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialAuthResourceStaticBearer,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mcp_server_url: value.mcp_server_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialAuthResourceStaticBearer>
        for VaultCredentialAuthResourceStaticBearer
    {
        fn from(value: super::VaultCredentialAuthResourceStaticBearer) -> Self {
            Self {
                mcp_server_url: Ok(value.mcp_server_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::VaultCredentialResource>,
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
            ::std::result::Result<super::VaultCredentialListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VaultCredentialListResource {
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
    impl VaultCredentialListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VaultCredentialResource>>,
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
            T: ::std::convert::TryInto<super::VaultCredentialListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialListResource> for super::VaultCredentialListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialListResource,
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
    impl ::std::convert::From<super::VaultCredentialListResource> for VaultCredentialListResource {
        fn from(value: super::VaultCredentialListResource) -> Self {
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
    pub struct VaultCredentialNetworkingParamLimited {
        allowed_hosts: ::std::result::Result<
            ::std::vec::Vec<super::VaultCredentialNetworkingParamLimitedAllowedHostsItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::VaultCredentialNetworkingParamLimitedType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialNetworkingParamLimited {
        fn default() -> Self {
            Self {
                allowed_hosts: Err("no value supplied for allowed_hosts".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialNetworkingParamLimited {
        pub fn allowed_hosts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::VaultCredentialNetworkingParamLimitedAllowedHostsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_hosts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_hosts: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingParamLimitedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialNetworkingParamLimited>
        for super::VaultCredentialNetworkingParamLimited
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialNetworkingParamLimited,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_hosts: value.allowed_hosts?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialNetworkingParamLimited>
        for VaultCredentialNetworkingParamLimited
    {
        fn from(value: super::VaultCredentialNetworkingParamLimited) -> Self {
            Self {
                allowed_hosts: Ok(value.allowed_hosts),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialNetworkingParamUnrestricted {
        type_: ::std::result::Result<
            super::VaultCredentialNetworkingParamUnrestrictedType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialNetworkingParamUnrestricted {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialNetworkingParamUnrestricted {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingParamUnrestrictedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialNetworkingParamUnrestricted>
        for super::VaultCredentialNetworkingParamUnrestricted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialNetworkingParamUnrestricted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialNetworkingParamUnrestricted>
        for VaultCredentialNetworkingParamUnrestricted
    {
        fn from(value: super::VaultCredentialNetworkingParamUnrestricted) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialNetworkingResourceLimited {
        allowed_hosts:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        type_: ::std::result::Result<
            super::VaultCredentialNetworkingResourceLimitedType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialNetworkingResourceLimited {
        fn default() -> Self {
            Self {
                allowed_hosts: Err("no value supplied for allowed_hosts".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialNetworkingResourceLimited {
        pub fn allowed_hosts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_hosts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_hosts: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingResourceLimitedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialNetworkingResourceLimited>
        for super::VaultCredentialNetworkingResourceLimited
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialNetworkingResourceLimited,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_hosts: value.allowed_hosts?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialNetworkingResourceLimited>
        for VaultCredentialNetworkingResourceLimited
    {
        fn from(value: super::VaultCredentialNetworkingResourceLimited) -> Self {
            Self {
                allowed_hosts: Ok(value.allowed_hosts),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialNetworkingResourceUnrestricted {
        type_: ::std::result::Result<
            super::VaultCredentialNetworkingResourceUnrestrictedType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VaultCredentialNetworkingResourceUnrestricted {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl VaultCredentialNetworkingResourceUnrestricted {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialNetworkingResourceUnrestrictedType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialNetworkingResourceUnrestricted>
        for super::VaultCredentialNetworkingResourceUnrestricted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialNetworkingResourceUnrestricted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialNetworkingResourceUnrestricted>
        for VaultCredentialNetworkingResourceUnrestricted
    {
        fn from(value: super::VaultCredentialNetworkingResourceUnrestricted) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultCredentialResource {
        auth: ::std::result::Result<super::VaultCredentialAuthResource, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::VaultCredentialResourceObject, ::std::string::String>,
        updated_at: ::std::result::Result<i64, ::std::string::String>,
        vault_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VaultCredentialResource {
        fn default() -> Self {
            Self {
                auth: Err("no value supplied for auth".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                updated_at: Err("no value supplied for updated_at".to_string()),
                vault_id: Err("no value supplied for vault_id".to_string()),
            }
        }
    }
    impl VaultCredentialResource {
        pub fn auth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialAuthResource>,
            T::Error: ::std::fmt::Display,
        {
            self.auth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth: {e}"));
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
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
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
            T: ::std::convert::TryInto<super::VaultCredentialResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
        pub fn vault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.vault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vault_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultCredentialResource> for super::VaultCredentialResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultCredentialResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auth: value.auth?,
                created_at: value.created_at?,
                id: value.id?,
                metadata: value.metadata?,
                name: value.name?,
                object: value.object?,
                updated_at: value.updated_at?,
                vault_id: value.vault_id?,
            })
        }
    }
    impl ::std::convert::From<super::VaultCredentialResource> for VaultCredentialResource {
        fn from(value: super::VaultCredentialResource) -> Self {
            Self {
                auth: Ok(value.auth),
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                object: Ok(value.object),
                updated_at: Ok(value.updated_at),
                vault_id: Ok(value.vault_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultListResource {
        data: ::std::result::Result<::std::vec::Vec<super::VaultResource>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::VaultListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VaultListResource {
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
    impl VaultListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VaultResource>>,
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
            T: ::std::convert::TryInto<super::VaultListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultListResource> for super::VaultListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultListResource,
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
    impl ::std::convert::From<super::VaultListResource> for VaultListResource {
        fn from(value: super::VaultListResource) -> Self {
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
    pub struct VaultResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::VaultResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VaultResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl VaultResource {
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
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VaultResource> for super::VaultResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                metadata: value.metadata?,
                name: value.name?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::VaultResource> for VaultResource {
        fn from(value: super::VaultResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VaultsApiSchema {
        create_vault_credential_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_credential_params:
            ::std::result::Result<super::CreateVaultCredentialParams, ::std::string::String>,
        create_vault_credential_request:
            ::std::result::Result<super::CreateVaultCredentialParams, ::std::string::String>,
        create_vault_credential_response:
            ::std::result::Result<super::VaultCredentialResource, ::std::string::String>,
        create_vault_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        create_vault_request:
            ::std::result::Result<super::CreateVaultParams, ::std::string::String>,
        create_vault_response: ::std::result::Result<super::VaultResource, ::std::string::String>,
        delete_vault_credential_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_credential_params:
            ::std::result::Result<super::DeleteVaultCredentialParams, ::std::string::String>,
        delete_vault_credential_response:
            ::std::result::Result<super::DeletedVaultCredentialResource, ::std::string::String>,
        delete_vault_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        delete_vault_params: ::std::result::Result<super::DeleteVaultParams, ::std::string::String>,
        delete_vault_response:
            ::std::result::Result<super::DeletedVaultResource, ::std::string::String>,
        list_vault_credentials_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vault_credentials_params:
            ::std::result::Result<super::ListVaultCredentialsParams, ::std::string::String>,
        list_vault_credentials_response:
            ::std::result::Result<super::VaultCredentialListResource, ::std::string::String>,
        list_vaults_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        list_vaults_params: ::std::result::Result<super::ListVaultsParams, ::std::string::String>,
        list_vaults_response:
            ::std::result::Result<super::VaultListResource, ::std::string::String>,
        retrieve_vault_credential_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_credential_params:
            ::std::result::Result<super::RetrieveVaultCredentialParams, ::std::string::String>,
        retrieve_vault_credential_response:
            ::std::result::Result<super::VaultCredentialResource, ::std::string::String>,
        retrieve_vault_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        retrieve_vault_params:
            ::std::result::Result<super::RetrieveVaultParams, ::std::string::String>,
        retrieve_vault_response: ::std::result::Result<super::VaultResource, ::std::string::String>,
        rotate_vault_credential_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        rotate_vault_credential_params:
            ::std::result::Result<super::RotateVaultCredentialParams, ::std::string::String>,
        rotate_vault_credential_request:
            ::std::result::Result<super::RotateVaultCredentialParams, ::std::string::String>,
        rotate_vault_credential_response:
            ::std::result::Result<super::VaultCredentialResource, ::std::string::String>,
        update_vault_error400: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error401: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error403: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error404: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error409: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error500: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_error503: ::std::result::Result<
            ::openai_api_types::common::ErrorResponse2,
            ::std::string::String,
        >,
        update_vault_params: ::std::result::Result<super::UpdateVaultParams, ::std::string::String>,
        update_vault_request:
            ::std::result::Result<super::UpdateVaultParams, ::std::string::String>,
        update_vault_response: ::std::result::Result<super::VaultResource, ::std::string::String>,
    }
    impl ::std::default::Default for VaultsApiSchema {
        fn default() -> Self {
            Self {
                create_vault_credential_error400: Err(
                    "no value supplied for create_vault_credential_error400".to_string(),
                ),
                create_vault_credential_error401: Err(
                    "no value supplied for create_vault_credential_error401".to_string(),
                ),
                create_vault_credential_error403: Err(
                    "no value supplied for create_vault_credential_error403".to_string(),
                ),
                create_vault_credential_error404: Err(
                    "no value supplied for create_vault_credential_error404".to_string(),
                ),
                create_vault_credential_error409: Err(
                    "no value supplied for create_vault_credential_error409".to_string(),
                ),
                create_vault_credential_error500: Err(
                    "no value supplied for create_vault_credential_error500".to_string(),
                ),
                create_vault_credential_error503: Err(
                    "no value supplied for create_vault_credential_error503".to_string(),
                ),
                create_vault_credential_params: Err(
                    "no value supplied for create_vault_credential_params".to_string(),
                ),
                create_vault_credential_request: Err(
                    "no value supplied for create_vault_credential_request".to_string(),
                ),
                create_vault_credential_response: Err(
                    "no value supplied for create_vault_credential_response".to_string(),
                ),
                create_vault_error400: Err(
                    "no value supplied for create_vault_error400".to_string()
                ),
                create_vault_error401: Err(
                    "no value supplied for create_vault_error401".to_string()
                ),
                create_vault_error403: Err(
                    "no value supplied for create_vault_error403".to_string()
                ),
                create_vault_error404: Err(
                    "no value supplied for create_vault_error404".to_string()
                ),
                create_vault_error409: Err(
                    "no value supplied for create_vault_error409".to_string()
                ),
                create_vault_error500: Err(
                    "no value supplied for create_vault_error500".to_string()
                ),
                create_vault_error503: Err(
                    "no value supplied for create_vault_error503".to_string()
                ),
                create_vault_request: Err("no value supplied for create_vault_request".to_string()),
                create_vault_response: Err(
                    "no value supplied for create_vault_response".to_string()
                ),
                delete_vault_credential_error400: Err(
                    "no value supplied for delete_vault_credential_error400".to_string(),
                ),
                delete_vault_credential_error401: Err(
                    "no value supplied for delete_vault_credential_error401".to_string(),
                ),
                delete_vault_credential_error403: Err(
                    "no value supplied for delete_vault_credential_error403".to_string(),
                ),
                delete_vault_credential_error404: Err(
                    "no value supplied for delete_vault_credential_error404".to_string(),
                ),
                delete_vault_credential_error409: Err(
                    "no value supplied for delete_vault_credential_error409".to_string(),
                ),
                delete_vault_credential_error500: Err(
                    "no value supplied for delete_vault_credential_error500".to_string(),
                ),
                delete_vault_credential_error503: Err(
                    "no value supplied for delete_vault_credential_error503".to_string(),
                ),
                delete_vault_credential_params: Err(
                    "no value supplied for delete_vault_credential_params".to_string(),
                ),
                delete_vault_credential_response: Err(
                    "no value supplied for delete_vault_credential_response".to_string(),
                ),
                delete_vault_error400: Err(
                    "no value supplied for delete_vault_error400".to_string()
                ),
                delete_vault_error401: Err(
                    "no value supplied for delete_vault_error401".to_string()
                ),
                delete_vault_error403: Err(
                    "no value supplied for delete_vault_error403".to_string()
                ),
                delete_vault_error404: Err(
                    "no value supplied for delete_vault_error404".to_string()
                ),
                delete_vault_error409: Err(
                    "no value supplied for delete_vault_error409".to_string()
                ),
                delete_vault_error500: Err(
                    "no value supplied for delete_vault_error500".to_string()
                ),
                delete_vault_error503: Err(
                    "no value supplied for delete_vault_error503".to_string()
                ),
                delete_vault_params: Err("no value supplied for delete_vault_params".to_string()),
                delete_vault_response: Err(
                    "no value supplied for delete_vault_response".to_string()
                ),
                list_vault_credentials_error400: Err(
                    "no value supplied for list_vault_credentials_error400".to_string(),
                ),
                list_vault_credentials_error401: Err(
                    "no value supplied for list_vault_credentials_error401".to_string(),
                ),
                list_vault_credentials_error403: Err(
                    "no value supplied for list_vault_credentials_error403".to_string(),
                ),
                list_vault_credentials_error404: Err(
                    "no value supplied for list_vault_credentials_error404".to_string(),
                ),
                list_vault_credentials_error409: Err(
                    "no value supplied for list_vault_credentials_error409".to_string(),
                ),
                list_vault_credentials_error500: Err(
                    "no value supplied for list_vault_credentials_error500".to_string(),
                ),
                list_vault_credentials_error503: Err(
                    "no value supplied for list_vault_credentials_error503".to_string(),
                ),
                list_vault_credentials_params: Err(
                    "no value supplied for list_vault_credentials_params".to_string(),
                ),
                list_vault_credentials_response: Err(
                    "no value supplied for list_vault_credentials_response".to_string(),
                ),
                list_vaults_error400: Err("no value supplied for list_vaults_error400".to_string()),
                list_vaults_error401: Err("no value supplied for list_vaults_error401".to_string()),
                list_vaults_error403: Err("no value supplied for list_vaults_error403".to_string()),
                list_vaults_error404: Err("no value supplied for list_vaults_error404".to_string()),
                list_vaults_error409: Err("no value supplied for list_vaults_error409".to_string()),
                list_vaults_error500: Err("no value supplied for list_vaults_error500".to_string()),
                list_vaults_error503: Err("no value supplied for list_vaults_error503".to_string()),
                list_vaults_params: Err("no value supplied for list_vaults_params".to_string()),
                list_vaults_response: Err("no value supplied for list_vaults_response".to_string()),
                retrieve_vault_credential_error400: Err(
                    "no value supplied for retrieve_vault_credential_error400".to_string(),
                ),
                retrieve_vault_credential_error401: Err(
                    "no value supplied for retrieve_vault_credential_error401".to_string(),
                ),
                retrieve_vault_credential_error403: Err(
                    "no value supplied for retrieve_vault_credential_error403".to_string(),
                ),
                retrieve_vault_credential_error404: Err(
                    "no value supplied for retrieve_vault_credential_error404".to_string(),
                ),
                retrieve_vault_credential_error409: Err(
                    "no value supplied for retrieve_vault_credential_error409".to_string(),
                ),
                retrieve_vault_credential_error500: Err(
                    "no value supplied for retrieve_vault_credential_error500".to_string(),
                ),
                retrieve_vault_credential_error503: Err(
                    "no value supplied for retrieve_vault_credential_error503".to_string(),
                ),
                retrieve_vault_credential_params: Err(
                    "no value supplied for retrieve_vault_credential_params".to_string(),
                ),
                retrieve_vault_credential_response: Err(
                    "no value supplied for retrieve_vault_credential_response".to_string(),
                ),
                retrieve_vault_error400: Err(
                    "no value supplied for retrieve_vault_error400".to_string()
                ),
                retrieve_vault_error401: Err(
                    "no value supplied for retrieve_vault_error401".to_string()
                ),
                retrieve_vault_error403: Err(
                    "no value supplied for retrieve_vault_error403".to_string()
                ),
                retrieve_vault_error404: Err(
                    "no value supplied for retrieve_vault_error404".to_string()
                ),
                retrieve_vault_error409: Err(
                    "no value supplied for retrieve_vault_error409".to_string()
                ),
                retrieve_vault_error500: Err(
                    "no value supplied for retrieve_vault_error500".to_string()
                ),
                retrieve_vault_error503: Err(
                    "no value supplied for retrieve_vault_error503".to_string()
                ),
                retrieve_vault_params: Err(
                    "no value supplied for retrieve_vault_params".to_string()
                ),
                retrieve_vault_response: Err(
                    "no value supplied for retrieve_vault_response".to_string()
                ),
                rotate_vault_credential_error400: Err(
                    "no value supplied for rotate_vault_credential_error400".to_string(),
                ),
                rotate_vault_credential_error401: Err(
                    "no value supplied for rotate_vault_credential_error401".to_string(),
                ),
                rotate_vault_credential_error403: Err(
                    "no value supplied for rotate_vault_credential_error403".to_string(),
                ),
                rotate_vault_credential_error404: Err(
                    "no value supplied for rotate_vault_credential_error404".to_string(),
                ),
                rotate_vault_credential_error409: Err(
                    "no value supplied for rotate_vault_credential_error409".to_string(),
                ),
                rotate_vault_credential_error500: Err(
                    "no value supplied for rotate_vault_credential_error500".to_string(),
                ),
                rotate_vault_credential_error503: Err(
                    "no value supplied for rotate_vault_credential_error503".to_string(),
                ),
                rotate_vault_credential_params: Err(
                    "no value supplied for rotate_vault_credential_params".to_string(),
                ),
                rotate_vault_credential_request: Err(
                    "no value supplied for rotate_vault_credential_request".to_string(),
                ),
                rotate_vault_credential_response: Err(
                    "no value supplied for rotate_vault_credential_response".to_string(),
                ),
                update_vault_error400: Err(
                    "no value supplied for update_vault_error400".to_string()
                ),
                update_vault_error401: Err(
                    "no value supplied for update_vault_error401".to_string()
                ),
                update_vault_error403: Err(
                    "no value supplied for update_vault_error403".to_string()
                ),
                update_vault_error404: Err(
                    "no value supplied for update_vault_error404".to_string()
                ),
                update_vault_error409: Err(
                    "no value supplied for update_vault_error409".to_string()
                ),
                update_vault_error500: Err(
                    "no value supplied for update_vault_error500".to_string()
                ),
                update_vault_error503: Err(
                    "no value supplied for update_vault_error503".to_string()
                ),
                update_vault_params: Err("no value supplied for update_vault_params".to_string()),
                update_vault_request: Err("no value supplied for update_vault_request".to_string()),
                update_vault_response: Err(
                    "no value supplied for update_vault_response".to_string()
                ),
            }
        }
    }
    impl VaultsApiSchema {
        pub fn create_vault_credential_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error400: {e}")
            });
            self
        }
        pub fn create_vault_credential_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error401: {e}")
            });
            self
        }
        pub fn create_vault_credential_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error403: {e}")
            });
            self
        }
        pub fn create_vault_credential_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error404: {e}")
            });
            self
        }
        pub fn create_vault_credential_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error409: {e}")
            });
            self
        }
        pub fn create_vault_credential_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error500: {e}")
            });
            self
        }
        pub fn create_vault_credential_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_error503: {e}")
            });
            self
        }
        pub fn create_vault_credential_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_params: {e}")
            });
            self
        }
        pub fn create_vault_credential_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_request: {e}")
            });
            self
        }
        pub fn create_vault_credential_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_credential_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_credential_response: {e}")
            });
            self
        }
        pub fn create_vault_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error400: {e}")
            });
            self
        }
        pub fn create_vault_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error401: {e}")
            });
            self
        }
        pub fn create_vault_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error403: {e}")
            });
            self
        }
        pub fn create_vault_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error404: {e}")
            });
            self
        }
        pub fn create_vault_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error409: {e}")
            });
            self
        }
        pub fn create_vault_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error500: {e}")
            });
            self
        }
        pub fn create_vault_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_error503: {e}")
            });
            self
        }
        pub fn create_vault_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVaultParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_request: {e}")
            });
            self
        }
        pub fn create_vault_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_vault_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_vault_response: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error400: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error401: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error403: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error404: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error409: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error500: {e}")
            });
            self
        }
        pub fn delete_vault_credential_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_error503: {e}")
            });
            self
        }
        pub fn delete_vault_credential_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_params: {e}")
            });
            self
        }
        pub fn delete_vault_credential_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedVaultCredentialResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_credential_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_credential_response: {e}")
            });
            self
        }
        pub fn delete_vault_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error400: {e}")
            });
            self
        }
        pub fn delete_vault_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error401: {e}")
            });
            self
        }
        pub fn delete_vault_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error403: {e}")
            });
            self
        }
        pub fn delete_vault_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error404: {e}")
            });
            self
        }
        pub fn delete_vault_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error409: {e}")
            });
            self
        }
        pub fn delete_vault_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error500: {e}")
            });
            self
        }
        pub fn delete_vault_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_error503: {e}")
            });
            self
        }
        pub fn delete_vault_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVaultParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_params: {e}")
            });
            self
        }
        pub fn delete_vault_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedVaultResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_vault_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_vault_response: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error400: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error401: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error403: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error404: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error409: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error500: {e}")
            });
            self
        }
        pub fn list_vault_credentials_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_error503: {e}")
            });
            self
        }
        pub fn list_vault_credentials_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVaultCredentialsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_params: {e}")
            });
            self
        }
        pub fn list_vault_credentials_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vault_credentials_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vault_credentials_response: {e}")
            });
            self
        }
        pub fn list_vaults_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error400: {e}")
            });
            self
        }
        pub fn list_vaults_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error401: {e}")
            });
            self
        }
        pub fn list_vaults_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error403: {e}")
            });
            self
        }
        pub fn list_vaults_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error404: {e}")
            });
            self
        }
        pub fn list_vaults_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error409: {e}")
            });
            self
        }
        pub fn list_vaults_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error500: {e}")
            });
            self
        }
        pub fn list_vaults_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_error503: {e}")
            });
            self
        }
        pub fn list_vaults_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVaultsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_params: {e}")
            });
            self
        }
        pub fn list_vaults_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_vaults_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_vaults_response: {e}")
            });
            self
        }
        pub fn retrieve_vault_credential_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error400: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error401 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error401: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error403: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error404: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error409 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error409: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error500 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error500: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_error503 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_error503: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_credential_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_credential_params: {e}")
            });
            self
        }
        pub fn retrieve_vault_credential_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialResource>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_credential_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_vault_credential_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_vault_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error400: {e}")
            });
            self
        }
        pub fn retrieve_vault_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error401: {e}")
            });
            self
        }
        pub fn retrieve_vault_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error403: {e}")
            });
            self
        }
        pub fn retrieve_vault_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error404: {e}")
            });
            self
        }
        pub fn retrieve_vault_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error409: {e}")
            });
            self
        }
        pub fn retrieve_vault_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error500: {e}")
            });
            self
        }
        pub fn retrieve_vault_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_error503: {e}")
            });
            self
        }
        pub fn retrieve_vault_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVaultParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_params: {e}")
            });
            self
        }
        pub fn retrieve_vault_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultResource>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_vault_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_vault_response: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error400: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error401: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error403: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error404: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error409: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error500: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_error503: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RotateVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_params: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RotateVaultCredentialParams>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_request: {e}")
            });
            self
        }
        pub fn rotate_vault_credential_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultCredentialResource>,
            T::Error: ::std::fmt::Display,
        {
            self.rotate_vault_credential_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for rotate_vault_credential_response: {e}")
            });
            self
        }
        pub fn update_vault_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error400: {e}")
            });
            self
        }
        pub fn update_vault_error401<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error401 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error401: {e}")
            });
            self
        }
        pub fn update_vault_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error403: {e}")
            });
            self
        }
        pub fn update_vault_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error404: {e}")
            });
            self
        }
        pub fn update_vault_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error409: {e}")
            });
            self
        }
        pub fn update_vault_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error500: {e}")
            });
            self
        }
        pub fn update_vault_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse2>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_error503: {e}")
            });
            self
        }
        pub fn update_vault_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVaultParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_params: {e}")
            });
            self
        }
        pub fn update_vault_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UpdateVaultParams>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_request: {e}")
            });
            self
        }
        pub fn update_vault_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VaultResource>,
            T::Error: ::std::fmt::Display,
        {
            self.update_vault_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_vault_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<VaultsApiSchema> for super::VaultsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VaultsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_vault_credential_error400: value.create_vault_credential_error400?,
                create_vault_credential_error401: value.create_vault_credential_error401?,
                create_vault_credential_error403: value.create_vault_credential_error403?,
                create_vault_credential_error404: value.create_vault_credential_error404?,
                create_vault_credential_error409: value.create_vault_credential_error409?,
                create_vault_credential_error500: value.create_vault_credential_error500?,
                create_vault_credential_error503: value.create_vault_credential_error503?,
                create_vault_credential_params: value.create_vault_credential_params?,
                create_vault_credential_request: value.create_vault_credential_request?,
                create_vault_credential_response: value.create_vault_credential_response?,
                create_vault_error400: value.create_vault_error400?,
                create_vault_error401: value.create_vault_error401?,
                create_vault_error403: value.create_vault_error403?,
                create_vault_error404: value.create_vault_error404?,
                create_vault_error409: value.create_vault_error409?,
                create_vault_error500: value.create_vault_error500?,
                create_vault_error503: value.create_vault_error503?,
                create_vault_request: value.create_vault_request?,
                create_vault_response: value.create_vault_response?,
                delete_vault_credential_error400: value.delete_vault_credential_error400?,
                delete_vault_credential_error401: value.delete_vault_credential_error401?,
                delete_vault_credential_error403: value.delete_vault_credential_error403?,
                delete_vault_credential_error404: value.delete_vault_credential_error404?,
                delete_vault_credential_error409: value.delete_vault_credential_error409?,
                delete_vault_credential_error500: value.delete_vault_credential_error500?,
                delete_vault_credential_error503: value.delete_vault_credential_error503?,
                delete_vault_credential_params: value.delete_vault_credential_params?,
                delete_vault_credential_response: value.delete_vault_credential_response?,
                delete_vault_error400: value.delete_vault_error400?,
                delete_vault_error401: value.delete_vault_error401?,
                delete_vault_error403: value.delete_vault_error403?,
                delete_vault_error404: value.delete_vault_error404?,
                delete_vault_error409: value.delete_vault_error409?,
                delete_vault_error500: value.delete_vault_error500?,
                delete_vault_error503: value.delete_vault_error503?,
                delete_vault_params: value.delete_vault_params?,
                delete_vault_response: value.delete_vault_response?,
                list_vault_credentials_error400: value.list_vault_credentials_error400?,
                list_vault_credentials_error401: value.list_vault_credentials_error401?,
                list_vault_credentials_error403: value.list_vault_credentials_error403?,
                list_vault_credentials_error404: value.list_vault_credentials_error404?,
                list_vault_credentials_error409: value.list_vault_credentials_error409?,
                list_vault_credentials_error500: value.list_vault_credentials_error500?,
                list_vault_credentials_error503: value.list_vault_credentials_error503?,
                list_vault_credentials_params: value.list_vault_credentials_params?,
                list_vault_credentials_response: value.list_vault_credentials_response?,
                list_vaults_error400: value.list_vaults_error400?,
                list_vaults_error401: value.list_vaults_error401?,
                list_vaults_error403: value.list_vaults_error403?,
                list_vaults_error404: value.list_vaults_error404?,
                list_vaults_error409: value.list_vaults_error409?,
                list_vaults_error500: value.list_vaults_error500?,
                list_vaults_error503: value.list_vaults_error503?,
                list_vaults_params: value.list_vaults_params?,
                list_vaults_response: value.list_vaults_response?,
                retrieve_vault_credential_error400: value.retrieve_vault_credential_error400?,
                retrieve_vault_credential_error401: value.retrieve_vault_credential_error401?,
                retrieve_vault_credential_error403: value.retrieve_vault_credential_error403?,
                retrieve_vault_credential_error404: value.retrieve_vault_credential_error404?,
                retrieve_vault_credential_error409: value.retrieve_vault_credential_error409?,
                retrieve_vault_credential_error500: value.retrieve_vault_credential_error500?,
                retrieve_vault_credential_error503: value.retrieve_vault_credential_error503?,
                retrieve_vault_credential_params: value.retrieve_vault_credential_params?,
                retrieve_vault_credential_response: value.retrieve_vault_credential_response?,
                retrieve_vault_error400: value.retrieve_vault_error400?,
                retrieve_vault_error401: value.retrieve_vault_error401?,
                retrieve_vault_error403: value.retrieve_vault_error403?,
                retrieve_vault_error404: value.retrieve_vault_error404?,
                retrieve_vault_error409: value.retrieve_vault_error409?,
                retrieve_vault_error500: value.retrieve_vault_error500?,
                retrieve_vault_error503: value.retrieve_vault_error503?,
                retrieve_vault_params: value.retrieve_vault_params?,
                retrieve_vault_response: value.retrieve_vault_response?,
                rotate_vault_credential_error400: value.rotate_vault_credential_error400?,
                rotate_vault_credential_error401: value.rotate_vault_credential_error401?,
                rotate_vault_credential_error403: value.rotate_vault_credential_error403?,
                rotate_vault_credential_error404: value.rotate_vault_credential_error404?,
                rotate_vault_credential_error409: value.rotate_vault_credential_error409?,
                rotate_vault_credential_error500: value.rotate_vault_credential_error500?,
                rotate_vault_credential_error503: value.rotate_vault_credential_error503?,
                rotate_vault_credential_params: value.rotate_vault_credential_params?,
                rotate_vault_credential_request: value.rotate_vault_credential_request?,
                rotate_vault_credential_response: value.rotate_vault_credential_response?,
                update_vault_error400: value.update_vault_error400?,
                update_vault_error401: value.update_vault_error401?,
                update_vault_error403: value.update_vault_error403?,
                update_vault_error404: value.update_vault_error404?,
                update_vault_error409: value.update_vault_error409?,
                update_vault_error500: value.update_vault_error500?,
                update_vault_error503: value.update_vault_error503?,
                update_vault_params: value.update_vault_params?,
                update_vault_request: value.update_vault_request?,
                update_vault_response: value.update_vault_response?,
            })
        }
    }
    impl ::std::convert::From<super::VaultsApiSchema> for VaultsApiSchema {
        fn from(value: super::VaultsApiSchema) -> Self {
            Self {
                create_vault_credential_error400: Ok(value.create_vault_credential_error400),
                create_vault_credential_error401: Ok(value.create_vault_credential_error401),
                create_vault_credential_error403: Ok(value.create_vault_credential_error403),
                create_vault_credential_error404: Ok(value.create_vault_credential_error404),
                create_vault_credential_error409: Ok(value.create_vault_credential_error409),
                create_vault_credential_error500: Ok(value.create_vault_credential_error500),
                create_vault_credential_error503: Ok(value.create_vault_credential_error503),
                create_vault_credential_params: Ok(value.create_vault_credential_params),
                create_vault_credential_request: Ok(value.create_vault_credential_request),
                create_vault_credential_response: Ok(value.create_vault_credential_response),
                create_vault_error400: Ok(value.create_vault_error400),
                create_vault_error401: Ok(value.create_vault_error401),
                create_vault_error403: Ok(value.create_vault_error403),
                create_vault_error404: Ok(value.create_vault_error404),
                create_vault_error409: Ok(value.create_vault_error409),
                create_vault_error500: Ok(value.create_vault_error500),
                create_vault_error503: Ok(value.create_vault_error503),
                create_vault_request: Ok(value.create_vault_request),
                create_vault_response: Ok(value.create_vault_response),
                delete_vault_credential_error400: Ok(value.delete_vault_credential_error400),
                delete_vault_credential_error401: Ok(value.delete_vault_credential_error401),
                delete_vault_credential_error403: Ok(value.delete_vault_credential_error403),
                delete_vault_credential_error404: Ok(value.delete_vault_credential_error404),
                delete_vault_credential_error409: Ok(value.delete_vault_credential_error409),
                delete_vault_credential_error500: Ok(value.delete_vault_credential_error500),
                delete_vault_credential_error503: Ok(value.delete_vault_credential_error503),
                delete_vault_credential_params: Ok(value.delete_vault_credential_params),
                delete_vault_credential_response: Ok(value.delete_vault_credential_response),
                delete_vault_error400: Ok(value.delete_vault_error400),
                delete_vault_error401: Ok(value.delete_vault_error401),
                delete_vault_error403: Ok(value.delete_vault_error403),
                delete_vault_error404: Ok(value.delete_vault_error404),
                delete_vault_error409: Ok(value.delete_vault_error409),
                delete_vault_error500: Ok(value.delete_vault_error500),
                delete_vault_error503: Ok(value.delete_vault_error503),
                delete_vault_params: Ok(value.delete_vault_params),
                delete_vault_response: Ok(value.delete_vault_response),
                list_vault_credentials_error400: Ok(value.list_vault_credentials_error400),
                list_vault_credentials_error401: Ok(value.list_vault_credentials_error401),
                list_vault_credentials_error403: Ok(value.list_vault_credentials_error403),
                list_vault_credentials_error404: Ok(value.list_vault_credentials_error404),
                list_vault_credentials_error409: Ok(value.list_vault_credentials_error409),
                list_vault_credentials_error500: Ok(value.list_vault_credentials_error500),
                list_vault_credentials_error503: Ok(value.list_vault_credentials_error503),
                list_vault_credentials_params: Ok(value.list_vault_credentials_params),
                list_vault_credentials_response: Ok(value.list_vault_credentials_response),
                list_vaults_error400: Ok(value.list_vaults_error400),
                list_vaults_error401: Ok(value.list_vaults_error401),
                list_vaults_error403: Ok(value.list_vaults_error403),
                list_vaults_error404: Ok(value.list_vaults_error404),
                list_vaults_error409: Ok(value.list_vaults_error409),
                list_vaults_error500: Ok(value.list_vaults_error500),
                list_vaults_error503: Ok(value.list_vaults_error503),
                list_vaults_params: Ok(value.list_vaults_params),
                list_vaults_response: Ok(value.list_vaults_response),
                retrieve_vault_credential_error400: Ok(value.retrieve_vault_credential_error400),
                retrieve_vault_credential_error401: Ok(value.retrieve_vault_credential_error401),
                retrieve_vault_credential_error403: Ok(value.retrieve_vault_credential_error403),
                retrieve_vault_credential_error404: Ok(value.retrieve_vault_credential_error404),
                retrieve_vault_credential_error409: Ok(value.retrieve_vault_credential_error409),
                retrieve_vault_credential_error500: Ok(value.retrieve_vault_credential_error500),
                retrieve_vault_credential_error503: Ok(value.retrieve_vault_credential_error503),
                retrieve_vault_credential_params: Ok(value.retrieve_vault_credential_params),
                retrieve_vault_credential_response: Ok(value.retrieve_vault_credential_response),
                retrieve_vault_error400: Ok(value.retrieve_vault_error400),
                retrieve_vault_error401: Ok(value.retrieve_vault_error401),
                retrieve_vault_error403: Ok(value.retrieve_vault_error403),
                retrieve_vault_error404: Ok(value.retrieve_vault_error404),
                retrieve_vault_error409: Ok(value.retrieve_vault_error409),
                retrieve_vault_error500: Ok(value.retrieve_vault_error500),
                retrieve_vault_error503: Ok(value.retrieve_vault_error503),
                retrieve_vault_params: Ok(value.retrieve_vault_params),
                retrieve_vault_response: Ok(value.retrieve_vault_response),
                rotate_vault_credential_error400: Ok(value.rotate_vault_credential_error400),
                rotate_vault_credential_error401: Ok(value.rotate_vault_credential_error401),
                rotate_vault_credential_error403: Ok(value.rotate_vault_credential_error403),
                rotate_vault_credential_error404: Ok(value.rotate_vault_credential_error404),
                rotate_vault_credential_error409: Ok(value.rotate_vault_credential_error409),
                rotate_vault_credential_error500: Ok(value.rotate_vault_credential_error500),
                rotate_vault_credential_error503: Ok(value.rotate_vault_credential_error503),
                rotate_vault_credential_params: Ok(value.rotate_vault_credential_params),
                rotate_vault_credential_request: Ok(value.rotate_vault_credential_request),
                rotate_vault_credential_response: Ok(value.rotate_vault_credential_response),
                update_vault_error400: Ok(value.update_vault_error400),
                update_vault_error401: Ok(value.update_vault_error401),
                update_vault_error403: Ok(value.update_vault_error403),
                update_vault_error404: Ok(value.update_vault_error404),
                update_vault_error409: Ok(value.update_vault_error409),
                update_vault_error500: Ok(value.update_vault_error500),
                update_vault_error503: Ok(value.update_vault_error503),
                update_vault_params: Ok(value.update_vault_params),
                update_vault_request: Ok(value.update_vault_request),
                update_vault_response: Ok(value.update_vault_response),
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
