pub use crate::common::{AssignedRoleDetails, DeletedRoleAssignmentResource, Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, PublicAssignOrganizationGroupRoleBody, Role, RoleListResource, User, UserRoleAssignment, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AssignUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssignUserRoleParams {
    pub user_id: ::std::string::String,
}
impl AssignUserRoleParams {
    pub fn builder() -> builder::AssignUserRoleParams {
        Default::default()
    }
}
#[doc = "`ListUserRoleAssignmentsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListUserRoleAssignmentsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListUserRoleAssignmentsParamsOrder>,
    pub user_id: ::std::string::String,
}
impl ListUserRoleAssignmentsParams {
    pub fn builder() -> builder::ListUserRoleAssignmentsParams {
        Default::default()
    }
}
#[doc = "`ListUserRoleAssignmentsParamsOrder`"]
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
pub enum ListUserRoleAssignmentsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListUserRoleAssignmentsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListUserRoleAssignmentsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListUserRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListUserRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveUserRoleParams {
    pub role_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl RetrieveUserRoleParams {
    pub fn builder() -> builder::RetrieveUserRoleParams {
        Default::default()
    }
}
#[doc = "`UnassignUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UnassignUserRoleParams {
    pub role_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl UnassignUserRoleParams {
    pub fn builder() -> builder::UnassignUserRoleParams {
        Default::default()
    }
}
#[doc = "`UserOrganizationRoleAssignmentsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UserOrganizationRoleAssignmentsApiSchema {
    #[serde(rename = "AssignUserRoleError400")]
    pub assign_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignUserRoleError403")]
    pub assign_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AssignUserRoleError404")]
    pub assign_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignUserRoleError429")]
    pub assign_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignUserRoleParams")]
    pub assign_user_role_params: AssignUserRoleParams,
    #[serde(rename = "AssignUserRoleRequest")]
    pub assign_user_role_request: ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
    #[serde(rename = "AssignUserRoleResponse")]
    pub assign_user_role_response: ::openai_api_types::common::UserRoleAssignment,
    #[serde(rename = "ListUserRoleAssignmentsError400")]
    pub list_user_role_assignments_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListUserRoleAssignmentsError403")]
    pub list_user_role_assignments_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListUserRoleAssignmentsError404")]
    pub list_user_role_assignments_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListUserRoleAssignmentsError429")]
    pub list_user_role_assignments_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListUserRoleAssignmentsParams")]
    pub list_user_role_assignments_params: ListUserRoleAssignmentsParams,
    #[serde(rename = "ListUserRoleAssignmentsResponse")]
    pub list_user_role_assignments_response: ::openai_api_types::common::RoleListResource,
    #[serde(rename = "RetrieveUserRoleError400")]
    pub retrieve_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveUserRoleError403")]
    pub retrieve_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveUserRoleError404")]
    pub retrieve_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveUserRoleError429")]
    pub retrieve_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveUserRoleParams")]
    pub retrieve_user_role_params: RetrieveUserRoleParams,
    #[serde(rename = "RetrieveUserRoleResponse")]
    pub retrieve_user_role_response: ::openai_api_types::common::AssignedRoleDetails,
    #[serde(rename = "UnassignUserRoleError400")]
    pub unassign_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignUserRoleError403")]
    pub unassign_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UnassignUserRoleError404")]
    pub unassign_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignUserRoleError429")]
    pub unassign_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignUserRoleParams")]
    pub unassign_user_role_params: UnassignUserRoleParams,
    #[serde(rename = "UnassignUserRoleResponse")]
    pub unassign_user_role_response: ::openai_api_types::common::DeletedRoleAssignmentResource,
}
impl UserOrganizationRoleAssignmentsApiSchema {
    pub fn builder() -> builder::UserOrganizationRoleAssignmentsApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AssignUserRoleParams {
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AssignUserRoleParams {
        fn default() -> Self {
            Self {
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl AssignUserRoleParams {
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
    impl ::std::convert::TryFrom<AssignUserRoleParams> for super::AssignUserRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssignUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::AssignUserRoleParams> for AssignUserRoleParams {
        fn from(value: super::AssignUserRoleParams) -> Self {
            Self {
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListUserRoleAssignmentsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListUserRoleAssignmentsParamsOrder>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListUserRoleAssignmentsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl ListUserRoleAssignmentsParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListUserRoleAssignmentsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
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
    impl ::std::convert::TryFrom<ListUserRoleAssignmentsParams>
        for super::ListUserRoleAssignmentsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListUserRoleAssignmentsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListUserRoleAssignmentsParams> for ListUserRoleAssignmentsParams {
        fn from(value: super::ListUserRoleAssignmentsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveUserRoleParams {
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveUserRoleParams {
        fn default() -> Self {
            Self {
                role_id: Err("no value supplied for role_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RetrieveUserRoleParams {
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
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
    impl ::std::convert::TryFrom<RetrieveUserRoleParams> for super::RetrieveUserRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                role_id: value.role_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveUserRoleParams> for RetrieveUserRoleParams {
        fn from(value: super::RetrieveUserRoleParams) -> Self {
            Self {
                role_id: Ok(value.role_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnassignUserRoleParams {
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnassignUserRoleParams {
        fn default() -> Self {
            Self {
                role_id: Err("no value supplied for role_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl UnassignUserRoleParams {
        pub fn role_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role_id: {e}"));
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
    impl ::std::convert::TryFrom<UnassignUserRoleParams> for super::UnassignUserRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnassignUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                role_id: value.role_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UnassignUserRoleParams> for UnassignUserRoleParams {
        fn from(value: super::UnassignUserRoleParams) -> Self {
            Self {
                role_id: Ok(value.role_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserOrganizationRoleAssignmentsApiSchema {
        assign_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        assign_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_user_role_params:
            ::std::result::Result<super::AssignUserRoleParams, ::std::string::String>,
        assign_user_role_request: ::std::result::Result<
            ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            ::std::string::String,
        >,
        assign_user_role_response: ::std::result::Result<
            ::openai_api_types::common::UserRoleAssignment,
            ::std::string::String,
        >,
        list_user_role_assignments_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_user_role_assignments_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_user_role_assignments_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_user_role_assignments_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_user_role_assignments_params:
            ::std::result::Result<super::ListUserRoleAssignmentsParams, ::std::string::String>,
        list_user_role_assignments_response: ::std::result::Result<
            ::openai_api_types::common::RoleListResource,
            ::std::string::String,
        >,
        retrieve_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_user_role_params:
            ::std::result::Result<super::RetrieveUserRoleParams, ::std::string::String>,
        retrieve_user_role_response: ::std::result::Result<
            ::openai_api_types::common::AssignedRoleDetails,
            ::std::string::String,
        >,
        unassign_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        unassign_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_user_role_params:
            ::std::result::Result<super::UnassignUserRoleParams, ::std::string::String>,
        unassign_user_role_response: ::std::result::Result<
            ::openai_api_types::common::DeletedRoleAssignmentResource,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UserOrganizationRoleAssignmentsApiSchema {
        fn default() -> Self {
            Self {
                assign_user_role_error400: Err(
                    "no value supplied for assign_user_role_error400".to_string()
                ),
                assign_user_role_error403: Err(
                    "no value supplied for assign_user_role_error403".to_string()
                ),
                assign_user_role_error404: Err(
                    "no value supplied for assign_user_role_error404".to_string()
                ),
                assign_user_role_error429: Err(
                    "no value supplied for assign_user_role_error429".to_string()
                ),
                assign_user_role_params: Err(
                    "no value supplied for assign_user_role_params".to_string()
                ),
                assign_user_role_request: Err(
                    "no value supplied for assign_user_role_request".to_string()
                ),
                assign_user_role_response: Err(
                    "no value supplied for assign_user_role_response".to_string()
                ),
                list_user_role_assignments_error400: Err(
                    "no value supplied for list_user_role_assignments_error400".to_string(),
                ),
                list_user_role_assignments_error403: Err(
                    "no value supplied for list_user_role_assignments_error403".to_string(),
                ),
                list_user_role_assignments_error404: Err(
                    "no value supplied for list_user_role_assignments_error404".to_string(),
                ),
                list_user_role_assignments_error429: Err(
                    "no value supplied for list_user_role_assignments_error429".to_string(),
                ),
                list_user_role_assignments_params: Err(
                    "no value supplied for list_user_role_assignments_params".to_string(),
                ),
                list_user_role_assignments_response: Err(
                    "no value supplied for list_user_role_assignments_response".to_string(),
                ),
                retrieve_user_role_error400: Err(
                    "no value supplied for retrieve_user_role_error400".to_string(),
                ),
                retrieve_user_role_error403: Err(
                    "no value supplied for retrieve_user_role_error403".to_string(),
                ),
                retrieve_user_role_error404: Err(
                    "no value supplied for retrieve_user_role_error404".to_string(),
                ),
                retrieve_user_role_error429: Err(
                    "no value supplied for retrieve_user_role_error429".to_string(),
                ),
                retrieve_user_role_params: Err(
                    "no value supplied for retrieve_user_role_params".to_string()
                ),
                retrieve_user_role_response: Err(
                    "no value supplied for retrieve_user_role_response".to_string(),
                ),
                unassign_user_role_error400: Err(
                    "no value supplied for unassign_user_role_error400".to_string(),
                ),
                unassign_user_role_error403: Err(
                    "no value supplied for unassign_user_role_error403".to_string(),
                ),
                unassign_user_role_error404: Err(
                    "no value supplied for unassign_user_role_error404".to_string(),
                ),
                unassign_user_role_error429: Err(
                    "no value supplied for unassign_user_role_error429".to_string(),
                ),
                unassign_user_role_params: Err(
                    "no value supplied for unassign_user_role_params".to_string()
                ),
                unassign_user_role_response: Err(
                    "no value supplied for unassign_user_role_response".to_string(),
                ),
            }
        }
    }
    impl UserOrganizationRoleAssignmentsApiSchema {
        pub fn assign_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_error400: {e}")
            });
            self
        }
        pub fn assign_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_error403: {e}")
            });
            self
        }
        pub fn assign_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_error404: {e}")
            });
            self
        }
        pub fn assign_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_error429: {e}")
            });
            self
        }
        pub fn assign_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssignUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_params: {e}")
            });
            self
        }
        pub fn assign_user_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_request: {e}")
            });
            self
        }
        pub fn assign_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::UserRoleAssignment>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_user_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_user_role_response: {e}")
            });
            self
        }
        pub fn list_user_role_assignments_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_error400: {e}"
                )
            });
            self
        }
        pub fn list_user_role_assignments_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_error403: {e}"
                )
            });
            self
        }
        pub fn list_user_role_assignments_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_error404: {e}"
                )
            });
            self
        }
        pub fn list_user_role_assignments_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_error429: {e}"
                )
            });
            self
        }
        pub fn list_user_role_assignments_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListUserRoleAssignmentsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_params: {e}"
                )
            });
            self
        }
        pub fn list_user_role_assignments_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::RoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_user_role_assignments_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_user_role_assignments_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_error400: {e}")
            });
            self
        }
        pub fn retrieve_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_error403: {e}")
            });
            self
        }
        pub fn retrieve_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_error404: {e}")
            });
            self
        }
        pub fn retrieve_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_error429: {e}")
            });
            self
        }
        pub fn retrieve_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_params: {e}")
            });
            self
        }
        pub fn retrieve_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::AssignedRoleDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_user_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_user_role_response: {e}")
            });
            self
        }
        pub fn unassign_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_error400: {e}")
            });
            self
        }
        pub fn unassign_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_error403: {e}")
            });
            self
        }
        pub fn unassign_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_error404: {e}")
            });
            self
        }
        pub fn unassign_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_error429: {e}")
            });
            self
        }
        pub fn unassign_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnassignUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_params: {e}")
            });
            self
        }
        pub fn unassign_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::DeletedRoleAssignmentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_user_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_user_role_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UserOrganizationRoleAssignmentsApiSchema>
        for super::UserOrganizationRoleAssignmentsApiSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UserOrganizationRoleAssignmentsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assign_user_role_error400: value.assign_user_role_error400?,
                assign_user_role_error403: value.assign_user_role_error403?,
                assign_user_role_error404: value.assign_user_role_error404?,
                assign_user_role_error429: value.assign_user_role_error429?,
                assign_user_role_params: value.assign_user_role_params?,
                assign_user_role_request: value.assign_user_role_request?,
                assign_user_role_response: value.assign_user_role_response?,
                list_user_role_assignments_error400: value.list_user_role_assignments_error400?,
                list_user_role_assignments_error403: value.list_user_role_assignments_error403?,
                list_user_role_assignments_error404: value.list_user_role_assignments_error404?,
                list_user_role_assignments_error429: value.list_user_role_assignments_error429?,
                list_user_role_assignments_params: value.list_user_role_assignments_params?,
                list_user_role_assignments_response: value.list_user_role_assignments_response?,
                retrieve_user_role_error400: value.retrieve_user_role_error400?,
                retrieve_user_role_error403: value.retrieve_user_role_error403?,
                retrieve_user_role_error404: value.retrieve_user_role_error404?,
                retrieve_user_role_error429: value.retrieve_user_role_error429?,
                retrieve_user_role_params: value.retrieve_user_role_params?,
                retrieve_user_role_response: value.retrieve_user_role_response?,
                unassign_user_role_error400: value.unassign_user_role_error400?,
                unassign_user_role_error403: value.unassign_user_role_error403?,
                unassign_user_role_error404: value.unassign_user_role_error404?,
                unassign_user_role_error429: value.unassign_user_role_error429?,
                unassign_user_role_params: value.unassign_user_role_params?,
                unassign_user_role_response: value.unassign_user_role_response?,
            })
        }
    }
    impl ::std::convert::From<super::UserOrganizationRoleAssignmentsApiSchema>
        for UserOrganizationRoleAssignmentsApiSchema
    {
        fn from(value: super::UserOrganizationRoleAssignmentsApiSchema) -> Self {
            Self {
                assign_user_role_error400: Ok(value.assign_user_role_error400),
                assign_user_role_error403: Ok(value.assign_user_role_error403),
                assign_user_role_error404: Ok(value.assign_user_role_error404),
                assign_user_role_error429: Ok(value.assign_user_role_error429),
                assign_user_role_params: Ok(value.assign_user_role_params),
                assign_user_role_request: Ok(value.assign_user_role_request),
                assign_user_role_response: Ok(value.assign_user_role_response),
                list_user_role_assignments_error400: Ok(value.list_user_role_assignments_error400),
                list_user_role_assignments_error403: Ok(value.list_user_role_assignments_error403),
                list_user_role_assignments_error404: Ok(value.list_user_role_assignments_error404),
                list_user_role_assignments_error429: Ok(value.list_user_role_assignments_error429),
                list_user_role_assignments_params: Ok(value.list_user_role_assignments_params),
                list_user_role_assignments_response: Ok(value.list_user_role_assignments_response),
                retrieve_user_role_error400: Ok(value.retrieve_user_role_error400),
                retrieve_user_role_error403: Ok(value.retrieve_user_role_error403),
                retrieve_user_role_error404: Ok(value.retrieve_user_role_error404),
                retrieve_user_role_error429: Ok(value.retrieve_user_role_error429),
                retrieve_user_role_params: Ok(value.retrieve_user_role_params),
                retrieve_user_role_response: Ok(value.retrieve_user_role_response),
                unassign_user_role_error400: Ok(value.unassign_user_role_error400),
                unassign_user_role_error403: Ok(value.unassign_user_role_error403),
                unassign_user_role_error404: Ok(value.unassign_user_role_error404),
                unassign_user_role_error429: Ok(value.unassign_user_role_error429),
                unassign_user_role_params: Ok(value.unassign_user_role_params),
                unassign_user_role_response: Ok(value.unassign_user_role_response),
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
