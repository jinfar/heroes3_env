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
struct Battle {
    attacker: Hero,
    defender: Hero,
    map: Pole,
    queue: Vec<Creature>,
    next_move_queue: Vec<Creature>,
}

impl Default for Pole {
    fn default() -> Self {
        Self {
            field_x: 15,
            field_y: 11,
            bariers: vec![],
        }
    }
}
impl Battle {
    pub fn new(attacker: Hero, defender: Hero, map: Pole) -> Self {
        let mut temp = Self {
            attacker,
            defender,
            map,
            queue: vec![],
            next_move_queue: vec![],
        };
        temp.initial_queue();
        temp
    }
    pub fn initial_queue(&mut self) {
        let mut temp = vec![];
        let mut temp_d = vec![];
        let mut counter: usize = 0;
        // place creatures of attacker on the field
        for mut i in self.attacker.creatures.clone() {
            i.is_attacers = true;
            i.pol_x = 1;
            i.pol_y = counter;
            counter = if counter < 5 || counter > 6 {
                counter + 2
            } else {
                counter + 1
            };
            temp.push(i);
        }
        counter = 0;
        // place creatures of defender on the field
        for mut i in self.defender.creatures.clone() {
            i.is_attacers = false;
            i.pol_x = 15;
            i.pol_y = counter;
            counter = if counter < 5 || counter > 6 {
                counter + 2
            } else {
                counter + 1
            };
            temp_d.push(i);
        }
        // temp = temp.iter().map(|x| x.is_attacers = true).collect();
        self.queue = temp;
        self.queue.append(&mut temp_d);
        self.sort_queue();
    }
    pub fn sort_queue(&mut self) {
        self.queue.sort_by_key(|k| k.speed);
    }
    pub fn check_dist(
        &self,
        first_pos_x: usize,
        first_pos_y: usize,
        second_pos_x: usize,
        second_pos_y: usize,
    ) -> usize {
        dbg!(first_pos_x, first_pos_y, second_pos_x, second_pos_y);
        let dist = (first_pos_x as i64 - second_pos_x as i64).abs()
            + (first_pos_y as i64 - second_pos_y as i64).abs();
        dist as usize
    }
    pub fn hod(&mut self) {
        while (self.queue > 0) {
            todo!()
        }
    }
    pub fn wait(&mut self) {
        assert!(self.queue.len() > 0);
        let temp = self.queue.pop().unwrap();
        self.queue.insert(0, temp);
    }
    pub fn defend(&mut self) {
        // dbg!(self.queue.last().unwrap().defence);
        let mut temp = self.queue.pop().unwrap();
        temp.defence = (temp.defence as f64 * 1.3).floor() as usize;
        self.queue.push(temp);
        self.finish_creature_move();
    }
    fn finish_creature_move(&mut self) {
        let temp = self.queue.pop().unwrap();
        self.next_move_queue.push(temp);
    }
    fn moving(&mut self, pos_x: usize, pos_y: usize) {
        let mut temp = self.queue.pop().unwrap();
        let speed = temp.speed;
        let dist = self.check_dist(temp.pol_x, temp.pol_y, pos_x, pos_y);
        if dist <= speed {
            temp.pol_x = pos_x;
            temp.pol_y = pos_y;
            self.queue.push(temp);
            self.finish_creature_move();
        } else {
            self.queue.push(temp);
        }
        todo!("KAK ZAKANCHIVAT HOD?");
    }
}

fn main() {
    println!("Hello, world!");
    let mut hero_a = Hero::new(1, 1, 0, 0);
    let angel = creature::get_creature("Angel");
    hero_a.add_creature(angel);
    let mut hero_d = Hero::new(1, 1, 0, 0);
    let map = Pole::default();
    let mut scena = Battle::new(hero_a, hero_d, map);
    dbg!(&scena.queue);
    scena.moving(15, 11);
    dbg!(&scena.queue);
}
