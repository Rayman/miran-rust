use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;

struct Stats {
    hp: f64,
    damage: f64,
    attack_speed: f64,
}

trait Character {
    fn name(&self) -> &str;
    fn stats(&self) -> Stats;
}

#[derive(Serialize, Deserialize, Debug)]
struct Monster<'a> {
    name: &'a str,
}

impl Character for Monster<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn stats(&self) -> Stats {
        Stats {
            hp: 100.,
            damage: 10.,
            attack_speed: 1.,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Player<'a> {
    name: &'a str,
}

impl Character for Player<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn stats(&self) -> Stats {
        Stats {
            hp: 150.,
            damage: 5.,
            attack_speed: 1.2,
        }
    }
}

struct Attack<'a> {
    attacker: &'a dyn Character,
    defender: &'a dyn Character,
    damage: f64,
    hp_left: f64,
}

impl fmt::Display for Attack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} attacked {} for {} damage ({} left)",
            self.attacker.name(),
            self.defender.name(),
            self.damage,
            self.hp_left,
        )
    }
}

struct BattleResult<'a> {
    attacker: &'a dyn Character,
    defender: &'a dyn Character,
    history: Vec<Attack<'a>>,
}

impl fmt::Display for BattleResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Battle between {} and {}",
            self.attacker.name(),
            self.defender.name()
        )?;
        for attack in self.history.iter() {
            writeln!(f, "{}", attack)?;
        }
        Ok(())
    }
}

// fn do_attack(attacker_stats: &Stats, defender_stats: &Stats) -> Attack {
//     let damage = attacker_stats.damage;
//     defender_stats.hp -= damage;
//     Attack {
//         attacker: attacker,
//         defender: defender,
//         damage,
//         hp_left: defender_stats.hp,
//         swing_speed: attacker_stats.attack_speed,
//     }
// }

fn battle<'a>(attacker: &'a dyn Character, defender: &'a dyn Character) -> BattleResult<'a> {
    let mut attacker_stats = attacker.stats();
    let mut defender_stats = defender.stats();

    let mut time = 0.;
    let mut history: Vec<Attack> = vec![];
    while attacker_stats.hp > 0. && defender_stats.hp > 0. {
        if time > 0. {
            let damage = attacker_stats.damage;
            defender_stats.hp -= damage;
            history.push(Attack {
                attacker: attacker,
                defender: defender,
                damage,
                hp_left: defender_stats.hp,
            });
            time -= attacker_stats.attack_speed;
        } else {
            let damage = defender_stats.damage;
            attacker_stats.hp -= damage;
            history.push(Attack {
                attacker: defender,
                defender: attacker,
                damage,
                hp_left: attacker_stats.hp,
            });
            time += defender_stats.attack_speed;
        }
    }

    BattleResult {
        attacker: attacker,
        defender: defender,
        history: history,
    }
}

fn main() -> Result<(), serde_json::Error> {
    // let point = Point { x: 1, y: 2 };
    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    let p = Player { name: "player" };

    let m = Monster { name: "monster" };

    println!("Player = {:?}", p);
    println!("Monster = {:?}", m);

    let result = battle(&p, &m);
    println!("{}", result);

    serde_json::to_writer(&File::create("m.json").unwrap(), &m)
}
