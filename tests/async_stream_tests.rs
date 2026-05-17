//! Integration tests for the async stream module.
//!
//! These tests run without real hardware. They verify that:
//!   - Each stream can be subscribed to (construction + `FFI` call succeeds)
//!   - `buffered_count()` returns 0 when no events have arrived
//!   - `try_next()` returns `None` when nothing is buffered
//!   - Dropping the stream handle (`SubscriptionHandle`) does not crash

#[cfg(feature = "async")]
mod async_stream_tests {
    use gamecontroller::async_api::{
        ControllerConnectionStream, GamepadValueStream, KeyboardKeyStream, MicroGamepadValueStream,
        MotionStream, MouseInputStream,
    };
    use std::time::Duration;

    fn wait_briefly() {
        std::thread::sleep(Duration::from_millis(20));
    }

    #[test]
    fn controller_connection_stream_subscribe_and_drop() {
        let stream = ControllerConnectionStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
        // drop() calls SubscriptionHandle::drop → should not crash
    }

    #[test]
    fn gamepad_value_stream_subscribe_and_drop() {
        let stream = GamepadValueStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn keyboard_key_stream_subscribe_and_drop() {
        let stream = KeyboardKeyStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn mouse_input_stream_subscribe_and_drop() {
        let stream = MouseInputStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn motion_stream_subscribe_and_drop() {
        let stream = MotionStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn micro_gamepad_value_stream_subscribe_and_drop() {
        let stream = MicroGamepadValueStream::subscribe(8);
        wait_briefly();
        assert_eq!(stream.buffered_count(), 0);
        assert!(stream.try_next().is_none());
    }

    #[test]
    fn multiple_streams_can_coexist() {
        let conn = ControllerConnectionStream::subscribe(4);
        let gamepad = GamepadValueStream::subscribe(4);
        let keyboard = KeyboardKeyStream::subscribe(4);
        let mouse = MouseInputStream::subscribe(4);
        let motion = MotionStream::subscribe(4);
        let micro = MicroGamepadValueStream::subscribe(4);
        wait_briefly();
        assert_eq!(conn.buffered_count(), 0);
        assert_eq!(gamepad.buffered_count(), 0);
        assert_eq!(keyboard.buffered_count(), 0);
        assert_eq!(mouse.buffered_count(), 0);
        assert_eq!(motion.buffered_count(), 0);
        assert_eq!(micro.buffered_count(), 0);
    }
}
