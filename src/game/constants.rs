pub const TILE_WIDTH: f32 = 92.0;

pub const TILES_PCS_W: usize = 128;
pub const TILES_PCS_H: usize = 128;

pub const TILE_SIZE: f32 = 48.0;

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

pub const AMMO_POSITIONS: [[f32; 2]; 4] = [[-640.0, -560.0], [-700.0, 400.0], [750.0, -450.0], [-50.0, 550.0]];
pub const HOUSE_POSITIONS: [[f32; 2]; 2] = [[-36.0, 644.0], [506.0, 230.0]];
pub const TREE_POSITIONS: [[f32; 2]; 5] = [[-506.0, -230.0], [368.0, -368.0], [-690.0, -506.0], [-874.0, -92.0], [-690.0, 138.0]];

pub const TERRAIN_OBJECTS: [[i32; 2]; 34] = [[48, 50],
  [48, 51],
  [48, 52],
  [49, 50],
  [49, 51],
  [49, 52],
  [50, 50],
  [50, 51],
  [50, 52],
  [51, 50],
  [51, 51],
  [51, 52],
  [69, 47],
  [69, 48],
  [69, 49],
  [70, 47],
  [70, 48],
  [70, 49],
  [71, 47],
  [71, 48],
  [71, 49],
  [72, 47],
  [72, 48],
  [72, 49],
  [61, 81],
  [60, 82],
  [81, 65],
  [82, 66],
  [61, 91],
  [62, 92],
  [48, 86],
  [49, 87],
  [47, 77],
  [48, 78]];

pub const GAME_VERSION: &str = "v0.3.12";

pub const HUD_TEXTS: [&str; 15] = [GAME_VERSION, "Ammo 0", "Ammo 1", "Ammo 2", "Ammo 3",
  "Ammo 4", "Ammo 5", "Ammo 6",
  "Ammo 7", "Ammo 8", "Ammo 9", "Ammo 10",
  "Magazines 0/2", "Magazines 1/2", "Magazines 2/2"];

pub const CURRENT_AMMO_TEXT: &str = "Ammo 10";
pub const CURRENT_MAGAZINE_TEXT: &str = "Magazines 2/2";
