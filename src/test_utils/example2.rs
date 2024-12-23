use std::path::PathBuf;

use crate as confique;
use crate::Config;

#[derive(Debug, Config)]
/// A sample configuration for our app.
#[allow(dead_code)]
pub struct Conf {
    #[config(nested)]
    pub http: Http,
}

/// Configuring the HTTP server of our app.
#[derive(Debug, Config)]
#[allow(dead_code)]
pub struct Http {
    #[config(nested)]
    pub headers: Headers,

    #[config(nested)]
    pub log: LogConfig,
}

#[derive(Debug, Config)]
#[allow(dead_code)]
pub struct Headers {
    /// The header in which the reverse proxy specifies the username.
    #[config(default = "x-username")]
    pub username: String,

    /// The header in which the reverse proxy specifies the display name.
    #[config(default = "x-display-name")]
    pub display_name: String,
}


#[derive(Debug, Config)]
#[allow(dead_code)]
pub struct LogConfig {
    /// If set to `true`, the app will log to stdout.
    #[config(default = true)]
    pub stdout: bool,

    /// If this is set, the app will write logs to the given file. Of course,
    /// the app has to have write access to that file.
    pub file: Option<PathBuf>,
}
