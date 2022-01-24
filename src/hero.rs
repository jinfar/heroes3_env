use crate::creature::Creature;

#[derive(Clone, Debug)]
pub struct Hero {
    pub attack: usize,
    pub defence: usize,
    pub knowledge: usize,
    pub spell_power: usize,
    pub creatures: Vec<Creature>,
}
impl Hero {
    pub fn new(attack: usize, defence: usize, knowledge: usize, spell_power: usize) -> Self {
        let mut x: Vec<Creature> = vec![];
        let peasant = Creature::new("Peasant", 1, 1, 1, 2, 10, 10, 4, 1, 0, 1, false, true, 0, 0, false);
        x.push(peasant);
        Self {
            attack,
            defence,
            knowledge,
            spell_power,
            creatures: x,
        }
    }
    pub fn add_creature(&mut self, x: Creature) {
        if self.creatures.len() < 7 {
            self.creatures.push(x);
        }
    }
}
#[derive(Clone, Debug)]
pub enum Deistvie {
   Move,
   Attack,
   Defence,
   Wait,
   SpecialAbility

}

#[derive(Clone, Debug)]
pub struct Action{
    pub deistvie: Deistvie,
    pub target: [usize; 2] 
}
impl Action{
    pub fn new(dei: Deistvie, tar: [usize; 2]) -> Self {
        Self{
           deistvie: dei,
           target: tar,
        }
    }
}

pub struct PosibbleActions{
    list: Vec<Action>
}
