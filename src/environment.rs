use crate::dbus;
use crate::systemd;
use ::dbus::blocking::Connection;
use std::{collections::HashMap, env, error::Error};

const ENVS_TO_CHECK: [&'static str; 10] = [
    "DESKTOP_SESSION",
    "XDG_CURRENT_DESKTOP",
    "XDG_SESSION_DESKTOP",
    "XDG_SESSION_TYPE",
    "DISPLAY",
    "I3SOCK",
    "SWAYSOCK",
    "WAYLAND_DISPLAY",
    "XCURSOR_THEME",
    "XCURSOR_SIZE",
];

pub fn set_envs(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let populated_envs: HashMap<String, String> = ENVS_TO_CHECK
        .into_iter()
        .filter_map(|key| env::var(key).ok().map(|val| (key.to_string(), val)))
        .collect();
    systemd::set_env(conn, &populated_envs)?;
    dbus::set_envs(conn, &populated_envs)?;
    Ok(())
}

pub fn unset_envs(conn: &Connection) -> Result<(), Box<dyn Error>> {
    systemd::unset_envs(conn, Vec::from(ENVS_TO_CHECK))?;
    Ok(())
}
