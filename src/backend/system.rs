pub enum OS {
    All,
    Linux,
    MacOs,
    Windows
}

impl OS {
    #[cfg(target_os="linux")]
    pub fn current() -> OS {
        OS::Linux
    }

    #[cfg(target_os="macos")]
    pub fn current() -> OS {
        OS::MacOs
    }

    #[cfg(target_os="windows")]
    pub fn current() -> OS {
        OS::Windows
    }
}
