mod player;
mod scene;
mod assets;
mod prelude;
mod tile;

use crate::prelude::*;
use bevy::prelude::*;


const SCALING: Vec2 = Vec2::splat(2.0);


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Bevy Bounce".to_string(),
                    width: 320.0,
                    height: 320.0,
                    // position: WindowPosition::At(Vec2::default()),
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_state(AppState::Main)
        .add_plugin(AssetsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
