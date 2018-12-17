pub enum OS {
    all,
    linux,
    macos,
    windows
}

impl OS {
    #[cfg(target_os="linux")]
    pub fn current() -> OS {
        OS::linux
    }

    #[cfg(target_os="macos")]
    pub fn current() -> OS {
        OS::macos
    }

    #[cfg(target_os="windows")]
    pub fn current() -> OS {
        OS::windows
    }
}
