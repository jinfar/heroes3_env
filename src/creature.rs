#![allow(dead_code)]

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
    pub pol_x: usize,
    pub pol_y: usize,
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
        pol_x: usize,
        pol_y: usize,
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
            pol_x,
            pol_y,
        }
    }
}
