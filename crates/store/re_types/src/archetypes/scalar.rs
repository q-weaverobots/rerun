// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/scalar.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A double-precision scalar, e.g. for use for time-series plots.
///
/// The current timeline value will be used for the time/X-axis, hence scalars
/// cannot be static.
///
/// When used to produce a plot, this archetype is used to provide the data that
/// is referenced by [`archetypes::SeriesLine`][crate::archetypes::SeriesLine] or [`archetypes::SeriesPoint`][crate::archetypes::SeriesPoint]. You can do
/// this by logging both archetypes to the same path, or alternatively configuring
/// the plot-specific archetypes through the blueprint.
///
/// ## Examples
///
/// ### Simple line plot
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar").spawn()?;
///
///     // Log the data on a timeline called "step".
///     for step in 0..64 {
///         rec.set_time_sequence("step", step);
///         rec.log("scalar", &rerun::Scalar::new((step as f64 / 10.0).sin()))?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
///   <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Multiple scalars in a single `send_columns` call
/// ```ignore
/// use rerun::TimeColumn;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar_send_columns").spawn()?;
///
///     const STEPS: i64 = 64;
///
///     let times = TimeColumn::new_sequence("step", 0..STEPS);
///     let scalars = (0..STEPS).map(|step| (step as f64 / 10.0).sin());
///
///     rec.send_columns(
///         "scalars",
///         [times],
///         rerun::Scalar::update_fields()
///             .with_many_scalar(scalars)
///             .columns_of_unit_batches()?,
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_send_columns/b4bf172256f521f4851dfec5c2c6e3143f5d6923/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_send_columns/b4bf172256f521f4851dfec5c2c6e3143f5d6923/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_send_columns/b4bf172256f521f4851dfec5c2c6e3143f5d6923/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_send_columns/b4bf172256f521f4851dfec5c2c6e3143f5d6923/1200w.png">
///   <img src="https://static.rerun.io/scalar_send_columns/b4bf172256f521f4851dfec5c2c6e3143f5d6923/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Scalar {
    /// The scalar value to log.
    pub scalar: Option<SerializedComponentBatch>,
}

impl Scalar {
    /// Returns the [`ComponentDescriptor`] for [`Self::scalar`].
    #[inline]
    pub fn descriptor_scalar() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Scalar".into()),
            component_name: "rerun.components.Scalar".into(),
            archetype_field_name: Some("scalar".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Scalar".into()),
            component_name: "rerun.components.ScalarIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Scalar::descriptor_scalar()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Scalar::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| [Scalar::descriptor_scalar(), Scalar::descriptor_indicator()]);

impl Scalar {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`Scalar`] [`::re_types_core::Archetype`]
pub type ScalarIndicator = ::re_types_core::GenericIndicatorComponent<Scalar>;

impl ::re_types_core::Archetype for Scalar {
    type Indicator = ScalarIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Scalar".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Scalar"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ScalarIndicator = ScalarIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let scalar = arrays_by_descr
            .get(&Self::descriptor_scalar())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_scalar()));
        Ok(Self { scalar })
    }
}

impl ::re_types_core::AsComponents for Scalar {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [Self::indicator().serialized(), self.scalar.clone()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Scalar {}

impl Scalar {
    /// Create a new `Scalar`.
    #[inline]
    pub fn new(scalar: impl Into<crate::components::Scalar>) -> Self {
        Self {
            scalar: try_serialize_field(Self::descriptor_scalar(), [scalar]),
        }
    }

    /// Update only some specific fields of a `Scalar`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `Scalar`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            scalar: Some(SerializedComponentBatch::new(
                crate::components::Scalar::arrow_empty(),
                Self::descriptor_scalar(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [self
            .scalar
            .map(|scalar| scalar.partitioned(_lengths.into_iter()))
            .transpose()?];
        let indicator_column = None;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_scalar = self.scalar.as_ref().map(|b| b.array.len());
        let len = None.or(len_scalar).unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    /// The scalar value to log.
    #[inline]
    pub fn with_scalar(mut self, scalar: impl Into<crate::components::Scalar>) -> Self {
        self.scalar = try_serialize_field(Self::descriptor_scalar(), [scalar]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Scalar`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_scalar`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_scalar(
        mut self,
        scalar: impl IntoIterator<Item = impl Into<crate::components::Scalar>>,
    ) -> Self {
        self.scalar = try_serialize_field(Self::descriptor_scalar(), scalar);
        self
    }
}

impl ::re_byte_size::SizeBytes for Scalar {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.scalar.heap_size_bytes()
    }
}
