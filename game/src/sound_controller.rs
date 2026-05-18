use agb::fixnum::{num, Num};
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};

const VOL_OFF: usize = 0;

static VOLUMES: [Num<i16, 8>; 7] = [
    num!(0.0),
    num!(0.25),
    num!(0.5),
    num!(0.75),
    num!(1.0),
    num!(1.5),
    num!(2.0),
];

pub struct SoundController<'gba> {
    mixer: Mixer<'gba>,
    bgm: Option<(Bgm, ChannelId)>,
    sfx_volume: usize,
    bgm_volume: usize,
}

impl<'gba> SoundController<'gba> {
    pub fn new(sfx_vol: u8, bgm_vol: u8, mixer: Mixer<'gba>) -> Self {
        Self {
            mixer,
            bgm: None,
            sfx_volume: sfx_vol as usize,
            bgm_volume: bgm_vol as usize,
        }
    }
}

impl <'gba> SoundController<'gba> {
    pub fn frame(&mut self) {
        self.mixer.frame();
    }

    pub fn update_settings(&mut self, sfx_vol: u8, bgm_vol: u8) {
        self.sfx_volume = sfx_vol as usize;
        self.bgm_volume = bgm_vol as usize;

        if self.bgm_volume == VOL_OFF {
            self.stop_bgm();
        }
    }

    pub fn play_sfx(&mut self, effect: SoundEffect) {
        if self.sfx_volume == VOL_OFF {
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
        if self.bgm_volume == VOL_OFF {
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

