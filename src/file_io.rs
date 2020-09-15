//! Holds functionality related to opening, reading, and writing game files.
use amethyst::utils::application_dir;
use std::error::Error;
use std::{env, io, path};
use webbrowser;

/// Gets the path to the game's asset directory.
pub fn assets_dir() -> Result<path::PathBuf, io::Error> {
    application_dir("assets")
}

/// Gets the path to the game's input bindings file.
pub fn input_bindings_file() -> Result<path::PathBuf, io::Error> {
    Ok(assets_dir()?.join("data").join("input.ron"))
}

/// Opens the game's player manual in the default browser.
///
/// # Errors
/// An error is returned if there was a problem getting the path to the manual
/// or an issue opening the browser.
pub fn open_player_manual() -> Result<(), Box<dyn Error>> {
    // Get the path to the player manual index file and ensure it exists.
    let player_manual_index = player_manual_dir()?.join("index.html");
    if !player_manual_index.exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to find the player manual files.",
        )));
    }

    // Covert the path to a string and open it in the default web browser.
    if let Some(path_str) = player_manual_index.to_str() {
        webbrowser::open(path_str)?;
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Unable to convert the path to a URL.",
        )))
    }
}

// Gets the player manual directory.
fn player_manual_dir() -> Result<path::PathBuf, io::Error> {
    // We check to see if we appear to be running from a cargo build, if so we
    // the player manual is in the target directory instead of the root directory.
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        Ok(path::PathBuf::from(manifest_dir)
            .join("target")
            .join("player-manual"))
    } else {
        application_dir("player-manual")
    }
}
