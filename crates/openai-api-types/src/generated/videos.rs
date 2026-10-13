pub use crate::common::{Error, ErrorResponse, MisalignmentErrorDetailsResource, OrderEnum, MisalignmentErrorType, MisalignmentSteer};
#[doc = "Parameters for creating a character from an uploaded video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoCharacterBody {
    #[doc = "Display name for this API character."]
    pub name: CreateVideoCharacterBodyName,
    #[doc = "Video file used to create a character."]
    pub video: ::std::string::String,
}
impl CreateVideoCharacterBody {
    pub fn builder() -> builder::CreateVideoCharacterBody {
        Default::default()
    }
}
#[doc = "Display name for this API character."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoCharacterBodyName(::std::string::String);
impl ::std::ops::Deref for CreateVideoCharacterBodyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoCharacterBodyName> for ::std::string::String {
    fn from(value: CreateVideoCharacterBodyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoCharacterBodyName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 80usize {
            return Err("longer than 80 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoCharacterBodyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoCharacterBodyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoCharacterBodyName {
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
#[doc = "JSON parameters for editing an existing generated video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoEditJsonBody {
    #[doc = "Text prompt that describes how to edit the source video."]
    pub prompt: CreateVideoEditJsonBodyPrompt,
    #[doc = "Reference to the completed video to edit."]
    pub video: VideoReferenceInputParam,
}
impl CreateVideoEditJsonBody {
    pub fn builder() -> builder::CreateVideoEditJsonBody {
        Default::default()
    }
}
#[doc = "Text prompt that describes how to edit the source video."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoEditJsonBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoEditJsonBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoEditJsonBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoEditJsonBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoEditJsonBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoEditJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoEditJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoEditJsonBodyPrompt {
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
#[doc = "Parameters for editing an existing generated video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoEditMultipartBody {
    #[doc = "Text prompt that describes how to edit the source video."]
    pub prompt: CreateVideoEditMultipartBodyPrompt,
    pub video: CreateVideoEditMultipartBodyVideo,
}
impl CreateVideoEditMultipartBody {
    pub fn builder() -> builder::CreateVideoEditMultipartBody {
        Default::default()
    }
}
#[doc = "Text prompt that describes how to edit the source video."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoEditMultipartBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoEditMultipartBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoEditMultipartBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoEditMultipartBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoEditMultipartBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoEditMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoEditMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoEditMultipartBodyPrompt {
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
#[doc = "`CreateVideoEditMultipartBodyVideo`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateVideoEditMultipartBodyVideo {
    Binary(::std::string::String),
    VideoReferenceInputParam(VideoReferenceInputParam),
}
impl ::std::convert::From<VideoReferenceInputParam> for CreateVideoEditMultipartBodyVideo {
    fn from(value: VideoReferenceInputParam) -> Self {
        Self::VideoReferenceInputParam(value)
    }
}
#[doc = "JSON parameters for extending an existing generated video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoExtendJsonBody {
    #[doc = "Updated text prompt that directs the extension generation."]
    pub prompt: CreateVideoExtendJsonBodyPrompt,
    #[doc = "Length of the newly generated extension segment in seconds (allowed values: 4, 8, 12, 16, 20)."]
    pub seconds: VideoSeconds,
    #[doc = "Reference to the completed video to extend."]
    pub video: VideoReferenceInputParam,
}
impl CreateVideoExtendJsonBody {
    pub fn builder() -> builder::CreateVideoExtendJsonBody {
        Default::default()
    }
}
#[doc = "Updated text prompt that directs the extension generation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoExtendJsonBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoExtendJsonBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoExtendJsonBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoExtendJsonBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoExtendJsonBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoExtendJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoExtendJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoExtendJsonBodyPrompt {
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
#[doc = "Multipart parameters for extending an existing generated video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoExtendMultipartBody {
    #[doc = "Updated text prompt that directs the extension generation."]
    pub prompt: CreateVideoExtendMultipartBodyPrompt,
    #[doc = "Length of the newly generated extension segment in seconds (allowed values: 4, 8, 12, 16, 20)."]
    pub seconds: VideoSeconds,
    pub video: CreateVideoExtendMultipartBodyVideo,
}
impl CreateVideoExtendMultipartBody {
    pub fn builder() -> builder::CreateVideoExtendMultipartBody {
        Default::default()
    }
}
#[doc = "Updated text prompt that directs the extension generation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoExtendMultipartBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoExtendMultipartBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoExtendMultipartBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoExtendMultipartBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoExtendMultipartBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoExtendMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoExtendMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoExtendMultipartBodyPrompt {
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
#[doc = "`CreateVideoExtendMultipartBodyVideo`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateVideoExtendMultipartBodyVideo {
    VideoReferenceInputParam(VideoReferenceInputParam),
    Binary(::std::string::String),
}
impl ::std::convert::From<VideoReferenceInputParam> for CreateVideoExtendMultipartBodyVideo {
    fn from(value: VideoReferenceInputParam) -> Self {
        Self::VideoReferenceInputParam(value)
    }
}
#[doc = "JSON parameters for creating a new video generation job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoJsonBody {
    #[doc = "Optional reference object that guides generation. Provide exactly one of `image_url` or `file_id`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_reference: ::std::option::Option<ImageRefParam2>,
    #[doc = "The video generation model to use (allowed values: sora-2, sora-2-pro). Defaults to `sora-2`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<VideoModel>,
    #[doc = "Text prompt that describes the video to generate."]
    pub prompt: CreateVideoJsonBodyPrompt,
    #[doc = "Clip duration in seconds (allowed values: 4, 8, 12). Defaults to 4 seconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seconds: ::std::option::Option<VideoSeconds>,
    #[doc = "Output resolution formatted as width x height (allowed values: 720x1280, 1280x720, 1024x1792, 1792x1024). Defaults to 720x1280."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<VideoSize>,
}
impl CreateVideoJsonBody {
    pub fn builder() -> builder::CreateVideoJsonBody {
        Default::default()
    }
}
#[doc = "Text prompt that describes the video to generate."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoJsonBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoJsonBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoJsonBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoJsonBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoJsonBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoJsonBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoJsonBodyPrompt {
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
#[doc = "Multipart parameters for creating a new video generation job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoMultipartBody {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_reference: ::std::option::Option<CreateVideoMultipartBodyInputReference>,
    #[doc = "The video generation model to use (allowed values: sora-2, sora-2-pro). Defaults to `sora-2`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<VideoModel>,
    #[doc = "Text prompt that describes the video to generate."]
    pub prompt: CreateVideoMultipartBodyPrompt,
    #[doc = "Clip duration in seconds (allowed values: 4, 8, 12). Defaults to 4 seconds."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seconds: ::std::option::Option<VideoSeconds>,
    #[doc = "Output resolution formatted as width x height (allowed values: 720x1280, 1280x720, 1024x1792, 1792x1024). Defaults to 720x1280."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<VideoSize>,
}
impl CreateVideoMultipartBody {
    pub fn builder() -> builder::CreateVideoMultipartBody {
        Default::default()
    }
}
#[doc = "`CreateVideoMultipartBodyInputReference`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateVideoMultipartBodyInputReference {
    Binary(::std::string::String),
    ImageRefParam2(ImageRefParam2),
}
impl ::std::convert::From<ImageRefParam2> for CreateVideoMultipartBodyInputReference {
    fn from(value: ImageRefParam2) -> Self {
        Self::ImageRefParam2(value)
    }
}
#[doc = "Text prompt that describes the video to generate."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoMultipartBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoMultipartBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoMultipartBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoMultipartBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoMultipartBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoMultipartBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoMultipartBodyPrompt {
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
#[doc = "Parameters for remixing an existing generated video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoRemixBody {
    #[doc = "Updated text prompt that directs the remix generation."]
    pub prompt: CreateVideoRemixBodyPrompt,
}
impl CreateVideoRemixBody {
    pub fn builder() -> builder::CreateVideoRemixBody {
        Default::default()
    }
}
#[doc = "Updated text prompt that directs the remix generation."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateVideoRemixBodyPrompt(::std::string::String);
impl ::std::ops::Deref for CreateVideoRemixBodyPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CreateVideoRemixBodyPrompt> for ::std::string::String {
    fn from(value: CreateVideoRemixBodyPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CreateVideoRemixBodyPrompt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32000usize {
            return Err("longer than 32000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CreateVideoRemixBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateVideoRemixBodyPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CreateVideoRemixBodyPrompt {
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
#[doc = "`CreateVideoRemixParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateVideoRemixParams {
    pub video_id: ::std::string::String,
}
impl CreateVideoRemixParams {
    pub fn builder() -> builder::CreateVideoRemixParams {
        Default::default()
    }
}
#[doc = "`DeleteVideoParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeleteVideoParams {
    pub video_id: ::std::string::String,
}
impl DeleteVideoParams {
    pub fn builder() -> builder::DeleteVideoParams {
        Default::default()
    }
}
#[doc = "Confirmation payload returned after deleting a video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DeletedVideoResource {
    #[doc = "Indicates that the video resource was deleted."]
    pub deleted: bool,
    #[doc = "Identifier of the deleted video."]
    pub id: ::std::string::String,
    #[doc = "The object type that signals the deletion response."]
    pub object: DeletedVideoResourceObject,
}
impl DeletedVideoResource {
    pub fn builder() -> builder::DeletedVideoResource {
        Default::default()
    }
}
#[doc = "The object type that signals the deletion response."]
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
pub enum DeletedVideoResourceObject {
    #[serde(rename = "video.deleted")]
    VideoDeleted,
}
impl ::std::fmt::Display for DeletedVideoResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VideoDeleted => f.write_str("video.deleted"),
        }
    }
}
impl ::std::str::FromStr for DeletedVideoResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "video.deleted" => Ok(Self::VideoDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeletedVideoResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeletedVideoResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An error that occurred while generating the response."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Error2 {
    #[doc = "A machine-readable error code that was returned."]
    pub code: ::std::string::String,
    #[doc = "The Retry-After and Retry-After-Ms headers returned with the original error, if any."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "A human-readable description of the error that was returned."]
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub misalignment:
        ::std::option::Option<::openai_api_types::common::MisalignmentErrorDetailsResource>,
}
impl Error2 {
    pub fn builder() -> builder::Error2 {
        Default::default()
    }
}
#[doc = "`GetVideoCharacterParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVideoCharacterParams {
    pub character_id: ::std::string::String,
}
impl GetVideoCharacterParams {
    pub fn builder() -> builder::GetVideoCharacterParams {
        Default::default()
    }
}
#[doc = "`GetVideoParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct GetVideoParams {
    pub video_id: ::std::string::String,
}
impl GetVideoParams {
    pub fn builder() -> builder::GetVideoParams {
        Default::default()
    }
}
#[doc = "`ImageRefParam2`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ImageRefParam2 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "A fully qualified URL or base64-encoded data URL."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<ImageRefParam2ImageUrl>,
}
impl ImageRefParam2 {
    pub fn builder() -> builder::ImageRefParam2 {
        Default::default()
    }
}
#[doc = "A fully qualified URL or base64-encoded data URL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ImageRefParam2ImageUrl(::std::string::String);
impl ::std::ops::Deref for ImageRefParam2ImageUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ImageRefParam2ImageUrl> for ::std::string::String {
    fn from(value: ImageRefParam2ImageUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ImageRefParam2ImageUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 20971520usize {
            return Err("longer than 20971520 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ImageRefParam2ImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageRefParam2ImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ImageRefParam2ImageUrl {
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
#[doc = "`ListVideosParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct ListVideosParams {
    #[doc = "Identifier for the last item from the previous pagination request"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub after: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<::openai_api_types::common::OrderEnum>,
}
impl ListVideosParams {
    pub fn builder() -> builder::ListVideosParams {
        Default::default()
    }
}
#[doc = "`RetrieveVideoContentParams`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct RetrieveVideoContentParams {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub variant: ::std::option::Option<VideoContentVariant>,
    pub video_id: ::std::string::String,
}
impl RetrieveVideoContentParams {
    pub fn builder() -> builder::RetrieveVideoContentParams {
        Default::default()
    }
}
#[doc = "`VideoCharacterResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VideoCharacterResource {
    #[doc = "Unix timestamp (in seconds) when the character was created."]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl VideoCharacterResource {
    pub fn builder() -> builder::VideoCharacterResource {
        Default::default()
    }
}
#[doc = "`VideoContentVariant`"]
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
pub enum VideoContentVariant {
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "spritesheet")]
    Spritesheet,
}
impl ::std::fmt::Display for VideoContentVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Video => f.write_str("video"),
            Self::Thumbnail => f.write_str("thumbnail"),
            Self::Spritesheet => f.write_str("spritesheet"),
        }
    }
}
impl ::std::str::FromStr for VideoContentVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "video" => Ok(Self::Video),
            "thumbnail" => Ok(Self::Thumbnail),
            "spritesheet" => Ok(Self::Spritesheet),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoContentVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoContentVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideoListResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VideoListResource {
    #[doc = "A list of items"]
    pub data: ::std::vec::Vec<VideoResource>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub first_id: ::std::option::Option<::std::string::String>,
    #[doc = "Whether there are more items available."]
    pub has_more: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub last_id: ::std::option::Option<::std::string::String>,
    #[doc = "The type of object returned, must be `list`."]
    pub object: VideoListResourceObject,
}
impl VideoListResource {
    pub fn builder() -> builder::VideoListResource {
        Default::default()
    }
}
#[doc = "The type of object returned, must be `list`."]
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
pub enum VideoListResourceObject {
    #[serde(rename = "list")]
    List,
}
impl ::std::fmt::Display for VideoListResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::List => f.write_str("list"),
        }
    }
}
impl ::std::str::FromStr for VideoListResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "list" => Ok(Self::List),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoListResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideoModel`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct VideoModel(pub ::std::string::String);
impl ::std::ops::Deref for VideoModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VideoModel> for ::std::string::String {
    fn from(value: VideoModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for VideoModel {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for VideoModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for VideoModel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[doc = "Reference to the completed video."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VideoReferenceInputParam {
    #[doc = "The identifier of the completed video."]
    pub id: ::std::string::String,
}
impl VideoReferenceInputParam {
    pub fn builder() -> builder::VideoReferenceInputParam {
        Default::default()
    }
}
#[doc = "Structured information describing a generated video job."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VideoResource {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub completed_at: ::std::option::Option<i64>,
    #[doc = "Unix timestamp (seconds) for when the job was created."]
    pub created_at: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub error: ::std::option::Option<Error2>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub expires_at: ::std::option::Option<i64>,
    #[doc = "Unique identifier for the video job."]
    pub id: ::std::string::String,
    #[doc = "The video generation model that produced the job."]
    pub model: VideoModel,
    #[doc = "The object type, which is always `video`."]
    pub object: VideoResourceObject,
    #[doc = "Approximate completion percentage for the generation task."]
    pub progress: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub prompt: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub remixed_from_video_id: ::std::option::Option<::std::string::String>,
    #[doc = "Duration of the generated clip in seconds. For extensions, this is the stitched total duration."]
    pub seconds: ::std::string::String,
    #[doc = "The resolution of the generated video."]
    pub size: VideoSize,
    #[doc = "Current lifecycle status of the video job."]
    pub status: VideoStatus,
}
impl VideoResource {
    pub fn builder() -> builder::VideoResource {
        Default::default()
    }
}
#[doc = "The object type, which is always `video`."]
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
pub enum VideoResourceObject {
    #[serde(rename = "video")]
    Video,
}
impl ::std::fmt::Display for VideoResourceObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Video => f.write_str("video"),
        }
    }
}
impl ::std::str::FromStr for VideoResourceObject {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoResourceObject {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideoSeconds`"]
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
pub enum VideoSeconds {
    #[serde(rename = "4")]
    X4,
    #[serde(rename = "8")]
    X8,
    #[serde(rename = "12")]
    X12,
}
impl ::std::fmt::Display for VideoSeconds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X4 => f.write_str("4"),
            Self::X8 => f.write_str("8"),
            Self::X12 => f.write_str("12"),
        }
    }
}
impl ::std::str::FromStr for VideoSeconds {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "4" => Ok(Self::X4),
            "8" => Ok(Self::X8),
            "12" => Ok(Self::X12),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoSeconds {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoSeconds {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideoSize`"]
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
pub enum VideoSize {
    #[serde(rename = "720x1280")]
    X720x1280,
    #[serde(rename = "1280x720")]
    X1280x720,
    #[serde(rename = "1024x1792")]
    X1024x1792,
    #[serde(rename = "1792x1024")]
    X1792x1024,
}
impl ::std::fmt::Display for VideoSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X720x1280 => f.write_str("720x1280"),
            Self::X1280x720 => f.write_str("1280x720"),
            Self::X1024x1792 => f.write_str("1024x1792"),
            Self::X1792x1024 => f.write_str("1792x1024"),
        }
    }
}
impl ::std::str::FromStr for VideoSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "720x1280" => Ok(Self::X720x1280),
            "1280x720" => Ok(Self::X1280x720),
            "1024x1792" => Ok(Self::X1024x1792),
            "1792x1024" => Ok(Self::X1792x1024),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoSize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideoStatus`"]
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
pub enum VideoStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for VideoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::InProgress => f.write_str("in_progress"),
            Self::Completed => f.write_str("completed"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for VideoStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VideoStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VideoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VideosApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct VideosApiSchema {
    #[serde(rename = "CreateVideoCharacterError400")]
    pub create_video_character_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoCharacterError404")]
    pub create_video_character_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoCharacterError500")]
    pub create_video_character_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoCharacterError504")]
    pub create_video_character_error504: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoCharacterRequest")]
    pub create_video_character_request: CreateVideoCharacterBody,
    #[serde(rename = "CreateVideoCharacterResponse")]
    pub create_video_character_response: VideoCharacterResource,
    #[serde(rename = "CreateVideoEditError400")]
    pub create_video_edit_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoEditError404")]
    pub create_video_edit_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoEditError429")]
    pub create_video_edit_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoEditError500")]
    pub create_video_edit_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoEditError503")]
    pub create_video_edit_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoEditMultipartFormDataRequest")]
    pub create_video_edit_multipart_form_data_request: CreateVideoEditMultipartBody,
    #[serde(rename = "CreateVideoEditRequest")]
    pub create_video_edit_request: CreateVideoEditJsonBody,
    #[serde(rename = "CreateVideoEditResponse")]
    pub create_video_edit_response: VideoResource,
    #[serde(rename = "CreateVideoError400")]
    pub create_video_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoError404")]
    pub create_video_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoError429")]
    pub create_video_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoError500")]
    pub create_video_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoError503")]
    pub create_video_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendError400")]
    pub create_video_extend_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendError404")]
    pub create_video_extend_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendError429")]
    pub create_video_extend_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendError500")]
    pub create_video_extend_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendError503")]
    pub create_video_extend_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoExtendMultipartFormDataRequest")]
    pub create_video_extend_multipart_form_data_request: CreateVideoExtendMultipartBody,
    #[serde(rename = "CreateVideoExtendRequest")]
    pub create_video_extend_request: CreateVideoExtendJsonBody,
    #[serde(rename = "CreateVideoExtendResponse")]
    pub create_video_extend_response: VideoResource,
    #[serde(rename = "CreateVideoMultipartFormDataRequest")]
    pub create_video_multipart_form_data_request: CreateVideoMultipartBody,
    #[serde(rename = "CreateVideoRemixError400")]
    pub create_video_remix_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoRemixError404")]
    pub create_video_remix_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoRemixError429")]
    pub create_video_remix_error429: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoRemixError500")]
    pub create_video_remix_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoRemixError503")]
    pub create_video_remix_error503: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "CreateVideoRemixMultipartFormDataRequest")]
    pub create_video_remix_multipart_form_data_request: CreateVideoRemixBody,
    #[serde(rename = "CreateVideoRemixParams")]
    pub create_video_remix_params: CreateVideoRemixParams,
    #[serde(rename = "CreateVideoRemixRequest")]
    pub create_video_remix_request: CreateVideoRemixBody,
    #[serde(rename = "CreateVideoRemixResponse")]
    pub create_video_remix_response: VideoResource,
    #[serde(rename = "CreateVideoRequest")]
    pub create_video_request: CreateVideoJsonBody,
    #[serde(rename = "CreateVideoResponse")]
    pub create_video_response: VideoResource,
    #[serde(rename = "DeleteVideoError400")]
    pub delete_video_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVideoError404")]
    pub delete_video_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVideoError500")]
    pub delete_video_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "DeleteVideoParams")]
    pub delete_video_params: DeleteVideoParams,
    #[serde(rename = "DeleteVideoResponse")]
    pub delete_video_response: DeletedVideoResource,
    #[serde(rename = "GetVideoCharacterError400")]
    pub get_video_character_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVideoCharacterError404")]
    pub get_video_character_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVideoCharacterParams")]
    pub get_video_character_params: GetVideoCharacterParams,
    #[serde(rename = "GetVideoCharacterResponse")]
    pub get_video_character_response: VideoCharacterResource,
    #[serde(rename = "GetVideoError400")]
    pub get_video_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVideoError404")]
    pub get_video_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "GetVideoParams")]
    pub get_video_params: GetVideoParams,
    #[serde(rename = "GetVideoResponse")]
    pub get_video_response: VideoResource,
    #[serde(rename = "ListVideosError400")]
    pub list_videos_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVideosError404")]
    pub list_videos_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "ListVideosParams")]
    pub list_videos_params: ListVideosParams,
    #[serde(rename = "ListVideosResponse")]
    pub list_videos_response: VideoListResource,
    #[serde(rename = "RetrieveVideoContentError400")]
    pub retrieve_video_content_error400: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVideoContentError404")]
    pub retrieve_video_content_error404: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVideoContentError500")]
    pub retrieve_video_content_error500: ::openai_api_types::common::ErrorResponse,
    #[serde(rename = "RetrieveVideoContentParams")]
    pub retrieve_video_content_params: RetrieveVideoContentParams,
    #[serde(rename = "RetrieveVideoContentResponse")]
    pub retrieve_video_content_response: ::std::string::String,
}
impl VideosApiSchema {
    pub fn builder() -> builder::VideosApiSchema {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CreateVideoCharacterBody {
        name: ::std::result::Result<super::CreateVideoCharacterBodyName, ::std::string::String>,
        video: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoCharacterBody {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                video: Err("no value supplied for video".to_string()),
            }
        }
    }
    impl CreateVideoCharacterBody {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoCharacterBodyName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoCharacterBody> for super::CreateVideoCharacterBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoCharacterBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoCharacterBody> for CreateVideoCharacterBody {
        fn from(value: super::CreateVideoCharacterBody) -> Self {
            Self {
                name: Ok(value.name),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoEditJsonBody {
        prompt: ::std::result::Result<super::CreateVideoEditJsonBodyPrompt, ::std::string::String>,
        video: ::std::result::Result<super::VideoReferenceInputParam, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoEditJsonBody {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
                video: Err("no value supplied for video".to_string()),
            }
        }
    }
    impl CreateVideoEditJsonBody {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoEditJsonBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoReferenceInputParam>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoEditJsonBody> for super::CreateVideoEditJsonBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoEditJsonBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoEditJsonBody> for CreateVideoEditJsonBody {
        fn from(value: super::CreateVideoEditJsonBody) -> Self {
            Self {
                prompt: Ok(value.prompt),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoEditMultipartBody {
        prompt:
            ::std::result::Result<super::CreateVideoEditMultipartBodyPrompt, ::std::string::String>,
        video:
            ::std::result::Result<super::CreateVideoEditMultipartBodyVideo, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoEditMultipartBody {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
                video: Err("no value supplied for video".to_string()),
            }
        }
    }
    impl CreateVideoEditMultipartBody {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoEditMultipartBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoEditMultipartBodyVideo>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoEditMultipartBody> for super::CreateVideoEditMultipartBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoEditMultipartBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoEditMultipartBody> for CreateVideoEditMultipartBody {
        fn from(value: super::CreateVideoEditMultipartBody) -> Self {
            Self {
                prompt: Ok(value.prompt),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoExtendJsonBody {
        prompt:
            ::std::result::Result<super::CreateVideoExtendJsonBodyPrompt, ::std::string::String>,
        seconds: ::std::result::Result<super::VideoSeconds, ::std::string::String>,
        video: ::std::result::Result<super::VideoReferenceInputParam, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoExtendJsonBody {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
                seconds: Err("no value supplied for seconds".to_string()),
                video: Err("no value supplied for video".to_string()),
            }
        }
    }
    impl CreateVideoExtendJsonBody {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoExtendJsonBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoSeconds>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoReferenceInputParam>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoExtendJsonBody> for super::CreateVideoExtendJsonBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoExtendJsonBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
                seconds: value.seconds?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoExtendJsonBody> for CreateVideoExtendJsonBody {
        fn from(value: super::CreateVideoExtendJsonBody) -> Self {
            Self {
                prompt: Ok(value.prompt),
                seconds: Ok(value.seconds),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoExtendMultipartBody {
        prompt: ::std::result::Result<
            super::CreateVideoExtendMultipartBodyPrompt,
            ::std::string::String,
        >,
        seconds: ::std::result::Result<super::VideoSeconds, ::std::string::String>,
        video: ::std::result::Result<
            super::CreateVideoExtendMultipartBodyVideo,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CreateVideoExtendMultipartBody {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
                seconds: Err("no value supplied for seconds".to_string()),
                video: Err("no value supplied for video".to_string()),
            }
        }
    }
    impl CreateVideoExtendMultipartBody {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoExtendMultipartBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoSeconds>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoExtendMultipartBodyVideo>,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoExtendMultipartBody>
        for super::CreateVideoExtendMultipartBody
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoExtendMultipartBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
                seconds: value.seconds?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoExtendMultipartBody>
        for CreateVideoExtendMultipartBody
    {
        fn from(value: super::CreateVideoExtendMultipartBody) -> Self {
            Self {
                prompt: Ok(value.prompt),
                seconds: Ok(value.seconds),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoJsonBody {
        input_reference: ::std::result::Result<
            ::std::option::Option<super::ImageRefParam2>,
            ::std::string::String,
        >,
        model:
            ::std::result::Result<::std::option::Option<super::VideoModel>, ::std::string::String>,
        prompt: ::std::result::Result<super::CreateVideoJsonBodyPrompt, ::std::string::String>,
        seconds: ::std::result::Result<
            ::std::option::Option<super::VideoSeconds>,
            ::std::string::String,
        >,
        size: ::std::result::Result<::std::option::Option<super::VideoSize>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoJsonBody {
        fn default() -> Self {
            Self {
                input_reference: Ok(Default::default()),
                model: Ok(Default::default()),
                prompt: Err("no value supplied for prompt".to_string()),
                seconds: Ok(Default::default()),
                size: Ok(Default::default()),
            }
        }
    }
    impl CreateVideoJsonBody {
        pub fn input_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageRefParam2>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_reference: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoModel>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoJsonBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoSeconds>>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoSize>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoJsonBody> for super::CreateVideoJsonBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoJsonBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_reference: value.input_reference?,
                model: value.model?,
                prompt: value.prompt?,
                seconds: value.seconds?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoJsonBody> for CreateVideoJsonBody {
        fn from(value: super::CreateVideoJsonBody) -> Self {
            Self {
                input_reference: Ok(value.input_reference),
                model: Ok(value.model),
                prompt: Ok(value.prompt),
                seconds: Ok(value.seconds),
                size: Ok(value.size),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoMultipartBody {
        input_reference: ::std::result::Result<
            ::std::option::Option<super::CreateVideoMultipartBodyInputReference>,
            ::std::string::String,
        >,
        model:
            ::std::result::Result<::std::option::Option<super::VideoModel>, ::std::string::String>,
        prompt: ::std::result::Result<super::CreateVideoMultipartBodyPrompt, ::std::string::String>,
        seconds: ::std::result::Result<
            ::std::option::Option<super::VideoSeconds>,
            ::std::string::String,
        >,
        size: ::std::result::Result<::std::option::Option<super::VideoSize>, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoMultipartBody {
        fn default() -> Self {
            Self {
                input_reference: Ok(Default::default()),
                model: Ok(Default::default()),
                prompt: Err("no value supplied for prompt".to_string()),
                seconds: Ok(Default::default()),
                size: Ok(Default::default()),
            }
        }
    }
    impl CreateVideoMultipartBody {
        pub fn input_reference<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CreateVideoMultipartBodyInputReference>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.input_reference = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_reference: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoModel>>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoMultipartBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoSeconds>>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoSize>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoMultipartBody> for super::CreateVideoMultipartBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoMultipartBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_reference: value.input_reference?,
                model: value.model?,
                prompt: value.prompt?,
                seconds: value.seconds?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoMultipartBody> for CreateVideoMultipartBody {
        fn from(value: super::CreateVideoMultipartBody) -> Self {
            Self {
                input_reference: Ok(value.input_reference),
                model: Ok(value.model),
                prompt: Ok(value.prompt),
                seconds: Ok(value.seconds),
                size: Ok(value.size),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoRemixBody {
        prompt: ::std::result::Result<super::CreateVideoRemixBodyPrompt, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoRemixBody {
        fn default() -> Self {
            Self {
                prompt: Err("no value supplied for prompt".to_string()),
            }
        }
    }
    impl CreateVideoRemixBody {
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoRemixBodyPrompt>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoRemixBody> for super::CreateVideoRemixBody {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoRemixBody,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                prompt: value.prompt?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoRemixBody> for CreateVideoRemixBody {
        fn from(value: super::CreateVideoRemixBody) -> Self {
            Self {
                prompt: Ok(value.prompt),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateVideoRemixParams {
        video_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CreateVideoRemixParams {
        fn default() -> Self {
            Self {
                video_id: Err("no value supplied for video_id".to_string()),
            }
        }
    }
    impl CreateVideoRemixParams {
        pub fn video_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.video_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CreateVideoRemixParams> for super::CreateVideoRemixParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CreateVideoRemixParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                video_id: value.video_id?,
            })
        }
    }
    impl ::std::convert::From<super::CreateVideoRemixParams> for CreateVideoRemixParams {
        fn from(value: super::CreateVideoRemixParams) -> Self {
            Self {
                video_id: Ok(value.video_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteVideoParams {
        video_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteVideoParams {
        fn default() -> Self {
            Self {
                video_id: Err("no value supplied for video_id".to_string()),
            }
        }
    }
    impl DeleteVideoParams {
        pub fn video_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.video_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteVideoParams> for super::DeleteVideoParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteVideoParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                video_id: value.video_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteVideoParams> for DeleteVideoParams {
        fn from(value: super::DeleteVideoParams) -> Self {
            Self {
                video_id: Ok(value.video_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeletedVideoResource {
        deleted: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        object: ::std::result::Result<super::DeletedVideoResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for DeletedVideoResource {
        fn default() -> Self {
            Self {
                deleted: Err("no value supplied for deleted".to_string()),
                id: Err("no value supplied for id".to_string()),
                object: Err("no value supplied for object".to_string()),
            }
        }
    }
    impl DeletedVideoResource {
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
            T: ::std::convert::TryInto<super::DeletedVideoResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DeletedVideoResource> for super::DeletedVideoResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeletedVideoResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deleted: value.deleted?,
                id: value.id?,
                object: value.object?,
            })
        }
    }
    impl ::std::convert::From<super::DeletedVideoResource> for DeletedVideoResource {
        fn from(value: super::DeletedVideoResource) -> Self {
            Self {
                deleted: Ok(value.deleted),
                id: Ok(value.id),
                object: Ok(value.object),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Error2 {
        code: ::std::result::Result<::std::string::String, ::std::string::String>,
        headers: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        misalignment: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::MisalignmentErrorDetailsResource>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Error2 {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                headers: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
                misalignment: Ok(Default::default()),
            }
        }
    }
    impl Error2 {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn headers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.headers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for headers: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn misalignment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::openai_api_types::common::MisalignmentErrorDetailsResource>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.misalignment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for misalignment: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Error2> for super::Error2 {
        type Error = super::error::ConversionError;
        fn try_from(value: Error2) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                headers: value.headers?,
                message: value.message?,
                misalignment: value.misalignment?,
            })
        }
    }
    impl ::std::convert::From<super::Error2> for Error2 {
        fn from(value: super::Error2) -> Self {
            Self {
                code: Ok(value.code),
                headers: Ok(value.headers),
                message: Ok(value.message),
                misalignment: Ok(value.misalignment),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVideoCharacterParams {
        character_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVideoCharacterParams {
        fn default() -> Self {
            Self {
                character_id: Err("no value supplied for character_id".to_string()),
            }
        }
    }
    impl GetVideoCharacterParams {
        pub fn character_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.character_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for character_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVideoCharacterParams> for super::GetVideoCharacterParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVideoCharacterParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                character_id: value.character_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVideoCharacterParams> for GetVideoCharacterParams {
        fn from(value: super::GetVideoCharacterParams) -> Self {
            Self {
                character_id: Ok(value.character_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetVideoParams {
        video_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetVideoParams {
        fn default() -> Self {
            Self {
                video_id: Err("no value supplied for video_id".to_string()),
            }
        }
    }
    impl GetVideoParams {
        pub fn video_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.video_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetVideoParams> for super::GetVideoParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetVideoParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                video_id: value.video_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetVideoParams> for GetVideoParams {
        fn from(value: super::GetVideoParams) -> Self {
            Self {
                video_id: Ok(value.video_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageRefParam2 {
        file_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        image_url: ::std::result::Result<
            ::std::option::Option<super::ImageRefParam2ImageUrl>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ImageRefParam2 {
        fn default() -> Self {
            Self {
                file_id: Ok(Default::default()),
                image_url: Ok(Default::default()),
            }
        }
    }
    impl ImageRefParam2 {
        pub fn file_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_id: {e}"));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageRefParam2ImageUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ImageRefParam2> for super::ImageRefParam2 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImageRefParam2,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_id: value.file_id?,
                image_url: value.image_url?,
            })
        }
    }
    impl ::std::convert::From<super::ImageRefParam2> for ImageRefParam2 {
        fn from(value: super::ImageRefParam2) -> Self {
            Self {
                file_id: Ok(value.file_id),
                image_url: Ok(value.image_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListVideosParams {
        after: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        limit: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        order: ::std::result::Result<
            ::std::option::Option<::openai_api_types::common::OrderEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListVideosParams {
        fn default() -> Self {
            Self {
                after: Ok(Default::default()),
                limit: Ok(Default::default()),
                order: Ok(Default::default()),
            }
        }
    }
    impl ListVideosParams {
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
                ::std::option::Option<::openai_api_types::common::OrderEnum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for order: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListVideosParams> for super::ListVideosParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListVideosParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                after: value.after?,
                limit: value.limit?,
                order: value.order?,
            })
        }
    }
    impl ::std::convert::From<super::ListVideosParams> for ListVideosParams {
        fn from(value: super::ListVideosParams) -> Self {
            Self {
                after: Ok(value.after),
                limit: Ok(value.limit),
                order: Ok(value.order),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RetrieveVideoContentParams {
        variant: ::std::result::Result<
            ::std::option::Option<super::VideoContentVariant>,
            ::std::string::String,
        >,
        video_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RetrieveVideoContentParams {
        fn default() -> Self {
            Self {
                variant: Ok(Default::default()),
                video_id: Err("no value supplied for video_id".to_string()),
            }
        }
    }
    impl RetrieveVideoContentParams {
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VideoContentVariant>>,
            T::Error: ::std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {e}"));
            self
        }
        pub fn video_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.video_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RetrieveVideoContentParams> for super::RetrieveVideoContentParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RetrieveVideoContentParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                variant: value.variant?,
                video_id: value.video_id?,
            })
        }
    }
    impl ::std::convert::From<super::RetrieveVideoContentParams> for RetrieveVideoContentParams {
        fn from(value: super::RetrieveVideoContentParams) -> Self {
            Self {
                variant: Ok(value.variant),
                video_id: Ok(value.video_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoCharacterResource {
        created_at: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VideoCharacterResource {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl VideoCharacterResource {
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
    impl ::std::convert::TryFrom<VideoCharacterResource> for super::VideoCharacterResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoCharacterResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::VideoCharacterResource> for VideoCharacterResource {
        fn from(value: super::VideoCharacterResource) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoListResource {
        data: ::std::result::Result<::std::vec::Vec<super::VideoResource>, ::std::string::String>,
        first_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_more: ::std::result::Result<bool, ::std::string::String>,
        last_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        object: ::std::result::Result<super::VideoListResourceObject, ::std::string::String>,
    }
    impl ::std::default::Default for VideoListResource {
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
    impl VideoListResource {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VideoResource>>,
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
            T: ::std::convert::TryInto<super::VideoListResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoListResource> for super::VideoListResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoListResource,
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
    impl ::std::convert::From<super::VideoListResource> for VideoListResource {
        fn from(value: super::VideoListResource) -> Self {
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
    pub struct VideoReferenceInputParam {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VideoReferenceInputParam {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl VideoReferenceInputParam {
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
    }
    impl ::std::convert::TryFrom<VideoReferenceInputParam> for super::VideoReferenceInputParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoReferenceInputParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { id: value.id? })
        }
    }
    impl ::std::convert::From<super::VideoReferenceInputParam> for VideoReferenceInputParam {
        fn from(value: super::VideoReferenceInputParam) -> Self {
            Self { id: Ok(value.id) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoResource {
        completed_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        created_at: ::std::result::Result<i64, ::std::string::String>,
        error: ::std::result::Result<::std::option::Option<super::Error2>, ::std::string::String>,
        expires_at: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        model: ::std::result::Result<super::VideoModel, ::std::string::String>,
        object: ::std::result::Result<super::VideoResourceObject, ::std::string::String>,
        progress: ::std::result::Result<i64, ::std::string::String>,
        prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        remixed_from_video_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        seconds: ::std::result::Result<::std::string::String, ::std::string::String>,
        size: ::std::result::Result<super::VideoSize, ::std::string::String>,
        status: ::std::result::Result<super::VideoStatus, ::std::string::String>,
    }
    impl ::std::default::Default for VideoResource {
        fn default() -> Self {
            Self {
                completed_at: Err("no value supplied for completed_at".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                error: Err("no value supplied for error".to_string()),
                expires_at: Err("no value supplied for expires_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                model: Err("no value supplied for model".to_string()),
                object: Err("no value supplied for object".to_string()),
                progress: Err("no value supplied for progress".to_string()),
                prompt: Err("no value supplied for prompt".to_string()),
                remixed_from_video_id: Err(
                    "no value supplied for remixed_from_video_id".to_string()
                ),
                seconds: Err("no value supplied for seconds".to_string()),
                size: Err("no value supplied for size".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl VideoResource {
        pub fn completed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.completed_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed_at: {e}"));
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
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Error2>>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
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
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoModel>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn object<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResourceObject>,
            T::Error: ::std::fmt::Display,
        {
            self.object = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object: {e}"));
            self
        }
        pub fn progress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
        pub fn remixed_from_video_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remixed_from_video_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for remixed_from_video_id: {e}")
            });
            self
        }
        pub fn seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seconds: {e}"));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoSize>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VideoResource> for super::VideoResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideoResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completed_at: value.completed_at?,
                created_at: value.created_at?,
                error: value.error?,
                expires_at: value.expires_at?,
                id: value.id?,
                model: value.model?,
                object: value.object?,
                progress: value.progress?,
                prompt: value.prompt?,
                remixed_from_video_id: value.remixed_from_video_id?,
                seconds: value.seconds?,
                size: value.size?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::VideoResource> for VideoResource {
        fn from(value: super::VideoResource) -> Self {
            Self {
                completed_at: Ok(value.completed_at),
                created_at: Ok(value.created_at),
                error: Ok(value.error),
                expires_at: Ok(value.expires_at),
                id: Ok(value.id),
                model: Ok(value.model),
                object: Ok(value.object),
                progress: Ok(value.progress),
                prompt: Ok(value.prompt),
                remixed_from_video_id: Ok(value.remixed_from_video_id),
                seconds: Ok(value.seconds),
                size: Ok(value.size),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideosApiSchema {
        create_video_character_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_character_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_character_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_character_error504:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_character_request:
            ::std::result::Result<super::CreateVideoCharacterBody, ::std::string::String>,
        create_video_character_response:
            ::std::result::Result<super::VideoCharacterResource, ::std::string::String>,
        create_video_edit_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_edit_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_edit_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_edit_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_edit_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_edit_multipart_form_data_request:
            ::std::result::Result<super::CreateVideoEditMultipartBody, ::std::string::String>,
        create_video_edit_request:
            ::std::result::Result<super::CreateVideoEditJsonBody, ::std::string::String>,
        create_video_edit_response:
            ::std::result::Result<super::VideoResource, ::std::string::String>,
        create_video_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_extend_multipart_form_data_request:
            ::std::result::Result<super::CreateVideoExtendMultipartBody, ::std::string::String>,
        create_video_extend_request:
            ::std::result::Result<super::CreateVideoExtendJsonBody, ::std::string::String>,
        create_video_extend_response:
            ::std::result::Result<super::VideoResource, ::std::string::String>,
        create_video_multipart_form_data_request:
            ::std::result::Result<super::CreateVideoMultipartBody, ::std::string::String>,
        create_video_remix_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_remix_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_remix_error429:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_remix_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_remix_error503:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        create_video_remix_multipart_form_data_request:
            ::std::result::Result<super::CreateVideoRemixBody, ::std::string::String>,
        create_video_remix_params:
            ::std::result::Result<super::CreateVideoRemixParams, ::std::string::String>,
        create_video_remix_request:
            ::std::result::Result<super::CreateVideoRemixBody, ::std::string::String>,
        create_video_remix_response:
            ::std::result::Result<super::VideoResource, ::std::string::String>,
        create_video_request:
            ::std::result::Result<super::CreateVideoJsonBody, ::std::string::String>,
        create_video_response: ::std::result::Result<super::VideoResource, ::std::string::String>,
        delete_video_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_video_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_video_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        delete_video_params: ::std::result::Result<super::DeleteVideoParams, ::std::string::String>,
        delete_video_response:
            ::std::result::Result<super::DeletedVideoResource, ::std::string::String>,
        get_video_character_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_video_character_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_video_character_params:
            ::std::result::Result<super::GetVideoCharacterParams, ::std::string::String>,
        get_video_character_response:
            ::std::result::Result<super::VideoCharacterResource, ::std::string::String>,
        get_video_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_video_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        get_video_params: ::std::result::Result<super::GetVideoParams, ::std::string::String>,
        get_video_response: ::std::result::Result<super::VideoResource, ::std::string::String>,
        list_videos_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_videos_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        list_videos_params: ::std::result::Result<super::ListVideosParams, ::std::string::String>,
        list_videos_response:
            ::std::result::Result<super::VideoListResource, ::std::string::String>,
        retrieve_video_content_error400:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_video_content_error404:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_video_content_error500:
            ::std::result::Result<::openai_api_types::common::ErrorResponse, ::std::string::String>,
        retrieve_video_content_params:
            ::std::result::Result<super::RetrieveVideoContentParams, ::std::string::String>,
        retrieve_video_content_response:
            ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for VideosApiSchema {
        fn default() -> Self {
            Self {
                create_video_character_error400: Err(
                    "no value supplied for create_video_character_error400".to_string(),
                ),
                create_video_character_error404: Err(
                    "no value supplied for create_video_character_error404".to_string(),
                ),
                create_video_character_error500: Err(
                    "no value supplied for create_video_character_error500".to_string(),
                ),
                create_video_character_error504: Err(
                    "no value supplied for create_video_character_error504".to_string(),
                ),
                create_video_character_request: Err(
                    "no value supplied for create_video_character_request".to_string(),
                ),
                create_video_character_response: Err(
                    "no value supplied for create_video_character_response".to_string(),
                ),
                create_video_edit_error400: Err(
                    "no value supplied for create_video_edit_error400".to_string()
                ),
                create_video_edit_error404: Err(
                    "no value supplied for create_video_edit_error404".to_string()
                ),
                create_video_edit_error429: Err(
                    "no value supplied for create_video_edit_error429".to_string()
                ),
                create_video_edit_error500: Err(
                    "no value supplied for create_video_edit_error500".to_string()
                ),
                create_video_edit_error503: Err(
                    "no value supplied for create_video_edit_error503".to_string()
                ),
                create_video_edit_multipart_form_data_request: Err(
                    "no value supplied for create_video_edit_multipart_form_data_request"
                        .to_string(),
                ),
                create_video_edit_request: Err(
                    "no value supplied for create_video_edit_request".to_string()
                ),
                create_video_edit_response: Err(
                    "no value supplied for create_video_edit_response".to_string()
                ),
                create_video_error400: Err(
                    "no value supplied for create_video_error400".to_string()
                ),
                create_video_error404: Err(
                    "no value supplied for create_video_error404".to_string()
                ),
                create_video_error429: Err(
                    "no value supplied for create_video_error429".to_string()
                ),
                create_video_error500: Err(
                    "no value supplied for create_video_error500".to_string()
                ),
                create_video_error503: Err(
                    "no value supplied for create_video_error503".to_string()
                ),
                create_video_extend_error400: Err(
                    "no value supplied for create_video_extend_error400".to_string(),
                ),
                create_video_extend_error404: Err(
                    "no value supplied for create_video_extend_error404".to_string(),
                ),
                create_video_extend_error429: Err(
                    "no value supplied for create_video_extend_error429".to_string(),
                ),
                create_video_extend_error500: Err(
                    "no value supplied for create_video_extend_error500".to_string(),
                ),
                create_video_extend_error503: Err(
                    "no value supplied for create_video_extend_error503".to_string(),
                ),
                create_video_extend_multipart_form_data_request: Err(
                    "no value supplied for create_video_extend_multipart_form_data_request"
                        .to_string(),
                ),
                create_video_extend_request: Err(
                    "no value supplied for create_video_extend_request".to_string(),
                ),
                create_video_extend_response: Err(
                    "no value supplied for create_video_extend_response".to_string(),
                ),
                create_video_multipart_form_data_request: Err(
                    "no value supplied for create_video_multipart_form_data_request".to_string(),
                ),
                create_video_remix_error400: Err(
                    "no value supplied for create_video_remix_error400".to_string(),
                ),
                create_video_remix_error404: Err(
                    "no value supplied for create_video_remix_error404".to_string(),
                ),
                create_video_remix_error429: Err(
                    "no value supplied for create_video_remix_error429".to_string(),
                ),
                create_video_remix_error500: Err(
                    "no value supplied for create_video_remix_error500".to_string(),
                ),
                create_video_remix_error503: Err(
                    "no value supplied for create_video_remix_error503".to_string(),
                ),
                create_video_remix_multipart_form_data_request: Err(
                    "no value supplied for create_video_remix_multipart_form_data_request"
                        .to_string(),
                ),
                create_video_remix_params: Err(
                    "no value supplied for create_video_remix_params".to_string()
                ),
                create_video_remix_request: Err(
                    "no value supplied for create_video_remix_request".to_string()
                ),
                create_video_remix_response: Err(
                    "no value supplied for create_video_remix_response".to_string(),
                ),
                create_video_request: Err("no value supplied for create_video_request".to_string()),
                create_video_response: Err(
                    "no value supplied for create_video_response".to_string()
                ),
                delete_video_error400: Err(
                    "no value supplied for delete_video_error400".to_string()
                ),
                delete_video_error404: Err(
                    "no value supplied for delete_video_error404".to_string()
                ),
                delete_video_error500: Err(
                    "no value supplied for delete_video_error500".to_string()
                ),
                delete_video_params: Err("no value supplied for delete_video_params".to_string()),
                delete_video_response: Err(
                    "no value supplied for delete_video_response".to_string()
                ),
                get_video_character_error400: Err(
                    "no value supplied for get_video_character_error400".to_string(),
                ),
                get_video_character_error404: Err(
                    "no value supplied for get_video_character_error404".to_string(),
                ),
                get_video_character_params: Err(
                    "no value supplied for get_video_character_params".to_string()
                ),
                get_video_character_response: Err(
                    "no value supplied for get_video_character_response".to_string(),
                ),
                get_video_error400: Err("no value supplied for get_video_error400".to_string()),
                get_video_error404: Err("no value supplied for get_video_error404".to_string()),
                get_video_params: Err("no value supplied for get_video_params".to_string()),
                get_video_response: Err("no value supplied for get_video_response".to_string()),
                list_videos_error400: Err("no value supplied for list_videos_error400".to_string()),
                list_videos_error404: Err("no value supplied for list_videos_error404".to_string()),
                list_videos_params: Err("no value supplied for list_videos_params".to_string()),
                list_videos_response: Err("no value supplied for list_videos_response".to_string()),
                retrieve_video_content_error400: Err(
                    "no value supplied for retrieve_video_content_error400".to_string(),
                ),
                retrieve_video_content_error404: Err(
                    "no value supplied for retrieve_video_content_error404".to_string(),
                ),
                retrieve_video_content_error500: Err(
                    "no value supplied for retrieve_video_content_error500".to_string(),
                ),
                retrieve_video_content_params: Err(
                    "no value supplied for retrieve_video_content_params".to_string(),
                ),
                retrieve_video_content_response: Err(
                    "no value supplied for retrieve_video_content_response".to_string(),
                ),
            }
        }
    }
    impl VideosApiSchema {
        pub fn create_video_character_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_error400: {e}")
            });
            self
        }
        pub fn create_video_character_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_error404: {e}")
            });
            self
        }
        pub fn create_video_character_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_error500: {e}")
            });
            self
        }
        pub fn create_video_character_error504<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_error504 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_error504: {e}")
            });
            self
        }
        pub fn create_video_character_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoCharacterBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_request: {e}")
            });
            self
        }
        pub fn create_video_character_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoCharacterResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_character_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_character_response: {e}")
            });
            self
        }
        pub fn create_video_edit_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_error400: {e}")
            });
            self
        }
        pub fn create_video_edit_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_error404: {e}")
            });
            self
        }
        pub fn create_video_edit_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_error429: {e}")
            });
            self
        }
        pub fn create_video_edit_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_error500: {e}")
            });
            self
        }
        pub fn create_video_edit_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_error503: {e}")
            });
            self
        }
        pub fn create_video_edit_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoEditMultipartBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_video_edit_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_video_edit_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_video_edit_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoEditJsonBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_request: {e}")
            });
            self
        }
        pub fn create_video_edit_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_edit_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_edit_response: {e}")
            });
            self
        }
        pub fn create_video_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_error400: {e}")
            });
            self
        }
        pub fn create_video_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_error404: {e}")
            });
            self
        }
        pub fn create_video_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_error429: {e}")
            });
            self
        }
        pub fn create_video_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_error500: {e}")
            });
            self
        }
        pub fn create_video_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_error503: {e}")
            });
            self
        }
        pub fn create_video_extend_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_error400: {e}")
            });
            self
        }
        pub fn create_video_extend_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_error404: {e}")
            });
            self
        }
        pub fn create_video_extend_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_error429: {e}")
            });
            self
        }
        pub fn create_video_extend_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_error500: {e}")
            });
            self
        }
        pub fn create_video_extend_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_error503: {e}")
            });
            self
        }
        pub fn create_video_extend_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoExtendMultipartBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_video_extend_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_video_extend_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_video_extend_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoExtendJsonBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_request: {e}")
            });
            self
        }
        pub fn create_video_extend_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_extend_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_extend_response: {e}")
            });
            self
        }
        pub fn create_video_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoMultipartBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_video_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_video_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_video_remix_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_error400: {e}")
            });
            self
        }
        pub fn create_video_remix_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_error404: {e}")
            });
            self
        }
        pub fn create_video_remix_error429<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_error429 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_error429: {e}")
            });
            self
        }
        pub fn create_video_remix_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_error500: {e}")
            });
            self
        }
        pub fn create_video_remix_error503<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_error503 = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_error503: {e}")
            });
            self
        }
        pub fn create_video_remix_multipart_form_data_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoRemixBody>,
            T::Error: ::std::fmt::Display,
        {
            self . create_video_remix_multipart_form_data_request = value . try_into () . map_err (| e | format ! ("error converting supplied value for create_video_remix_multipart_form_data_request: {e}")) ;
            self
        }
        pub fn create_video_remix_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoRemixParams>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_params: {e}")
            });
            self
        }
        pub fn create_video_remix_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoRemixBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_request: {e}")
            });
            self
        }
        pub fn create_video_remix_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_remix_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_remix_response: {e}")
            });
            self
        }
        pub fn create_video_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CreateVideoJsonBody>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_request: {e}")
            });
            self
        }
        pub fn create_video_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.create_video_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_video_response: {e}")
            });
            self
        }
        pub fn delete_video_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_video_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_video_error400: {e}")
            });
            self
        }
        pub fn delete_video_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_video_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_video_error404: {e}")
            });
            self
        }
        pub fn delete_video_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_video_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_video_error500: {e}")
            });
            self
        }
        pub fn delete_video_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteVideoParams>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_video_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_video_params: {e}")
            });
            self
        }
        pub fn delete_video_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeletedVideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.delete_video_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for delete_video_response: {e}")
            });
            self
        }
        pub fn get_video_character_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_character_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_character_error400: {e}")
            });
            self
        }
        pub fn get_video_character_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_character_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_character_error404: {e}")
            });
            self
        }
        pub fn get_video_character_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVideoCharacterParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_character_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_character_params: {e}")
            });
            self
        }
        pub fn get_video_character_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoCharacterResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_character_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_character_response: {e}")
            });
            self
        }
        pub fn get_video_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_error400: {e}")
            });
            self
        }
        pub fn get_video_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_error404: {e}")
            });
            self
        }
        pub fn get_video_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetVideoParams>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for get_video_params: {e}"));
            self
        }
        pub fn get_video_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoResource>,
            T::Error: ::std::fmt::Display,
        {
            self.get_video_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for get_video_response: {e}")
            });
            self
        }
        pub fn list_videos_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_videos_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_videos_error400: {e}")
            });
            self
        }
        pub fn list_videos_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.list_videos_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_videos_error404: {e}")
            });
            self
        }
        pub fn list_videos_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListVideosParams>,
            T::Error: ::std::fmt::Display,
        {
            self.list_videos_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_videos_params: {e}")
            });
            self
        }
        pub fn list_videos_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VideoListResource>,
            T::Error: ::std::fmt::Display,
        {
            self.list_videos_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for list_videos_response: {e}")
            });
            self
        }
        pub fn retrieve_video_content_error400<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_video_content_error400 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_video_content_error400: {e}")
            });
            self
        }
        pub fn retrieve_video_content_error404<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_video_content_error404 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_video_content_error404: {e}")
            });
            self
        }
        pub fn retrieve_video_content_error500<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::openai_api_types::common::ErrorResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_video_content_error500 = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_video_content_error500: {e}")
            });
            self
        }
        pub fn retrieve_video_content_params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RetrieveVideoContentParams>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_video_content_params = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_video_content_params: {e}")
            });
            self
        }
        pub fn retrieve_video_content_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.retrieve_video_content_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for retrieve_video_content_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<VideosApiSchema> for super::VideosApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VideosApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_video_character_error400: value.create_video_character_error400?,
                create_video_character_error404: value.create_video_character_error404?,
                create_video_character_error500: value.create_video_character_error500?,
                create_video_character_error504: value.create_video_character_error504?,
                create_video_character_request: value.create_video_character_request?,
                create_video_character_response: value.create_video_character_response?,
                create_video_edit_error400: value.create_video_edit_error400?,
                create_video_edit_error404: value.create_video_edit_error404?,
                create_video_edit_error429: value.create_video_edit_error429?,
                create_video_edit_error500: value.create_video_edit_error500?,
                create_video_edit_error503: value.create_video_edit_error503?,
                create_video_edit_multipart_form_data_request: value
                    .create_video_edit_multipart_form_data_request?,
                create_video_edit_request: value.create_video_edit_request?,
                create_video_edit_response: value.create_video_edit_response?,
                create_video_error400: value.create_video_error400?,
                create_video_error404: value.create_video_error404?,
                create_video_error429: value.create_video_error429?,
                create_video_error500: value.create_video_error500?,
                create_video_error503: value.create_video_error503?,
                create_video_extend_error400: value.create_video_extend_error400?,
                create_video_extend_error404: value.create_video_extend_error404?,
                create_video_extend_error429: value.create_video_extend_error429?,
                create_video_extend_error500: value.create_video_extend_error500?,
                create_video_extend_error503: value.create_video_extend_error503?,
                create_video_extend_multipart_form_data_request: value
                    .create_video_extend_multipart_form_data_request?,
                create_video_extend_request: value.create_video_extend_request?,
                create_video_extend_response: value.create_video_extend_response?,
                create_video_multipart_form_data_request: value
                    .create_video_multipart_form_data_request?,
                create_video_remix_error400: value.create_video_remix_error400?,
                create_video_remix_error404: value.create_video_remix_error404?,
                create_video_remix_error429: value.create_video_remix_error429?,
                create_video_remix_error500: value.create_video_remix_error500?,
                create_video_remix_error503: value.create_video_remix_error503?,
                create_video_remix_multipart_form_data_request: value
                    .create_video_remix_multipart_form_data_request?,
                create_video_remix_params: value.create_video_remix_params?,
                create_video_remix_request: value.create_video_remix_request?,
                create_video_remix_response: value.create_video_remix_response?,
                create_video_request: value.create_video_request?,
                create_video_response: value.create_video_response?,
                delete_video_error400: value.delete_video_error400?,
                delete_video_error404: value.delete_video_error404?,
                delete_video_error500: value.delete_video_error500?,
                delete_video_params: value.delete_video_params?,
                delete_video_response: value.delete_video_response?,
                get_video_character_error400: value.get_video_character_error400?,
                get_video_character_error404: value.get_video_character_error404?,
                get_video_character_params: value.get_video_character_params?,
                get_video_character_response: value.get_video_character_response?,
                get_video_error400: value.get_video_error400?,
                get_video_error404: value.get_video_error404?,
                get_video_params: value.get_video_params?,
                get_video_response: value.get_video_response?,
                list_videos_error400: value.list_videos_error400?,
                list_videos_error404: value.list_videos_error404?,
                list_videos_params: value.list_videos_params?,
                list_videos_response: value.list_videos_response?,
                retrieve_video_content_error400: value.retrieve_video_content_error400?,
                retrieve_video_content_error404: value.retrieve_video_content_error404?,
                retrieve_video_content_error500: value.retrieve_video_content_error500?,
                retrieve_video_content_params: value.retrieve_video_content_params?,
                retrieve_video_content_response: value.retrieve_video_content_response?,
            })
        }
    }
    impl ::std::convert::From<super::VideosApiSchema> for VideosApiSchema {
        fn from(value: super::VideosApiSchema) -> Self {
            Self {
                create_video_character_error400: Ok(value.create_video_character_error400),
                create_video_character_error404: Ok(value.create_video_character_error404),
                create_video_character_error500: Ok(value.create_video_character_error500),
                create_video_character_error504: Ok(value.create_video_character_error504),
                create_video_character_request: Ok(value.create_video_character_request),
                create_video_character_response: Ok(value.create_video_character_response),
                create_video_edit_error400: Ok(value.create_video_edit_error400),
                create_video_edit_error404: Ok(value.create_video_edit_error404),
                create_video_edit_error429: Ok(value.create_video_edit_error429),
                create_video_edit_error500: Ok(value.create_video_edit_error500),
                create_video_edit_error503: Ok(value.create_video_edit_error503),
                create_video_edit_multipart_form_data_request: Ok(
                    value.create_video_edit_multipart_form_data_request
                ),
                create_video_edit_request: Ok(value.create_video_edit_request),
                create_video_edit_response: Ok(value.create_video_edit_response),
                create_video_error400: Ok(value.create_video_error400),
                create_video_error404: Ok(value.create_video_error404),
                create_video_error429: Ok(value.create_video_error429),
                create_video_error500: Ok(value.create_video_error500),
                create_video_error503: Ok(value.create_video_error503),
                create_video_extend_error400: Ok(value.create_video_extend_error400),
                create_video_extend_error404: Ok(value.create_video_extend_error404),
                create_video_extend_error429: Ok(value.create_video_extend_error429),
                create_video_extend_error500: Ok(value.create_video_extend_error500),
                create_video_extend_error503: Ok(value.create_video_extend_error503),
                create_video_extend_multipart_form_data_request: Ok(
                    value.create_video_extend_multipart_form_data_request
                ),
                create_video_extend_request: Ok(value.create_video_extend_request),
                create_video_extend_response: Ok(value.create_video_extend_response),
                create_video_multipart_form_data_request: Ok(
                    value.create_video_multipart_form_data_request
                ),
                create_video_remix_error400: Ok(value.create_video_remix_error400),
                create_video_remix_error404: Ok(value.create_video_remix_error404),
                create_video_remix_error429: Ok(value.create_video_remix_error429),
                create_video_remix_error500: Ok(value.create_video_remix_error500),
                create_video_remix_error503: Ok(value.create_video_remix_error503),
                create_video_remix_multipart_form_data_request: Ok(
                    value.create_video_remix_multipart_form_data_request
                ),
                create_video_remix_params: Ok(value.create_video_remix_params),
                create_video_remix_request: Ok(value.create_video_remix_request),
                create_video_remix_response: Ok(value.create_video_remix_response),
                create_video_request: Ok(value.create_video_request),
                create_video_response: Ok(value.create_video_response),
                delete_video_error400: Ok(value.delete_video_error400),
                delete_video_error404: Ok(value.delete_video_error404),
                delete_video_error500: Ok(value.delete_video_error500),
                delete_video_params: Ok(value.delete_video_params),
                delete_video_response: Ok(value.delete_video_response),
                get_video_character_error400: Ok(value.get_video_character_error400),
                get_video_character_error404: Ok(value.get_video_character_error404),
                get_video_character_params: Ok(value.get_video_character_params),
                get_video_character_response: Ok(value.get_video_character_response),
                get_video_error400: Ok(value.get_video_error400),
                get_video_error404: Ok(value.get_video_error404),
                get_video_params: Ok(value.get_video_params),
                get_video_response: Ok(value.get_video_response),
                list_videos_error400: Ok(value.list_videos_error400),
                list_videos_error404: Ok(value.list_videos_error404),
                list_videos_params: Ok(value.list_videos_params),
                list_videos_response: Ok(value.list_videos_response),
                retrieve_video_content_error400: Ok(value.retrieve_video_content_error400),
                retrieve_video_content_error404: Ok(value.retrieve_video_content_error404),
                retrieve_video_content_error500: Ok(value.retrieve_video_content_error500),
                retrieve_video_content_params: Ok(value.retrieve_video_content_params),
                retrieve_video_content_response: Ok(value.retrieve_video_content_response),
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
