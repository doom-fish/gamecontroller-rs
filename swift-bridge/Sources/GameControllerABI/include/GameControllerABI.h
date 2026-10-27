#ifndef GAME_CONTROLLER_ABI_H
#define GAME_CONTROLLER_ABI_H
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
typedef struct { const uint8_t *data; size_t len; } GCStringRaw;
typedef struct { const void *data; size_t len; } GCSliceRaw;
typedef struct { const void *data; size_t len; void *owner; } GCSnapshotRaw;
typedef struct GCVector3Raw GCVector3Raw;
typedef struct GCQuaternionRaw GCQuaternionRaw;
typedef struct GCColorRaw GCColorRaw;
typedef struct GCButtonInputStateRaw GCButtonInputStateRaw;
typedef struct GCAxisInputStateRaw GCAxisInputStateRaw;
typedef struct GCDirectionPadInputStateRaw GCDirectionPadInputStateRaw;
typedef struct GCTouchpadDetailsRaw GCTouchpadDetailsRaw;
typedef struct GCGamepadDetailsRaw GCGamepadDetailsRaw;
typedef struct GCMicroGamepadDetailsRaw GCMicroGamepadDetailsRaw;
typedef struct GCDirectionalGamepadDetailsRaw GCDirectionalGamepadDetailsRaw;
typedef struct GCExtendedGamepadDetailsRaw GCExtendedGamepadDetailsRaw;
typedef struct GCDualShockGamepadDetailsRaw GCDualShockGamepadDetailsRaw;
typedef struct GCDualSenseAdaptiveTriggerStateRaw GCDualSenseAdaptiveTriggerStateRaw;
typedef struct GCDualSenseGamepadDetailsRaw GCDualSenseGamepadDetailsRaw;
typedef struct GCXboxGamepadDetailsRaw GCXboxGamepadDetailsRaw;
typedef struct GCBatteryInfoRaw GCBatteryInfoRaw;
typedef struct GCDeviceLightDetailsRaw GCDeviceLightDetailsRaw;
typedef struct GCDeviceHapticsDetailsRaw GCDeviceHapticsDetailsRaw;
typedef struct GCDeviceDetailsRaw GCDeviceDetailsRaw;
typedef struct GCConnectedDevicesSnapshotRaw GCConnectedDevicesSnapshotRaw;
typedef struct GCMotionDetailsRaw GCMotionDetailsRaw;
typedef struct GCNamedButtonInputStateRaw GCNamedButtonInputStateRaw;
typedef struct GCNamedAxisInputStateRaw GCNamedAxisInputStateRaw;
typedef struct GCNamedDirectionPadStateRaw GCNamedDirectionPadStateRaw;
typedef struct GCNamedTouchpadStateRaw GCNamedTouchpadStateRaw;
typedef struct GCPhysicalInputProfileDetailsRaw GCPhysicalInputProfileDetailsRaw;
typedef struct GCAxisElementStateRaw GCAxisElementStateRaw;
typedef struct GCSwitchInputStateRaw GCSwitchInputStateRaw;
typedef struct GCNamedButtonElementStateRaw GCNamedButtonElementStateRaw;
typedef struct GCNamedAxisElementStateRaw GCNamedAxisElementStateRaw;
typedef struct GCNamedSwitchElementStateRaw GCNamedSwitchElementStateRaw;
typedef struct GCNamedDirectionPadElementStateRaw GCNamedDirectionPadElementStateRaw;
typedef struct GCInputElementMetadataRaw GCInputElementMetadataRaw;
typedef struct GCControllerElementDetailsRaw GCControllerElementDetailsRaw;
typedef struct GCNamedControllerElementDetailsRaw GCNamedControllerElementDetailsRaw;
typedef struct GCPhysicalInputSourceDetailsRaw GCPhysicalInputSourceDetailsRaw;
typedef struct GCPhysicalInputExtentsDetailsRaw GCPhysicalInputExtentsDetailsRaw;
typedef struct GCLinearInputDetailsRaw GCLinearInputDetailsRaw;
typedef struct GCPressedStateInputDetailsRaw GCPressedStateInputDetailsRaw;
typedef struct GCTouchedStateInputDetailsRaw GCTouchedStateInputDetailsRaw;
typedef struct GCRelativeInputDetailsRaw GCRelativeInputDetailsRaw;
typedef struct GCAxisInputDetailsRaw GCAxisInputDetailsRaw;
typedef struct GCAxis2DInputDetailsRaw GCAxis2DInputDetailsRaw;
typedef struct GCSwitchPositionInputDetailsRaw GCSwitchPositionInputDetailsRaw;
typedef struct GCLinearPressedInputDetailsRaw GCLinearPressedInputDetailsRaw;
typedef struct GCButtonElementDetailsRaw GCButtonElementDetailsRaw;
typedef struct GCAxisElementDetailsRaw GCAxisElementDetailsRaw;
typedef struct GCSwitchElementDetailsRaw GCSwitchElementDetailsRaw;
typedef struct GCDirectionPadElementDetailsRaw GCDirectionPadElementDetailsRaw;
typedef struct GCPhysicalInputElementSnapshotRaw GCPhysicalInputElementSnapshotRaw;
typedef struct GCDevicePhysicalInputStateDiffDetailsRaw GCDevicePhysicalInputStateDiffDetailsRaw;
typedef struct GCControllerInputStateDetailsRaw GCControllerInputStateDetailsRaw;
typedef struct GCControllerLiveInputDetailsRaw GCControllerLiveInputDetailsRaw;
typedef struct GCDevicePhysicalInputSourceDetailsRaw GCDevicePhysicalInputSourceDetailsRaw;
typedef struct GCControllerDetailsRaw GCControllerDetailsRaw;
typedef struct GCKeyboardSnapshotRaw GCKeyboardSnapshotRaw;
typedef struct GCMouseSnapshotRaw GCMouseSnapshotRaw;
typedef struct GCEventViewControllerDetailsRaw GCEventViewControllerDetailsRaw;
typedef struct GCSteeringWheelDetailsRaw GCSteeringWheelDetailsRaw;
typedef struct GCGearShifterDetailsRaw GCGearShifterDetailsRaw;
typedef struct GCRacingWheelInputDetailsRaw GCRacingWheelInputDetailsRaw;
typedef struct GCRacingWheelDetailsRaw GCRacingWheelDetailsRaw;
typedef struct GCPoint2Raw GCPoint2Raw;
struct GCVector3Raw {
    double x;
    double y;
    double z;
};
struct GCQuaternionRaw {
    double x;
    double y;
    double z;
    double w;
};
struct GCColorRaw {
    float red;
    float green;
    float blue;
};
struct GCButtonInputStateRaw {
    float value;
    uint8_t pressed;
    uint8_t touched;
};
struct GCAxisInputStateRaw {
    float value;
};
struct GCDirectionPadInputStateRaw {
    float x;
    float y;
    GCButtonInputStateRaw up;
    GCButtonInputStateRaw down;
    GCButtonInputStateRaw left;
    GCButtonInputStateRaw right;
};
struct GCTouchpadDetailsRaw {
    GCButtonInputStateRaw button;
    GCDirectionPadInputStateRaw touch_surface;
    int32_t touch_state;
    uint8_t reports_absolute_touch_surface_values;
};
struct GCGamepadDetailsRaw {
    GCDirectionPadInputStateRaw dpad;
    GCButtonInputStateRaw button_a;
    GCButtonInputStateRaw button_b;
    GCButtonInputStateRaw button_x;
    GCButtonInputStateRaw button_y;
    GCButtonInputStateRaw left_shoulder;
    GCButtonInputStateRaw right_shoulder;
};
struct GCMicroGamepadDetailsRaw {
    GCDirectionPadInputStateRaw dpad;
    GCButtonInputStateRaw button_a;
    GCButtonInputStateRaw button_x;
    const GCButtonInputStateRaw * button_menu;
    uint8_t reports_absolute_dpad_values;
    uint8_t allows_rotation;
};
struct GCDirectionalGamepadDetailsRaw {
    GCDirectionPadInputStateRaw dpad;
    GCButtonInputStateRaw button_a;
    GCButtonInputStateRaw button_x;
    const GCButtonInputStateRaw * button_menu;
    uint8_t reports_absolute_dpad_values;
    uint8_t allows_rotation;
};
struct GCExtendedGamepadDetailsRaw {
    GCDirectionPadInputStateRaw dpad;
    GCButtonInputStateRaw button_a;
    GCButtonInputStateRaw button_b;
    GCButtonInputStateRaw button_x;
    GCButtonInputStateRaw button_y;
    const GCButtonInputStateRaw * button_menu;
    const GCButtonInputStateRaw * button_options;
    const GCButtonInputStateRaw * button_home;
    GCDirectionPadInputStateRaw left_thumbstick;
    GCDirectionPadInputStateRaw right_thumbstick;
    GCButtonInputStateRaw left_shoulder;
    GCButtonInputStateRaw right_shoulder;
    GCButtonInputStateRaw left_trigger;
    GCButtonInputStateRaw right_trigger;
    const GCButtonInputStateRaw * left_thumbstick_button;
    const GCButtonInputStateRaw * right_thumbstick_button;
};
struct GCDualShockGamepadDetailsRaw {
    GCButtonInputStateRaw touchpad_button;
    GCDirectionPadInputStateRaw touchpad_primary;
    GCDirectionPadInputStateRaw touchpad_secondary;
};
struct GCDualSenseAdaptiveTriggerStateRaw {
    float value;
    uint8_t pressed;
    uint8_t touched;
    int32_t mode;
    int32_t status;
    float arm_position;
};
struct GCDualSenseGamepadDetailsRaw {
    GCButtonInputStateRaw touchpad_button;
    GCDirectionPadInputStateRaw touchpad_primary;
    GCDirectionPadInputStateRaw touchpad_secondary;
    GCDualSenseAdaptiveTriggerStateRaw left_trigger;
    GCDualSenseAdaptiveTriggerStateRaw right_trigger;
};
struct GCXboxGamepadDetailsRaw {
    const GCButtonInputStateRaw * paddle_button1;
    const GCButtonInputStateRaw * paddle_button2;
    const GCButtonInputStateRaw * paddle_button3;
    const GCButtonInputStateRaw * paddle_button4;
    const GCButtonInputStateRaw * button_share;
};
struct GCBatteryInfoRaw {
    float level;
    int32_t state;
};
struct GCDeviceLightDetailsRaw {
    GCColorRaw color;
};
struct GCDeviceHapticsDetailsRaw {
    GCSliceRaw supported_localities;
};
struct GCDeviceDetailsRaw {
    GCStringRaw vendor_name;
    GCStringRaw product_category;
    GCStringRaw handler_queue_label;
};
struct GCConnectedDevicesSnapshotRaw {
    GCSliceRaw controllers;
    const GCDeviceDetailsRaw * keyboard;
    const GCDeviceDetailsRaw * mouse;
    GCSliceRaw racing_wheels;
};
struct GCMotionDetailsRaw {
    uint8_t sensors_require_manual_activation;
    uint8_t sensors_active;
    uint8_t has_gravity_and_user_acceleration;
    GCVector3Raw gravity;
    GCVector3Raw user_acceleration;
    GCVector3Raw acceleration;
    uint8_t has_attitude;
    uint8_t has_rotation_rate;
    GCQuaternionRaw attitude;
    GCVector3Raw rotation_rate;
};
struct GCNamedButtonInputStateRaw {
    GCStringRaw alias;
    GCButtonInputStateRaw value;
};
struct GCNamedAxisInputStateRaw {
    GCStringRaw alias;
    GCAxisInputStateRaw value;
};
struct GCNamedDirectionPadStateRaw {
    GCStringRaw alias;
    GCDirectionPadInputStateRaw value;
};
struct GCNamedTouchpadStateRaw {
    GCStringRaw alias;
    GCTouchpadDetailsRaw value;
};
struct GCPhysicalInputProfileDetailsRaw {
    double last_event_timestamp;
    uint8_t has_remapped_elements;
    GCSliceRaw element_aliases;
    GCSliceRaw button_aliases;
    GCSliceRaw axis_aliases;
    GCSliceRaw dpad_aliases;
    GCSliceRaw touchpad_aliases;
    GCSliceRaw buttons;
    GCSliceRaw axes;
    GCSliceRaw dpads;
    GCSliceRaw touchpads;
};
struct GCAxisElementStateRaw {
    const float * absolute_value;
    float relative_delta;
};
struct GCSwitchInputStateRaw {
    int32_t position;
    int32_t position_lower_bound;
    int32_t position_count;
    uint8_t sequential;
    uint8_t can_wrap;
};
struct GCNamedButtonElementStateRaw {
    GCStringRaw primary_alias;
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
    GCButtonInputStateRaw value;
};
struct GCNamedAxisElementStateRaw {
    GCStringRaw primary_alias;
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
    GCAxisElementStateRaw value;
};
struct GCNamedSwitchElementStateRaw {
    GCStringRaw primary_alias;
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
    GCSwitchInputStateRaw value;
};
struct GCNamedDirectionPadElementStateRaw {
    GCStringRaw primary_alias;
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
    GCDirectionPadInputStateRaw value;
};
struct GCInputElementMetadataRaw {
    GCStringRaw primary_alias;
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
};
struct GCControllerElementDetailsRaw {
    uint8_t analog;
    GCSliceRaw collection_path;
    uint8_t is_bound_to_system_gesture;
    int32_t preferred_system_gesture_state;
    const GCStringRaw * sf_symbols_name;
    const GCStringRaw * localized_name;
    const GCStringRaw * unmapped_sf_symbols_name;
    const GCStringRaw * unmapped_localized_name;
};
struct GCNamedControllerElementDetailsRaw {
    GCStringRaw name;
    GCControllerElementDetailsRaw value;
};
struct GCPhysicalInputSourceDetailsRaw {
    GCSliceRaw element_aliases;
    const GCStringRaw * element_localized_name;
    const GCStringRaw * sf_symbols_name;
    uint8_t direction;
};
struct GCPhysicalInputExtentsDetailsRaw {
    double scaled_value;
    double minimum_value;
    double maximum_value;
};
struct GCLinearInputDetailsRaw {
    float value;
    uint8_t analog;
    uint8_t can_wrap;
    double last_value_timestamp;
    double last_value_latency;
    const GCPhysicalInputExtentsDetailsRaw * physical_extents;
    GCSliceRaw sources;
};
struct GCPressedStateInputDetailsRaw {
    uint8_t pressed;
    double last_pressed_state_timestamp;
    double last_pressed_state_latency;
    GCSliceRaw sources;
};
struct GCTouchedStateInputDetailsRaw {
    uint8_t touched;
    double last_touched_state_timestamp;
    double last_touched_state_latency;
    GCSliceRaw sources;
};
struct GCRelativeInputDetailsRaw {
    float delta;
    uint8_t analog;
    double last_delta_timestamp;
    double last_delta_latency;
    GCSliceRaw sources;
};
struct GCAxisInputDetailsRaw {
    float value;
    uint8_t analog;
    uint8_t can_wrap;
    double last_value_timestamp;
    double last_value_latency;
    GCSliceRaw sources;
};
struct GCPoint2Raw {
    float x;
    float y;
};
struct GCAxis2DInputDetailsRaw {
    GCPoint2Raw value;
    uint8_t analog;
    uint8_t can_wrap;
    double last_value_timestamp;
    double last_value_latency;
    GCSliceRaw sources;
};
struct GCSwitchPositionInputDetailsRaw {
    int32_t position;
    int32_t position_lower_bound;
    int32_t position_count;
    uint8_t sequential;
    uint8_t can_wrap;
    GCSliceRaw sources;
};
struct GCLinearPressedInputDetailsRaw {
    GCLinearInputDetailsRaw linear_input;
    GCPressedStateInputDetailsRaw pressed_state;
};
struct GCButtonElementDetailsRaw {
    GCInputElementMetadataRaw metadata;
    GCLinearInputDetailsRaw linear_input;
    GCPressedStateInputDetailsRaw pressed_state;
    const GCTouchedStateInputDetailsRaw * touched_state;
    const GCLinearInputDetailsRaw * force_input;
};
struct GCAxisElementDetailsRaw {
    GCInputElementMetadataRaw metadata;
    const GCAxisInputDetailsRaw * absolute_input;
    GCRelativeInputDetailsRaw relative_input;
};
struct GCSwitchElementDetailsRaw {
    GCInputElementMetadataRaw metadata;
    GCSwitchPositionInputDetailsRaw position_input;
};
struct GCDirectionPadElementDetailsRaw {
    GCInputElementMetadataRaw metadata;
    const GCAxis2DInputDetailsRaw * xy_axes;
    GCAxisInputDetailsRaw x_axis;
    GCAxisInputDetailsRaw y_axis;
    GCLinearPressedInputDetailsRaw up;
    GCLinearPressedInputDetailsRaw down;
    GCLinearPressedInputDetailsRaw left;
    GCLinearPressedInputDetailsRaw right;
};
struct GCPhysicalInputElementSnapshotRaw {
    GCSliceRaw buttons;
    GCSliceRaw axes;
    GCSliceRaw switches;
    GCSliceRaw dpads;
};
struct GCDevicePhysicalInputStateDiffDetailsRaw {
    uint8_t changed_elements_known;
    GCSliceRaw changed_aliases;
    GCSliceRaw changed_elements;
};
struct GCControllerInputStateDetailsRaw {
    double last_event_timestamp;
    double last_event_latency;
    size_t element_count;
    size_t button_count;
    size_t axis_count;
    size_t switch_count;
    size_t dpad_count;
    GCSliceRaw buttons;
    GCSliceRaw axes;
    GCSliceRaw switches;
    GCSliceRaw dpads;
    uint8_t changed_aliases_known;
    GCSliceRaw changed_aliases;
};
struct GCControllerLiveInputDetailsRaw {
    intptr_t input_state_queue_depth;
    GCControllerInputStateDetailsRaw live;
    const GCControllerInputStateDetailsRaw * unmapped;
    const GCControllerInputStateDetailsRaw * next;
};
struct GCDevicePhysicalInputSourceDetailsRaw {
    intptr_t input_state_queue_depth;
    GCControllerInputStateDetailsRaw live;
    GCControllerInputStateDetailsRaw capture;
    const GCControllerInputStateDetailsRaw * unmapped;
    const GCControllerInputStateDetailsRaw * next;
    const GCDevicePhysicalInputStateDiffDetailsRaw * next_diff;
};
struct GCControllerDetailsRaw {
    GCStringRaw vendor_name;
    GCStringRaw product_category;
    int32_t player_index;
    uint8_t is_attached_to_device;
    uint8_t is_current;
    uint8_t supports_background_events;
    uint8_t has_live_input;
    const GCGamepadDetailsRaw * gamepad;
    const GCMicroGamepadDetailsRaw * micro_gamepad;
    const GCDirectionalGamepadDetailsRaw * directional_gamepad;
    const GCExtendedGamepadDetailsRaw * extended_gamepad;
    const GCDualShockGamepadDetailsRaw * dual_shock;
    const GCDualSenseGamepadDetailsRaw * dual_sense;
    const GCXboxGamepadDetailsRaw * xbox;
    const GCBatteryInfoRaw * battery;
    const GCMotionDetailsRaw * motion;
    uint8_t has_light;
    const GCDeviceLightDetailsRaw * light;
    uint8_t has_haptics;
    const GCDeviceHapticsDetailsRaw * haptics;
    const GCPhysicalInputProfileDetailsRaw * physical_input;
    const GCControllerLiveInputDetailsRaw * input;
};
struct GCKeyboardSnapshotRaw {
    GCStringRaw vendor_name;
    GCStringRaw product_category;
    uint8_t any_key_pressed;
    GCSliceRaw pressed_aliases;
    GCSliceRaw pressed_keys;
    GCPhysicalInputProfileDetailsRaw physical_input;
};
struct GCMouseSnapshotRaw {
    GCStringRaw vendor_name;
    GCStringRaw product_category;
    uint8_t is_current;
    size_t known_mouse_count;
    GCDirectionPadInputStateRaw scroll;
    GCButtonInputStateRaw left_button;
    const GCButtonInputStateRaw * right_button;
    const GCButtonInputStateRaw * middle_button;
    GCSliceRaw auxiliary_buttons;
    GCPhysicalInputProfileDetailsRaw physical_input;
};
struct GCEventViewControllerDetailsRaw {
    uint8_t controller_user_interaction_enabled;
};
struct GCSteeringWheelDetailsRaw {
    float maximum_degrees_of_rotation;
    const float * absolute_value;
    float relative_delta;
};
struct GCGearShifterDetailsRaw {
    GCSliceRaw aliases;
    const GCStringRaw * localized_name;
    const GCStringRaw * sf_symbols_name;
    const int32_t * pattern_position;
    const int32_t * pattern_lower_bound;
    const int32_t * pattern_count;
    const uint8_t * pattern_sequential;
    const uint8_t * pattern_can_wrap;
    const float * sequential_delta;
};
struct GCRacingWheelInputDetailsRaw {
    GCSteeringWheelDetailsRaw wheel;
    const GCButtonInputStateRaw * accelerator_pedal;
    const GCButtonInputStateRaw * brake_pedal;
    const GCButtonInputStateRaw * clutch_pedal;
    const GCGearShifterDetailsRaw * shifter;
};
struct GCRacingWheelDetailsRaw {
    GCStringRaw vendor_name;
    GCStringRaw product_category;
    uint8_t is_acquired;
    uint8_t is_snapshot;
    const GCRacingWheelInputDetailsRaw * wheel_input;
};
typedef struct {
    char * vendor_name;
    char * product_category;
    int32_t player_index;
    bool is_attached_to_device;
    bool has_extended_gamepad;
    float button_a;
    float button_b;
    float button_x;
    float button_y;
    float left_shoulder;
    float right_shoulder;
    float left_trigger;
    float right_trigger;
    float menu_button;
    float options_button;
    float home_button;
    float left_thumbstick_x;
    float left_thumbstick_y;
    float right_thumbstick_x;
    float right_thumbstick_y;
    float dpad_up;
    float dpad_down;
    float dpad_left;
    float dpad_right;
} GCControllerInfoRaw;
typedef struct {
    bool has_motion;
    bool has_haptics;
    bool has_light;
    bool has_battery;
    float battery_level;
    int32_t battery_state;
    double gravity_x;
    double gravity_y;
    double gravity_z;
    double user_acceleration_x;
    double user_acceleration_y;
    double user_acceleration_z;
} GCExtraInfoRaw;
typedef struct {
    int64_t keycode;
    bool pressed;
    float value;
} GCKeyEventRaw;
typedef struct {
    float delta_x;
    float delta_y;
    bool pressed;
    float value;
} GCMouseEventRaw;
typedef struct {
    double gravity_x;
    double gravity_y;
    double gravity_z;
    double user_acceleration_x;
    double user_acceleration_y;
    double user_acceleration_z;
    double attitude_x;
    double attitude_y;
    double attitude_z;
    double attitude_w;
    double rotation_rate_x;
    double rotation_rate_y;
    double rotation_rate_z;
} GCMotionEventRaw;
typedef struct {
    float button_a;
    float button_x;
    float dpad_x;
    float dpad_y;
} GCMicroGamepadEventRaw;
void gc_snapshot_free(void *owner);
#endif
