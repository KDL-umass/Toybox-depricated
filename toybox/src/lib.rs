#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate png;
extern crate toybox_core;

pub use toybox_core::graphics;
pub use toybox_core::random;
/// Input represents the buttons pressed given to our games.
pub use toybox_core::Input;
pub use toybox_core::Simulation;
pub use toybox_core::State;

/// This method returns a Box<Simulation> if possible for a given game name.
pub fn get_simulation_by_name(name: &str) -> Result<Box<Simulation>, failure::Error> {
    let y: Result<Box<Simulation>, _> = match name.to_lowercase().as_str() {
        #[cfg(feature = "breakout")]
        "breakout" => Ok(Box::new(breakout::Breakout::default())),
        _ => Err(format_err!(
            "Cannot construct game: `{}`. Try any of {:?}.",
            name,
            GAME_LIST
        )),
    };
    y
}

/// This defines the set of games that are known. An index into this array is used in human_play, so try not to shuffle them!
pub const GAME_LIST: &[&str] = &[
    #[cfg(feature = "breakout")]
    "breakout",
];

/// Breakout defined in this module.
#[cfg(feature = "breakout")]
extern crate breakout;
