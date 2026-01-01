use dialoguer::Confirm;

use crate::{cli::atomic::should_accept_all, log_prompt};

/// Ask "Y/N?"; returns true if accept_all is set or the user types "y" or "Y"
#[must_use]
pub fn confirm_action(prompt: &str) -> bool {
    if should_accept_all() {
        log_prompt!("{prompt} [y/N]: (auto-accepted)");

        return true;
    }

    Confirm::new()
        .with_prompt(prompt)
        .interact()
        .unwrap_or(false)
}
