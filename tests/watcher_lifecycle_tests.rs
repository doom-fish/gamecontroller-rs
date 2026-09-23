use std::sync::{Arc, Barrier};
use std::thread;

use gamecontroller::prelude::*;

#[test]
fn dropping_watchers_releases_their_callbacks() {
    let probe = Arc::new(());
    let capture = |probe: &Arc<()>| {
        let probe = Arc::clone(probe);
        move |_: bool| {
            let _ = &probe;
        }
    };
    let watchers = vec![
        watch_connections(capture(&probe)),
        watch_current_controller(capture(&probe)),
        watch_keyboard_connections(capture(&probe)),
        watch_mouse_connections(capture(&probe)),
        watch_mouse_current(capture(&probe)),
        watch_racing_wheel_connections(capture(&probe)),
    ];
    let customizations = watch_controller_customizations({
        let probe = Arc::clone(&probe);
        move || {
            let _ = &probe;
        }
    });
    assert_eq!(Arc::strong_count(&probe), 8);

    drop(watchers);
    drop(customizations);
    assert_eq!(Arc::strong_count(&probe), 1);
}

#[test]
fn watchers_can_be_registered_and_dropped_from_many_threads() {
    const THREADS: usize = 8;
    const ROUNDS: usize = 200;

    let probe = Arc::new(());
    let barrier = Arc::new(Barrier::new(THREADS));
    let workers = (0..THREADS)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            let probe = Arc::clone(&probe);
            thread::spawn(move || {
                barrier.wait();
                for _ in 0..ROUNDS {
                    let connection = {
                        let probe = Arc::clone(&probe);
                        watch_connections(move |_| {
                            let _ = &probe;
                        })
                    };
                    let customizations = {
                        let probe = Arc::clone(&probe);
                        watch_controller_customizations(move || {
                            let _ = &probe;
                        })
                    };
                    drop(connection);
                    drop(customizations);
                }
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().expect("watcher thread");
    }
    assert_eq!(Arc::strong_count(&probe), 1);
}

#[test]
#[ignore = "starts real wireless controller discovery, which can pair a nearby controller that is in pairing mode"]
fn wireless_discovery_can_be_started_and_stopped_from_many_threads() {
    let workers = (0..4)
        .map(|index| {
            thread::spawn(move || {
                for _ in 0..50 {
                    if index % 2 == 0 {
                        start_wireless_controller_discovery_with_callback(|| {});
                    } else {
                        stop_wireless_controller_discovery();
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().expect("discovery thread");
    }
    stop_wireless_controller_discovery();
}
