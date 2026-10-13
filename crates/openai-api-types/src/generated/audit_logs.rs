pub use crate::common::{AwsExternalStorageProviderResponse, AzureExternalStorageProviderResponse, Error, ErrorResponse, GcpExternalStorageProviderResponse, MisalignmentErrorDetailsResource, OciExternalStorageProviderResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "A log of a user action or configuration change within this organization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AuditLog {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub actor: ::std::option::Option<AuditLogActor>,
    #[serde(
        rename = "api_key.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_key_created: ::std::option::Option<AuditLogApiKeyCreated>,
    #[serde(
        rename = "api_key.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_key_deleted: ::std::option::Option<AuditLogApiKeyDeleted>,
    #[serde(
        rename = "api_key.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_key_updated: ::std::option::Option<AuditLogApiKeyUpdated>,
    #[serde(
        rename = "certificate.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub certificate_created: ::std::option::Option<AuditLogCertificateCreated>,
    #[serde(
        rename = "certificate.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub certificate_deleted: ::std::option::Option<AuditLogCertificateDeleted>,
    #[serde(
        rename = "certificate.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub certificate_updated: ::std::option::Option<AuditLogCertificateUpdated>,
    #[serde(
        rename = "certificates.activated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub certificates_activated: ::std::option::Option<AuditLogCertificatesActivated>,
    #[serde(
        rename = "certificates.deactivated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub certificates_deactivated: ::std::option::Option<AuditLogCertificatesDeactivated>,
    #[serde(
        rename = "checkpoint.permission.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub checkpoint_permission_created: ::std::option::Option<AuditLogCheckpointPermissionCreated>,
    #[serde(
        rename = "checkpoint.permission.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub checkpoint_permission_deleted: ::std::option::Option<AuditLogCheckpointPermissionDeleted>,
    #[doc = "The Unix timestamp (in seconds) of the event."]
    pub effective_at: i64,
    #[serde(
        rename = "external_key.registered",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_key_registered: ::std::option::Option<AuditLogExternalKeyRegistered>,
    #[serde(
        rename = "external_key.removed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_key_removed: ::std::option::Option<AuditLogExternalKeyRemoved>,
    #[serde(
        rename = "external_storage.registered",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_storage_registered: ::std::option::Option<AuditLogExternalStorageRegistered>,
    #[serde(
        rename = "external_storage.removed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_storage_removed: ::std::option::Option<AuditLogExternalStorageRemoved>,
    #[serde(
        rename = "group.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_created: ::std::option::Option<AuditLogGroupCreated>,
    #[serde(
        rename = "group.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_deleted: ::std::option::Option<AuditLogGroupDeleted>,
    #[serde(
        rename = "group.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_updated: ::std::option::Option<AuditLogGroupUpdated>,
    #[doc = "The ID of this log."]
    pub id: ::std::string::String,
    #[serde(
        rename = "invite.accepted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub invite_accepted: ::std::option::Option<AuditLogInviteAccepted>,
    #[serde(
        rename = "invite.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub invite_deleted: ::std::option::Option<AuditLogInviteDeleted>,
    #[serde(
        rename = "invite.sent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub invite_sent: ::std::option::Option<AuditLogInviteSent>,
    #[serde(
        rename = "ip_allowlist.config.activated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_allowlist_config_activated: ::std::option::Option<AuditLogIpAllowlistConfigActivated>,
    #[serde(
        rename = "ip_allowlist.config.deactivated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_allowlist_config_deactivated:
        ::std::option::Option<AuditLogIpAllowlistConfigDeactivated>,
    #[serde(
        rename = "ip_allowlist.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_allowlist_created: ::std::option::Option<AuditLogIpAllowlistCreated>,
    #[serde(
        rename = "ip_allowlist.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_allowlist_deleted: ::std::option::Option<AuditLogIpAllowlistDeleted>,
    #[serde(
        rename = "ip_allowlist.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ip_allowlist_updated: ::std::option::Option<AuditLogIpAllowlistUpdated>,
    #[serde(
        rename = "login.failed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub login_failed: ::std::option::Option<AuditLogLoginFailed>,
    #[doc = "This event has no additional fields beyond the standard audit log attributes."]
    #[serde(
        rename = "login.succeeded",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub login_succeeded: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "logout.failed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub logout_failed: ::std::option::Option<AuditLogLogoutFailed>,
    #[doc = "This event has no additional fields beyond the standard audit log attributes."]
    #[serde(
        rename = "logout.succeeded",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub logout_succeeded: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "organization.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub organization_updated: ::std::option::Option<AuditLogOrganizationUpdated>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project: ::std::option::Option<AuditLogProject>,
    #[serde(
        rename = "project.archived",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_archived: ::std::option::Option<AuditLogProjectArchived>,
    #[serde(
        rename = "project.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_created: ::std::option::Option<AuditLogProjectCreated>,
    #[serde(
        rename = "project.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_deleted: ::std::option::Option<AuditLogProjectDeleted>,
    #[serde(
        rename = "project.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_updated: ::std::option::Option<AuditLogProjectUpdated>,
    #[serde(
        rename = "rate_limit.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit_deleted: ::std::option::Option<AuditLogRateLimitDeleted>,
    #[serde(
        rename = "rate_limit.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit_updated: ::std::option::Option<AuditLogRateLimitUpdated>,
    #[serde(
        rename = "role.assignment.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_assignment_created: ::std::option::Option<AuditLogRoleAssignmentCreated>,
    #[serde(
        rename = "role.assignment.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_assignment_deleted: ::std::option::Option<AuditLogRoleAssignmentDeleted>,
    #[serde(
        rename = "role.bound_to_resource",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_bound_to_resource: ::std::option::Option<AuditLogRoleBoundToResource>,
    #[serde(
        rename = "role.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_created: ::std::option::Option<AuditLogRoleCreated>,
    #[serde(
        rename = "role.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_deleted: ::std::option::Option<AuditLogRoleDeleted>,
    #[serde(
        rename = "role.unbound_from_resource",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_unbound_from_resource: ::std::option::Option<AuditLogRoleUnboundFromResource>,
    #[serde(
        rename = "role.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub role_updated: ::std::option::Option<AuditLogRoleUpdated>,
    #[serde(
        rename = "scim.disabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scim_disabled: ::std::option::Option<AuditLogScimDisabled>,
    #[serde(
        rename = "scim.enabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scim_enabled: ::std::option::Option<AuditLogScimEnabled>,
    #[serde(
        rename = "service_account.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_account_created: ::std::option::Option<AuditLogServiceAccountCreated>,
    #[serde(
        rename = "service_account.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_account_deleted: ::std::option::Option<AuditLogServiceAccountDeleted>,
    #[serde(
        rename = "service_account.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_account_updated: ::std::option::Option<AuditLogServiceAccountUpdated>,
    #[serde(rename = "type")]
    pub type_: AuditLogEventType,
    #[serde(
        rename = "user.added",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_added: ::std::option::Option<AuditLogUserAdded>,
    #[serde(
        rename = "user.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_deleted: ::std::option::Option<AuditLogUserDeleted>,
    #[serde(
        rename = "user.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_updated: ::std::option::Option<AuditLogUserUpdated>,
    #[serde(
        rename = "workload_identity_provider.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_created:
        ::std::option::Option<AuditLogWorkloadIdentityProviderCreated>,
    #[serde(
        rename = "workload_identity_provider.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_deleted:
        ::std::option::Option<AuditLogWorkloadIdentityProviderDeleted>,
    #[serde(
        rename = "workload_identity_provider_mapping.created",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_mapping_created:
        ::std::option::Option<AuditLogWorkloadIdentityProviderMappingCreated>,
    #[serde(
        rename = "workload_identity_provider_mapping.deleted",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_mapping_deleted:
        ::std::option::Option<AuditLogWorkloadIdentityProviderMappingDeleted>,
    #[serde(
        rename = "workload_identity_provider_mapping.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_mapping_updated:
        ::std::option::Option<AuditLogWorkloadIdentityProviderMappingUpdated>,
    #[serde(
        rename = "workload_identity_provider.updated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workload_identity_provider_updated:
        ::std::option::Option<AuditLogWorkloadIdentityProviderUpdated>,
}
impl AuditLog {
    pub fn builder() -> builder::AuditLog {
        Default::default()
    }
}
#[doc = "The actor who performed the audit logged action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogActor {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_key: ::std::option::Option<AuditLogActorApiKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub session: ::std::option::Option<AuditLogActorSession>,
    #[doc = "The type of actor. Is either `session` or `api_key`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<AuditLogActorType>,
}
impl AuditLogActor {
    pub fn builder() -> builder::AuditLogActor {
        Default::default()
    }
}
#[doc = "The API Key used to perform the audit logged action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogActorApiKey {
    #[doc = "The tracking id of the API key."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_account: ::std::option::Option<AuditLogActorServiceAccount>,
    #[doc = "The type of API key. Can be either `user` or `service_account`."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<AuditLogActorApiKeyType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<AuditLogActorUser>,
}
impl AuditLogActorApiKey {
    pub fn builder() -> builder::AuditLogActorApiKey {
        Default::default()
    }
}
#[doc = "The type of API key. Can be either `user` or `service_account`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuditLogActorApiKeyType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "service_account")]
    ServiceAccount,
}
impl ::std::fmt::Display for AuditLogActorApiKeyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::ServiceAccount => f.write_str("service_account"),
        }
    }
}
impl ::std::str::FromStr for AuditLogActorApiKeyType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "service_account" => Ok(Self::ServiceAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuditLogActorApiKeyType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuditLogActorApiKeyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The service account that performed the audit logged action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogActorServiceAccount {
    #[doc = "The service account id."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogActorServiceAccount {
    pub fn builder() -> builder::AuditLogActorServiceAccount {
        Default::default()
    }
}
#[doc = "The session in which the audit logged action was performed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogActorSession {
    #[doc = "The IP address from which the action was performed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<AuditLogActorUser>,
}
impl AuditLogActorSession {
    pub fn builder() -> builder::AuditLogActorSession {
        Default::default()
    }
}
#[doc = "The type of actor. Is either `session` or `api_key`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuditLogActorType {
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "api_key")]
    ApiKey,
}
impl ::std::fmt::Display for AuditLogActorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Session => f.write_str("session"),
            Self::ApiKey => f.write_str("api_key"),
        }
    }
}
impl ::std::str::FromStr for AuditLogActorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session" => Ok(Self::Session),
            "api_key" => Ok(Self::ApiKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuditLogActorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuditLogActorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The user who performed the audit logged action."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogActorUser {
    #[doc = "The user email."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "The user id."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogActorUser {
    pub fn builder() -> builder::AuditLogActorUser {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogApiKeyCreated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogApiKeyCreatedData>,
    #[doc = "The tracking ID of the API key."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogApiKeyCreated {
    pub fn builder() -> builder::AuditLogApiKeyCreated {
        Default::default()
    }
}
#[doc = "The payload used to create the API key."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogApiKeyCreatedData {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
impl AuditLogApiKeyCreatedData {
    pub fn builder() -> builder::AuditLogApiKeyCreatedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogApiKeyDeleted {
    #[doc = "The tracking ID of the API key."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogApiKeyDeleted {
    pub fn builder() -> builder::AuditLogApiKeyDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogApiKeyUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogApiKeyUpdatedChangesRequested>,
    #[doc = "The tracking ID of the API key."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogApiKeyUpdated {
    pub fn builder() -> builder::AuditLogApiKeyUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the API key."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogApiKeyUpdatedChangesRequested {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
impl AuditLogApiKeyUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogApiKeyUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificateCreated {
    #[doc = "The certificate ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogCertificateCreated {
    pub fn builder() -> builder::AuditLogCertificateCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificateDeleted {
    #[doc = "The certificate content in PEM format."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    #[doc = "The certificate ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogCertificateDeleted {
    pub fn builder() -> builder::AuditLogCertificateDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificateUpdated {
    #[doc = "The certificate ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogCertificateUpdated {
    pub fn builder() -> builder::AuditLogCertificateUpdated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificatesActivated {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub certificates: ::std::vec::Vec<AuditLogCertificatesActivatedCertificatesItem>,
}
impl AuditLogCertificatesActivated {
    pub fn builder() -> builder::AuditLogCertificatesActivated {
        Default::default()
    }
}
#[doc = "`AuditLogCertificatesActivatedCertificatesItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificatesActivatedCertificatesItem {
    #[doc = "The certificate ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogCertificatesActivatedCertificatesItem {
    pub fn builder() -> builder::AuditLogCertificatesActivatedCertificatesItem {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificatesDeactivated {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub certificates: ::std::vec::Vec<AuditLogCertificatesDeactivatedCertificatesItem>,
}
impl AuditLogCertificatesDeactivated {
    pub fn builder() -> builder::AuditLogCertificatesDeactivated {
        Default::default()
    }
}
#[doc = "`AuditLogCertificatesDeactivatedCertificatesItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCertificatesDeactivatedCertificatesItem {
    #[doc = "The certificate ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the certificate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogCertificatesDeactivatedCertificatesItem {
    pub fn builder() -> builder::AuditLogCertificatesDeactivatedCertificatesItem {
        Default::default()
    }
}
#[doc = "The project and fine-tuned model checkpoint that the checkpoint permission was created for."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCheckpointPermissionCreated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogCheckpointPermissionCreatedData>,
    #[doc = "The ID of the checkpoint permission."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogCheckpointPermissionCreated {
    pub fn builder() -> builder::AuditLogCheckpointPermissionCreated {
        Default::default()
    }
}
#[doc = "The payload used to create the checkpoint permission."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCheckpointPermissionCreatedData {
    #[doc = "The ID of the fine-tuned model checkpoint."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fine_tuned_model_checkpoint: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the project that the checkpoint permission was created for."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogCheckpointPermissionCreatedData {
    pub fn builder() -> builder::AuditLogCheckpointPermissionCreatedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogCheckpointPermissionDeleted {
    #[doc = "The ID of the checkpoint permission."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogCheckpointPermissionDeleted {
    pub fn builder() -> builder::AuditLogCheckpointPermissionDeleted {
        Default::default()
    }
}
#[doc = "The event type."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuditLogEventType {
    #[serde(rename = "api_key.created")]
    ApiKeyCreated,
    #[serde(rename = "api_key.updated")]
    ApiKeyUpdated,
    #[serde(rename = "api_key.deleted")]
    ApiKeyDeleted,
    #[serde(rename = "certificate.created")]
    CertificateCreated,
    #[serde(rename = "certificate.updated")]
    CertificateUpdated,
    #[serde(rename = "certificate.deleted")]
    CertificateDeleted,
    #[serde(rename = "certificates.activated")]
    CertificatesActivated,
    #[serde(rename = "certificates.deactivated")]
    CertificatesDeactivated,
    #[serde(rename = "checkpoint.permission.created")]
    CheckpointPermissionCreated,
    #[serde(rename = "checkpoint.permission.deleted")]
    CheckpointPermissionDeleted,
    #[serde(rename = "external_key.registered")]
    ExternalKeyRegistered,
    #[serde(rename = "external_key.removed")]
    ExternalKeyRemoved,
    #[serde(rename = "external_storage.registered")]
    ExternalStorageRegistered,
    #[serde(rename = "external_storage.removed")]
    ExternalStorageRemoved,
    #[serde(rename = "group.created")]
    GroupCreated,
    #[serde(rename = "group.updated")]
    GroupUpdated,
    #[serde(rename = "group.deleted")]
    GroupDeleted,
    #[serde(rename = "invite.sent")]
    InviteSent,
    #[serde(rename = "invite.accepted")]
    InviteAccepted,
    #[serde(rename = "invite.deleted")]
    InviteDeleted,
    #[serde(rename = "ip_allowlist.created")]
    IpAllowlistCreated,
    #[serde(rename = "ip_allowlist.updated")]
    IpAllowlistUpdated,
    #[serde(rename = "ip_allowlist.deleted")]
    IpAllowlistDeleted,
    #[serde(rename = "ip_allowlist.config.activated")]
    IpAllowlistConfigActivated,
    #[serde(rename = "ip_allowlist.config.deactivated")]
    IpAllowlistConfigDeactivated,
    #[serde(rename = "login.succeeded")]
    LoginSucceeded,
    #[serde(rename = "login.failed")]
    LoginFailed,
    #[serde(rename = "logout.succeeded")]
    LogoutSucceeded,
    #[serde(rename = "logout.failed")]
    LogoutFailed,
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,
    #[serde(rename = "project.created")]
    ProjectCreated,
    #[serde(rename = "project.updated")]
    ProjectUpdated,
    #[serde(rename = "project.archived")]
    ProjectArchived,
    #[serde(rename = "project.deleted")]
    ProjectDeleted,
    #[serde(rename = "rate_limit.updated")]
    RateLimitUpdated,
    #[serde(rename = "rate_limit.deleted")]
    RateLimitDeleted,
    #[serde(rename = "resource.deleted")]
    ResourceDeleted,
    #[serde(rename = "tunnel.created")]
    TunnelCreated,
    #[serde(rename = "tunnel.updated")]
    TunnelUpdated,
    #[serde(rename = "tunnel.deleted")]
    TunnelDeleted,
    #[serde(rename = "workload_identity_provider.created")]
    WorkloadIdentityProviderCreated,
    #[serde(rename = "workload_identity_provider.updated")]
    WorkloadIdentityProviderUpdated,
    #[serde(rename = "workload_identity_provider.deleted")]
    WorkloadIdentityProviderDeleted,
    #[serde(rename = "workload_identity_provider_mapping.created")]
    WorkloadIdentityProviderMappingCreated,
    #[serde(rename = "workload_identity_provider_mapping.updated")]
    WorkloadIdentityProviderMappingUpdated,
    #[serde(rename = "workload_identity_provider_mapping.deleted")]
    WorkloadIdentityProviderMappingDeleted,
    #[serde(rename = "role.created")]
    RoleCreated,
    #[serde(rename = "role.updated")]
    RoleUpdated,
    #[serde(rename = "role.deleted")]
    RoleDeleted,
    #[serde(rename = "role.assignment.created")]
    RoleAssignmentCreated,
    #[serde(rename = "role.assignment.deleted")]
    RoleAssignmentDeleted,
    #[serde(rename = "role.bound_to_resource")]
    RoleBoundToResource,
    #[serde(rename = "role.unbound_from_resource")]
    RoleUnboundFromResource,
    #[serde(rename = "scim.enabled")]
    ScimEnabled,
    #[serde(rename = "scim.disabled")]
    ScimDisabled,
    #[serde(rename = "service_account.created")]
    ServiceAccountCreated,
    #[serde(rename = "service_account.updated")]
    ServiceAccountUpdated,
    #[serde(rename = "service_account.deleted")]
    ServiceAccountDeleted,
    #[serde(rename = "user.added")]
    UserAdded,
    #[serde(rename = "user.updated")]
    UserUpdated,
    #[serde(rename = "user.deleted")]
    UserDeleted,
    #[serde(rename = "tenant.metadata.updated")]
    TenantMetadataUpdated,
    #[serde(rename = "tenant.microsoft_entra_mapping.upserted")]
    TenantMicrosoftEntraMappingUpserted,
    #[serde(rename = "tenant.microsoft_entra_mapping.deleted")]
    TenantMicrosoftEntraMappingDeleted,
    #[serde(rename = "tenant.workload_identity.provider.created")]
    TenantWorkloadIdentityProviderCreated,
    #[serde(rename = "tenant.workload_identity.provider.updated")]
    TenantWorkloadIdentityProviderUpdated,
    #[serde(rename = "tenant.workload_identity.provider.archived")]
    TenantWorkloadIdentityProviderArchived,
    #[serde(rename = "tenant.workload_identity.mapping.created")]
    TenantWorkloadIdentityMappingCreated,
    #[serde(rename = "tenant.workload_identity.mapping.updated")]
    TenantWorkloadIdentityMappingUpdated,
    #[serde(rename = "tenant.workload_identity.mapping.archived")]
    TenantWorkloadIdentityMappingArchived,
    #[serde(rename = "tenant.workload_identity.binding.created")]
    TenantWorkloadIdentityBindingCreated,
    #[serde(rename = "tenant.workload_identity.principal.provisioned")]
    TenantWorkloadIdentityPrincipalProvisioned,
    #[serde(rename = "tenant.workload_identity.access_token.issued")]
    TenantWorkloadIdentityAccessTokenIssued,
    #[serde(rename = "tenant.admin_api_key.created")]
    TenantAdminApiKeyCreated,
    #[serde(rename = "tenant.admin_api_key.updated")]
    TenantAdminApiKeyUpdated,
    #[serde(rename = "tenant.admin_api_key.deleted")]
    TenantAdminApiKeyDeleted,
    #[serde(rename = "tenant.project_api_key.created")]
    TenantProjectApiKeyCreated,
    #[serde(rename = "tenant.trusted_access.business_verification.started")]
    TenantTrustedAccessBusinessVerificationStarted,
    #[serde(rename = "tenant.trusted_access.application.submitted")]
    TenantTrustedAccessApplicationSubmitted,
    #[serde(rename = "tenant.chatgpt_access_token.revoked")]
    TenantChatgptAccessTokenRevoked,
    #[serde(rename = "tenant.migration.completed")]
    TenantMigrationCompleted,
    #[serde(rename = "tenant.sso.migrated")]
    TenantSsoMigrated,
    #[serde(rename = "tenant.domains.migrated")]
    TenantDomainsMigrated,
    #[serde(rename = "tenant.sso_connection.created")]
    TenantSsoConnectionCreated,
    #[serde(rename = "tenant.sso_connection.updated")]
    TenantSsoConnectionUpdated,
    #[serde(rename = "tenant.sso_connection.deleted")]
    TenantSsoConnectionDeleted,
    #[serde(rename = "tenant.sso_connection.setup.started")]
    TenantSsoConnectionSetupStarted,
    #[serde(rename = "tenant.policy.created")]
    TenantPolicyCreated,
    #[serde(rename = "tenant.policy.updated")]
    TenantPolicyUpdated,
    #[serde(rename = "tenant.policy.deleted")]
    TenantPolicyDeleted,
    #[serde(rename = "tenant.policy.attached")]
    TenantPolicyAttached,
    #[serde(rename = "tenant.policy.detached")]
    TenantPolicyDetached,
    #[serde(rename = "tenant.principal_authentication_policy.resolved")]
    TenantPrincipalAuthenticationPolicyResolved,
    #[serde(rename = "tenant.scim.setup.started")]
    TenantScimSetupStarted,
    #[serde(rename = "tenant.scim.deletion.requested")]
    TenantScimDeletionRequested,
    #[serde(rename = "tenant.scim.directory.created")]
    TenantScimDirectoryCreated,
    #[serde(rename = "tenant.product_access_policy.updated")]
    TenantProductAccessPolicyUpdated,
    #[serde(rename = "tenant.resource_share_grant.created")]
    TenantResourceShareGrantCreated,
    #[serde(rename = "tenant.resource_share_grant.updated")]
    TenantResourceShareGrantUpdated,
    #[serde(rename = "tenant.resource_share_grant.accepted")]
    TenantResourceShareGrantAccepted,
    #[serde(rename = "tenant.resource_share_grant.declined")]
    TenantResourceShareGrantDeclined,
    #[serde(rename = "tenant.resource_share_grant.revoked")]
    TenantResourceShareGrantRevoked,
    #[serde(rename = "tenant.resource_share_grant.deleted")]
    TenantResourceShareGrantDeleted,
    #[serde(rename = "tenant.service_account.updated")]
    TenantServiceAccountUpdated,
    #[serde(rename = "tenant.service_account.deleted")]
    TenantServiceAccountDeleted,
    #[serde(rename = "tenant.service_account.token.revoked")]
    TenantServiceAccountTokenRevoked,
    #[serde(rename = "tenant.billing.overage_limit.updated")]
    TenantBillingOverageLimitUpdated,
    #[serde(rename = "tenant.billing.alerts.updated")]
    TenantBillingAlertsUpdated,
    #[serde(rename = "tenant.billing.info.updated")]
    TenantBillingInfoUpdated,
    #[serde(rename = "tenant.usage_limit.workspace.updated")]
    TenantUsageLimitWorkspaceUpdated,
    #[serde(rename = "tenant.usage_limit.group.updated")]
    TenantUsageLimitGroupUpdated,
    #[serde(rename = "tenant.usage_limit.user.updated")]
    TenantUsageLimitUserUpdated,
    #[serde(rename = "tenant.usage_limit.increase_request.updated")]
    TenantUsageLimitIncreaseRequestUpdated,
    #[serde(rename = "tenant.usage_limit.increase_request.resolved")]
    TenantUsageLimitIncreaseRequestResolved,
    #[serde(rename = "tenant.group.created")]
    TenantGroupCreated,
    #[serde(rename = "tenant.group.updated")]
    TenantGroupUpdated,
    #[serde(rename = "tenant.group.deleted")]
    TenantGroupDeleted,
    #[serde(rename = "tenant.group.member.added")]
    TenantGroupMemberAdded,
    #[serde(rename = "tenant.group.member.removed")]
    TenantGroupMemberRemoved,
    #[serde(rename = "tenant.migration_rollout.status.updated")]
    TenantMigrationRolloutStatusUpdated,
    #[serde(rename = "tenant.migration_rollout.tier.updated")]
    TenantMigrationRolloutTierUpdated,
    #[serde(rename = "tenant.role.metadata.updated")]
    TenantRoleMetadataUpdated,
    #[serde(rename = "tenant.custom_role.created")]
    TenantCustomRoleCreated,
    #[serde(rename = "tenant.custom_role.updated")]
    TenantCustomRoleUpdated,
    #[serde(rename = "tenant.custom_role.deleted")]
    TenantCustomRoleDeleted,
    #[serde(rename = "tenant.role_assignment.created")]
    TenantRoleAssignmentCreated,
    #[serde(rename = "tenant.role_assignment.deleted")]
    TenantRoleAssignmentDeleted,
    #[serde(rename = "tenant.resource_role_assignment.created")]
    TenantResourceRoleAssignmentCreated,
    #[serde(rename = "tenant.resource_role_assignment.deleted")]
    TenantResourceRoleAssignmentDeleted,
    #[serde(rename = "tenant.resource_access.updated")]
    TenantResourceAccessUpdated,
    #[serde(rename = "tenant.resource_access.deleted")]
    TenantResourceAccessDeleted,
    #[serde(rename = "tenant.ads_account.onboarding.redemption")]
    TenantAdsAccountOnboardingRedemption,
    #[serde(rename = "tenant.session_policy.created")]
    TenantSessionPolicyCreated,
    #[serde(rename = "tenant.session_policy.updated")]
    TenantSessionPolicyUpdated,
    #[serde(rename = "tenant.session_policy.deleted")]
    TenantSessionPolicyDeleted,
    #[serde(rename = "tenant.session_revocation.started")]
    TenantSessionRevocationStarted,
    #[serde(rename = "tenant.third_party_app_policy.updated")]
    TenantThirdPartyAppPolicyUpdated,
    #[serde(rename = "tenant.user.added")]
    TenantUserAdded,
    #[serde(rename = "tenant.user.updated")]
    TenantUserUpdated,
    #[serde(rename = "tenant.user.removed")]
    TenantUserRemoved,
    #[serde(rename = "tenant.user.looked_up")]
    TenantUserLookedUp,
    #[serde(rename = "tenant.user.invited")]
    TenantUserInvited,
    #[serde(rename = "tenant.membership.revoked")]
    TenantMembershipRevoked,
    #[serde(rename = "tenant.api_organization_invite.upserted")]
    TenantApiOrganizationInviteUpserted,
    #[serde(rename = "tenant.api_organization_invite.deleted")]
    TenantApiOrganizationInviteDeleted,
    #[serde(rename = "tenant.chatgpt_workspace_invite.upserted")]
    TenantChatgptWorkspaceInviteUpserted,
    #[serde(rename = "tenant.membership.accepted")]
    TenantMembershipAccepted,
    #[serde(rename = "tenant.membership.declined")]
    TenantMembershipDeclined,
    #[serde(rename = "tenant.workspace_invite_email_settings.updated")]
    TenantWorkspaceInviteEmailSettingsUpdated,
}
impl ::std::fmt::Display for AuditLogEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApiKeyCreated => f.write_str("api_key.created"),
            Self::ApiKeyUpdated => f.write_str("api_key.updated"),
            Self::ApiKeyDeleted => f.write_str("api_key.deleted"),
            Self::CertificateCreated => f.write_str("certificate.created"),
            Self::CertificateUpdated => f.write_str("certificate.updated"),
            Self::CertificateDeleted => f.write_str("certificate.deleted"),
            Self::CertificatesActivated => f.write_str("certificates.activated"),
            Self::CertificatesDeactivated => f.write_str("certificates.deactivated"),
            Self::CheckpointPermissionCreated => f.write_str("checkpoint.permission.created"),
            Self::CheckpointPermissionDeleted => f.write_str("checkpoint.permission.deleted"),
            Self::ExternalKeyRegistered => f.write_str("external_key.registered"),
            Self::ExternalKeyRemoved => f.write_str("external_key.removed"),
            Self::ExternalStorageRegistered => f.write_str("external_storage.registered"),
            Self::ExternalStorageRemoved => f.write_str("external_storage.removed"),
            Self::GroupCreated => f.write_str("group.created"),
            Self::GroupUpdated => f.write_str("group.updated"),
            Self::GroupDeleted => f.write_str("group.deleted"),
            Self::InviteSent => f.write_str("invite.sent"),
            Self::InviteAccepted => f.write_str("invite.accepted"),
            Self::InviteDeleted => f.write_str("invite.deleted"),
            Self::IpAllowlistCreated => f.write_str("ip_allowlist.created"),
            Self::IpAllowlistUpdated => f.write_str("ip_allowlist.updated"),
            Self::IpAllowlistDeleted => f.write_str("ip_allowlist.deleted"),
            Self::IpAllowlistConfigActivated => f.write_str("ip_allowlist.config.activated"),
            Self::IpAllowlistConfigDeactivated => f.write_str("ip_allowlist.config.deactivated"),
            Self::LoginSucceeded => f.write_str("login.succeeded"),
            Self::LoginFailed => f.write_str("login.failed"),
            Self::LogoutSucceeded => f.write_str("logout.succeeded"),
            Self::LogoutFailed => f.write_str("logout.failed"),
            Self::OrganizationUpdated => f.write_str("organization.updated"),
            Self::ProjectCreated => f.write_str("project.created"),
            Self::ProjectUpdated => f.write_str("project.updated"),
            Self::ProjectArchived => f.write_str("project.archived"),
            Self::ProjectDeleted => f.write_str("project.deleted"),
            Self::RateLimitUpdated => f.write_str("rate_limit.updated"),
            Self::RateLimitDeleted => f.write_str("rate_limit.deleted"),
            Self::ResourceDeleted => f.write_str("resource.deleted"),
            Self::TunnelCreated => f.write_str("tunnel.created"),
            Self::TunnelUpdated => f.write_str("tunnel.updated"),
            Self::TunnelDeleted => f.write_str("tunnel.deleted"),
            Self::WorkloadIdentityProviderCreated => {
                f.write_str("workload_identity_provider.created")
            }
            Self::WorkloadIdentityProviderUpdated => {
                f.write_str("workload_identity_provider.updated")
            }
            Self::WorkloadIdentityProviderDeleted => {
                f.write_str("workload_identity_provider.deleted")
            }
            Self::WorkloadIdentityProviderMappingCreated => {
                f.write_str("workload_identity_provider_mapping.created")
            }
            Self::WorkloadIdentityProviderMappingUpdated => {
                f.write_str("workload_identity_provider_mapping.updated")
            }
            Self::WorkloadIdentityProviderMappingDeleted => {
                f.write_str("workload_identity_provider_mapping.deleted")
            }
            Self::RoleCreated => f.write_str("role.created"),
            Self::RoleUpdated => f.write_str("role.updated"),
            Self::RoleDeleted => f.write_str("role.deleted"),
            Self::RoleAssignmentCreated => f.write_str("role.assignment.created"),
            Self::RoleAssignmentDeleted => f.write_str("role.assignment.deleted"),
            Self::RoleBoundToResource => f.write_str("role.bound_to_resource"),
            Self::RoleUnboundFromResource => f.write_str("role.unbound_from_resource"),
            Self::ScimEnabled => f.write_str("scim.enabled"),
            Self::ScimDisabled => f.write_str("scim.disabled"),
            Self::ServiceAccountCreated => f.write_str("service_account.created"),
            Self::ServiceAccountUpdated => f.write_str("service_account.updated"),
            Self::ServiceAccountDeleted => f.write_str("service_account.deleted"),
            Self::UserAdded => f.write_str("user.added"),
            Self::UserUpdated => f.write_str("user.updated"),
            Self::UserDeleted => f.write_str("user.deleted"),
            Self::TenantMetadataUpdated => f.write_str("tenant.metadata.updated"),
            Self::TenantMicrosoftEntraMappingUpserted => {
                f.write_str("tenant.microsoft_entra_mapping.upserted")
            }
            Self::TenantMicrosoftEntraMappingDeleted => {
                f.write_str("tenant.microsoft_entra_mapping.deleted")
            }
            Self::TenantWorkloadIdentityProviderCreated => {
                f.write_str("tenant.workload_identity.provider.created")
            }
            Self::TenantWorkloadIdentityProviderUpdated => {
                f.write_str("tenant.workload_identity.provider.updated")
            }
            Self::TenantWorkloadIdentityProviderArchived => {
                f.write_str("tenant.workload_identity.provider.archived")
            }
            Self::TenantWorkloadIdentityMappingCreated => {
                f.write_str("tenant.workload_identity.mapping.created")
            }
            Self::TenantWorkloadIdentityMappingUpdated => {
                f.write_str("tenant.workload_identity.mapping.updated")
            }
            Self::TenantWorkloadIdentityMappingArchived => {
                f.write_str("tenant.workload_identity.mapping.archived")
            }
            Self::TenantWorkloadIdentityBindingCreated => {
                f.write_str("tenant.workload_identity.binding.created")
            }
            Self::TenantWorkloadIdentityPrincipalProvisioned => {
                f.write_str("tenant.workload_identity.principal.provisioned")
            }
            Self::TenantWorkloadIdentityAccessTokenIssued => {
                f.write_str("tenant.workload_identity.access_token.issued")
            }
            Self::TenantAdminApiKeyCreated => f.write_str("tenant.admin_api_key.created"),
            Self::TenantAdminApiKeyUpdated => f.write_str("tenant.admin_api_key.updated"),
            Self::TenantAdminApiKeyDeleted => f.write_str("tenant.admin_api_key.deleted"),
            Self::TenantProjectApiKeyCreated => f.write_str("tenant.project_api_key.created"),
            Self::TenantTrustedAccessBusinessVerificationStarted => {
                f.write_str("tenant.trusted_access.business_verification.started")
            }
            Self::TenantTrustedAccessApplicationSubmitted => {
                f.write_str("tenant.trusted_access.application.submitted")
            }
            Self::TenantChatgptAccessTokenRevoked => {
                f.write_str("tenant.chatgpt_access_token.revoked")
            }
            Self::TenantMigrationCompleted => f.write_str("tenant.migration.completed"),
            Self::TenantSsoMigrated => f.write_str("tenant.sso.migrated"),
            Self::TenantDomainsMigrated => f.write_str("tenant.domains.migrated"),
            Self::TenantSsoConnectionCreated => f.write_str("tenant.sso_connection.created"),
            Self::TenantSsoConnectionUpdated => f.write_str("tenant.sso_connection.updated"),
            Self::TenantSsoConnectionDeleted => f.write_str("tenant.sso_connection.deleted"),
            Self::TenantSsoConnectionSetupStarted => {
                f.write_str("tenant.sso_connection.setup.started")
            }
            Self::TenantPolicyCreated => f.write_str("tenant.policy.created"),
            Self::TenantPolicyUpdated => f.write_str("tenant.policy.updated"),
            Self::TenantPolicyDeleted => f.write_str("tenant.policy.deleted"),
            Self::TenantPolicyAttached => f.write_str("tenant.policy.attached"),
            Self::TenantPolicyDetached => f.write_str("tenant.policy.detached"),
            Self::TenantPrincipalAuthenticationPolicyResolved => {
                f.write_str("tenant.principal_authentication_policy.resolved")
            }
            Self::TenantScimSetupStarted => f.write_str("tenant.scim.setup.started"),
            Self::TenantScimDeletionRequested => f.write_str("tenant.scim.deletion.requested"),
            Self::TenantScimDirectoryCreated => f.write_str("tenant.scim.directory.created"),
            Self::TenantProductAccessPolicyUpdated => {
                f.write_str("tenant.product_access_policy.updated")
            }
            Self::TenantResourceShareGrantCreated => {
                f.write_str("tenant.resource_share_grant.created")
            }
            Self::TenantResourceShareGrantUpdated => {
                f.write_str("tenant.resource_share_grant.updated")
            }
            Self::TenantResourceShareGrantAccepted => {
                f.write_str("tenant.resource_share_grant.accepted")
            }
            Self::TenantResourceShareGrantDeclined => {
                f.write_str("tenant.resource_share_grant.declined")
            }
            Self::TenantResourceShareGrantRevoked => {
                f.write_str("tenant.resource_share_grant.revoked")
            }
            Self::TenantResourceShareGrantDeleted => {
                f.write_str("tenant.resource_share_grant.deleted")
            }
            Self::TenantServiceAccountUpdated => f.write_str("tenant.service_account.updated"),
            Self::TenantServiceAccountDeleted => f.write_str("tenant.service_account.deleted"),
            Self::TenantServiceAccountTokenRevoked => {
                f.write_str("tenant.service_account.token.revoked")
            }
            Self::TenantBillingOverageLimitUpdated => {
                f.write_str("tenant.billing.overage_limit.updated")
            }
            Self::TenantBillingAlertsUpdated => f.write_str("tenant.billing.alerts.updated"),
            Self::TenantBillingInfoUpdated => f.write_str("tenant.billing.info.updated"),
            Self::TenantUsageLimitWorkspaceUpdated => {
                f.write_str("tenant.usage_limit.workspace.updated")
            }
            Self::TenantUsageLimitGroupUpdated => f.write_str("tenant.usage_limit.group.updated"),
            Self::TenantUsageLimitUserUpdated => f.write_str("tenant.usage_limit.user.updated"),
            Self::TenantUsageLimitIncreaseRequestUpdated => {
                f.write_str("tenant.usage_limit.increase_request.updated")
            }
            Self::TenantUsageLimitIncreaseRequestResolved => {
                f.write_str("tenant.usage_limit.increase_request.resolved")
            }
            Self::TenantGroupCreated => f.write_str("tenant.group.created"),
            Self::TenantGroupUpdated => f.write_str("tenant.group.updated"),
            Self::TenantGroupDeleted => f.write_str("tenant.group.deleted"),
            Self::TenantGroupMemberAdded => f.write_str("tenant.group.member.added"),
            Self::TenantGroupMemberRemoved => f.write_str("tenant.group.member.removed"),
            Self::TenantMigrationRolloutStatusUpdated => {
                f.write_str("tenant.migration_rollout.status.updated")
            }
            Self::TenantMigrationRolloutTierUpdated => {
                f.write_str("tenant.migration_rollout.tier.updated")
            }
            Self::TenantRoleMetadataUpdated => f.write_str("tenant.role.metadata.updated"),
            Self::TenantCustomRoleCreated => f.write_str("tenant.custom_role.created"),
            Self::TenantCustomRoleUpdated => f.write_str("tenant.custom_role.updated"),
            Self::TenantCustomRoleDeleted => f.write_str("tenant.custom_role.deleted"),
            Self::TenantRoleAssignmentCreated => f.write_str("tenant.role_assignment.created"),
            Self::TenantRoleAssignmentDeleted => f.write_str("tenant.role_assignment.deleted"),
            Self::TenantResourceRoleAssignmentCreated => {
                f.write_str("tenant.resource_role_assignment.created")
            }
            Self::TenantResourceRoleAssignmentDeleted => {
                f.write_str("tenant.resource_role_assignment.deleted")
            }
            Self::TenantResourceAccessUpdated => f.write_str("tenant.resource_access.updated"),
            Self::TenantResourceAccessDeleted => f.write_str("tenant.resource_access.deleted"),
            Self::TenantAdsAccountOnboardingRedemption => {
                f.write_str("tenant.ads_account.onboarding.redemption")
            }
            Self::TenantSessionPolicyCreated => f.write_str("tenant.session_policy.created"),
            Self::TenantSessionPolicyUpdated => f.write_str("tenant.session_policy.updated"),
            Self::TenantSessionPolicyDeleted => f.write_str("tenant.session_policy.deleted"),
            Self::TenantSessionRevocationStarted => {
                f.write_str("tenant.session_revocation.started")
            }
            Self::TenantThirdPartyAppPolicyUpdated => {
                f.write_str("tenant.third_party_app_policy.updated")
            }
            Self::TenantUserAdded => f.write_str("tenant.user.added"),
            Self::TenantUserUpdated => f.write_str("tenant.user.updated"),
            Self::TenantUserRemoved => f.write_str("tenant.user.removed"),
            Self::TenantUserLookedUp => f.write_str("tenant.user.looked_up"),
            Self::TenantUserInvited => f.write_str("tenant.user.invited"),
            Self::TenantMembershipRevoked => f.write_str("tenant.membership.revoked"),
            Self::TenantApiOrganizationInviteUpserted => {
                f.write_str("tenant.api_organization_invite.upserted")
            }
            Self::TenantApiOrganizationInviteDeleted => {
                f.write_str("tenant.api_organization_invite.deleted")
            }
            Self::TenantChatgptWorkspaceInviteUpserted => {
                f.write_str("tenant.chatgpt_workspace_invite.upserted")
            }
            Self::TenantMembershipAccepted => f.write_str("tenant.membership.accepted"),
            Self::TenantMembershipDeclined => f.write_str("tenant.membership.declined"),
            Self::TenantWorkspaceInviteEmailSettingsUpdated => {
                f.write_str("tenant.workspace_invite_email_settings.updated")
            }
        }
    }
}
impl ::std::str::FromStr for AuditLogEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "api_key.created" => Ok(Self::ApiKeyCreated),
            "api_key.updated" => Ok(Self::ApiKeyUpdated),
            "api_key.deleted" => Ok(Self::ApiKeyDeleted),
            "certificate.created" => Ok(Self::CertificateCreated),
            "certificate.updated" => Ok(Self::CertificateUpdated),
            "certificate.deleted" => Ok(Self::CertificateDeleted),
            "certificates.activated" => Ok(Self::CertificatesActivated),
            "certificates.deactivated" => Ok(Self::CertificatesDeactivated),
            "checkpoint.permission.created" => Ok(Self::CheckpointPermissionCreated),
            "checkpoint.permission.deleted" => Ok(Self::CheckpointPermissionDeleted),
            "external_key.registered" => Ok(Self::ExternalKeyRegistered),
            "external_key.removed" => Ok(Self::ExternalKeyRemoved),
            "external_storage.registered" => Ok(Self::ExternalStorageRegistered),
            "external_storage.removed" => Ok(Self::ExternalStorageRemoved),
            "group.created" => Ok(Self::GroupCreated),
            "group.updated" => Ok(Self::GroupUpdated),
            "group.deleted" => Ok(Self::GroupDeleted),
            "invite.sent" => Ok(Self::InviteSent),
            "invite.accepted" => Ok(Self::InviteAccepted),
            "invite.deleted" => Ok(Self::InviteDeleted),
            "ip_allowlist.created" => Ok(Self::IpAllowlistCreated),
            "ip_allowlist.updated" => Ok(Self::IpAllowlistUpdated),
            "ip_allowlist.deleted" => Ok(Self::IpAllowlistDeleted),
            "ip_allowlist.config.activated" => Ok(Self::IpAllowlistConfigActivated),
            "ip_allowlist.config.deactivated" => Ok(Self::IpAllowlistConfigDeactivated),
            "login.succeeded" => Ok(Self::LoginSucceeded),
            "login.failed" => Ok(Self::LoginFailed),
            "logout.succeeded" => Ok(Self::LogoutSucceeded),
            "logout.failed" => Ok(Self::LogoutFailed),
            "organization.updated" => Ok(Self::OrganizationUpdated),
            "project.created" => Ok(Self::ProjectCreated),
            "project.updated" => Ok(Self::ProjectUpdated),
            "project.archived" => Ok(Self::ProjectArchived),
            "project.deleted" => Ok(Self::ProjectDeleted),
            "rate_limit.updated" => Ok(Self::RateLimitUpdated),
            "rate_limit.deleted" => Ok(Self::RateLimitDeleted),
            "resource.deleted" => Ok(Self::ResourceDeleted),
            "tunnel.created" => Ok(Self::TunnelCreated),
            "tunnel.updated" => Ok(Self::TunnelUpdated),
            "tunnel.deleted" => Ok(Self::TunnelDeleted),
            "workload_identity_provider.created" => Ok(Self::WorkloadIdentityProviderCreated),
            "workload_identity_provider.updated" => Ok(Self::WorkloadIdentityProviderUpdated),
            "workload_identity_provider.deleted" => Ok(Self::WorkloadIdentityProviderDeleted),
            "workload_identity_provider_mapping.created" => {
                Ok(Self::WorkloadIdentityProviderMappingCreated)
            }
            "workload_identity_provider_mapping.updated" => {
                Ok(Self::WorkloadIdentityProviderMappingUpdated)
            }
            "workload_identity_provider_mapping.deleted" => {
                Ok(Self::WorkloadIdentityProviderMappingDeleted)
            }
            "role.created" => Ok(Self::RoleCreated),
            "role.updated" => Ok(Self::RoleUpdated),
            "role.deleted" => Ok(Self::RoleDeleted),
            "role.assignment.created" => Ok(Self::RoleAssignmentCreated),
            "role.assignment.deleted" => Ok(Self::RoleAssignmentDeleted),
            "role.bound_to_resource" => Ok(Self::RoleBoundToResource),
            "role.unbound_from_resource" => Ok(Self::RoleUnboundFromResource),
            "scim.enabled" => Ok(Self::ScimEnabled),
            "scim.disabled" => Ok(Self::ScimDisabled),
            "service_account.created" => Ok(Self::ServiceAccountCreated),
            "service_account.updated" => Ok(Self::ServiceAccountUpdated),
            "service_account.deleted" => Ok(Self::ServiceAccountDeleted),
            "user.added" => Ok(Self::UserAdded),
            "user.updated" => Ok(Self::UserUpdated),
            "user.deleted" => Ok(Self::UserDeleted),
            "tenant.metadata.updated" => Ok(Self::TenantMetadataUpdated),
            "tenant.microsoft_entra_mapping.upserted" => {
                Ok(Self::TenantMicrosoftEntraMappingUpserted)
            }
            "tenant.microsoft_entra_mapping.deleted" => {
                Ok(Self::TenantMicrosoftEntraMappingDeleted)
            }
            "tenant.workload_identity.provider.created" => {
                Ok(Self::TenantWorkloadIdentityProviderCreated)
            }
            "tenant.workload_identity.provider.updated" => {
                Ok(Self::TenantWorkloadIdentityProviderUpdated)
            }
            "tenant.workload_identity.provider.archived" => {
                Ok(Self::TenantWorkloadIdentityProviderArchived)
            }
            "tenant.workload_identity.mapping.created" => {
                Ok(Self::TenantWorkloadIdentityMappingCreated)
            }
            "tenant.workload_identity.mapping.updated" => {
                Ok(Self::TenantWorkloadIdentityMappingUpdated)
            }
            "tenant.workload_identity.mapping.archived" => {
                Ok(Self::TenantWorkloadIdentityMappingArchived)
            }
            "tenant.workload_identity.binding.created" => {
                Ok(Self::TenantWorkloadIdentityBindingCreated)
            }
            "tenant.workload_identity.principal.provisioned" => {
                Ok(Self::TenantWorkloadIdentityPrincipalProvisioned)
            }
            "tenant.workload_identity.access_token.issued" => {
                Ok(Self::TenantWorkloadIdentityAccessTokenIssued)
            }
            "tenant.admin_api_key.created" => Ok(Self::TenantAdminApiKeyCreated),
            "tenant.admin_api_key.updated" => Ok(Self::TenantAdminApiKeyUpdated),
            "tenant.admin_api_key.deleted" => Ok(Self::TenantAdminApiKeyDeleted),
            "tenant.project_api_key.created" => Ok(Self::TenantProjectApiKeyCreated),
            "tenant.trusted_access.business_verification.started" => {
                Ok(Self::TenantTrustedAccessBusinessVerificationStarted)
            }
            "tenant.trusted_access.application.submitted" => {
                Ok(Self::TenantTrustedAccessApplicationSubmitted)
            }
            "tenant.chatgpt_access_token.revoked" => Ok(Self::TenantChatgptAccessTokenRevoked),
            "tenant.migration.completed" => Ok(Self::TenantMigrationCompleted),
            "tenant.sso.migrated" => Ok(Self::TenantSsoMigrated),
            "tenant.domains.migrated" => Ok(Self::TenantDomainsMigrated),
            "tenant.sso_connection.created" => Ok(Self::TenantSsoConnectionCreated),
            "tenant.sso_connection.updated" => Ok(Self::TenantSsoConnectionUpdated),
            "tenant.sso_connection.deleted" => Ok(Self::TenantSsoConnectionDeleted),
            "tenant.sso_connection.setup.started" => Ok(Self::TenantSsoConnectionSetupStarted),
            "tenant.policy.created" => Ok(Self::TenantPolicyCreated),
            "tenant.policy.updated" => Ok(Self::TenantPolicyUpdated),
            "tenant.policy.deleted" => Ok(Self::TenantPolicyDeleted),
            "tenant.policy.attached" => Ok(Self::TenantPolicyAttached),
            "tenant.policy.detached" => Ok(Self::TenantPolicyDetached),
            "tenant.principal_authentication_policy.resolved" => {
                Ok(Self::TenantPrincipalAuthenticationPolicyResolved)
            }
            "tenant.scim.setup.started" => Ok(Self::TenantScimSetupStarted),
            "tenant.scim.deletion.requested" => Ok(Self::TenantScimDeletionRequested),
            "tenant.scim.directory.created" => Ok(Self::TenantScimDirectoryCreated),
            "tenant.product_access_policy.updated" => Ok(Self::TenantProductAccessPolicyUpdated),
            "tenant.resource_share_grant.created" => Ok(Self::TenantResourceShareGrantCreated),
            "tenant.resource_share_grant.updated" => Ok(Self::TenantResourceShareGrantUpdated),
            "tenant.resource_share_grant.accepted" => Ok(Self::TenantResourceShareGrantAccepted),
            "tenant.resource_share_grant.declined" => Ok(Self::TenantResourceShareGrantDeclined),
            "tenant.resource_share_grant.revoked" => Ok(Self::TenantResourceShareGrantRevoked),
            "tenant.resource_share_grant.deleted" => Ok(Self::TenantResourceShareGrantDeleted),
            "tenant.service_account.updated" => Ok(Self::TenantServiceAccountUpdated),
            "tenant.service_account.deleted" => Ok(Self::TenantServiceAccountDeleted),
            "tenant.service_account.token.revoked" => Ok(Self::TenantServiceAccountTokenRevoked),
            "tenant.billing.overage_limit.updated" => Ok(Self::TenantBillingOverageLimitUpdated),
            "tenant.billing.alerts.updated" => Ok(Self::TenantBillingAlertsUpdated),
            "tenant.billing.info.updated" => Ok(Self::TenantBillingInfoUpdated),
            "tenant.usage_limit.workspace.updated" => Ok(Self::TenantUsageLimitWorkspaceUpdated),
            "tenant.usage_limit.group.updated" => Ok(Self::TenantUsageLimitGroupUpdated),
            "tenant.usage_limit.user.updated" => Ok(Self::TenantUsageLimitUserUpdated),
            "tenant.usage_limit.increase_request.updated" => {
                Ok(Self::TenantUsageLimitIncreaseRequestUpdated)
            }
            "tenant.usage_limit.increase_request.resolved" => {
                Ok(Self::TenantUsageLimitIncreaseRequestResolved)
            }
            "tenant.group.created" => Ok(Self::TenantGroupCreated),
            "tenant.group.updated" => Ok(Self::TenantGroupUpdated),
            "tenant.group.deleted" => Ok(Self::TenantGroupDeleted),
            "tenant.group.member.added" => Ok(Self::TenantGroupMemberAdded),
            "tenant.group.member.removed" => Ok(Self::TenantGroupMemberRemoved),
            "tenant.migration_rollout.status.updated" => {
                Ok(Self::TenantMigrationRolloutStatusUpdated)
            }
            "tenant.migration_rollout.tier.updated" => Ok(Self::TenantMigrationRolloutTierUpdated),
            "tenant.role.metadata.updated" => Ok(Self::TenantRoleMetadataUpdated),
            "tenant.custom_role.created" => Ok(Self::TenantCustomRoleCreated),
            "tenant.custom_role.updated" => Ok(Self::TenantCustomRoleUpdated),
            "tenant.custom_role.deleted" => Ok(Self::TenantCustomRoleDeleted),
            "tenant.role_assignment.created" => Ok(Self::TenantRoleAssignmentCreated),
            "tenant.role_assignment.deleted" => Ok(Self::TenantRoleAssignmentDeleted),
            "tenant.resource_role_assignment.created" => {
                Ok(Self::TenantResourceRoleAssignmentCreated)
            }
            "tenant.resource_role_assignment.deleted" => {
                Ok(Self::TenantResourceRoleAssignmentDeleted)
            }
            "tenant.resource_access.updated" => Ok(Self::TenantResourceAccessUpdated),
            "tenant.resource_access.deleted" => Ok(Self::TenantResourceAccessDeleted),
            "tenant.ads_account.onboarding.redemption" => {
                Ok(Self::TenantAdsAccountOnboardingRedemption)
            }
            "tenant.session_policy.created" => Ok(Self::TenantSessionPolicyCreated),
            "tenant.session_policy.updated" => Ok(Self::TenantSessionPolicyUpdated),
            "tenant.session_policy.deleted" => Ok(Self::TenantSessionPolicyDeleted),
            "tenant.session_revocation.started" => Ok(Self::TenantSessionRevocationStarted),
            "tenant.third_party_app_policy.updated" => Ok(Self::TenantThirdPartyAppPolicyUpdated),
            "tenant.user.added" => Ok(Self::TenantUserAdded),
            "tenant.user.updated" => Ok(Self::TenantUserUpdated),
            "tenant.user.removed" => Ok(Self::TenantUserRemoved),
            "tenant.user.looked_up" => Ok(Self::TenantUserLookedUp),
            "tenant.user.invited" => Ok(Self::TenantUserInvited),
            "tenant.membership.revoked" => Ok(Self::TenantMembershipRevoked),
            "tenant.api_organization_invite.upserted" => {
                Ok(Self::TenantApiOrganizationInviteUpserted)
            }
            "tenant.api_organization_invite.deleted" => {
                Ok(Self::TenantApiOrganizationInviteDeleted)
            }
            "tenant.chatgpt_workspace_invite.upserted" => {
                Ok(Self::TenantChatgptWorkspaceInviteUpserted)
            }
            "tenant.membership.accepted" => Ok(Self::TenantMembershipAccepted),
            "tenant.membership.declined" => Ok(Self::TenantMembershipDeclined),
            "tenant.workspace_invite_email_settings.updated" => {
                Ok(Self::TenantWorkspaceInviteEmailSettingsUpdated)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuditLogEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuditLogEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogExternalKeyRegistered {
    #[doc = "The configuration for the external key."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The ID of the external key configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogExternalKeyRegistered {
    pub fn builder() -> builder::AuditLogExternalKeyRegistered {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogExternalKeyRemoved {
    #[doc = "The ID of the external key configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogExternalKeyRemoved {
    pub fn builder() -> builder::AuditLogExternalKeyRemoved {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogExternalStorageRegistered {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogExternalStorageRegisteredData>,
    #[doc = "The ID of the external storage configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogExternalStorageRegistered {
    pub fn builder() -> builder::AuditLogExternalStorageRegistered {
        Default::default()
    }
}
#[doc = "The configuration for the external storage."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogExternalStorageRegisteredData {
    #[doc = "The OpenAI geography derived from the storage region."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub geography: ::std::option::Option<::std::string::String>,
    #[doc = "The external storage provider configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<Provider>,
}
impl AuditLogExternalStorageRegisteredData {
    pub fn builder() -> builder::AuditLogExternalStorageRegisteredData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogExternalStorageRemoved {
    #[doc = "The ID of the external storage configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogExternalStorageRemoved {
    pub fn builder() -> builder::AuditLogExternalStorageRemoved {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogGroupCreated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogGroupCreatedData>,
    #[doc = "The ID of the group."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogGroupCreated {
    pub fn builder() -> builder::AuditLogGroupCreated {
        Default::default()
    }
}
#[doc = "Information about the created group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogGroupCreatedData {
    #[doc = "The group name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_name: ::std::option::Option<::std::string::String>,
}
impl AuditLogGroupCreatedData {
    pub fn builder() -> builder::AuditLogGroupCreatedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogGroupDeleted {
    #[doc = "The ID of the group."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogGroupDeleted {
    pub fn builder() -> builder::AuditLogGroupDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogGroupUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogGroupUpdatedChangesRequested>,
    #[doc = "The ID of the group."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogGroupUpdated {
    pub fn builder() -> builder::AuditLogGroupUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogGroupUpdatedChangesRequested {
    #[doc = "The updated group name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_name: ::std::option::Option<::std::string::String>,
}
impl AuditLogGroupUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogGroupUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogInviteAccepted {
    #[doc = "The ID of the invite."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogInviteAccepted {
    pub fn builder() -> builder::AuditLogInviteAccepted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogInviteDeleted {
    #[doc = "The ID of the invite."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogInviteDeleted {
    pub fn builder() -> builder::AuditLogInviteDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogInviteSent {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogInviteSentData>,
    #[doc = "The ID of the invite."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogInviteSent {
    pub fn builder() -> builder::AuditLogInviteSent {
        Default::default()
    }
}
#[doc = "The payload used to create the invite."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogInviteSentData {
    #[doc = "The email invited to the organization."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "The role the email was invited to be. Is either `owner` or `member`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl AuditLogInviteSentData {
    pub fn builder() -> builder::AuditLogInviteSentData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistConfigActivated {
    #[doc = "The configurations that were activated."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configs: ::std::vec::Vec<AuditLogIpAllowlistConfigActivatedConfigsItem>,
}
impl AuditLogIpAllowlistConfigActivated {
    pub fn builder() -> builder::AuditLogIpAllowlistConfigActivated {
        Default::default()
    }
}
#[doc = "`AuditLogIpAllowlistConfigActivatedConfigsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistConfigActivatedConfigsItem {
    #[doc = "The ID of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogIpAllowlistConfigActivatedConfigsItem {
    pub fn builder() -> builder::AuditLogIpAllowlistConfigActivatedConfigsItem {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistConfigDeactivated {
    #[doc = "The configurations that were deactivated."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configs: ::std::vec::Vec<AuditLogIpAllowlistConfigDeactivatedConfigsItem>,
}
impl AuditLogIpAllowlistConfigDeactivated {
    pub fn builder() -> builder::AuditLogIpAllowlistConfigDeactivated {
        Default::default()
    }
}
#[doc = "`AuditLogIpAllowlistConfigDeactivatedConfigsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistConfigDeactivatedConfigsItem {
    #[doc = "The ID of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogIpAllowlistConfigDeactivatedConfigsItem {
    pub fn builder() -> builder::AuditLogIpAllowlistConfigDeactivatedConfigsItem {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistCreated {
    #[doc = "The IP addresses or CIDR ranges included in the configuration."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_ips: ::std::vec::Vec<::std::string::String>,
    #[doc = "The ID of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogIpAllowlistCreated {
    pub fn builder() -> builder::AuditLogIpAllowlistCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistDeleted {
    #[doc = "The IP addresses or CIDR ranges that were in the configuration."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_ips: ::std::vec::Vec<::std::string::String>,
    #[doc = "The ID of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogIpAllowlistDeleted {
    pub fn builder() -> builder::AuditLogIpAllowlistDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogIpAllowlistUpdated {
    #[doc = "The updated set of IP addresses or CIDR ranges in the configuration."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_ips: ::std::vec::Vec<::std::string::String>,
    #[doc = "The ID of the IP allowlist configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogIpAllowlistUpdated {
    pub fn builder() -> builder::AuditLogIpAllowlistUpdated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogLoginFailed {
    #[doc = "The error code of the failure."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[doc = "The error message of the failure."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
}
impl AuditLogLoginFailed {
    pub fn builder() -> builder::AuditLogLoginFailed {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogLogoutFailed {
    #[doc = "The error code of the failure."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<::std::string::String>,
    #[doc = "The error message of the failure."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
}
impl AuditLogLogoutFailed {
    pub fn builder() -> builder::AuditLogLogoutFailed {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogOrganizationUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogOrganizationUpdatedChangesRequested>,
    #[doc = "The organization ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogOrganizationUpdated {
    pub fn builder() -> builder::AuditLogOrganizationUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the organization settings."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogOrganizationUpdatedChangesRequested {
    #[doc = "How your organization logs data from supported API calls. One of `disabled`, `enabled_per_call`, `enabled_for_all_projects`, or `enabled_for_selected_projects`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_call_logging: ::std::option::Option<::std::string::String>,
    #[doc = "The list of project ids if api_call_logging is set to `enabled_for_selected_projects`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub api_call_logging_project_ids: ::std::option::Option<::std::string::String>,
    #[doc = "The organization description."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The organization name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub threads_ui_visibility: ::std::option::Option<::std::string::String>,
    #[doc = "The organization title."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage_dashboard_visibility: ::std::option::Option<::std::string::String>,
}
impl AuditLogOrganizationUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogOrganizationUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The project that the action was scoped to. Absent for actions not scoped to projects. Note that any admin actions taken via Admin API keys are associated with the default project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProject {
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The project title."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogProject {
    pub fn builder() -> builder::AuditLogProject {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectArchived {
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectArchived {
    pub fn builder() -> builder::AuditLogProjectArchived {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectCreated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogProjectCreatedData>,
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectCreated {
    pub fn builder() -> builder::AuditLogProjectCreated {
        Default::default()
    }
}
#[doc = "The payload used to create the project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectCreatedData {
    #[doc = "The project name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The title of the project as seen on the dashboard."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectCreatedData {
    pub fn builder() -> builder::AuditLogProjectCreatedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectDeleted {
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectDeleted {
    pub fn builder() -> builder::AuditLogProjectDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogProjectUpdatedChangesRequested>,
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectUpdated {
    pub fn builder() -> builder::AuditLogProjectUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogProjectUpdatedChangesRequested {
    #[doc = "The title of the project as seen on the dashboard."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl AuditLogProjectUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogProjectUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRateLimitDeleted {
    #[doc = "The rate limit ID"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRateLimitDeleted {
    pub fn builder() -> builder::AuditLogRateLimitDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRateLimitUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogRateLimitUpdatedChangesRequested>,
    #[doc = "The rate limit ID"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRateLimitUpdated {
    pub fn builder() -> builder::AuditLogRateLimitUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the rate limits."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRateLimitUpdatedChangesRequested {
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
impl AuditLogRateLimitUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogRateLimitUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleAssignmentCreated {
    #[doc = "The identifier of the role assignment."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The principal (user or group) that received the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub principal_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of principal (user or group) that received the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub principal_type: ::std::option::Option<::std::string::String>,
    #[doc = "The resource the role assignment is scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role assignment is scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleAssignmentCreated {
    pub fn builder() -> builder::AuditLogRoleAssignmentCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleAssignmentDeleted {
    #[doc = "The identifier of the role assignment."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The principal (user or group) that had the role removed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub principal_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of principal (user or group) that had the role removed."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub principal_type: ::std::option::Option<::std::string::String>,
    #[doc = "The resource the role assignment was scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role assignment was scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleAssignmentDeleted {
    pub fn builder() -> builder::AuditLogRoleAssignmentDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleBoundToResource {
    #[doc = "The connector ID for a ChatGPT workspace connector resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_id: ::std::option::Option<::std::string::String>,
    #[doc = "The connector display name for a ChatGPT workspace connector resource, or the connector ID when the display name could not be resolved."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the connector is enabled for the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[doc = "The ID of the resource the role was bound to. ChatGPT workspace connector resources use `<workspace_id>__<connector_id>`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The permissions granted to the role for the resource."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The ID of the resource the role was bound to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role was bound to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the role that was bound to the resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_id: ::std::option::Option<::std::string::String>,
    #[doc = "The connector role mutation path that produced the event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<AuditLogRoleBoundToResourceSource>,
    #[doc = "The workspace ID for a ChatGPT workspace connector resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleBoundToResource {
    pub fn builder() -> builder::AuditLogRoleBoundToResource {
        Default::default()
    }
}
#[doc = "The connector role mutation path that produced the event."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuditLogRoleBoundToResourceSource {
    #[serde(rename = "role_toggle")]
    RoleToggle,
    #[serde(rename = "role_connector_update")]
    RoleConnectorUpdate,
    #[serde(rename = "role_delete")]
    RoleDelete,
    #[serde(rename = "workspace_permissions")]
    WorkspacePermissions,
    #[serde(rename = "connector_publish")]
    ConnectorPublish,
}
impl ::std::fmt::Display for AuditLogRoleBoundToResourceSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RoleToggle => f.write_str("role_toggle"),
            Self::RoleConnectorUpdate => f.write_str("role_connector_update"),
            Self::RoleDelete => f.write_str("role_delete"),
            Self::WorkspacePermissions => f.write_str("workspace_permissions"),
            Self::ConnectorPublish => f.write_str("connector_publish"),
        }
    }
}
impl ::std::str::FromStr for AuditLogRoleBoundToResourceSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "role_toggle" => Ok(Self::RoleToggle),
            "role_connector_update" => Ok(Self::RoleConnectorUpdate),
            "role_delete" => Ok(Self::RoleDelete),
            "workspace_permissions" => Ok(Self::WorkspacePermissions),
            "connector_publish" => Ok(Self::ConnectorPublish),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuditLogRoleBoundToResourceSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuditLogRoleBoundToResourceSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleCreated {
    #[doc = "The role ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The permissions granted by the role."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The resource the role is scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role belongs to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleCreated {
    pub fn builder() -> builder::AuditLogRoleCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleDeleted {
    #[doc = "The role ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleDeleted {
    pub fn builder() -> builder::AuditLogRoleDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleUnboundFromResource {
    #[doc = "The connector ID for a ChatGPT workspace connector resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_id: ::std::option::Option<::std::string::String>,
    #[doc = "The connector display name for a ChatGPT workspace connector resource, or the connector ID when the display name could not be resolved."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub connector_name: ::std::option::Option<::std::string::String>,
    #[doc = "Whether the connector is enabled for the role."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[doc = "The ID of the resource the role was unbound from. ChatGPT workspace connector resources use `<workspace_id>__<connector_id>`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The permissions remaining for the role after the change."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The ID of the resource the role was unbound from."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role was unbound from."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the role that was unbound from the resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_id: ::std::option::Option<::std::string::String>,
    #[doc = "The connector role mutation path that produced the event."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<AuditLogRoleUnboundFromResourceSource>,
    #[doc = "The workspace ID for a ChatGPT workspace connector resource."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleUnboundFromResource {
    pub fn builder() -> builder::AuditLogRoleUnboundFromResource {
        Default::default()
    }
}
#[doc = "The connector role mutation path that produced the event."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AuditLogRoleUnboundFromResourceSource {
    #[serde(rename = "role_toggle")]
    RoleToggle,
    #[serde(rename = "role_connector_update")]
    RoleConnectorUpdate,
    #[serde(rename = "role_delete")]
    RoleDelete,
    #[serde(rename = "workspace_permissions")]
    WorkspacePermissions,
    #[serde(rename = "connector_publish")]
    ConnectorPublish,
}
impl ::std::fmt::Display for AuditLogRoleUnboundFromResourceSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RoleToggle => f.write_str("role_toggle"),
            Self::RoleConnectorUpdate => f.write_str("role_connector_update"),
            Self::RoleDelete => f.write_str("role_delete"),
            Self::WorkspacePermissions => f.write_str("workspace_permissions"),
            Self::ConnectorPublish => f.write_str("connector_publish"),
        }
    }
}
impl ::std::str::FromStr for AuditLogRoleUnboundFromResourceSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "role_toggle" => Ok(Self::RoleToggle),
            "role_connector_update" => Ok(Self::RoleConnectorUpdate),
            "role_delete" => Ok(Self::RoleDelete),
            "workspace_permissions" => Ok(Self::WorkspacePermissions),
            "connector_publish" => Ok(Self::ConnectorPublish),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuditLogRoleUnboundFromResourceSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuditLogRoleUnboundFromResourceSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogRoleUpdatedChangesRequested>,
    #[doc = "The role ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleUpdated {
    pub fn builder() -> builder::AuditLogRoleUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the role."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogRoleUpdatedChangesRequested {
    #[doc = "The updated role description, when provided."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Additional metadata stored on the role."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The permissions added to the role."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions_added: ::std::vec::Vec<::std::string::String>,
    #[doc = "The permissions removed from the role."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions_removed: ::std::vec::Vec<::std::string::String>,
    #[doc = "The resource the role is scoped to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of resource the role belongs to."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<::std::string::String>,
    #[doc = "The updated role name, when provided."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
}
impl AuditLogRoleUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogRoleUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogScimDisabled {
    #[doc = "The ID of the SCIM was disabled for."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogScimDisabled {
    pub fn builder() -> builder::AuditLogScimDisabled {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogScimEnabled {
    #[doc = "The ID of the SCIM was enabled for."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogScimEnabled {
    pub fn builder() -> builder::AuditLogScimEnabled {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogServiceAccountCreated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogServiceAccountCreatedData>,
    #[doc = "The service account ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogServiceAccountCreated {
    pub fn builder() -> builder::AuditLogServiceAccountCreated {
        Default::default()
    }
}
#[doc = "The payload used to create the service account."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogServiceAccountCreatedData {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl AuditLogServiceAccountCreatedData {
    pub fn builder() -> builder::AuditLogServiceAccountCreatedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogServiceAccountDeleted {
    #[doc = "The service account ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogServiceAccountDeleted {
    pub fn builder() -> builder::AuditLogServiceAccountDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogServiceAccountUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogServiceAccountUpdatedChangesRequested>,
    #[doc = "The service account ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogServiceAccountUpdated {
    pub fn builder() -> builder::AuditLogServiceAccountUpdated {
        Default::default()
    }
}
#[doc = "The payload used to updated the service account."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl AuditLogServiceAccountUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogServiceAccountUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogUserAdded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<AuditLogUserAddedData>,
    #[doc = "The user ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogUserAdded {
    pub fn builder() -> builder::AuditLogUserAdded {
        Default::default()
    }
}
#[doc = "The payload used to add the user to the project."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogUserAddedData {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl AuditLogUserAddedData {
    pub fn builder() -> builder::AuditLogUserAddedData {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogUserDeleted {
    #[doc = "The user ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogUserDeleted {
    pub fn builder() -> builder::AuditLogUserDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogUserUpdated {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub changes_requested: ::std::option::Option<AuditLogUserUpdatedChangesRequested>,
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogUserUpdated {
    pub fn builder() -> builder::AuditLogUserUpdated {
        Default::default()
    }
}
#[doc = "The payload used to update the user."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogUserUpdatedChangesRequested {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
}
impl AuditLogUserUpdatedChangesRequested {
    pub fn builder() -> builder::AuditLogUserUpdatedChangesRequested {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderCreated {
    #[doc = "The payload used to create the workload identity provider."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderCreated {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderDeleted {
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The workload identity provider name."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderDeleted {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingCreated {
    #[doc = "The payload used to create the workload identity provider mapping."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The workload identity provider mapping ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub identity_provider_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderMappingCreated {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderMappingCreated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingDeleted {
    #[doc = "The workload identity provider mapping ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub identity_provider_id: ::std::option::Option<::std::string::String>,
    #[doc = "The project ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[doc = "The mapped service account ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_account_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderMappingDeleted {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderMappingDeleted {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderMappingUpdated {
    #[doc = "The payload used to update the workload identity provider mapping."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub changes_requested: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The workload identity provider mapping ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub identity_provider_id: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderMappingUpdated {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderMappingUpdated {
        Default::default()
    }
}
#[doc = "The details for events with this `type`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct AuditLogWorkloadIdentityProviderUpdated {
    #[doc = "The payload used to update the workload identity provider."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub changes_requested: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The workload identity provider ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl AuditLogWorkloadIdentityProviderUpdated {
    pub fn builder() -> builder::AuditLogWorkloadIdentityProviderUpdated {
        Default::default()
    }
}
#[doc = "`AuditLogsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AuditLogsApiSchema {
    #[serde(rename = "ListAuditLogsError400")]
    pub list_audit_logs_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListAuditLogsError404")]
    pub list_audit_logs_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListAuditLogsError429")]
    pub list_audit_logs_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListAuditLogsParams")]
    pub list_audit_logs_params: ListAuditLogsParams,
    #[serde(rename = "ListAuditLogsResponse")]
    pub list_audit_logs_response: ListAuditLogsResponse,
}
impl AuditLogsApiSchema {
    pub fn builder() -> builder::AuditLogsApiSchema {
        Default::default()
    }
}
#[doc = "`ListAuditLogsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListAuditLogsParams {
    #[serde(
        rename = "actor_emails[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub actor_emails: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "actor_ids[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub actor_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub before: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub effective_at: ::std::option::Option<ListAuditLogsParamsEffectiveAt>,
    #[serde(
        rename = "event_types[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub event_types: ::std::vec::Vec<AuditLogEventType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(
        rename = "project_ids[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub project_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "resource_ids[]",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub resource_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant_only: ::std::option::Option<bool>,
}
impl ListAuditLogsParams {
    pub fn builder() -> builder::ListAuditLogsParams {
        Default::default()
    }
}
#[doc = "`ListAuditLogsParamsEffectiveAt`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListAuditLogsParamsEffectiveAt {
    #[doc = "Return only events whose `effective_at` (Unix seconds) is greater than this value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gt: ::std::option::Option<i64>,
    #[doc = "Return only events whose `effective_at` (Unix seconds) is greater than or equal to this value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gte: ::std::option::Option<i64>,
    #[doc = "Return only events whose `effective_at` (Unix seconds) is less than this value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lt: ::std::option::Option<i64>,
    #[doc = "Return only events whose `effective_at` (Unix seconds) is less than or equal to this value."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lte: ::std::option::Option<i64>,
}
impl ListAuditLogsParamsEffectiveAt {
    pub fn builder() -> builder::ListAuditLogsParamsEffectiveAt {
        Default::default()
    }
}
#[doc = "`ListAuditLogsResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListAuditLogsResponse {
    pub data: ::std::vec::Vec<AuditLog>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: ListAuditLogsResponseObject,
}
impl ListAuditLogsResponse {
    pub fn builder() -> builder::ListAuditLogsResponse {
        Default::default()
    }
}
#[doc = "`ListAuditLogsResponseObject`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListAuditLogsResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for ListAuditLogsResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for ListAuditLogsResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListAuditLogsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListAuditLogsResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Provider`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Provider {
    Variant0(::openai_api_types::common::AwsExternalStorageProviderResponse),
    Variant1(::openai_api_types::common::AzureExternalStorageProviderResponse),
    Variant2(::openai_api_types::common::GcpExternalStorageProviderResponse),
    Variant3(::openai_api_types::common::OciExternalStorageProviderResponse),
}
impl ::std::convert::From<::openai_api_types::common::AwsExternalStorageProviderResponse>
    for Provider
{
    fn from(value: ::openai_api_types::common::AwsExternalStorageProviderResponse) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::AzureExternalStorageProviderResponse>
    for Provider
{
    fn from(value: ::openai_api_types::common::AzureExternalStorageProviderResponse) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::GcpExternalStorageProviderResponse>
    for Provider
{
    fn from(value: ::openai_api_types::common::GcpExternalStorageProviderResponse) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<::openai_api_types::common::OciExternalStorageProviderResponse>
    for Provider
{
    fn from(value: ::openai_api_types::common::OciExternalStorageProviderResponse) -> Self {
        Self::Variant3(value)
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AuditLog {
        actor: ::std::result::Result<
            ::std::option::Option<super::AuditLogActor>,
            ::std::string::String,
        >,
        api_key_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogApiKeyCreated>,
            ::std::string::String,
        >,
        api_key_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogApiKeyDeleted>,
            ::std::string::String,
        >,
        api_key_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogApiKeyUpdated>,
            ::std::string::String,
        >,
        certificate_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogCertificateCreated>,
            ::std::string::String,
        >,
        certificate_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogCertificateDeleted>,
            ::std::string::String,
        >,
        certificate_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogCertificateUpdated>,
            ::std::string::String,
        >,
        certificates_activated: ::std::result::Result<
            ::std::option::Option<super::AuditLogCertificatesActivated>,
            ::std::string::String,
        >,
        certificates_deactivated: ::std::result::Result<
            ::std::option::Option<super::AuditLogCertificatesDeactivated>,
            ::std::string::String,
        >,
        checkpoint_permission_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogCheckpointPermissionCreated>,
            ::std::string::String,
        >,
        checkpoint_permission_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogCheckpointPermissionDeleted>,
            ::std::string::String,
        >,
        effective_at: ::std::result::Result<i64, ::std::string::String>,
        external_key_registered: ::std::result::Result<
            ::std::option::Option<super::AuditLogExternalKeyRegistered>,
            ::std::string::String,
        >,
        external_key_removed: ::std::result::Result<
            ::std::option::Option<super::AuditLogExternalKeyRemoved>,
            ::std::string::String,
        >,
        external_storage_registered: ::std::result::Result<
            ::std::option::Option<super::AuditLogExternalStorageRegistered>,
            ::std::string::String,
        >,
        external_storage_removed: ::std::result::Result<
            ::std::option::Option<super::AuditLogExternalStorageRemoved>,
            ::std::string::String,
        >,
        group_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogGroupCreated>,
            ::std::string::String,
        >,
        group_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogGroupDeleted>,
            ::std::string::String,
        >,
        group_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogGroupUpdated>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        invite_accepted: ::std::result::Result<
            ::std::option::Option<super::AuditLogInviteAccepted>,
            ::std::string::String,
        >,
        invite_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogInviteDeleted>,
            ::std::string::String,
        >,
        invite_sent: ::std::result::Result<
            ::std::option::Option<super::AuditLogInviteSent>,
            ::std::string::String,
        >,
        ip_allowlist_config_activated: ::std::result::Result<
            ::std::option::Option<super::AuditLogIpAllowlistConfigActivated>,
            ::std::string::String,
        >,
        ip_allowlist_config_deactivated: ::std::result::Result<
            ::std::option::Option<super::AuditLogIpAllowlistConfigDeactivated>,
            ::std::string::String,
        >,
        ip_allowlist_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogIpAllowlistCreated>,
            ::std::string::String,
        >,
        ip_allowlist_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogIpAllowlistDeleted>,
            ::std::string::String,
        >,
        ip_allowlist_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogIpAllowlistUpdated>,
            ::std::string::String,
        >,
        login_failed: ::std::result::Result<
            ::std::option::Option<super::AuditLogLoginFailed>,
            ::std::string::String,
        >,
        login_succeeded: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        logout_failed: ::std::result::Result<
            ::std::option::Option<super::AuditLogLogoutFailed>,
            ::std::string::String,
        >,
        logout_succeeded: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        organization_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogOrganizationUpdated>,
            ::std::string::String,
        >,
        project: ::std::result::Result<
            ::std::option::Option<super::AuditLogProject>,
            ::std::string::String,
        >,
        project_archived: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectArchived>,
            ::std::string::String,
        >,
        project_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectCreated>,
            ::std::string::String,
        >,
        project_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectDeleted>,
            ::std::string::String,
        >,
        project_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectUpdated>,
            ::std::string::String,
        >,
        rate_limit_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogRateLimitDeleted>,
            ::std::string::String,
        >,
        rate_limit_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogRateLimitUpdated>,
            ::std::string::String,
        >,
        role_assignment_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleAssignmentCreated>,
            ::std::string::String,
        >,
        role_assignment_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleAssignmentDeleted>,
            ::std::string::String,
        >,
        role_bound_to_resource: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleBoundToResource>,
            ::std::string::String,
        >,
        role_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleCreated>,
            ::std::string::String,
        >,
        role_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleDeleted>,
            ::std::string::String,
        >,
        role_unbound_from_resource: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleUnboundFromResource>,
            ::std::string::String,
        >,
        role_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleUpdated>,
            ::std::string::String,
        >,
        scim_disabled: ::std::result::Result<
            ::std::option::Option<super::AuditLogScimDisabled>,
            ::std::string::String,
        >,
        scim_enabled: ::std::result::Result<
            ::std::option::Option<super::AuditLogScimEnabled>,
            ::std::string::String,
        >,
        service_account_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogServiceAccountCreated>,
            ::std::string::String,
        >,
        service_account_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogServiceAccountDeleted>,
            ::std::string::String,
        >,
        service_account_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogServiceAccountUpdated>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AuditLogEventType, ::std::string::String>,
        user_added: ::std::result::Result<
            ::std::option::Option<super::AuditLogUserAdded>,
            ::std::string::String,
        >,
        user_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogUserDeleted>,
            ::std::string::String,
        >,
        user_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogUserUpdated>,
            ::std::string::String,
        >,
        workload_identity_provider_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderCreated>,
            ::std::string::String,
        >,
        workload_identity_provider_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderDeleted>,
            ::std::string::String,
        >,
        workload_identity_provider_mapping_created: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingCreated>,
            ::std::string::String,
        >,
        workload_identity_provider_mapping_deleted: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingDeleted>,
            ::std::string::String,
        >,
        workload_identity_provider_mapping_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingUpdated>,
            ::std::string::String,
        >,
        workload_identity_provider_updated: ::std::result::Result<
            ::std::option::Option<super::AuditLogWorkloadIdentityProviderUpdated>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLog {
        fn default() -> Self {
            Self {
                actor: Ok(Default::default()),
                api_key_created: Ok(Default::default()),
                api_key_deleted: Ok(Default::default()),
                api_key_updated: Ok(Default::default()),
                certificate_created: Ok(Default::default()),
                certificate_deleted: Ok(Default::default()),
                certificate_updated: Ok(Default::default()),
                certificates_activated: Ok(Default::default()),
                certificates_deactivated: Ok(Default::default()),
                checkpoint_permission_created: Ok(Default::default()),
                checkpoint_permission_deleted: Ok(Default::default()),
                effective_at: Err("no value supplied for effective_at".to_string()),
                external_key_registered: Ok(Default::default()),
                external_key_removed: Ok(Default::default()),
                external_storage_registered: Ok(Default::default()),
                external_storage_removed: Ok(Default::default()),
                group_created: Ok(Default::default()),
                group_deleted: Ok(Default::default()),
                group_updated: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                invite_accepted: Ok(Default::default()),
                invite_deleted: Ok(Default::default()),
                invite_sent: Ok(Default::default()),
                ip_allowlist_config_activated: Ok(Default::default()),
                ip_allowlist_config_deactivated: Ok(Default::default()),
                ip_allowlist_created: Ok(Default::default()),
                ip_allowlist_deleted: Ok(Default::default()),
                ip_allowlist_updated: Ok(Default::default()),
                login_failed: Ok(Default::default()),
                login_succeeded: Ok(Default::default()),
                logout_failed: Ok(Default::default()),
                logout_succeeded: Ok(Default::default()),
                organization_updated: Ok(Default::default()),
                project: Ok(Default::default()),
                project_archived: Ok(Default::default()),
                project_created: Ok(Default::default()),
                project_deleted: Ok(Default::default()),
                project_updated: Ok(Default::default()),
                rate_limit_deleted: Ok(Default::default()),
                rate_limit_updated: Ok(Default::default()),
                role_assignment_created: Ok(Default::default()),
                role_assignment_deleted: Ok(Default::default()),
                role_bound_to_resource: Ok(Default::default()),
                role_created: Ok(Default::default()),
                role_deleted: Ok(Default::default()),
                role_unbound_from_resource: Ok(Default::default()),
                role_updated: Ok(Default::default()),
                scim_disabled: Ok(Default::default()),
                scim_enabled: Ok(Default::default()),
                service_account_created: Ok(Default::default()),
                service_account_deleted: Ok(Default::default()),
                service_account_updated: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                user_added: Ok(Default::default()),
                user_deleted: Ok(Default::default()),
                user_updated: Ok(Default::default()),
                workload_identity_provider_created: Ok(Default::default()),
                workload_identity_provider_deleted: Ok(Default::default()),
                workload_identity_provider_mapping_created: Ok(Default::default()),
                workload_identity_provider_mapping_deleted: Ok(Default::default()),
                workload_identity_provider_mapping_updated: Ok(Default::default()),
                workload_identity_provider_updated: Ok(Default::default()),
            }
        }
    }
    impl AuditLog {
        pub fn actor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActor>>,
            T::Error: ::std::fmt::Display,
        {
            self.actor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actor: {e}"));
            self
        }
        pub fn api_key_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogApiKeyCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_created: {e}"));
            self
        }
        pub fn api_key_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogApiKeyDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_deleted: {e}"));
            self
        }
        pub fn api_key_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogApiKeyUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key_updated: {e}"));
            self
        }
        pub fn certificate_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogCertificateCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_created: {e}")
            });
            self
        }
        pub fn certificate_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogCertificateDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_deleted: {e}")
            });
            self
        }
        pub fn certificate_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogCertificateUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate_updated = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificate_updated: {e}")
            });
            self
        }
        pub fn certificates_activated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogCertificatesActivated>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificates_activated = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificates_activated: {e}")
            });
            self
        }
        pub fn certificates_deactivated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogCertificatesDeactivated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.certificates_deactivated = value.try_into().map_err(|e| {
                format!("error converting supplied value for certificates_deactivated: {e}")
            });
            self
        }
        pub fn checkpoint_permission_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogCheckpointPermissionCreated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.checkpoint_permission_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for checkpoint_permission_created: {e}")
            });
            self
        }
        pub fn checkpoint_permission_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogCheckpointPermissionDeleted>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.checkpoint_permission_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for checkpoint_permission_deleted: {e}")
            });
            self
        }
        pub fn effective_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.effective_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effective_at: {e}"));
            self
        }
        pub fn external_key_registered<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogExternalKeyRegistered>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_key_registered = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_key_registered: {e}")
            });
            self
        }
        pub fn external_key_removed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogExternalKeyRemoved>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_key_removed = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_key_removed: {e}")
            });
            self
        }
        pub fn external_storage_registered<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogExternalStorageRegistered>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_storage_registered = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_storage_registered: {e}")
            });
            self
        }
        pub fn external_storage_removed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogExternalStorageRemoved>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_storage_removed = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_storage_removed: {e}")
            });
            self
        }
        pub fn group_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogGroupCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_created: {e}"));
            self
        }
        pub fn group_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogGroupDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_deleted: {e}"));
            self
        }
        pub fn group_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogGroupUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_updated: {e}"));
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
        pub fn invite_accepted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogInviteAccepted>>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_accepted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invite_accepted: {e}"));
            self
        }
        pub fn invite_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogInviteDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invite_deleted: {e}"));
            self
        }
        pub fn invite_sent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogInviteSent>>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_sent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invite_sent: {e}"));
            self
        }
        pub fn ip_allowlist_config_activated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogIpAllowlistConfigActivated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ip_allowlist_config_activated = value.try_into().map_err(|e| {
                format!("error converting supplied value for ip_allowlist_config_activated: {e}")
            });
            self
        }
        pub fn ip_allowlist_config_deactivated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogIpAllowlistConfigDeactivated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ip_allowlist_config_deactivated = value.try_into().map_err(|e| {
                format!("error converting supplied value for ip_allowlist_config_deactivated: {e}")
            });
            self
        }
        pub fn ip_allowlist_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogIpAllowlistCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.ip_allowlist_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for ip_allowlist_created: {e}")
            });
            self
        }
        pub fn ip_allowlist_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogIpAllowlistDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.ip_allowlist_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for ip_allowlist_deleted: {e}")
            });
            self
        }
        pub fn ip_allowlist_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogIpAllowlistUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.ip_allowlist_updated = value.try_into().map_err(|e| {
                format!("error converting supplied value for ip_allowlist_updated: {e}")
            });
            self
        }
        pub fn login_failed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogLoginFailed>>,
            T::Error: ::std::fmt::Display,
        {
            self.login_failed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for login_failed: {e}"));
            self
        }
        pub fn login_succeeded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.login_succeeded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for login_succeeded: {e}"));
            self
        }
        pub fn logout_failed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogLogoutFailed>>,
            T::Error: ::std::fmt::Display,
        {
            self.logout_failed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logout_failed: {e}"));
            self
        }
        pub fn logout_succeeded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.logout_succeeded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for logout_succeeded: {e}"));
            self
        }
        pub fn organization_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogOrganizationUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.organization_updated = value.try_into().map_err(|e| {
                format!("error converting supplied value for organization_updated: {e}")
            });
            self
        }
        pub fn project<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProject>>,
            T::Error: ::std::fmt::Display,
        {
            self.project = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project: {e}"));
            self
        }
        pub fn project_archived<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProjectArchived>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_archived = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_archived: {e}"));
            self
        }
        pub fn project_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProjectCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_created: {e}"));
            self
        }
        pub fn project_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProjectDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_deleted: {e}"));
            self
        }
        pub fn project_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProjectUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.project_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for project_updated: {e}"));
            self
        }
        pub fn rate_limit_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRateLimitDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate_limit_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for rate_limit_deleted: {e}")
            });
            self
        }
        pub fn rate_limit_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRateLimitUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.rate_limit_updated = value.try_into().map_err(|e| {
                format!("error converting supplied value for rate_limit_updated: {e}")
            });
            self
        }
        pub fn role_assignment_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleAssignmentCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_assignment_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for role_assignment_created: {e}")
            });
            self
        }
        pub fn role_assignment_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleAssignmentDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_assignment_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for role_assignment_deleted: {e}")
            });
            self
        }
        pub fn role_bound_to_resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleBoundToResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_bound_to_resource = value.try_into().map_err(|e| {
                format!("error converting supplied value for role_bound_to_resource: {e}")
            });
            self
        }
        pub fn role_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_created: {e}"));
            self
        }
        pub fn role_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_deleted: {e}"));
            self
        }
        pub fn role_unbound_from_resource<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogRoleUnboundFromResource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.role_unbound_from_resource = value.try_into().map_err(|e| {
                format!("error converting supplied value for role_unbound_from_resource: {e}")
            });
            self
        }
        pub fn role_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogRoleUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_updated: {e}"));
            self
        }
        pub fn scim_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogScimDisabled>>,
            T::Error: ::std::fmt::Display,
        {
            self.scim_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scim_disabled: {e}"));
            self
        }
        pub fn scim_enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogScimEnabled>>,
            T::Error: ::std::fmt::Display,
        {
            self.scim_enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scim_enabled: {e}"));
            self
        }
        pub fn service_account_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogServiceAccountCreated>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_created = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_created: {e}")
            });
            self
        }
        pub fn service_account_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogServiceAccountDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_deleted = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_deleted: {e}")
            });
            self
        }
        pub fn service_account_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogServiceAccountUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_updated = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_updated: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AuditLogEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn user_added<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogUserAdded>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_added = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_added: {e}"));
            self
        }
        pub fn user_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogUserDeleted>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_deleted: {e}"));
            self
        }
        pub fn user_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogUserUpdated>>,
            T::Error: ::std::fmt::Display,
        {
            self.user_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_updated: {e}"));
            self
        }
        pub fn workload_identity_provider_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderCreated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_provider_created = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for workload_identity_provider_created: {e}"
                )
            });
            self
        }
        pub fn workload_identity_provider_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderDeleted>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_provider_deleted = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for workload_identity_provider_deleted: {e}"
                )
            });
            self
        }
        pub fn workload_identity_provider_mapping_created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingCreated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . workload_identity_provider_mapping_created = value . try_into () . map_err (| e | format ! ("error converting supplied value for workload_identity_provider_mapping_created: {e}")) ;
            self
        }
        pub fn workload_identity_provider_mapping_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingDeleted>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . workload_identity_provider_mapping_deleted = value . try_into () . map_err (| e | format ! ("error converting supplied value for workload_identity_provider_mapping_deleted: {e}")) ;
            self
        }
        pub fn workload_identity_provider_mapping_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderMappingUpdated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . workload_identity_provider_mapping_updated = value . try_into () . map_err (| e | format ! ("error converting supplied value for workload_identity_provider_mapping_updated: {e}")) ;
            self
        }
        pub fn workload_identity_provider_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogWorkloadIdentityProviderUpdated>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.workload_identity_provider_updated = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for workload_identity_provider_updated: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLog> for super::AuditLog {
        type Error = super::error::ConversionError;
        fn try_from(value: AuditLog) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actor: value.actor?,
                api_key_created: value.api_key_created?,
                api_key_deleted: value.api_key_deleted?,
                api_key_updated: value.api_key_updated?,
                certificate_created: value.certificate_created?,
                certificate_deleted: value.certificate_deleted?,
                certificate_updated: value.certificate_updated?,
                certificates_activated: value.certificates_activated?,
                certificates_deactivated: value.certificates_deactivated?,
                checkpoint_permission_created: value.checkpoint_permission_created?,
                checkpoint_permission_deleted: value.checkpoint_permission_deleted?,
                effective_at: value.effective_at?,
                external_key_registered: value.external_key_registered?,
                external_key_removed: value.external_key_removed?,
                external_storage_registered: value.external_storage_registered?,
                external_storage_removed: value.external_storage_removed?,
                group_created: value.group_created?,
                group_deleted: value.group_deleted?,
                group_updated: value.group_updated?,
                id: value.id?,
                invite_accepted: value.invite_accepted?,
                invite_deleted: value.invite_deleted?,
                invite_sent: value.invite_sent?,
                ip_allowlist_config_activated: value.ip_allowlist_config_activated?,
                ip_allowlist_config_deactivated: value.ip_allowlist_config_deactivated?,
                ip_allowlist_created: value.ip_allowlist_created?,
                ip_allowlist_deleted: value.ip_allowlist_deleted?,
                ip_allowlist_updated: value.ip_allowlist_updated?,
                login_failed: value.login_failed?,
                login_succeeded: value.login_succeeded?,
                logout_failed: value.logout_failed?,
                logout_succeeded: value.logout_succeeded?,
                organization_updated: value.organization_updated?,
                project: value.project?,
                project_archived: value.project_archived?,
                project_created: value.project_created?,
                project_deleted: value.project_deleted?,
                project_updated: value.project_updated?,
                rate_limit_deleted: value.rate_limit_deleted?,
                rate_limit_updated: value.rate_limit_updated?,
                role_assignment_created: value.role_assignment_created?,
                role_assignment_deleted: value.role_assignment_deleted?,
                role_bound_to_resource: value.role_bound_to_resource?,
                role_created: value.role_created?,
                role_deleted: value.role_deleted?,
                role_unbound_from_resource: value.role_unbound_from_resource?,
                role_updated: value.role_updated?,
                scim_disabled: value.scim_disabled?,
                scim_enabled: value.scim_enabled?,
                service_account_created: value.service_account_created?,
                service_account_deleted: value.service_account_deleted?,
                service_account_updated: value.service_account_updated?,
                type_: value.type_?,
                user_added: value.user_added?,
                user_deleted: value.user_deleted?,
                user_updated: value.user_updated?,
                workload_identity_provider_created: value.workload_identity_provider_created?,
                workload_identity_provider_deleted: value.workload_identity_provider_deleted?,
                workload_identity_provider_mapping_created: value
                    .workload_identity_provider_mapping_created?,
                workload_identity_provider_mapping_deleted: value
                    .workload_identity_provider_mapping_deleted?,
                workload_identity_provider_mapping_updated: value
                    .workload_identity_provider_mapping_updated?,
                workload_identity_provider_updated: value.workload_identity_provider_updated?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLog> for AuditLog {
        fn from(value: super::AuditLog) -> Self {
            Self {
                actor: Ok(value.actor),
                api_key_created: Ok(value.api_key_created),
                api_key_deleted: Ok(value.api_key_deleted),
                api_key_updated: Ok(value.api_key_updated),
                certificate_created: Ok(value.certificate_created),
                certificate_deleted: Ok(value.certificate_deleted),
                certificate_updated: Ok(value.certificate_updated),
                certificates_activated: Ok(value.certificates_activated),
                certificates_deactivated: Ok(value.certificates_deactivated),
                checkpoint_permission_created: Ok(value.checkpoint_permission_created),
                checkpoint_permission_deleted: Ok(value.checkpoint_permission_deleted),
                effective_at: Ok(value.effective_at),
                external_key_registered: Ok(value.external_key_registered),
                external_key_removed: Ok(value.external_key_removed),
                external_storage_registered: Ok(value.external_storage_registered),
                external_storage_removed: Ok(value.external_storage_removed),
                group_created: Ok(value.group_created),
                group_deleted: Ok(value.group_deleted),
                group_updated: Ok(value.group_updated),
                id: Ok(value.id),
                invite_accepted: Ok(value.invite_accepted),
                invite_deleted: Ok(value.invite_deleted),
                invite_sent: Ok(value.invite_sent),
                ip_allowlist_config_activated: Ok(value.ip_allowlist_config_activated),
                ip_allowlist_config_deactivated: Ok(value.ip_allowlist_config_deactivated),
                ip_allowlist_created: Ok(value.ip_allowlist_created),
                ip_allowlist_deleted: Ok(value.ip_allowlist_deleted),
                ip_allowlist_updated: Ok(value.ip_allowlist_updated),
                login_failed: Ok(value.login_failed),
                login_succeeded: Ok(value.login_succeeded),
                logout_failed: Ok(value.logout_failed),
                logout_succeeded: Ok(value.logout_succeeded),
                organization_updated: Ok(value.organization_updated),
                project: Ok(value.project),
                project_archived: Ok(value.project_archived),
                project_created: Ok(value.project_created),
                project_deleted: Ok(value.project_deleted),
                project_updated: Ok(value.project_updated),
                rate_limit_deleted: Ok(value.rate_limit_deleted),
                rate_limit_updated: Ok(value.rate_limit_updated),
                role_assignment_created: Ok(value.role_assignment_created),
                role_assignment_deleted: Ok(value.role_assignment_deleted),
                role_bound_to_resource: Ok(value.role_bound_to_resource),
                role_created: Ok(value.role_created),
                role_deleted: Ok(value.role_deleted),
                role_unbound_from_resource: Ok(value.role_unbound_from_resource),
                role_updated: Ok(value.role_updated),
                scim_disabled: Ok(value.scim_disabled),
                scim_enabled: Ok(value.scim_enabled),
                service_account_created: Ok(value.service_account_created),
                service_account_deleted: Ok(value.service_account_deleted),
                service_account_updated: Ok(value.service_account_updated),
                type_: Ok(value.type_),
                user_added: Ok(value.user_added),
                user_deleted: Ok(value.user_deleted),
                user_updated: Ok(value.user_updated),
                workload_identity_provider_created: Ok(value.workload_identity_provider_created),
                workload_identity_provider_deleted: Ok(value.workload_identity_provider_deleted),
                workload_identity_provider_mapping_created: Ok(
                    value.workload_identity_provider_mapping_created
                ),
                workload_identity_provider_mapping_deleted: Ok(
                    value.workload_identity_provider_mapping_deleted
                ),
                workload_identity_provider_mapping_updated: Ok(
                    value.workload_identity_provider_mapping_updated
                ),
                workload_identity_provider_updated: Ok(value.workload_identity_provider_updated),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogActor {
        api_key: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorApiKey>,
            ::std::string::String,
        >,
        session: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorSession>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogActor {
        fn default() -> Self {
            Self {
                api_key: Ok(Default::default()),
                session: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl AuditLogActor {
        pub fn api_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorApiKey>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_key: {e}"));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorSession>>,
            T::Error: ::std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogActor> for super::AuditLogActor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogActor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key: value.api_key?,
                session: value.session?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogActor> for AuditLogActor {
        fn from(value: super::AuditLogActor) -> Self {
            Self {
                api_key: Ok(value.api_key),
                session: Ok(value.session),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogActorApiKey {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        service_account: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorServiceAccount>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorApiKeyType>,
            ::std::string::String,
        >,
        user: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogActorApiKey {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                service_account: Ok(Default::default()),
                type_: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl AuditLogActorApiKey {
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
        pub fn service_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorServiceAccount>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_account: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorApiKeyType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogActorApiKey> for super::AuditLogActorApiKey {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogActorApiKey,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                service_account: value.service_account?,
                type_: value.type_?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogActorApiKey> for AuditLogActorApiKey {
        fn from(value: super::AuditLogActorApiKey) -> Self {
            Self {
                id: Ok(value.id),
                service_account: Ok(value.service_account),
                type_: Ok(value.type_),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogActorServiceAccount {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogActorServiceAccount {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogActorServiceAccount {
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
    }
    impl ::std::convert::TryFrom<AuditLogActorServiceAccount> for super::AuditLogActorServiceAccount {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogActorServiceAccount,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogActorServiceAccount> for AuditLogActorServiceAccount {
        fn from(value: super::AuditLogActorServiceAccount) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogActorSession {
        ip_address: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user: ::std::result::Result<
            ::std::option::Option<super::AuditLogActorUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogActorSession {
        fn default() -> Self {
            Self {
                ip_address: Ok(Default::default()),
                user: Ok(Default::default()),
            }
        }
    }
    impl AuditLogActorSession {
        pub fn ip_address<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ip_address = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ip_address: {e}"));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogActorUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogActorSession> for super::AuditLogActorSession {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogActorSession,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ip_address: value.ip_address?,
                user: value.user?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogActorSession> for AuditLogActorSession {
        fn from(value: super::AuditLogActorSession) -> Self {
            Self {
                ip_address: Ok(value.ip_address),
                user: Ok(value.user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogActorUser {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogActorUser {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogActorUser {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogActorUser> for super::AuditLogActorUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogActorUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogActorUser> for AuditLogActorUser {
        fn from(value: super::AuditLogActorUser) -> Self {
            Self {
                email: Ok(value.email),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogApiKeyCreated {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogApiKeyCreatedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogApiKeyCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogApiKeyCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogApiKeyCreatedData>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogApiKeyCreated> for super::AuditLogApiKeyCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogApiKeyCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogApiKeyCreated> for AuditLogApiKeyCreated {
        fn from(value: super::AuditLogApiKeyCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogApiKeyCreatedData {
        scopes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AuditLogApiKeyCreatedData {
        fn default() -> Self {
            Self {
                scopes: Ok(Default::default()),
            }
        }
    }
    impl AuditLogApiKeyCreatedData {
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
    impl ::std::convert::TryFrom<AuditLogApiKeyCreatedData> for super::AuditLogApiKeyCreatedData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogApiKeyCreatedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                scopes: value.scopes?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogApiKeyCreatedData> for AuditLogApiKeyCreatedData {
        fn from(value: super::AuditLogApiKeyCreatedData) -> Self {
            Self {
                scopes: Ok(value.scopes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogApiKeyDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogApiKeyDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogApiKeyDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogApiKeyDeleted> for super::AuditLogApiKeyDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogApiKeyDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogApiKeyDeleted> for AuditLogApiKeyDeleted {
        fn from(value: super::AuditLogApiKeyDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogApiKeyUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogApiKeyUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogApiKeyUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogApiKeyUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogApiKeyUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogApiKeyUpdated> for super::AuditLogApiKeyUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogApiKeyUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogApiKeyUpdated> for AuditLogApiKeyUpdated {
        fn from(value: super::AuditLogApiKeyUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogApiKeyUpdatedChangesRequested {
        scopes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AuditLogApiKeyUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                scopes: Ok(Default::default()),
            }
        }
    }
    impl AuditLogApiKeyUpdatedChangesRequested {
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
    impl ::std::convert::TryFrom<AuditLogApiKeyUpdatedChangesRequested>
        for super::AuditLogApiKeyUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogApiKeyUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                scopes: value.scopes?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogApiKeyUpdatedChangesRequested>
        for AuditLogApiKeyUpdatedChangesRequested
    {
        fn from(value: super::AuditLogApiKeyUpdatedChangesRequested) -> Self {
            Self {
                scopes: Ok(value.scopes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificateCreated {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificateCreated {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificateCreated {
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
    }
    impl ::std::convert::TryFrom<AuditLogCertificateCreated> for super::AuditLogCertificateCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificateCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificateCreated> for AuditLogCertificateCreated {
        fn from(value: super::AuditLogCertificateCreated) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificateDeleted {
        certificate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificateDeleted {
        fn default() -> Self {
            Self {
                certificate: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificateDeleted {
        pub fn certificate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.certificate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificate: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogCertificateDeleted> for super::AuditLogCertificateDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificateDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificate: value.certificate?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificateDeleted> for AuditLogCertificateDeleted {
        fn from(value: super::AuditLogCertificateDeleted) -> Self {
            Self {
                certificate: Ok(value.certificate),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificateUpdated {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificateUpdated {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificateUpdated {
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
    }
    impl ::std::convert::TryFrom<AuditLogCertificateUpdated> for super::AuditLogCertificateUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificateUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificateUpdated> for AuditLogCertificateUpdated {
        fn from(value: super::AuditLogCertificateUpdated) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificatesActivated {
        certificates: ::std::result::Result<
            ::std::vec::Vec<super::AuditLogCertificatesActivatedCertificatesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificatesActivated {
        fn default() -> Self {
            Self {
                certificates: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificatesActivated {
        pub fn certificates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::AuditLogCertificatesActivatedCertificatesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.certificates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificates: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogCertificatesActivated>
        for super::AuditLogCertificatesActivated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificatesActivated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificates: value.certificates?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificatesActivated> for AuditLogCertificatesActivated {
        fn from(value: super::AuditLogCertificatesActivated) -> Self {
            Self {
                certificates: Ok(value.certificates),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificatesActivatedCertificatesItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificatesActivatedCertificatesItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificatesActivatedCertificatesItem {
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
    }
    impl ::std::convert::TryFrom<AuditLogCertificatesActivatedCertificatesItem>
        for super::AuditLogCertificatesActivatedCertificatesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificatesActivatedCertificatesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificatesActivatedCertificatesItem>
        for AuditLogCertificatesActivatedCertificatesItem
    {
        fn from(value: super::AuditLogCertificatesActivatedCertificatesItem) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificatesDeactivated {
        certificates: ::std::result::Result<
            ::std::vec::Vec<super::AuditLogCertificatesDeactivatedCertificatesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificatesDeactivated {
        fn default() -> Self {
            Self {
                certificates: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificatesDeactivated {
        pub fn certificates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::AuditLogCertificatesDeactivatedCertificatesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.certificates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for certificates: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogCertificatesDeactivated>
        for super::AuditLogCertificatesDeactivated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificatesDeactivated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                certificates: value.certificates?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificatesDeactivated>
        for AuditLogCertificatesDeactivated
    {
        fn from(value: super::AuditLogCertificatesDeactivated) -> Self {
            Self {
                certificates: Ok(value.certificates),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCertificatesDeactivatedCertificatesItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCertificatesDeactivatedCertificatesItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCertificatesDeactivatedCertificatesItem {
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
    }
    impl ::std::convert::TryFrom<AuditLogCertificatesDeactivatedCertificatesItem>
        for super::AuditLogCertificatesDeactivatedCertificatesItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCertificatesDeactivatedCertificatesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCertificatesDeactivatedCertificatesItem>
        for AuditLogCertificatesDeactivatedCertificatesItem
    {
        fn from(value: super::AuditLogCertificatesDeactivatedCertificatesItem) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCheckpointPermissionCreated {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogCheckpointPermissionCreatedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCheckpointPermissionCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCheckpointPermissionCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogCheckpointPermissionCreatedData>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogCheckpointPermissionCreated>
        for super::AuditLogCheckpointPermissionCreated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCheckpointPermissionCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCheckpointPermissionCreated>
        for AuditLogCheckpointPermissionCreated
    {
        fn from(value: super::AuditLogCheckpointPermissionCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCheckpointPermissionCreatedData {
        fine_tuned_model_checkpoint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCheckpointPermissionCreatedData {
        fn default() -> Self {
            Self {
                fine_tuned_model_checkpoint: Ok(Default::default()),
                project_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCheckpointPermissionCreatedData {
        pub fn fine_tuned_model_checkpoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fine_tuned_model_checkpoint = value.try_into().map_err(|e| {
                format!("error converting supplied value for fine_tuned_model_checkpoint: {e}")
            });
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
    impl ::std::convert::TryFrom<AuditLogCheckpointPermissionCreatedData>
        for super::AuditLogCheckpointPermissionCreatedData
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCheckpointPermissionCreatedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fine_tuned_model_checkpoint: value.fine_tuned_model_checkpoint?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogCheckpointPermissionCreatedData>
        for AuditLogCheckpointPermissionCreatedData
    {
        fn from(value: super::AuditLogCheckpointPermissionCreatedData) -> Self {
            Self {
                fine_tuned_model_checkpoint: Ok(value.fine_tuned_model_checkpoint),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogCheckpointPermissionDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogCheckpointPermissionDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogCheckpointPermissionDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogCheckpointPermissionDeleted>
        for super::AuditLogCheckpointPermissionDeleted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogCheckpointPermissionDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogCheckpointPermissionDeleted>
        for AuditLogCheckpointPermissionDeleted
    {
        fn from(value: super::AuditLogCheckpointPermissionDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogExternalKeyRegistered {
        data: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogExternalKeyRegistered {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogExternalKeyRegistered {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogExternalKeyRegistered>
        for super::AuditLogExternalKeyRegistered
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogExternalKeyRegistered,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogExternalKeyRegistered> for AuditLogExternalKeyRegistered {
        fn from(value: super::AuditLogExternalKeyRegistered) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogExternalKeyRemoved {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogExternalKeyRemoved {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogExternalKeyRemoved {
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
    }
    impl ::std::convert::TryFrom<AuditLogExternalKeyRemoved> for super::AuditLogExternalKeyRemoved {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogExternalKeyRemoved,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogExternalKeyRemoved> for AuditLogExternalKeyRemoved {
        fn from(value: super::AuditLogExternalKeyRemoved) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogExternalStorageRegistered {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogExternalStorageRegisteredData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogExternalStorageRegistered {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogExternalStorageRegistered {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogExternalStorageRegisteredData>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogExternalStorageRegistered>
        for super::AuditLogExternalStorageRegistered
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogExternalStorageRegistered,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogExternalStorageRegistered>
        for AuditLogExternalStorageRegistered
    {
        fn from(value: super::AuditLogExternalStorageRegistered) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogExternalStorageRegisteredData {
        geography: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        provider:
            ::std::result::Result<::std::option::Option<super::Provider>, ::std::string::String>,
    }
    impl ::std::default::Default for AuditLogExternalStorageRegisteredData {
        fn default() -> Self {
            Self {
                geography: Ok(Default::default()),
                provider: Ok(Default::default()),
            }
        }
    }
    impl AuditLogExternalStorageRegisteredData {
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
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Provider>>,
            T::Error: ::std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogExternalStorageRegisteredData>
        for super::AuditLogExternalStorageRegisteredData
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogExternalStorageRegisteredData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                geography: value.geography?,
                provider: value.provider?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogExternalStorageRegisteredData>
        for AuditLogExternalStorageRegisteredData
    {
        fn from(value: super::AuditLogExternalStorageRegisteredData) -> Self {
            Self {
                geography: Ok(value.geography),
                provider: Ok(value.provider),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogExternalStorageRemoved {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogExternalStorageRemoved {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogExternalStorageRemoved {
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
    }
    impl ::std::convert::TryFrom<AuditLogExternalStorageRemoved>
        for super::AuditLogExternalStorageRemoved
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogExternalStorageRemoved,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogExternalStorageRemoved>
        for AuditLogExternalStorageRemoved
    {
        fn from(value: super::AuditLogExternalStorageRemoved) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogGroupCreated {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogGroupCreatedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogGroupCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogGroupCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogGroupCreatedData>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogGroupCreated> for super::AuditLogGroupCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogGroupCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogGroupCreated> for AuditLogGroupCreated {
        fn from(value: super::AuditLogGroupCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogGroupCreatedData {
        group_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogGroupCreatedData {
        fn default() -> Self {
            Self {
                group_name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogGroupCreatedData {
        pub fn group_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogGroupCreatedData> for super::AuditLogGroupCreatedData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogGroupCreatedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_name: value.group_name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogGroupCreatedData> for AuditLogGroupCreatedData {
        fn from(value: super::AuditLogGroupCreatedData) -> Self {
            Self {
                group_name: Ok(value.group_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogGroupDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogGroupDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogGroupDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogGroupDeleted> for super::AuditLogGroupDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogGroupDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogGroupDeleted> for AuditLogGroupDeleted {
        fn from(value: super::AuditLogGroupDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogGroupUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogGroupUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogGroupUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogGroupUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogGroupUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogGroupUpdated> for super::AuditLogGroupUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogGroupUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogGroupUpdated> for AuditLogGroupUpdated {
        fn from(value: super::AuditLogGroupUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogGroupUpdatedChangesRequested {
        group_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogGroupUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                group_name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogGroupUpdatedChangesRequested {
        pub fn group_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogGroupUpdatedChangesRequested>
        for super::AuditLogGroupUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogGroupUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_name: value.group_name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogGroupUpdatedChangesRequested>
        for AuditLogGroupUpdatedChangesRequested
    {
        fn from(value: super::AuditLogGroupUpdatedChangesRequested) -> Self {
            Self {
                group_name: Ok(value.group_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogInviteAccepted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogInviteAccepted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogInviteAccepted {
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
    }
    impl ::std::convert::TryFrom<AuditLogInviteAccepted> for super::AuditLogInviteAccepted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogInviteAccepted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogInviteAccepted> for AuditLogInviteAccepted {
        fn from(value: super::AuditLogInviteAccepted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogInviteDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogInviteDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogInviteDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogInviteDeleted> for super::AuditLogInviteDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogInviteDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogInviteDeleted> for AuditLogInviteDeleted {
        fn from(value: super::AuditLogInviteDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogInviteSent {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogInviteSentData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogInviteSent {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogInviteSent {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogInviteSentData>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogInviteSent> for super::AuditLogInviteSent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogInviteSent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogInviteSent> for AuditLogInviteSent {
        fn from(value: super::AuditLogInviteSent) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogInviteSentData {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogInviteSentData {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                role: Ok(Default::default()),
            }
        }
    }
    impl AuditLogInviteSentData {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogInviteSentData> for super::AuditLogInviteSentData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogInviteSentData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogInviteSentData> for AuditLogInviteSentData {
        fn from(value: super::AuditLogInviteSentData) -> Self {
            Self {
                email: Ok(value.email),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistConfigActivated {
        configs: ::std::result::Result<
            ::std::vec::Vec<super::AuditLogIpAllowlistConfigActivatedConfigsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistConfigActivated {
        fn default() -> Self {
            Self {
                configs: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistConfigActivated {
        pub fn configs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::AuditLogIpAllowlistConfigActivatedConfigsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.configs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistConfigActivated>
        for super::AuditLogIpAllowlistConfigActivated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistConfigActivated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configs: value.configs?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistConfigActivated>
        for AuditLogIpAllowlistConfigActivated
    {
        fn from(value: super::AuditLogIpAllowlistConfigActivated) -> Self {
            Self {
                configs: Ok(value.configs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistConfigActivatedConfigsItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistConfigActivatedConfigsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistConfigActivatedConfigsItem {
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
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistConfigActivatedConfigsItem>
        for super::AuditLogIpAllowlistConfigActivatedConfigsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistConfigActivatedConfigsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistConfigActivatedConfigsItem>
        for AuditLogIpAllowlistConfigActivatedConfigsItem
    {
        fn from(value: super::AuditLogIpAllowlistConfigActivatedConfigsItem) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistConfigDeactivated {
        configs: ::std::result::Result<
            ::std::vec::Vec<super::AuditLogIpAllowlistConfigDeactivatedConfigsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistConfigDeactivated {
        fn default() -> Self {
            Self {
                configs: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistConfigDeactivated {
        pub fn configs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::AuditLogIpAllowlistConfigDeactivatedConfigsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.configs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistConfigDeactivated>
        for super::AuditLogIpAllowlistConfigDeactivated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistConfigDeactivated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configs: value.configs?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistConfigDeactivated>
        for AuditLogIpAllowlistConfigDeactivated
    {
        fn from(value: super::AuditLogIpAllowlistConfigDeactivated) -> Self {
            Self {
                configs: Ok(value.configs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistConfigDeactivatedConfigsItem {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistConfigDeactivatedConfigsItem {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistConfigDeactivatedConfigsItem {
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
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistConfigDeactivatedConfigsItem>
        for super::AuditLogIpAllowlistConfigDeactivatedConfigsItem
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistConfigDeactivatedConfigsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistConfigDeactivatedConfigsItem>
        for AuditLogIpAllowlistConfigDeactivatedConfigsItem
    {
        fn from(value: super::AuditLogIpAllowlistConfigDeactivatedConfigsItem) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistCreated {
        allowed_ips:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistCreated {
        fn default() -> Self {
            Self {
                allowed_ips: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistCreated {
        pub fn allowed_ips<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_ips = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_ips: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistCreated> for super::AuditLogIpAllowlistCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_ips: value.allowed_ips?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistCreated> for AuditLogIpAllowlistCreated {
        fn from(value: super::AuditLogIpAllowlistCreated) -> Self {
            Self {
                allowed_ips: Ok(value.allowed_ips),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistDeleted {
        allowed_ips:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistDeleted {
        fn default() -> Self {
            Self {
                allowed_ips: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistDeleted {
        pub fn allowed_ips<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_ips = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_ips: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistDeleted> for super::AuditLogIpAllowlistDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_ips: value.allowed_ips?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistDeleted> for AuditLogIpAllowlistDeleted {
        fn from(value: super::AuditLogIpAllowlistDeleted) -> Self {
            Self {
                allowed_ips: Ok(value.allowed_ips),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogIpAllowlistUpdated {
        allowed_ips:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogIpAllowlistUpdated {
        fn default() -> Self {
            Self {
                allowed_ips: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogIpAllowlistUpdated {
        pub fn allowed_ips<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_ips = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_ips: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogIpAllowlistUpdated> for super::AuditLogIpAllowlistUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogIpAllowlistUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_ips: value.allowed_ips?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogIpAllowlistUpdated> for AuditLogIpAllowlistUpdated {
        fn from(value: super::AuditLogIpAllowlistUpdated) -> Self {
            Self {
                allowed_ips: Ok(value.allowed_ips),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogLoginFailed {
        error_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogLoginFailed {
        fn default() -> Self {
            Self {
                error_code: Ok(Default::default()),
                error_message: Ok(Default::default()),
            }
        }
    }
    impl AuditLogLoginFailed {
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_code: {e}"));
            self
        }
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogLoginFailed> for super::AuditLogLoginFailed {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogLoginFailed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error_code: value.error_code?,
                error_message: value.error_message?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogLoginFailed> for AuditLogLoginFailed {
        fn from(value: super::AuditLogLoginFailed) -> Self {
            Self {
                error_code: Ok(value.error_code),
                error_message: Ok(value.error_message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogLogoutFailed {
        error_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogLogoutFailed {
        fn default() -> Self {
            Self {
                error_code: Ok(Default::default()),
                error_message: Ok(Default::default()),
            }
        }
    }
    impl AuditLogLogoutFailed {
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_code: {e}"));
            self
        }
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogLogoutFailed> for super::AuditLogLogoutFailed {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogLogoutFailed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error_code: value.error_code?,
                error_message: value.error_message?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogLogoutFailed> for AuditLogLogoutFailed {
        fn from(value: super::AuditLogLogoutFailed) -> Self {
            Self {
                error_code: Ok(value.error_code),
                error_message: Ok(value.error_message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogOrganizationUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogOrganizationUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogOrganizationUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogOrganizationUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogOrganizationUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogOrganizationUpdated> for super::AuditLogOrganizationUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogOrganizationUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogOrganizationUpdated> for AuditLogOrganizationUpdated {
        fn from(value: super::AuditLogOrganizationUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogOrganizationUpdatedChangesRequested {
        api_call_logging: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        api_call_logging_project_ids: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        threads_ui_visibility: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        usage_dashboard_visibility: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogOrganizationUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                api_call_logging: Ok(Default::default()),
                api_call_logging_project_ids: Ok(Default::default()),
                description: Ok(Default::default()),
                name: Ok(Default::default()),
                threads_ui_visibility: Ok(Default::default()),
                title: Ok(Default::default()),
                usage_dashboard_visibility: Ok(Default::default()),
            }
        }
    }
    impl AuditLogOrganizationUpdatedChangesRequested {
        pub fn api_call_logging<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_call_logging = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for api_call_logging: {e}"));
            self
        }
        pub fn api_call_logging_project_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_call_logging_project_ids = value.try_into().map_err(|e| {
                format!("error converting supplied value for api_call_logging_project_ids: {e}")
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
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
        pub fn threads_ui_visibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.threads_ui_visibility = value.try_into().map_err(|e| {
                format!("error converting supplied value for threads_ui_visibility: {e}")
            });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn usage_dashboard_visibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage_dashboard_visibility = value.try_into().map_err(|e| {
                format!("error converting supplied value for usage_dashboard_visibility: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogOrganizationUpdatedChangesRequested>
        for super::AuditLogOrganizationUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogOrganizationUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_call_logging: value.api_call_logging?,
                api_call_logging_project_ids: value.api_call_logging_project_ids?,
                description: value.description?,
                name: value.name?,
                threads_ui_visibility: value.threads_ui_visibility?,
                title: value.title?,
                usage_dashboard_visibility: value.usage_dashboard_visibility?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogOrganizationUpdatedChangesRequested>
        for AuditLogOrganizationUpdatedChangesRequested
    {
        fn from(value: super::AuditLogOrganizationUpdatedChangesRequested) -> Self {
            Self {
                api_call_logging: Ok(value.api_call_logging),
                api_call_logging_project_ids: Ok(value.api_call_logging_project_ids),
                description: Ok(value.description),
                name: Ok(value.name),
                threads_ui_visibility: Ok(value.threads_ui_visibility),
                title: Ok(value.title),
                usage_dashboard_visibility: Ok(value.usage_dashboard_visibility),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProject {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProject {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProject {
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
    }
    impl ::std::convert::TryFrom<AuditLogProject> for super::AuditLogProject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogProject> for AuditLogProject {
        fn from(value: super::AuditLogProject) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectArchived {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectArchived {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectArchived {
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
    }
    impl ::std::convert::TryFrom<AuditLogProjectArchived> for super::AuditLogProjectArchived {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectArchived,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectArchived> for AuditLogProjectArchived {
        fn from(value: super::AuditLogProjectArchived) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectCreated {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectCreatedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogProjectCreatedData>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogProjectCreated> for super::AuditLogProjectCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectCreated> for AuditLogProjectCreated {
        fn from(value: super::AuditLogProjectCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectCreatedData {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectCreatedData {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectCreatedData {
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
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogProjectCreatedData> for super::AuditLogProjectCreatedData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectCreatedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectCreatedData> for AuditLogProjectCreatedData {
        fn from(value: super::AuditLogProjectCreatedData) -> Self {
            Self {
                name: Ok(value.name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogProjectDeleted> for super::AuditLogProjectDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectDeleted> for AuditLogProjectDeleted {
        fn from(value: super::AuditLogProjectDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogProjectUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogProjectUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogProjectUpdated> for super::AuditLogProjectUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectUpdated> for AuditLogProjectUpdated {
        fn from(value: super::AuditLogProjectUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogProjectUpdatedChangesRequested {
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogProjectUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                title: Ok(Default::default()),
            }
        }
    }
    impl AuditLogProjectUpdatedChangesRequested {
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogProjectUpdatedChangesRequested>
        for super::AuditLogProjectUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogProjectUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogProjectUpdatedChangesRequested>
        for AuditLogProjectUpdatedChangesRequested
    {
        fn from(value: super::AuditLogProjectUpdatedChangesRequested) -> Self {
            Self {
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRateLimitDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRateLimitDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRateLimitDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogRateLimitDeleted> for super::AuditLogRateLimitDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRateLimitDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogRateLimitDeleted> for AuditLogRateLimitDeleted {
        fn from(value: super::AuditLogRateLimitDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRateLimitUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogRateLimitUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRateLimitUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRateLimitUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogRateLimitUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogRateLimitUpdated> for super::AuditLogRateLimitUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRateLimitUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRateLimitUpdated> for AuditLogRateLimitUpdated {
        fn from(value: super::AuditLogRateLimitUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRateLimitUpdatedChangesRequested {
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
    impl ::std::default::Default for AuditLogRateLimitUpdatedChangesRequested {
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
    impl AuditLogRateLimitUpdatedChangesRequested {
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
    impl ::std::convert::TryFrom<AuditLogRateLimitUpdatedChangesRequested>
        for super::AuditLogRateLimitUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRateLimitUpdatedChangesRequested,
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
    impl ::std::convert::From<super::AuditLogRateLimitUpdatedChangesRequested>
        for AuditLogRateLimitUpdatedChangesRequested
    {
        fn from(value: super::AuditLogRateLimitUpdatedChangesRequested) -> Self {
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
    pub struct AuditLogRoleAssignmentCreated {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        principal_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        principal_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleAssignmentCreated {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                principal_id: Ok(Default::default()),
                principal_type: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleAssignmentCreated {
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
        pub fn principal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.principal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal_id: {e}"));
            self
        }
        pub fn principal_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.principal_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal_type: {e}"));
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleAssignmentCreated>
        for super::AuditLogRoleAssignmentCreated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleAssignmentCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                principal_id: value.principal_id?,
                principal_type: value.principal_type?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleAssignmentCreated> for AuditLogRoleAssignmentCreated {
        fn from(value: super::AuditLogRoleAssignmentCreated) -> Self {
            Self {
                id: Ok(value.id),
                principal_id: Ok(value.principal_id),
                principal_type: Ok(value.principal_type),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleAssignmentDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        principal_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        principal_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleAssignmentDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                principal_id: Ok(Default::default()),
                principal_type: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleAssignmentDeleted {
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
        pub fn principal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.principal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal_id: {e}"));
            self
        }
        pub fn principal_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.principal_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for principal_type: {e}"));
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleAssignmentDeleted>
        for super::AuditLogRoleAssignmentDeleted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleAssignmentDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                principal_id: value.principal_id?,
                principal_type: value.principal_type?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleAssignmentDeleted> for AuditLogRoleAssignmentDeleted {
        fn from(value: super::AuditLogRoleAssignmentDeleted) -> Self {
            Self {
                id: Ok(value.id),
                principal_id: Ok(value.principal_id),
                principal_type: Ok(value.principal_type),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleBoundToResource {
        connector_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        connector_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        permissions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleBoundToResourceSource>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleBoundToResource {
        fn default() -> Self {
            Self {
                connector_id: Ok(Default::default()),
                connector_name: Ok(Default::default()),
                enabled: Ok(Default::default()),
                id: Ok(Default::default()),
                permissions: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
                role_id: Ok(Default::default()),
                source: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleBoundToResource {
        pub fn connector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.connector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connector_id: {e}"));
            self
        }
        pub fn connector_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.connector_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connector_name: {e}"));
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
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
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {e}"));
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogRoleBoundToResourceSource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleBoundToResource> for super::AuditLogRoleBoundToResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleBoundToResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                connector_id: value.connector_id?,
                connector_name: value.connector_name?,
                enabled: value.enabled?,
                id: value.id?,
                permissions: value.permissions?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
                role_id: value.role_id?,
                source: value.source?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleBoundToResource> for AuditLogRoleBoundToResource {
        fn from(value: super::AuditLogRoleBoundToResource) -> Self {
            Self {
                connector_id: Ok(value.connector_id),
                connector_name: Ok(value.connector_name),
                enabled: Ok(value.enabled),
                id: Ok(value.id),
                permissions: Ok(value.permissions),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
                role_id: Ok(value.role_id),
                source: Ok(value.source),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleCreated {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        permissions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleCreated {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                permissions: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
                role_name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleCreated {
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
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {e}"));
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
        pub fn role_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleCreated> for super::AuditLogRoleCreated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                permissions: value.permissions?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
                role_name: value.role_name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleCreated> for AuditLogRoleCreated {
        fn from(value: super::AuditLogRoleCreated) -> Self {
            Self {
                id: Ok(value.id),
                permissions: Ok(value.permissions),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
                role_name: Ok(value.role_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogRoleDeleted> for super::AuditLogRoleDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleDeleted> for AuditLogRoleDeleted {
        fn from(value: super::AuditLogRoleDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleUnboundFromResource {
        connector_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        connector_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enabled: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        permissions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleUnboundFromResourceSource>,
            ::std::string::String,
        >,
        workspace_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleUnboundFromResource {
        fn default() -> Self {
            Self {
                connector_id: Ok(Default::default()),
                connector_name: Ok(Default::default()),
                enabled: Ok(Default::default()),
                id: Ok(Default::default()),
                permissions: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
                role_id: Ok(Default::default()),
                source: Ok(Default::default()),
                workspace_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleUnboundFromResource {
        pub fn connector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.connector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connector_id: {e}"));
            self
        }
        pub fn connector_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.connector_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connector_name: {e}"));
            self
        }
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {e}"));
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
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {e}"));
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogRoleUnboundFromResourceSource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn workspace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.workspace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workspace_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleUnboundFromResource>
        for super::AuditLogRoleUnboundFromResource
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleUnboundFromResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                connector_id: value.connector_id?,
                connector_name: value.connector_name?,
                enabled: value.enabled?,
                id: value.id?,
                permissions: value.permissions?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
                role_id: value.role_id?,
                source: value.source?,
                workspace_id: value.workspace_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleUnboundFromResource>
        for AuditLogRoleUnboundFromResource
    {
        fn from(value: super::AuditLogRoleUnboundFromResource) -> Self {
            Self {
                connector_id: Ok(value.connector_id),
                connector_name: Ok(value.connector_name),
                enabled: Ok(value.enabled),
                id: Ok(value.id),
                permissions: Ok(value.permissions),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
                role_id: Ok(value.role_id),
                source: Ok(value.source),
                workspace_id: Ok(value.workspace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogRoleUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogRoleUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogRoleUpdated> for super::AuditLogRoleUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleUpdated> for AuditLogRoleUpdated {
        fn from(value: super::AuditLogRoleUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogRoleUpdatedChangesRequested {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        permissions_added:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        permissions_removed:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resource_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        resource_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogRoleUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                metadata: Ok(Default::default()),
                permissions_added: Ok(Default::default()),
                permissions_removed: Ok(Default::default()),
                resource_id: Ok(Default::default()),
                resource_type: Ok(Default::default()),
                role_name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogRoleUpdatedChangesRequested {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn permissions_added<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions_added = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions_added: {e}"));
            self
        }
        pub fn permissions_removed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions_removed = value.try_into().map_err(|e| {
                format!("error converting supplied value for permissions_removed: {e}")
            });
            self
        }
        pub fn resource_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_id: {e}"));
            self
        }
        pub fn resource_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_type: {e}"));
            self
        }
        pub fn role_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.role_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogRoleUpdatedChangesRequested>
        for super::AuditLogRoleUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogRoleUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                metadata: value.metadata?,
                permissions_added: value.permissions_added?,
                permissions_removed: value.permissions_removed?,
                resource_id: value.resource_id?,
                resource_type: value.resource_type?,
                role_name: value.role_name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogRoleUpdatedChangesRequested>
        for AuditLogRoleUpdatedChangesRequested
    {
        fn from(value: super::AuditLogRoleUpdatedChangesRequested) -> Self {
            Self {
                description: Ok(value.description),
                metadata: Ok(value.metadata),
                permissions_added: Ok(value.permissions_added),
                permissions_removed: Ok(value.permissions_removed),
                resource_id: Ok(value.resource_id),
                resource_type: Ok(value.resource_type),
                role_name: Ok(value.role_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogScimDisabled {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogScimDisabled {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogScimDisabled {
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
    }
    impl ::std::convert::TryFrom<AuditLogScimDisabled> for super::AuditLogScimDisabled {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogScimDisabled,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogScimDisabled> for AuditLogScimDisabled {
        fn from(value: super::AuditLogScimDisabled) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogScimEnabled {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogScimEnabled {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogScimEnabled {
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
    }
    impl ::std::convert::TryFrom<AuditLogScimEnabled> for super::AuditLogScimEnabled {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogScimEnabled,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogScimEnabled> for AuditLogScimEnabled {
        fn from(value: super::AuditLogScimEnabled) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogServiceAccountCreated {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogServiceAccountCreatedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogServiceAccountCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogServiceAccountCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogServiceAccountCreatedData>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogServiceAccountCreated>
        for super::AuditLogServiceAccountCreated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogServiceAccountCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogServiceAccountCreated> for AuditLogServiceAccountCreated {
        fn from(value: super::AuditLogServiceAccountCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogServiceAccountCreatedData {
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogServiceAccountCreatedData {
        fn default() -> Self {
            Self {
                role: Ok(Default::default()),
            }
        }
    }
    impl AuditLogServiceAccountCreatedData {
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
    impl ::std::convert::TryFrom<AuditLogServiceAccountCreatedData>
        for super::AuditLogServiceAccountCreatedData
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogServiceAccountCreatedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { role: value.role? })
        }
    }
    impl ::std::convert::From<super::AuditLogServiceAccountCreatedData>
        for AuditLogServiceAccountCreatedData
    {
        fn from(value: super::AuditLogServiceAccountCreatedData) -> Self {
            Self {
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogServiceAccountDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogServiceAccountDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogServiceAccountDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogServiceAccountDeleted>
        for super::AuditLogServiceAccountDeleted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogServiceAccountDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogServiceAccountDeleted> for AuditLogServiceAccountDeleted {
        fn from(value: super::AuditLogServiceAccountDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogServiceAccountUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogServiceAccountUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogServiceAccountUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogServiceAccountUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogServiceAccountUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogServiceAccountUpdated>
        for super::AuditLogServiceAccountUpdated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogServiceAccountUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogServiceAccountUpdated> for AuditLogServiceAccountUpdated {
        fn from(value: super::AuditLogServiceAccountUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogServiceAccountUpdatedChangesRequested {
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogServiceAccountUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                role: Ok(Default::default()),
            }
        }
    }
    impl AuditLogServiceAccountUpdatedChangesRequested {
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
    impl ::std::convert::TryFrom<AuditLogServiceAccountUpdatedChangesRequested>
        for super::AuditLogServiceAccountUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogServiceAccountUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { role: value.role? })
        }
    }
    impl ::std::convert::From<super::AuditLogServiceAccountUpdatedChangesRequested>
        for AuditLogServiceAccountUpdatedChangesRequested
    {
        fn from(value: super::AuditLogServiceAccountUpdatedChangesRequested) -> Self {
            Self {
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogUserAdded {
        data: ::std::result::Result<
            ::std::option::Option<super::AuditLogUserAddedData>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogUserAdded {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogUserAdded {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuditLogUserAddedData>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogUserAdded> for super::AuditLogUserAdded {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogUserAdded,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogUserAdded> for AuditLogUserAdded {
        fn from(value: super::AuditLogUserAdded) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogUserAddedData {
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogUserAddedData {
        fn default() -> Self {
            Self {
                role: Ok(Default::default()),
            }
        }
    }
    impl AuditLogUserAddedData {
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
    impl ::std::convert::TryFrom<AuditLogUserAddedData> for super::AuditLogUserAddedData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogUserAddedData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { role: value.role? })
        }
    }
    impl ::std::convert::From<super::AuditLogUserAddedData> for AuditLogUserAddedData {
        fn from(value: super::AuditLogUserAddedData) -> Self {
            Self {
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogUserDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogUserDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogUserDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogUserDeleted> for super::AuditLogUserDeleted {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogUserDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::AuditLogUserDeleted> for AuditLogUserDeleted {
        fn from(value: super::AuditLogUserDeleted) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogUserUpdated {
        changes_requested: ::std::result::Result<
            ::std::option::Option<super::AuditLogUserUpdatedChangesRequested>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogUserUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogUserUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AuditLogUserUpdatedChangesRequested>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogUserUpdated> for super::AuditLogUserUpdated {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogUserUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogUserUpdated> for AuditLogUserUpdated {
        fn from(value: super::AuditLogUserUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogUserUpdatedChangesRequested {
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogUserUpdatedChangesRequested {
        fn default() -> Self {
            Self {
                role: Ok(Default::default()),
            }
        }
    }
    impl AuditLogUserUpdatedChangesRequested {
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
    impl ::std::convert::TryFrom<AuditLogUserUpdatedChangesRequested>
        for super::AuditLogUserUpdatedChangesRequested
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogUserUpdatedChangesRequested,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { role: value.role? })
        }
    }
    impl ::std::convert::From<super::AuditLogUserUpdatedChangesRequested>
        for AuditLogUserUpdatedChangesRequested
    {
        fn from(value: super::AuditLogUserUpdatedChangesRequested) -> Self {
            Self {
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderCreated {
        data: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderCreated>
        for super::AuditLogWorkloadIdentityProviderCreated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderCreated>
        for AuditLogWorkloadIdentityProviderCreated
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderDeleted {
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
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderDeleted>
        for super::AuditLogWorkloadIdentityProviderDeleted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderDeleted>
        for AuditLogWorkloadIdentityProviderDeleted
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderDeleted) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderMappingCreated {
        data: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        identity_provider_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderMappingCreated {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                id: Ok(Default::default()),
                identity_provider_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderMappingCreated {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn identity_provider_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.identity_provider_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for identity_provider_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderMappingCreated>
        for super::AuditLogWorkloadIdentityProviderMappingCreated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderMappingCreated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                id: value.id?,
                identity_provider_id: value.identity_provider_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderMappingCreated>
        for AuditLogWorkloadIdentityProviderMappingCreated
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderMappingCreated) -> Self {
            Self {
                data: Ok(value.data),
                id: Ok(value.id),
                identity_provider_id: Ok(value.identity_provider_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderMappingDeleted {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        identity_provider_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        service_account_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderMappingDeleted {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                identity_provider_id: Ok(Default::default()),
                project_id: Ok(Default::default()),
                service_account_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderMappingDeleted {
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
        pub fn identity_provider_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.identity_provider_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for identity_provider_id: {e}")
            });
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
        pub fn service_account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.service_account_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for service_account_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderMappingDeleted>
        for super::AuditLogWorkloadIdentityProviderMappingDeleted
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderMappingDeleted,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                identity_provider_id: value.identity_provider_id?,
                project_id: value.project_id?,
                service_account_id: value.service_account_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderMappingDeleted>
        for AuditLogWorkloadIdentityProviderMappingDeleted
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderMappingDeleted) -> Self {
            Self {
                id: Ok(value.id),
                identity_provider_id: Ok(value.identity_provider_id),
                project_id: Ok(value.project_id),
                service_account_id: Ok(value.service_account_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderMappingUpdated {
        changes_requested: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        identity_provider_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderMappingUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
                identity_provider_id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderMappingUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
        pub fn identity_provider_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.identity_provider_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for identity_provider_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderMappingUpdated>
        for super::AuditLogWorkloadIdentityProviderMappingUpdated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderMappingUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
                identity_provider_id: value.identity_provider_id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderMappingUpdated>
        for AuditLogWorkloadIdentityProviderMappingUpdated
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderMappingUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
                identity_provider_id: Ok(value.identity_provider_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogWorkloadIdentityProviderUpdated {
        changes_requested: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AuditLogWorkloadIdentityProviderUpdated {
        fn default() -> Self {
            Self {
                changes_requested: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl AuditLogWorkloadIdentityProviderUpdated {
        pub fn changes_requested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.changes_requested = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for changes_requested: {e}"));
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
    }
    impl ::std::convert::TryFrom<AuditLogWorkloadIdentityProviderUpdated>
        for super::AuditLogWorkloadIdentityProviderUpdated
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogWorkloadIdentityProviderUpdated,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                changes_requested: value.changes_requested?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogWorkloadIdentityProviderUpdated>
        for AuditLogWorkloadIdentityProviderUpdated
    {
        fn from(value: super::AuditLogWorkloadIdentityProviderUpdated) -> Self {
            Self {
                changes_requested: Ok(value.changes_requested),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuditLogsApiSchema {
        list_audit_logs_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_audit_logs_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_audit_logs_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_audit_logs_params:
            ::std::result::Result<super::ListAuditLogsParams, ::std::string::String>,
        list_audit_logs_response:
            ::std::result::Result<super::ListAuditLogsResponse, ::std::string::String>,
    }
    impl ::std::default::Default for AuditLogsApiSchema {
        fn default() -> Self {
            Self {
                list_audit_logs_error400: Err(
                    "no value supplied for list_audit_logs_error400".to_string()
                ),
                list_audit_logs_error404: Err(
                    "no value supplied for list_audit_logs_error404".to_string()
                ),
                list_audit_logs_error429: Err(
                    "no value supplied for list_audit_logs_error429".to_string()
                ),
                list_audit_logs_params: Err(
                    "no value supplied for list_audit_logs_params".to_string()
                ),
                list_audit_logs_response: Err(
                    "no value supplied for list_audit_logs_response".to_string()
                ),
            }
        }
    }
    impl AuditLogsApiSchema {
        pub fn list_audit_logs_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_audit_logs_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_audit_logs_error400: {e}")
            });
            self
        }
        pub fn list_audit_logs_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_audit_logs_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_audit_logs_error404: {e}")
            });
            self
        }
        pub fn list_audit_logs_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_audit_logs_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_audit_logs_error429: {e}")
            });
            self
        }
        pub fn list_audit_logs_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListAuditLogsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_audit_logs_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_audit_logs_params: {e}")
            });
            self
        }
        pub fn list_audit_logs_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListAuditLogsResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_audit_logs_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_audit_logs_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AuditLogsApiSchema> for super::AuditLogsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuditLogsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                list_audit_logs_error400: value.list_audit_logs_error400?,
                list_audit_logs_error404: value.list_audit_logs_error404?,
                list_audit_logs_error429: value.list_audit_logs_error429?,
                list_audit_logs_params: value.list_audit_logs_params?,
                list_audit_logs_response: value.list_audit_logs_response?,
            })
        }
    }
    impl ::std::convert::From<super::AuditLogsApiSchema> for AuditLogsApiSchema {
        fn from(value: super::AuditLogsApiSchema) -> Self {
            Self {
                list_audit_logs_error400: Ok(value.list_audit_logs_error400),
                list_audit_logs_error404: Ok(value.list_audit_logs_error404),
                list_audit_logs_error429: Ok(value.list_audit_logs_error429),
                list_audit_logs_params: Ok(value.list_audit_logs_params),
                list_audit_logs_response: Ok(value.list_audit_logs_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListAuditLogsParams {
        actor_emails:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        actor_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        before: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        effective_at: ::std::result::Result<
            ::std::option::Option<super::ListAuditLogsParamsEffectiveAt>,
            ::std::string::String,
        >,
        event_types:
            ::std::result::Result<::std::vec::Vec<super::AuditLogEventType>, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        project_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        resource_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        tenant_only: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ListAuditLogsParams {
        fn default() -> Self {
            Self {
                actor_emails: Ok(Default::default()),
                actor_ids: Ok(Default::default()),
                after: Ok(Default::default()),
                before: Ok(Default::default()),
                effective_at: Ok(Default::default()),
                event_types: Ok(Default::default()),
                limit: Ok(Default::default()),
                project_ids: Ok(Default::default()),
                resource_ids: Ok(Default::default()),
                tenant_only: Ok(Default::default()),
            }
        }
    }
    impl ListAuditLogsParams {
        pub fn actor_emails<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.actor_emails = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actor_emails: {e}"));
            self
        }
        pub fn actor_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.actor_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actor_ids: {e}"));
            self
        }
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
        pub fn effective_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListAuditLogsParamsEffectiveAt>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.effective_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effective_at: {e}"));
            self
        }
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AuditLogEventType>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
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
        pub fn resource_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.resource_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resource_ids: {e}"));
            self
        }
        pub fn tenant_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant_only: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListAuditLogsParams> for super::ListAuditLogsParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListAuditLogsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actor_emails: value.actor_emails?,
                actor_ids: value.actor_ids?,
                after: value.after?,
                before: value.before?,
                effective_at: value.effective_at?,
                event_types: value.event_types?,
                limit: value.limit?,
                project_ids: value.project_ids?,
                resource_ids: value.resource_ids?,
                tenant_only: value.tenant_only?,
            })
        }
    }
    impl ::std::convert::From<super::ListAuditLogsParams> for ListAuditLogsParams {
        fn from(value: super::ListAuditLogsParams) -> Self {
            Self {
                actor_emails: Ok(value.actor_emails),
                actor_ids: Ok(value.actor_ids),
                after: Ok(value.after),
                before: Ok(value.before),
                effective_at: Ok(value.effective_at),
                event_types: Ok(value.event_types),
                limit: Ok(value.limit),
                project_ids: Ok(value.project_ids),
                resource_ids: Ok(value.resource_ids),
                tenant_only: Ok(value.tenant_only),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListAuditLogsParamsEffectiveAt {
        gt: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        gte: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        lt: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        lte: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListAuditLogsParamsEffectiveAt {
        fn default() -> Self {
            Self {
                gt: Ok(Default::default()),
                gte: Ok(Default::default()),
                lt: Ok(Default::default()),
                lte: Ok(Default::default()),
            }
        }
    }
    impl ListAuditLogsParamsEffectiveAt {
        pub fn gt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gt: {e}"));
            self
        }
        pub fn gte<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gte = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gte: {e}"));
            self
        }
        pub fn lt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lt: {e}"));
            self
        }
        pub fn lte<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lte = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lte: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListAuditLogsParamsEffectiveAt>
        for super::ListAuditLogsParamsEffectiveAt
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListAuditLogsParamsEffectiveAt,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                gt: value.gt?,
                gte: value.gte?,
                lt: value.lt?,
                lte: value.lte?,
            })
        }
    }
    impl ::std::convert::From<super::ListAuditLogsParamsEffectiveAt>
        for ListAuditLogsParamsEffectiveAt
    {
        fn from(value: super::ListAuditLogsParamsEffectiveAt) -> Self {
            Self {
                gt: Ok(value.gt),
                gte: Ok(value.gte),
                lt: Ok(value.lt),
                lte: Ok(value.lte),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListAuditLogsResponse {
        data: ::std::result::Result<::std::vec::Vec<super::AuditLog>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::ListAuditLogsResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for ListAuditLogsResponse {
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
    impl ListAuditLogsResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AuditLog>>,
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
            T: ::std::convert::TryInto<super::ListAuditLogsResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListAuditLogsResponse> for super::ListAuditLogsResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListAuditLogsResponse,
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
    impl ::std::convert::From<super::ListAuditLogsResponse> for ListAuditLogsResponse {
        fn from(value: super::ListAuditLogsResponse) -> Self {
            Self {
                data: Ok(value.data),
                first_id: Ok(value.first_id),
                has_more: Ok(value.has_more),
                last_id: Ok(value.last_id),
                object: Ok(value.object),
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
