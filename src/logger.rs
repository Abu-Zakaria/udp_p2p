use log::{LevelFilter, Log, Metadata, Record};

pub struct Logger {
    pub level: LevelFilter,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.level() == LevelFilter::Info {
                println!("{}", record.args());
            } else if record.level() == LevelFilter::Error {
                eprintln!("ERROR: {}", record.args());
            } else {
                println!("[{}] => {}", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {}
}
