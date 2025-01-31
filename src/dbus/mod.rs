use std::{
    collections::HashMap,
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use ::dbus::{
    blocking::Connection,
    message::{MatchRule, Message},
};

use crate::systemd;
use dbus::arg::{prop_cast, PropMap};

mod org_freedesktop_dbus;

pub fn set_envs(conn: &Connection, envs: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    use crate::dbus::org_freedesktop_dbus::OrgFreedesktopDBus;
    let dbus = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        Duration::from_secs(5),
    );
    let dbus_envs: HashMap<&str, &str> = envs
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    dbus.update_activation_environment(dbus_envs)?;
    Ok(())
}

pub fn start_dbus_thread(
    conn: Arc<Mutex<Connection>>,
    running_flag: Arc<AtomicBool>,
) -> Result<thread::JoinHandle<()>, Box<dyn Error>> {
    println!("Initializing dbus thread");
    let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged")
        .with_sender("org.kde.StatusNotifierWatcher")
        .with_path("/StatusNotifierWatcher");

    conn.lock()
        .unwrap()
        .add_match(rule, |_: (), conn: &Connection, msg: &Message| {
            let (interface, properties): (&str, PropMap) = msg.read2().unwrap();
            if interface == "org.kde.StatusNotifierWatcher"
                && Some(&true) == prop_cast::<bool>(&properties, "IsStatusNotifierHostRegistered")
            {
                systemd::start_sway_xdg(conn).unwrap();
            }
            true
        })?;

    Ok(thread::spawn(move || {
        println!("Starting dbus thread");
        while running_flag.load(SeqCst) {
            if let Ok(conn) = conn.lock() {
                conn.process(Duration::from_millis(100)).unwrap();
            }
            thread::sleep(Duration::from_millis(100));
        }
        println!("Exiting dbus thread");
    }))
}
