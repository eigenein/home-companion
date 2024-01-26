use prost::Message;

#[derive(Copy, Clone, Debug, prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    Error = 0,
    Info = 10,
    Debug = 20,
    Trace = 30,
}

#[derive(Clone, Message)]
#[must_use]
pub struct Log {
    #[prost(string, tag = "1", required)]
    pub message: String,

    #[prost(enumeration = "LogLevel", tag = "2")]
    pub level: i32,
}

#[cfg(feature = "guest")]
impl Log {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level: level as i32,
            message: message.into(),
        }
    }

    #[inline]
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(LogLevel::Error, message)
    }

    #[inline]
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(LogLevel::Info, message)
    }

    #[inline]
    pub fn debug(message: impl Into<String>) -> Self {
        Self::new(LogLevel::Debug, message)
    }

    #[inline]
    pub fn trace(message: impl Into<String>) -> Self {
        Self::new(LogLevel::Trace, message)
    }
}
