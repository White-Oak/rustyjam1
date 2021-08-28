use std::fmt::Display;

use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct PlayerStatsMods {
    pub light_radius: f32,
    pub area_of_effect: f32,
    pub duration: f32,
    pub movement_speed: f32,
    pub cooldown_reduction: f32,
}

pub struct Item {
    pub name: String,
    pub slot: Slot,
    pub mods: Vec<Mod>,
}

pub struct Mod {
    pub kind: ModKind,
    pub value: f32,
}

impl Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% {}", (self.value * 100.) as i32, self.kind.suffix())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ModKind {
    LightRadius,
    AreaOfEffect,
    Duration,
    MovementSpeed,
    CooldownReduction,
}

impl ModKind {
    fn suffix(&self) -> &'static str {
        match self {
            ModKind::LightRadius => "increased light radius",
            ModKind::AreaOfEffect => "increased area of effect",
            ModKind::Duration => "increased duration",
            ModKind::MovementSpeed => "increased movement speed",
            ModKind::CooldownReduction => "increased cooldown reduction",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Slot {
    Head,
    Cloak,
    Lockpick,
    Boots,
}

impl Default for Slot {
    fn default() -> Self {
        Slot::Head
    }
}

pub struct SlotItems {
    slot: Slot,
    equipped: usize,
    available: Vec<Item>,
}

impl SlotItems {
    pub fn equipped(&self) -> &Item {
        &self.available[self.equipped]
    }
}

pub struct PlayerItems {
    pub head: SlotItems,
    pub cloak: SlotItems,
    pub lockpick: SlotItems,
    pub boots: SlotItems,
}

impl PlayerItems {
    pub fn stats(&self) -> PlayerStatsMods {
        let mut stats = PlayerStatsMods::default();
        let slots = [&self.head, &self.cloak, &self.lockpick, &self.boots];
        for slot in slots {
            for a_mod in &slot.equipped().mods {
                match a_mod.kind {
                    ModKind::LightRadius => stats.light_radius += a_mod.value,
                    ModKind::AreaOfEffect => stats.area_of_effect += a_mod.value,
                    ModKind::Duration => stats.duration += a_mod.value,
                    ModKind::MovementSpeed => stats.movement_speed += a_mod.value,
                    ModKind::CooldownReduction => stats.cooldown_reduction += a_mod.value,
                }
            }
        }
        stats
    }
}

impl Default for PlayerItems {
    fn default() -> Self {
        let head = Item {
            name: "Mask".to_string(),
            slot: Slot::Head,
            mods: vec![Mod {
                kind: ModKind::LightRadius,
                value: 0.33,
            }
            , Mod {
                kind: ModKind::AreaOfEffect,
                value: 0.5
            }
            , Mod {
                kind: ModKind::Duration,
                value: 0.5
            }
            , Mod {
                kind: ModKind::MovementSpeed,
                value: 0.5
            }
            ],
        };
        let cloak = Item {
            name: "Cloak".to_string(),
            slot: Slot::Cloak,
            mods: vec![Mod {
                kind: ModKind::LightRadius,
                value: 0.33,
            }],
        };
        let lockpick = Item {
            name: "Lockpick".to_string(),
            slot: Slot::Lockpick,
            mods: vec![Mod {
                kind: ModKind::LightRadius,
                value: 0.33,
            }],
        };
        let boots = Item {
            name: "Boots".to_string(),
            slot: Slot::Boots,
            mods: vec![Mod {
                kind: ModKind::LightRadius,
                value: 0.33,
            }],
        };
        let head = SlotItems {
            slot: Slot::Head,
            equipped: 0,
            available: vec![head],
        };
        let cloak = SlotItems {
            slot: Slot::Cloak,
            equipped: 0,
            available: vec![cloak],
        };
        let lockpick = SlotItems {
            slot: Slot::Lockpick,
            equipped: 0,
            available: vec![lockpick],
        };
        let boots = SlotItems {
            slot: Slot::Boots,
            equipped: 0,
            available: vec![boots],
        };
        PlayerItems {
            head,
            cloak,
            lockpick,
            boots,
        }
    }
}
