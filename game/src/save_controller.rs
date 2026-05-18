use agb::eprintln;
use agb::save::{SaveError, SaveSlotManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameData {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub sfx_volume: u8,
    pub bgm_volume: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sfx_volume: 5,
            bgm_volume: 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveBlock {
    settings: Settings,
    save: Option<GameData>,
}

impl Default for SaveBlock {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            save: None,
        }
    }
}

pub struct SaveController {
    manager: Option<SaveSlotManager<()>>,
    block: SaveBlock,
    corrupted: bool,
}

impl SaveController {
    pub fn new(mut manager: SaveSlotManager<()>) -> Self {
        let (block, manager, corrupted) = match manager.read(0) {
            Ok(save) => (save, Some(manager), false),
            Err(e) => match e {
                SaveError::SlotEmpty => (SaveBlock::default(), Some(manager), false),
                SaveError::SlotCorrupted => (SaveBlock::default(), Some(manager), true),
                _ => {
                    eprintln!("Error reading save block: {e:?}");
                    (SaveBlock::default(), None, false)
                }
            },
        };

        Self {
            manager,
            block,
            corrupted,
        }
    }

    pub fn new_broken() -> Self {
        Self {
            block: SaveBlock::default(),
            manager: None,
            corrupted: false,
        }
    }
}

impl SaveController {
    fn save(&mut self) {
        if let Some(manager) = self.manager.as_mut()
            && let Err(e) = manager.write(0, &self.block, &())
        {
            eprintln!("Error saving: {e:?}");
            self.corrupted = true;
        }
        self.corrupted = false;
    }
}

impl SaveController {
    pub fn save_game(&mut self, data: GameData) {
        self.block.save = Some(data);
        self.save();
    }

    pub fn save_settings(&mut self, settings: Settings) {
        self.block.settings = settings;
        self.save();
    }

    pub fn settings(&self) -> Settings {
        self.block.settings.clone()
    }

    pub fn game_save_data(&self) -> Option<&GameData> {
        self.block.save.as_ref()
    }

    pub fn is_working(&self) -> bool {
        self.manager.is_some()
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }
}
