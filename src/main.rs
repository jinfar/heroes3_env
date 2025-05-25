#![allow(dead_code, unused_mut)]
mod battle;
mod creature;
mod hero;
mod pole;
mod test;
use hero::{Action, Deistvie, Hero};
use pole::Pole;

const DEF_MULTI: f64 = 1.3;
const POLE_SIZE_X: usize = 15;
const POLE_SIZE_Y: usize = 11;

fn main() {
    println!("Hello, world!");
    let mut hero_a = Hero::default();
    let angel = creature::get_creature("Angel");
    hero_a.add_creature(angel.clone());
    let mut hero_d = Hero::default();
    hero_d.add_creature(angel);
    let map = Pole::default();
    let mut scena = battle::Battle::new(hero_a, hero_d, map);
    scena.init_battle();
    // dbg!(scena.return_actions().len());
    // dbg!(scena.return_enemies_vec().len());
    dbg!(scena.get_state());
    let someshite = scena
        .return_actions()
        .into_iter()
        .filter(|x| x.deistvie == Deistvie::Attack)
        .collect::<Vec<Action>>();
    dbg!(someshite.clone());
    dbg!(someshite.len());
}
