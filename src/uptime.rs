use nix::sys::sysinfo::sysinfo;
use std::io;

pub fn get_current() -> Result<String, io::Error> {
    let info = sysinfo().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let uptime_seconds = info.uptime().as_secs();

    let total_minutes = uptime_seconds / 60;
    let days = total_minutes / (60 * 24);
    let hours = (total_minutes % (60 * 24)) / 60;
    let minutes = total_minutes % 60;

    let parts = [(days, "day"), (hours, "hour"), (minutes, "minute")]
        .iter()
        .filter(|&&(value, _)| value > 0)
        .map(|&(value, label)| format!("{value} {label}{}", if value > 1 { "s" } else { "" }))
        .collect::<Vec<_>>();

    Ok(parts.join(", "))
}
