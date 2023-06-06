mod battlefield;
mod damage;
mod player;
mod skills;
mod weapons;

use crate::weapons::blades::BladeFactory;
use battlefield::*;
use player::armor::*;
use player::*;
use skills::*;
use weapons::guns::*;
use weapons::*;

fn main() {
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
        .add_armor(Armor::new(10));

    battlefield.add_player(player1);
    battlefield.add_player(player2);
    battlefield.add_player(player3);

    let battlefield_json = serde_json::to_string_pretty(&battlefield).unwrap();

    let results = battlefield.start();

    let results_json = serde_json::to_string_pretty(&results).unwrap();

    println!("{}", battlefield_json);
    println!("{}", results_json);
}
