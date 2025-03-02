// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/query_expressions.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A way to filter a set of `EntityPath`s.
///
/// This implements as simple set of include/exclude rules:
///
/// ```diff
/// + /world/**           # add everything…
/// - /world/roads/**     # …but remove all roads…
/// + /world/roads/main   # …but show main road
/// ```
///
/// If there is multiple matching rules, the most specific rule wins.
/// If there are multiple rules of the same specificity, the last one wins.
/// If no rules match, the path is excluded.
///
/// The `/**` suffix matches the whole subtree, i.e. self and any child, recursively
/// (`/world/**` matches both `/world` and `/world/car/driver`).
/// Other uses of `*` are not (yet) supported.
///
/// `EntityPathFilter` sorts the rule by entity path, with recursive coming before non-recursive.
/// This means the last matching rule is also the most specific one.
/// For instance:
///
/// ```diff
/// + /world/**
/// - /world
/// - /world/car/**
/// + /world/car/driver
/// ```
///
/// The last rule matching `/world/car/driver` is `+ /world/car/driver`, so it is included.
/// The last rule matching `/world/car/hood` is `- /world/car/**`, so it is excluded.
/// The last rule matching `/world` is `- /world`, so it is excluded.
/// The last rule matching `/world/house` is `+ /world/**`, so it is included.
///
/// Unstable. Used for the ongoing blueprint experimentations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryExpressions(pub ::re_types_core::ArrowString);

impl From<::re_types_core::ArrowString> for QueryExpressions {
    #[inline]
    fn from(filter: ::re_types_core::ArrowString) -> Self {
        Self(filter)
    }
}

impl From<QueryExpressions> for ::re_types_core::ArrowString {
    #[inline]
    fn from(value: QueryExpressions) -> Self {
        value.0
    }
}

::re_types_core::macros::impl_into_cow!(QueryExpressions);

impl ::re_types_core::Loggable for QueryExpressions {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.components.QueryExpressions".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Utf8
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                let inner_data: arrow2::buffer::Buffer<u8> =
                    data0.iter().flatten().flat_map(|s| s.0.clone()).collect();
                let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()),
                )
                .unwrap()
                .into();

                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                unsafe {
                    Utf8Array::<i32>::new_unchecked(
                        Self::arrow_datatype(),
                        offsets,
                        inner_data,
                        data0_bitmap,
                    )
                }
                .boxed()
            }
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::Utf8Array<i32>>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Utf8,
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.blueprint.components.QueryExpressions#filter")?;
            let arrow_data_buf = arrow_data.values();
            let offsets = arrow_data.offsets();
            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                offsets.iter().zip(offsets.lengths()),
                arrow_data.validity(),
            )
            .map(|elem| {
                elem.map(|(start, len)| {
                    let start = *start as usize;
                    let end = start + len;
                    if end as usize > arrow_data_buf.len() {
                        return Err(DeserializationError::offset_slice_oob(
                            (start, end),
                            arrow_data_buf.len(),
                        ));
                    }

                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                    let data = unsafe { arrow_data_buf.clone().sliced_unchecked(start, len) };
                    Ok(data)
                })
                .transpose()
            })
            .map(|res_or_opt| {
                res_or_opt.map(|res_or_opt| res_or_opt.map(|v| ::re_types_core::ArrowString(v)))
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.blueprint.components.QueryExpressions#filter")?
            .into_iter()
        }
        .map(|v| v.ok_or_else(DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.blueprint.components.QueryExpressions#filter")
        .with_context("rerun.blueprint.components.QueryExpressions")?)
    }
}
