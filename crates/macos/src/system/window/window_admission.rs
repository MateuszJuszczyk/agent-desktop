use agent_desktop_core::AdapterError;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::system::window::cg_window::WindowRecord;

/// CoreGraphics reports every window layer, and window managers raise real application windows
/// above `kCGNormalWindowLevel` to place them under their own canvases. A raised window stays a
/// real application window when its owning process publishes it through accessibility as a
/// standard window, so the layer alone cannot decide. Menus, panels and status items stay
/// excluded because they are never `AXStandardWindow` elements of their owner.
pub(in crate::system) fn retain_ax_admitted(
    records: Vec<WindowRecord>,
    mut standard_window_ids: impl FnMut(i32) -> Result<FxHashSet<i64>, AdapterError>,
) -> Vec<WindowRecord> {
    let mut admitted_by_pid: FxHashMap<i32, FxHashSet<i64>> = FxHashMap::default();
    records
        .into_iter()
        .filter(|record| {
            record.is_normal_layer()
                || admitted_by_pid
                    .entry(record.pid)
                    .or_insert_with(|| probe(record.pid, &mut standard_window_ids))
                    .contains(&record.window_number)
        })
        .collect()
}

fn probe(
    pid: i32,
    standard_window_ids: &mut impl FnMut(i32) -> Result<FxHashSet<i64>, AdapterError>,
) -> FxHashSet<i64> {
    standard_window_ids(pid).unwrap_or_else(|error| {
        tracing::debug!(
            pid,
            error = %error.message,
            "raised-layer window admission probe failed"
        );
        FxHashSet::default()
    })
}

#[cfg(test)]
#[path = "window_admission_tests.rs"]
mod tests;
