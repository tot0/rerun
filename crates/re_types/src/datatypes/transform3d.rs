// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/transform3d.fbs".

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

/// **Datatype**: Representation of a 3D affine transform.
///
/// Rarely used directly, prefer using the underlying representation classes and pass them
/// directly to `Transform3D::child_from_parent` or `Transform3D::parent_from_child`.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Transform3D {
    TranslationAndMat3x3(crate::datatypes::TranslationAndMat3x3),
    TranslationRotationScale(crate::datatypes::TranslationRotationScale3D),
}

::re_types_core::macros::impl_into_cow!(Transform3D);

impl ::re_types_core::Loggable for Transform3D {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Transform3D".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationAndMat3x3".to_owned(),
                    data_type: <crate::datatypes::TranslationAndMat3x3>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationRotationScale".to_owned(),
                    data_type: <crate::datatypes::TranslationRotationScale3D>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            Some(vec![0i32, 1i32, 2i32]),
            UnionMode::Dense,
        )
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
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            UnionArray::new(
                <crate::datatypes::Transform3D>::arrow_datatype(),
                data.iter()
                    .map(|a| match a.as_deref() {
                        None => 0,
                        Some(Transform3D::TranslationAndMat3x3(_)) => 1i8,
                        Some(Transform3D::TranslationRotationScale(_)) => 2i8,
                    })
                    .collect(),
                vec![
                    NullArray::new(DataType::Null, data.iter().filter(|v| v.is_none()).count())
                        .boxed(),
                    {
                        let (somes, translation_and_mat3x3): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(
                                    datum.as_deref(),
                                    Some(Transform3D::TranslationAndMat3x3(_))
                                )
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Transform3D::TranslationAndMat3x3(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_and_mat3x3_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_and_mat3x3_bitmap;
                            crate::datatypes::TranslationAndMat3x3::to_arrow_opt(
                                translation_and_mat3x3,
                            )?
                        }
                    },
                    {
                        let (somes, translation_rotation_scale): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(
                                    datum.as_deref(),
                                    Some(Transform3D::TranslationRotationScale(_))
                                )
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Transform3D::TranslationRotationScale(v)) => {
                                        Some(v.clone())
                                    }
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_rotation_scale_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_rotation_scale_bitmap;
                            crate::datatypes::TranslationRotationScale3D::to_arrow_opt(
                                translation_rotation_scale,
                            )?
                        }
                    },
                ],
                Some({
                    let mut translation_and_mat3x3_offset = 0;
                    let mut translation_rotation_scale_offset = 0;
                    let mut nulls_offset = 0;
                    data.iter()
                        .map(|v| match v.as_deref() {
                            None => {
                                let offset = nulls_offset;
                                nulls_offset += 1;
                                offset
                            }
                            Some(Transform3D::TranslationAndMat3x3(_)) => {
                                let offset = translation_and_mat3x3_offset;
                                translation_and_mat3x3_offset += 1;
                                offset
                            }
                            Some(Transform3D::TranslationRotationScale(_)) => {
                                let offset = translation_rotation_scale_offset;
                                translation_rotation_scale_offset += 1;
                                offset
                            }
                        })
                        .collect()
                }),
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
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Union(
                            vec![
                            Field { name : "_null_markers".to_owned(), data_type :
                            DataType::Null, is_nullable : true, metadata : [].into(), },
                            Field { name : "TranslationAndMat3x3".to_owned(), data_type :
                            < crate ::datatypes::TranslationAndMat3x3 >
                            ::arrow_datatype(), is_nullable : false, metadata : []
                            .into(), }, Field { name : "TranslationRotationScale"
                            .to_owned(), data_type : < crate
                            ::datatypes::TranslationRotationScale3D > ::arrow_datatype(),
                            is_nullable : false, metadata : [].into(), },
                        ],
                            Some(vec![0i32, 1i32, 2i32]),
                            UnionMode::Dense,
                        ),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.Transform3D")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_types, arrow_data_arrays) =
                    (arrow_data.types(), arrow_data.fields());
                let arrow_data_offsets = arrow_data
                    .offsets()
                    .ok_or_else(|| {
                        DeserializationError::datatype_mismatch(
                            Self::arrow_datatype(),
                            arrow_data.data_type().clone(),
                        )
                    })
                    .with_context("rerun.datatypes.Transform3D")?;
                if arrow_data_types.len() != arrow_data_offsets.len() {
                    return Err(DeserializationError::offset_slice_oob(
                        (0, arrow_data_types.len()),
                        arrow_data_offsets.len(),
                    ))
                    .with_context("rerun.datatypes.Transform3D");
                }
                let translation_and_mat3x3 = {
                    if 1usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[1usize];
                    crate::datatypes::TranslationAndMat3x3::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.Transform3D#TranslationAndMat3x3")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                let translation_rotation_scale = {
                    if 2usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[2usize];
                    crate::datatypes::TranslationRotationScale3D::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.Transform3D#TranslationRotationScale")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                arrow_data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = arrow_data_offsets[i];
                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(match typ {
                                1i8 => Transform3D::TranslationAndMat3x3({
                                    if offset as usize >= translation_and_mat3x3.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            translation_and_mat3x3.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.Transform3D#TranslationAndMat3x3",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { translation_and_mat3x3.get_unchecked(offset as usize) }
                                        .clone()
                                        .ok_or_else(DeserializationError::missing_data)
                                        .with_context(
                                            "rerun.datatypes.Transform3D#TranslationAndMat3x3",
                                        )?
                                }),
                                2i8 => Transform3D::TranslationRotationScale({
                                    if offset as usize >= translation_rotation_scale.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            translation_rotation_scale.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.Transform3D#TranslationRotationScale",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe {
                                        translation_rotation_scale.get_unchecked(offset as usize)
                                    }
                                    .clone()
                                    .ok_or_else(DeserializationError::missing_data)
                                    .with_context(
                                        "rerun.datatypes.Transform3D#TranslationRotationScale",
                                    )?
                                }),
                                _ => {
                                    return Err(DeserializationError::missing_union_arm(
                                        Self::arrow_datatype(),
                                        "<invalid>",
                                        *typ as _,
                                    ))
                                    .with_context("rerun.datatypes.Transform3D");
                                }
                            }))
                        }
                    })
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.datatypes.Transform3D")?
            }
        })
    }
}
