pub const TILE_WIDTH: f32 = 46.0;

pub const TILES_PCS_W: usize = 64;
pub const TILES_PCS_H: usize = 64;

#[cfg_attr(feature = "cargo-clippy", allow(decimal_literal_representation))]
pub const TILE_MAP_BUF_LENGTH: usize = 4096;
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

//Assets
pub const ZOMBIE_JSON_PATH: &str = "assets/zombie.json";
pub const CHARACTER_JSON_PATH: &str = "assets/character.json";
pub const PISTOL_AUDIO_PATH: &str = "assets/audio/pistol.ogg";
pub const MAP_FILE_PATH: &str = "assets/maps/tilemap.tmx";

pub const RUN_SPRITE_OFFSET: usize = 64;
pub const ZOMBIE_STILL_SPRITE_OFFSET: usize = 32;
pub const NORMAL_DEATH_SPRITE_OFFSET: usize = 64;

pub const AMMO_POSITIONS: [[f32; 2]; 1] = [[-640.0, -560.0]];
pub const HOUSE_POSITIONS: [[f32; 2]; 2] = [[-36.0, 644.0], [506.0, 230.0]];
pub const TREE_POSITIONS: [[f32; 2]; 5] = [[-506.0, -230.0], [368.0, -368.0], [-690.0, -506.0], [-874.0, -92.0], [-690.0, 138.0]];

pub const TERRAIN_OBJECTS: [[usize; 2]; 34] = [
  [16, 18], [16, 19], [16, 20],
  [17, 18], [17, 19], [17, 20],
  [18, 18], [18, 19], [18, 20],
  [19, 18], [19, 19], [19, 20],
  [37, 15], [37, 16], [37, 17],
  [38, 15], [38, 16], [38, 17],
  [39, 15], [39, 16], [39, 17],
  [40, 15], [40, 16], [40, 17],
  [29, 49], [28, 50],
  [49, 33], [50, 34],
  [29, 59], [30, 60],
  [16, 54], [17, 55],
  [15, 45], [16, 46]];

pub const HUD_TEXTS: [&str; 15] = ["v0.3.8", "Ammo 0", "Ammo 1", "Ammo 2", "Ammo 3",
  "Ammo 4", "Ammo 5", "Ammo 6",
  "Ammo 7", "Ammo 8", "Ammo 9", "Ammo 10",
  "Magazines 0", "Magazines 1", "Magazines 2"];

pub const CURRENT_AMMO_TEXT: &str = "Ammo 10";
pub const CURRENT_MAGAZINE_TEXT: &str = "Magazines 2";
pub const VERSION_NUMBER_TEXT: &str = "v0.3.8";
