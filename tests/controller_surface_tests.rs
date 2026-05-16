use gamecontroller::prelude::*;

#[test]
fn controller_surface_smoke() -> Result<(), GameControllerError> {
    let controllers = connected_controller_details()?;
    if let Some(controller) = controllers.first() {
        let _ = &controller.directional_gamepad;
        let _ = &controller.dual_shock;
        let _ = &controller.dual_sense;
        let _ = &controller.xbox;
        let _ = &controller.battery;
        let _ = &controller.light;
        let _ = &controller.haptics;
        let _ = &controller.input;
    }

    let _ = current_controller_input_snapshot()?;
    let _ = first_controller_battery()?;
    let _ = first_controller_light()?;
    let _ = first_controller_haptics()?;
    let _ = set_first_controller_light_color(Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    });
    Ok(())
}
