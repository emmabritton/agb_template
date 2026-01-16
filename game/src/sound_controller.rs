use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};

pub struct SoundController<'gba> {
    mixer: Mixer<'gba>,
    bgm: Option<(Bgm, ChannelId)>,
    sfx_enabled: bool,
    bgm_enabled: bool,
}

impl <'gba> SoundController<'gba> {
    pub fn new(bgm_enabled: bool, sfx_enabled: bool, mixer: Mixer<'gba>) -> Self {
        Self { mixer, bgm: None, sfx_enabled, bgm_enabled }
    }
}

impl <'gba> SoundController<'gba> {
    pub fn frame(&mut self) {
        self.mixer.frame();
    }
    
    pub fn update_settings(&mut self, sfx_enabled: bool, bgm_enabled: bool) {
        self.sfx_enabled = sfx_enabled;
        self.bgm_enabled = bgm_enabled;

        if !self.bgm_enabled {
            self.stop_bgm();
        }
    }

    pub fn play_sfx(&mut self, effect: SoundEffect) {
        if !self.sfx_enabled {
            return;
        }

        let sound_data = match effect {
            SoundEffect::Cursor => {
                //SoundData for cursor
                unimplemented!()
            }
            SoundEffect::Select => {
                //SoundData for select
                unimplemented!()
            }
            SoundEffect::Close => {
                //SoundData for close
                unimplemented!()
            }
        };

        let mut channel = SoundChannel::new(sound_data);
        channel.stereo();
        self.mixer.play_sound(channel);
    }

    pub fn play_bgm(&mut self, track: Bgm) {
        if !self.bgm_enabled {
            return;
        }

        if let Some((current_track, _)) = &self.bgm {
            if *current_track == track {
                return;
            }
        }

        self.stop_bgm();

        //if implemented with wav bgm, otherwise use tracker

        // let mut channel = SoundChannel::new_high_priority(SoundData);
        //channel.should_loop().stereo();
        //let channel_id = self.mixer.play_sound(channel).expect("Failed to play BGM");

        // self.bgm = Some((track, channel_id));
    }

    fn stop_bgm(&mut self) {
        if let Some((_, channel_id)) = &self.bgm {
            if let Some(channel) = self.mixer.channel(channel_id) {
                channel.stop();
            }
            self.bgm = None;
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bgm {
    Menu
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SoundEffect {
    Cursor,
    Select,
    Close,
}

