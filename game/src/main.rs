#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod printer;
mod rng;
mod highlight;
mod sound_controller;
mod gfx;
mod save_controller;

use crate::printer::VariWidthType;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{Graphics, Priority};
use agb::eprintln;
use agb::fixnum::vec2;
use agb::input::ButtonController;
use agb::sound::mixer::{Frequency, Mixer};
use resources::bg;
use resources::prelude::*;
use crate::save_controller::SaveController;
use crate::sound_controller::SoundController;

extern crate alloc;

#[cfg(all(feature = "sram", feature = "flash64"))]
compile_error!("Features `sram` and `flash64` are mutually exclusive. Enable only one.");

const SAVE_MAGIC: [u8; 32] = *b"????????????????????????????????";

const TILE_SIZE: i32 = 8;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    #[cfg(feature = "sram")]
    let save_controller = match gba.save.init_sram(1, SAVE_MAGIC) {
        Ok(save_manager) => SaveController::new(save_manager),
        Err(e) => {
            eprintln!("Error init'ing save manager: {e:?}");
            SaveController::new_broken()
        }
    };

    #[cfg(feature = "flash64")]
    let save_controller = match gba.save.init_flash_64k(1, crate::SAVE_MAGIC) {
        Ok(save_manager) => SaveController::new(save_manager),
        Err(e) => {
            eprintln!("Error init'ing save manager: {e:?}");
            SaveController::new_broken()
        }
    };

    let mixer = gba.mixer.mixer(Frequency::Hz18157);
    let gfx = gba.graphics.get();
    let button_controller = ButtonController::new();

    run(mixer, gfx, button_controller, save_controller);
}

fn run(mixer: Mixer, mut gfx: Graphics, mut button_controller: ButtonController, save_controller: SaveController) -> ! {
    VRAM_MANAGER.set_background_palettes(bg::PALETTES);

    let settings = save_controller.settings();
    let mut sound_controller =
        SoundController::new(settings.sfx_volume, settings.bgm_volume, mixer);

    let mut background = RegularBackground::new(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    for y in 0..20 {
        for x in 0..40 {
            background.set_tile(
                vec2(x, y),
                &bg::main.tiles,
                bg::main.tile_settings[bg_idx::WHITE],
            );
        }
    }

    let text = VariWidthType::new("Hello, GBA!");

    loop {
        let mut frame = gfx.frame();
        button_controller.update();

        //game here

        //example
        background.show(&mut frame);
        text.show(vec2(16, 16), &mut frame);
        //end example

        sound_controller.frame();
        frame.commit();
    }
}

//test with `cargo test --package game`
#[cfg(test)]
mod test {
    use agb::Gba;

    #[test_case]
    fn test_example_bin(_gba: &mut Gba) {
        assert_eq!(2 + 2, 4);
    }
}