use ::dbus::blocking::Connection;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{error::Error, thread};

mod dbus;
mod environment;
mod status_notifier_watcher;
mod sway_ipc;
mod systemd;

fn register_signal_handler(flag: Arc<AtomicBool>) -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("\nReceived signal {:?}", sig);
            flag.store(false, Ordering::SeqCst);
            break;
        }
    });
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let conn = Arc::new(Mutex::new(Connection::new_session()?));
    let dbus_handle: JoinHandle<()>;
    let ipc_handle: JoinHandle<()>;

    {
        let conn_lock = conn.lock().unwrap();

        // Check for an already running 'sway-session.target'
        if systemd::sway_session_is_active(&conn_lock)? {
            println!("Sway session is already active");
            return Ok(());
        }
    }

    register_signal_handler(running.clone())?;

    dbus_handle = dbus::start_dbus_thread(conn.clone(), running.clone())?;
    ipc_handle = sway_ipc::start_ipc_thread(running.clone())?;

    {
        let conn_lock = conn.lock().unwrap();

        systemd::reset_failed(&conn_lock)?;
        environment::set_envs(&conn_lock)?;

        systemd::start_sway_session(&conn_lock)?;
    }

    println!("Waiting for threads to join");
    dbus_handle.join().unwrap();
    ipc_handle.join().unwrap();
    println!("All threads joined");

    // Clean up
    {
        let conn_lock = conn.lock().unwrap();
        systemd::stop_sway_session(&conn_lock)?;
        environment::unset_envs(&conn_lock)?;
    }
    Ok(())
}
