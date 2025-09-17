pub fn get_desktop_info() -> String {
    // Retrieve the environment variables and handle Result types
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP");
    let display_backend = {
        let mut v = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        if let Some(c) = v.as_mut_str().get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        if v.is_empty() {
            "Unknown".to_string()
        } else {
            v
        }
    };

    if std::env::var("NIRI_SOCKET").is_ok() {
        return format!("niri ({display_backend})");
    }

    // Trim "none+" from the start of desktop_env if present
    // Use "Unknown" if desktop_env is empty or has an error
    let desktop_env = match desktop_env {
        Err(_) => "Unknown".to_string(),
        Ok(s) => s.trim_start_matches("none+").to_string(),
    };

    format!("{desktop_env} ({display_backend})")
}
