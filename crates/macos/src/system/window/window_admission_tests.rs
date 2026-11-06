use super::*;
use agent_desktop_core::Rect;

#[test]
fn raised_window_is_admitted_when_accessibility_reports_its_window_number() {
    let admitted = retain_ax_admitted(vec![record(10, 13458, 2), record(10, 13460, 0)], |_| {
        Ok(FxHashSet::from_iter([13458]))
    });

    assert_eq!(
        admitted
            .iter()
            .map(|record| record.window_number)
            .collect::<Vec<_>>(),
        [13458, 13460]
    );
}

#[test]
fn raised_overlay_windows_of_the_same_owner_stay_excluded() {
    let admitted = retain_ax_admitted(vec![record(10, 7, 2), record(10, 15089, 101)], |_| {
        Ok(FxHashSet::from_iter([7]))
    });

    assert_eq!(
        admitted
            .iter()
            .map(|record| record.window_number)
            .collect::<Vec<_>>(),
        [7]
    );
}

#[test]
fn normal_layer_windows_never_need_an_accessibility_probe() {
    let mut probed = Vec::new();
    let admitted = retain_ax_admitted(vec![record(10, 7, 0), record(20, 8, 0)], |pid| {
        probed.push(pid);
        Ok(FxHashSet::default())
    });

    assert_eq!(admitted.len(), 2);
    assert!(probed.is_empty());
}

#[test]
fn failed_probe_drops_only_the_raised_windows_of_that_owner() {
    let admitted = retain_ax_admitted(
        vec![record(10, 7, 2), record(10, 8, 0), record(20, 9, 2)],
        |pid| match pid {
            20 => Ok(FxHashSet::from_iter([9])),
            _ => Err(AdapterError::timeout("accessibility unavailable")),
        },
    );

    assert_eq!(
        admitted
            .iter()
            .map(|record| record.window_number)
            .collect::<Vec<_>>(),
        [8, 9]
    );
}

#[test]
fn every_owner_is_probed_once_for_all_of_its_raised_windows() {
    let mut probed = Vec::new();
    let admitted = retain_ax_admitted(
        vec![record(10, 7, 2), record(10, 8, 3), record(10, 9, 0)],
        |pid| {
            probed.push(pid);
            Ok(FxHashSet::from_iter([7, 8]))
        },
    );

    assert_eq!(admitted.len(), 3);
    assert_eq!(probed, [10]);
}

fn record(pid: i32, window_number: i64, layer: i64) -> WindowRecord {
    WindowRecord {
        app_name: "Fixture".into(),
        pid,
        title: Some(format!("Window {window_number}")),
        window_number,
        layer,
        bounds: Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        },
        visible: true,
        process_instance: Some(format!("instance-{pid}")),
    }
}
