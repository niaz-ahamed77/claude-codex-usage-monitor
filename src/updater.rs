#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstallChannel {
    Portable,
    Winget,
}

#[derive(Clone, Debug)]
pub struct ReleaseDescriptor {
    pub latest_version: String,
}

#[derive(Debug)]
pub enum UpdateCheckResult {
    UpToDate,
    Available(ReleaseDescriptor),
}

pub fn handle_cli_mode(_args: &[String]) -> Option<i32> {
    None
}

pub fn current_install_channel() -> InstallChannel {
    InstallChannel::Portable
}

pub fn check_for_updates() -> Result<UpdateCheckResult, String> {
    Ok(UpdateCheckResult::UpToDate)
}

pub fn begin_winget_update() -> Result<(), String> {
    Err("Automatic updates are disabled in the audited fork. Review and rebuild a pinned source revision instead.".to_string())
}

pub fn begin_self_update(_release: &ReleaseDescriptor) -> Result<(), String> {
    Err("Automatic updates are disabled in the audited fork. Review and rebuild a pinned source revision instead.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_update_check_is_disabled() {
        assert!(matches!(
            check_for_updates(),
            Ok(UpdateCheckResult::UpToDate)
        ));
    }

    #[test]
    fn executable_update_paths_are_disabled() {
        let release = ReleaseDescriptor {
            latest_version: "9.9.9".to_string(),
        };
        assert!(begin_self_update(&release).is_err());
        assert!(begin_winget_update().is_err());
    }

    #[test]
    fn update_cli_mode_is_disabled() {
        let args = vec!["monitor.exe".to_string(), "--apply-update".to_string()];
        assert_eq!(handle_cli_mode(&args), None);
    }
}
