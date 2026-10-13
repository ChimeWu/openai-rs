pub use crate::common::{AssignedRoleDetails, DeletedRoleAssignmentResource, Error, ErrorResponse, Group, GroupRoleAssignment, MisalignmentErrorDetailsResource, PermissionErrorResponse, PublicAssignOrganizationGroupRoleBody, Role, RoleListResource, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AssignProjectGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssignProjectGroupRoleParams {
    pub group_id: ::std::string::String,
    pub project_id: ::std::string::String,
}
impl AssignProjectGroupRoleParams {
    pub fn builder() -> builder::AssignProjectGroupRoleParams {
        Default::default()
    }
}
#[doc = "`ListProjectGroupRoleAssignmentsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListProjectGroupRoleAssignmentsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub group_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListProjectGroupRoleAssignmentsParamsOrder>,
    pub project_id: ::std::string::String,
}
impl ListProjectGroupRoleAssignmentsParams {
    pub fn builder() -> builder::ListProjectGroupRoleAssignmentsParams {
        Default::default()
    }
}
#[doc = "`ListProjectGroupRoleAssignmentsParamsOrder`"]
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
pub enum ListProjectGroupRoleAssignmentsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListProjectGroupRoleAssignmentsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListProjectGroupRoleAssignmentsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListProjectGroupRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListProjectGroupRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ProjectGroupRoleAssignmentsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProjectGroupRoleAssignmentsApiSchema {
    #[serde(rename = "AssignProjectGroupRoleError400")]
    pub assign_project_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectGroupRoleError403")]
    pub assign_project_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AssignProjectGroupRoleError404")]
    pub assign_project_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectGroupRoleError429")]
    pub assign_project_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignProjectGroupRoleParams")]
    pub assign_project_group_role_params: AssignProjectGroupRoleParams,
    #[serde(rename = "AssignProjectGroupRoleRequest")]
    pub assign_project_group_role_request:
        ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
    #[serde(rename = "AssignProjectGroupRoleResponse")]
    pub assign_project_group_role_response: ::openai_api_types::common::GroupRoleAssignment,
    #[serde(rename = "ListProjectGroupRoleAssignmentsError400")]
    pub list_project_group_role_assignments_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupRoleAssignmentsError403")]
    pub list_project_group_role_assignments_error403:
        ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListProjectGroupRoleAssignmentsError404")]
    pub list_project_group_role_assignments_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupRoleAssignmentsError429")]
    pub list_project_group_role_assignments_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListProjectGroupRoleAssignmentsParams")]
    pub list_project_group_role_assignments_params: ListProjectGroupRoleAssignmentsParams,
    #[serde(rename = "ListProjectGroupRoleAssignmentsResponse")]
    pub list_project_group_role_assignments_response: ::openai_api_types::common::RoleListResource,
    #[serde(rename = "RetrieveProjectGroupRoleError400")]
    pub retrieve_project_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupRoleError403")]
    pub retrieve_project_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveProjectGroupRoleError404")]
    pub retrieve_project_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupRoleError429")]
    pub retrieve_project_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveProjectGroupRoleParams")]
    pub retrieve_project_group_role_params: RetrieveProjectGroupRoleParams,
    #[serde(rename = "RetrieveProjectGroupRoleResponse")]
    pub retrieve_project_group_role_response: ::openai_api_types::common::AssignedRoleDetails,
    #[serde(rename = "UnassignProjectGroupRoleError400")]
    pub unassign_project_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectGroupRoleError403")]
    pub unassign_project_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UnassignProjectGroupRoleError404")]
    pub unassign_project_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectGroupRoleError429")]
    pub unassign_project_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignProjectGroupRoleParams")]
    pub unassign_project_group_role_params: UnassignProjectGroupRoleParams,
    #[serde(rename = "UnassignProjectGroupRoleResponse")]
    pub unassign_project_group_role_response:
        ::openai_api_types::common::DeletedRoleAssignmentResource,
}
impl ProjectGroupRoleAssignmentsApiSchema {
    pub fn builder() -> builder::ProjectGroupRoleAssignmentsApiSchema {
        Default::default()
    }
}
#[doc = "`RetrieveProjectGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveProjectGroupRoleParams {
    pub group_id: ::std::string::String,
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl RetrieveProjectGroupRoleParams {
    pub fn builder() -> builder::RetrieveProjectGroupRoleParams {
        Default::default()
    }
}
#[doc = "`UnassignProjectGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UnassignProjectGroupRoleParams {
    pub group_id: ::std::string::String,
    pub project_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl UnassignProjectGroupRoleParams {
    pub fn builder() -> builder::UnassignProjectGroupRoleParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AssignProjectGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AssignProjectGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl AssignProjectGroupRoleParams {
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
    }
    impl ::std::convert::TryFrom<AssignProjectGroupRoleParams> for super::AssignProjectGroupRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssignProjectGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::AssignProjectGroupRoleParams> for AssignProjectGroupRoleParams {
        fn from(value: super::AssignProjectGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListProjectGroupRoleAssignmentsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListProjectGroupRoleAssignmentsParamsOrder>,
            ::std::string::String,
        >,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListProjectGroupRoleAssignmentsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                group_id: Err("no value supplied for group_id".to_string()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
                project_id: Err("no value supplied for project_id".to_string()),
            }
        }
    }
    impl ListProjectGroupRoleAssignmentsParams {
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::ListProjectGroupRoleAssignmentsParamsOrder>,
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
    }
    impl ::std::convert::TryFrom<ListProjectGroupRoleAssignmentsParams>
        for super::ListProjectGroupRoleAssignmentsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListProjectGroupRoleAssignmentsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                group_id: value.group_id?,
                limit: value.limit?,
                order: value.order?,
                project_id: value.project_id?,
            })
        }
    }
    impl ::std::convert::From<super::ListProjectGroupRoleAssignmentsParams>
        for ListProjectGroupRoleAssignmentsParams
    {
        fn from(value: super::ListProjectGroupRoleAssignmentsParams) -> Self {
            Self {
                after: Ok(value.after),
                group_id: Ok(value.group_id),
                limit: Ok(value.limit),
                order: Ok(value.order),
                project_id: Ok(value.project_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectGroupRoleAssignmentsApiSchema {
        assign_project_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        assign_project_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_project_group_role_params:
            ::std::result::Result<super::AssignProjectGroupRoleParams, ::std::string::String>,
        assign_project_group_role_request: ::std::result::Result<
            ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            ::std::string::String,
        >,
        assign_project_group_role_response: ::std::result::Result<
            ::openai_api_types::common::GroupRoleAssignment,
            ::std::string::String,
        >,
        list_project_group_role_assignments_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_group_role_assignments_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_project_group_role_assignments_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_group_role_assignments_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_project_group_role_assignments_params: ::std::result::Result<
            super::ListProjectGroupRoleAssignmentsParams,
            ::std::string::String,
        >,
        list_project_group_role_assignments_response: ::std::result::Result<
            ::openai_api_types::common::RoleListResource,
            ::std::string::String,
        >,
        retrieve_project_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_project_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_project_group_role_params:
            ::std::result::Result<super::RetrieveProjectGroupRoleParams, ::std::string::String>,
        retrieve_project_group_role_response: ::std::result::Result<
            ::openai_api_types::common::AssignedRoleDetails,
            ::std::string::String,
        >,
        unassign_project_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        unassign_project_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_project_group_role_params:
            ::std::result::Result<super::UnassignProjectGroupRoleParams, ::std::string::String>,
        unassign_project_group_role_response: ::std::result::Result<
            ::openai_api_types::common::DeletedRoleAssignmentResource,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ProjectGroupRoleAssignmentsApiSchema {
        fn default() -> Self {
            Self {
                assign_project_group_role_error400: Err(
                    "no value supplied for assign_project_group_role_error400".to_string(),
                ),
                assign_project_group_role_error403: Err(
                    "no value supplied for assign_project_group_role_error403".to_string(),
                ),
                assign_project_group_role_error404: Err(
                    "no value supplied for assign_project_group_role_error404".to_string(),
                ),
                assign_project_group_role_error429: Err(
                    "no value supplied for assign_project_group_role_error429".to_string(),
                ),
                assign_project_group_role_params: Err(
                    "no value supplied for assign_project_group_role_params".to_string(),
                ),
                assign_project_group_role_request: Err(
                    "no value supplied for assign_project_group_role_request".to_string(),
                ),
                assign_project_group_role_response: Err(
                    "no value supplied for assign_project_group_role_response".to_string(),
                ),
                list_project_group_role_assignments_error400: Err(
                    "no value supplied for list_project_group_role_assignments_error400"
                        .to_string(),
                ),
                list_project_group_role_assignments_error403: Err(
                    "no value supplied for list_project_group_role_assignments_error403"
                        .to_string(),
                ),
                list_project_group_role_assignments_error404: Err(
                    "no value supplied for list_project_group_role_assignments_error404"
                        .to_string(),
                ),
                list_project_group_role_assignments_error429: Err(
                    "no value supplied for list_project_group_role_assignments_error429"
                        .to_string(),
                ),
                list_project_group_role_assignments_params: Err(
                    "no value supplied for list_project_group_role_assignments_params".to_string(),
                ),
                list_project_group_role_assignments_response: Err(
                    "no value supplied for list_project_group_role_assignments_response"
                        .to_string(),
                ),
                retrieve_project_group_role_error400: Err(
                    "no value supplied for retrieve_project_group_role_error400".to_string(),
                ),
                retrieve_project_group_role_error403: Err(
                    "no value supplied for retrieve_project_group_role_error403".to_string(),
                ),
                retrieve_project_group_role_error404: Err(
                    "no value supplied for retrieve_project_group_role_error404".to_string(),
                ),
                retrieve_project_group_role_error429: Err(
                    "no value supplied for retrieve_project_group_role_error429".to_string(),
                ),
                retrieve_project_group_role_params: Err(
                    "no value supplied for retrieve_project_group_role_params".to_string(),
                ),
                retrieve_project_group_role_response: Err(
                    "no value supplied for retrieve_project_group_role_response".to_string(),
                ),
                unassign_project_group_role_error400: Err(
                    "no value supplied for unassign_project_group_role_error400".to_string(),
                ),
                unassign_project_group_role_error403: Err(
                    "no value supplied for unassign_project_group_role_error403".to_string(),
                ),
                unassign_project_group_role_error404: Err(
                    "no value supplied for unassign_project_group_role_error404".to_string(),
                ),
                unassign_project_group_role_error429: Err(
                    "no value supplied for unassign_project_group_role_error429".to_string(),
                ),
                unassign_project_group_role_params: Err(
                    "no value supplied for unassign_project_group_role_params".to_string(),
                ),
                unassign_project_group_role_response: Err(
                    "no value supplied for unassign_project_group_role_response".to_string(),
                ),
            }
        }
    }
    impl ProjectGroupRoleAssignmentsApiSchema {
        pub fn assign_project_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_error400: {e}"
                )
            });
            self
        }
        pub fn assign_project_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_error403: {e}"
                )
            });
            self
        }
        pub fn assign_project_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_error404: {e}"
                )
            });
            self
        }
        pub fn assign_project_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_error429: {e}"
                )
            });
            self
        }
        pub fn assign_project_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssignProjectGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_project_group_role_params: {e}")
            });
            self
        }
        pub fn assign_project_group_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_request: {e}"
                )
            });
            self
        }
        pub fn assign_project_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::GroupRoleAssignment>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_project_group_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for assign_project_group_role_response: {e}"
                )
            });
            self
        }
        pub fn list_project_group_role_assignments_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_error400 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_error400: {e}")) ;
            self
        }
        pub fn list_project_group_role_assignments_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_error403 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_error403: {e}")) ;
            self
        }
        pub fn list_project_group_role_assignments_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_error404 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_error404: {e}")) ;
            self
        }
        pub fn list_project_group_role_assignments_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_error429 = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_error429: {e}")) ;
            self
        }
        pub fn list_project_group_role_assignments_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListProjectGroupRoleAssignmentsParams>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_params = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_params: {e}")) ;
            self
        }
        pub fn list_project_group_role_assignments_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::RoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self . list_project_group_role_assignments_response = value . try_into () . map_err (| e | format ! ("error converting supplied value for list_project_group_role_assignments_response: {e}")) ;
            self
        }
        pub fn retrieve_project_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_error400: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_error403: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_error404: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_error429: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveProjectGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_params: {e}"
                )
            });
            self
        }
        pub fn retrieve_project_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::AssignedRoleDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_project_group_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for retrieve_project_group_role_response: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_error400: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_error403: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_error404: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_error429: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnassignProjectGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_params: {e}"
                )
            });
            self
        }
        pub fn unassign_project_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::DeletedRoleAssignmentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_project_group_role_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for unassign_project_group_role_response: {e}"
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectGroupRoleAssignmentsApiSchema>
        for super::ProjectGroupRoleAssignmentsApiSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectGroupRoleAssignmentsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assign_project_group_role_error400: value.assign_project_group_role_error400?,
                assign_project_group_role_error403: value.assign_project_group_role_error403?,
                assign_project_group_role_error404: value.assign_project_group_role_error404?,
                assign_project_group_role_error429: value.assign_project_group_role_error429?,
                assign_project_group_role_params: value.assign_project_group_role_params?,
                assign_project_group_role_request: value.assign_project_group_role_request?,
                assign_project_group_role_response: value.assign_project_group_role_response?,
                list_project_group_role_assignments_error400: value
                    .list_project_group_role_assignments_error400?,
                list_project_group_role_assignments_error403: value
                    .list_project_group_role_assignments_error403?,
                list_project_group_role_assignments_error404: value
                    .list_project_group_role_assignments_error404?,
                list_project_group_role_assignments_error429: value
                    .list_project_group_role_assignments_error429?,
                list_project_group_role_assignments_params: value
                    .list_project_group_role_assignments_params?,
                list_project_group_role_assignments_response: value
                    .list_project_group_role_assignments_response?,
                retrieve_project_group_role_error400: value.retrieve_project_group_role_error400?,
                retrieve_project_group_role_error403: value.retrieve_project_group_role_error403?,
                retrieve_project_group_role_error404: value.retrieve_project_group_role_error404?,
                retrieve_project_group_role_error429: value.retrieve_project_group_role_error429?,
                retrieve_project_group_role_params: value.retrieve_project_group_role_params?,
                retrieve_project_group_role_response: value.retrieve_project_group_role_response?,
                unassign_project_group_role_error400: value.unassign_project_group_role_error400?,
                unassign_project_group_role_error403: value.unassign_project_group_role_error403?,
                unassign_project_group_role_error404: value.unassign_project_group_role_error404?,
                unassign_project_group_role_error429: value.unassign_project_group_role_error429?,
                unassign_project_group_role_params: value.unassign_project_group_role_params?,
                unassign_project_group_role_response: value.unassign_project_group_role_response?,
            })
        }
    }
    impl ::std::convert::From<super::ProjectGroupRoleAssignmentsApiSchema>
        for ProjectGroupRoleAssignmentsApiSchema
    {
        fn from(value: super::ProjectGroupRoleAssignmentsApiSchema) -> Self {
            Self {
                assign_project_group_role_error400: Ok(value.assign_project_group_role_error400),
                assign_project_group_role_error403: Ok(value.assign_project_group_role_error403),
                assign_project_group_role_error404: Ok(value.assign_project_group_role_error404),
                assign_project_group_role_error429: Ok(value.assign_project_group_role_error429),
                assign_project_group_role_params: Ok(value.assign_project_group_role_params),
                assign_project_group_role_request: Ok(value.assign_project_group_role_request),
                assign_project_group_role_response: Ok(value.assign_project_group_role_response),
                list_project_group_role_assignments_error400: Ok(
                    value.list_project_group_role_assignments_error400
                ),
                list_project_group_role_assignments_error403: Ok(
                    value.list_project_group_role_assignments_error403
                ),
                list_project_group_role_assignments_error404: Ok(
                    value.list_project_group_role_assignments_error404
                ),
                list_project_group_role_assignments_error429: Ok(
                    value.list_project_group_role_assignments_error429
                ),
                list_project_group_role_assignments_params: Ok(
                    value.list_project_group_role_assignments_params
                ),
                list_project_group_role_assignments_response: Ok(
                    value.list_project_group_role_assignments_response
                ),
                retrieve_project_group_role_error400: Ok(value.retrieve_project_group_role_error400),
                retrieve_project_group_role_error403: Ok(value.retrieve_project_group_role_error403),
                retrieve_project_group_role_error404: Ok(value.retrieve_project_group_role_error404),
                retrieve_project_group_role_error429: Ok(value.retrieve_project_group_role_error429),
                retrieve_project_group_role_params: Ok(value.retrieve_project_group_role_params),
                retrieve_project_group_role_response: Ok(value.retrieve_project_group_role_response),
                unassign_project_group_role_error400: Ok(value.unassign_project_group_role_error400),
                unassign_project_group_role_error403: Ok(value.unassign_project_group_role_error403),
                unassign_project_group_role_error404: Ok(value.unassign_project_group_role_error404),
                unassign_project_group_role_error429: Ok(value.unassign_project_group_role_error429),
                unassign_project_group_role_params: Ok(value.unassign_project_group_role_params),
                unassign_project_group_role_response: Ok(value.unassign_project_group_role_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveProjectGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveProjectGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl RetrieveProjectGroupRoleParams {
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
    }
    impl ::std::convert::TryFrom<RetrieveProjectGroupRoleParams>
        for super::RetrieveProjectGroupRoleParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveProjectGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                project_id: value.project_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveProjectGroupRoleParams>
        for RetrieveProjectGroupRoleParams
    {
        fn from(value: super::RetrieveProjectGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnassignProjectGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        project_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnassignProjectGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                project_id: Err("no value supplied for project_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl UnassignProjectGroupRoleParams {
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
    }
    impl ::std::convert::TryFrom<UnassignProjectGroupRoleParams>
        for super::UnassignProjectGroupRoleParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnassignProjectGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                project_id: value.project_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::UnassignProjectGroupRoleParams>
        for UnassignProjectGroupRoleParams
    {
        fn from(value: super::UnassignProjectGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                project_id: Ok(value.project_id),
                role_id: Ok(value.role_id),
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
