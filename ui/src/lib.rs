use std::{error::Error, fmt::Display};

mod app;
mod platform;
mod wgpu;

// Lota的异常类
#[derive(Debug, Clone)]
pub struct Exception {
    info: String,
}

impl Error for Exception {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.info.as_str())
    }
}

pub fn run() {
    crate::app::launch();
}
