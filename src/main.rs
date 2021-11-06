#![allow(dead_code, unused_mut)]
mod creature;
use creature::Creature;
mod hero;
use hero::Hero;

#[derive(Clone, Debug)]
struct Pole {
    field_x: usize,
    field_y: usize,
    bariers: Vec<[usize; 2]>,
}
#[derive(Clone, Debug)]
struct Battle{
    attacker: Hero,
    defender: Hero,
    map: Pole,
    queue: Vec<Creature>,
}

impl Battle{
    pub fn new(attacker: Hero, defender: Hero, map: Pole)->Self{
        let mut a = Self {
            attacker,
            defender,
            map,
            queue: vec![],
        };
        a.initial_queue();
        a
    }

    pub fn initial_queue(&mut self){
        self.queue = self.attacker.creatures.clone();
        self.queue.append(&mut self.defender.creatures);
        self.sort_queue();

    }
    pub fn sort_queue(&mut self){
        self.queue.sort_by_key(|k| k.speed);
    }
    pub fn check_dist(
        first_pos_x: usize,
        first_pos_y: usize,
        second_pos_x: usize,
        second_pos_y: usize,
    ) -> usize {
        let dist = (first_pos_x as i64 - second_pos_x as i64).abs()
            + (first_pos_y as i64 - second_pos_y as i64).abs();
        dist as usize
    }
    pub fn hod(&mut self, id: usize, pos_x: usize, pos_y: usize) -> usize {
        todo!("Move it, Move it");
    }
}

fn main() {
    println!("Hello, world!");
    let mut hero_a = Hero::new(1, 1, 0, 0);
    let angel = Creature::new("Angel", 20, 20, 50, 50, 150, 150, 17, 1, 0, 1, true, 0, 0);
    hero_a.add_creature(angel);
    let mut hero_d = Hero::new(1, 1, 0, 0);
    let mut map = Pole {
        field_x: 15,
        field_y: 11,
        bariers: vec![[5, 3]],
    };
    let mut scena = Battle::new(hero_a, hero_d, map); 
    dbg!(&scena.queue);
    dbg!(&scena.queue);
}
