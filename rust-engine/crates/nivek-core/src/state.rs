use std::time::Instant;

pub struct DaemonState {
    pub start_time: Instant,
    pub version: &'static str,
}

impl DaemonState {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    pub fn memory_rss_bytes(&self) -> u64 {
        if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
            let mut parts = statm.split_whitespace();
            if let Some(_size) = parts.next() {
                if let Some(resident) = parts.next() {
                    if let Ok(pages) = resident.parse::<u64>() {
                        // Páginas estándar de 4096 bytes en arquitectura x86_64 Linux
                        return pages * 4096;
                    }
                }
            }
        }
        0
    }
}
