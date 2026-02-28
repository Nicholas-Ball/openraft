//! A boxed error wrapper for smaller error type sizes.

use std::error::Error;
use std::fmt;

use anyerror::AnyError;

use crate::errors::ErrorSource;

/// A boxed wrapper around [`AnyError`] for smaller error type sizes.
///
/// This type stores `AnyError` in a `Box`, reducing the size of error types
/// that contain it (like `StorageError`). This is the default `ErrorSource`
/// implementation used by [`declare_raft_types!`](crate::declare_raft_types).
///
/// Use [`AnyError`] directly if you prefer inline storage and don't mind
/// larger error types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize))]
pub struct BoxedErrorSource {
    #[cfg_attr(feature = "rkyv", rkyv(with = AnyErrorAsString))]
    inner: Box<AnyError>,
}

#[cfg(feature = "rkyv")]
struct AnyErrorAsString;

#[cfg(feature = "rkyv")]
impl rkyv::with::ArchiveWith<Box<AnyError>> for AnyErrorAsString {
    type Archived = <String as rkyv::Archive>::Archived;
    type Resolver = <String as rkyv::Archive>::Resolver;

    fn resolve_with(field: &Box<AnyError>, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
        <String as rkyv::Archive>::resolve(&field.to_string(), resolver, out);
    }
}

#[cfg(feature = "rkyv")]
impl<S> rkyv::with::SerializeWith<Box<AnyError>, S> for AnyErrorAsString
where
    S: rkyv::rancor::Fallible + ?Sized,
    String: rkyv::Serialize<S>,
{
    fn serialize_with(field: &Box<AnyError>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        <String as rkyv::Serialize<S>>::serialize(&field.to_string(), serializer)
    }
}

#[cfg(feature = "rkyv")]
impl<D> rkyv::with::DeserializeWith<<String as rkyv::Archive>::Archived, Box<AnyError>, D> for AnyErrorAsString
where
    D: rkyv::rancor::Fallible + ?Sized,
    <String as rkyv::Archive>::Archived: rkyv::Deserialize<String, D>,
{
    fn deserialize_with(
        field: &<String as rkyv::Archive>::Archived,
        deserializer: &mut D,
    ) -> Result<Box<AnyError>, D::Error> {
        let message =
            <<String as rkyv::Archive>::Archived as rkyv::Deserialize<String, D>>::deserialize(field, deserializer)?;
        Ok(Box::new(AnyError::error(message)))
    }
}

impl fmt::Display for BoxedErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Error for BoxedErrorSource {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.source()
    }
}

impl ErrorSource for BoxedErrorSource {
    fn from_error<E: Error + 'static>(error: &E) -> Self {
        Self {
            inner: Box::new(AnyError::new(error)),
        }
    }

    fn from_string(msg: impl ToString) -> Self {
        Self {
            inner: Box::new(AnyError::error(msg)),
        }
    }

    fn has_backtrace(&self) -> bool {
        anyerror::backtrace_str().is_some()
    }

    fn fmt_backtrace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(bt) = anyerror::backtrace_str() {
            write!(f, "{}", bt)
        } else {
            Ok(())
        }
    }
}
