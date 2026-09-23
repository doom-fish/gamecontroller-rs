use gamecontroller::prelude::*;

#[test]
fn controller_handles_follow_the_connected_controllers() {
    let handles = ControllerHandle::all();
    assert_eq!(handles.len(), connected_controllers().len());
    for handle in &handles {
        let clone = handle.clone();
        assert_eq!(&clone, handle);
        drop(clone);
        let _ = handle.vendor_name();
        let _ = handle.product_category();
        assert!((-1..=3).contains(&handle.player_index()));
        if let Some(level) = handle.battery_level() {
            assert!((0.0..=1.0).contains(&level));
        }
        if !handle.is_dualsense() {
            assert!(
                !handle.set_dualsense_trigger(DualSenseTrigger::Left, DualSenseTriggerEffect::Off)
            );
        }
        assert!(!handle.set_player_index(4));
        assert!(!handle.set_player_index(-2));
    }
    if let Some(current) = ControllerHandle::current() {
        assert!(handles.contains(&current));
    }
}

#[test]
fn first_controller_helpers_report_failure_without_controllers() {
    if !ControllerHandle::all().is_empty() {
        return;
    }
    assert_eq!(ControllerHandle::first(), None);
    assert_eq!(ControllerHandle::first_dualsense(), None);
    assert!(!set_first_controller_light(0.0, 0.0, 0.0));
    assert!(!set_first_controller_player_index(0));
    assert!(first_controller_battery_level() < 0.0);
    assert!(!rumble_first_controller(0.0, 0.0, 0.1));
    assert!(!dualsense_is_connected());
    assert!(!dualsense_trigger_off(DualSenseTrigger::Left));
    assert!(!dualsense_trigger_weapon(
        DualSenseTrigger::Right,
        0.2,
        0.6,
        1.0
    ));
}
