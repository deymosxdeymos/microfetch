use nix::sys::sysinfo::sysinfo;
use std::io;

pub fn get_current() -> Result<String, io::Error> {
    let info = sysinfo().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let uptime_seconds = info.uptime().as_secs();

    let days = uptime_seconds / (60 * 60 * 24);
    let hours = (uptime_seconds / 3600) % 24;
    let minutes = (uptime_seconds / 60) % 60;

    let mut result = String::new();
    if days > 0 {
        result.push_str(&format!("{days} day{}", if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{hours} hour{}", if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!(
            "{minutes} minute{}",
            if minutes > 1 { "s" } else { "" }
        ));
    }

    Ok(result)
}
