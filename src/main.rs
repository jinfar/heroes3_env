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

impl Default for Pole{
    fn default() -> Self {
        Self{
            field_x: 15,
            field_y: 11,
            bariers: vec![],
        }
    }
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
        let mut temp = vec![];
        let mut temp_d = vec![];
        let mut counter: usize = 0;
        for mut i in self.attacker.creatures.clone(){
            i.is_attacers = true;
            i.pol_x = 1;
            i.pol_y = counter;
            counter = if counter<5 || counter>6 {counter + 2} else {counter +1}; 
            temp.push(i);
        }
        counter = 0;
        for mut i in self.defender.creatures.clone(){
            i.is_attacers = false;
            i.pol_x = 15;
            i.pol_y = counter;
            counter = if counter<5 || counter>6 {counter + 2} else {counter +1}; 
            temp_d.push(i);
        }
        // temp = temp.iter().map(|x| x.is_attacers = true).collect();
        self.queue = temp;
        self.queue.append(&mut temp_d);
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
    pub fn wait(&mut self){
        assert!(self.queue.len() > 0);
        let temp = self.queue.pop().unwrap();
        self.queue.insert(0,temp);
    }
    pub fn defend(&mut self){
        // dbg!(self.queue.last().unwrap().defence);
        let mut temp = self.queue.pop().unwrap();
        temp.defence= (temp.defence as f64 * 1.3).floor() as usize;
        self.queue.push(temp);
    }
}

fn main() {
    println!("Hello, world!");
    let mut hero_a = Hero::new(1, 1, 0, 0);
    let angel = creature::get_creature("Angel");
    hero_a.add_creature(angel);
    let mut hero_d = Hero::new(1, 1, 0, 0);
    let mut map = Pole {
        field_x: 15,
        field_y: 11,
        bariers: vec![[5, 3]],
    };
    let mut scena = Battle::new(hero_a, hero_d, map); 
    let map2 = Pole::default();
    dbg!(map2);
    // dbg!(&scena.queue);
    scena.defend();
    // dbg!(&scena.queue);
}
