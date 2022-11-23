use std::fs::File;
// use std::io::{BufReader, Read};
use std::io::prelude::*;
use bevy::prelude::*;
use crate::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy::ecs::system::EntityCommands;
use crate::scene::Id::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Splash,
    Main,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(Self::setup_scene.after("graphics")));
        // app.add_system_set(SystemSet::on_enter(AppState::Main).with_system(Self::setup_scene));
        app.add_startup_system(Self::setup_camera)
            .add_startup_system(Self::setup_scene);
    }
}


impl ScenePlugin {
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }

    fn setup_scene(
        mut commands: Commands,
        graphics: Res<Graphics>,
        asset_server: Res<AssetServer>,
    ) {
        let mut file = File::open("assets/levels/J2MElvl.001").unwrap();

        let start_col = file.read_u8().unwrap();
        let start_row = file.read_u8().unwrap();
        let start_ball_size = match file.read_u8().unwrap() {
            0 => 12,
            _ => 16,
        };
        let exit_pos_x = file.read_u8().unwrap();
        let exit_pos_y = file.read_u8().unwrap();
        let total_num_rings = file.read_u8().unwrap();
        let tile_map_width = file.read_u8().unwrap() as usize;
        let tile_map_height = file.read_u8().unwrap() as usize;
        let mut tile_map = vec![vec![0; tile_map_width]; tile_map_height];
        for row in 0..tile_map_height {
            for col in 0..tile_map_width {
                tile_map[row][col] = file.read_u8().unwrap();
            }
        }
        let num_move_obj = file.read_u8().unwrap() as usize;
        if num_move_obj != 0 {
            let mut mo_top_left = vec![vec![0; 2]; num_move_obj];
            let mut mo_bottom_right = vec![vec![0; 2]; num_move_obj];
            let mut mo_direction = vec![vec![0; 2]; num_move_obj];
            let mut mo_offset = vec![vec![0; 2]; num_move_obj];
            // let mut mo_img = vec![0; num_move_obj];
            let mut mo_img_graphics = vec![0; num_move_obj];
            for k in 0..num_move_obj {
                mo_top_left[k][0] = file.read_u8().unwrap();
                mo_top_left[k][1] = file.read_u8().unwrap();
                mo_bottom_right[k][0] = file.read_u8().unwrap();
                mo_bottom_right[k][1] = file.read_u8().unwrap();
                mo_direction[k][0] = file.read_u8().unwrap();
                mo_direction[k][1] = file.read_u8().unwrap();
                mo_offset[k][0] = file.read_u8().unwrap();
                mo_offset[k][1] = file.read_u8().unwrap();
            }

            // mSpikeImgPtr = Image.createImage(24, 24);
            // Graphics g = mSpikeImgPtr.getGraphics();
            // g.drawImage(tileImages[46], 0, 0, 20);
            // g.drawImage(manipulateImage(tileImages[46], 0), 12, 0, 20);
            // g.drawImage(manipulateImage(tileImages[46], 4), 12, 12, 20);
            // g.drawImage(manipulateImage(tileImages[46], 1), 0, 12, 20);
        }


        for row in 0..tile_map_height {
            for col in 0..tile_map_width {
                if (tile_map[row][col] & ID_COLLIDER_FLAG) != 0 {
                    tile_map[row][col] &= ID_COLLIDER_MASK;
                }
                let mut id = tile_map[row][col];
                let is_water = (id & ID_WATER_FLAG) != 0;
                if is_water {
                    id &= ID_WATER_MASK;
                }

                let t = Transform {
                    translation: Vec3::new(col as f32 * 12.0, row as f32 * -12.0 + 12. * 7., 0.0),
                    rotation: Quat::from_rotation_z(0.0),
                    scale: Vec3::new(1.0, 1.0, 0.0),
                };
                let mut c = commands.spawn(SpriteBundle {
                    transform: t.clone(),
                    sprite: Sprite {
                        color: BACKGROUND_COLOUR,
                        custom_size: Some(Vec2::new(12.0, 12.0)),
                        ..default()
                    },
                    ..default()
                });

                let atlas_index: Option<u8> = match id {
                    ID_BRICK_RED => Some(1),
                    ID_BRICK_BLUE => Some(9),
                    ID_SPIKE_FLOOR |
                    ID_SPIKE_LEFT_WALL |
                    ID_SPIKE_CEILING |
                    ID_SPIKE_RIGHT_WALL => Some(12),
                    ID_HOOP_ACTIVE_VERT_TOP |
                    ID_HOOP_ACTIVE_VERT_BOTTOM |
                    ID_HOOP_ACTIVE_HORIZ_LEFT |
                    ID_HOOP_ACTIVE_HORIZ_RIGHT => Some(21),
                    ID_HOOP_INACTIVE_VERT_TOP |
                    ID_HOOP_INACTIVE_VERT_BOTTOM |
                    ID_HOOP_INACTIVE_HORIZ_LEFT |
                    ID_HOOP_INACTIVE_HORIZ_RIGHT => Some(23),
                    ID_LARGE_HOOP_ACTIVE_VERT_TOP |
                    ID_LARGE_HOOP_ACTIVE_VERT_BOTTOM |
                    ID_LARGE_HOOP_ACTIVE_HORIZ_LEFT |
                    ID_LARGE_HOOP_ACTIVE_HORIZ_RIGHT => Some(20),
                    ID_LARGE_HOOP_INACTIVE_VERT_TOP |
                    ID_LARGE_HOOP_INACTIVE_VERT_BOTTOM |
                    ID_LARGE_HOOP_INACTIVE_HORIZ_LEFT |
                    ID_LARGE_HOOP_INACTIVE_HORIZ_RIGHT => Some(22),
                    _ => None,
                };

                match id {
                    ID_BRICK_RED |
                    ID_BRICK_BLUE |
                    ID_HOOP_ACTIVE_VERT_TOP |
                    ID_HOOP_ACTIVE_VERT_BOTTOM |
                    ID_HOOP_ACTIVE_HORIZ_LEFT |
                    ID_HOOP_ACTIVE_HORIZ_RIGHT => {
                        c.insert(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(atlas_index.unwrap().into()),
                            texture_atlas: graphics.texture_atlas.clone(),
                            transform: t.clone(),
                            ..default()
                        });
                    }
                    _ => {
                        // c.insert(SpriteSheetBundle {
                        //     sprite: TextureAtlasSprite::new(id as usize % 24),
                        //     texture_atlas: graphics.texture_atlas.clone(),
                        //     transform: Transform::from_xyz(
                        //         col as f32 * 12.0,
                        //         row as f32 * -12.0 + 12. * 8.,
                        //         0.0,
                        //     ),
                        //     ..default()
                        // });
                    }
                };
            }
        }
    }

    // fn draw_tile(
    //     mut commands: &mut Commands,
    //     graphics: &Res<Graphics>,
    //     index: usize,
    //     col: usize,
    //     row: usize,
    // ) {
    //     commands.spawn(SpriteSheetBundle {
    //         sprite: TextureAtlasSprite::new(index),
    //         texture_atlas: graphics.texture_atlas.clone(),
    //         transform: Transform::from_xyz(
    //             col as f32 * 12.0,
    //             row as f32 * -12.0 + 12. * 8.,
    //             0.0,
    //         ),
    //         ..default()
    //     });
    // }
}


// trait CommandsTile<'w, 's> {
//     fn addTile<'a, T: Bundle>(&'a mut self, graphics: &Res<Graphics>, index: usize, col: usize, row: usize) -> EntityCommands<'w, 's, 'a>;
// }
//
// impl<'w, 's> CommandsTile<'w, 's> for Commands<'w, 's> {
//     fn addTile<'a, T: Bundle>(
//         &'a mut self,
//         graphics: &Res<Graphics>,
//         index: usize,
//         col: usize,
//         row: usize) -> EntityCommands<'w, 's, 'a>
//     {
//         return self.spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(index),
//             texture_atlas: graphics.texture_atlas.clone(),
//             transform: Transform::from_xyz(
//                 col as f32 * 12.0,
//                 row as f32 * -12.0 + 12. * 8.,
//                 0.0,
//             ),
//             ..default()
//         });
//     }
// }


pub mod Id {
    use bevy::prelude::Color;

    pub const ID_EMPTY_SPACE: u8 = 0;
    pub const ID_BRICK_RED: u8 = 1;
    pub const ID_BRICK_BLUE: u8 = 2;
    pub const ID_SPIKE_FLOOR: u8 = 3;
    pub const ID_SPIKE_LEFT_WALL: u8 = 4;
    pub const ID_SPIKE_CEILING: u8 = 5;
    pub const ID_SPIKE_RIGHT_WALL: u8 = 6;
    pub const ID_RESPAWN_GEM: u8 = 7;
    pub const ID_RESPAWN_INDICATOR: u8 = 8;
    pub const ID_EXIT_TILE: u8 = 9;
    pub const ID_MOVING_SPIKE_TILE: u8 = 10;
    pub const ID_HOOP_ACTIVE_VERT_TOP: u8 = 13;
    pub const ID_HOOP_ACTIVE_VERT_BOTTOM: u8 = 14;
    pub const ID_HOOP_ACTIVE_HORIZ_LEFT: u8 = 15;
    pub const ID_HOOP_ACTIVE_HORIZ_RIGHT: u8 = 16;
    pub const ID_HOOP_INACTIVE_VERT_TOP: u8 = 17;
    pub const ID_HOOP_INACTIVE_VERT_BOTTOM: u8 = 18;
    pub const ID_HOOP_INACTIVE_HORIZ_LEFT: u8 = 19;
    pub const ID_HOOP_INACTIVE_HORIZ_RIGHT: u8 = 20;
    pub const ID_LARGE_HOOP_ACTIVE_VERT_TOP: u8 = 21;
    pub const ID_LARGE_HOOP_ACTIVE_VERT_BOTTOM: u8 = 22;
    pub const ID_LARGE_HOOP_ACTIVE_HORIZ_LEFT: u8 = 23;
    pub const ID_LARGE_HOOP_ACTIVE_HORIZ_RIGHT: u8 = 24;
    pub const ID_LARGE_HOOP_INACTIVE_VERT_TOP: u8 = 25;
    pub const ID_LARGE_HOOP_INACTIVE_VERT_BOTTOM: u8 = 26;
    pub const ID_LARGE_HOOP_INACTIVE_HORIZ_LEFT: u8 = 27;
    pub const ID_LARGE_HOOP_INACTIVE_HORIZ_RIGHT: u8 = 28;
    pub const ID_EXTRA_LIFE: u8 = 29;
    pub const ID_TRIANGLE_TOP_LEFT: u8 = 30;
    pub const ID_TRIANGLE_TOP_RIGHT: u8 = 31;
    pub const ID_TRIANGLE_BOT_RIGHT: u8 = 32;
    pub const ID_TRIANGLE_BOT_LEFT: u8 = 33;
    pub const ID_RUBBER_TRIANGLE_TOP_LEFT: u8 = 34;
    pub const ID_RUBBER_TRIANGLE_TOP_RIGHT: u8 = 35;
    pub const ID_RUBBER_TRIANGLE_BOT_RIGHT: u8 = 36;
    pub const ID_RUBBER_TRIANGLE_BOT_LEFT: u8 = 37;
    pub const ID_SPEED: u8 = 38;
    pub const ID_DEFLATOR_FLOOR: u8 = 39;
    pub const ID_DEFLATOR_LEFT_WALL: u8 = 40;
    pub const ID_DEFLATOR_CEILING: u8 = 41;
    pub const ID_DEFLATOR_RIGHT_WALL: u8 = 42;
    pub const ID_INFLATOR_FLOOR: u8 = 43;
    pub const ID_INFLATOR_LEFT_WALL: u8 = 44;
    pub const ID_INFLATOR_CEILING: u8 = 45;
    pub const ID_INFLATOR_RIGHT_WALL: u8 = 46;
    pub const ID_GRAVITY_FLOOR: u8 = 47;
    pub const ID_GRAVITY_LEFT_WALL: u8 = 48;
    pub const ID_GRAVITY_CEILING: u8 = 49;
    pub const ID_GRAVITY_RIGHT_WALL: u8 = 50;
    pub const ID_JUMP_FLOOR: u8 = 51;
    pub const ID_JUMP_LEFT_WALL: u8 = 52;
    pub const ID_JUMP_CEILING: u8 = 53;
    pub const ID_JUMP_RIGHT_WALL: u8 = 54;

    pub const BACKGROUND_COLOUR: Color = Color::rgb(0.69, 0.88, 0.94);
    pub const WATER_COLOUR: Color = Color::rgb(0.06, 0.38, 0.69);

    pub const ID_WATER_FLAG: u8 = 1 << 6;
    pub const ID_COLLIDER_FLAG: u8 = 1 << 7;
    pub const ID_WATER_MASK: u8 = !ID_WATER_FLAG;
    pub const ID_COLLIDER_MASK: u8 = !ID_COLLIDER_FLAG;
}
