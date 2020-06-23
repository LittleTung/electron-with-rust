use neon::register_module;

pub use error::Error;

mod error;
mod ext;
mod prelude;
mod runtime;
mod transfer;

pub type Result<T> = std::result::Result<T, crate::Error>;

register_module!(mut cx, {
    cx.export_function("nativeStartTokioRuntime", runtime::start_runtime)
        .unwrap();
    cx.export_function("nativeShutdownTokioRuntime", runtime::shutdown_runtime)
        .unwrap();
    cx.export_function("nativeStartServer", transfer::start_server).unwrap();
    cx.export_function("nativeSendFile", transfer::send_file).unwrap();

    Ok(())
});