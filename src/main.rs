use serde::{Deserialize, Serialize};
use std::fmt;

trait TotalStats {
    fn name(&self) -> &str;
}

#[derive(Serialize, Deserialize, Debug)]
struct Monster {
    name: String,
}

impl TotalStats for Monster {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
}

impl TotalStats for Player {
    fn name(&self) -> &str {
        &self.name
    }
}

struct Attack {}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} attacked {} for {} damage ({} left)",
            "m", "p", 125, 800
        )
    }
}

struct BattleResult<'a> {
    attacker: &'a dyn TotalStats,
    defender: &'a dyn TotalStats,
    history: Vec<Attack>,
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

fn battle<'a>(attacker: &'a dyn TotalStats, defender: &'a dyn TotalStats) -> BattleResult<'a> {
    BattleResult {
        attacker: attacker,
        defender: defender,
        history: vec![Attack {}, Attack {}],
    }
}

fn main() {
    // let point = Point { x: 1, y: 2 };
    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    let p = Player {
        name: "player".to_string(),
    };

    let m = Monster {
        name: "monster".to_string(),
    };

    println!("Player = {:?}", p);
    println!("Monster = {:?}", m);

    let result = battle(&p, &m);
    println!("{}", result)
}
