use num_derive::ToPrimitive;
use pumpkin_macros::packet;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize, Clone)]
#[packet(0x03)]
pub struct CEntityAnimation {
    entity_id: VarInt,
    /// See `Animation`
    animation: u8,
}

impl CEntityAnimation {
    pub fn new(entity_id: VarInt, animation: u8) -> Self {
        Self {
            entity_id,
            animation,
        }
    }
}

#[derive(ToPrimitive)]
pub enum Animation {
    SwingMainArm,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticaleffect,
}
