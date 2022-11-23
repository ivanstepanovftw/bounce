// pub use crate::error::Error;
// pub use anyhow::Result;

use bevy::{asset::LoadState, prelude::*};

// App-related
use crate::assets;
pub use assets::{AssetsPlugin, Graphics};

use crate::player;
pub use player::PlayerPlugin;

use crate::scene;
pub use scene::{AppState, ScenePlugin};

// struct AppGroup;
// impl PluginGrout for AppGroup {
//     fn build(&mut self, group: &mut PluginGroupBuilder) {
//
//     }
// }

// New (Bevy 0.9)
// impl PluginGroup for HelloWorldPlugins {
//     fn build(self) -> PluginGroupBuilder {
//         PluginGroupBuilder::start::<Self>()
//             .add(PrintHelloPlugin)
//             .add(PrintWorldPlugin)
//     }
// }

// Generic Wrapper tuple struct for newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preferences
pub use std::format as f;
