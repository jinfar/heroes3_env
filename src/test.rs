
#[cfg(test)]
mod test{
    #[test]
    fn dobavlenie_sushestv(){
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        assert_eq!(hero_a.creatures[0].name, "Peasant", "Proverka dobavleniya sushestv - HeroA pesant");
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        assert_eq!(hero_a.creatures[1].name, "Angel", "Proverka dobavleniya sushestv - HeroA angel");
    }
    #[test]
    fn check_speed_in_queue(){
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = crate::hero::Hero::new(1, 1, 0, 0);
        let map = crate::Pole::default();
        let mut scena = crate::Battle::new(hero_a, hero_d, map);
        assert!(scena.queue[0].speed >= scena.queue[1].speed, "Proverka sckorosti pervix sushestv v ocheredi");
    }
    #[test]
    fn check_amount_of_moves(){
        let mut hero_a = crate::hero::Hero::new(1, 1, 0, 0);
        let angel = crate::creature::get_creature("Angel");
        hero_a.add_creature(angel);
        let mut hero_d = crate::hero::Hero::new(1, 1, 0, 0);
        let map = crate::Pole::default();
        let mut scena = crate::Battle::new(hero_a, hero_d, map);
        assert!(scena.return_actions().len() == 157, "Proverka kolichestva hodov");
    }
}
