#[cfg(test)]
mod test {
    #[test]
    fn dobavlenie_sushestv() {
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
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
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = crate::hero::Hero::new(1, 1, 0, 0);
        let map = crate::Pole::default();
        let mut scena = crate::Battle::new(hero_a, hero_d, map);
        assert!(
            scena.queue[0].speed >= scena.queue[1].speed,
            "Proverka sckorosti pervix sushestv v ocheredi"
        );
    }
    #[test]
    fn check_amount_of_moves() {
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = crate::hero::Hero::new(1, 1, 0, 0);
        let map = crate::Pole::default();
        let mut scena = crate::Battle::new(hero_a, hero_d, map);
        assert!(
            scena.return_actions().len() == 157,
            "Proverka kolichestva hodov"
        );
    }
    #[test]
    fn check_enemies_dist() {
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = crate::hero::Hero::new(1, 1, 0, 0);
        let map = crate::Pole::default();
        let mut scena = crate::Battle::new(hero_a.clone(), hero_d.clone(), map.clone());
        assert!(
            scena.return_enemies_vec().len() == 1,
            "Proverka kolichestva vragov 1"
        );
        hero_d.add_creature(crate::creature::get_creature("Peasant"));
        hero_d.add_creature(crate::creature::get_creature("Peasant"));
        let mut scena = crate::Battle::new(hero_a, hero_d, map);
        assert!(
            scena.return_enemies_vec().len() == 3,
            "Proverka kolichestva vragov 3"
        );
    }
}
