#[doc = "`DeletedWebhookEndpointResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedWebhookEndpointResource {
    #[doc = "Whether the endpoint was deleted."]
    pub deleted: bool,
    #[doc = "The ID of the deleted webhook endpoint."]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always webhook_endpoint.deleted."]
    pub object: DeletedWebhookEndpointResourceObject,
}
impl DeletedWebhookEndpointResource {
    pub fn builder() -> builder::DeletedWebhookEndpointResource {
        Default::default()
    }
}
#[doc = "The object type, which is always webhook_endpoint.deleted."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DeletedWebhookEndpointResourceObject {
    #[serde(rename = "webhook_endpoint.deleted")]
    WebhookEndpointDeleted,
}
impl ::std::fmt::Display for DeletedWebhookEndpointResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebhookEndpointDeleted => f.write_str("webhook_endpoint.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedWebhookEndpointResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "webhook_endpoint.deleted" => Ok(Self::WebhookEndpointDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedWebhookEndpointResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedWebhookEndpointResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ListWebhookEndpointsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListWebhookEndpointsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
}
impl ListWebhookEndpointsParams {
    pub fn builder() -> builder::ListWebhookEndpointsParams {
        Default::default()
    }
}
#[doc = "`ProjectEventTypeEnum`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProjectEventTypeEnum {
    #[serde(rename = "batch.completed")]
    BatchCompleted,
    #[serde(rename = "batch.failed")]
    BatchFailed,
    #[serde(rename = "batch.expired")]
    BatchExpired,
    #[serde(rename = "batch.cancelled")]
    BatchCancelled,
    #[serde(rename = "response.completed")]
    ResponseCompleted,
    #[serde(rename = "response.failed")]
    ResponseFailed,
    #[serde(rename = "response.cancelled")]
    ResponseCancelled,
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete,
    #[serde(rename = "eval.run.succeeded")]
    EvalRunSucceeded,
    #[serde(rename = "eval.run.failed")]
    EvalRunFailed,
    #[serde(rename = "eval.run.canceled")]
    EvalRunCanceled,
    #[serde(rename = "fine_tuning.job.succeeded")]
    FineTuningJobSucceeded,
    #[serde(rename = "fine_tuning.job.failed")]
    FineTuningJobFailed,
    #[serde(rename = "fine_tuning.job.cancelled")]
    FineTuningJobCancelled,
    #[serde(rename = "realtime.call.incoming")]
    RealtimeCallIncoming,
    #[serde(rename = "video.completed")]
    VideoCompleted,
    #[serde(rename = "video.failed")]
    VideoFailed,
    #[serde(rename = "agent.environment.ready")]
    AgentEnvironmentReady,
    #[serde(rename = "agent.environment.failed")]
    AgentEnvironmentFailed,
    #[serde(rename = "agent.environment.suspended")]
    AgentEnvironmentSuspended,
    #[serde(rename = "agent.environment.expired")]
    AgentEnvironmentExpired,
    #[serde(rename = "agent.session.created")]
    AgentSessionCreated,
    #[serde(rename = "agent.session.action_required")]
    AgentSessionActionRequired,
    #[serde(rename = "agent.session.in_progress")]
    AgentSessionInProgress,
    #[serde(rename = "agent.session.idle")]
    AgentSessionIdle,
    #[serde(rename = "agent.session.failed")]
    AgentSessionFailed,
    #[serde(rename = "safety.alert.created")]
    SafetyAlertCreated,
}
impl ::std::fmt::Display for ProjectEventTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BatchCompleted => f.write_str("batch.completed"),
            Self::BatchFailed => f.write_str("batch.failed"),
            Self::BatchExpired => f.write_str("batch.expired"),
            Self::BatchCancelled => f.write_str("batch.cancelled"),
            Self::ResponseCompleted => f.write_str("response.completed"),
            Self::ResponseFailed => f.write_str("response.failed"),
            Self::ResponseCancelled => f.write_str("response.cancelled"),
            Self::ResponseIncomplete => f.write_str("response.incomplete"),
            Self::EvalRunSucceeded => f.write_str("eval.run.succeeded"),
            Self::EvalRunFailed => f.write_str("eval.run.failed"),
            Self::EvalRunCanceled => f.write_str("eval.run.canceled"),
            Self::FineTuningJobSucceeded => f.write_str("fine_tuning.job.succeeded"),
            Self::FineTuningJobFailed => f.write_str("fine_tuning.job.failed"),
            Self::FineTuningJobCancelled => f.write_str("fine_tuning.job.cancelled"),
            Self::RealtimeCallIncoming => f.write_str("realtime.call.incoming"),
            Self::VideoCompleted => f.write_str("video.completed"),
            Self::VideoFailed => f.write_str("video.failed"),
            Self::AgentEnvironmentReady => f.write_str("agent.environment.ready"),
            Self::AgentEnvironmentFailed => f.write_str("agent.environment.failed"),
            Self::AgentEnvironmentSuspended => f.write_str("agent.environment.suspended"),
            Self::AgentEnvironmentExpired => f.write_str("agent.environment.expired"),
            Self::AgentSessionCreated => f.write_str("agent.session.created"),
            Self::AgentSessionActionRequired => f.write_str("agent.session.action_required"),
            Self::AgentSessionInProgress => f.write_str("agent.session.in_progress"),
            Self::AgentSessionIdle => f.write_str("agent.session.idle"),
            Self::AgentSessionFailed => f.write_str("agent.session.failed"),
            Self::SafetyAlertCreated => f.write_str("safety.alert.created"),
        }
    }
}
impl ::std::str::FromStr for ProjectEventTypeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "batch.completed" => Ok(Self::BatchCompleted),
            "batch.failed" => Ok(Self::BatchFailed),
            "batch.expired" => Ok(Self::BatchExpired),
            "batch.cancelled" => Ok(Self::BatchCancelled),
            "response.completed" => Ok(Self::ResponseCompleted),
            "response.failed" => Ok(Self::ResponseFailed),
            "response.cancelled" => Ok(Self::ResponseCancelled),
            "response.incomplete" => Ok(Self::ResponseIncomplete),
            "eval.run.succeeded" => Ok(Self::EvalRunSucceeded),
            "eval.run.failed" => Ok(Self::EvalRunFailed),
            "eval.run.canceled" => Ok(Self::EvalRunCanceled),
            "fine_tuning.job.succeeded" => Ok(Self::FineTuningJobSucceeded),
            "fine_tuning.job.failed" => Ok(Self::FineTuningJobFailed),
            "fine_tuning.job.cancelled" => Ok(Self::FineTuningJobCancelled),
            "realtime.call.incoming" => Ok(Self::RealtimeCallIncoming),
            "video.completed" => Ok(Self::VideoCompleted),
            "video.failed" => Ok(Self::VideoFailed),
            "agent.environment.ready" => Ok(Self::AgentEnvironmentReady),
            "agent.environment.failed" => Ok(Self::AgentEnvironmentFailed),
            "agent.environment.suspended" => Ok(Self::AgentEnvironmentSuspended),
            "agent.environment.expired" => Ok(Self::AgentEnvironmentExpired),
            "agent.session.created" => Ok(Self::AgentSessionCreated),
            "agent.session.action_required" => Ok(Self::AgentSessionActionRequired),
            "agent.session.in_progress" => Ok(Self::AgentSessionInProgress),
            "agent.session.idle" => Ok(Self::AgentSessionIdle),
            "agent.session.failed" => Ok(Self::AgentSessionFailed),
            "safety.alert.created" => Ok(Self::SafetyAlertCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectEventTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectEventTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PublicCreateEndpointBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicCreateEndpointBody {
    #[doc = "The event types that trigger deliveries to this endpoint."]
    pub event_types: ::std::vec::Vec<ProjectEventTypeEnum>,
    #[doc = "A human-readable name for the webhook endpoint."]
    pub name: PublicCreateEndpointBodyName,
    #[doc = "The HTTPS URL that receives webhook deliveries."]
    pub url: PublicCreateEndpointBodyUrl,
}
impl PublicCreateEndpointBody {
    pub fn builder() -> builder::PublicCreateEndpointBody {
        Default::default()
    }
}
#[doc = "A human-readable name for the webhook endpoint."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PublicCreateEndpointBodyName(::std::string::String);
impl ::std::ops::Deref for PublicCreateEndpointBodyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicCreateEndpointBodyName> for ::std::string::String {
    fn from(value: PublicCreateEndpointBodyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicCreateEndpointBodyName {
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
impl ::std::convert::TryFrom<&str> for PublicCreateEndpointBodyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicCreateEndpointBodyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicCreateEndpointBodyName {
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
#[doc = "The HTTPS URL that receives webhook deliveries."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PublicCreateEndpointBodyUrl(::std::string::String);
impl ::std::ops::Deref for PublicCreateEndpointBodyUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicCreateEndpointBodyUrl> for ::std::string::String {
    fn from(value: PublicCreateEndpointBodyUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicCreateEndpointBodyUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2048usize {
            return Err("longer than 2048 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^https://").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^https://\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PublicCreateEndpointBodyUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicCreateEndpointBodyUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicCreateEndpointBodyUrl {
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
#[doc = "`PublicRotateSecretBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PublicRotateSecretBody {
    #[doc = "Whether to keep the previous signing secret valid for 24 hours after rotation. Defaults to false, which invalidates the previous secret immediately."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keep_old_secret_active_for_24_hours: ::std::option::Option<bool>,
}
impl PublicRotateSecretBody {
    pub fn builder() -> builder::PublicRotateSecretBody {
        Default::default()
    }
}
#[doc = "`PublicTestEndpointBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicTestEndpointBody {
    #[doc = "The event type to send as a sample delivery."]
    pub event_type: ProjectEventTypeEnum,
}
impl PublicTestEndpointBody {
    pub fn builder() -> builder::PublicTestEndpointBody {
        Default::default()
    }
}
#[doc = "`PublicUpdateEndpointBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct PublicUpdateEndpointBody {
    #[doc = "The complete set of event types that should trigger deliveries."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub event_types: ::std::vec::Vec<ProjectEventTypeEnum>,
    #[doc = "A new human-readable name for the webhook endpoint."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<PublicUpdateEndpointBodyName>,
    #[doc = "A new HTTPS URL that receives webhook deliveries."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<PublicUpdateEndpointBodyUrl>,
}
impl PublicUpdateEndpointBody {
    pub fn builder() -> builder::PublicUpdateEndpointBody {
        Default::default()
    }
}
#[doc = "A new human-readable name for the webhook endpoint."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PublicUpdateEndpointBodyName(::std::string::String);
impl ::std::ops::Deref for PublicUpdateEndpointBodyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicUpdateEndpointBodyName> for ::std::string::String {
    fn from(value: PublicUpdateEndpointBodyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicUpdateEndpointBodyName {
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
impl ::std::convert::TryFrom<&str> for PublicUpdateEndpointBodyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicUpdateEndpointBodyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicUpdateEndpointBodyName {
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
#[doc = "A new HTTPS URL that receives webhook deliveries."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PublicUpdateEndpointBodyUrl(::std::string::String);
impl ::std::ops::Deref for PublicUpdateEndpointBodyUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicUpdateEndpointBodyUrl> for ::std::string::String {
    fn from(value: PublicUpdateEndpointBodyUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicUpdateEndpointBodyUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2048usize {
            return Err("longer than 2048 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^https://").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^https://\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PublicUpdateEndpointBodyUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicUpdateEndpointBodyUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicUpdateEndpointBodyUrl {
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
#[doc = "`WebhookEndpointBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhookEndpointBody {
    #[doc = "The Unix timestamp when the endpoint was created."]
    pub created_at: i64,
    #[doc = "The event types that trigger deliveries to this endpoint."]
    pub event_types: ::std::vec::Vec<::std::string::String>,
    #[doc = "The unique ID of the webhook endpoint."]
    pub id: ::std::string::String,
    #[doc = "The human-readable name of the endpoint."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always webhook_endpoint."]
    pub object: WebhookEndpointBodyObject,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub signing_secret_hint: ::std::option::Option<::std::string::String>,
    #[doc = "The Unix timestamp of the last endpoint configuration or signing-secret change. Initialized at creation; tests and unchanged updates do not advance it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<i64>,
    #[doc = "The HTTPS URL that receives webhook deliveries."]
    pub url: ::std::string::String,
}
impl WebhookEndpointBody {
    pub fn builder() -> builder::WebhookEndpointBody {
        Default::default()
    }
}
#[doc = "The object type, which is always webhook_endpoint."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookEndpointBodyObject {
    #[serde(rename = "webhook_endpoint")]
    WebhookEndpoint,
}
impl ::std::fmt::Display for WebhookEndpointBodyObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebhookEndpoint => f.write_str("webhook_endpoint"),
        }
    }
}
impl ::std::str::FromStr for WebhookEndpointBodyObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "webhook_endpoint" => Ok(Self::WebhookEndpoint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookEndpointBodyObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookEndpointBodyObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookEndpointListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhookEndpointListResource {
    #[doc = "The webhook endpoints in this page."]
    pub data: ::std::vec::Vec<WebhookEndpointBody>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether more webhook endpoints are available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always list."]
    pub object: WebhookEndpointListResourceObject,
}
impl WebhookEndpointListResource {
    pub fn builder() -> builder::WebhookEndpointListResource {
        Default::default()
    }
}
#[doc = "The object type, which is always list."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookEndpointListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for WebhookEndpointListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for WebhookEndpointListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookEndpointListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookEndpointListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookEndpointTestResultResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhookEndpointTestResultResource {
    #[doc = "The event type sent in the test."]
    pub event_type: ::std::string::String,
    #[doc = "The object type, which is always webhook_endpoint.test."]
    pub object: WebhookEndpointTestResultResourceObject,
    #[doc = "The HTTP status code returned by the endpoint."]
    pub status_code: i64,
    #[doc = "Whether the test request completed. Always true for returned results; use status_code to determine the endpoint response."]
    pub success: bool,
    #[doc = "The ID of the webhook endpoint that received the test."]
    pub webhook_endpoint_id: ::std::string::String,
}
impl WebhookEndpointTestResultResource {
    pub fn builder() -> builder::WebhookEndpointTestResultResource {
        Default::default()
    }
}
#[doc = "The object type, which is always webhook_endpoint.test."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookEndpointTestResultResourceObject {
    #[serde(rename = "webhook_endpoint.test")]
    WebhookEndpointTest,
}
impl ::std::fmt::Display for WebhookEndpointTestResultResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebhookEndpointTest => f.write_str("webhook_endpoint.test"),
        }
    }
}
impl ::std::str::FromStr for WebhookEndpointTestResultResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "webhook_endpoint.test" => Ok(Self::WebhookEndpointTest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookEndpointTestResultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookEndpointTestResultResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookEndpointWithSecretResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhookEndpointWithSecretResource {
    #[doc = "The Unix timestamp when the endpoint was created."]
    pub created_at: i64,
    #[doc = "The event types that trigger deliveries to this endpoint."]
    pub event_types: ::std::vec::Vec<::std::string::String>,
    #[doc = "The unique ID of the webhook endpoint."]
    pub id: ::std::string::String,
    #[doc = "The human-readable name of the endpoint."]
    pub name: ::std::string::String,
    #[doc = "The object type, which is always webhook_endpoint."]
    pub object: WebhookEndpointWithSecretResourceObject,
    #[doc = "The endpoint's signing secret. This is returned only when the endpoint is created or the secret is rotated."]
    pub signing_secret: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub signing_secret_hint: ::std::option::Option<::std::string::String>,
    #[doc = "The Unix timestamp of the last endpoint configuration or signing-secret change. Initialized at creation; tests and unchanged updates do not advance it."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub updated_at: ::std::option::Option<i64>,
    #[doc = "The HTTPS URL that receives webhook deliveries."]
    pub url: ::std::string::String,
}
impl WebhookEndpointWithSecretResource {
    pub fn builder() -> builder::WebhookEndpointWithSecretResource {
        Default::default()
    }
}
#[doc = "The object type, which is always webhook_endpoint."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookEndpointWithSecretResourceObject {
    #[serde(rename = "webhook_endpoint")]
    WebhookEndpoint,
}
impl ::std::fmt::Display for WebhookEndpointWithSecretResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::WebhookEndpoint => f.write_str("webhook_endpoint"),
        }
    }
}
impl ::std::str::FromStr for WebhookEndpointWithSecretResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "webhook_endpoint" => Ok(Self::WebhookEndpoint),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookEndpointWithSecretResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookEndpointWithSecretResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhookEventTypeListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhookEventTypeListResource {
    #[doc = "The webhook event types available to the authenticated project."]
    pub data: ::std::vec::Vec<::std::string::String>,
    #[doc = "The object type, which is always list."]
    pub object: WebhookEventTypeListResourceObject,
}
impl WebhookEventTypeListResource {
    pub fn builder() -> builder::WebhookEventTypeListResource {
        Default::default()
    }
}
#[doc = "The object type, which is always list."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WebhookEventTypeListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for WebhookEventTypeListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for WebhookEventTypeListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WebhookEventTypeListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WebhookEventTypeListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WebhooksApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct WebhooksApiSchema {
    #[serde(rename = "CreateWebhookEndpointRequest")]
    pub create_webhook_endpoint_request: PublicCreateEndpointBody,
    #[serde(rename = "CreateWebhookEndpointResponse")]
    pub create_webhook_endpoint_response: WebhookEndpointWithSecretResource,
    #[serde(rename = "DeleteWebhookEndpointResponse")]
    pub delete_webhook_endpoint_response: DeletedWebhookEndpointResource,
    #[serde(rename = "ListWebhookEndpointsParams")]
    pub list_webhook_endpoints_params: ListWebhookEndpointsParams,
    #[serde(rename = "ListWebhookEndpointsResponse")]
    pub list_webhook_endpoints_response: WebhookEndpointListResource,
    #[serde(rename = "ListWebhookEventTypesResponse")]
    pub list_webhook_event_types_response: WebhookEventTypeListResource,
    #[serde(rename = "RetrieveWebhookEndpointResponse")]
    pub retrieve_webhook_endpoint_response: WebhookEndpointBody,
    #[serde(rename = "RotateWebhookEndpointSigningSecretRequest")]
    pub rotate_webhook_endpoint_signing_secret_request: PublicRotateSecretBody,
    #[serde(rename = "RotateWebhookEndpointSigningSecretResponse")]
    pub rotate_webhook_endpoint_signing_secret_response: WebhookEndpointWithSecretResource,
    #[serde(rename = "TestWebhookEndpointRequest")]
    pub test_webhook_endpoint_request: PublicTestEndpointBody,
    #[serde(rename = "TestWebhookEndpointResponse")]
    pub test_webhook_endpoint_response: WebhookEndpointTestResultResource,
    #[serde(rename = "UpdateWebhookEndpointRequest")]
    pub update_webhook_endpoint_request: PublicUpdateEndpointBody,
    #[serde(rename = "UpdateWebhookEndpointResponse")]
    pub update_webhook_endpoint_response: WebhookEndpointBody,
}
impl WebhooksApiSchema {
    pub fn builder() -> builder::WebhooksApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DeletedWebhookEndpointResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::DeletedWebhookEndpointResourceObject,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeletedWebhookEndpointResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedWebhookEndpointResource {
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
            T: ::std::convert::TryInto<super::DeletedWebhookEndpointResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedWebhookEndpointResource>
        for super::DeletedWebhookEndpointResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedWebhookEndpointResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedWebhookEndpointResource>
        for DeletedWebhookEndpointResource
    {
        fn from(value: super::DeletedWebhookEndpointResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListWebhookEndpointsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListWebhookEndpointsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListWebhookEndpointsParams {
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
    }
    impl ::std::convert::TryFrom<ListWebhookEndpointsParams> for super::ListWebhookEndpointsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListWebhookEndpointsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListWebhookEndpointsParams> for ListWebhookEndpointsParams {
        fn from(value: super::ListWebhookEndpointsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicCreateEndpointBody {
        event_types: ::std::result::Result<
            ::std::vec::Vec<super::ProjectEventTypeEnum>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::PublicCreateEndpointBodyName, ::std::string::String>,
        url: ::std::result::Result<super::PublicCreateEndpointBodyUrl, ::std::string::String>,
    }
    impl ::std::default::Default for PublicCreateEndpointBody {
        fn default() -> Self {
            Self {
                event_types: Err("no value supplied for event_types".to_string()),
                name: Err("no value supplied for name".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl PublicCreateEndpointBody {
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectEventTypeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicCreateEndpointBodyName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicCreateEndpointBodyUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicCreateEndpointBody> for super::PublicCreateEndpointBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicCreateEndpointBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_types: value.event_types?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::PublicCreateEndpointBody> for PublicCreateEndpointBody {
        fn from(value: super::PublicCreateEndpointBody) -> Self {
            Self {
                event_types: Ok(value.event_types),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicRotateSecretBody {
        keep_old_secret_active_for_24_hours:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicRotateSecretBody {
        fn default() -> Self {
            Self {
                keep_old_secret_active_for_24_hours: Ok(Default::default()),
            }
        }
    }
    impl PublicRotateSecretBody {
        pub fn keep_old_secret_active_for_24_hours<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.keep_old_secret_active_for_24_hours = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for keep_old_secret_active_for_24_hours: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PublicRotateSecretBody> for super::PublicRotateSecretBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicRotateSecretBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keep_old_secret_active_for_24_hours: value.keep_old_secret_active_for_24_hours?,
            })
        }
    }
    impl ::std::convert::From<super::PublicRotateSecretBody> for PublicRotateSecretBody {
        fn from(value: super::PublicRotateSecretBody) -> Self {
            Self {
                keep_old_secret_active_for_24_hours: Ok(value.keep_old_secret_active_for_24_hours),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicTestEndpointBody {
        event_type: ::std::result::Result<super::ProjectEventTypeEnum, ::std::string::String>,
    }
    impl ::std::default::Default for PublicTestEndpointBody {
        fn default() -> Self {
            Self {
                event_type: Err("no value supplied for event_type".to_string()),
            }
        }
    }
    impl PublicTestEndpointBody {
        pub fn event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProjectEventTypeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.event_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicTestEndpointBody> for super::PublicTestEndpointBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicTestEndpointBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
            })
        }
    }
    impl ::std::convert::From<super::PublicTestEndpointBody> for PublicTestEndpointBody {
        fn from(value: super::PublicTestEndpointBody) -> Self {
            Self {
                event_type: Ok(value.event_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicUpdateEndpointBody {
        event_types: ::std::result::Result<
            ::std::vec::Vec<super::ProjectEventTypeEnum>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<super::PublicUpdateEndpointBodyName>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<super::PublicUpdateEndpointBodyUrl>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PublicUpdateEndpointBody {
        fn default() -> Self {
            Self {
                event_types: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl PublicUpdateEndpointBody {
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectEventTypeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PublicUpdateEndpointBodyName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PublicUpdateEndpointBodyUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PublicUpdateEndpointBody> for super::PublicUpdateEndpointBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicUpdateEndpointBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_types: value.event_types?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::PublicUpdateEndpointBody> for PublicUpdateEndpointBody {
        fn from(value: super::PublicUpdateEndpointBody) -> Self {
            Self {
                event_types: Ok(value.event_types),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookEndpointBody {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        event_types:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::WebhookEndpointBodyObject, ::std::string::String>,
        signing_secret_hint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookEndpointBody {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                event_types: Err("no value supplied for event_types".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                signing_secret_hint: Err("no value supplied for signing_secret_hint".to_string()),
                updated_at: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl WebhookEndpointBody {
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
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
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
            T: ::std::convert::TryInto<super::WebhookEndpointBodyObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn signing_secret_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.signing_secret_hint = value.try_into().map_err(|e| {
                format!("error converting supplied value for signing_secret_hint: {e}")
            });
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookEndpointBody> for super::WebhookEndpointBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookEndpointBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                event_types: value.event_types?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                signing_secret_hint: value.signing_secret_hint?,
                updated_at: value.updated_at?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookEndpointBody> for WebhookEndpointBody {
        fn from(value: super::WebhookEndpointBody) -> Self {
            Self {
                created_at: Ok(value.created_at),
                event_types: Ok(value.event_types),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                signing_secret_hint: Ok(value.signing_secret_hint),
                updated_at: Ok(value.updated_at),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookEndpointListResource {
        data: ::std::result::Result<
            ::std::vec::Vec<super::WebhookEndpointBody>,
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
            ::std::result::Result<super::WebhookEndpointListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookEndpointListResource {
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
    impl WebhookEndpointListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WebhookEndpointBody>>,
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
            T: ::std::convert::TryInto<super::WebhookEndpointListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookEndpointListResource> for super::WebhookEndpointListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookEndpointListResource,
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
    impl ::std::convert::From<super::WebhookEndpointListResource> for WebhookEndpointListResource {
        fn from(value: super::WebhookEndpointListResource) -> Self {
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
    pub struct WebhookEndpointTestResultResource {
        event_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::WebhookEndpointTestResultResourceObject,
            ::std::string::String,
        >,
        status_code: ::std::result::Result<i64, ::std::string::String>,
        success: ::std::result::Result<bool, ::std::string::String>,
        webhook_endpoint_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookEndpointTestResultResource {
        fn default() -> Self {
            Self {
                event_type: Err("no value supplied for event_type".to_string()),
                object: Err("no value supplied for object".to_string()),
                status_code: Err("no value supplied for status_code".to_string()),
                success: Err("no value supplied for success".to_string()),
                webhook_endpoint_id: Err("no value supplied for webhook_endpoint_id".to_string()),
            }
        }
    }
    impl WebhookEndpointTestResultResource {
        pub fn event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_type: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointTestResultResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_code: {e}"));
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for success: {e}"));
            self
        }
        pub fn webhook_endpoint_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.webhook_endpoint_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for webhook_endpoint_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookEndpointTestResultResource>
        for super::WebhookEndpointTestResultResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookEndpointTestResultResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
                object: value.object?,
                status_code: value.status_code?,
                success: value.success?,
                webhook_endpoint_id: value.webhook_endpoint_id?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookEndpointTestResultResource>
        for WebhookEndpointTestResultResource
    {
        fn from(value: super::WebhookEndpointTestResultResource) -> Self {
            Self {
                event_type: Ok(value.event_type),
                object: Ok(value.object),
                status_code: Ok(value.status_code),
                success: Ok(value.success),
                webhook_endpoint_id: Ok(value.webhook_endpoint_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookEndpointWithSecretResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        event_types:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<
            super::WebhookEndpointWithSecretResourceObject,
            ::std::string::String,
        >,
        signing_secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        signing_secret_hint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookEndpointWithSecretResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                event_types: Err("no value supplied for event_types".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                object: Err("no value supplied for object".to_string()),
                signing_secret: Err("no value supplied for signing_secret".to_string()),
                signing_secret_hint: Err("no value supplied for signing_secret_hint".to_string()),
                updated_at: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl WebhookEndpointWithSecretResource {
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
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
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
            T: ::std::convert::TryInto<super::WebhookEndpointWithSecretResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn signing_secret<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.signing_secret = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signing_secret: {e}"));
            self
        }
        pub fn signing_secret_hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.signing_secret_hint = value.try_into().map_err(|e| {
                format!("error converting supplied value for signing_secret_hint: {e}")
            });
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookEndpointWithSecretResource>
        for super::WebhookEndpointWithSecretResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookEndpointWithSecretResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                event_types: value.event_types?,
                id: value.id?,
                name: value.name?,
                object: value.object?,
                signing_secret: value.signing_secret?,
                signing_secret_hint: value.signing_secret_hint?,
                updated_at: value.updated_at?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookEndpointWithSecretResource>
        for WebhookEndpointWithSecretResource
    {
        fn from(value: super::WebhookEndpointWithSecretResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                event_types: Ok(value.event_types),
                id: Ok(value.id),
                name: Ok(value.name),
                object: Ok(value.object),
                signing_secret: Ok(value.signing_secret),
                signing_secret_hint: Ok(value.signing_secret_hint),
                updated_at: Ok(value.updated_at),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhookEventTypeListResource {
        data: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        object:
            ::std::result::Result<super::WebhookEventTypeListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for WebhookEventTypeListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl WebhookEventTypeListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEventTypeListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WebhookEventTypeListResource> for super::WebhookEventTypeListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhookEventTypeListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::WebhookEventTypeListResource> for WebhookEventTypeListResource {
        fn from(value: super::WebhookEventTypeListResource) -> Self {
            Self {
                data: Ok(value.data),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WebhooksApiSchema {
        create_webhook_endpoint_request:
            ::std::result::Result<super::PublicCreateEndpointBody, ::std::string::String>,
        create_webhook_endpoint_response:
            ::std::result::Result<super::WebhookEndpointWithSecretResource, ::std::string::String>,
        delete_webhook_endpoint_response:
            ::std::result::Result<super::DeletedWebhookEndpointResource, ::std::string::String>,
        list_webhook_endpoints_params:
            ::std::result::Result<super::ListWebhookEndpointsParams, ::std::string::String>,
        list_webhook_endpoints_response:
            ::std::result::Result<super::WebhookEndpointListResource, ::std::string::String>,
        list_webhook_event_types_response:
            ::std::result::Result<super::WebhookEventTypeListResource, ::std::string::String>,
        retrieve_webhook_endpoint_response:
            ::std::result::Result<super::WebhookEndpointBody, ::std::string::String>,
        rotate_webhook_endpoint_signing_secret_request:
            ::std::result::Result<super::PublicRotateSecretBody, ::std::string::String>,
        rotate_webhook_endpoint_signing_secret_response:
            ::std::result::Result<super::WebhookEndpointWithSecretResource, ::std::string::String>,
        test_webhook_endpoint_request:
            ::std::result::Result<super::PublicTestEndpointBody, ::std::string::String>,
        test_webhook_endpoint_response:
            ::std::result::Result<super::WebhookEndpointTestResultResource, ::std::string::String>,
        update_webhook_endpoint_request:
            ::std::result::Result<super::PublicUpdateEndpointBody, ::std::string::String>,
        update_webhook_endpoint_response:
            ::std::result::Result<super::WebhookEndpointBody, ::std::string::String>,
    }
    impl ::std::default::Default for WebhooksApiSchema {
        fn default() -> Self {
            Self {
                create_webhook_endpoint_request: Err(
                    "no value supplied for create_webhook_endpoint_request".to_string(),
                ),
                create_webhook_endpoint_response: Err(
                    "no value supplied for create_webhook_endpoint_response".to_string(),
                ),
                delete_webhook_endpoint_response: Err(
                    "no value supplied for delete_webhook_endpoint_response".to_string(),
                ),
                list_webhook_endpoints_params: Err(
                    "no value supplied for list_webhook_endpoints_params".to_string(),
                ),
                list_webhook_endpoints_response: Err(
                    "no value supplied for list_webhook_endpoints_response".to_string(),
                ),
                list_webhook_event_types_response: Err(
                    "no value supplied for list_webhook_event_types_response".to_string(),
                ),
                retrieve_webhook_endpoint_response: Err(
                    "no value supplied for retrieve_webhook_endpoint_response".to_string(),
                ),
                rotate_webhook_endpoint_signing_secret_request: Err(
                    "no value supplied for rotate_webhook_endpoint_signing_secret_request"
                        .to_string(),
                ),
                rotate_webhook_endpoint_signing_secret_response: Err(
                    "no value supplied for rotate_webhook_endpoint_signing_secret_response"
                        .to_string(),
                ),
                test_webhook_endpoint_request: Err(
                    "no value supplied for test_webhook_endpoint_request".to_string(),
                ),
                test_webhook_endpoint_response: Err(
                    "no value supplied for test_webhook_endpoint_response".to_string(),
                ),
                update_webhook_endpoint_request: Err(
                    "no value supplied for update_webhook_endpoint_request".to_string(),
                ),
                update_webhook_endpoint_response: Err(
                    "no value supplied for update_webhook_endpoint_response".to_string(),
                ),
            }
        }
    }
    impl WebhooksApiSchema {
        pub fn create_webhook_endpoint_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicCreateEndpointBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_webhook_endpoint_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_webhook_endpoint_request: {e}")
            });
            self
        }
        pub fn create_webhook_endpoint_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointWithSecretResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_webhook_endpoint_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_webhook_endpoint_response: {e}")
            });
            self
        }
        pub fn delete_webhook_endpoint_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedWebhookEndpointResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_webhook_endpoint_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_webhook_endpoint_response: {e}")
            });
            self
        }
        pub fn list_webhook_endpoints_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListWebhookEndpointsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_webhook_endpoints_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_webhook_endpoints_params: {e}")
            });
            self
        }
        pub fn list_webhook_endpoints_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_webhook_endpoints_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_webhook_endpoints_response: {e}")
            });
            self
        }
        pub fn list_webhook_event_types_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEventTypeListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_webhook_event_types_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_webhook_event_types_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_webhook_endpoint_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointBody>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_webhook_endpoint_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_webhook_endpoint_response: {e}"
                )
            });
            self
        }
        pub fn rotate_webhook_endpoint_signing_secret_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicRotateSecretBody>,
            T::Error: ::std::fmt::Display,
        {
            self . rotate_webhook_endpoint_signing_secret_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for rotate_webhook_endpoint_signing_secret_request: {e}")) ;
            self
        }
        pub fn rotate_webhook_endpoint_signing_secret_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointWithSecretResource>,
            T::Error: ::std::fmt::Display,
        {
            self . rotate_webhook_endpoint_signing_secret_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for rotate_webhook_endpoint_signing_secret_response: {e}")) ;
            self
        }
        pub fn test_webhook_endpoint_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicTestEndpointBody>,
            T::Error: ::std::fmt::Display,
        {
            self.test_webhook_endpoint_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for test_webhook_endpoint_request: {e}")
            });
            self
        }
        pub fn test_webhook_endpoint_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointTestResultResource>,
            T::Error: ::std::fmt::Display,
        {
            self.test_webhook_endpoint_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for test_webhook_endpoint_response: {e}")
            });
            self
        }
        pub fn update_webhook_endpoint_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PublicUpdateEndpointBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_webhook_endpoint_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_webhook_endpoint_request: {e}")
            });
            self
        }
        pub fn update_webhook_endpoint_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WebhookEndpointBody>,
            T::Error: ::std::fmt::Display,
        {
            self.update_webhook_endpoint_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for update_webhook_endpoint_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<WebhooksApiSchema> for super::WebhooksApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WebhooksApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_webhook_endpoint_request: value.create_webhook_endpoint_request?,
                create_webhook_endpoint_response: value.create_webhook_endpoint_response?,
                delete_webhook_endpoint_response: value.delete_webhook_endpoint_response?,
                list_webhook_endpoints_params: value.list_webhook_endpoints_params?,
                list_webhook_endpoints_response: value.list_webhook_endpoints_response?,
                list_webhook_event_types_response: value.list_webhook_event_types_response?,
                retrieve_webhook_endpoint_response: value.retrieve_webhook_endpoint_response?,
                rotate_webhook_endpoint_signing_secret_request: value
                    .rotate_webhook_endpoint_signing_secret_request?,
                rotate_webhook_endpoint_signing_secret_response: value
                    .rotate_webhook_endpoint_signing_secret_response?,
                test_webhook_endpoint_request: value.test_webhook_endpoint_request?,
                test_webhook_endpoint_response: value.test_webhook_endpoint_response?,
                update_webhook_endpoint_request: value.update_webhook_endpoint_request?,
                update_webhook_endpoint_response: value.update_webhook_endpoint_response?,
            })
        }
    }
    impl ::std::convert::From<super::WebhooksApiSchema> for WebhooksApiSchema {
        fn from(value: super::WebhooksApiSchema) -> Self {
            Self {
                create_webhook_endpoint_request: Ok(value.create_webhook_endpoint_request),
                create_webhook_endpoint_response: Ok(value.create_webhook_endpoint_response),
                delete_webhook_endpoint_response: Ok(value.delete_webhook_endpoint_response),
                list_webhook_endpoints_params: Ok(value.list_webhook_endpoints_params),
                list_webhook_endpoints_response: Ok(value.list_webhook_endpoints_response),
                list_webhook_event_types_response: Ok(value.list_webhook_event_types_response),
                retrieve_webhook_endpoint_response: Ok(value.retrieve_webhook_endpoint_response),
                rotate_webhook_endpoint_signing_secret_request: Ok(
                    value.rotate_webhook_endpoint_signing_secret_request
                ),
                rotate_webhook_endpoint_signing_secret_response: Ok(
                    value.rotate_webhook_endpoint_signing_secret_response
                ),
                test_webhook_endpoint_request: Ok(value.test_webhook_endpoint_request),
                test_webhook_endpoint_response: Ok(value.test_webhook_endpoint_response),
                update_webhook_endpoint_request: Ok(value.update_webhook_endpoint_request),
                update_webhook_endpoint_response: Ok(value.update_webhook_endpoint_response),
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
