#![allow(dead_code)]

// use std::collections::HashMap;

// let mut slovar = HashMap::new();

// pub struct Slovar{
// peasant: ("Peasant, 0, 0, 1, 2, 1, 1, 4, 1, 0, 1, false, true, 0, 0"),
// }

pub fn get_creature(imya: &str) -> Creature {
    let x = match imya {
        "Angel" => Creature::new(
            "Angel", 20, 20, 50, 50, 150, 150, 17, 1, 0, 1, true, true, 0, 0, false),
        _ => Creature::new("Peasant", 0, 0, 1, 2, 1, 1, 4, 1, 0, 1, false, true, 0, 0, false),
    };
    x
}




#[derive(Clone, Debug)]
pub struct Creature {
    pub name: String,
    pub attack: usize,
    pub defence: usize,
    pub damage_min: usize,
    pub damage_max: usize,
    pub health_max: usize,
    pub health_current: usize,
    pub speed: usize,
    pub size: usize,
    pub shots: usize,
    pub amount: usize,
    pub is_flying: bool,
    pub is_attacers: bool,
    pub pol_x: usize,
    pub pol_y: usize,
    pub already_waited: bool,
}

impl Creature {
    pub fn new(
        name: &str,
        attack: usize,
        defence: usize,
        damage_min: usize,
        damage_max: usize,
        health_max: usize,
        health_current: usize,
        speed: usize,
        size: usize,
        shots: usize,
        amount: usize,
        is_flying: bool,
        is_attacers: bool,
        pol_x: usize,
        pol_y: usize,
        already_waited: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            attack,
            defence,
            damage_min,
            damage_max,
            health_max,
            health_current,
            speed,
            size,
            shots,
            amount,
            is_flying,
            is_attacers,
            pol_x,
            pol_y,
            already_waited,
        }
    }
}
impl Default for Creature {
    fn default() -> Self {
        Self {
            name: "Peasant".to_string(),
            attack: 0,
            defence: 0,
            damage_min: 1,
            damage_max: 2,
            health_max: 1,
            health_current: 1,
            speed: 4,
            size: 1,
            shots: 0,
            amount: 1,
            is_flying: false,
            is_attacers: true,
            pol_x: 0,
            pol_y: 0,
            already_waited: false,
        }
    } }
