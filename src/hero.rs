use crate::creature::Creature;


#[derive(Clone, Debug)]
pub struct Hero {
    pub attack: usize,
    pub defence: usize,
    pub knowledge: usize,
    pub spell_power: usize,
    pub mana_max: usize,
    pub mana_cur: usize,
    pub creatures: Vec<Creature>,
}
impl Hero {
    pub fn new(attack: usize, defence: usize, knowledge: usize, spell_power: usize, mana_max: usize, mana_cur: usize) -> Self {
        let mut x: Vec<Creature> = vec![];
        let peasant = Creature::new("Peasant", 1, 1, 1, 2, 10, 10, 4, 1, 0, 1, false, true, 0, 0, false, 0);
        x.push(peasant);
        Self {
            attack,
            defence,
            knowledge,
            spell_power,
            mana_max, 
            mana_cur, 
            creatures: x,
        }
    }
    pub fn add_creature(&mut self, x: Creature) {
        if self.creatures.len() < 7 {
            self.creatures.push(x);
        }
    }
    pub fn default() -> Self{
        Self::new(1,1,1,1,10,10)
    }
    pub fn to_vec(&self) -> Vec<usize>{
        let mut temp = vec![];
        temp.push(self.mana_cur);
        temp.push(self.mana_max);
        temp.push(self.spell_power);
        temp.push(self.knowledge);
        temp.push(self.defence);
        temp.push(self.attack);
        temp

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
    list: Vec<Action>,
}
