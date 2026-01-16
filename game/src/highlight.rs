use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::{num, Num};
use crate::gfx::draw_button_highlight;
use crate::TILE_SIZE;

/// Highlight/focus indicator for button that animates between buttons
///
/// All methods work in tile coords
/// 
/// Usage:
/// Assuming buttons at 3,3 5,3 7,3
/// all 3x5
/// ```rust
/// let mut highlight = ButtonHighlight::new(3,3);
/// //player presses right
/// highlight.set_target(5,3);
/// 
/// //in update
/// highlight.update();
/// 
/// //in show
/// highlight.show(&mut frame, BUTTON_IMAGES, (3,5));
/// ```
/// 
/// See [draw_button_highlight] for BUTTON_IMAGES
/// 
/// (set `factor` to control animation speed)
pub struct ButtonHighlight {
    pos_x: Num<i32, 8>,
    pos_y: Num<i32, 8>,
    target_x: Num<i32, 8>,
    target_y: Num<i32, 8>,
}

impl ButtonHighlight {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            pos_x: Num::from(x as i32 * TILE_SIZE),
            pos_y: Num::from(y as i32 * TILE_SIZE),
            target_x: Num::from(x as i32 * TILE_SIZE),
            target_y: Num::from(y as i32 * TILE_SIZE),
        }
    }

    pub fn set_target(&mut self, x: u8, y: u8) {
        self.target_x = Num::from(x as i32 * TILE_SIZE);
        self.target_y = Num::from(y as i32 * TILE_SIZE);
    }

    pub fn update(&mut self) {
        let factor = num!(0.35);
        self.pos_x += (self.target_x - self.pos_x) * factor;
        self.pos_y += (self.target_y - self.pos_y) * factor;

        if (self.target_x - self.pos_x).abs() < Num::from_f32(0.5) {
            self.pos_x = self.target_x;
        }
        if (self.target_y - self.pos_y).abs() < Num::from_f32(0.5) {
            self.pos_y = self.target_y;
        }
    }

    pub fn show(
        &self,
        graphics: &mut GraphicsFrame,
        sprites: &mut [Object; 3],
        button_size: (u8, u8),
    ) {
        draw_button_highlight(
            (self.pos_x.round(), self.pos_y.round()),
            sprites,
            button_size,
            graphics,
        );
    }
}