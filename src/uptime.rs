use sysinfo_dot_h::try_collect;

pub async fn get_uptime() -> String {
    let info = try_collect().expect("Failed to collect system info");
    let uptime = info.uptime;

    let (days, hrs, mins) = (
        uptime / (60 * 60 * 24),
        (uptime / (60 * 60)) % 24,
        (uptime / 60) % 60,
    );

    let mut formatted_uptime = String::new();

    if days > 0 {
        formatted_uptime.push_str(&format!("{}d, ", days));
    }
    if hrs > 0 || days > 0 {
        formatted_uptime.push_str(&format!("{}h, ", hrs));
    }
    if mins > 0 || hrs > 0 || days > 0 {
        formatted_uptime.push_str(&format!("{}m", mins));
    } else {
        formatted_uptime.push_str(&format!("{}s", uptime));
    }

    formatted_uptime
}
