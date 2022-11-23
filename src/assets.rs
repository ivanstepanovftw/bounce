use bevy::prelude::*;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct Graphics {
    pub texture_atlas: Handle<TextureAtlas>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics);
    }
}

impl AssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_handle = asset_server.load("objects_nm.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 4, 6, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.insert_resource(Graphics { texture_atlas: texture_atlas_handle.clone() });
    }
}
