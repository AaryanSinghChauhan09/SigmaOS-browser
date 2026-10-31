# SigmaOS Browser

This crate exports a `BrowserIntegration` API for browser selection and URL
handoff. The subsystem keeps selection and deferred URL state; process launch is
left to the caller. It is not a browser engine and does not establish that a
browser is installed, launched, or able to load the URL.

The crate depends on the sibling `SigmaOS-kernel` checkout through a local path.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).

