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
        let peasant = Creature::new("Peasant", 1, 1, 1, 2, 10, 10, 4, 1, 0, 1, false, 0, 0);
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
