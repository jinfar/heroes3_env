#[cfg(test)]
mod test {
    use crate::battle::Battle;
    use crate::hero::Hero;
    #[test]
    fn dobavlenie_sushestv() {
        let mut hero_a = Hero::default();
        assert_eq!(
            hero_a.creatures[0].name, "Peasant",
            "Proverka dobavleniya sushestv - HeroA pesant"
        );
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        assert_eq!(
            hero_a.creatures[1].name, "Angel",
            "Proverka dobavleniya sushestv - HeroA angel"
        );
    }
    #[test]
    fn check_speed_in_queue() {
        let mut hero_a = Hero::default();
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel.clone());
        let mut hero_d = Hero::default();
        hero_d.add_creature(angel);
        let map = crate::Pole::default();
        let mut scena = Battle::new(hero_a, hero_d, map);
        assert!(
            scena.queue[0].speed >= scena.queue[1].speed,
            "Proverka sckorosti pervix sushestv v ocheredi"
        );
    }
    #[test]
    fn check_amount_of_moves() {
        let mut hero_a = Hero::default();
        let angel = crate::creature::get_creature("Angel");
        dbg!(hero_a.clone());
        hero_a.add_creature(angel);
        dbg!(hero_a.clone());
        let mut hero_d = Hero::default();
        let map = crate::Pole::default();
        let mut scena = Battle::new(hero_a, hero_d, map);
        println!("Kolichestvo hodov: {}", scena.return_actions().len());
        assert!(
            scena.return_actions().len() == 157,
            "Proverka kolichestva hodov"
        );
    }
    #[test]
    fn check_enemies_dist() {
        let mut hero_a = Hero::default();
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = Hero::default();
        let map = crate::Pole::default();
        let mut scena = Battle::new(hero_a.clone(), hero_d.clone(), map.clone());
        assert!(
            scena.return_enemies_vec().len() == 1,
            "Proverka kolichestva vragov 1"
        );
        hero_d.add_creature(crate::creature::get_creature("Peasant"));
        hero_d.add_creature(crate::creature::get_creature("Peasant"));
        let mut scena = Battle::new(hero_a, hero_d, map);
        assert!(
            scena.return_enemies_vec().len() == 3,
            "Proverka kolichestva vragov 3"
        );
    }
}
