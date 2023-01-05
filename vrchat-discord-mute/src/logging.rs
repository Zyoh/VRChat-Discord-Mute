pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // Include timestamp
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            println!("{} - {} - {}", timestamp, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
