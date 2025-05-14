use std::io::Write;

pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| { writeln!(buf, "{}: {}", record.level(), record.args()) })
        .filter_level(log::LevelFilter::Info)
        .init();
}
