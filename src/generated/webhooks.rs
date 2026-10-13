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
#[doc = "`PublicTestEndpointBody`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PublicTestEndpointBody {
    #[doc = "The event type to send as a sample delivery."]
    pub event_type: ProjectEventTypeEnum,
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
