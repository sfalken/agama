// Copyright (c) [2024] SUSE LLC
//
// All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, contact SUSE LLC.
//
// To contact SUSE LLC about this file by physical or electronic mail, you may
// find current contact information at www.suse.com.

//! Handles Agama web server configuration.
//!
//! The configuration can be written in YAML or JSON formats, although we plan to choose just one
//! of them in the future. It is read from the following locations:
//!
//! * `/usr/etc/agama.d/server.{json/yaml}`
//! * `/etc/agama.d/server.{json/yaml}`
//! * `etc/agama.d/server.{json/yaml}`
//!
//! All the settings are merged into a single configuration. The values in the latter locations
//! take precedence.

use config::{Config, ConfigError, File};
use rand::distr::{Alphanumeric, SampleString};
use serde::Deserialize;

/// Web service configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfig {
    /// Key to sign the JSON Web Tokens.
    pub jwt_secret: String,
}

const JWT_SECRET_FILE: &str = "/run/agama/jwt_secret";

impl ServiceConfig {
    pub fn load() -> Result<Self, ConfigError> {
        const JWT_SECRET_SIZE: usize = 30;
        let jwt_secret: Self::load_or_create_jwt_secret(JWT_SECRET_SIZE);

        let config = Config::builder()
            .set_default("jwt_secret", jwt_secret)?
            .add_source(File::with_name("/usr/etc/agama.d/server").required(false))
            .add_source(File::with_name("/etc/agama.d/server").required(false))
            .add_source(File::with_name("etc/agama.d/server").required(false))
            .build()?;
        config.try_deserialize()
    }

    fn load_or_create_jwt_secret(size: usize) -> String {
        use std::fs;
        use std::io::Write;

        if let Ok(secret) = fs::read_to_string(JWT_SECRET_FILE) {
            let secret = secret.trim().to_string();
            if !secret.is_empty() {
                return secret;
            }
        }

        let secret = Alphanumeric.sample_string(&mut rand::rng(), size);
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(JWT_SECRET_FILE)
        {
            let _ = file.write_all(secret.as_bytes());
        }
        secret
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "".to_string(),
        }
    }
}
