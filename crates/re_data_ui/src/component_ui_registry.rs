use re_data_store::LatestAtQuery;
use re_log_types::{external::arrow2, EntityPath};
use re_query::ComponentWithInstances;
use re_types::external::arrow2::array::Utf8Array;
use re_viewer_context::{ComponentUiRegistry, UiVerbosity, ViewerContext};

use super::EntityDataUi;

pub fn create_component_ui_registry() -> ComponentUiRegistry {
    re_tracing::profile_function!();

    let mut registry = ComponentUiRegistry::new(Box::new(&fallback_component_ui));

    add_to_registry::<re_types::components::AnnotationContext>(&mut registry);
    add_to_registry::<re_types::components::ClassId>(&mut registry);
    add_to_registry::<re_types::components::Color>(&mut registry);
    add_to_registry::<re_types::components::PinholeProjection>(&mut registry);
    add_to_registry::<re_types::components::KeypointId>(&mut registry);
    add_to_registry::<re_types::components::LineStrip2D>(&mut registry);
    add_to_registry::<re_types::components::LineStrip3D>(&mut registry);
    add_to_registry::<re_types::components::Resolution>(&mut registry);
    add_to_registry::<re_types::components::Rotation3D>(&mut registry);
    add_to_registry::<re_types::components::Material>(&mut registry);
    add_to_registry::<re_types::components::MeshProperties>(&mut registry);
    add_to_registry::<re_types::components::TensorData>(&mut registry);
    add_to_registry::<re_types::components::Transform3D>(&mut registry);
    add_to_registry::<re_types::components::OutOfTreeTransform3D>(&mut registry);
    add_to_registry::<re_types::components::ViewCoordinates>(&mut registry);

    add_to_registry::<re_types::blueprint::components::IncludedQueries>(&mut registry);

    registry
}

/// Registers how to show a given component in the ui.
pub fn add_to_registry<C: EntityDataUi + re_types::Component>(registry: &mut ComponentUiRegistry) {
    registry.add(
        C::name(),
        Box::new(
            |ctx, ui, verbosity, query, entity_path, component, instance| match component
                .lookup::<C>(instance)
            {
                Ok(component) => {
                    component.entity_data_ui(ctx, ui, verbosity, entity_path, query);
                }
                Err(re_query::QueryError::ComponentNotFound) => {
                    ui.weak("(not found)");
                }
                Err(err) => {
                    re_log::warn_once!("Expected component {}, {}", C::name(), err);
                }
            },
        ),
    );
}

fn fallback_component_ui(
    _ctx: &ViewerContext<'_>,
    ui: &mut egui::Ui,
    verbosity: UiVerbosity,
    _query: &LatestAtQuery,
    _entity_path: &EntityPath,
    component: &ComponentWithInstances,
    instance_key: &re_types::components::InstanceKey,
) {
    // No special ui implementation - use a generic one:
    if let Some(value) = component.lookup_arrow(instance_key) {
        arrow_ui(ui, verbosity, &*value);
    } else {
        ui.weak("(null)");
    }
}

fn arrow_ui(ui: &mut egui::Ui, verbosity: UiVerbosity, array: &dyn arrow2::array::Array) {
    use re_types::SizeBytes as _;

    // Special-treat text.
    // Note: we match on the raw data here, so this works for any component containing text.
    if let Some(utf8) = array.as_any().downcast_ref::<Utf8Array<i32>>() {
        if utf8.len() == 1 {
            let string = utf8.value(0);
            text_ui(string, ui, verbosity);
            return;
        }
    }
    if let Some(utf8) = array.as_any().downcast_ref::<Utf8Array<i64>>() {
        if utf8.len() == 1 {
            let string = utf8.value(0);
            text_ui(string, ui, verbosity);
            return;
        }
    }

    let num_bytes = array.total_size_bytes();
    if num_bytes < 256 {
        // Print small items:
        let mut string = String::new();
        let display = arrow2::array::get_display(array, "null");
        if display(&mut string, 0).is_ok() {
            ui.label(string);
            return;
        }
    }

    // Fallback:
    ui.label(format!(
        "{} of {:?}",
        re_format::format_bytes(num_bytes as _),
        array.data_type()
    ));
}

fn text_ui(string: &str, ui: &mut egui::Ui, verbosity: UiVerbosity) {
    let font_id = egui::TextStyle::Monospace.resolve(ui.style());
    let color = ui.visuals().text_color();
    let wrap_width = ui.available_width();
    let mut layout_job =
        egui::text::LayoutJob::simple(string.to_owned(), font_id, color, wrap_width);

    let mut needs_scroll_area = false;

    match verbosity {
        UiVerbosity::Small => {
            // Elide
            layout_job.wrap.max_rows = 1;
            layout_job.wrap.break_anywhere = true;
        }
        UiVerbosity::Reduced => {
            layout_job.wrap.max_rows = 3;
        }
        UiVerbosity::LimitHeight => {
            let num_newlines = string.chars().filter(|&c| c == '\n').count();
            needs_scroll_area = 10 < num_newlines || 300 < string.len();
        }
        UiVerbosity::Full => {
            needs_scroll_area = false;
        }
    }

    if needs_scroll_area {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(layout_job);
        });
    } else {
        ui.label(layout_job);
    }
}
