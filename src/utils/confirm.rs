use dialoguer::Confirm;

use crate::{
    cli::atomic::should_accept_all,
    utils::logger::{LogLevel, print_log},
};

/// Ask "Y/N?"; returns true if accept_all is set or the user types "y" or "Y"
pub fn confirm_action(prompt: &str) -> bool {
    if should_accept_all() {
        print_log(
            LogLevel::Prompt,
            &format!("{prompt} [y/N]: y (auto-accepted)"),
        );

        return true;
    }

    Confirm::new().with_prompt(prompt).interact().unwrap()
}
