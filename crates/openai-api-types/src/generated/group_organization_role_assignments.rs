pub use crate::common::{AssignedRoleDetails, DeletedRoleAssignmentResource, Error, ErrorResponse, Group, GroupRoleAssignment, MisalignmentErrorDetailsResource, PermissionErrorResponse, PublicAssignOrganizationGroupRoleBody, Role, RoleListResource, MisalignmentErrorType, MisalignmentSteer};
#[doc = "`AssignGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct AssignGroupRoleParams {
    pub group_id: ::std::string::String,
}
impl AssignGroupRoleParams {
    pub fn builder() -> builder::AssignGroupRoleParams {
        Default::default()
    }
}
#[doc = "`GroupOrganizationRoleAssignmentsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GroupOrganizationRoleAssignmentsApiSchema {
    #[serde(rename = "AssignGroupRoleError400")]
    pub assign_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignGroupRoleError403")]
    pub assign_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "AssignGroupRoleError404")]
    pub assign_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignGroupRoleError429")]
    pub assign_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "AssignGroupRoleParams")]
    pub assign_group_role_params: AssignGroupRoleParams,
    #[serde(rename = "AssignGroupRoleRequest")]
    pub assign_group_role_request:
        ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
    #[serde(rename = "AssignGroupRoleResponse")]
    pub assign_group_role_response: ::openai_api_types::common::GroupRoleAssignment,
    #[serde(rename = "ListGroupRoleAssignmentsError400")]
    pub list_group_role_assignments_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupRoleAssignmentsError403")]
    pub list_group_role_assignments_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "ListGroupRoleAssignmentsError404")]
    pub list_group_role_assignments_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupRoleAssignmentsError429")]
    pub list_group_role_assignments_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListGroupRoleAssignmentsParams")]
    pub list_group_role_assignments_params: ListGroupRoleAssignmentsParams,
    #[serde(rename = "ListGroupRoleAssignmentsResponse")]
    pub list_group_role_assignments_response: ::openai_api_types::common::RoleListResource,
    #[serde(rename = "RetrieveGroupRoleError400")]
    pub retrieve_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupRoleError403")]
    pub retrieve_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "RetrieveGroupRoleError404")]
    pub retrieve_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupRoleError429")]
    pub retrieve_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveGroupRoleParams")]
    pub retrieve_group_role_params: RetrieveGroupRoleParams,
    #[serde(rename = "RetrieveGroupRoleResponse")]
    pub retrieve_group_role_response: ::openai_api_types::common::AssignedRoleDetails,
    #[serde(rename = "UnassignGroupRoleError400")]
    pub unassign_group_role_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignGroupRoleError403")]
    pub unassign_group_role_error403: ::openai_api_types::common::PermissionErrorResponse,
    #[serde(rename = "UnassignGroupRoleError404")]
    pub unassign_group_role_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignGroupRoleError429")]
    pub unassign_group_role_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "UnassignGroupRoleParams")]
    pub unassign_group_role_params: UnassignGroupRoleParams,
    #[serde(rename = "UnassignGroupRoleResponse")]
    pub unassign_group_role_response: ::openai_api_types::common::DeletedRoleAssignmentResource,
}
impl GroupOrganizationRoleAssignmentsApiSchema {
    pub fn builder() -> builder::GroupOrganizationRoleAssignmentsApiSchema {
        Default::default()
    }
}
#[doc = "`ListGroupRoleAssignmentsParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ListGroupRoleAssignmentsParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    pub group_id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<ListGroupRoleAssignmentsParamsOrder>,
}
impl ListGroupRoleAssignmentsParams {
    pub fn builder() -> builder::ListGroupRoleAssignmentsParams {
        Default::default()
    }
}
#[doc = "`ListGroupRoleAssignmentsParamsOrder`"]
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
pub enum ListGroupRoleAssignmentsParamsOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for ListGroupRoleAssignmentsParamsOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for ListGroupRoleAssignmentsParamsOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListGroupRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListGroupRoleAssignmentsParamsOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`RetrieveGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveGroupRoleParams {
    pub group_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl RetrieveGroupRoleParams {
    pub fn builder() -> builder::RetrieveGroupRoleParams {
        Default::default()
    }
}
#[doc = "`UnassignGroupRoleParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct UnassignGroupRoleParams {
    pub group_id: ::std::string::String,
    pub role_id: ::std::string::String,
}
impl UnassignGroupRoleParams {
    pub fn builder() -> builder::UnassignGroupRoleParams {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AssignGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AssignGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
            }
        }
    }
    impl AssignGroupRoleParams {
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
    impl ::std::convert::TryFrom<AssignGroupRoleParams> for super::AssignGroupRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AssignGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
            })
        }
    }
    impl ::std::convert::From<super::AssignGroupRoleParams> for AssignGroupRoleParams {
        fn from(value: super::AssignGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupOrganizationRoleAssignmentsApiSchema {
        assign_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        assign_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        assign_group_role_params:
            ::std::result::Result<super::AssignGroupRoleParams, ::std::string::String>,
        assign_group_role_request: ::std::result::Result<
            ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            ::std::string::String,
        >,
        assign_group_role_response: ::std::result::Result<
            ::openai_api_types::common::GroupRoleAssignment,
            ::std::string::String,
        >,
        list_group_role_assignments_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_role_assignments_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        list_group_role_assignments_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_role_assignments_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_group_role_assignments_params:
            ::std::result::Result<super::ListGroupRoleAssignmentsParams, ::std::string::String>,
        list_group_role_assignments_response: ::std::result::Result<
            ::openai_api_types::common::RoleListResource,
            ::std::string::String,
        >,
        retrieve_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        retrieve_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_group_role_params:
            ::std::result::Result<super::RetrieveGroupRoleParams, ::std::string::String>,
        retrieve_group_role_response: ::std::result::Result<
            ::openai_api_types::common::AssignedRoleDetails,
            ::std::string::String,
        >,
        unassign_group_role_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_group_role_error403: ::std::result::Result<
            ::openai_api_types::common::PermissionErrorResponse,
            ::std::string::String,
        >,
        unassign_group_role_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_group_role_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        unassign_group_role_params:
            ::std::result::Result<super::UnassignGroupRoleParams, ::std::string::String>,
        unassign_group_role_response: ::std::result::Result<
            ::openai_api_types::common::DeletedRoleAssignmentResource,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GroupOrganizationRoleAssignmentsApiSchema {
        fn default() -> Self {
            Self {
                assign_group_role_error400: Err(
                    "no value supplied for assign_group_role_error400".to_string()
                ),
                assign_group_role_error403: Err(
                    "no value supplied for assign_group_role_error403".to_string()
                ),
                assign_group_role_error404: Err(
                    "no value supplied for assign_group_role_error404".to_string()
                ),
                assign_group_role_error429: Err(
                    "no value supplied for assign_group_role_error429".to_string()
                ),
                assign_group_role_params: Err(
                    "no value supplied for assign_group_role_params".to_string()
                ),
                assign_group_role_request: Err(
                    "no value supplied for assign_group_role_request".to_string()
                ),
                assign_group_role_response: Err(
                    "no value supplied for assign_group_role_response".to_string()
                ),
                list_group_role_assignments_error400: Err(
                    "no value supplied for list_group_role_assignments_error400".to_string(),
                ),
                list_group_role_assignments_error403: Err(
                    "no value supplied for list_group_role_assignments_error403".to_string(),
                ),
                list_group_role_assignments_error404: Err(
                    "no value supplied for list_group_role_assignments_error404".to_string(),
                ),
                list_group_role_assignments_error429: Err(
                    "no value supplied for list_group_role_assignments_error429".to_string(),
                ),
                list_group_role_assignments_params: Err(
                    "no value supplied for list_group_role_assignments_params".to_string(),
                ),
                list_group_role_assignments_response: Err(
                    "no value supplied for list_group_role_assignments_response".to_string(),
                ),
                retrieve_group_role_error400: Err(
                    "no value supplied for retrieve_group_role_error400".to_string(),
                ),
                retrieve_group_role_error403: Err(
                    "no value supplied for retrieve_group_role_error403".to_string(),
                ),
                retrieve_group_role_error404: Err(
                    "no value supplied for retrieve_group_role_error404".to_string(),
                ),
                retrieve_group_role_error429: Err(
                    "no value supplied for retrieve_group_role_error429".to_string(),
                ),
                retrieve_group_role_params: Err(
                    "no value supplied for retrieve_group_role_params".to_string()
                ),
                retrieve_group_role_response: Err(
                    "no value supplied for retrieve_group_role_response".to_string(),
                ),
                unassign_group_role_error400: Err(
                    "no value supplied for unassign_group_role_error400".to_string(),
                ),
                unassign_group_role_error403: Err(
                    "no value supplied for unassign_group_role_error403".to_string(),
                ),
                unassign_group_role_error404: Err(
                    "no value supplied for unassign_group_role_error404".to_string(),
                ),
                unassign_group_role_error429: Err(
                    "no value supplied for unassign_group_role_error429".to_string(),
                ),
                unassign_group_role_params: Err(
                    "no value supplied for unassign_group_role_params".to_string()
                ),
                unassign_group_role_response: Err(
                    "no value supplied for unassign_group_role_response".to_string(),
                ),
            }
        }
    }
    impl GroupOrganizationRoleAssignmentsApiSchema {
        pub fn assign_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_error400: {e}")
            });
            self
        }
        pub fn assign_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_error403: {e}")
            });
            self
        }
        pub fn assign_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_error404: {e}")
            });
            self
        }
        pub fn assign_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_error429: {e}")
            });
            self
        }
        pub fn assign_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AssignGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_params: {e}")
            });
            self
        }
        pub fn assign_group_role_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::openai_api_types::common::PublicAssignOrganizationGroupRoleBody,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_request: {e}")
            });
            self
        }
        pub fn assign_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::GroupRoleAssignment>,
            T::Error: ::std::fmt::Display,
        {
            self.assign_group_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for assign_group_role_response: {e}")
            });
            self
        }
        pub fn list_group_role_assignments_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_error400 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_error400: {e}"
                )
            });
            self
        }
        pub fn list_group_role_assignments_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_error403 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_error403: {e}"
                )
            });
            self
        }
        pub fn list_group_role_assignments_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_error404 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_error404: {e}"
                )
            });
            self
        }
        pub fn list_group_role_assignments_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_error429 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_error429: {e}"
                )
            });
            self
        }
        pub fn list_group_role_assignments_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListGroupRoleAssignmentsParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_params = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_params: {e}"
                )
            });
            self
        }
        pub fn list_group_role_assignments_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::RoleListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_group_role_assignments_response = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for list_group_role_assignments_response: {e}"
                )
            });
            self
        }
        pub fn retrieve_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_error400: {e}")
            });
            self
        }
        pub fn retrieve_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_error403: {e}")
            });
            self
        }
        pub fn retrieve_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_error404: {e}")
            });
            self
        }
        pub fn retrieve_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_error429: {e}")
            });
            self
        }
        pub fn retrieve_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_params: {e}")
            });
            self
        }
        pub fn retrieve_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::AssignedRoleDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_group_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_group_role_response: {e}")
            });
            self
        }
        pub fn unassign_group_role_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_error400: {e}")
            });
            self
        }
        pub fn unassign_group_role_error403<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::PermissionErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_error403 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_error403: {e}")
            });
            self
        }
        pub fn unassign_group_role_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_error404: {e}")
            });
            self
        }
        pub fn unassign_group_role_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_error429: {e}")
            });
            self
        }
        pub fn unassign_group_role_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnassignGroupRoleParams>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_params: {e}")
            });
            self
        }
        pub fn unassign_group_role_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::DeletedRoleAssignmentResource>,
            T::Error: ::std::fmt::Display,
        {
            self.unassign_group_role_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for unassign_group_role_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GroupOrganizationRoleAssignmentsApiSchema>
        for super::GroupOrganizationRoleAssignmentsApiSchema
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupOrganizationRoleAssignmentsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                assign_group_role_error400: value.assign_group_role_error400?,
                assign_group_role_error403: value.assign_group_role_error403?,
                assign_group_role_error404: value.assign_group_role_error404?,
                assign_group_role_error429: value.assign_group_role_error429?,
                assign_group_role_params: value.assign_group_role_params?,
                assign_group_role_request: value.assign_group_role_request?,
                assign_group_role_response: value.assign_group_role_response?,
                list_group_role_assignments_error400: value.list_group_role_assignments_error400?,
                list_group_role_assignments_error403: value.list_group_role_assignments_error403?,
                list_group_role_assignments_error404: value.list_group_role_assignments_error404?,
                list_group_role_assignments_error429: value.list_group_role_assignments_error429?,
                list_group_role_assignments_params: value.list_group_role_assignments_params?,
                list_group_role_assignments_response: value.list_group_role_assignments_response?,
                retrieve_group_role_error400: value.retrieve_group_role_error400?,
                retrieve_group_role_error403: value.retrieve_group_role_error403?,
                retrieve_group_role_error404: value.retrieve_group_role_error404?,
                retrieve_group_role_error429: value.retrieve_group_role_error429?,
                retrieve_group_role_params: value.retrieve_group_role_params?,
                retrieve_group_role_response: value.retrieve_group_role_response?,
                unassign_group_role_error400: value.unassign_group_role_error400?,
                unassign_group_role_error403: value.unassign_group_role_error403?,
                unassign_group_role_error404: value.unassign_group_role_error404?,
                unassign_group_role_error429: value.unassign_group_role_error429?,
                unassign_group_role_params: value.unassign_group_role_params?,
                unassign_group_role_response: value.unassign_group_role_response?,
            })
        }
    }
    impl ::std::convert::From<super::GroupOrganizationRoleAssignmentsApiSchema>
        for GroupOrganizationRoleAssignmentsApiSchema
    {
        fn from(value: super::GroupOrganizationRoleAssignmentsApiSchema) -> Self {
            Self {
                assign_group_role_error400: Ok(value.assign_group_role_error400),
                assign_group_role_error403: Ok(value.assign_group_role_error403),
                assign_group_role_error404: Ok(value.assign_group_role_error404),
                assign_group_role_error429: Ok(value.assign_group_role_error429),
                assign_group_role_params: Ok(value.assign_group_role_params),
                assign_group_role_request: Ok(value.assign_group_role_request),
                assign_group_role_response: Ok(value.assign_group_role_response),
                list_group_role_assignments_error400: Ok(value.list_group_role_assignments_error400),
                list_group_role_assignments_error403: Ok(value.list_group_role_assignments_error403),
                list_group_role_assignments_error404: Ok(value.list_group_role_assignments_error404),
                list_group_role_assignments_error429: Ok(value.list_group_role_assignments_error429),
                list_group_role_assignments_params: Ok(value.list_group_role_assignments_params),
                list_group_role_assignments_response: Ok(value.list_group_role_assignments_response),
                retrieve_group_role_error400: Ok(value.retrieve_group_role_error400),
                retrieve_group_role_error403: Ok(value.retrieve_group_role_error403),
                retrieve_group_role_error404: Ok(value.retrieve_group_role_error404),
                retrieve_group_role_error429: Ok(value.retrieve_group_role_error429),
                retrieve_group_role_params: Ok(value.retrieve_group_role_params),
                retrieve_group_role_response: Ok(value.retrieve_group_role_response),
                unassign_group_role_error400: Ok(value.unassign_group_role_error400),
                unassign_group_role_error403: Ok(value.unassign_group_role_error403),
                unassign_group_role_error404: Ok(value.unassign_group_role_error404),
                unassign_group_role_error429: Ok(value.unassign_group_role_error429),
                unassign_group_role_params: Ok(value.unassign_group_role_params),
                unassign_group_role_response: Ok(value.unassign_group_role_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListGroupRoleAssignmentsParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<super::ListGroupRoleAssignmentsParamsOrder>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListGroupRoleAssignmentsParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                group_id: Err("no value supplied for group_id".to_string()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListGroupRoleAssignmentsParams {
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
                ::std::option::Option<super::ListGroupRoleAssignmentsParamsOrder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListGroupRoleAssignmentsParams>
        for super::ListGroupRoleAssignmentsParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListGroupRoleAssignmentsParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                group_id: value.group_id?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListGroupRoleAssignmentsParams>
        for ListGroupRoleAssignmentsParams
    {
        fn from(value: super::ListGroupRoleAssignmentsParams) -> Self {
            Self {
                after: Ok(value.after),
                group_id: Ok(value.group_id),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl RetrieveGroupRoleParams {
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
    impl ::std::convert::TryFrom<RetrieveGroupRoleParams> for super::RetrieveGroupRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveGroupRoleParams> for RetrieveGroupRoleParams {
        fn from(value: super::RetrieveGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
                role_id: Ok(value.role_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnassignGroupRoleParams {
        group_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnassignGroupRoleParams {
        fn default() -> Self {
            Self {
                group_id: Err("no value supplied for group_id".to_string()),
                role_id: Err("no value supplied for role_id".to_string()),
            }
        }
    }
    impl UnassignGroupRoleParams {
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
    impl ::std::convert::TryFrom<UnassignGroupRoleParams> for super::UnassignGroupRoleParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnassignGroupRoleParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                group_id: value.group_id?,
                role_id: value.role_id?,
            })
        }
    }
    impl ::std::convert::From<super::UnassignGroupRoleParams> for UnassignGroupRoleParams {
        fn from(value: super::UnassignGroupRoleParams) -> Self {
            Self {
                group_id: Ok(value.group_id),
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
