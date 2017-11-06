pub const TILE_WIDTH: f64 = 60.0;

pub const TILES_PCS_W: usize = 64;
pub const TILES_PCS_H: usize = 64;

pub const TILEMAP_BUF_LENGTH: usize = 4096;
pub const CHARACTER_BUF_LENGTH: usize = 224;

pub const RESOLUTION_X: u32 = 1920;
pub const RESOLUTION_Y: u32 = 1080;

pub const ASPECT_RATIO: f32 = (RESOLUTION_X / RESOLUTION_Y) as f32;

pub const VIEW_DISTANCE: f32 = 300.0;

pub const CHARSHEET_TOTAL_WIDTH: f32 = 16_128f32;
pub const SPRITE_OFFSET: f32 = 2.0;

pub const ZOMBIESHEET_TOTAL_WIDTH: f32 = 7_872f32;

//Assets
pub const ZOMBIE_JSON_PATH: &str = "assets/zombie.json";
pub const CHARACTER_JSON_PATH: &str = "assets/character.json";
pub const PISTOL_AUDIO_PATH: &str = "assets/audio/pistol.ogg";
pub const MAP_FILE_PATH: &str = "assets/maps/tilemap.tmx";
pub const RUN_SPRITE_OFFSET: usize = 64;
pub const STILL_SPRITE_OFFSET: usize = 32;
