use gamecontroller::prelude::*;

#[test]
fn device_snapshot_smoke() -> Result<(), GameControllerError> {
    let _ = keyboard_snapshot()?;
    let _ = mouse_snapshot()?;
    let event_view_controller = event_view_controller_snapshot()?;
    let _ = event_view_controller.controller_user_interaction_enabled;
    let _ = connected_racing_wheels()?;
    Ok(())
}
