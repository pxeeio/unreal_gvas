use std::fmt::{Display, Formatter, Result};

/// Unreal Engine Version Information
pub struct UEngineVersion {
    /// Unreal Engine major version
    pub major: u16,
    /// Unreal Engine minor version
    pub minor: u16,
    /// Unreal Engine patch version
    pub patch: u16,
    /// Unreal Engine changelist number
    pub changelist: u32,
    /// Unreal Engine build branch name
    pub branch: String,
}

impl Display for UEngineVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Unreal Engine Version {}.{}.{}-{}{}",
            self.major, self.minor, self.patch, self.changelist, self.branch
        )
    }
}
