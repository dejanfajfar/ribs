mod damage;
mod player;
mod skills;
mod weapons;

use damage::*;
use player::armor::*;
use player::*;
use skills::*;
use weapons::guns::*;
use weapons::*;

fn main() {
    let mut loop_safety: i32 = 10000;

    let gun_factory = GunFactory {};

    let gun: Weapon = Weapon::Gun(gun_factory.m_10af_lexington());
    let gun2: Weapon = Weapon::Gun(gun_factory.m_10af_lexington());

    let mut player1 = Player::new("Bob".to_owned(), Skills::random(), 400).add_weapon(gun);

    let mut player2 = Player::new("Dave".to_owned(), Skills::random(), 400)
        .add_weapon(gun2)
        .add_armor(Armor::new(10.0, 5.0));

    println!(
        "| {0:<10} | {1:<10} | {2:>6} | {3:>4} |",
        "Attacker", "Attacked", "Damage", "HP"
    );
    while player1.is_alive() && player2.is_alive() && loop_safety > 0 {
        let p1_damage_output: Damage = player1.attack();
        let p2_damage_taken: (u16, u16) = player2.apply_damage(p1_damage_output);

        println!(
            "| {0:<10} | {1:<10} | {2:>6} | {3:>4} |",
            player1.name(),
            player2.name(),
            p2_damage_taken.0,
            p2_damage_taken.1
        );

        let p2_damage_output: Damage = player2.attack();
        let p1_damage_taken: (u16, u16) = player1.apply_damage(p2_damage_output);

        println!(
            "| {0:<10} | {1:<10} | {2:>6} | {3:>4} |",
            player2.name(),
            player1.name(),
            p1_damage_taken.0,
            p1_damage_taken.1
        );

        loop_safety -= 1;
    }
}
