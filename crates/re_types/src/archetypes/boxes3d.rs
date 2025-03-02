// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/boxes3d.fbs".

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

/// **Archetype**: 3D boxes with half-extents and optional center, rotations, rotations, colors etc.
///
/// ## Example
///
/// ### Batch of 3D boxes
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_box3d_batch").spawn()?;
///
///     rec.log(
///         "batch",
///         &rerun::Boxes3D::from_centers_and_half_sizes(
///             [(2.0, 0.0, 0.0), (-2.0, 0.0, 0.0), (0.0, 0.0, 2.0)],
///             [(2.0, 2.0, 1.0), (1.0, 1.0, 0.5), (2.0, 0.5, 1.0)],
///         )
///         .with_rotations([
///             rerun::Rotation3D::IDENTITY,
///             rerun::Quaternion::from_xyzw([0.0, 0.0, 0.382683, 0.923880]).into(), // 45 degrees around Z
///             rerun::RotationAxisAngle::new((0.0, 1.0, 0.0), rerun::Angle::Degrees(30.0)).into(),
///         ])
///         .with_radii([0.025])
///         .with_colors([
///             rerun::Color::from_rgb(255, 0, 0),
///             rerun::Color::from_rgb(0, 255, 0),
///             rerun::Color::from_rgb(0, 0, 255),
///         ])
///         .with_labels(["red", "green", "blue"]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/box3d_batch/6d3e453c3a0201ae42bbae9de941198513535f1d/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/box3d_batch/6d3e453c3a0201ae42bbae9de941198513535f1d/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/box3d_batch/6d3e453c3a0201ae42bbae9de941198513535f1d/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/box3d_batch/6d3e453c3a0201ae42bbae9de941198513535f1d/1200w.png">
///   <img src="https://static.rerun.io/box3d_batch/6d3e453c3a0201ae42bbae9de941198513535f1d/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Boxes3D {
    /// All half-extents that make up the batch of boxes.
    pub half_sizes: Vec<crate::components::HalfSizes3D>,

    /// Optional center positions of the boxes.
    pub centers: Option<Vec<crate::components::Position3D>>,
    pub rotations: Option<Vec<crate::components::Rotation3D>>,

    /// Optional colors for the boxes.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional radii for the lines that make up the boxes.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional text labels for the boxes.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional `ClassId`s for the boxes.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Unique identifiers for each individual boxes in the batch.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.HalfSizes3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Boxes3DIndicator".into(),
            "rerun.components.Color".into(),
            "rerun.components.Position3D".into(),
            "rerun.components.Rotation3D".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.ClassId".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Text".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.HalfSizes3D".into(),
            "rerun.components.Boxes3DIndicator".into(),
            "rerun.components.Color".into(),
            "rerun.components.Position3D".into(),
            "rerun.components.Rotation3D".into(),
            "rerun.components.ClassId".into(),
            "rerun.components.InstanceKey".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Text".into(),
        ]
    });

impl Boxes3D {
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Boxes3D`] [`::re_types_core::Archetype`]
pub type Boxes3DIndicator = ::re_types_core::GenericIndicatorComponent<Boxes3D>;

impl ::re_types_core::Archetype for Boxes3D {
    type Indicator = Boxes3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Boxes3D".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Boxes3DIndicator = Boxes3DIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let half_sizes = {
            let array = arrays_by_name
                .get("rerun.components.HalfSizes3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Boxes3D#half_sizes")?;
            <crate::components::HalfSizes3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Boxes3D#half_sizes")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Boxes3D#half_sizes")?
        };
        let centers = if let Some(array) = arrays_by_name.get("rerun.components.Position3D") {
            Some({
                <crate::components::Position3D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#centers")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#centers")?
            })
        } else {
            None
        };
        let rotations = if let Some(array) = arrays_by_name.get("rerun.components.Rotation3D") {
            Some({
                <crate::components::Rotation3D>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#rotations")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#rotations")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#colors")?
            })
        } else {
            None
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#radii")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#labels")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#class_ids")?
            })
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("rerun.components.InstanceKey")
        {
            Some({
                <crate::components::InstanceKey>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Boxes3D#instance_keys")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Boxes3D#instance_keys")?
            })
        } else {
            None
        };
        Ok(Self {
            half_sizes,
            centers,
            rotations,
            colors,
            radii,
            labels,
            class_ids,
            instance_keys,
        })
    }
}

impl ::re_types_core::AsComponents for Boxes3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.half_sizes as &dyn ComponentBatch).into()),
            self.centers
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.rotations
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.instance_keys
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.half_sizes.len()
    }
}

impl Boxes3D {
    pub(crate) fn new(
        half_sizes: impl IntoIterator<Item = impl Into<crate::components::HalfSizes3D>>,
    ) -> Self {
        Self {
            half_sizes: half_sizes.into_iter().map(Into::into).collect(),
            centers: None,
            rotations: None,
            colors: None,
            radii: None,
            labels: None,
            class_ids: None,
            instance_keys: None,
        }
    }

    #[inline]
    pub fn with_centers(
        mut self,
        centers: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        self.centers = Some(centers.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_rotations(
        mut self,
        rotations: impl IntoIterator<Item = impl Into<crate::components::Rotation3D>>,
    ) -> Self {
        self.rotations = Some(rotations.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
