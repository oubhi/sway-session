use std::{collections::HashMap, error::Error, time::Duration};

use ::dbus::blocking::Connection;

mod org_freedesktop_systemd_manager;
mod org_freedesktop_systemd_unit;

fn start_unit(conn: &Connection, unit: &str) -> Result<(), Box<dyn Error>> {
    use crate::systemd::org_freedesktop_systemd_manager::OrgFreedesktopSystemd1Manager;
    let systemd = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    systemd.start_unit(unit, "replace")?;
    Ok(())
}

pub fn reset_failed(conn: &Connection) -> Result<(), Box<dyn Error>> {
    use crate::systemd::org_freedesktop_systemd_manager::OrgFreedesktopSystemd1Manager;
    let systemd = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    systemd.reset_failed()?;
    Ok(())
}

pub fn sway_session_is_active(conn: &Connection) -> Result<bool, Box<dyn Error>> {
    use crate::systemd::org_freedesktop_systemd_manager::OrgFreedesktopSystemd1Manager;
    use crate::systemd::org_freedesktop_systemd_unit::OrgFreedesktopSystemd1Unit;
    let systemd = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );

    let sway_session_target_path = match systemd.get_unit("sway-session.target") {
        Ok(path) => path,
        Err(e) if e.name() == Some("org.freedesktop.systemd1.NoSuchUnit") => return Ok(false),
        Err(e) => return Err(Box::new(e)),
    };

    let sway_session_target = conn.with_proxy(
        "org.freedesktop.systemd1",
        sway_session_target_path,
        Duration::from_secs(5),
    );

    let sway_session_target_state = sway_session_target.active_state()?;

    Ok(sway_session_target_state != "inactive" && sway_session_target_state != "failed")
}

pub fn set_env(conn: &Connection, envs: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    use crate::systemd::org_freedesktop_systemd_manager::OrgFreedesktopSystemd1Manager;
    let systemd = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    let systemd_envs: Vec<String> = envs
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();
    systemd.set_environment(systemd_envs.iter().map(|e| e.as_str()).collect())?;
    Ok(())
}

pub fn unset_envs(conn: &Connection, envs: Vec<&str>) -> Result<(), Box<dyn Error>> {
    use crate::systemd::org_freedesktop_systemd_manager::OrgFreedesktopSystemd1Manager;
    let systemd = conn.with_proxy(
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        Duration::from_secs(5),
    );
    systemd.unset_environment(envs)?;
    Ok(())
}

pub fn start_sway_session(conn: &Connection) -> Result<(), Box<dyn Error>> {
    start_unit(conn, "sway-session.target")?;
    Ok(())
}
pub fn stop_sway_session(conn: &Connection) -> Result<(), Box<dyn Error>> {
    start_unit(conn, "sway-session-shutdown.target")?;
    Ok(())
}
pub fn start_sway_xdg(conn: &Connection) -> Result<(), Box<dyn Error>> {
    start_unit(conn, "xdg-sway-autostart.target")?;
    Ok(())
}
