pub const TILE_WIDTH: f64 = 46.0;

pub const TILES_PCS_W: usize = 64;
pub const TILES_PCS_H: usize = 64;

#[cfg_attr(feature = "cargo-clippy", allow(decimal_literal_representation))]
pub const TILE_MAP_BUF_LENGTH: usize = 4096;
pub const CHARACTER_BUF_LENGTH: usize = 224;

pub const RESOLUTION_X: u32 = 1920;
pub const RESOLUTION_Y: u32 = 1080;

pub const ASPECT_RATIO: f32 = (RESOLUTION_X / RESOLUTION_Y) as f32;

pub const VIEW_DISTANCE: f32 = 300.0;

pub const CHARACTER_SHEET_TOTAL_WIDTH: f32 = 16_128f32;
pub const SPRITE_OFFSET: f32 = 2.0;

pub const ZOMBIE_SHEET_TOTAL_WIDTH: f32 = 9_184f32;

pub const BULLET_SPEED: f32 = 1.0;

//Assets
pub const ZOMBIE_JSON_PATH: &str = "assets/zombie.json";
pub const CHARACTER_JSON_PATH: &str = "assets/character.json";
pub const PISTOL_AUDIO_PATH: &str = "assets/audio/pistol.ogg";
pub const MAP_FILE_PATH: &str = "assets/maps/tilemap.tmx";

pub const RUN_SPRITE_OFFSET: usize = 64;
pub const ZOMBIE_STILL_SPRITE_OFFSET: usize = 32;
pub const NORMAL_DEATH_SPRITE_OFFSET: usize = 64;

pub const HOUSE_POSITIONS: [[f32; 2]; 2] = [[-50.0, 650.0], [500.0, 250.0]];
pub const TREE_POSITIONS: [[f32; 2]; 2] = [[-550.0, -250.0], [400.0, -400.0]];
