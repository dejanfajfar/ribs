mod player;
mod skills;
mod weapons;
use player::*;
use skills::*;
use weapons::damage::*;
use weapons::guns::*;
use weapons::*;

fn main() {
    let gun_factory = GunFactory {};

    let gun: Weapon = Weapon::Gun(gun_factory.m_10af_lexington());
    let gun2: Weapon = Weapon::Gun(gun_factory.m_10af_lexington());

    let mut player1 = Player::new("Bob".to_owned(), Skills::random(), 400).add_weapon(gun);

    let mut player2 = Player::new("Dave".to_owned(), Skills::random(), 400).add_weapon(gun2);

    while player1.is_alive() && player2.is_alive() {
        let p1_damage_output: Damage = player1.attack();
        println!("{} attacked for {}", player1, p1_damage_output);
        let p2_damage_taken = player2.apply_damage(p1_damage_output);
        println!(
            "{} taken {} damage, {} HP remaining",
            player2, p2_damage_taken.0, p2_damage_taken.1
        );

        let p2_damage_output: Damage = player2.attack();
        println!("{} attacked for {}", player2, p2_damage_output);
        let p1_damage_taken = player1.apply_damage(p2_damage_output);
        println!(
            "{} taken {} damage, {} HP remaining",
            player1, p1_damage_taken.0, p1_damage_taken.1
        );
    }
}
