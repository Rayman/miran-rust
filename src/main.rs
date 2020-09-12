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

struct BattleResult {
    history: Vec<Attack>,
}

impl fmt::Display for BattleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for attack in self.history.iter() {
            writeln!(f, "{}", attack)?;
        }
        Ok(())
    }
}

fn battle(attacker: &dyn TotalStats, defender: &dyn TotalStats) -> BattleResult {
    BattleResult {
        history: vec![Attack {}, Attack {}],
    }
}

fn main() {
    // let point = Point { x: 1, y: 2 };
    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    let m = Monster {
        name: "monster".to_string(),
    };
    let p = Player {
        name: "player".to_string(),
    };
    println!("Monster = {:?}", m);
    println!("Player = {:?}", p);
    let result = battle(&m, &p);
    println!("{}", result)
}
