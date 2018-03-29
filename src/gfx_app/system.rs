use bullet;
use character;
use critter::CharacterSprite;
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::renderer::EncoderQueue;
use graphics::{DeltaTime, orientation::Stance};
use specs;
use specs::{Fetch, WriteStorage};
use std::time::Instant;
use terrain;
use terrain_object;
use terrain_object::TerrainTexture;
use zombie;

pub struct DrawSystem<D: gfx::Device> {
  render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
  terrain_system: terrain::TerrainDrawSystem<D::Resources>,
  character_system: character::CharacterDrawSystem<D::Resources>,
  zombie_system: zombie::ZombieDrawSystem<D::Resources>,
  bullet_system: bullet::BulletDrawSystem<D::Resources>,
  terrain_object_system: [terrain_object::TerrainObjectDrawSystem<D::Resources>; 2],
  encoder_queue: EncoderQueue<D>,
  game_time: Instant,
  frames: u32,
  cool_down: f64,
  fire_cool_down: f64,
}

impl<D: gfx::Device> DrawSystem<D> {
  pub fn new<F>(factory: &mut F,
                rtv: &gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                dsv: &gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
                encoder_queue: EncoderQueue<D>)
                -> DrawSystem<D>
                where F: gfx::Factory<D::Resources> {
    DrawSystem {
      render_target_view: rtv.clone(),
      depth_stencil_view: dsv.clone(),
      terrain_system: terrain::TerrainDrawSystem::new(factory, rtv.clone(), dsv.clone()),
      character_system: character::CharacterDrawSystem::new(factory, rtv.clone(), dsv.clone()),
      zombie_system: zombie::ZombieDrawSystem::new(factory, rtv.clone(), dsv.clone()),
      bullet_system: bullet::BulletDrawSystem::new(factory, rtv.clone(), dsv.clone()),
      terrain_object_system: [
        terrain_object::TerrainObjectDrawSystem::new(factory, rtv.clone(), dsv.clone(), TerrainTexture::House),
        terrain_object::TerrainObjectDrawSystem::new(factory, rtv.clone(), dsv.clone(), TerrainTexture::Tree)
      ],
      encoder_queue,
      game_time: Instant::now(),
      frames: 0,
      cool_down: 1.0,
      fire_cool_down: 1.0,
    }
  }
}

impl<'a, D> specs::System<'a> for DrawSystem<D>
  where D: gfx::Device,
        D::CommandBuffer: Send {
  #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
  type SystemData = (WriteStorage<'a, terrain::TerrainDrawable>,
                     WriteStorage<'a, character::CharacterDrawable>,
                     WriteStorage<'a, CharacterSprite>,
                     WriteStorage<'a, zombie::zombies::Zombies>,
                     WriteStorage<'a, bullet::bullets::Bullets>,
                     WriteStorage<'a, terrain_object::terrain_objects::TerrainObjects>,
                     Fetch<'a, DeltaTime>);

  fn run(&mut self, (mut terrain, mut character, mut character_sprite, mut zombies, mut bullets, mut terrain_objects, d): Self::SystemData) {
    use specs::Join;
    let mut encoder = self.encoder_queue.receiver.recv().unwrap();

    let delta = d.0;

    if self.cool_down == 0.0 {
      self.cool_down += 0.1;
    }
    if self.fire_cool_down == 0.0 {
      self.fire_cool_down += 0.2;
    }
    self.cool_down = (self.cool_down - delta).max(0.0);
    self.fire_cool_down = (self.fire_cool_down - delta).max(0.0);

    let current_time = Instant::now();
    self.frames += 1;
    if cfg!(feature = "fps") && (current_time.duration_since(self.game_time).as_secs()) >= 1 {
      println!("{:?} ms/frames", 1000.0 / f64::from(self.frames));
      self.frames = 0;
      self.game_time = Instant::now();
    }

    encoder.clear(&self.render_target_view, [16.0 / 256.0, 16.0 / 256.0, 20.0 / 256.0, 1.0]);
    encoder.clear_depth(&self.depth_stencil_view, 1.0);

    for (t, c, cs, zs, bs, obj) in (&mut terrain, &mut character, &mut character_sprite, &mut zombies, &mut bullets, &mut terrain_objects).join() {
      self.terrain_system.draw(t, &mut encoder);

      if self.cool_down == 0.0 {
        if c.stance == Stance::Walking {
          cs.update_run();
        }
        for mut z in &mut zs.zombies {
          match z.stance {
            Stance::NormalDeath => z.update_normal_death(),
            Stance::CriticalDeath => z.update_critical_death(),
            Stance::Walking => z.update_walk(),
            Stance::Still => z.update_still(),
            _ => ()
          };
        }
      } else if self.fire_cool_down == 0.0 && c.stance == Stance::Firing {
        cs.update_fire();
      }
      for mut z in &mut zs.zombies {
        if c.position.position[1] <= z.position.position[1] {
          self.zombie_system.draw(&mut z, &mut encoder);
        }
      }
      for (idx, o) in &mut obj.objects.iter().enumerate() {
        if idx < 2 && c.position.position[1] <= o.position.position[1] {
          self.terrain_object_system[0].draw(o, &mut encoder);
        }
        else if idx > 1 && c.position.position[1] <= o.position.position[1] {
          self.terrain_object_system[1].draw(o, &mut encoder);
        }
      }
      self.character_system.draw(c, cs, &mut encoder);

      for (idx, o) in &mut obj.objects.iter().enumerate() {
        if idx < 2 && c.position.position[1] > o.position.position[1] {
          self.terrain_object_system[0].draw(o, &mut encoder);
        }
        else if idx > 1 && c.position.position[1] > o.position.position[1] {
          self.terrain_object_system[1].draw(o, &mut encoder);
        }
      }

      for mut z in &mut zs.zombies {
        if c.position.position[1] > z.position.position[1] {
          self.zombie_system.draw(&mut z, &mut encoder);
        }
      }

      for b in &mut bs.bullets {
        self.bullet_system.draw(b, &mut encoder);
      }
    }

    if let Err(e) = self.encoder_queue.sender.send(encoder) {
      panic!("Disconnected, cannot return encoder to mpsc: {}", e);
    };
  }
}
