pub const TILES_PCS_W: usize = 128;
pub const TILES_PCS_H: usize = 128;

pub const TILE_SIZE: f32 = 48.0;
pub const TILE_WIDTH: f32 = TILE_SIZE * 2.0;

pub const Y_OFFSET: f32 = TILES_PCS_W as f32 / 2.0 * TILE_WIDTH;

pub const CHARACTER_BUF_LENGTH: usize = 224;

pub const RESOLUTION_X: u32 = 1600;
pub const RESOLUTION_Y: u32 = 900;

pub const ASPECT_RATIO: f32 = (RESOLUTION_X / RESOLUTION_Y) as f32;

pub const VIEW_DISTANCE: f32 = 300.0;

pub const CHARACTER_SHEET_TOTAL_WIDTH: f32 = 16_128f32;
pub const SPRITE_OFFSET: f32 = 2.0;

pub const ZOMBIE_SHEET_TOTAL_WIDTH: f32 = 9_184f32;

pub const BULLET_SPEED: f32 = 15.0;
pub const CHARACTER_X_SPEED: f32 = 3.0;
pub const CHARACTER_Y_SPEED: f32 = 3.0;

pub const GAME_TITLE: &str = "Hinterland";

//Assets
pub const ZOMBIE_JSON_PATH: &str = "assets/zombie.json";
pub const CHARACTER_JSON_PATH: &str = "assets/character.json";
pub const PISTOL_AUDIO_PATH: &str = "assets/audio/pistol.ogg";
pub const MAP_FILE_PATH: &str = "assets/maps/tilemap.tmx";

pub const RUN_SPRITE_OFFSET: usize = 64;
pub const ZOMBIE_STILL_SPRITE_OFFSET: usize = 32;
pub const NORMAL_DEATH_SPRITE_OFFSET: usize = 64;

// Object positions
pub const AMMO_POSITIONS: [[i32; 2]; 4] = [ [ -13, -12 ], [ -15, 8 ], [ 16, -8 ], [ 1, 14 ] ];
pub const HOUSE_POSITIONS: [[i32; 2]; 2] = [[1, 17], [10, 5]];
pub const TREE_POSITIONS: [[i32; 2]; 5] = [[-11, -5], [8, -8], [-14, -11], [-18, -2], [-14, 3]];

pub const TERRAIN_OBJECTS: [[i32; 2]; 13] = [
    [ 55, 54 ], [ 56, 54 ],   // House A
    [ 55, 55 ], [ 56, 55 ],   // House A
    [ 66, 57 ], [ 67, 57 ],   // House B
    [ 66, 56 ], [ 67, 56 ],   // House B
    [ 72, 65 ], [ 61, 73 ], [ 63, 77 ], [ 56, 70 ], [ 56, 74 ]  // Trees
];

pub const SMALL_HILLS: [[i32; 2]; 3] = [[4, 2], [20, -2], [-14, -6]];

pub const GAME_VERSION: &str = "v0.3.12";

pub const HUD_TEXTS: [&str; 15] = [GAME_VERSION, "Ammo 0", "Ammo 1", "Ammo 2", "Ammo 3",
  "Ammo 4", "Ammo 5", "Ammo 6",
  "Ammo 7", "Ammo 8", "Ammo 9", "Ammo 10",
  "Magazines 0/2", "Magazines 1/2", "Magazines 2/2"];

pub const CURRENT_AMMO_TEXT: &str = "Ammo 10";
pub const CURRENT_MAGAZINE_TEXT: &str = "Magazines 2/2";
