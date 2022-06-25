
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __log {
    (lvl: $lvl:expr, $($arg:tt)+ ) => {
        log::log!(target: hlog::CORE, $lvl, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log_error {
    ( $($arg:tt)+ ) => {
        __log!(lvl:log::Level::Error, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log_warn {
    ( $($arg:tt)+ ) => {
        __log!(lvl:log::Level::Warn, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log_info {
    ( $($arg:tt)+ ) => {
        __log!(lvl:log::Level::Info, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log_debug {
    ( $($arg:tt)+ ) => {
        __log!(lvl:log::Level::Debug, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! log_trace {
    ( $($arg:tt)+ ) => {
        __log!(lvl:log::Level::Trace, $($arg)+)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! bit {
    ( $x:literal ) => {
        1 << $x
    };
}
