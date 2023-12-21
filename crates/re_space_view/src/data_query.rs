use re_data_store::{EntityProperties, EntityPropertyMap};
use re_viewer_context::{DataQueryResult, EntitiesPerSystem, StoreContext};

pub struct EntityOverrideContext {
    pub root: EntityProperties,
    pub individual: EntityPropertyMap,
    pub group: EntityPropertyMap,
}

/// Trait for resolving properties needed by most implementations of [`DataQuery`]
///
/// The `SpaceViewBlueprint` is the only thing that likely implements this today
/// but we use a trait here so we don't have to pick up a full dependency on `re_viewport`.
pub trait PropertyResolver {
    fn update_overrides(&self, ctx: &StoreContext<'_>, query_result: &mut DataQueryResult);
}

/// The common trait implemented for data queries
///
/// Both interfaces return [`re_viewer_context::DataResult`]s, which are self-contained description of the data
/// to be added to a `SpaceView` including both the [`re_log_types::EntityPath`] and context for any overrides.
pub trait DataQuery {
    /// Execute a full query, returning a `DataResultTree` containing all results.
    ///
    /// `auto_properties` is a map containing any heuristic-derived auto properties for the given `SpaceView`.
    ///
    /// This is used when building up the contents for a `SpaceView`.
    fn execute_query(
        &self,
        ctx: &StoreContext<'_>,
        entities_per_system: &EntitiesPerSystem,
    ) -> DataQueryResult;
}
