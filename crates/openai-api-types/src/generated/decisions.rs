#[doc = "A completed question always includes its name, including null when unnamed."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnswerResource {
    Predicate(AnswerResourcePredicate),
    Choice(AnswerResourceChoice),
    Score(AnswerResourceScore),
    Refusal(AnswerResourceRefusal),
}
impl ::std::convert::From<AnswerResourcePredicate> for AnswerResource {
    fn from(value: AnswerResourcePredicate) -> Self {
        Self::Predicate(value)
    }
}
impl ::std::convert::From<AnswerResourceChoice> for AnswerResource {
    fn from(value: AnswerResourceChoice) -> Self {
        Self::Choice(value)
    }
}
impl ::std::convert::From<AnswerResourceScore> for AnswerResource {
    fn from(value: AnswerResourceScore) -> Self {
        Self::Score(value)
    }
}
impl ::std::convert::From<AnswerResourceRefusal> for AnswerResource {
    fn from(value: AnswerResourceRefusal) -> Self {
        Self::Refusal(value)
    }
}
#[doc = "`AnswerResourceChoice`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AnswerResourceChoice {
    pub choice: ChoiceValueResource,
    pub confidence: f64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    pub probabilities: ::std::vec::Vec<ChoiceProbabilityResource>,
    #[doc = "The type of the object. Always `choice`."]
    #[serde(rename = "type")]
    pub type_: AnswerResourceChoiceType,
}
impl AnswerResourceChoice {
    pub fn builder() -> builder::AnswerResourceChoice {
        Default::default()
    }
}
#[doc = "The type of the object. Always `choice`."]
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
pub enum AnswerResourceChoiceType {
    #[serde(rename = "choice")]
    Choice,
}
impl ::std::fmt::Display for AnswerResourceChoiceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Choice => f.write_str("choice"),
        }
    }
}
impl ::std::str::FromStr for AnswerResourceChoiceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "choice" => Ok(Self::Choice),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnswerResourceChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnswerResourceChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AnswerResourcePredicate`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AnswerResourcePredicate {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    pub probability: f64,
    #[doc = "The type of the object. Always `predicate`."]
    #[serde(rename = "type")]
    pub type_: AnswerResourcePredicateType,
}
impl AnswerResourcePredicate {
    pub fn builder() -> builder::AnswerResourcePredicate {
        Default::default()
    }
}
#[doc = "The type of the object. Always `predicate`."]
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
pub enum AnswerResourcePredicateType {
    #[serde(rename = "predicate")]
    Predicate,
}
impl ::std::fmt::Display for AnswerResourcePredicateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Predicate => f.write_str("predicate"),
        }
    }
}
impl ::std::str::FromStr for AnswerResourcePredicateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "predicate" => Ok(Self::Predicate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnswerResourcePredicateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnswerResourcePredicateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The model declined to answer this question. Other questions in the same request can still receive answers."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AnswerResourceRefusal {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The type of the object. Always `refusal`."]
    #[serde(rename = "type")]
    pub type_: AnswerResourceRefusalType,
}
impl AnswerResourceRefusal {
    pub fn builder() -> builder::AnswerResourceRefusal {
        Default::default()
    }
}
#[doc = "The type of the object. Always `refusal`."]
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
pub enum AnswerResourceRefusalType {
    #[serde(rename = "refusal")]
    Refusal,
}
impl ::std::fmt::Display for AnswerResourceRefusalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Refusal => f.write_str("refusal"),
        }
    }
}
impl ::std::str::FromStr for AnswerResourceRefusalType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "refusal" => Ok(Self::Refusal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnswerResourceRefusalType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnswerResourceRefusalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AnswerResourceScore`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AnswerResourceScore {
    pub confidence: f64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub name: ::std::option::Option<::std::string::String>,
    pub probabilities: ::std::vec::Vec<ScoreProbabilityResource>,
    pub score: f64,
    #[doc = "The type of the object. Always `score`."]
    #[serde(rename = "type")]
    pub type_: AnswerResourceScoreType,
}
impl AnswerResourceScore {
    pub fn builder() -> builder::AnswerResourceScore {
        Default::default()
    }
}
#[doc = "The type of the object. Always `score`."]
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
pub enum AnswerResourceScoreType {
    #[serde(rename = "score")]
    Score,
}
impl ::std::fmt::Display for AnswerResourceScoreType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => f.write_str("score"),
        }
    }
}
impl ::std::str::FromStr for AnswerResourceScoreType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score" => Ok(Self::Score),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnswerResourceScoreType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnswerResourceScoreType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ChoiceOptionParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ChoiceOptionParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<ChoiceOptionParamDescription>,
    pub value: ChoiceValueParam,
}
impl ChoiceOptionParam {
    pub fn builder() -> builder::ChoiceOptionParam {
        Default::default()
    }
}
#[doc = "`ChoiceOptionParamDescription`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChoiceOptionParamDescription(::std::string::String);
impl ::std::ops::Deref for ChoiceOptionParamDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ChoiceOptionParamDescription> for ::std::string::String {
    fn from(value: ChoiceOptionParamDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ChoiceOptionParamDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ChoiceOptionParamDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChoiceOptionParamDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ChoiceOptionParamDescription {
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
#[doc = "`ChoiceProbabilityResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ChoiceProbabilityResource {
    pub probability: f64,
    pub value: ChoiceValueResource,
}
impl ChoiceProbabilityResource {
    pub fn builder() -> builder::ChoiceProbabilityResource {
        Default::default()
    }
}
#[doc = "Choice values are typed: a string and a boolean with the same text are distinct."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChoiceValueParam {
    String(ChoiceValueParamString),
    Boolean(bool),
}
impl ::std::fmt::Display for ChoiceValueParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for ChoiceValueParam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChoiceValueParam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChoiceValueParam {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<ChoiceValueParamString> for ChoiceValueParam {
    fn from(value: ChoiceValueParamString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<bool> for ChoiceValueParam {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`ChoiceValueParamString`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ChoiceValueParamString(::std::string::String);
impl ::std::ops::Deref for ChoiceValueParamString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ChoiceValueParamString> for ::std::string::String {
    fn from(value: ChoiceValueParamString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ChoiceValueParamString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ChoiceValueParamString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChoiceValueParamString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ChoiceValueParamString {
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
#[doc = "Choice values are typed: a string and a boolean with the same text are distinct."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ChoiceValueResource {
    String(::std::string::String),
    Boolean(bool),
}
impl ::std::fmt::Display for ChoiceValueResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ChoiceValueResource {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "The text or images to evaluate for every question. Provide a text string or user messages containing text and images. Images can be base64 data URLs or publicly accessible HTTP(S) URLs; at most 128 images are allowed across all messages in one request. Files, audio, tools, and item references are not supported."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DecisionInput {
    String(DecisionInputString),
    Array(::std::vec::Vec<DecisionInputMessage>),
}
impl ::std::convert::From<DecisionInputString> for DecisionInput {
    fn from(value: DecisionInputString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<DecisionInputMessage>> for DecisionInput {
    fn from(value: ::std::vec::Vec<DecisionInputMessage>) -> Self {
        Self::Array(value)
    }
}
#[doc = "Text evidence or an ordered list of text and image parts."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputContent {
    String(DecisionInputContentString),
    DecisionInputContentParts(DecisionInputContentParts),
}
impl ::std::convert::From<DecisionInputContentString> for DecisionInputContent {
    fn from(value: DecisionInputContentString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<DecisionInputContentParts> for DecisionInputContent {
    fn from(value: DecisionInputContentParts) -> Self {
        Self::DecisionInputContentParts(value)
    }
}
#[doc = "`DecisionInputContentParts`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct DecisionInputContentParts(pub ::std::vec::Vec<DecisionInputContentPartsItem>);
impl ::std::ops::Deref for DecisionInputContentParts {
    type Target = ::std::vec::Vec<DecisionInputContentPartsItem>;
    fn deref(&self) -> &::std::vec::Vec<DecisionInputContentPartsItem> {
        &self.0
    }
}
impl ::std::convert::From<DecisionInputContentParts>
    for ::std::vec::Vec<DecisionInputContentPartsItem>
{
    fn from(value: DecisionInputContentParts) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<DecisionInputContentPartsItem>>
    for DecisionInputContentParts
{
    fn from(value: ::std::vec::Vec<DecisionInputContentPartsItem>) -> Self {
        Self(value)
    }
}
#[doc = "`DecisionInputContentPartsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DecisionInputContentPartsItem {
    Text(DecisionInputText),
    Image(DecisionInputImage),
}
impl ::std::convert::From<DecisionInputText> for DecisionInputContentPartsItem {
    fn from(value: DecisionInputText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<DecisionInputImage> for DecisionInputContentPartsItem {
    fn from(value: DecisionInputImage) -> Self {
        Self::Image(value)
    }
}
#[doc = "`DecisionInputContentString`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionInputContentString(::std::string::String);
impl ::std::ops::Deref for DecisionInputContentString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionInputContentString> for ::std::string::String {
    fn from(value: DecisionInputContentString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionInputContentString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputContentString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputContentString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionInputContentString {
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
#[doc = "An image provided as a base64 data URL or a publicly accessible HTTP(S) URL. File IDs are not supported."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionInputImage {
    #[doc = "The image detail level, using the selected model's image profile. Defaults to auto."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<ImageDetailParam>,
    #[doc = "A base64-encoded image in a data URL or a publicly accessible HTTP(S) image URL."]
    pub image_url: DecisionInputImageImageUrl,
    #[serde(rename = "type")]
    pub type_: DecisionInputImageType,
}
impl DecisionInputImage {
    pub fn builder() -> builder::DecisionInputImage {
        Default::default()
    }
}
#[doc = "A base64-encoded image in a data URL or a publicly accessible HTTP(S) image URL."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionInputImageImageUrl(::std::string::String);
impl ::std::ops::Deref for DecisionInputImageImageUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionInputImageImageUrl> for ::std::string::String {
    fn from(value: DecisionInputImageImageUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionInputImageImageUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1073741824usize {
            return Err("longer than 1073741824 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^(data:|https?://)").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(data:|https?://)\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputImageImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputImageImageUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionInputImageImageUrl {
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
#[doc = "`DecisionInputImageType`"]
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
pub enum DecisionInputImageType {
    #[serde(rename = "input_image")]
    InputImage,
}
impl ::std::fmt::Display for DecisionInputImageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputImage => f.write_str("input_image"),
        }
    }
}
impl ::std::str::FromStr for DecisionInputImageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_image" => Ok(Self::InputImage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputImageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputImageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A user message containing text or images."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionInputMessage {
    pub content: DecisionInputContent,
    pub role: DecisionInputMessageRole,
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<DecisionInputMessageType>,
}
impl DecisionInputMessage {
    pub fn builder() -> builder::DecisionInputMessage {
        Default::default()
    }
}
#[doc = "`DecisionInputMessageRole`"]
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
pub enum DecisionInputMessageRole {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for DecisionInputMessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for DecisionInputMessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputMessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionInputMessageType`"]
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
pub enum DecisionInputMessageType {
    #[serde(rename = "message")]
    Message,
}
impl ::std::fmt::Display for DecisionInputMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Message => f.write_str("message"),
        }
    }
}
impl ::std::str::FromStr for DecisionInputMessageType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message" => Ok(Self::Message),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionInputString`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionInputString(::std::string::String);
impl ::std::ops::Deref for DecisionInputString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionInputString> for ::std::string::String {
    fn from(value: DecisionInputString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionInputString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionInputString {
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
#[doc = "`DecisionInputText`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionInputText {
    pub text: DecisionInputTextText,
    #[serde(rename = "type")]
    pub type_: DecisionInputTextType,
}
impl DecisionInputText {
    pub fn builder() -> builder::DecisionInputText {
        Default::default()
    }
}
#[doc = "`DecisionInputTextText`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionInputTextText(::std::string::String);
impl ::std::ops::Deref for DecisionInputTextText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionInputTextText> for ::std::string::String {
    fn from(value: DecisionInputTextText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionInputTextText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10485760usize {
            return Err("longer than 10485760 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputTextText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputTextText {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionInputTextText {
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
#[doc = "`DecisionInputTextType`"]
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
pub enum DecisionInputTextType {
    #[serde(rename = "input_text")]
    InputText,
}
impl ::std::fmt::Display for DecisionInputTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InputText => f.write_str("input_text"),
        }
    }
}
impl ::std::str::FromStr for DecisionInputTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "input_text" => Ok(Self::InputText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DecisionInputTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionInputTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`DecisionRequest`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionRequest {
    pub input: DecisionInput,
    pub model: DecisionRequestModel,
    pub questions: ::std::vec::Vec<QuestionParam>,
    #[doc = "Opaque caller-provided end-user identifier, scoped by the verified org. Match Responses' limit; this is never the authenticated user identity."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub safety_identifier: ::std::option::Option<DecisionRequestSafetyIdentifier>,
}
impl DecisionRequest {
    pub fn builder() -> builder::DecisionRequest {
        Default::default()
    }
}
#[doc = "`DecisionRequestModel`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionRequestModel(::std::string::String);
impl ::std::ops::Deref for DecisionRequestModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionRequestModel> for ::std::string::String {
    fn from(value: DecisionRequestModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionRequestModel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionRequestModel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionRequestModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionRequestModel {
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
#[doc = "Opaque caller-provided end-user identifier, scoped by the verified org. Match Responses' limit; this is never the authenticated user identity."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionRequestSafetyIdentifier(::std::string::String);
impl ::std::ops::Deref for DecisionRequestSafetyIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionRequestSafetyIdentifier> for ::std::string::String {
    fn from(value: DecisionRequestSafetyIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DecisionRequestSafetyIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DecisionRequestSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionRequestSafetyIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionRequestSafetyIdentifier {
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
#[doc = "`DecisionResponse`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionResponse {
    pub answers: ::std::vec::Vec<AnswerResource>,
    pub model: ::std::string::String,
    pub usage: ResponseUsageResource,
}
impl DecisionResponse {
    pub fn builder() -> builder::DecisionResponse {
        Default::default()
    }
}
#[doc = "`DecisionsApiSchema`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct DecisionsApiSchema {
    #[serde(rename = "CreateDecisionRequest")]
    pub create_decision_request: DecisionRequest,
    #[serde(rename = "CreateDecisionResponse")]
    pub create_decision_response: DecisionResponse,
}
impl DecisionsApiSchema {
    pub fn builder() -> builder::DecisionsApiSchema {
        Default::default()
    }
}
#[doc = "`ImageDetailParam`"]
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
pub enum ImageDetailParam {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "original")]
    Original,
}
impl ::std::fmt::Display for ImageDetailParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
            Self::Auto => f.write_str("auto"),
            Self::Original => f.write_str("original"),
        }
    }
}
impl ::std::str::FromStr for ImageDetailParam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            "auto" => Ok(Self::Auto),
            "original" => Ok(Self::Original),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageDetailParam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageDetailParam {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`InputTokensDetailsResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InputTokensDetailsResource {
    pub cache_write_tokens: i64,
    pub cached_tokens: i64,
}
impl InputTokensDetailsResource {
    pub fn builder() -> builder::InputTokensDetailsResource {
        Default::default()
    }
}
#[doc = "`OutputTokensDetailsResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OutputTokensDetailsResource {
    pub reasoning_tokens: i64,
}
impl OutputTokensDetailsResource {
    pub fn builder() -> builder::OutputTokensDetailsResource {
        Default::default()
    }
}
#[doc = "A question about the request's input, with an optional correlation name."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum QuestionParam {
    Predicate(QuestionParamPredicate),
    Choice(QuestionParamChoice),
    Score(QuestionParamScore),
}
impl ::std::convert::From<QuestionParamPredicate> for QuestionParam {
    fn from(value: QuestionParamPredicate) -> Self {
        Self::Predicate(value)
    }
}
impl ::std::convert::From<QuestionParamChoice> for QuestionParam {
    fn from(value: QuestionParamChoice) -> Self {
        Self::Choice(value)
    }
}
impl ::std::convert::From<QuestionParamScore> for QuestionParam {
    fn from(value: QuestionParamScore) -> Self {
        Self::Score(value)
    }
}
#[doc = "Choose from the supplied options based on the input."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct QuestionParamChoice {
    #[doc = "Provide between 2 and 255 choices. Each choice must be unique."]
    pub choices: ::std::vec::Vec<ChoiceOptionParam>,
    pub instructions: QuestionParamChoiceInstructions,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<QuestionParamChoiceName>,
    #[doc = "The type of the object. Always `choice`."]
    #[serde(rename = "type")]
    pub type_: QuestionParamChoiceType,
}
impl QuestionParamChoice {
    pub fn builder() -> builder::QuestionParamChoice {
        Default::default()
    }
}
#[doc = "`QuestionParamChoiceInstructions`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamChoiceInstructions(::std::string::String);
impl ::std::ops::Deref for QuestionParamChoiceInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamChoiceInstructions> for ::std::string::String {
    fn from(value: QuestionParamChoiceInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamChoiceInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamChoiceInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamChoiceInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamChoiceInstructions {
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
#[doc = "`QuestionParamChoiceName`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamChoiceName(::std::string::String);
impl ::std::ops::Deref for QuestionParamChoiceName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamChoiceName> for ::std::string::String {
    fn from(value: QuestionParamChoiceName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamChoiceName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamChoiceName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamChoiceName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamChoiceName {
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
#[doc = "The type of the object. Always `choice`."]
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
pub enum QuestionParamChoiceType {
    #[serde(rename = "choice")]
    Choice,
}
impl ::std::fmt::Display for QuestionParamChoiceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Choice => f.write_str("choice"),
        }
    }
}
impl ::std::str::FromStr for QuestionParamChoiceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "choice" => Ok(Self::Choice),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamChoiceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Estimate how likely it is that a statement about the input is true."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct QuestionParamPredicate {
    pub instructions: QuestionParamPredicateInstructions,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<QuestionParamPredicateName>,
    #[doc = "The type of the object. Always `predicate`."]
    #[serde(rename = "type")]
    pub type_: QuestionParamPredicateType,
}
impl QuestionParamPredicate {
    pub fn builder() -> builder::QuestionParamPredicate {
        Default::default()
    }
}
#[doc = "`QuestionParamPredicateInstructions`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamPredicateInstructions(::std::string::String);
impl ::std::ops::Deref for QuestionParamPredicateInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamPredicateInstructions> for ::std::string::String {
    fn from(value: QuestionParamPredicateInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamPredicateInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamPredicateInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamPredicateInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamPredicateInstructions {
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
#[doc = "`QuestionParamPredicateName`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamPredicateName(::std::string::String);
impl ::std::ops::Deref for QuestionParamPredicateName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamPredicateName> for ::std::string::String {
    fn from(value: QuestionParamPredicateName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamPredicateName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamPredicateName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamPredicateName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamPredicateName {
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
#[doc = "The type of the object. Always `predicate`."]
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
pub enum QuestionParamPredicateType {
    #[serde(rename = "predicate")]
    Predicate,
}
impl ::std::fmt::Display for QuestionParamPredicateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Predicate => f.write_str("predicate"),
        }
    }
}
impl ::std::str::FromStr for QuestionParamPredicateType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "predicate" => Ok(Self::Predicate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamPredicateType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamPredicateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Rate the input against the supplied ordered levels."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct QuestionParamScore {
    pub instructions: QuestionParamScoreInstructions,
    pub levels: ::std::vec::Vec<ScoreLevelParam>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<QuestionParamScoreName>,
    #[doc = "The type of the object. Always `score`."]
    #[serde(rename = "type")]
    pub type_: QuestionParamScoreType,
}
impl QuestionParamScore {
    pub fn builder() -> builder::QuestionParamScore {
        Default::default()
    }
}
#[doc = "`QuestionParamScoreInstructions`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamScoreInstructions(::std::string::String);
impl ::std::ops::Deref for QuestionParamScoreInstructions {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamScoreInstructions> for ::std::string::String {
    fn from(value: QuestionParamScoreInstructions) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamScoreInstructions {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamScoreInstructions {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamScoreInstructions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamScoreInstructions {
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
#[doc = "`QuestionParamScoreName`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QuestionParamScoreName(::std::string::String);
impl ::std::ops::Deref for QuestionParamScoreName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QuestionParamScoreName> for ::std::string::String {
    fn from(value: QuestionParamScoreName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QuestionParamScoreName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamScoreName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamScoreName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QuestionParamScoreName {
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
#[doc = "The type of the object. Always `score`."]
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
pub enum QuestionParamScoreType {
    #[serde(rename = "score")]
    Score,
}
impl ::std::fmt::Display for QuestionParamScoreType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => f.write_str("score"),
        }
    }
}
impl ::std::str::FromStr for QuestionParamScoreType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score" => Ok(Self::Score),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for QuestionParamScoreType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QuestionParamScoreType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ResponseUsageResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResponseUsageResource {
    pub input_tokens: i64,
    pub input_tokens_details: InputTokensDetailsResource,
    pub output_tokens: i64,
    pub output_tokens_details: OutputTokensDetailsResource,
    pub total_tokens: i64,
}
impl ResponseUsageResource {
    pub fn builder() -> builder::ResponseUsageResource {
        Default::default()
    }
}
#[doc = "`ScoreLevelParam`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScoreLevelParam {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<ScoreLevelParamDescription>,
    pub label: ScoreLevelParamLabel,
}
impl ScoreLevelParam {
    pub fn builder() -> builder::ScoreLevelParam {
        Default::default()
    }
}
#[doc = "`ScoreLevelParamDescription`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScoreLevelParamDescription(::std::string::String);
impl ::std::ops::Deref for ScoreLevelParamDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScoreLevelParamDescription> for ::std::string::String {
    fn from(value: ScoreLevelParamDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ScoreLevelParamDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScoreLevelParamDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScoreLevelParamDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScoreLevelParamDescription {
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
#[doc = "`ScoreLevelParamLabel`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ScoreLevelParamLabel(::std::string::String);
impl ::std::ops::Deref for ScoreLevelParamLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScoreLevelParamLabel> for ::std::string::String {
    fn from(value: ScoreLevelParamLabel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ScoreLevelParamLabel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScoreLevelParamLabel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScoreLevelParamLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScoreLevelParamLabel {
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
#[doc = "`ScoreProbabilityResource`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScoreProbabilityResource {
    pub label: ::std::string::String,
    pub probability: f64,
    pub value: i64,
}
impl ScoreProbabilityResource {
    pub fn builder() -> builder::ScoreProbabilityResource {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AnswerResourceChoice {
        choice: ::std::result::Result<super::ChoiceValueResource, ::std::string::String>,
        confidence: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        probabilities: ::std::result::Result<
            ::std::vec::Vec<super::ChoiceProbabilityResource>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AnswerResourceChoiceType, ::std::string::String>,
    }
    impl ::std::default::Default for AnswerResourceChoice {
        fn default() -> Self {
            Self {
                choice: Err("no value supplied for choice".to_string()),
                confidence: Err("no value supplied for confidence".to_string()),
                name: Err("no value supplied for name".to_string()),
                probabilities: Err("no value supplied for probabilities".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnswerResourceChoice {
        pub fn choice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChoiceValueResource>,
            T::Error: ::std::fmt::Display,
        {
            self.choice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choice: {e}"));
            self
        }
        pub fn confidence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confidence: {e}"));
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
        pub fn probabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChoiceProbabilityResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.probabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for probabilities: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnswerResourceChoiceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AnswerResourceChoice> for super::AnswerResourceChoice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnswerResourceChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                choice: value.choice?,
                confidence: value.confidence?,
                name: value.name?,
                probabilities: value.probabilities?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AnswerResourceChoice> for AnswerResourceChoice {
        fn from(value: super::AnswerResourceChoice) -> Self {
            Self {
                choice: Ok(value.choice),
                confidence: Ok(value.confidence),
                name: Ok(value.name),
                probabilities: Ok(value.probabilities),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AnswerResourcePredicate {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        probability: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<super::AnswerResourcePredicateType, ::std::string::String>,
    }
    impl ::std::default::Default for AnswerResourcePredicate {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                probability: Err("no value supplied for probability".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnswerResourcePredicate {
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
        pub fn probability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.probability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for probability: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnswerResourcePredicateType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AnswerResourcePredicate> for super::AnswerResourcePredicate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnswerResourcePredicate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                probability: value.probability?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AnswerResourcePredicate> for AnswerResourcePredicate {
        fn from(value: super::AnswerResourcePredicate) -> Self {
            Self {
                name: Ok(value.name),
                probability: Ok(value.probability),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AnswerResourceRefusal {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AnswerResourceRefusalType, ::std::string::String>,
    }
    impl ::std::default::Default for AnswerResourceRefusal {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnswerResourceRefusal {
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnswerResourceRefusalType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AnswerResourceRefusal> for super::AnswerResourceRefusal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnswerResourceRefusal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AnswerResourceRefusal> for AnswerResourceRefusal {
        fn from(value: super::AnswerResourceRefusal) -> Self {
            Self {
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AnswerResourceScore {
        confidence: ::std::result::Result<f64, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        probabilities: ::std::result::Result<
            ::std::vec::Vec<super::ScoreProbabilityResource>,
            ::std::string::String,
        >,
        score: ::std::result::Result<f64, ::std::string::String>,
        type_: ::std::result::Result<super::AnswerResourceScoreType, ::std::string::String>,
    }
    impl ::std::default::Default for AnswerResourceScore {
        fn default() -> Self {
            Self {
                confidence: Err("no value supplied for confidence".to_string()),
                name: Err("no value supplied for name".to_string()),
                probabilities: Err("no value supplied for probabilities".to_string()),
                score: Err("no value supplied for score".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnswerResourceScore {
        pub fn confidence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confidence: {e}"));
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
        pub fn probabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ScoreProbabilityResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.probabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for probabilities: {e}"));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnswerResourceScoreType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AnswerResourceScore> for super::AnswerResourceScore {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnswerResourceScore,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                confidence: value.confidence?,
                name: value.name?,
                probabilities: value.probabilities?,
                score: value.score?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AnswerResourceScore> for AnswerResourceScore {
        fn from(value: super::AnswerResourceScore) -> Self {
            Self {
                confidence: Ok(value.confidence),
                name: Ok(value.name),
                probabilities: Ok(value.probabilities),
                score: Ok(value.score),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChoiceOptionParam {
        description: ::std::result::Result<
            ::std::option::Option<super::ChoiceOptionParamDescription>,
            ::std::string::String,
        >,
        value: ::std::result::Result<super::ChoiceValueParam, ::std::string::String>,
    }
    impl ::std::default::Default for ChoiceOptionParam {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ChoiceOptionParam {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ChoiceOptionParamDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChoiceValueParam>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChoiceOptionParam> for super::ChoiceOptionParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChoiceOptionParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ChoiceOptionParam> for ChoiceOptionParam {
        fn from(value: super::ChoiceOptionParam) -> Self {
            Self {
                description: Ok(value.description),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChoiceProbabilityResource {
        probability: ::std::result::Result<f64, ::std::string::String>,
        value: ::std::result::Result<super::ChoiceValueResource, ::std::string::String>,
    }
    impl ::std::default::Default for ChoiceProbabilityResource {
        fn default() -> Self {
            Self {
                probability: Err("no value supplied for probability".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ChoiceProbabilityResource {
        pub fn probability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.probability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for probability: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ChoiceValueResource>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ChoiceProbabilityResource> for super::ChoiceProbabilityResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ChoiceProbabilityResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                probability: value.probability?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ChoiceProbabilityResource> for ChoiceProbabilityResource {
        fn from(value: super::ChoiceProbabilityResource) -> Self {
            Self {
                probability: Ok(value.probability),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionInputImage {
        detail: ::std::result::Result<
            ::std::option::Option<super::ImageDetailParam>,
            ::std::string::String,
        >,
        image_url: ::std::result::Result<super::DecisionInputImageImageUrl, ::std::string::String>,
        type_: ::std::result::Result<super::DecisionInputImageType, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionInputImage {
        fn default() -> Self {
            Self {
                detail: Ok(Default::default()),
                image_url: Err("no value supplied for image_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DecisionInputImage {
        pub fn detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImageDetailParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.detail = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detail: {e}"));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputImageImageUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputImageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionInputImage> for super::DecisionInputImage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionInputImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detail: value.detail?,
                image_url: value.image_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionInputImage> for DecisionInputImage {
        fn from(value: super::DecisionInputImage) -> Self {
            Self {
                detail: Ok(value.detail),
                image_url: Ok(value.image_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionInputMessage {
        content: ::std::result::Result<super::DecisionInputContent, ::std::string::String>,
        role: ::std::result::Result<super::DecisionInputMessageRole, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::DecisionInputMessageType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionInputMessage {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                role: Err("no value supplied for role".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl DecisionInputMessage {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputMessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DecisionInputMessageType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionInputMessage> for super::DecisionInputMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionInputMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                role: value.role?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionInputMessage> for DecisionInputMessage {
        fn from(value: super::DecisionInputMessage) -> Self {
            Self {
                content: Ok(value.content),
                role: Ok(value.role),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionInputText {
        text: ::std::result::Result<super::DecisionInputTextText, ::std::string::String>,
        type_: ::std::result::Result<super::DecisionInputTextType, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionInputText {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DecisionInputText {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputTextText>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInputTextType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionInputText> for super::DecisionInputText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionInputText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionInputText> for DecisionInputText {
        fn from(value: super::DecisionInputText) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionRequest {
        input: ::std::result::Result<super::DecisionInput, ::std::string::String>,
        model: ::std::result::Result<super::DecisionRequestModel, ::std::string::String>,
        questions:
            ::std::result::Result<::std::vec::Vec<super::QuestionParam>, ::std::string::String>,
        safety_identifier: ::std::result::Result<
            ::std::option::Option<super::DecisionRequestSafetyIdentifier>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionRequest {
        fn default() -> Self {
            Self {
                input: Err("no value supplied for input".to_string()),
                model: Err("no value supplied for model".to_string()),
                questions: Err("no value supplied for questions".to_string()),
                safety_identifier: Ok(Default::default()),
            }
        }
    }
    impl DecisionRequest {
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionInput>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionRequestModel>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn questions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::QuestionParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.questions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for questions: {e}"));
            self
        }
        pub fn safety_identifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DecisionRequestSafetyIdentifier>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.safety_identifier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for safety_identifier: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionRequest> for super::DecisionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input: value.input?,
                model: value.model?,
                questions: value.questions?,
                safety_identifier: value.safety_identifier?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionRequest> for DecisionRequest {
        fn from(value: super::DecisionRequest) -> Self {
            Self {
                input: Ok(value.input),
                model: Ok(value.model),
                questions: Ok(value.questions),
                safety_identifier: Ok(value.safety_identifier),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionResponse {
        answers:
            ::std::result::Result<::std::vec::Vec<super::AnswerResource>, ::std::string::String>,
        model: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<super::ResponseUsageResource, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionResponse {
        fn default() -> Self {
            Self {
                answers: Err("no value supplied for answers".to_string()),
                model: Err("no value supplied for model".to_string()),
                usage: Err("no value supplied for usage".to_string()),
            }
        }
    }
    impl DecisionResponse {
        pub fn answers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AnswerResource>>,
            T::Error: ::std::fmt::Display,
        {
            self.answers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for answers: {e}"));
            self
        }
        pub fn model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.model = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model: {e}"));
            self
        }
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ResponseUsageResource>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for usage: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionResponse> for super::DecisionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                answers: value.answers?,
                model: value.model?,
                usage: value.usage?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionResponse> for DecisionResponse {
        fn from(value: super::DecisionResponse) -> Self {
            Self {
                answers: Ok(value.answers),
                model: Ok(value.model),
                usage: Ok(value.usage),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionsApiSchema {
        create_decision_request:
            ::std::result::Result<super::DecisionRequest, ::std::string::String>,
        create_decision_response:
            ::std::result::Result<super::DecisionResponse, ::std::string::String>,
    }
    impl ::std::default::Default for DecisionsApiSchema {
        fn default() -> Self {
            Self {
                create_decision_request: Err(
                    "no value supplied for create_decision_request".to_string()
                ),
                create_decision_response: Err(
                    "no value supplied for create_decision_response".to_string()
                ),
            }
        }
    }
    impl DecisionsApiSchema {
        pub fn create_decision_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionRequest>,
            T::Error: ::std::fmt::Display,
        {
            self.create_decision_request = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_decision_request: {e}")
            });
            self
        }
        pub fn create_decision_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionResponse>,
            T::Error: ::std::fmt::Display,
        {
            self.create_decision_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for create_decision_response: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionsApiSchema> for super::DecisionsApiSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionsApiSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                create_decision_request: value.create_decision_request?,
                create_decision_response: value.create_decision_response?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionsApiSchema> for DecisionsApiSchema {
        fn from(value: super::DecisionsApiSchema) -> Self {
            Self {
                create_decision_request: Ok(value.create_decision_request),
                create_decision_response: Ok(value.create_decision_response),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputTokensDetailsResource {
        cache_write_tokens: ::std::result::Result<i64, ::std::string::String>,
        cached_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for InputTokensDetailsResource {
        fn default() -> Self {
            Self {
                cache_write_tokens: Err("no value supplied for cache_write_tokens".to_string()),
                cached_tokens: Err("no value supplied for cached_tokens".to_string()),
            }
        }
    }
    impl InputTokensDetailsResource {
        pub fn cache_write_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cache_write_tokens = value.try_into().map_err(|e| {
                format!("error converting supplied value for cache_write_tokens: {e}")
            });
            self
        }
        pub fn cached_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.cached_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cached_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<InputTokensDetailsResource> for super::InputTokensDetailsResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InputTokensDetailsResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cache_write_tokens: value.cache_write_tokens?,
                cached_tokens: value.cached_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::InputTokensDetailsResource> for InputTokensDetailsResource {
        fn from(value: super::InputTokensDetailsResource) -> Self {
            Self {
                cache_write_tokens: Ok(value.cache_write_tokens),
                cached_tokens: Ok(value.cached_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OutputTokensDetailsResource {
        reasoning_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for OutputTokensDetailsResource {
        fn default() -> Self {
            Self {
                reasoning_tokens: Err("no value supplied for reasoning_tokens".to_string()),
            }
        }
    }
    impl OutputTokensDetailsResource {
        pub fn reasoning_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.reasoning_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasoning_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OutputTokensDetailsResource> for super::OutputTokensDetailsResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OutputTokensDetailsResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                reasoning_tokens: value.reasoning_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::OutputTokensDetailsResource> for OutputTokensDetailsResource {
        fn from(value: super::OutputTokensDetailsResource) -> Self {
            Self {
                reasoning_tokens: Ok(value.reasoning_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct QuestionParamChoice {
        choices:
            ::std::result::Result<::std::vec::Vec<super::ChoiceOptionParam>, ::std::string::String>,
        instructions:
            ::std::result::Result<super::QuestionParamChoiceInstructions, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<super::QuestionParamChoiceName>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::QuestionParamChoiceType, ::std::string::String>,
    }
    impl ::std::default::Default for QuestionParamChoice {
        fn default() -> Self {
            Self {
                choices: Err("no value supplied for choices".to_string()),
                instructions: Err("no value supplied for instructions".to_string()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl QuestionParamChoice {
        pub fn choices<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ChoiceOptionParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.choices = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for choices: {e}"));
            self
        }
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamChoiceInstructions>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::QuestionParamChoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamChoiceType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<QuestionParamChoice> for super::QuestionParamChoice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: QuestionParamChoice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                choices: value.choices?,
                instructions: value.instructions?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::QuestionParamChoice> for QuestionParamChoice {
        fn from(value: super::QuestionParamChoice) -> Self {
            Self {
                choices: Ok(value.choices),
                instructions: Ok(value.instructions),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct QuestionParamPredicate {
        instructions:
            ::std::result::Result<super::QuestionParamPredicateInstructions, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<super::QuestionParamPredicateName>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::QuestionParamPredicateType, ::std::string::String>,
    }
    impl ::std::default::Default for QuestionParamPredicate {
        fn default() -> Self {
            Self {
                instructions: Err("no value supplied for instructions".to_string()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl QuestionParamPredicate {
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamPredicateInstructions>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::QuestionParamPredicateName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamPredicateType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<QuestionParamPredicate> for super::QuestionParamPredicate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: QuestionParamPredicate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instructions: value.instructions?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::QuestionParamPredicate> for QuestionParamPredicate {
        fn from(value: super::QuestionParamPredicate) -> Self {
            Self {
                instructions: Ok(value.instructions),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct QuestionParamScore {
        instructions:
            ::std::result::Result<super::QuestionParamScoreInstructions, ::std::string::String>,
        levels:
            ::std::result::Result<::std::vec::Vec<super::ScoreLevelParam>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<super::QuestionParamScoreName>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::QuestionParamScoreType, ::std::string::String>,
    }
    impl ::std::default::Default for QuestionParamScore {
        fn default() -> Self {
            Self {
                instructions: Err("no value supplied for instructions".to_string()),
                levels: Err("no value supplied for levels".to_string()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl QuestionParamScore {
        pub fn instructions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamScoreInstructions>,
            T::Error: ::std::fmt::Display,
        {
            self.instructions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for instructions: {e}"));
            self
        }
        pub fn levels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ScoreLevelParam>>,
            T::Error: ::std::fmt::Display,
        {
            self.levels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for levels: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::QuestionParamScoreName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QuestionParamScoreType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<QuestionParamScore> for super::QuestionParamScore {
        type Error = super::error::ConversionError;
        fn try_from(
            value: QuestionParamScore,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                instructions: value.instructions?,
                levels: value.levels?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::QuestionParamScore> for QuestionParamScore {
        fn from(value: super::QuestionParamScore) -> Self {
            Self {
                instructions: Ok(value.instructions),
                levels: Ok(value.levels),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResponseUsageResource {
        input_tokens: ::std::result::Result<i64, ::std::string::String>,
        input_tokens_details:
            ::std::result::Result<super::InputTokensDetailsResource, ::std::string::String>,
        output_tokens: ::std::result::Result<i64, ::std::string::String>,
        output_tokens_details:
            ::std::result::Result<super::OutputTokensDetailsResource, ::std::string::String>,
        total_tokens: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for ResponseUsageResource {
        fn default() -> Self {
            Self {
                input_tokens: Err("no value supplied for input_tokens".to_string()),
                input_tokens_details: Err("no value supplied for input_tokens_details".to_string()),
                output_tokens: Err("no value supplied for output_tokens".to_string()),
                output_tokens_details: Err(
                    "no value supplied for output_tokens_details".to_string()
                ),
                total_tokens: Err("no value supplied for total_tokens".to_string()),
            }
        }
    }
    impl ResponseUsageResource {
        pub fn input_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_tokens: {e}"));
            self
        }
        pub fn input_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::InputTokensDetailsResource>,
            T::Error: ::std::fmt::Display,
        {
            self.input_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for input_tokens_details: {e}")
            });
            self
        }
        pub fn output_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_tokens: {e}"));
            self
        }
        pub fn output_tokens_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OutputTokensDetailsResource>,
            T::Error: ::std::fmt::Display,
        {
            self.output_tokens_details = value.try_into().map_err(|e| {
                format!("error converting supplied value for output_tokens_details: {e}")
            });
            self
        }
        pub fn total_tokens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.total_tokens = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_tokens: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ResponseUsageResource> for super::ResponseUsageResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResponseUsageResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_tokens: value.input_tokens?,
                input_tokens_details: value.input_tokens_details?,
                output_tokens: value.output_tokens?,
                output_tokens_details: value.output_tokens_details?,
                total_tokens: value.total_tokens?,
            })
        }
    }
    impl ::std::convert::From<super::ResponseUsageResource> for ResponseUsageResource {
        fn from(value: super::ResponseUsageResource) -> Self {
            Self {
                input_tokens: Ok(value.input_tokens),
                input_tokens_details: Ok(value.input_tokens_details),
                output_tokens: Ok(value.output_tokens),
                output_tokens_details: Ok(value.output_tokens_details),
                total_tokens: Ok(value.total_tokens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScoreLevelParam {
        description: ::std::result::Result<
            ::std::option::Option<super::ScoreLevelParamDescription>,
            ::std::string::String,
        >,
        label: ::std::result::Result<super::ScoreLevelParamLabel, ::std::string::String>,
    }
    impl ::std::default::Default for ScoreLevelParam {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                label: Err("no value supplied for label".to_string()),
            }
        }
    }
    impl ScoreLevelParam {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ScoreLevelParamDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScoreLevelParamLabel>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ScoreLevelParam> for super::ScoreLevelParam {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScoreLevelParam,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::ScoreLevelParam> for ScoreLevelParam {
        fn from(value: super::ScoreLevelParam) -> Self {
            Self {
                description: Ok(value.description),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScoreProbabilityResource {
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        probability: ::std::result::Result<f64, ::std::string::String>,
        value: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for ScoreProbabilityResource {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                probability: Err("no value supplied for probability".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ScoreProbabilityResource {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn probability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.probability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for probability: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ScoreProbabilityResource> for super::ScoreProbabilityResource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScoreProbabilityResource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                probability: value.probability?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ScoreProbabilityResource> for ScoreProbabilityResource {
        fn from(value: super::ScoreProbabilityResource) -> Self {
            Self {
                label: Ok(value.label),
                probability: Ok(value.probability),
                value: Ok(value.value),
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
