use std::{fmt::Display, fs::File, io::Read, iter::{once, repeat}, ops::Range};

use bevy::{log, prelude::*};
use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct PlayerStatsMods {
    pub light_radius: f32,
    pub area_of_effect: f32,
    pub duration: f32,
    pub movement_speed: f32,
    pub cooldown_reduction: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub slot: Slot,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mod {
    pub kind: ModKind,
    pub value: f32,
}

impl Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}% {}", (self.value * 100.) as i32, self.kind.suffix())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModKind {
    LightRadius,
    AreaOfEffect,
    Duration,
    MovementSpeed,
    CooldownReduction,
}

impl Default for ModKind {
    fn default() -> Self {
        ModKind::AreaOfEffect
    }
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

    fn range(&self) -> Range<f32> {
        match self {
            ModKind::LightRadius => 0.1..0.33,
            ModKind::AreaOfEffect => 0.1..0.33,
            ModKind::Duration => 0.1..0.2,
            ModKind::MovementSpeed => 0.05..0.2,
            ModKind::CooldownReduction => 0.1..0.3,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Slot {
    Head,
    Cloak,
    Lockpick,
    Boots,
}

impl Slot {
    fn common_name(&self) -> String {
        match self {
            Slot::Head => "Mask".to_string(),
            Slot::Cloak => "Cloak".to_string(),
            Slot::Lockpick => "Lockpick".to_string(),
            Slot::Boots => "Boots".to_string(),
        }
    }

    fn magic_name(&self) -> String {
        match self {
            Slot::Head => "Magic Mask".to_string(),
            Slot::Cloak => "Magic Cloak".to_string(),
            Slot::Lockpick => "Magic Lockpick".to_string(),
            Slot::Boots => "Magic Boots".to_string(),
        }
    }

    fn rare_name(&self) -> String {
        match self {
            Slot::Head => "Great Mask".to_string(),
            Slot::Cloak => "Great Cloak".to_string(),
            Slot::Lockpick => "Great Lockpick".to_string(),
            Slot::Boots => "Great Boots".to_string(),
        }
    }
}

impl Default for Slot {
    fn default() -> Self {
        Slot::Head
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotItems {
    pub slot: Slot,
    pub equipped: usize,
    pub available: Vec<Item>,
}

impl SlotItems {
    pub fn equipped(&self) -> &Item {
        &self.available[self.equipped]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn all_equipped_items(&self) -> impl Iterator<Item = &Item> {
        vec![
            self.head.equipped(),
            self.cloak.equipped(),
            self.lockpick.equipped(),
            self.boots.equipped(),
        ]
        .into_iter()
    }

    pub fn slot_items(&self, slot: Slot) -> &SlotItems {
        match slot {
            Slot::Head => &self.head,
            Slot::Cloak => &self.cloak,
            Slot::Lockpick => &self.lockpick,
            Slot::Boots => &self.boots,
        }
    }

    pub fn slot_items_mut(&mut self, slot: Slot) -> &mut SlotItems {
        match slot {
            Slot::Head => &mut self.head,
            Slot::Cloak => &mut self.cloak,
            Slot::Lockpick => &mut self.lockpick,
            Slot::Boots => &mut self.boots,
        }
    }

    pub fn all_available_items_for_slot(&self, slot: Slot) -> impl Iterator<Item = &Item> {
        let slot_items = self.slot_items(slot);
        slot_items.available.iter()
    }

    pub fn all_equipped_mods(&self) -> impl Iterator<Item = Mod> + '_ {
        let grouped = self
            .all_equipped_items()
            .flat_map(|item| item.mods.clone())
            .into_grouping_map_by(|a_mod| a_mod.kind);
        grouped
            .fold(Mod::default(), |mut acc, kind, next| {
                // fold_first(|mut acc, _, next| {
                acc.kind = *kind;
                acc.value += next.value;
                acc
            })
            .into_values()
    }

    pub fn delete_for_slot(&mut self, slot: Slot, index: usize) {
        let mut slot_items = self.slot_items_mut(slot);
        if index == slot_items.equipped {
            log::warn!("tried to delete what's equipped");
            return;
        }
        if index < slot_items.equipped {
            slot_items.equipped -= 1;
        }
        slot_items.available.remove(index);
    }

    pub fn equip_on_slot(&mut self, slot: Slot, index: usize) {
        self.slot_items_mut(slot).equipped = index;
    }
}

const RARE_CHANCE: f32 = 0.9;
const MAGIC_CHANCE: f32 = 0.6;

const KINDS: [ModKind; 5] = [
    ModKind::MovementSpeed,
    ModKind::LightRadius,
    ModKind::Duration,
    ModKind::CooldownReduction,
    ModKind::AreaOfEffect,
];

const SLOTS: [Slot; 4] = [Slot::Head, Slot::Cloak, Slot::Lockpick, Slot::Boots];

pub fn generate() -> Vec<Item> {
    let mut items = vec![];
    let mut rng = thread_rng();
    for _ in 0..3 {
        let mut kinds = 1;
        let r: f32 = rng.gen();
        if r > RARE_CHANCE {
            kinds += 2;
        }
        if r > MAGIC_CHANCE {
            kinds += 1;
        }
        let slot = *SLOTS
            .choose_multiple(&mut rng, 1)
            .next()
            .expect("no slots generated");
        let name = match kinds {
            1 => slot.common_name(),
            2 => slot.magic_name(),
            4 => slot.rare_name(),
            _ => todo!(),
        };
        let mods = KINDS
            .choose_multiple(&mut rng, kinds)
            .map(|kind| Mod {
                value: rng.gen_range(kind.range()),
                kind: *kind,
            })
            .collect_vec();
        let item = Item { name, slot, mods };
        items.push(item);
    }
    items
}

impl FromWorld for PlayerItems {
    fn from_world(_world: &mut World) -> Self {
        match File::open("save.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("cant read a save");
                serde_json::from_str(&contents).expect("cant deserialize a save")
            }
            Err(_) => default_items(),
        }
    }
}

fn default_items() -> PlayerItems {
    let bad_head = Item {
        name: "Mask".to_string(),
        slot: Slot::Head,
        mods: vec![Mod {
            kind: ModKind::AreaOfEffect,
            value: 0.5,
        }],
    };
    let head = Item {
        name: "Mask".to_string(),
        slot: Slot::Head,
        mods: vec![Mod {
            kind: ModKind::LightRadius,
            value: 0.33,
        }],
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
        available: once(bad_head).chain(once(head)).collect_vec(),
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
