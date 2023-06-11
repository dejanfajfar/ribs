use crate::engine::{
    armor::Armor,
    battlefield::BattleField,
    player::Player,
    skills::SkillsFactory,
    weapons::{blades::BladeFactory, guns::GunFactory, Weapon},
};

#[get("/")]
pub fn get_battlefield() -> String {
    let mut battlefield: BattleField = BattleField::default();

    let gun_factory: GunFactory = GunFactory {};
    let blade_factory: BladeFactory = BladeFactory {};
    let skills_factory: SkillsFactory = SkillsFactory {};

    let player1 = Player::new("Bob".to_owned(), skills_factory.random(), 400)
        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));

    let player3 = Player::new("Carl".to_owned(), skills_factory.random(), 400)
        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));

    let player2 = Player::new("Dave".to_owned(), skills_factory.ninja(), 400)
        .add_weapon(Weapon::Blade(blade_factory.katana()))
        .add_armor(Armor::new(10, false));

    battlefield.add_player(player1);
    battlefield.add_player(player2);
    battlefield.add_player(player3);

    let battlefield_json = serde_json::to_string_pretty(&battlefield).unwrap();

    let results = battlefield.start();

    let results_json = serde_json::to_string_pretty(&results).unwrap();

    println!("{}", battlefield_json);
    return results_json;
}