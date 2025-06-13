use crate::creature::Creature;
use crate::hero::*;
use crate::pole::Pole;

use crate::DEF_MULTI;
use crate::POLE_SIZE_X;
use crate::POLE_SIZE_Y;

#[derive(Clone, Debug)]
pub struct Battle {
    pub attacker: Hero,
    pub defender: Hero,
    pub map: Pole,
    pub queue: Vec<Creature>,
    pub next_move_queue: Vec<Creature>,
    pub current_unit: Creature,
}

impl Battle {
    pub fn new(attacker: Hero, defender: Hero, map: Pole) -> Self {
        let mut temp = Self {
            attacker,
            defender,
            map,
            queue: vec![],
            next_move_queue: vec![],
            current_unit: Creature::default(),
        };
        // temp.render();
        temp
    }
    pub fn initial_queue(&mut self) {
        // put units on the battle field
        // and create move queue
        let mut temp = vec![];
        let mut temp_d = vec![];
        let mut counter: usize = 0;
        // place creatures of attacker on the field
        let mut idx: usize = 0;
        for mut i in self.attacker.creatures.clone() {
            idx = idx + 1;
            i.is_attacers = true;
            i.pol_x = 1;
            i.pol_y = counter;
            i.id = idx;
            counter = if counter < 5 || counter > 6 {
                counter + 2
            } else {
                counter + 1
            };
            temp.push(i);
        }

        counter = 0;
        idx = 7;
        // place creatures of defender on the field
        for mut i in self.defender.creatures.clone() {
            idx = idx + 1;
            i.is_attacers = false;
            i.pol_x = 15;
            i.pol_y = counter;
            i.id = idx;
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
    pub fn choose_cur_unit(&mut self) {
        // define Current moving creature
        self.current_unit = self.queue.pop().unwrap();
    }
    pub fn sort_queue(&mut self) {
        //sort units in queue by their speed
        self.queue.sort_by_key(|k| k.speed);
    }
    pub fn get_dist(
        &self,
        first_pos_x: usize,
        first_pos_y: usize,
        second_pos_x: usize,
        second_pos_y: usize,
    ) -> usize {
        // dbg!(first_pos_x, first_pos_y, second_pos_x, second_pos_y);
        let dist = (first_pos_x as isize - second_pos_x as isize).abs()
            + (first_pos_y as isize - second_pos_y as isize).abs();
        dist as usize
    }
    pub fn end_of_move(&mut self) {
        if self.queue.len() > 0 {
            self.next_move_queue.push(self.current_unit.clone());
            self.current_unit = self.queue.pop().unwrap();
        } else {
            self.end_of_round();
        }
    }
    pub fn end_of_round(&mut self) {
        todo!();
    }

    pub fn hod(&mut self) {
        self.choose_cur_unit();
        let action_list = self.return_actions();
        let act = self.select_action(action_list);

        // match action.deistvie {
        //     _defence => self.defend(),
        // _ => panic!("Impossible action"),
        // }
    }

    pub fn select_action(&mut self, actions: Vec<Action>) -> Action {
        let index = rand::random_range(..actions.len());
        let itog = actions.get(index).ok_or(0).unwrap();
        return itog.to_owned();
    }

    pub fn zanatie_kletki(&self) -> Vec<[usize; 2]> {
        // Returns field of map which contains bariers or units
        let mut itog = vec![];
        for bar in self.map.bariers.clone() {
            itog.push(bar);
        }
        for unit in self.queue.clone() {
            itog.push([unit.pol_x, unit.pol_y]);
        }
        itog
    }
    pub fn return_enemies_vec(&self) -> Vec<Creature> {
        self.queue
            .clone()
            .into_iter()
            .filter(|x| x.is_attacers != self.current_unit.is_attacers)
            .collect()
    }

    pub fn return_actions(&self) -> Vec<Action> {
        // List vozmozhnix deistviy
        let mut ret = Vec::new();
        // zanatue unitami kletki
        let barieri = self.zanatie_kletki();
        // polozhenie unitovoppa
        let spisok_vragov: Vec<[usize; 2]> = self
            .queue
            .clone()
            .into_iter()
            .filter(|x| x.is_attacers != self.current_unit.is_attacers)
            .map(|x| [x.pol_x, x.pol_y])
            .collect();

        ret.push(Action::new(Deistvie::Wait, [0, 0]));
        ret.push(Action::new(Deistvie::Defence, [0, 0]));
        for pole_x in 0..POLE_SIZE_X {
            for pole_y in 0..POLE_SIZE_Y {
                if (self.get_dist(
                    self.current_unit.pol_x,
                    self.current_unit.pol_y,
                    pole_x,
                    pole_y,
                ) <= self.current_unit.speed)
                    & !barieri.contains(&[pole_x, pole_y])
                {
                    ret.push(Action::new(Deistvie::Move, [pole_x, pole_y]));
                    for enemy in &spisok_vragov {
                        if self.get_dist(enemy[0], enemy[1], pole_x, pole_y) == 1 {
                            ret.push(Action::new(Deistvie::Attack, [pole_x, pole_y]));
                        }
                    }
                }
            }
        }
        ret
    }

    pub fn wait(&mut self) {
        self.queue.insert(0, self.current_unit.clone());
        self.end_of_move();
    }
    pub fn defend(&mut self) {
        // dbg!(self.queue.last().unwrap().defence);
        // Defend move add defense to unit and skip move
        let mut temp = self.current_unit.clone();
        temp.defence = (temp.defence as f64 * DEF_MULTI).floor() as usize;
        self.current_unit = temp;
        self.end_of_move();
    }
    pub fn init_battle(&mut self) {
        self.initial_queue();
        self.choose_cur_unit();
    }
    pub fn get_state(&self) -> Vec<usize> {
        // How to push names?
        let mut state: Vec<usize> = vec![];
        for unit in &self.queue {
            state.append(&mut unit.to_vec());
        }
        state
        //state.resize(20, 0)
    }
}
