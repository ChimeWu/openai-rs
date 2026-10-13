pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, User, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`DeleteUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteUserParams {
    pub user_id: ::std::string::String,
}
impl DeleteUserParams {
    pub fn builder() -> builder::DeleteUserParams {
        Default::default()
    }
}
#[doc = "`ListUsersParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListUsersParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub emails: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
}
impl ListUsersParams {
    pub fn builder() -> builder::ListUsersParams {
        Default::default()
    }
}
#[doc = "`ModifyUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ModifyUserParams {
    pub user_id: ::std::string::String,
}
impl ModifyUserParams {
    pub fn builder() -> builder::ModifyUserParams {
        Default::default()
    }
}
#[doc = "`RetrieveUserParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveUserParams {
    pub user_id: ::std::string::String,
}
impl RetrieveUserParams {
    pub fn builder() -> builder::RetrieveUserParams {
        Default::default()
    }
}
#[doc = "`UserDeleteResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserDeleteResponse {
    pub deleted: bool,
    pub id: ::std::string::String,
    pub object: UserDeleteResponseObject,
}
impl UserDeleteResponse {
    pub fn builder() -> builder::UserDeleteResponse {
        Default::default()
    }
}
#[doc = "`UserDeleteResponseObject`"]
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
pub enum UserDeleteResponseObject {
    #[serde(rename = "organization.user.deleted")]
    OrganizationUserDeleted,
}
impl ::std::fmt::Display for UserDeleteResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrganizationUserDeleted => f.write_str("organization.user.deleted"),
        }
    }
}
impl ::std::str::FromStr for UserDeleteResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "organization.user.deleted" => Ok(Self::OrganizationUserDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserDeleteResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UserListResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserListResponse {
    pub data: ::std::vec::Vec<::openai_api_types::common::User>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub first_id: ::std::option::Option<::std::string::String>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_id: ::std::option::Option<::std::string::String>,
    pub object: UserListResponseObject,
}
impl UserListResponse {
    pub fn builder() -> builder::UserListResponse {
        Default::default()
    }
}
#[doc = "`UserListResponseObject`"]
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
pub enum UserListResponseObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for UserListResponseObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for UserListResponseObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UserListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UserListResponseObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UserRoleUpdateRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct UserRoleUpdateRequest {
    #[doc = "Developer persona metadata."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub developer_persona: ::std::option::Option<::std::string::String>,
    #[doc = "`owner` or `reader`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[doc = "Role ID to assign to the user."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role_id: ::std::option::Option<::std::string::String>,
    #[doc = "Technical level metadata."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub technical_level: ::std::option::Option<::std::string::String>,
}
impl UserRoleUpdateRequest {
    pub fn builder() -> builder::UserRoleUpdateRequest {
        Default::default()
    }
}
#[doc = "`UsersApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UsersApiSchema {
    #[serde(rename = "DeleteUserError400")]
    pub delete_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteUserError403")]
    pub delete_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "DeleteUserError404")]
    pub delete_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteUserError429")]
    pub delete_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteUserParams")]
    pub delete_user_params: DeleteUserParams,
    #[serde(rename = "DeleteUserResponse")]
    pub delete_user_response: UserDeleteResponse,
    #[serde(rename = "ListUsersError400")]
    pub list_users_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListUsersError403")]
    pub list_users_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListUsersError429")]
    pub list_users_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListUsersParams")]
    pub list_users_params: ListUsersParams,
    #[serde(rename = "ListUsersResponse")]
    pub list_users_response: UserListResponse,
    #[serde(rename = "ModifyUserError400")]
    pub modify_user_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyUserError403")]
    pub modify_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ModifyUserError404")]
    pub modify_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyUserError429")]
    pub modify_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ModifyUserParams")]
    pub modify_user_params: ModifyUserParams,
    #[serde(rename = "ModifyUserRequest")]
    pub modify_user_request: UserRoleUpdateRequest,
    #[serde(rename = "ModifyUserResponse")]
    pub modify_user_response: ::openai_api_types::common::User,
    #[serde(rename = "RetrieveUserError403")]
    pub retrieve_user_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveUserError404")]
    pub retrieve_user_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveUserError429")]
    pub retrieve_user_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveUserParams")]
    pub retrieve_user_params: RetrieveUserParams,
    #[serde(rename = "RetrieveUserResponse")]
    pub retrieve_user_response: ::openai_api_types::common::User,
}
impl UsersApiSchema {
    pub fn builder() -> builder::UsersApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DeleteUserParams {
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteUserParams {
        fn default() -> Self {
            Self {
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl DeleteUserParams {
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
    impl ::std::convert::TryFrom<DeleteUserParams> for super::DeleteUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteUserParams> for DeleteUserParams {
        fn from(value: super::DeleteUserParams) -> Self {
            Self {
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListUsersParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        emails:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for ListUsersParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                emails: Ok(Default::default()),
                limit: Ok(Default::default()),
            }
        }
    }
    impl ListUsersParams {
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
        pub fn emails<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.emails = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emails: {e}"));
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
    impl ::std::convert::TryFrom<ListUsersParams> for super::ListUsersParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListUsersParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                emails: value.emails?,
                limit: value.limit?,
            })
        }
    }
    impl ::std::convert::From<super::ListUsersParams> for ListUsersParams {
        fn from(value: super::ListUsersParams) -> Self {
            Self {
                after: Ok(value.after),
                emails: Ok(value.emails),
                limit: Ok(value.limit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ModifyUserParams {
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ModifyUserParams {
        fn default() -> Self {
            Self {
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl ModifyUserParams {
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
    impl ::std::convert::TryFrom<ModifyUserParams> for super::ModifyUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ModifyUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::ModifyUserParams> for ModifyUserParams {
        fn from(value: super::ModifyUserParams) -> Self {
            Self {
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveUserParams {
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveUserParams {
        fn default() -> Self {
            Self {
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RetrieveUserParams {
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
    impl ::std::convert::TryFrom<RetrieveUserParams> for super::RetrieveUserParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveUserParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveUserParams> for RetrieveUserParams {
        fn from(value: super::RetrieveUserParams) -> Self {
            Self {
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserDeleteResponse {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::UserDeleteResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for UserDeleteResponse {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl UserDeleteResponse {
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
            T: ::std::convert::TryInto<super::UserDeleteResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserDeleteResponse> for super::UserDeleteResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserDeleteResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::UserDeleteResponse> for UserDeleteResponse {
        fn from(value: super::UserDeleteResponse) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserListResponse {
        data: ::std::result::Result<
            ::std::vec::Vec<::openai_api_types::common::User>,
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
        object: ::std::result::Result<super::UserListResponseObject, ::std::string::String>,
    }
    impl ::std::default::Default for UserListResponse {
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
    impl UserListResponse {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::openai_api_types::common::User>>,
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
            T: ::std::convert::TryInto<super::UserListResponseObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserListResponse> for super::UserListResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserListResponse,
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
    impl ::std::convert::From<super::UserListResponse> for UserListResponse {
        fn from(value: super::UserListResponse) -> Self {
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
    pub struct UserRoleUpdateRequest {
        developer_persona: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        role_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        technical_level: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UserRoleUpdateRequest {
        fn default() -> Self {
            Self {
                developer_persona: Ok(Default::default()),
                role: Ok(Default::default()),
                role_id: Ok(Default::default()),
                technical_level: Ok(Default::default()),
            }
        }
    }
    impl UserRoleUpdateRequest {
        pub fn developer_persona<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.developer_persona = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for developer_persona: {e}"));
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
        pub fn technical_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.technical_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for technical_level: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UserRoleUpdateRequest> for super::UserRoleUpdateRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserRoleUpdateRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                developer_persona: value.developer_persona?,
                role: value.role?,
                role_id: value.role_id?,
                technical_level: value.technical_level?,
            })
        }
    }
    impl ::std::convert::From<super::UserRoleUpdateRequest> for UserRoleUpdateRequest {
        fn from(value: super::UserRoleUpdateRequest) -> Self {
            Self {
                developer_persona: Ok(value.developer_persona),
                role: Ok(value.role),
                role_id: Ok(value.role_id),
                technical_level: Ok(value.technical_level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UsersApiSchema {
        delete_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        delete_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_user_params: ::std::result::Result<super::DeleteUserParams, ::std::string::String>,
        delete_user_response:
            ::std::result::Result<super::UserDeleteResponse, ::std::string::String>,
        list_users_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_users_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_users_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_users_params: ::std::result::Result<super::ListUsersParams, ::std::string::String>,
        list_users_response: ::std::result::Result<super::UserListResponse, ::std::string::String>,
        modify_user_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        modify_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        modify_user_params: ::std::result::Result<super::ModifyUserParams, ::std::string::String>,
        modify_user_request:
            ::std::result::Result<super::UserRoleUpdateRequest, ::std::string::String>,
        modify_user_response:
            ::std::result::Result<::openai_api_types::common::User, ::std::string::String>,
        retrieve_user_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_user_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_user_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_user_params:
            ::std::result::Result<super::RetrieveUserParams, ::std::string::String>,
        retrieve_user_response:
            ::std::result::Result<::openai_api_types::common::User, ::std::string::String>,
    }
    impl ::std::default::Default for UsersApiSchema {
        fn default() -> Self {
            Self {
                delete_user_error400: Err("no value supplied for delete_user_error400".to_string()),
                delete_user_error403: Err("no value supplied for delete_user_error403".to_string()),
                delete_user_error404: Err("no value supplied for delete_user_error404".to_string()),
                delete_user_error429: Err("no value supplied for delete_user_error429".to_string()),
                delete_user_params: Err("no value supplied for delete_user_params".to_string()),
                delete_user_response: Err("no value supplied for delete_user_response".to_string()),
                list_users_error400: Err("no value supplied for list_users_error400".to_string()),
                list_users_error403: Err("no value supplied for list_users_error403".to_string()),
                list_users_error429: Err("no value supplied for list_users_error429".to_string()),
                list_users_params: Err("no value supplied for list_users_params".to_string()),
                list_users_response: Err("no value supplied for list_users_response".to_string()),
                modify_user_error400: Err("no value supplied for modify_user_error400".to_string()),
                modify_user_error403: Err("no value supplied for modify_user_error403".to_string()),
                modify_user_error404: Err("no value supplied for modify_user_error404".to_string()),
                modify_user_error429: Err("no value supplied for modify_user_error429".to_string()),
                modify_user_params: Err("no value supplied for modify_user_params".to_string()),
                modify_user_request: Err("no value supplied for modify_user_request".to_string()),
                modify_user_response: Err("no value supplied for modify_user_response".to_string()),
                retrieve_user_error403: Err(
                    "no value supplied for retrieve_user_error403".to_string()
                ),
                retrieve_user_error404: Err(
                    "no value supplied for retrieve_user_error404".to_string()
                ),
                retrieve_user_error429: Err(
                    "no value supplied for retrieve_user_error429".to_string()
                ),
                retrieve_user_params: Err("no value supplied for retrieve_user_params".to_string()),
                retrieve_user_response: Err(
                    "no value supplied for retrieve_user_response".to_string()
                ),
            }
        }
    }
    impl UsersApiSchema {
        pub fn delete_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_error400: {e}")
            });
            self
        }
        pub fn delete_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_error403: {e}")
            });
            self
        }
        pub fn delete_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_error404: {e}")
            });
            self
        }
        pub fn delete_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_error429: {e}")
            });
            self
        }
        pub fn delete_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_params: {e}")
            });
            self
        }
        pub fn delete_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserDeleteResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_user_response: {e}")
            });
            self
        }
        pub fn list_users_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_users_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_users_error400: {e}")
            });
            self
        }
        pub fn list_users_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_users_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_users_error403: {e}")
            });
            self
        }
        pub fn list_users_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_users_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_users_error429: {e}")
            });
            self
        }
        pub fn list_users_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListUsersParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_users_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list_users_params: {e}"));
            self
        }
        pub fn list_users_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserListResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_users_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_users_response: {e}")
            });
            self
        }
        pub fn modify_user_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_error400: {e}")
            });
            self
        }
        pub fn modify_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_error403: {e}")
            });
            self
        }
        pub fn modify_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_error404: {e}")
            });
            self
        }
        pub fn modify_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_error429: {e}")
            });
            self
        }
        pub fn modify_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ModifyUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_params: {e}")
            });
            self
        }
        pub fn modify_user_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UserRoleUpdateRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_request: {e}")
            });
            self
        }
        pub fn modify_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::User>,
            T::Error: ::std::fmt::Display,
        {
            self.modify_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for modify_user_response: {e}")
            });
            self
        }
        pub fn retrieve_user_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_error403: {e}")
            });
            self
        }
        pub fn retrieve_user_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_error404: {e}")
            });
            self
        }
        pub fn retrieve_user_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_error429: {e}")
            });
            self
        }
        pub fn retrieve_user_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveUserParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_params: {e}")
            });
            self
        }
        pub fn retrieve_user_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::User>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UsersApiSchema> for super::UsersApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UsersApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                delete_user_error400: value.delete_user_error400?,
                delete_user_error403: value.delete_user_error403?,
                delete_user_error404: value.delete_user_error404?,
                delete_user_error429: value.delete_user_error429?,
                delete_user_params: value.delete_user_params?,
                delete_user_response: value.delete_user_response?,
                list_users_error400: value.list_users_error400?,
                list_users_error403: value.list_users_error403?,
                list_users_error429: value.list_users_error429?,
                list_users_params: value.list_users_params?,
                list_users_response: value.list_users_response?,
                modify_user_error400: value.modify_user_error400?,
                modify_user_error403: value.modify_user_error403?,
                modify_user_error404: value.modify_user_error404?,
                modify_user_error429: value.modify_user_error429?,
                modify_user_params: value.modify_user_params?,
                modify_user_request: value.modify_user_request?,
                modify_user_response: value.modify_user_response?,
                retrieve_user_error403: value.retrieve_user_error403?,
                retrieve_user_error404: value.retrieve_user_error404?,
                retrieve_user_error429: value.retrieve_user_error429?,
                retrieve_user_params: value.retrieve_user_params?,
                retrieve_user_response: value.retrieve_user_response?,
            })
        }
    }
    impl ::std::convert::From<super::UsersApiSchema> for UsersApiSchema {
        fn from(value: super::UsersApiSchema) -> Self {
            Self {
                delete_user_error400: Ok(value.delete_user_error400),
                delete_user_error403: Ok(value.delete_user_error403),
                delete_user_error404: Ok(value.delete_user_error404),
                delete_user_error429: Ok(value.delete_user_error429),
                delete_user_params: Ok(value.delete_user_params),
                delete_user_response: Ok(value.delete_user_response),
                list_users_error400: Ok(value.list_users_error400),
                list_users_error403: Ok(value.list_users_error403),
                list_users_error429: Ok(value.list_users_error429),
                list_users_params: Ok(value.list_users_params),
                list_users_response: Ok(value.list_users_response),
                modify_user_error400: Ok(value.modify_user_error400),
                modify_user_error403: Ok(value.modify_user_error403),
                modify_user_error404: Ok(value.modify_user_error404),
                modify_user_error429: Ok(value.modify_user_error429),
                modify_user_params: Ok(value.modify_user_params),
                modify_user_request: Ok(value.modify_user_request),
                modify_user_response: Ok(value.modify_user_response),
                retrieve_user_error403: Ok(value.retrieve_user_error403),
                retrieve_user_error404: Ok(value.retrieve_user_error404),
                retrieve_user_error429: Ok(value.retrieve_user_error429),
                retrieve_user_params: Ok(value.retrieve_user_params),
                retrieve_user_response: Ok(value.retrieve_user_response),
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
