// Copyright (c) [2026] Shawn W Dunn
// Copyright (c) [2026] SUSE LLC
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

//! Implements the module set for controlling which Agama modules are enabled
//! at runtime


pub struct ModuleSet {
    pub disabled: Vec<string>,  // everything else is enabled
}

impl ModuleSet {
    pub fn is_enabled(&self, name: &str) -> bool {
        !self.disabled.iter().any(|d| d == name)
    }
    // product merges at image-level
    pub fn merge_with(&self, other: &ModuleSet) -> ModuleSet { ... }

    // Load from $AGAMA_SHARE_DIR/agama.yaml; returns default if absent.
    pub fn load() -> Self { ... }
}
