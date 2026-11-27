// src/browser_handoff/mod.rs
// Browser URL handoff and default-browser management for SigmaOS
// (`sigmaos-default-browser`, `sigmaos-cmd-browser-handoff`,
// `sigmaos-chromium-copy-url-host`, `sigmaos-chromium-ytdlp-host`).

pub mod browser_handoff;

pub use browser_handoff::{
    BrowserHandoff, DefaultBrowser, HandoffError, HandoffOutcome, HandoffResult,
};
