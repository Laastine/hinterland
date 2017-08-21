//pub const PLAYER_SPEED: f64 = 170.0;
//pub const ZOOM_SPEED: f32 = 0.01;
//
//pub const FIRE_SPRITE_START_INDEX: usize = 211;
//
pub const TILES_PCS_W: usize = 32;
pub const TILES_PCS_H: usize = 32;
//
//pub const TILESHEET_PCS_W: usize = TILES_PCS_W - 1;
//pub const TILESHEET_PCS_H: usize = TILES_PCS_H - 1;

pub const CHARACTER_W: f32 = 45.0;
pub const CHARACTER_H: f32 = 60.0;

//pub const CHARACTER_POS_W: f64 = SCREEN_WIDTH * 0.5;
//pub const CHARACTER_POS_H: f64 = SCREEN_HEIGHT * 0.4;
//
//pub const ZOMBIE_W: f64 = 56.0;
//pub const ZOMBIE_H: f64 = 28.0;
//
//pub const ZOMBIE_POS_W: f64 = SCREEN_WIDTH * 0.7;
//pub const ZOMBIE_POS_H: f64 = SCREEN_HEIGHT * 0.2;
//
//pub const BULLET_W: f64 = 2.0;
//pub const BULLET_H: f64 = 2.0;
//pub const BULLET_SPEED: f64 = 2000.0;

//Assets
//pub const ZOMBIE_PATH: &'static str = "assets/zombie.png";
pub const ZOMBIE_JSON_PATH: &'static str = "assets/zombie.json";
//pub const CHARACTER_PATH: &'static str = "assets/character.png";
pub const CHARACTER_JSON_PATH: &'static str = "assets/character.json";
//pub const PISTOL_AUDIO_PATH: &'static str = "assets/audio/pistol.ogg";
pub const MAP_FILE_PATH: &'static str = "assets/maps/tilemap.tmx";

pub const TILEMAP_BUF_LENGTH: usize = 4096;
pub const CHARACTER_BUF_LENGTH: usize = 210;

pub const RESOLUTION_X: u32 = 1920;
pub const RESOLUTION_Y: u32 = 1080;

pub const ASPECT_RATIO: f32 = (RESOLUTION_X / RESOLUTION_Y) as f32;

pub const VIEW_DISTANCE: f32 = 300.0;
