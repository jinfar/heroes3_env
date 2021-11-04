
#![allow(dead_code, unused_mut)]


#[derive(Clone, Debug)]
pub struct Creature{
    name:   String,
    attack: usize,
    defence:    usize, 
    damage_min: usize,
    damage_max: usize,
    health_max: usize,
    health_current: usize,
    speed:  usize,
    size:   usize,
    shots:  usize,
    amount: usize,
    is_flying:  bool,
}


impl Creature {
    pub fn new(name: &str, 
               attack: usize, 
               defence: usize,
               damage_min: usize, 
               damage_max: usize, 
               health_max: usize, 
               health_current: usize, 
               speed:  usize, 
               size:   usize, 
               shots:  usize, 
               amount: usize, 
               is_flying:  bool, ) -> Self{
        Self{ 
            name: name.to_string(),
            attack: attack,
            defence: defence,
            damage_min: damage_min,
            damage_max: damage_max,
            health_max: health_max,
            health_current: health_current,
            speed: speed,
            size: size,
            shots: shots,
            amount: amount,
            is_flying: is_flying,
        }
    }
}
#[derive(Clone, Debug)]
struct Pole{
    field_x:    usize,
    field_y:    usize,
    bariers:    Vec<[usize;2]>
}
#[derive(Clone, Debug)]
struct Hero{
    attack: usize,
    defence:    usize, 
    knowledge:  usize,
    spell_power:    usize,
    creatures:  Vec<Creature>,
}
impl Hero{
    pub fn new(attack: usize, defence: usize, knowledge: usize, spell_power: usize, ) -> Self{
        let mut x: Vec<Creature> = vec![];
        let peasant = Creature::new("Peasant", 1, 1, 1, 2, 10, 10, 4, 1, 0, 1, false);
        x.push(peasant);
        Self{
            attack:  attack,
            defence: defence,
            knowledge: knowledge,
            spell_power: spell_power,
            creatures: x, 
        }
    }
    pub fn add_creature(&mut self, x: Creature){
        if self.creatures.len() < 7{
            self.creatures.push(x);
        }
    }
}
#[derive(Clone, Debug)]
struct Battle{
    attacker:   Hero,
    defender:   Hero,
    map:    Pole,
    queue:  Vec<Creature>,
}

impl Battle {
    pub fn make_queue(&mut self){
        let mut temp = self.attacker.creatures.clone();
        temp.append(&mut self.defender.creatures);
        temp.sort_by_key(|k| k.speed);
        self.queue = temp;
    }

}







fn main() {
    println!("Hello, world!");
    let mut hero_a = Hero::new(1,1,0,0);
    let angel = Creature::new("Angel", 20, 20, 50, 50, 150, 150, 17, 1, 0, 1, true);
    hero_a.add_creature(angel);
    let mut hero_d = Hero::new(1,1,0,0);
    let mut map = Pole{ field_x: 15, field_y: 11,  bariers: vec![[5,3]]};
    let mut scena = Battle{ attacker: hero_a, defender: hero_d, map: map, queue: vec![] } ;
    dbg!(&scena.queue);
    scena.make_queue();
    dbg!(&scena.queue);

}
