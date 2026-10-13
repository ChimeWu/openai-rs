pub use crate::common::{AssignedRoleDetails, DeletedRoleAssignmentResource, Error, ErrorResponse, MisalignmentErrorDetailsResource, PermissionErrorResponse, PublicAssignOrganizationGroupRoleBody, Role, RoleListResource, User, UserRoleAssignment, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AssignProjectUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssignProjectUserRoleParams {
    pub project_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl AssignProjectUserRoleParams {
    pub fn builder() -> builder::AssignProjectUserRoleParams {
        Default::default()
    }
}
#[doc = "`ListProjectUserRoleAssignmentsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectUserRoleAssignmentsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectUserRoleAssignmentsParamsOrder>,
    pub project_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl ListProjectUserRoleAssignmentsParams {
    pub fn builder() -> builder::ListProjectUserRoleAssignmentsParams {
        Default::default()
    }
}
#[doc = "`ListProjectUserRoleAssignmentsParamsOrder`"]
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
pub enum ListProjectUserRoleAssignmentsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectUserRoleAssignmentsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectUserRoleAssignmentsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectUserRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectUserRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectUserRoleAssignmentsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectUserRoleAssignmentsApiSchema {
    #[serde(rename = "AssignProjectUserRoleError400")]
    pub assign_project_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectUserRoleError403")]
    pub assign_project_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AssignProjectUserRoleError404")]
    pub assign_project_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectUserRoleError429")]
    pub assign_project_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectUserRoleParams")]
    pub assign_project_user_role_params: AssignProjectUserRoleParams,
    #[serde(rename = "AssignProjectUserRoleRequest")]
    pub assign_project_user_role_request:
        ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
    #[serde(rename = "AssignProjectUserRoleResponse")]
    pub assign_project_user_role_response: ::openai_api_types::common::UserRoleAssignment,
    #[serde(rename = "ListProjectUserRoleAssignmentsError400")]
    pub list_project_user_role_assignments_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUserRoleAssignmentsError403")]
    pub list_project_user_role_assignments_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectUserRoleAssignmentsError404")]
    pub list_project_user_role_assignments_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUserRoleAssignmentsError429")]
    pub list_project_user_role_assignments_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectUserRoleAssignmentsParams")]
    pub list_project_user_role_assignments_params: ListProjectUserRoleAssignmentsParams,
    #[serde(rename = "ListProjectUserRoleAssignmentsResponse")]
    pub list_project_user_role_assignments_response: ::openai_api_types::common::RoleListResource,
    #[serde(rename = "RetrieveProjectUserRoleError400")]
    pub retrieve_project_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserRoleError403")]
    pub retrieve_project_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectUserRoleError404")]
    pub retrieve_project_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserRoleError429")]
    pub retrieve_project_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectUserRoleParams")]
    pub retrieve_project_user_role_params: RetrieveProjectUserRoleParams,
    #[serde(rename = "RetrieveProjectUserRoleResponse")]
    pub retrieve_project_user_role_response: ::openai_api_types::common::AssignedRoleDetails,
    #[serde(rename = "UnassignProjectUserRoleError400")]
    pub unassign_project_user_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectUserRoleError403")]
    pub unassign_project_user_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UnassignProjectUserRoleError404")]
    pub unassign_project_user_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectUserRoleError429")]
    pub unassign_project_user_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectUserRoleParams")]
    pub unassign_project_user_role_params: UnassignProjectUserRoleParams,
    #[serde(rename = "UnassignProjectUserRoleResponse")]
    pub unassign_project_user_role_response:
        ::openai_api_types::common::DeletedRoleAssignmentResource,
}
impl ProjectUserRoleAssignmentsApiSchema {
    pub fn builder() -> builder::ProjectUserRoleAssignmentsApiSchema {
        Default::default()
    }
}
#[doc = "`RetrieveProjectUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectUserRoleParams {
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl RetrieveProjectUserRoleParams {
    pub fn builder() -> builder::RetrieveProjectUserRoleParams {
        Default::default()
    }
}
#[doc = "`UnassignProjectUserRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UnassignProjectUserRoleParams {
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
    pub user_id: ::std::string::String,
}
impl UnassignProjectUserRoleParams {
    pub fn builder() -> builder::UnassignProjectUserRoleParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AssignProjectUserRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AssignProjectUserRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl AssignProjectUserRoleParams {
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
    impl ::std::convert::TryFrom<AssignProjectUserRoleParams> for super::AssignProjectUserRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssignProjectUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::AssignProjectUserRoleParams> for AssignProjectUserRoleParams {
        fn from(value: super::AssignProjectUserRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectUserRoleAssignmentsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectUserRoleAssignmentsParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectUserRoleAssignmentsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl ListProjectUserRoleAssignmentsParams {
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
                ::std::option::Option<super::ListProjectUserRoleAssignmentsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
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
    impl ::std::convert::TryFrom<ListProjectUserRoleAssignmentsParams>
        for super::ListProjectUserRoleAssignmentsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectUserRoleAssignmentsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectUserRoleAssignmentsParams>
        for ListProjectUserRoleAssignmentsParams
    {
        fn from(value: super::ListProjectUserRoleAssignmentsParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectUserRoleAssignmentsApiSchema {
        assign_project_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        assign_project_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_user_role_params:
            ::std::result::Result<super::AssignProjectUserRoleParams, ::std::string::String>,
        assign_project_user_role_request: ::std::result::Result<
            ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            ::std::string::String,
        >,
        assign_project_user_role_response: ::std::result::Result<
            ::openai_api_types::common::UserRoleAssignment,
            ::std::string::String,
        >,
        list_project_user_role_assignments_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_user_role_assignments_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_user_role_assignments_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_user_role_assignments_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_user_role_assignments_params: ::std::result::Result<
            super::ListProjectUserRoleAssignmentsParams,
            ::std::string::String,
        >,
        list_project_user_role_assignments_response: ::std::result::Result<
            ::openai_api_types::common::RoleListResource,
            ::std::string::String,
        >,
        retrieve_project_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_user_role_params:
            ::std::result::Result<super::RetrieveProjectUserRoleParams, ::std::string::String>,
        retrieve_project_user_role_response: ::std::result::Result<
            ::openai_api_types::common::AssignedRoleDetails,
            ::std::string::String,
        >,
        unassign_project_user_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_user_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        unassign_project_user_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_user_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_user_role_params:
            ::std::result::Result<super::UnassignProjectUserRoleParams, ::std::string::String>,
        unassign_project_user_role_response: ::std::result::Result<
            ::openai_api_types::common::DeletedRoleAssignmentResource,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectUserRoleAssignmentsApiSchema {
        fn default() -> Self {
            Self {
                assign_project_user_role_error400: Err(
                    "no value supplied for assign_project_user_role_error400".to_string(),
                ),
                assign_project_user_role_error403: Err(
                    "no value supplied for assign_project_user_role_error403".to_string(),
                ),
                assign_project_user_role_error404: Err(
                    "no value supplied for assign_project_user_role_error404".to_string(),
                ),
                assign_project_user_role_error429: Err(
                    "no value supplied for assign_project_user_role_error429".to_string(),
                ),
                assign_project_user_role_params: Err(
                    "no value supplied for assign_project_user_role_params".to_string(),
                ),
                assign_project_user_role_request: Err(
                    "no value supplied for assign_project_user_role_request".to_string(),
                ),
                assign_project_user_role_response: Err(
                    "no value supplied for assign_project_user_role_response".to_string(),
                ),
                list_project_user_role_assignments_error400: Err(
                    "no value supplied for list_project_user_role_assignments_error400".to_string(),
                ),
                list_project_user_role_assignments_error403: Err(
                    "no value supplied for list_project_user_role_assignments_error403".to_string(),
                ),
                list_project_user_role_assignments_error404: Err(
                    "no value supplied for list_project_user_role_assignments_error404".to_string(),
                ),
                list_project_user_role_assignments_error429: Err(
                    "no value supplied for list_project_user_role_assignments_error429".to_string(),
                ),
                list_project_user_role_assignments_params: Err(
                    "no value supplied for list_project_user_role_assignments_params".to_string(),
                ),
                list_project_user_role_assignments_response: Err(
                    "no value supplied for list_project_user_role_assignments_response".to_string(),
                ),
                retrieve_project_user_role_error400: Err(
                    "no value supplied for retrieve_project_user_role_error400".to_string(),
                ),
                retrieve_project_user_role_error403: Err(
                    "no value supplied for retrieve_project_user_role_error403".to_string(),
                ),
                retrieve_project_user_role_error404: Err(
                    "no value supplied for retrieve_project_user_role_error404".to_string(),
                ),
                retrieve_project_user_role_error429: Err(
                    "no value supplied for retrieve_project_user_role_error429".to_string(),
                ),
                retrieve_project_user_role_params: Err(
                    "no value supplied for retrieve_project_user_role_params".to_string(),
                ),
                retrieve_project_user_role_response: Err(
                    "no value supplied for retrieve_project_user_role_response".to_string(),
                ),
                unassign_project_user_role_error400: Err(
                    "no value supplied for unassign_project_user_role_error400".to_string(),
                ),
                unassign_project_user_role_error403: Err(
                    "no value supplied for unassign_project_user_role_error403".to_string(),
                ),
                unassign_project_user_role_error404: Err(
                    "no value supplied for unassign_project_user_role_error404".to_string(),
                ),
                unassign_project_user_role_error429: Err(
                    "no value supplied for unassign_project_user_role_error429".to_string(),
                ),
                unassign_project_user_role_params: Err(
                    "no value supplied for unassign_project_user_role_params".to_string(),
                ),
                unassign_project_user_role_response: Err(
                    "no value supplied for unassign_project_user_role_response".to_string(),
                ),
            }
        }
    }
    impl ProjectUserRoleAssignmentsApiSchema {
        pub fn assign_project_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_user_role_error400: {e}"
                )
            });
            self
        }
        pub fn assign_project_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_user_role_error403: {e}"
                )
            });
            self
        }
        pub fn assign_project_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_user_role_error404: {e}"
                )
            });
            self
        }
        pub fn assign_project_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_user_role_error429: {e}"
                )
            });
            self
        }
        pub fn assign_project_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssignProjectUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_project_user_role_params: {e}")
            });
            self
        }
        pub fn assign_project_user_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_project_user_role_request: {e}")
            });
            self
        }
        pub fn assign_project_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::UserRoleAssignment>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_user_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_user_role_response: {e}"
                )
            });
            self
        }
        pub fn list_project_user_role_assignments_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_error400: {e}")) ;
            self
        }
        pub fn list_project_user_role_assignments_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_error403: {e}")) ;
            self
        }
        pub fn list_project_user_role_assignments_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_error404: {e}")) ;
            self
        }
        pub fn list_project_user_role_assignments_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_error429: {e}")) ;
            self
        }
        pub fn list_project_user_role_assignments_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectUserRoleAssignmentsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_params: {e}")) ;
            self
        }
        pub fn list_project_user_role_assignments_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::RoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_user_role_assignments_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_user_role_assignments_response: {e}")) ;
            self
        }
        pub fn retrieve_project_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_error400: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_error403: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_error404: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_error429: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_params: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::AssignedRoleDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_user_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_user_role_response: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_error400: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_error403: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_error404: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_error429: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnassignProjectUserRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_params: {e}"
                )
            });
            self
        }
        pub fn unassign_project_user_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::DeletedRoleAssignmentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_user_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_user_role_response: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectUserRoleAssignmentsApiSchema>
        for super::ProjectUserRoleAssignmentsApiSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectUserRoleAssignmentsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assign_project_user_role_error400: value.assign_project_user_role_error400?,
                assign_project_user_role_error403: value.assign_project_user_role_error403?,
                assign_project_user_role_error404: value.assign_project_user_role_error404?,
                assign_project_user_role_error429: value.assign_project_user_role_error429?,
                assign_project_user_role_params: value.assign_project_user_role_params?,
                assign_project_user_role_request: value.assign_project_user_role_request?,
                assign_project_user_role_response: value.assign_project_user_role_response?,
                list_project_user_role_assignments_error400: value
                    .list_project_user_role_assignments_error400?,
                list_project_user_role_assignments_error403: value
                    .list_project_user_role_assignments_error403?,
                list_project_user_role_assignments_error404: value
                    .list_project_user_role_assignments_error404?,
                list_project_user_role_assignments_error429: value
                    .list_project_user_role_assignments_error429?,
                list_project_user_role_assignments_params: value
                    .list_project_user_role_assignments_params?,
                list_project_user_role_assignments_response: value
                    .list_project_user_role_assignments_response?,
                retrieve_project_user_role_error400: value.retrieve_project_user_role_error400?,
                retrieve_project_user_role_error403: value.retrieve_project_user_role_error403?,
                retrieve_project_user_role_error404: value.retrieve_project_user_role_error404?,
                retrieve_project_user_role_error429: value.retrieve_project_user_role_error429?,
                retrieve_project_user_role_params: value.retrieve_project_user_role_params?,
                retrieve_project_user_role_response: value.retrieve_project_user_role_response?,
                unassign_project_user_role_error400: value.unassign_project_user_role_error400?,
                unassign_project_user_role_error403: value.unassign_project_user_role_error403?,
                unassign_project_user_role_error404: value.unassign_project_user_role_error404?,
                unassign_project_user_role_error429: value.unassign_project_user_role_error429?,
                unassign_project_user_role_params: value.unassign_project_user_role_params?,
                unassign_project_user_role_response: value.unassign_project_user_role_response?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectUserRoleAssignmentsApiSchema>
        for ProjectUserRoleAssignmentsApiSchema
    {
        fn from(value: super::ProjectUserRoleAssignmentsApiSchema) -> Self {
            Self {
                assign_project_user_role_error400: Ok(value.assign_project_user_role_error400),
                assign_project_user_role_error403: Ok(value.assign_project_user_role_error403),
                assign_project_user_role_error404: Ok(value.assign_project_user_role_error404),
                assign_project_user_role_error429: Ok(value.assign_project_user_role_error429),
                assign_project_user_role_params: Ok(value.assign_project_user_role_params),
                assign_project_user_role_request: Ok(value.assign_project_user_role_request),
                assign_project_user_role_response: Ok(value.assign_project_user_role_response),
                list_project_user_role_assignments_error400: Ok(
                    value.list_project_user_role_assignments_error400
                ),
                list_project_user_role_assignments_error403: Ok(
                    value.list_project_user_role_assignments_error403
                ),
                list_project_user_role_assignments_error404: Ok(
                    value.list_project_user_role_assignments_error404
                ),
                list_project_user_role_assignments_error429: Ok(
                    value.list_project_user_role_assignments_error429
                ),
                list_project_user_role_assignments_params: Ok(
                    value.list_project_user_role_assignments_params
                ),
                list_project_user_role_assignments_response: Ok(
                    value.list_project_user_role_assignments_response
                ),
                retrieve_project_user_role_error400: Ok(value.retrieve_project_user_role_error400),
                retrieve_project_user_role_error403: Ok(value.retrieve_project_user_role_error403),
                retrieve_project_user_role_error404: Ok(value.retrieve_project_user_role_error404),
                retrieve_project_user_role_error429: Ok(value.retrieve_project_user_role_error429),
                retrieve_project_user_role_params: Ok(value.retrieve_project_user_role_params),
                retrieve_project_user_role_response: Ok(value.retrieve_project_user_role_response),
                unassign_project_user_role_error400: Ok(value.unassign_project_user_role_error400),
                unassign_project_user_role_error403: Ok(value.unassign_project_user_role_error403),
                unassign_project_user_role_error404: Ok(value.unassign_project_user_role_error404),
                unassign_project_user_role_error429: Ok(value.unassign_project_user_role_error429),
                unassign_project_user_role_params: Ok(value.unassign_project_user_role_params),
                unassign_project_user_role_response: Ok(value.unassign_project_user_role_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectUserRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectUserRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RetrieveProjectUserRoleParams {
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
    impl ::std::convert::TryFrom<RetrieveProjectUserRoleParams>
        for super::RetrieveProjectUserRoleParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                role_id: value.role_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectUserRoleParams> for RetrieveProjectUserRoleParams {
        fn from(value: super::RetrieveProjectUserRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnassignProjectUserRoleParams {
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnassignProjectUserRoleParams {
        fn default() -> Self {
            Self {
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl UnassignProjectUserRoleParams {
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
    impl ::std::convert::TryFrom<UnassignProjectUserRoleParams>
        for super::UnassignProjectUserRoleParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnassignProjectUserRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                project_id: value.project_id?,
                role_id: value.role_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::UnassignProjectUserRoleParams> for UnassignProjectUserRoleParams {
        fn from(value: super::UnassignProjectUserRoleParams) -> Self {
            Self {
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
                user_id: Ok(value.user_id),
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
