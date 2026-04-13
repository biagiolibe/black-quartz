use crate::animation::DrillAnimation;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
#[require(
    Inventory,
    Health,
    Fuel,
    FieldOfView,
    DrillState,
    PlayerAttributes,
    Currency,
    DrillAnimation,
    PlayerDirection
)]
pub struct Player;

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct PlayerAttributes {
    pub drill_power: f32,
    pub damage_factor: f32,
    pub armor_resistance: f32,
    pub ground_speed_factor: f32,
    pub flying_speed_factor: f32,
    pub fuel_efficiency: f32,
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        Self {
            drill_power: 1.0,
            damage_factor: 0.05,
            armor_resistance: 1.0,
            ground_speed_factor: 200.0,
            flying_speed_factor: 200.0,
            fuel_efficiency: 0.3,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        PlayerDirection::Right
    }
}

#[derive(Event)]
pub struct PlayerImpactEvent {
    pub impact_speed: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    #[allow(dead_code)]
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component)]
pub struct Fuel {
    pub current: f32,
    pub max: f32,
}

impl Default for Fuel {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum DrillState {
    #[default]
    Idle,
    Flying,
    Drilling,
    Falling,
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<(i32, i32)>,
    pub radius: i32,
    pub dirty: bool,
}

impl Default for FieldOfView {
    fn default() -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: 10,
            dirty: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: usize,
    pub value: u32,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 10,
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, new_item: Item) {
        if self.size() + new_item.quantity <= self.capacity {
            if let Some(existing) = self.items.iter_mut().find(|i| i.id == new_item.id) {
                existing.quantity += new_item.quantity;
            } else {
                self.items.push(new_item);
            }
        } else {
            info!("Inventory full!");
        }
    }

    pub fn size(&self) -> usize {
        self.items.iter().map(|i| i.quantity).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn print_items(&self) -> String {
        self.items
            .iter()
            .map(|i| format!("{} x{}", i.name, i.quantity))
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[derive(Component, Debug)]
pub struct Currency {
    pub amount: u32,
}

impl Default for Currency {
    fn default() -> Self {
        Self { amount: 100 }
    }
}

impl Currency {
    pub fn add_amount(&mut self, amount: u32) {
        self.amount += amount;
    }
}
