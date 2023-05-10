mod damage;
mod player;
mod skills;
mod weapons;

use std::collections::HashMap;
use damage::*;
use player::armor::*;
use player::*;
use rand::seq::{SliceChooseIter, SliceRandom};
use skills::*;
use weapons::guns::*;
use weapons::*;

use crate::weapons::blades::BladeFactory;

fn main() {
    let mut loop_counter: u32 = 1;
    let max_loop_count: u32 = 10000;

    let gun_factory: GunFactory = GunFactory {};
    let blade_factory: BladeFactory = BladeFactory {};
    let skills_factory: SkillsFactory = SkillsFactory {};

    let player1 = Player::new("Bob".to_owned(), skills_factory.random(), 400)
        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));

    let player3 = Player::new("Carl".to_owned(), skills_factory.random(), 400)
        .add_weapon(Weapon::Gun(gun_factory.m_10af_lexington()));

    let player2 = Player::new("Dave".to_owned(), skills_factory.ninja(), 400)
        .add_weapon(Weapon::Blade(blade_factory.katana()))
        .add_armor(Armor::new(10.0, 5.0));


    println!("{}", player1.pretty_print());
    println!("{}", player2.pretty_print());
    println!("{}", player3.pretty_print());

    let mut players: HashMap<String, Player> = HashMap::from([
        (player1.name(), player1),
        (player2.name(), player2),
        (player3.name(), player3),
    ]);

    println!(
        "| {0:<5} | {1:<10} | {2:<10} | {3:>6} | {4:>4} |",
         "Round", "Attacker", "Attacked", "Damage", "HP"
    );
    while players.iter().any(|p: (&String, &Player)| p.1.is_alive()) && loop_counter <= max_loop_count {
        let mut live_players: Vec<String> = live_players(&players);

        if live_players.len() == 1 {
            break;
        }
        let (attacker_name, attacked_name) = choose_two(&mut live_players);

        let mut attacker: Player = players.get(attacker_name).unwrap().to_owned();
        let mut attacked: Player = players.get(attacked_name).unwrap().to_owned();

        let attacker_dmg_out: Damage = attacker.attack();
        let (attacked_dmg_taken, attacked_hp_remaining) = attacked.apply_damage(attacker_dmg_out);

        players.insert(attacker.name(), attacker);
        players.insert(attacked.name(), attacked);

        println!(
            "| {0:<5} | {1:<10} | {2:<10} | {3:>6} | {4:>4} |",
            loop_counter, attacker_name, attacked_name, attacked_dmg_taken, attacked_hp_remaining
        );

        loop_counter += 1;
    }
}

fn choose_two(players: &mut Vec<String>) -> (&String, &String) {
    players.shuffle(&mut rand::thread_rng());

    let chosen: SliceChooseIter<[String], String> =
        players.choose_multiple(&mut rand::thread_rng(), 2);

    let mut chosen_iter: SliceChooseIter<[String], String> = chosen.into_iter();

    return (chosen_iter.next().unwrap(), chosen_iter.next().unwrap());
}

fn live_players(players: &HashMap<String, Player>) -> Vec<String> {
    let mut ret_val: Vec<String> = vec![];

    for (player_name, player) in players {
        if player.is_alive() {
            ret_val.insert(0, player_name.clone());
        }
    }

    return ret_val;
}
