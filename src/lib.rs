#![deny(clippy::all)]

use napi_derive::napi;
use sdl2::mixer::{InitFlag, DEFAULT_CHANNELS, DEFAULT_FORMAT, DEFAULT_FREQUENCY};
use std::sync::{Arc, Once};

static INIT: Once = Once::new();
static mut SDL_CONTEXT: Option<SdlContext> = None;

struct SdlContext {
  _context: sdl2::Sdl,
  _audio: sdl2::AudioSubsystem,
}

#[napi]
pub struct Chunk {
  inner: Arc<sdl2::mixer::Chunk>,
}

#[napi]
pub struct Music {
  inner: Arc<sdl2::mixer::Music<'static>>,
}

#[napi]
pub struct Mixer {
  _marker: (),
}

fn init_sdl() -> napi::Result<()> {
  unsafe {
    INIT.call_once(|| {
      let context = sdl2::init().expect("Failed to initialize SDL");
      let audio = context.audio().expect("Failed to initialize audio");

      sdl2::mixer::open_audio(DEFAULT_FREQUENCY, DEFAULT_FORMAT, DEFAULT_CHANNELS, 1024)
        .expect("Failed to open audio");

      let flags = InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG | InitFlag::MID;
      let result = sdl2::mixer::init(flags);
      match result {
        Ok(_) => println!("SDL_mixer initialized with flags: {:?}", flags),
        Err(e) => println!("SDL_mixer init error: {}", e),
      }

      let (freq, format, channels) = sdl2::mixer::query_spec().expect("Failed to query audio spec");
      println!(
        "Audio specs - frequency: {}, format: {:?}, channels: {}",
        freq, format, channels
      );

      sdl2::mixer::allocate_channels(16);

      SDL_CONTEXT = Some(SdlContext {
        _context: context,
        _audio: audio,
      });
    });

    Ok(())
  }
}

#[napi]
impl Mixer {
  #[napi(constructor)]
  pub fn new() -> napi::Result<Self> {
    init_sdl()?;
    Ok(Mixer { _marker: () })
  }

  #[napi]
  pub fn load_wav(&self, path: String) -> napi::Result<Chunk> {
    let chunk = sdl2::mixer::Chunk::from_file(&path).map_err(|e| {
      println!("Error loading WAV: {}", e);
      napi::Error::from_reason(e.to_string())
    })?;
    Ok(Chunk {
      inner: Arc::new(chunk),
    })
  }

  #[napi]
  pub fn load_music(&self, path: String) -> napi::Result<Music> {
    let music = sdl2::mixer::Music::from_file(&path).map_err(|e| {
      println!("Error loading music: {}", e);
      napi::Error::from_reason(format!("Couldn't open '{}': {}", path, e))
    })?;
    let static_music = unsafe { std::mem::transmute(music) };
    Ok(Music {
      inner: Arc::new(static_music),
    })
  }

  #[napi]
  pub fn play_channel(&self, chunk: &Chunk, channel: i32, loops: i32) -> napi::Result<()> {
    sdl2::mixer::Channel(channel)
      .play(&chunk.inner, loops)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn play_music(&self, music: &Music, loops: i32) -> napi::Result<()> {
    music
      .inner
      .play(loops)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn halt_channel(&self, channel: i32) {
    sdl2::mixer::Channel(channel).halt();
  }

  #[napi]
  pub fn halt_music(&self) {
    sdl2::mixer::Music::halt();
  }

  #[napi]
  pub fn volume_music(&self, volume: i32) -> i32 {
    let current = sdl2::mixer::Music::get_volume();
    sdl2::mixer::Music::set_volume(volume);
    current
  }

  #[napi]
  pub fn volume_chunk(&self, channel: i32, volume: i32) -> i32 {
    sdl2::mixer::Channel(channel).set_volume(volume)
  }
}
