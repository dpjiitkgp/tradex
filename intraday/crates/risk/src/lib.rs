//! ExitManager: the hard-stop / minimum-2x-target-floor / dynamic-trailing state machine shared by every open position.

pub mod exit_manager;

pub use exit_manager::DefaultExitManager;
