// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/uuid.fbs".

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

/// **Datatype**: A 16-byte uuid.
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Uuid {
    pub bytes: [u8; 16usize],
}

impl From<[u8; 16usize]> for Uuid {
    #[inline]
    fn from(bytes: [u8; 16usize]) -> Self {
        Self { bytes }
    }
}

impl From<Uuid> for [u8; 16usize] {
    #[inline]
    fn from(value: Uuid) -> Self {
        value.bytes
    }
}

::re_types_core::macros::impl_into_cow!(Uuid);

impl ::re_types_core::Loggable for Uuid {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Uuid".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "bytes".to_owned(),
            data_type: DataType::FixedSizeList(
                Box::new(Field {
                    name: "item".to_owned(),
                    data_type: DataType::UInt8,
                    is_nullable: false,
                    metadata: [].into(),
                }),
                16usize,
            ),
            is_nullable: false,
            metadata: [].into(),
        }])
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::datatypes::Uuid>::arrow_datatype(),
                vec![{
                    let (somes, bytes): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum = datum.as_ref().map(|datum| {
                                let Self { bytes, .. } = &**datum;
                                bytes.clone()
                            });
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let bytes_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let bytes_inner_data: Vec<_> = bytes
                            .iter()
                            .flat_map(|v| match v {
                                Some(v) => itertools::Either::Left(v.iter().cloned()),
                                None => itertools::Either::Right(
                                    std::iter::repeat(Default::default()).take(16usize),
                                ),
                            })
                            .map(Some)
                            .collect();
                        let bytes_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                            bytes_bitmap.as_ref().map(|bitmap| {
                                bitmap
                                    .iter()
                                    .map(|i| std::iter::repeat(i).take(16usize))
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .into()
                            });
                        FixedSizeListArray::new(
                            DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::UInt8,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                16usize,
                            ),
                            PrimitiveArray::new(
                                DataType::UInt8,
                                bytes_inner_data
                                    .into_iter()
                                    .map(|v| v.unwrap_or_default())
                                    .collect(),
                                bytes_inner_bitmap,
                            )
                            .boxed(),
                            bytes_bitmap,
                        )
                        .boxed()
                    }
                }],
                bitmap,
            )
            .boxed()
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
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![Field {
                            name: "bytes".to_owned(),
                            data_type: DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::UInt8,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                16usize,
                            ),
                            is_nullable: false,
                            metadata: [].into(),
                        }]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.Uuid")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let bytes = {
                    if !arrays_by_name.contains_key("bytes") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "bytes",
                        ))
                        .with_context("rerun.datatypes.Uuid");
                    }
                    let arrow_data = &**arrays_by_name["bytes"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                DeserializationError::datatype_mismatch(
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::UInt8,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        16usize,
                                    ),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.datatypes.Uuid#bytes")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(16usize)
                                .zip((16usize..).step_by(16usize).take(arrow_data.len()));
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<UInt8Array>()
                                    .ok_or_else(|| {
                                        DeserializationError::datatype_mismatch(
                                            DataType::UInt8,
                                            arrow_data_inner.data_type().clone(),
                                        )
                                    })
                                    .with_context("rerun.datatypes.Uuid#bytes")?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end)| {
                                    debug_assert!(end - start == 16usize);
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);
                                    let arr = array_init::from_iter(data).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(bytes),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(bytes)| {
                        Ok(Self {
                            bytes: bytes
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.Uuid#bytes")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.Uuid")?
            }
        })
    }
}
