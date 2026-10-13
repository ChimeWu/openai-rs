pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AddGroupUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AddGroupUserParams {
    pub group_id: ::std::string::String,
}
impl AddGroupUserParams {
    pub fn builder() -> builder::AddGroupUserParams {
        Default::default()
    }
}
#[doc = "Request payload for adding a user to a group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateGroupUserBody {
    #[doc = "Identifier of the user to add to the group."]
    pub user_id: ::std::string::String,
}
impl CreateGroupUserBody {
    pub fn builder() -> builder::CreateGroupUserBody {
        Default::default()
    }
}
#[doc = "Details about a user returned from an organization group membership lookup."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupMemberUser {
    #[doc = "Email address of the user, or `null` for users without an email."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier for the user."]
    pub id: ::std::string::String,
    #[doc = "Whether the user is a service account."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub is_service_account: ::std::option::Option<bool>,
    #[doc = "Display name of the user."]
    pub name: ::std::string::String,
    #[doc = "URL of the user's profile picture, if available."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub picture: ::std::option::Option<::std::string::String>,
    #[doc = "The type of user."]
    pub user_type: GroupMemberUserUserType,
}
impl GroupMemberUser {
    pub fn builder() -> builder::GroupMemberUser {
        Default::default()
    }
}
#[doc = "The type of user."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GroupMemberUserUserType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "tenant_user")]
    TenantUser,
}
impl ::std::fmt::Display for GroupMemberUserUserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::TenantUser => f.write_str("tenant_user"),
        }
    }
}
impl ::std::str::FromStr for GroupMemberUserUserType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "tenant_user" => Ok(Self::TenantUser),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupMemberUserUserType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupMemberUserUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents an individual user returned when inspecting group membership."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupUser {
    #[doc = "The email address of the user."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub email: ::std::option::Option<::std::string::String>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: ::std::string::String,
    #[doc = "The name of the user."]
    pub name: ::std::string::String,
}
impl GroupUser {
    pub fn builder() -> builder::GroupUser {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after adding a user to a group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupUserAssignment {
    #[doc = "Identifier of the group the user was added to."]
    pub group_id: ::std::string::String,
    #[doc = "Always `group.user`."]
    pub object: GroupUserAssignmentObject,
    #[doc = "Identifier of the user that was added."]
    pub user_id: ::std::string::String,
}
impl GroupUserAssignment {
    pub fn builder() -> builder::GroupUserAssignment {
        Default::default()
    }
}
#[doc = "Always `group.user`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GroupUserAssignmentObject {
    #[serde(rename = "group.user")]
    GroupUser,
}
impl ::std::fmt::Display for GroupUserAssignmentObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GroupUser => f.write_str("group.user"),
        }
    }
}
impl ::std::str::FromStr for GroupUserAssignmentObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group.user" => Ok(Self::GroupUser),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupUserAssignmentObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupUserAssignmentObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Confirmation payload returned after removing a user from a group."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupUserDeletedResource {
    #[doc = "Whether the group membership was removed."]
    pub deleted: bool,
    #[doc = "Always `group.user.deleted`."]
    pub object: GroupUserDeletedResourceObject,
}
impl GroupUserDeletedResource {
    pub fn builder() -> builder::GroupUserDeletedResource {
        Default::default()
    }
}
#[doc = "Always `group.user.deleted`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GroupUserDeletedResourceObject {
    #[serde(rename = "group.user.deleted")]
    GroupUserDeleted,
}
impl ::std::fmt::Display for GroupUserDeletedResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GroupUserDeleted => f.write_str("group.user.deleted"),
        }
    }
}
impl ::std::str::FromStr for GroupUserDeletedResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "group.user.deleted" => Ok(Self::GroupUserDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GroupUserDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupUserDeletedResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GroupUsersApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupUsersApiSchema {
    #[serde(rename = "AddGroupUserError400")]
    pub add_group_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddGroupUserError403")]
    pub add_group_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AddGroupUserError404")]
    pub add_group_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddGroupUserError409")]
    pub add_group_user_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddGroupUserError429")]
    pub add_group_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AddGroupUserParams")]
    pub add_group_user_params: AddGroupUserParams,
    #[serde(rename = "AddGroupUserRequest")]
    pub add_group_user_request: CreateGroupUserBody,
    #[serde(rename = "AddGroupUserResponse")]
    pub add_group_user_response: GroupUserAssignment,
    #[serde(rename = "ListGroupUsersError400")]
    pub list_group_users_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupUsersError403")]
    pub list_group_users_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListGroupUsersError404")]
    pub list_group_users_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupUsersError429")]
    pub list_group_users_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupUsersError502")]
    pub list_group_users_error502: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupUsersParams")]
    pub list_group_users_params: ListGroupUsersParams,
    #[serde(rename = "ListGroupUsersResponse")]
    pub list_group_users_response: UserListResource,
    #[serde(rename = "RemoveGroupUserError400")]
    pub remove_group_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveGroupUserError403")]
    pub remove_group_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RemoveGroupUserError404")]
    pub remove_group_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveGroupUserError409")]
    pub remove_group_user_error409: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveGroupUserError429")]
    pub remove_group_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RemoveGroupUserParams")]
    pub remove_group_user_params: RemoveGroupUserParams,
    #[serde(rename = "RemoveGroupUserResponse")]
    pub remove_group_user_response: GroupUserDeletedResource,
    #[serde(rename = "RetrieveGroupUserError400")]
    pub retrieve_group_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupUserError403")]
    pub retrieve_group_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveGroupUserError404")]
    pub retrieve_group_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupUserError429")]
    pub retrieve_group_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupUserParams")]
    pub retrieve_group_user_params: RetrieveGroupUserParams,
    #[serde(rename = "RetrieveGroupUserResponse")]
    pub retrieve_group_user_response: GroupMemberUser,
}
impl GroupUsersApiSchema {
    pub fn builder() -> builder::GroupUsersApiSchema {
        Default::default()
    }
}
#[doc = "`ListGroupUsersParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListGroupUsersParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub group_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListGroupUsersParamsOrder>,
}
impl ListGroupUsersParams {
    pub fn builder() -> builder::ListGroupUsersParams {
        Default::default()
    }
}
#[doc = "`ListGroupUsersParamsOrder`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ListGroupUsersParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListGroupUsersParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListGroupUsersParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListGroupUsersParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListGroupUsersParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RemoveGroupUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RemoveGroupUserParams {
    pub group_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl RemoveGroupUserParams {
    pub fn builder() -> builder::RemoveGroupUserParams {
        Default::default()
    }
}
#[doc = "`RetrieveGroupUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveGroupUserParams {
    pub group_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl RetrieveGroupUserParams {
    pub fn builder() -> builder::RetrieveGroupUserParams {
        Default::default()
    }
}
#[doc = "Paginated list of user objects returned when inspecting group membership."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserListResource {
    #[doc = "Users in the current page."]
    pub data: ::std::vec::Vec<GroupUser>,
    #[doc = "Whether more users are available when paginating."]
    pub has_more: bool,
    #[doc = "Cursor to fetch the next page of results, or `null` when no further users are available."]
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub next: ::std::option::Option<::std::string::String>,
    #[doc = "Always `list`."]
    pub object: UserListResourceObject,
}
impl UserListResource {
    pub fn builder() -> builder::UserListResource {
        Default::default()
    }
}
#[doc = "Always `list`."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UserListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for UserListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for UserListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserListResourceObject {
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
    pub struct AddGroupUserParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AddGroupUserParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
            }
        }
    }
    impl AddGroupUserParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AddGroupUserParams> for super::AddGroupUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AddGroupUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
            })
        }
    }
    impl ::std::convert::From<super::AddGroupUserParams> for AddGroupUserParams {
        fn from(value: super::AddGroupUserParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateGroupUserBody {
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateGroupUserBody {
        fn default() -> Self {
            Self {
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl CreateGroupUserBody {
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
    impl ::std::convert::TryFrom<CreateGroupUserBody> for super::CreateGroupUserBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateGroupUserBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateGroupUserBody> for CreateGroupUserBody {
        fn from(value: super::CreateGroupUserBody) -> Self {
            Self {
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupMemberUser {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        is_service_account:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        picture: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_type: ::std::result::Result<super::GroupMemberUserUserType, ::std::string::String>,
    }
    impl ::std::default::Default for GroupMemberUser {
        fn default() -> Self {
            Self {
                email: Err("no value supplied for email".to_string()),
                id: Err("no value supplied for id".to_string()),
                is_service_account: Err("no value supplied for is_service_account".to_string()),
                name: Err("no value supplied for name".to_string()),
                picture: Err("no value supplied for picture".to_string()),
                user_type: Err("no value supplied for user_type".to_string()),
            }
        }
    }
    impl GroupMemberUser {
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
        pub fn is_service_account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_service_account = value.try_into().map_err(|e| {
                format!("error converting supplied value for is_service_account: {e}")
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
        pub fn picture<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.picture = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for picture: {e}"));
            self
        }
        pub fn user_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupMemberUserUserType>,
            T::Error: ::std::fmt::Display,
        {
            self.user_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupMemberUser> for super::GroupMemberUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupMemberUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                id: value.id?,
                is_service_account: value.is_service_account?,
                name: value.name?,
                picture: value.picture?,
                user_type: value.user_type?,
            })
        }
    }
    impl ::std::convert::From<super::GroupMemberUser> for GroupMemberUser {
        fn from(value: super::GroupMemberUser) -> Self {
            Self {
                email: Ok(value.email),
                id: Ok(value.id),
                is_service_account: Ok(value.is_service_account),
                name: Ok(value.name),
                picture: Ok(value.picture),
                user_type: Ok(value.user_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupUser {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupUser {
        fn default() -> Self {
            Self {
                email: Err("no value supplied for email".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl GroupUser {
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupUser> for super::GroupUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::GroupUser> for GroupUser {
        fn from(value: super::GroupUser) -> Self {
            Self {
                email: Ok(value.email),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupUserAssignment {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::GroupUserAssignmentObject, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupUserAssignment {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                object: Err("no value supplied for object".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl GroupUserAssignment {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupUserAssignmentObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
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
    impl ::std::convert::TryFrom<GroupUserAssignment> for super::GroupUserAssignment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupUserAssignment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                object: value.object?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::GroupUserAssignment> for GroupUserAssignment {
        fn from(value: super::GroupUserAssignment) -> Self {
            Self {
                group_id: Ok(value.group_id),
                object: Ok(value.object),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupUserDeletedResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        object: ::std::result::Result<super::GroupUserDeletedResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for GroupUserDeletedResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl GroupUserDeletedResource {
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
            T: ::std::convert::TryInto<super::GroupUserDeletedResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupUserDeletedResource> for super::GroupUserDeletedResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupUserDeletedResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::GroupUserDeletedResource> for GroupUserDeletedResource {
        fn from(value: super::GroupUserDeletedResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupUsersApiSchema {
        add_group_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_group_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        add_group_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_group_user_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_group_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        add_group_user_params:
            ::std::result::Result<super::AddGroupUserParams, ::std::string::String>,
        add_group_user_request:
            ::std::result::Result<super::CreateGroupUserBody, ::std::string::String>,
        add_group_user_response:
            ::std::result::Result<super::GroupUserAssignment, ::std::string::String>,
        list_group_users_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_users_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_group_users_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_users_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_users_error502:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_users_params:
            ::std::result::Result<super::ListGroupUsersParams, ::std::string::String>,
        list_group_users_response:
            ::std::result::Result<super::UserListResource, ::std::string::String>,
        remove_group_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_group_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        remove_group_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_group_user_error409:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_group_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        remove_group_user_params:
            ::std::result::Result<super::RemoveGroupUserParams, ::std::string::String>,
        remove_group_user_response:
            ::std::result::Result<super::GroupUserDeletedResource, ::std::string::String>,
        retrieve_group_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_group_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_user_params:
            ::std::result::Result<super::RetrieveGroupUserParams, ::std::string::String>,
        retrieve_group_user_response:
            ::std::result::Result<super::GroupMemberUser, ::std::string::String>,
    }
    impl ::std::default::Default for GroupUsersApiSchema {
        fn default() -> Self {
            Self {
                add_group_user_error400: Err(
                    "no value supplied for add_group_user_error400".to_string()
                ),
                add_group_user_error403: Err(
                    "no value supplied for add_group_user_error403".to_string()
                ),
                add_group_user_error404: Err(
                    "no value supplied for add_group_user_error404".to_string()
                ),
                add_group_user_error409: Err(
                    "no value supplied for add_group_user_error409".to_string()
                ),
                add_group_user_error429: Err(
                    "no value supplied for add_group_user_error429".to_string()
                ),
                add_group_user_params: Err(
                    "no value supplied for add_group_user_params".to_string()
                ),
                add_group_user_request: Err(
                    "no value supplied for add_group_user_request".to_string()
                ),
                add_group_user_response: Err(
                    "no value supplied for add_group_user_response".to_string()
                ),
                list_group_users_error400: Err(
                    "no value supplied for list_group_users_error400".to_string()
                ),
                list_group_users_error403: Err(
                    "no value supplied for list_group_users_error403".to_string()
                ),
                list_group_users_error404: Err(
                    "no value supplied for list_group_users_error404".to_string()
                ),
                list_group_users_error429: Err(
                    "no value supplied for list_group_users_error429".to_string()
                ),
                list_group_users_error502: Err(
                    "no value supplied for list_group_users_error502".to_string()
                ),
                list_group_users_params: Err(
                    "no value supplied for list_group_users_params".to_string()
                ),
                list_group_users_response: Err(
                    "no value supplied for list_group_users_response".to_string()
                ),
                remove_group_user_error400: Err(
                    "no value supplied for remove_group_user_error400".to_string()
                ),
                remove_group_user_error403: Err(
                    "no value supplied for remove_group_user_error403".to_string()
                ),
                remove_group_user_error404: Err(
                    "no value supplied for remove_group_user_error404".to_string()
                ),
                remove_group_user_error409: Err(
                    "no value supplied for remove_group_user_error409".to_string()
                ),
                remove_group_user_error429: Err(
                    "no value supplied for remove_group_user_error429".to_string()
                ),
                remove_group_user_params: Err(
                    "no value supplied for remove_group_user_params".to_string()
                ),
                remove_group_user_response: Err(
                    "no value supplied for remove_group_user_response".to_string()
                ),
                retrieve_group_user_error400: Err(
                    "no value supplied for retrieve_group_user_error400".to_string(),
                ),
                retrieve_group_user_error403: Err(
                    "no value supplied for retrieve_group_user_error403".to_string(),
                ),
                retrieve_group_user_error404: Err(
                    "no value supplied for retrieve_group_user_error404".to_string(),
                ),
                retrieve_group_user_error429: Err(
                    "no value supplied for retrieve_group_user_error429".to_string(),
                ),
                retrieve_group_user_params: Err(
                    "no value supplied for retrieve_group_user_params".to_string()
                ),
                retrieve_group_user_response: Err(
                    "no value supplied for retrieve_group_user_response".to_string(),
                ),
            }
        }
    }
    impl GroupUsersApiSchema {
        pub fn add_group_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_error400: {e}")
            });
            self
        }
        pub fn add_group_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_error403: {e}")
            });
            self
        }
        pub fn add_group_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_error404: {e}")
            });
            self
        }
        pub fn add_group_user_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_error409: {e}")
            });
            self
        }
        pub fn add_group_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_error429: {e}")
            });
            self
        }
        pub fn add_group_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AddGroupUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_params: {e}")
            });
            self
        }
        pub fn add_group_user_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateGroupUserBody>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_request: {e}")
            });
            self
        }
        pub fn add_group_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupUserAssignment>,
            T::Error: ::std::fmt::Display,
        {
            self.add_group_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for add_group_user_response: {e}")
            });
            self
        }
        pub fn list_group_users_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_error400: {e}")
            });
            self
        }
        pub fn list_group_users_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_error403: {e}")
            });
            self
        }
        pub fn list_group_users_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_error404: {e}")
            });
            self
        }
        pub fn list_group_users_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_error429: {e}")
            });
            self
        }
        pub fn list_group_users_error502<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_error502 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_error502: {e}")
            });
            self
        }
        pub fn list_group_users_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListGroupUsersParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_params: {e}")
            });
            self
        }
        pub fn list_group_users_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_users_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_group_users_response: {e}")
            });
            self
        }
        pub fn remove_group_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_error400: {e}")
            });
            self
        }
        pub fn remove_group_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_error403: {e}")
            });
            self
        }
        pub fn remove_group_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_error404: {e}")
            });
            self
        }
        pub fn remove_group_user_error409<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_error409 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_error409: {e}")
            });
            self
        }
        pub fn remove_group_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_error429: {e}")
            });
            self
        }
        pub fn remove_group_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RemoveGroupUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_params: {e}")
            });
            self
        }
        pub fn remove_group_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupUserDeletedResource>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_group_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for remove_group_user_response: {e}")
            });
            self
        }
        pub fn retrieve_group_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_error400: {e}")
            });
            self
        }
        pub fn retrieve_group_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_error403: {e}")
            });
            self
        }
        pub fn retrieve_group_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_error404: {e}")
            });
            self
        }
        pub fn retrieve_group_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_error429: {e}")
            });
            self
        }
        pub fn retrieve_group_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveGroupUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_params: {e}")
            });
            self
        }
        pub fn retrieve_group_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupMemberUser>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_user_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GroupUsersApiSchema> for super::GroupUsersApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupUsersApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                add_group_user_error400: value.add_group_user_error400?,
                add_group_user_error403: value.add_group_user_error403?,
                add_group_user_error404: value.add_group_user_error404?,
                add_group_user_error409: value.add_group_user_error409?,
                add_group_user_error429: value.add_group_user_error429?,
                add_group_user_params: value.add_group_user_params?,
                add_group_user_request: value.add_group_user_request?,
                add_group_user_response: value.add_group_user_response?,
                list_group_users_error400: value.list_group_users_error400?,
                list_group_users_error403: value.list_group_users_error403?,
                list_group_users_error404: value.list_group_users_error404?,
                list_group_users_error429: value.list_group_users_error429?,
                list_group_users_error502: value.list_group_users_error502?,
                list_group_users_params: value.list_group_users_params?,
                list_group_users_response: value.list_group_users_response?,
                remove_group_user_error400: value.remove_group_user_error400?,
                remove_group_user_error403: value.remove_group_user_error403?,
                remove_group_user_error404: value.remove_group_user_error404?,
                remove_group_user_error409: value.remove_group_user_error409?,
                remove_group_user_error429: value.remove_group_user_error429?,
                remove_group_user_params: value.remove_group_user_params?,
                remove_group_user_response: value.remove_group_user_response?,
                retrieve_group_user_error400: value.retrieve_group_user_error400?,
                retrieve_group_user_error403: value.retrieve_group_user_error403?,
                retrieve_group_user_error404: value.retrieve_group_user_error404?,
                retrieve_group_user_error429: value.retrieve_group_user_error429?,
                retrieve_group_user_params: value.retrieve_group_user_params?,
                retrieve_group_user_response: value.retrieve_group_user_response?,
            })
        }
    }
    impl ::std::convert::From<super::GroupUsersApiSchema> for GroupUsersApiSchema {
        fn from(value: super::GroupUsersApiSchema) -> Self {
            Self {
                add_group_user_error400: Ok(value.add_group_user_error400),
                add_group_user_error403: Ok(value.add_group_user_error403),
                add_group_user_error404: Ok(value.add_group_user_error404),
                add_group_user_error409: Ok(value.add_group_user_error409),
                add_group_user_error429: Ok(value.add_group_user_error429),
                add_group_user_params: Ok(value.add_group_user_params),
                add_group_user_request: Ok(value.add_group_user_request),
                add_group_user_response: Ok(value.add_group_user_response),
                list_group_users_error400: Ok(value.list_group_users_error400),
                list_group_users_error403: Ok(value.list_group_users_error403),
                list_group_users_error404: Ok(value.list_group_users_error404),
                list_group_users_error429: Ok(value.list_group_users_error429),
                list_group_users_error502: Ok(value.list_group_users_error502),
                list_group_users_params: Ok(value.list_group_users_params),
                list_group_users_response: Ok(value.list_group_users_response),
                remove_group_user_error400: Ok(value.remove_group_user_error400),
                remove_group_user_error403: Ok(value.remove_group_user_error403),
                remove_group_user_error404: Ok(value.remove_group_user_error404),
                remove_group_user_error409: Ok(value.remove_group_user_error409),
                remove_group_user_error429: Ok(value.remove_group_user_error429),
                remove_group_user_params: Ok(value.remove_group_user_params),
                remove_group_user_response: Ok(value.remove_group_user_response),
                retrieve_group_user_error400: Ok(value.retrieve_group_user_error400),
                retrieve_group_user_error403: Ok(value.retrieve_group_user_error403),
                retrieve_group_user_error404: Ok(value.retrieve_group_user_error404),
                retrieve_group_user_error429: Ok(value.retrieve_group_user_error429),
                retrieve_group_user_params: Ok(value.retrieve_group_user_params),
                retrieve_group_user_response: Ok(value.retrieve_group_user_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListGroupUsersParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListGroupUsersParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListGroupUsersParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                group_id: Err("no value supplied for group_id".to_string()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListGroupUsersParams {
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
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::ListGroupUsersParamsOrder>>,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListGroupUsersParams> for super::ListGroupUsersParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListGroupUsersParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                group_id: value.group_id?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListGroupUsersParams> for ListGroupUsersParams {
        fn from(value: super::ListGroupUsersParams) -> Self {
            Self {
                after: Ok(value.after),
                group_id: Ok(value.group_id),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RemoveGroupUserParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RemoveGroupUserParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RemoveGroupUserParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
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
    impl ::std::convert::TryFrom<RemoveGroupUserParams> for super::RemoveGroupUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RemoveGroupUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RemoveGroupUserParams> for RemoveGroupUserParams {
        fn from(value: super::RemoveGroupUserParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveGroupUserParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveGroupUserParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RetrieveGroupUserParams {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {e}"));
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
    impl ::std::convert::TryFrom<RetrieveGroupUserParams> for super::RetrieveGroupUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveGroupUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveGroupUserParams> for RetrieveGroupUserParams {
        fn from(value: super::RetrieveGroupUserParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserListResource {
        data: ::std::result::Result<::std::vec::Vec<super::GroupUser>, ::std::string::String>,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        next: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::UserListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for UserListResource {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                has_more: Err("no value supplied for has_more".to_string()),
                next: Err("no value supplied for next".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl UserListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::GroupUser>>,
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
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserListResource> for super::UserListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserListResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                has_more: value.has_more?,
                next: value.next?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::UserListResource> for UserListResource {
        fn from(value: super::UserListResource) -> Self {
            Self {
                data: Ok(value.data),
                has_more: Ok(value.has_more),
                next: Ok(value.next),
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
