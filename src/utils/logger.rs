pub fn init() {
    let console_logger = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout());

    //    let file_logger = fern::Dispatch::new()
    //        .level(log::LevelFilter::Info)
    //        .chain(fern::log_file("rust_blog.log")?);

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{},{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(console_logger)
        .apply()
        .unwrap_or_default();
}
