pub const TILE_WIDTH: f32 = 46.0;

pub const TILES_PCS_W: usize = 256;
pub const TILES_PCS_H: usize = 256;

pub const TILE_MAP_BUF_LENGTH: usize = 65536;
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
pub const MAP_A_FILE_PATH: &str = "assets/maps/tilemapa.tmx";
pub const MAP_B_FILE_PATH: &str = "assets/maps/tilemapb.tmx";

pub const RUN_SPRITE_OFFSET: usize = 64;
pub const ZOMBIE_STILL_SPRITE_OFFSET: usize = 32;
pub const NORMAL_DEATH_SPRITE_OFFSET: usize = 64;

pub const AMMO_POSITIONS: [[f32; 2]; 4] = [[-640.0, -560.0], [-700.0, 400.0], [750.0, -450.0], [-50.0, 550.0]];
pub const HOUSE_POSITIONS: [[f32; 2]; 2] = [[-36.0, 644.0], [506.0, 230.0]];
pub const TREE_POSITIONS: [[f32; 2]; 5] = [[-506.0, -230.0], [368.0, -368.0], [-690.0, -506.0], [-874.0, -92.0], [-690.0, 138.0]];

pub const TERRAIN_OBJECTS: [[usize; 2]; 34] = [
  [ 112, 114 ],
  [ 112, 115 ],
  [ 112, 116 ],
  [ 113, 114 ],
  [ 113, 115 ],
  [ 113, 116 ],
  [ 114, 114 ],
  [ 114, 115 ],
  [ 114, 116 ],
  [ 115, 114 ],
  [ 115, 115 ],
  [ 115, 116 ],
  [ 133, 111 ],
  [ 133, 112 ],
  [ 133, 113 ],
  [ 134, 111 ],
  [ 134, 112 ],
  [ 134, 113 ],
  [ 135, 111 ],
  [ 135, 112 ],
  [ 135, 113 ],
  [ 136, 111 ],
  [ 136, 112 ],
  [ 136, 113 ],
  [ 125, 145 ],
  [ 124, 146 ],
  [ 145, 129 ],
  [ 146, 130 ],
  [ 125, 155 ],
  [ 126, 156 ],
  [ 112, 150 ],
  [ 113, 151 ],
  [ 111, 141 ],
  [ 112, 142 ]];

pub const HUD_TEXTS: [&str; 15] = ["v0.3.9", "Ammo 0", "Ammo 1", "Ammo 2", "Ammo 3",
  "Ammo 4", "Ammo 5", "Ammo 6",
  "Ammo 7", "Ammo 8", "Ammo 9", "Ammo 10",
  "Magazines 0", "Magazines 1", "Magazines 2"];

pub const CURRENT_AMMO_TEXT: &str = "Ammo 10";
pub const CURRENT_MAGAZINE_TEXT: &str = "Magazines 2";
pub const VERSION_NUMBER_TEXT: &str = "v0.3.9";
