use agb::external::portable_atomic::AtomicU32;
use core::sync::atomic::Ordering;

#[unsafe(link_section = ".ewram.achievements")]
static ACHIEVEMENTS: AtomicU32 = AtomicU32::new(0);

#[repr(usize)]
pub enum Achievement {
    CollectedPrincess,
    ReachedMillionWithoutPrincess,
}

pub fn set_achievement(achievement: Achievement) {
    let bit = achievement as usize;
    ACHIEVEMENTS.fetch_or(1 << bit, Ordering::SeqCst);
}
