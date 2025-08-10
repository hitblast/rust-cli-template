use std::sync::atomic::{AtomicBool, Ordering};

/*
 * Atomic variables which are only set by global flags per process execution.
 * You may modify them according to your own needs.
 *
 */

// --accept-all
static ACCEPT_ALL: AtomicBool = AtomicBool::new(false);

pub fn set_accept_all(value: bool) {
    ACCEPT_ALL.store(value, Ordering::SeqCst);
}

pub fn should_accept_all() -> bool {
    ACCEPT_ALL.load(Ordering::SeqCst)
}

// --quiet
static QUIET: AtomicBool = AtomicBool::new(false);

pub fn set_quiet(value: bool) {
    QUIET.store(value, Ordering::SeqCst);
}

pub fn should_be_quiet() -> bool {
    QUIET.load(Ordering::SeqCst)
}

// --verbose
static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(value: bool) {
    VERBOSE.store(value, Ordering::SeqCst);
}

pub fn should_be_verbose() -> bool {
    VERBOSE.load(Ordering::SeqCst)
}

// --dry-run
static DRY_RUN: AtomicBool = AtomicBool::new(false);

pub fn set_dry_run(value: bool) {
    DRY_RUN.store(value, Ordering::SeqCst);
}

pub fn should_dry_run() -> bool {
    DRY_RUN.load(Ordering::SeqCst)
}
