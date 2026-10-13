pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`DeleteInviteParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteInviteParams {
    pub invite_id: ::std::string::String,
}
impl DeleteInviteParams {
    pub fn builder() -> builder::DeleteInviteParams {
        Default::default()
    }
}
#[doc = "Represents an individual `invite` to the organization."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Invite {
    #[doc = "The Unix timestamp (in seconds) of when the invite was accepted."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub accepted_at: ::std::option::Option<i64>,
    #[doc = "The Unix timestamp (in seconds) of when the invite was sent."]
    pub created_at: i64,
    #[doc = "The email address of the individual to whom the invite was sent"]
    pub email: ::std::string::String,
    #[doc = "The Unix timestamp (in seconds) of when the invite expires."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `organization.invite`"]
    pub object: InviteObject,
    #[doc = "The projects that were granted membership upon acceptance of the invite."]
    pub projects: ::std::vec::Vec<InviteProjectsItem>,
    #[doc = "`owner` or `reader`"]
    pub role: InviteRole,
    #[doc = "`accepted`,`expired`, or `pending`"]
    pub status: InviteStatus,
}
impl Invite {
    pub fn builder() -> builder::Invite {
        Default::default()
    }
}
#[doc = "`InviteDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    #[doc = "The object type, which is always `organization.invite.deleted`"]
    pub object: InviteDeleteResponseObject,
}
impl InviteDeleteResponse {
    pub fn builder() -> builder::InviteDeleteResponse {
        Default::default()
    }
}
#[doc = "The object type, which is always `organization.invite.deleted`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteDeleteResponseObject {
    #[serde(rename = "organization.invite.deleted")]
    OrganizationInviteDeleted,
}
impl ::std::fmt::Display for InviteDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationInviteDeleted => f.write_str("organization.invite.deleted"),
        }
    }
}
impl ::std::str::FromStr for InviteDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.invite.deleted" => Ok(Self::OrganizationInviteDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InviteListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteListResponse {
    pub data: ::std::vec::Vec<Invite>,
    #[doc = "The first `invite_id` in the retrieved `list`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "The `has_more` property is used for pagination to indicate there are additional results."]
    pub has_more: bool,
    #[doc = "The last `invite_id` in the retrieved `list`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The object type, which is always `list`"]
    pub object: InviteListResponseObject,
}
impl InviteListResponse {
    pub fn builder() -> builder::InviteListResponse {
        Default::default()
    }
}
#[doc = "The object type, which is always `list`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for InviteListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for InviteListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The object type, which is always `organization.invite`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteObject {
    #[serde(rename = "organization.invite")]
    OrganizationInvite,
}
impl ::std::fmt::Display for InviteObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationInvite => f.write_str("organization.invite"),
        }
    }
}
impl ::std::str::FromStr for InviteObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.invite" => Ok(Self::OrganizationInvite),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InviteProjectsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteProjectsItem {
    #[doc = "Project's public ID"]
    pub id: ::std::string::String,
    #[doc = "Project membership role"]
    pub role: InviteProjectsItemRole,
}
impl InviteProjectsItem {
    pub fn builder() -> builder::InviteProjectsItem {
        Default::default()
    }
}
#[doc = "Project membership role"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteProjectsItemRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for InviteProjectsItemRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for InviteProjectsItemRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteProjectsItemRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteProjectsItemRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InviteRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteRequest {
    #[doc = "Send an email to this address"]
    pub email: ::std::string::String,
    #[doc = "An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior. If empty list is passed, the user will not be invited to any projects, including the default one."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<InviteRequestProjectsItem>,
    #[doc = "`owner` or `reader`"]
    pub role: InviteRequestRole,
}
impl InviteRequest {
    pub fn builder() -> builder::InviteRequest {
        Default::default()
    }
}
#[doc = "`InviteRequestProjectsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InviteRequestProjectsItem {
    #[doc = "Project's public ID"]
    pub id: ::std::string::String,
    #[doc = "Project membership role"]
    pub role: InviteRequestProjectsItemRole,
}
impl InviteRequestProjectsItem {
    pub fn builder() -> builder::InviteRequestProjectsItem {
        Default::default()
    }
}
#[doc = "Project membership role"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteRequestProjectsItemRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for InviteRequestProjectsItemRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for InviteRequestProjectsItemRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteRequestProjectsItemRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteRequestProjectsItemRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`owner` or `reader`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteRequestRole {
    #[serde(rename = "reader")]
    Reader,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for InviteRequestRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reader => f.write_str("reader"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for InviteRequestRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reader" => Ok(Self::Reader),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`owner` or `reader`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
}
impl ::std::fmt::Display for InviteRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Owner => f.write_str("owner"),
            Self::Reader => f.write_str("reader"),
        }
    }
}
impl ::std::str::FromStr for InviteRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "owner" => Ok(Self::Owner),
            "reader" => Ok(Self::Reader),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`accepted`,`expired`, or `pending`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum InviteStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "pending")]
    Pending,
}
impl ::std::fmt::Display for InviteStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accepted => f.write_str("accepted"),
            Self::Expired => f.write_str("expired"),
            Self::Pending => f.write_str("pending"),
        }
    }
}
impl ::std::str::FromStr for InviteStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "expired" => Ok(Self::Expired),
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InviteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InviteStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InvitesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct InvitesApiSchema {
    #[serde(rename = "DeleteInviteError400")]
    pub delete_invite_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteInviteError403")]
    pub delete_invite_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteInviteError404")]
    pub delete_invite_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteInviteError429")]
    pub delete_invite_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteInviteParams")]
    pub delete_invite_params: DeleteInviteParams,
    #[serde(rename = "DeleteInviteResponse")]
    pub delete_invite_response: InviteDeleteResponse,
    #[serde(rename = "InviteUserError400")]
    pub invite_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "InviteUserError403")]
    pub invite_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "InviteUserError429")]
    pub invite_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "InviteUserRequest")]
    pub invite_user_request: InviteRequest,
    #[serde(rename = "InviteUserResponse")]
    pub invite_user_response: Invite,
    #[serde(rename = "ListInvitesError400")]
    pub list_invites_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListInvitesError403")]
    pub list_invites_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListInvitesError404")]
    pub list_invites_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListInvitesError429")]
    pub list_invites_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListInvitesParams")]
    pub list_invites_params: ListInvitesParams,
    #[serde(rename = "ListInvitesResponse")]
    pub list_invites_response: InviteListResponse,
    #[serde(rename = "RetrieveInviteError400")]
    pub retrieve_invite_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveInviteError403")]
    pub retrieve_invite_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveInviteError404")]
    pub retrieve_invite_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveInviteError429")]
    pub retrieve_invite_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveInviteError500")]
    pub retrieve_invite_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveInviteParams")]
    pub retrieve_invite_params: RetrieveInviteParams,
    #[serde(rename = "RetrieveInviteResponse")]
    pub retrieve_invite_response: Invite,
}
impl InvitesApiSchema {
    pub fn builder() -> builder::InvitesApiSchema {
        Default::default()
    }
}
#[doc = "`ListInvitesParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListInvitesParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
impl ListInvitesParams {
    pub fn builder() -> builder::ListInvitesParams {
        Default::default()
    }
}
#[doc = "`RetrieveInviteParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveInviteParams {
    pub invite_id: ::std::string::String,
}
impl RetrieveInviteParams {
    pub fn builder() -> builder::RetrieveInviteParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DeleteInviteParams {
        invite_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteInviteParams {
        fn default() -> Self {
            Self {
                invite_id: Err("no value supplied for invite_id".to_string()),
            }
        }
    }
    impl DeleteInviteParams {
        pub fn invite_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invite_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteInviteParams> for super::DeleteInviteParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteInviteParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                invite_id: value.invite_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteInviteParams> for DeleteInviteParams {
        fn from(value: super::DeleteInviteParams) -> Self {
            Self {
                invite_id: Ok(value.invite_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Invite {
        accepted_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        email: ::std::result::Result<::std::string::String, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::InviteObject, ::std::string::String>,
        projects: ::std::result::Result<
            ::std::vec::Vec<super::InviteProjectsItem>,
            ::std::string::String,
        >,
        role: ::std::result::Result<super::InviteRole, ::std::string::String>,
        status: ::std::result::Result<super::InviteStatus, ::std::string::String>,
    }
    impl ::std::default::Default for Invite {
        fn default() -> Self {
            Self {
                accepted_at: Ok(Default::default()),
                created_at: Err("no value supplied for created_at".to_string()),
                email: Err("no value supplied for email".to_string()),
                expires_at: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
                projects: Err("no value supplied for projects".to_string()),
                role: Err("no value supplied for role".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl Invite {
        pub fn accepted_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.accepted_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accepted_at: {e}"));
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
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn projects<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::InviteProjectsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.projects = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for projects: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Invite> for super::Invite {
        type Error = super::error::ConversionError;
        fn try_from(value: Invite) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accepted_at: value.accepted_at?,
                created_at: value.created_at?,
                email: value.email?,
                expires_at: value.expires_at?,
                id: value.id?,
                object: value.object?,
                projects: value.projects?,
                role: value.role?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::Invite> for Invite {
        fn from(value: super::Invite) -> Self {
            Self {
                accepted_at: Ok(value.accepted_at),
                created_at: Ok(value.created_at),
                email: Ok(value.email),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                object: Ok(value.object),
                projects: Ok(value.projects),
                role: Ok(value.role),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InviteDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::InviteDeleteResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for InviteDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl InviteDeleteResponse {
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
            T: ::std::convert::TryInto<super::InviteDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteDeleteResponse> for super::InviteDeleteResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::InviteDeleteResponse> for InviteDeleteResponse {
        fn from(value: super::InviteDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InviteListResponse {
        data: ::std::result::Result<::std::vec::Vec<super::Invite>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::InviteListResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for InviteListResponse {
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
    impl InviteListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Invite>>,
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
            T: ::std::convert::TryInto<super::InviteListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteListResponse> for super::InviteListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteListResponse,
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
    impl ::std::convert::From<super::InviteListResponse> for InviteListResponse {
        fn from(value: super::InviteListResponse) -> Self {
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
    pub struct InviteProjectsItem {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::InviteProjectsItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for InviteProjectsItem {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl InviteProjectsItem {
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteProjectsItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteProjectsItem> for super::InviteProjectsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteProjectsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::InviteProjectsItem> for InviteProjectsItem {
        fn from(value: super::InviteProjectsItem) -> Self {
            Self {
                id: Ok(value.id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InviteRequest {
        email: ::std::result::Result<::std::string::String, ::std::string::String>,
        projects: ::std::result::Result<
            ::std::vec::Vec<super::InviteRequestProjectsItem>,
            ::std::string::String,
        >,
        role: ::std::result::Result<super::InviteRequestRole, ::std::string::String>,
    }
    impl ::std::default::Default for InviteRequest {
        fn default() -> Self {
            Self {
                email: Err("no value supplied for email".to_string()),
                projects: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl InviteRequest {
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
        pub fn projects<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::InviteRequestProjectsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.projects = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for projects: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteRequestRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteRequest> for super::InviteRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                projects: value.projects?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::InviteRequest> for InviteRequest {
        fn from(value: super::InviteRequest) -> Self {
            Self {
                email: Ok(value.email),
                projects: Ok(value.projects),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InviteRequestProjectsItem {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::InviteRequestProjectsItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for InviteRequestProjectsItem {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl InviteRequestProjectsItem {
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteRequestProjectsItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InviteRequestProjectsItem> for super::InviteRequestProjectsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InviteRequestProjectsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::InviteRequestProjectsItem> for InviteRequestProjectsItem {
        fn from(value: super::InviteRequestProjectsItem) -> Self {
            Self {
                id: Ok(value.id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvitesApiSchema {
        delete_invite_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_invite_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_invite_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_invite_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_invite_params:
            ::std::result::Result<super::DeleteInviteParams, ::std::string::String>,
        delete_invite_response:
            ::std::result::Result<super::InviteDeleteResponse, ::std::string::String>,
        invite_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        invite_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        invite_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        invite_user_request: ::std::result::Result<super::InviteRequest, ::std::string::String>,
        invite_user_response: ::std::result::Result<super::Invite, ::std::string::String>,
        list_invites_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_invites_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_invites_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_invites_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_invites_params: ::std::result::Result<super::ListInvitesParams, ::std::string::String>,
        list_invites_response:
            ::std::result::Result<super::InviteListResponse, ::std::string::String>,
        retrieve_invite_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_invite_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_invite_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_invite_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_invite_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_invite_params:
            ::std::result::Result<super::RetrieveInviteParams, ::std::string::String>,
        retrieve_invite_response: ::std::result::Result<super::Invite, ::std::string::String>,
    }
    impl ::std::default::Default for InvitesApiSchema {
        fn default() -> Self {
            Self {
                delete_invite_error400: Err(
                    "no value supplied for delete_invite_error400".to_string()
                ),
                delete_invite_error403: Err(
                    "no value supplied for delete_invite_error403".to_string()
                ),
                delete_invite_error404: Err(
                    "no value supplied for delete_invite_error404".to_string()
                ),
                delete_invite_error429: Err(
                    "no value supplied for delete_invite_error429".to_string()
                ),
                delete_invite_params: Err("no value supplied for delete_invite_params".to_string()),
                delete_invite_response: Err(
                    "no value supplied for delete_invite_response".to_string()
                ),
                invite_user_error400: Err("no value supplied for invite_user_error400".to_string()),
                invite_user_error403: Err("no value supplied for invite_user_error403".to_string()),
                invite_user_error429: Err("no value supplied for invite_user_error429".to_string()),
                invite_user_request: Err("no value supplied for invite_user_request".to_string()),
                invite_user_response: Err("no value supplied for invite_user_response".to_string()),
                list_invites_error400: Err(
                    "no value supplied for list_invites_error400".to_string()
                ),
                list_invites_error403: Err(
                    "no value supplied for list_invites_error403".to_string()
                ),
                list_invites_error404: Err(
                    "no value supplied for list_invites_error404".to_string()
                ),
                list_invites_error429: Err(
                    "no value supplied for list_invites_error429".to_string()
                ),
                list_invites_params: Err("no value supplied for list_invites_params".to_string()),
                list_invites_response: Err(
                    "no value supplied for list_invites_response".to_string()
                ),
                retrieve_invite_error400: Err(
                    "no value supplied for retrieve_invite_error400".to_string()
                ),
                retrieve_invite_error403: Err(
                    "no value supplied for retrieve_invite_error403".to_string()
                ),
                retrieve_invite_error404: Err(
                    "no value supplied for retrieve_invite_error404".to_string()
                ),
                retrieve_invite_error429: Err(
                    "no value supplied for retrieve_invite_error429".to_string()
                ),
                retrieve_invite_error500: Err(
                    "no value supplied for retrieve_invite_error500".to_string()
                ),
                retrieve_invite_params: Err(
                    "no value supplied for retrieve_invite_params".to_string()
                ),
                retrieve_invite_response: Err(
                    "no value supplied for retrieve_invite_response".to_string()
                ),
            }
        }
    }
    impl InvitesApiSchema {
        pub fn delete_invite_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_error400: {e}")
            });
            self
        }
        pub fn delete_invite_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_error403: {e}")
            });
            self
        }
        pub fn delete_invite_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_error404: {e}")
            });
            self
        }
        pub fn delete_invite_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_error429: {e}")
            });
            self
        }
        pub fn delete_invite_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteInviteParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_params: {e}")
            });
            self
        }
        pub fn delete_invite_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_invite_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_invite_response: {e}")
            });
            self
        }
        pub fn invite_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for invite_user_error400: {e}")
            });
            self
        }
        pub fn invite_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for invite_user_error403: {e}")
            });
            self
        }
        pub fn invite_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for invite_user_error429: {e}")
            });
            self
        }
        pub fn invite_user_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_user_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for invite_user_request: {e}")
            });
            self
        }
        pub fn invite_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Invite>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for invite_user_response: {e}")
            });
            self
        }
        pub fn list_invites_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_error400: {e}")
            });
            self
        }
        pub fn list_invites_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_error403: {e}")
            });
            self
        }
        pub fn list_invites_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_error404: {e}")
            });
            self
        }
        pub fn list_invites_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_error429: {e}")
            });
            self
        }
        pub fn list_invites_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListInvitesParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_params: {e}")
            });
            self
        }
        pub fn list_invites_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InviteListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_invites_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_invites_response: {e}")
            });
            self
        }
        pub fn retrieve_invite_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_error400: {e}")
            });
            self
        }
        pub fn retrieve_invite_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_error403: {e}")
            });
            self
        }
        pub fn retrieve_invite_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_error404: {e}")
            });
            self
        }
        pub fn retrieve_invite_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_error429: {e}")
            });
            self
        }
        pub fn retrieve_invite_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_error500: {e}")
            });
            self
        }
        pub fn retrieve_invite_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveInviteParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_params: {e}")
            });
            self
        }
        pub fn retrieve_invite_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Invite>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_invite_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_invite_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<InvitesApiSchema> for super::InvitesApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvitesApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delete_invite_error400: value.delete_invite_error400?,
                delete_invite_error403: value.delete_invite_error403?,
                delete_invite_error404: value.delete_invite_error404?,
                delete_invite_error429: value.delete_invite_error429?,
                delete_invite_params: value.delete_invite_params?,
                delete_invite_response: value.delete_invite_response?,
                invite_user_error400: value.invite_user_error400?,
                invite_user_error403: value.invite_user_error403?,
                invite_user_error429: value.invite_user_error429?,
                invite_user_request: value.invite_user_request?,
                invite_user_response: value.invite_user_response?,
                list_invites_error400: value.list_invites_error400?,
                list_invites_error403: value.list_invites_error403?,
                list_invites_error404: value.list_invites_error404?,
                list_invites_error429: value.list_invites_error429?,
                list_invites_params: value.list_invites_params?,
                list_invites_response: value.list_invites_response?,
                retrieve_invite_error400: value.retrieve_invite_error400?,
                retrieve_invite_error403: value.retrieve_invite_error403?,
                retrieve_invite_error404: value.retrieve_invite_error404?,
                retrieve_invite_error429: value.retrieve_invite_error429?,
                retrieve_invite_error500: value.retrieve_invite_error500?,
                retrieve_invite_params: value.retrieve_invite_params?,
                retrieve_invite_response: value.retrieve_invite_response?,
            })
        }
    }
    impl ::std::convert::From<super::InvitesApiSchema> for InvitesApiSchema {
        fn from(value: super::InvitesApiSchema) -> Self {
            Self {
                delete_invite_error400: Ok(value.delete_invite_error400),
                delete_invite_error403: Ok(value.delete_invite_error403),
                delete_invite_error404: Ok(value.delete_invite_error404),
                delete_invite_error429: Ok(value.delete_invite_error429),
                delete_invite_params: Ok(value.delete_invite_params),
                delete_invite_response: Ok(value.delete_invite_response),
                invite_user_error400: Ok(value.invite_user_error400),
                invite_user_error403: Ok(value.invite_user_error403),
                invite_user_error429: Ok(value.invite_user_error429),
                invite_user_request: Ok(value.invite_user_request),
                invite_user_response: Ok(value.invite_user_response),
                list_invites_error400: Ok(value.list_invites_error400),
                list_invites_error403: Ok(value.list_invites_error403),
                list_invites_error404: Ok(value.list_invites_error404),
                list_invites_error429: Ok(value.list_invites_error429),
                list_invites_params: Ok(value.list_invites_params),
                list_invites_response: Ok(value.list_invites_response),
                retrieve_invite_error400: Ok(value.retrieve_invite_error400),
                retrieve_invite_error403: Ok(value.retrieve_invite_error403),
                retrieve_invite_error404: Ok(value.retrieve_invite_error404),
                retrieve_invite_error429: Ok(value.retrieve_invite_error429),
                retrieve_invite_error500: Ok(value.retrieve_invite_error500),
                retrieve_invite_params: Ok(value.retrieve_invite_params),
                retrieve_invite_response: Ok(value.retrieve_invite_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListInvitesParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListInvitesParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListInvitesParams {
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
    }
    impl ::std::convert::TryFrom<ListInvitesParams> for super::ListInvitesParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListInvitesParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListInvitesParams> for ListInvitesParams {
        fn from(value: super::ListInvitesParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveInviteParams {
        invite_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveInviteParams {
        fn default() -> Self {
            Self {
                invite_id: Err("no value supplied for invite_id".to_string()),
            }
        }
    }
    impl RetrieveInviteParams {
        pub fn invite_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.invite_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invite_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveInviteParams> for super::RetrieveInviteParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveInviteParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                invite_id: value.invite_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveInviteParams> for RetrieveInviteParams {
        fn from(value: super::RetrieveInviteParams) -> Self {
            Self {
                invite_id: Ok(value.invite_id),
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
