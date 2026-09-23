use core::ffi::c_void;
use core::fmt;
use core::ptr::{self, NonNull};
use std::ffi::CString;

use super::{Color, DualSenseTrigger, GCHapticsLocality};
use super::{
    GCDualSenseAdaptiveTriggerPositionalAmplitudes,
    GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
};
use crate::ffi;

pub struct ControllerHandle {
    raw: NonNull<c_void>,
}

unsafe impl Send for ControllerHandle {}
unsafe impl Sync for ControllerHandle {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualSenseTriggerEffect {
    Off,
    Feedback {
        start_position: f32,
        strength: f32,
    },
    Weapon {
        start_position: f32,
        end_position: f32,
        strength: f32,
    },
    Vibration {
        start_position: f32,
        amplitude: f32,
        frequency: f32,
    },
    SlopeFeedback {
        start_position: f32,
        end_position: f32,
        start_strength: f32,
        end_strength: f32,
    },
    FeedbackResistiveStrengths(GCDualSenseAdaptiveTriggerPositionalResistiveStrengths),
    VibrationAmplitudes {
        amplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes,
        frequency: f32,
    },
}

impl DualSenseTriggerEffect {
    const fn mode_parameters(self) -> Option<(i32, f32, f32, f32, f32)> {
        match self {
            Self::Off => Some((0, 0.0, 0.0, 0.0, 0.0)),
            Self::Feedback {
                start_position,
                strength,
            } => Some((1, start_position, 0.0, strength, 0.0)),
            Self::Weapon {
                start_position,
                end_position,
                strength,
            } => Some((2, start_position, end_position, strength, 0.0)),
            Self::Vibration {
                start_position,
                amplitude,
                frequency,
            } => Some((3, start_position, 0.0, amplitude, frequency)),
            Self::SlopeFeedback {
                start_position,
                end_position,
                start_strength,
                end_strength,
            } => Some((
                4,
                start_position,
                end_position,
                start_strength,
                end_strength,
            )),
            Self::FeedbackResistiveStrengths(_) | Self::VibrationAmplitudes { .. } => None,
        }
    }
}

impl ControllerHandle {
    unsafe fn from_retained(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    #[must_use]
    pub fn all() -> Vec<Self> {
        let mut capacity = 8;
        loop {
            let mut raw = vec![ptr::null_mut(); capacity];
            let total = unsafe { ffi::gc_controller_handles_copy(raw.as_mut_ptr(), capacity) };
            let handles = raw
                .into_iter()
                .take(total.min(capacity))
                .filter_map(|raw| unsafe { Self::from_retained(raw) })
                .collect::<Vec<_>>();
            if total <= capacity {
                return handles;
            }
            capacity = total;
        }
    }

    #[must_use]
    pub fn current() -> Option<Self> {
        unsafe { Self::from_retained(ffi::gc_controller_current_handle()) }
    }

    #[must_use]
    pub fn first() -> Option<Self> {
        Self::all().into_iter().next()
    }

    #[must_use]
    pub fn first_dualsense() -> Option<Self> {
        Self::all().into_iter().find(Self::is_dualsense)
    }

    #[must_use]
    pub fn vendor_name(&self) -> Option<String> {
        let raw = unsafe { ffi::gc_controller_handle_vendor_name(self.as_ptr()) };
        unsafe {
            doom_fish_utils::ffi_string::take_owned_cstring_c(raw, |p| ffi::gc_string_free(p))
        }
    }

    #[must_use]
    pub fn product_category(&self) -> String {
        let raw = unsafe { ffi::gc_controller_handle_product_category(self.as_ptr()) };
        unsafe {
            doom_fish_utils::ffi_string::take_owned_cstring_c(raw, |p| ffi::gc_string_free(p))
        }
        .unwrap_or_default()
    }

    #[must_use]
    pub fn player_index(&self) -> i32 {
        unsafe { ffi::gc_controller_handle_player_index(self.as_ptr()) }
    }

    #[must_use]
    pub fn set_player_index(&self, index: i32) -> bool {
        unsafe { ffi::gc_controller_handle_set_player_index(self.as_ptr(), index) }
    }

    #[must_use]
    pub fn set_light(&self, color: Color) -> bool {
        unsafe {
            ffi::gc_controller_handle_set_light(self.as_ptr(), color.red, color.green, color.blue)
        }
    }

    #[must_use]
    pub fn battery_level(&self) -> Option<f32> {
        let level = unsafe { ffi::gc_controller_handle_battery_level(self.as_ptr()) };
        (level >= 0.0).then_some(level)
    }

    #[must_use]
    pub fn rumble(
        &self,
        locality: GCHapticsLocality,
        intensity: f32,
        sharpness: f32,
        duration: f64,
    ) -> bool {
        let Ok(locality) = CString::new(locality.as_str()) else {
            return false;
        };
        unsafe {
            ffi::gc_controller_handle_rumble(
                self.as_ptr(),
                locality.as_ptr(),
                intensity,
                sharpness,
                duration,
            )
        }
    }

    #[must_use]
    pub fn is_dualsense(&self) -> bool {
        unsafe { ffi::gc_controller_handle_is_dualsense(self.as_ptr()) }
    }

    #[must_use]
    pub fn set_dualsense_trigger(
        &self,
        which: DualSenseTrigger,
        effect: DualSenseTriggerEffect,
    ) -> bool {
        if let Some((mode, start, end, strength, frequency)) = effect.mode_parameters() {
            return unsafe {
                ffi::gc_controller_handle_dualsense_set_trigger_mode(
                    self.as_ptr(),
                    which as i32,
                    mode,
                    start,
                    end,
                    strength,
                    frequency,
                )
            };
        }
        match effect {
            DualSenseTriggerEffect::FeedbackResistiveStrengths(strengths) => unsafe {
                ffi::gc_controller_handle_dualsense_set_trigger_feedback_resistive_strengths(
                    self.as_ptr(),
                    which as i32,
                    strengths.values.as_ptr(),
                    strengths.values.len(),
                )
            },
            DualSenseTriggerEffect::VibrationAmplitudes {
                amplitudes,
                frequency,
            } => unsafe {
                ffi::gc_controller_handle_dualsense_set_trigger_vibration_amplitudes(
                    self.as_ptr(),
                    which as i32,
                    amplitudes.values.as_ptr(),
                    amplitudes.values.len(),
                    frequency,
                )
            },
            _ => false,
        }
    }
}

impl Clone for ControllerHandle {
    fn clone(&self) -> Self {
        unsafe { ffi::gc_controller_handle_retain(self.as_ptr()) };
        Self { raw: self.raw }
    }
}

impl Drop for ControllerHandle {
    fn drop(&mut self) {
        unsafe { ffi::gc_controller_handle_release(self.as_ptr()) };
    }
}

impl PartialEq for ControllerHandle {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for ControllerHandle {}

impl fmt::Debug for ControllerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControllerHandle")
            .field("vendor_name", &self.vendor_name())
            .field("product_category", &self.product_category())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::DualSenseTriggerEffect;
    use crate::controller::{
        GCDualSenseAdaptiveTriggerPositionalAmplitudes,
        GCDualSenseAdaptiveTriggerPositionalResistiveStrengths,
    };

    #[test]
    fn trigger_effects_map_to_the_bridge_modes() {
        assert_eq!(
            DualSenseTriggerEffect::Off.mode_parameters(),
            Some((0, 0.0, 0.0, 0.0, 0.0))
        );
        assert_eq!(
            DualSenseTriggerEffect::Feedback {
                start_position: 0.25,
                strength: 0.5,
            }
            .mode_parameters(),
            Some((1, 0.25, 0.0, 0.5, 0.0))
        );
        assert_eq!(
            DualSenseTriggerEffect::Weapon {
                start_position: 0.25,
                end_position: 0.75,
                strength: 1.0,
            }
            .mode_parameters(),
            Some((2, 0.25, 0.75, 1.0, 0.0))
        );
        assert_eq!(
            DualSenseTriggerEffect::Vibration {
                start_position: 0.5,
                amplitude: 0.25,
                frequency: 0.125,
            }
            .mode_parameters(),
            Some((3, 0.5, 0.0, 0.25, 0.125))
        );
        assert_eq!(
            DualSenseTriggerEffect::SlopeFeedback {
                start_position: 0.125,
                end_position: 0.875,
                start_strength: 0.25,
                end_strength: 0.75,
            }
            .mode_parameters(),
            Some((4, 0.125, 0.875, 0.25, 0.75))
        );
        assert_eq!(
            DualSenseTriggerEffect::FeedbackResistiveStrengths(
                GCDualSenseAdaptiveTriggerPositionalResistiveStrengths::default()
            )
            .mode_parameters(),
            None
        );
        assert_eq!(
            DualSenseTriggerEffect::VibrationAmplitudes {
                amplitudes: GCDualSenseAdaptiveTriggerPositionalAmplitudes::default(),
                frequency: 0.5,
            }
            .mode_parameters(),
            None
        );
    }
}
