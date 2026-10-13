#[doc = "`CreateImageEditRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateImageEditRequest {
    #[doc = "Set the background of the generated image(s). This parameter is only supported for\nthe GPT image models. Must be one of `transparent`, `opaque`, or `auto` (default\nvalue). When `auto` is used, the model will automatically determine the best\nbackground for the image.\n\n`gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08`\nsnapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds\nare available for supported GPT Image models. For `gpt-image-2` and\n`gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`,\nset the output format to `png` or `webp`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<CreateImageEditRequestBackground>,
    #[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor the GPT image models (`gpt-image-1`, `gpt-image-1-mini`, `gpt-image-1.5`,\n`gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,\n`gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and\n`gpt-image-2.5-flare-2026-09-08`), each image should be a `png`, `webp`, or `jpg`\nfile less than 50MB. You can provide up to 16 images. `chatgpt-image-latest`\nfollows the same input constraints as GPT image models.\n"]
    pub image: CreateImageEditRequestImage,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_fidelity: ::std::option::Option<InputFidelity>,
    #[doc = "An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mask: ::std::option::Option<::std::string::String>,
    #[doc = "The GPT image model to use for image editing (`gpt-image-1`, `gpt-image-1-mini`, `gpt-image-1.5`, `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, `gpt-image-2.5-flare-2026-09-08`, or `chatgpt-image-latest`)."]
    pub model: ::std::string::String,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "The compression level (0-100%) for the generated images. This parameter\nis only supported for the GPT image models with the `webp` or `jpeg` output\nformats, and defaults to 100.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_compression: ::std::option::Option<i64>,
    #[doc = "The format in which the generated images are returned. This parameter is\nonly supported for the GPT image models. Must be one of `png`, `jpeg`, or `webp`.\nThe default value is `png`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<CreateImageEditRequestOutputFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub partial_images: ::std::option::Option<PartialImages>,
    #[doc = "A text description of the desired image(s). The maximum length is 32000 characters for the GPT image models."]
    pub prompt: ::std::string::String,
    #[doc = "The quality of the image that will be generated for GPT image models. The GPT image models support `low`, `medium`, and `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, also support `xhigh` and `max`. Defaults to `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<CreateImageEditRequestQuality>,
    #[doc = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateImageEditRequestResponseFormat>,
    #[doc = "The size of the generated images. Defaults to `auto`. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<CreateImageEditRequestSize>,
    #[doc = "Edit the image in streaming mode. Defaults to `false`. See the\n[Image generation guide](https://developers.openai.com/api/docs/guides/image-generation) for more information.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "`CreateImageEditRequestBackground`"]
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
pub enum CreateImageEditRequestBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateImageEditRequestBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateImageEditRequestBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageEditRequestBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor the GPT image models (`gpt-image-1`, `gpt-image-1-mini`, `gpt-image-1.5`,\n`gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`,\n`gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and\n`gpt-image-2.5-flare-2026-09-08`), each image should be a `png`, `webp`, or `jpg`\nfile less than 50MB. You can provide up to 16 images. `chatgpt-image-latest`\nfollows the same input constraints as GPT image models.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CreateImageEditRequestImage {
    Binary(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for CreateImageEditRequestImage {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`CreateImageEditRequestOutputFormat`"]
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
pub enum CreateImageEditRequestOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
}
impl ::std::fmt::Display for CreateImageEditRequestOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Jpeg => f.write_str("jpeg"),
            Self::Webp => f.write_str("webp"),
        }
    }
}
impl ::std::str::FromStr for CreateImageEditRequestOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageEditRequestOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateImageEditRequestQuality`"]
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
pub enum CreateImageEditRequestQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "standard")]
    Standard,
}
impl ::std::fmt::Display for CreateImageEditRequestQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
            Self::Standard => f.write_str("standard"),
        }
    }
}
impl ::std::str::FromStr for CreateImageEditRequestQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            "standard" => Ok(Self::Standard),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageEditRequestQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."]
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
pub enum CreateImageEditRequestResponseFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}
impl ::std::fmt::Display for CreateImageEditRequestResponseFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Url => f.write_str("url"),
            Self::B64Json => f.write_str("b64_json"),
        }
    }
}
impl ::std::str::FromStr for CreateImageEditRequestResponseFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url" => Ok(Self::Url),
            "b64_json" => Ok(Self::B64Json),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageEditRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The size of the generated images. Defaults to `auto`. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateImageEditRequestSize {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<CreateImageEditRequestSizeSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
#[doc = "`CreateImageEditRequestSizeSubtype1`"]
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
pub enum CreateImageEditRequestSizeSubtype1 {
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1536x1024")]
    X1536x1024,
    #[serde(rename = "1024x1536")]
    X1024x1536,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "256x256")]
    X256x256,
    #[serde(rename = "512x512")]
    X512x512,
}
impl ::std::fmt::Display for CreateImageEditRequestSizeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1536x1024 => f.write_str("1536x1024"),
            Self::X1024x1536 => f.write_str("1024x1536"),
            Self::Auto => f.write_str("auto"),
            Self::X256x256 => f.write_str("256x256"),
            Self::X512x512 => f.write_str("512x512"),
        }
    }
}
impl ::std::str::FromStr for CreateImageEditRequestSizeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1024x1024" => Ok(Self::X1024x1024),
            "1536x1024" => Ok(Self::X1536x1024),
            "1024x1536" => Ok(Self::X1024x1536),
            "auto" => Ok(Self::Auto),
            "256x256" => Ok(Self::X256x256),
            "512x512" => Ok(Self::X512x512),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageEditRequestSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageEditRequestSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateImageRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CreateImageRequest {
    #[doc = "Set the background of the generated image(s). This parameter is only supported for\nthe GPT image models. Must be one of `transparent`, `opaque`, or `auto` (default\nvalue). When `auto` is used, the model will automatically determine the best\nbackground for the image.\n\n`gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08`\nsnapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds\nare available for supported GPT Image models. For `gpt-image-2` and\n`gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`,\nset the output format to `png` or `webp`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<CreateImageRequestBackground>,
    #[doc = "The GPT image model to use for image generation. Specify a model explicitly. Supported models include `gpt-image-1`, `gpt-image-1-mini`, `gpt-image-1.5`, `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, `gpt-image-2.5-flare-2026-09-08`, and `chatgpt-image-latest`."]
    pub model: ::std::string::String,
    #[doc = "Control the content-moderation level for images generated by the GPT image models. Must be either `low` for less restrictive filtering or `auto` (default value)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<CreateImageRequestModeration>,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "The compression level (0-100%) for the generated images. This parameter is only supported for the GPT image models with the `webp` or `jpeg` output formats, and defaults to 100."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_compression: ::std::option::Option<i64>,
    #[doc = "The format in which the generated images are returned. This parameter is only supported for the GPT image models. Must be one of `png`, `jpeg`, or `webp`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<CreateImageRequestOutputFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub partial_images: ::std::option::Option<PartialImages>,
    #[doc = "A text description of the desired image(s). The maximum length is 32000 characters."]
    pub prompt: ::std::string::String,
    #[doc = "The quality of the image that will be generated.\n\n- `auto` (default value) will automatically select the best quality for the given\n  model.\n- `high`, `medium` and `low` are supported for the GPT image models.\n- `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08`\n  snapshots, also support `xhigh` and `max`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<CreateImageRequestQuality>,
    #[doc = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateImageRequestResponseFormat>,
    #[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<CreateImageRequestSize>,
    #[doc = "Generate the image in streaming mode. Defaults to `false`. See the\n[Image generation guide](https://developers.openai.com/api/docs/guides/image-generation) for more information.\nThis parameter is only supported for the GPT image models.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "Legacy style parameter for retired image models. Unsupported for GPT image models; describe the desired style in the prompt instead."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub style: ::std::option::Option<CreateImageRequestStyle>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://developers.openai.com/api/docs/guides/safety-best-practices#implement-safety-identifiers).\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "`CreateImageRequestBackground`"]
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
pub enum CreateImageRequestBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateImageRequestBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateImageRequestModeration`"]
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
pub enum CreateImageRequestModeration {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for CreateImageRequestModeration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestModeration {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestModeration {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestModeration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateImageRequestOutputFormat`"]
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
pub enum CreateImageRequestOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
}
impl ::std::fmt::Display for CreateImageRequestOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Jpeg => f.write_str("jpeg"),
            Self::Webp => f.write_str("webp"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`CreateImageRequestQuality`"]
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
pub enum CreateImageRequestQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hd")]
    Hd,
}
impl ::std::fmt::Display for CreateImageRequestQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
            Self::Standard => f.write_str("standard"),
            Self::Hd => f.write_str("hd"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            "standard" => Ok(Self::Standard),
            "hd" => Ok(Self::Hd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Legacy response format parameter for retired image models. Unsupported for GPT image models, which always return base64-encoded images."]
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
pub enum CreateImageRequestResponseFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}
impl ::std::fmt::Display for CreateImageRequestResponseFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Url => f.write_str("url"),
            Self::B64Json => f.write_str("b64_json"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestResponseFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "url" => Ok(Self::Url),
            "b64_json" => Ok(Self::B64Json),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestResponseFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct CreateImageRequestSize {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<CreateImageRequestSizeSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
#[doc = "`CreateImageRequestSizeSubtype1`"]
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
pub enum CreateImageRequestSizeSubtype1 {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1536x1024")]
    X1536x1024,
    #[serde(rename = "1024x1536")]
    X1024x1536,
    #[serde(rename = "256x256")]
    X256x256,
    #[serde(rename = "512x512")]
    X512x512,
    #[serde(rename = "1792x1024")]
    X1792x1024,
    #[serde(rename = "1024x1792")]
    X1024x1792,
}
impl ::std::fmt::Display for CreateImageRequestSizeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1536x1024 => f.write_str("1536x1024"),
            Self::X1024x1536 => f.write_str("1024x1536"),
            Self::X256x256 => f.write_str("256x256"),
            Self::X512x512 => f.write_str("512x512"),
            Self::X1792x1024 => f.write_str("1792x1024"),
            Self::X1024x1792 => f.write_str("1024x1792"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestSizeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "1024x1024" => Ok(Self::X1024x1024),
            "1536x1024" => Ok(Self::X1536x1024),
            "1024x1536" => Ok(Self::X1024x1536),
            "256x256" => Ok(Self::X256x256),
            "512x512" => Ok(Self::X512x512),
            "1792x1024" => Ok(Self::X1792x1024),
            "1024x1792" => Ok(Self::X1024x1792),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Legacy style parameter for retired image models. Unsupported for GPT image models; describe the desired style in the prompt instead."]
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
pub enum CreateImageRequestStyle {
    #[serde(rename = "vivid")]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}
impl ::std::fmt::Display for CreateImageRequestStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Vivid => f.write_str("vivid"),
            Self::Natural => f.write_str("natural"),
        }
    }
}
impl ::std::str::FromStr for CreateImageRequestStyle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "vivid" => Ok(Self::Vivid),
            "natural" => Ok(Self::Natural),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateImageRequestStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateImageRequestStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JSON request body for image edits.\n\nUse `images` (array of `ImageRefParam`) instead of multipart `image` uploads.\nYou can reference images via external URLs, data URLs, or uploaded file IDs.\nJSON edits support GPT image models.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EditImageBodyJsonParam {
    #[doc = "Set the background of the generated image output. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their `2026-09-08` snapshots, support `opaque` and `transparent` backgrounds. Transparent backgrounds are available for supported GPT Image models. For `gpt-image-2` and `gpt-image-2-2026-04-21`, this support is in preview. When using `transparent`, set the output format to `png` or `webp`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<EditImageBodyJsonParamBackground>,
    #[doc = "Input image references to edit.\nFor GPT image models, you can provide up to 16 images.\n"]
    pub images: ::std::vec::Vec<ImageRefParam>,
    #[doc = "Control how much effort the model will exert to match the style and features, especially facial features, of input images. Supports `high` and `low` on `gpt-image-1` and `gpt-image-1.5`; `gpt-image-1-mini` supports only `low`. For `gpt-image-2`, omit this parameter. Defaults to `low` on supported models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input_fidelity: ::std::option::Option<EditImageBodyJsonParamInputFidelity>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mask: ::std::option::Option<ImageRefParam>,
    #[doc = "The GPT image model to use for image editing, including `gpt-image-2`, its dated snapshot `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`. Defaults to `gpt-image-2.5-sunburst`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<EditImageBodyJsonParamModel>,
    #[doc = "Moderation level for GPT image models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub moderation: ::std::option::Option<EditImageBodyJsonParamModeration>,
    #[doc = "The number of edited images to generate."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub n: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Compression level for `jpeg` or `webp` output."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_compression: ::std::option::Option<i64>,
    #[doc = "Output image format. Supported for GPT image models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<EditImageBodyJsonParamOutputFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub partial_images: ::std::option::Option<PartialImages>,
    #[doc = "A text description of the desired image edit."]
    pub prompt: EditImageBodyJsonParamPrompt,
    #[doc = "Output quality for GPT image models. The GPT image models support `low`, `medium`,\nand `high`. `gpt-image-2.5-sunburst` and `gpt-image-2.5-flare`, including their\n`2026-09-08` snapshots, also support `xhigh` and `max`. Defaults to `auto`.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<EditImageBodyJsonParamQuality>,
    #[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<EditImageBodyJsonParamSize>,
    #[doc = "Stream partial image results as events."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stream: ::std::option::Option<bool>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI\nmonitor and detect abuse.\n"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
#[doc = "`EditImageBodyJsonParamBackground`"]
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
pub enum EditImageBodyJsonParamBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for EditImageBodyJsonParamBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EditImageBodyJsonParamInputFidelity`"]
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
pub enum EditImageBodyJsonParamInputFidelity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}
impl ::std::fmt::Display for EditImageBodyJsonParamInputFidelity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::High => f.write_str("high"),
            Self::Low => f.write_str("low"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamInputFidelity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamInputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamInputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The GPT image model to use for image editing, including `gpt-image-2`, its dated snapshot `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`. Defaults to `gpt-image-2.5-sunburst`."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct EditImageBodyJsonParamModel {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<EditImageBodyJsonParamModelSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
#[doc = "`EditImageBodyJsonParamModelSubtype1`"]
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
pub enum EditImageBodyJsonParamModelSubtype1 {
    #[serde(rename = "gpt-image-1.5")]
    GptImage15,
    #[serde(rename = "gpt-image-2")]
    GptImage2,
    #[serde(rename = "gpt-image-2-2026-04-21")]
    GptImage220260421,
    #[serde(rename = "gpt-image-2.5-sunburst")]
    GptImage25Sunburst,
    #[serde(rename = "gpt-image-2.5-sunburst-2026-09-08")]
    GptImage25Sunburst20260908,
    #[serde(rename = "gpt-image-2.5-flare")]
    GptImage25Flare,
    #[serde(rename = "gpt-image-2.5-flare-2026-09-08")]
    GptImage25Flare20260908,
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
    #[serde(rename = "chatgpt-image-latest")]
    ChatgptImageLatest,
}
impl ::std::fmt::Display for EditImageBodyJsonParamModelSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GptImage15 => f.write_str("gpt-image-1.5"),
            Self::GptImage2 => f.write_str("gpt-image-2"),
            Self::GptImage220260421 => f.write_str("gpt-image-2-2026-04-21"),
            Self::GptImage25Sunburst => f.write_str("gpt-image-2.5-sunburst"),
            Self::GptImage25Sunburst20260908 => f.write_str("gpt-image-2.5-sunburst-2026-09-08"),
            Self::GptImage25Flare => f.write_str("gpt-image-2.5-flare"),
            Self::GptImage25Flare20260908 => f.write_str("gpt-image-2.5-flare-2026-09-08"),
            Self::GptImage1 => f.write_str("gpt-image-1"),
            Self::GptImage1Mini => f.write_str("gpt-image-1-mini"),
            Self::ChatgptImageLatest => f.write_str("chatgpt-image-latest"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamModelSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gpt-image-1.5" => Ok(Self::GptImage15),
            "gpt-image-2" => Ok(Self::GptImage2),
            "gpt-image-2-2026-04-21" => Ok(Self::GptImage220260421),
            "gpt-image-2.5-sunburst" => Ok(Self::GptImage25Sunburst),
            "gpt-image-2.5-sunburst-2026-09-08" => Ok(Self::GptImage25Sunburst20260908),
            "gpt-image-2.5-flare" => Ok(Self::GptImage25Flare),
            "gpt-image-2.5-flare-2026-09-08" => Ok(Self::GptImage25Flare20260908),
            "gpt-image-1" => Ok(Self::GptImage1),
            "gpt-image-1-mini" => Ok(Self::GptImage1Mini),
            "chatgpt-image-latest" => Ok(Self::ChatgptImageLatest),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamModelSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EditImageBodyJsonParamModeration`"]
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
pub enum EditImageBodyJsonParamModeration {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for EditImageBodyJsonParamModeration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamModeration {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamModeration {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamModeration {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EditImageBodyJsonParamOutputFormat`"]
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
pub enum EditImageBodyJsonParamOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
}
impl ::std::fmt::Display for EditImageBodyJsonParamOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Jpeg => f.write_str("jpeg"),
            Self::Webp => f.write_str("webp"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A text description of the desired image edit."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EditImageBodyJsonParamPrompt(::std::string::String);
impl ::std::ops::Deref for EditImageBodyJsonParamPrompt {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EditImageBodyJsonParamPrompt> for ::std::string::String {
    fn from(value: EditImageBodyJsonParamPrompt) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamPrompt {
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
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamPrompt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamPrompt {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EditImageBodyJsonParamPrompt {
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
#[doc = "`EditImageBodyJsonParamQuality`"]
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
pub enum EditImageBodyJsonParamQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for EditImageBodyJsonParamQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The size of the generated images. For `gpt-image-2`, `gpt-image-2-2026-04-21`, `gpt-image-2.5-sunburst`, `gpt-image-2.5-sunburst-2026-09-08`, `gpt-image-2.5-flare`, and `gpt-image-2.5-flare-2026-09-08`, arbitrary resolutions are supported as `WIDTHxHEIGHT` strings, for example `1536x864`. Width and height must both be divisible by 16 and the requested aspect ratio must be between 1:3 and 3:1. Resolutions above `2560x1440` are experimental, and the maximum supported resolution is `3840x2160`. The requested size must also satisfy the model's current pixel and edge limits. The standard sizes `1024x1024`, `1536x1024`, and `1024x1536` are supported by the GPT image models; `auto` is supported for models that allow automatic sizing."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct EditImageBodyJsonParamSize {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<EditImageBodyJsonParamSizeSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<()>,
}
#[doc = "`EditImageBodyJsonParamSizeSubtype1`"]
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
pub enum EditImageBodyJsonParamSizeSubtype1 {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    X1024x1024,
    #[serde(rename = "1536x1024")]
    X1536x1024,
    #[serde(rename = "1024x1536")]
    X1024x1536,
}
impl ::std::fmt::Display for EditImageBodyJsonParamSizeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::X1024x1024 => f.write_str("1024x1024"),
            Self::X1536x1024 => f.write_str("1536x1024"),
            Self::X1024x1536 => f.write_str("1024x1536"),
        }
    }
}
impl ::std::str::FromStr for EditImageBodyJsonParamSizeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "1024x1024" => Ok(Self::X1024x1024),
            "1536x1024" => Ok(Self::X1536x1024),
            "1024x1536" => Ok(Self::X1024x1536),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EditImageBodyJsonParamSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EditImageBodyJsonParamSizeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents the content or the URL of an image generated by the OpenAI API."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
pub struct Image {
    #[doc = "The base64-encoded JSON of the generated image. Returned by default for GPT image models, or when `response_format` is set to `b64_json` for models that support that parameter."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub b64_json: ::std::option::Option<::std::string::String>,
    #[doc = "The revised prompt used to generate the image, for models that support prompt revision. Not returned by GPT image models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub revised_prompt: ::std::option::Option<::std::string::String>,
    #[doc = "The URL of the generated image when `response_format` is set to `url` for models that support that parameter. Unsupported for GPT image models."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
#[doc = "Emitted when image editing has completed and the final image is available.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageEditCompletedEvent {
    #[doc = "Base64-encoded final edited image data, suitable for rendering as an image.\n"]
    pub b64_json: ::std::string::String,
    #[doc = "The background setting for the edited image.\n"]
    pub background: ImageEditCompletedEventBackground,
    #[doc = "The Unix timestamp when the event was created.\n"]
    pub created_at: i64,
    #[doc = "The output format for the edited image.\n"]
    pub output_format: ImageEditCompletedEventOutputFormat,
    #[doc = "The quality setting for the edited image.\n"]
    pub quality: ImageEditCompletedEventQuality,
    #[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
    pub size: ::std::string::String,
    #[doc = "The type of the event. Always `image_edit.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: ImageEditCompletedEventType,
    pub usage: ImagesUsage,
}
#[doc = "The background setting for the edited image.\n"]
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
pub enum ImageEditCompletedEventBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageEditCompletedEventBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageEditCompletedEventBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditCompletedEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditCompletedEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format for the edited image.\n"]
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
pub enum ImageEditCompletedEventOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageEditCompletedEventOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageEditCompletedEventOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditCompletedEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditCompletedEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality setting for the edited image.\n"]
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
pub enum ImageEditCompletedEventQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageEditCompletedEventQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageEditCompletedEventQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditCompletedEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditCompletedEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `image_edit.completed`.\n"]
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
pub enum ImageEditCompletedEventType {
    #[serde(rename = "image_edit.completed")]
    ImageEditCompleted,
}
impl ::std::fmt::Display for ImageEditCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageEditCompleted => f.write_str("image_edit.completed"),
        }
    }
}
impl ::std::str::FromStr for ImageEditCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_edit.completed" => Ok(Self::ImageEditCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Emitted when a partial image is available during image editing streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageEditPartialImageEvent {
    #[doc = "Base64-encoded partial image data, suitable for rendering as an image.\n"]
    pub b64_json: ::std::string::String,
    #[doc = "The background setting for the requested edited image.\n"]
    pub background: ImageEditPartialImageEventBackground,
    #[doc = "The Unix timestamp when the event was created.\n"]
    pub created_at: i64,
    #[doc = "The output format for the requested edited image.\n"]
    pub output_format: ImageEditPartialImageEventOutputFormat,
    #[doc = "0-based index for the partial image (streaming).\n"]
    pub partial_image_index: i64,
    #[doc = "The quality setting for the requested edited image.\n"]
    pub quality: ImageEditPartialImageEventQuality,
    #[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
    pub size: ::std::string::String,
    #[doc = "The type of the event. Always `image_edit.partial_image`.\n"]
    #[serde(rename = "type")]
    pub type_: ImageEditPartialImageEventType,
}
#[doc = "The background setting for the requested edited image.\n"]
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
pub enum ImageEditPartialImageEventBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageEditPartialImageEventBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageEditPartialImageEventBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditPartialImageEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditPartialImageEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format for the requested edited image.\n"]
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
pub enum ImageEditPartialImageEventOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageEditPartialImageEventOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageEditPartialImageEventOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditPartialImageEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditPartialImageEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality setting for the requested edited image.\n"]
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
pub enum ImageEditPartialImageEventQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageEditPartialImageEventQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageEditPartialImageEventQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditPartialImageEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditPartialImageEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `image_edit.partial_image`.\n"]
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
pub enum ImageEditPartialImageEventType {
    #[serde(rename = "image_edit.partial_image")]
    ImageEditPartialImage,
}
impl ::std::fmt::Display for ImageEditPartialImageEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageEditPartialImage => f.write_str("image_edit.partial_image"),
        }
    }
}
impl ::std::str::FromStr for ImageEditPartialImageEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_edit.partial_image" => Ok(Self::ImageEditPartialImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageEditPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageEditPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageEditStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ImageEditStreamEvent {
    PartialImageEvent(ImageEditPartialImageEvent),
    CompletedEvent(ImageEditCompletedEvent),
}
impl ::std::convert::From<ImageEditPartialImageEvent> for ImageEditStreamEvent {
    fn from(value: ImageEditPartialImageEvent) -> Self {
        Self::PartialImageEvent(value)
    }
}
impl ::std::convert::From<ImageEditCompletedEvent> for ImageEditStreamEvent {
    fn from(value: ImageEditCompletedEvent) -> Self {
        Self::CompletedEvent(value)
    }
}
#[doc = "Emitted when image generation has completed and the final image is available.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenCompletedEvent {
    #[doc = "Base64-encoded image data, suitable for rendering as an image.\n"]
    pub b64_json: ::std::string::String,
    #[doc = "The background setting for the generated image.\n"]
    pub background: ImageGenCompletedEventBackground,
    #[doc = "The Unix timestamp when the event was created.\n"]
    pub created_at: i64,
    #[doc = "The output format for the generated image.\n"]
    pub output_format: ImageGenCompletedEventOutputFormat,
    #[doc = "The quality setting for the generated image.\n"]
    pub quality: ImageGenCompletedEventQuality,
    #[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
    pub size: ::std::string::String,
    #[doc = "The type of the event. Always `image_generation.completed`.\n"]
    #[serde(rename = "type")]
    pub type_: ImageGenCompletedEventType,
    pub usage: ImagesUsage,
}
#[doc = "The background setting for the generated image.\n"]
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
pub enum ImageGenCompletedEventBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenCompletedEventBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenCompletedEventBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenCompletedEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenCompletedEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format for the generated image.\n"]
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
pub enum ImageGenCompletedEventOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageGenCompletedEventOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageGenCompletedEventOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenCompletedEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenCompletedEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality setting for the generated image.\n"]
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
pub enum ImageGenCompletedEventQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenCompletedEventQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenCompletedEventQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenCompletedEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenCompletedEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `image_generation.completed`.\n"]
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
pub enum ImageGenCompletedEventType {
    #[serde(rename = "image_generation.completed")]
    ImageGenerationCompleted,
}
impl ::std::fmt::Display for ImageGenCompletedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGenerationCompleted => f.write_str("image_generation.completed"),
        }
    }
}
impl ::std::str::FromStr for ImageGenCompletedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation.completed" => Ok(Self::ImageGenerationCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenCompletedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The input tokens detailed information for the image generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenInputUsageDetails {
    #[doc = "The number of image tokens in the input prompt."]
    pub image_tokens: i64,
    #[doc = "The number of text tokens in the input prompt."]
    pub text_tokens: i64,
}
#[doc = "The output token details for the image generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenOutputTokensDetails {
    #[doc = "The number of image output tokens generated by the model."]
    pub image_tokens: i64,
    #[doc = "The number of text output tokens generated by the model."]
    pub text_tokens: i64,
}
#[doc = "Emitted when a partial image is available during image generation streaming.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenPartialImageEvent {
    #[doc = "Base64-encoded partial image data, suitable for rendering as an image.\n"]
    pub b64_json: ::std::string::String,
    #[doc = "The background setting for the requested image.\n"]
    pub background: ImageGenPartialImageEventBackground,
    #[doc = "The Unix timestamp when the event was created.\n"]
    pub created_at: i64,
    #[doc = "The output format for the requested image.\n"]
    pub output_format: ImageGenPartialImageEventOutputFormat,
    #[doc = "0-based index for the partial image (streaming).\n"]
    pub partial_image_index: i64,
    #[doc = "The quality setting for the requested image.\n"]
    pub quality: ImageGenPartialImageEventQuality,
    #[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
    pub size: ::std::string::String,
    #[doc = "The type of the event. Always `image_generation.partial_image`.\n"]
    #[serde(rename = "type")]
    pub type_: ImageGenPartialImageEventType,
}
#[doc = "The background setting for the requested image.\n"]
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
pub enum ImageGenPartialImageEventBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenPartialImageEventBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenPartialImageEventBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenPartialImageEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenPartialImageEventBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format for the requested image.\n"]
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
pub enum ImageGenPartialImageEventOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImageGenPartialImageEventOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImageGenPartialImageEventOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenPartialImageEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenPartialImageEventOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality setting for the requested image.\n"]
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
pub enum ImageGenPartialImageEventQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ImageGenPartialImageEventQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ImageGenPartialImageEventQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenPartialImageEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenPartialImageEventQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type of the event. Always `image_generation.partial_image`.\n"]
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
pub enum ImageGenPartialImageEventType {
    #[serde(rename = "image_generation.partial_image")]
    ImageGenerationPartialImage,
}
impl ::std::fmt::Display for ImageGenPartialImageEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageGenerationPartialImage => f.write_str("image_generation.partial_image"),
        }
    }
}
impl ::std::str::FromStr for ImageGenPartialImageEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_generation.partial_image" => Ok(Self::ImageGenerationPartialImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageGenPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageGenPartialImageEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ImageGenStreamEvent`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ImageGenStreamEvent {
    PartialImageEvent(ImageGenPartialImageEvent),
    CompletedEvent(ImageGenCompletedEvent),
}
impl ::std::convert::From<ImageGenPartialImageEvent> for ImageGenStreamEvent {
    fn from(value: ImageGenPartialImageEvent) -> Self {
        Self::PartialImageEvent(value)
    }
}
impl ::std::convert::From<ImageGenCompletedEvent> for ImageGenStreamEvent {
    fn from(value: ImageGenCompletedEvent) -> Self {
        Self::CompletedEvent(value)
    }
}
#[doc = "For `gpt-image-1` only, the token usage information for the image generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImageGenUsage {
    #[doc = "The number of tokens (images and text) in the input prompt."]
    pub input_tokens: i64,
    pub input_tokens_details: ImageGenInputUsageDetails,
    #[doc = "The number of output tokens generated by the model."]
    pub output_tokens: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_tokens_details: ::std::option::Option<ImageGenOutputTokensDetails>,
    #[doc = "The total number of tokens (images and text) used for the image generation."]
    pub total_tokens: i64,
}
#[doc = "Reference an input image by either URL or uploaded file ID.\nProvide exactly one of `image_url` or `file_id`.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImageRefParam {
    #[doc = "The File API ID of an uploaded image to use as input."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[doc = "A fully qualified URL or base64-encoded data URL."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_url: ::std::option::Option<ImageRefParamImageUrl>,
}
#[doc = "A fully qualified URL or base64-encoded data URL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ImageRefParamImageUrl(::std::string::String);
impl ::std::ops::Deref for ImageRefParamImageUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ImageRefParamImageUrl> for ::std::string::String {
    fn from(value: ImageRefParamImageUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ImageRefParamImageUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 20971520usize {
            return Err("longer than 20971520 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ImageRefParamImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageRefParamImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ImageRefParamImageUrl {
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
#[doc = "`ImagesApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImagesApiSchema {
    #[serde(rename = "CreateImageEditMultipartFormDataRequest")]
    pub create_image_edit_multipart_form_data_request: CreateImageEditRequest,
    #[serde(rename = "CreateImageEditRequest")]
    pub create_image_edit_request: EditImageBodyJsonParam,
    #[serde(rename = "CreateImageEditResponse")]
    pub create_image_edit_response: ImagesResponse,
    #[serde(rename = "CreateImageEditStreamResponse")]
    pub create_image_edit_stream_response: ImageEditStreamEvent,
    #[serde(rename = "CreateImageRequest")]
    pub create_image_request: CreateImageRequest,
    #[serde(rename = "CreateImageResponse")]
    pub create_image_response: ImagesResponse,
    #[serde(rename = "CreateImageStreamResponse")]
    pub create_image_stream_response: ImageGenStreamEvent,
}
#[doc = "The response from the image generation endpoint."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImagesResponse {
    #[doc = "The background parameter used for the image generation. Either `transparent` or `opaque`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<ImagesResponseBackground>,
    #[doc = "The Unix timestamp (in seconds) of when the image was created."]
    pub created: i64,
    #[doc = "The list of generated images."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub data: ::std::vec::Vec<Image>,
    #[doc = "The output format of the image generation. Either `png`, `webp`, or `jpeg`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output_format: ::std::option::Option<ImagesResponseOutputFormat>,
    #[doc = "The quality of the image generated. One of `low`, `medium`, `high`, `xhigh`, or `max`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<ImagesResponseQuality>,
    #[doc = "The image dimensions as a `WIDTHxHEIGHT` string, for example `1536x864`."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<ImageGenUsage>,
}
#[doc = "The background parameter used for the image generation. Either `transparent` or `opaque`."]
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
pub enum ImagesResponseBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
}
impl ::std::fmt::Display for ImagesResponseBackground {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transparent => f.write_str("transparent"),
            Self::Opaque => f.write_str("opaque"),
        }
    }
}
impl ::std::str::FromStr for ImagesResponseBackground {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImagesResponseBackground {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImagesResponseBackground {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The output format of the image generation. Either `png`, `webp`, or `jpeg`."]
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
pub enum ImagesResponseOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
}
impl ::std::fmt::Display for ImagesResponseOutputFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Png => f.write_str("png"),
            Self::Webp => f.write_str("webp"),
            Self::Jpeg => f.write_str("jpeg"),
        }
    }
}
impl ::std::str::FromStr for ImagesResponseOutputFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::Webp),
            "jpeg" => Ok(Self::Jpeg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImagesResponseOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImagesResponseOutputFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The quality of the image generated. One of `low`, `medium`, `high`, `xhigh`, or `max`."]
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
pub enum ImagesResponseQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    Xhigh,
    #[serde(rename = "max")]
    Max,
}
impl ::std::fmt::Display for ImagesResponseQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Xhigh => f.write_str("xhigh"),
            Self::Max => f.write_str("max"),
        }
    }
}
impl ::std::str::FromStr for ImagesResponseQuality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImagesResponseQuality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImagesResponseQuality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "For the GPT image models only, the token usage information for the image generation.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImagesUsage {
    #[doc = "The number of tokens (images and text) in the input prompt."]
    pub input_tokens: i64,
    pub input_tokens_details: ImagesUsageInputTokensDetails,
    #[doc = "The number of image tokens in the output image."]
    pub output_tokens: i64,
    #[doc = "The total number of tokens (images and text) used for the image generation.\n"]
    pub total_tokens: i64,
}
#[doc = "The input tokens detailed information for the image generation."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ImagesUsageInputTokensDetails {
    #[doc = "The number of image tokens in the input prompt."]
    pub image_tokens: i64,
    #[doc = "The number of text tokens in the input prompt."]
    pub text_tokens: i64,
}
#[doc = "Control how much effort the model will exert to match the style and features, especially facial features, of input images. Supports `high` and `low` on `gpt-image-1` and `gpt-image-1.5`; `gpt-image-1-mini` supports only `low`. For `gpt-image-2`, omit this parameter. Defaults to `low` on supported models."]
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
pub enum InputFidelity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}
impl ::std::fmt::Display for InputFidelity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::High => f.write_str("high"),
            Self::Low => f.write_str("low"),
        }
    }
}
impl ::std::str::FromStr for InputFidelity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for InputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InputFidelity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`PartialImages`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PartialImages(pub ::std::option::Option<PartialImagesValue>);
impl ::std::ops::Deref for PartialImages {
    type Target = ::std::option::Option<PartialImagesValue>;
    fn deref(&self) -> &::std::option::Option<PartialImagesValue> {
        &self.0
    }
}
impl ::std::convert::From<PartialImages> for ::std::option::Option<PartialImagesValue> {
    fn from(value: PartialImages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PartialImagesValue>> for PartialImages {
    fn from(value: ::std::option::Option<PartialImagesValue>) -> Self {
        Self(value)
    }
}
#[doc = "The number of partial images to generate. This parameter is used for\nstreaming responses that return partial images. Value must be between 0 and 3.\nWhen set to 0, the response will be a single image sent in one streaming event.\n\nNote that the final image may be sent before the full number of partial images\nare generated if the full image is generated more quickly.\n"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PartialImagesValue(pub i64);
impl ::std::ops::Deref for PartialImagesValue {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PartialImagesValue> for i64 {
    fn from(value: PartialImagesValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for PartialImagesValue {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for PartialImagesValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for PartialImagesValue {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PartialImagesValue {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PartialImagesValue {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
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
