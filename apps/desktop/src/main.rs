// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(all(not(debug_assertions), windows), windows_subsystem = "windows")]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gota_ui::run()?;
    Ok(())
}
