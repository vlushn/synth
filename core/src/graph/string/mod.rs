use super::prelude::*;

use rand_regex::Regex as RandRegex;

pub mod date_time;
pub use date_time::RandomDateTime;

pub mod faker;
pub mod format;
pub mod serialized;
pub mod truncated;
pub mod sliced;
pub mod uuid;
pub mod constant;

pub use self::uuid::UuidGen;
pub use faker::{FakerArgs, Locale, RandFaker};
pub use format::{Format, FormatArgs};
pub use serialized::Serialized;
pub use truncated::Truncated;
pub use sliced::Sliced;
pub use constant::Constant;

derive_generator! {
    yield String,
    return Result<String, Error>,
    pub enum RandomString {
        Regex(OnceInfallible<Random<String, RandRegex>>),
        Faker(TryOnce<RandFaker>),
        Serialized(TryOnce<Serialized>)
        Categorical(OnceInfallible<Random<String, Categorical<String>>>)
        Uuid(OnceInfallible<UuidGen>),
        Format(Format),
        Truncated(Truncated),
        Sliced(Sliced),
        Constant(Constant)
    }
}

impl From<RandFaker> for RandomString {
    fn from(faker: RandFaker) -> Self {
        Self::Faker(faker.try_once())
    }
}

impl From<Serialized> for RandomString {
    fn from(serialized: Serialized) -> Self {
        Self::Serialized(serialized.try_once())
    }
}

impl From<RandRegex> for RandomString {
    fn from(regex: RandRegex) -> Self {
        Self::Regex(Random::new_with(regex).infallible().try_once())
    }
}

impl From<Categorical<String>> for RandomString {
    fn from(cat: Categorical<String>) -> Self {
        Self::Categorical(Random::new_with(cat).infallible().try_once())
    }
}

impl From<UuidGen> for RandomString {
    fn from(uuid: UuidGen) -> Self {
        Self::Uuid(uuid.infallible().try_once())
    }
}

impl From<Truncated> for RandomString {
    fn from(trunc: Truncated) -> Self {
        Self::Truncated(trunc)
    }
}

impl From<Sliced> for RandomString {
    fn from(trunc: Sliced) -> Self {
        Self::Sliced(trunc)
    }
}

impl From<Constant> for RandomString {
    fn from(trunc: Constant) -> Self {
        Self::Constant(trunc)
    }
}

impl From<Format> for RandomString {
    fn from(format: Format) -> Self {
        RandomString::Format(format)
    }
}

derive_generator! {
    yield Token,
    return Result<Value, Error>,
    pub enum StringNode {
        String(Valuize<Tokenizer<RandomString>, String>),
        DateTime(Valuize<Tokenizer<RandomDateTime>, ChronoValueAndFormat>)
    }
}

impl From<RandomString> for StringNode {
    fn from(value: RandomString) -> Self {
        Self::String(value.into_token().map_complete(value_from_ok::<String>))
    }
}

impl From<RandomDateTime> for StringNode {
    fn from(value: RandomDateTime) -> Self {
        Self::DateTime(
            value
                .into_token()
                .map_complete(value_from_ok::<ChronoValueAndFormat>),
        )
    }
}
