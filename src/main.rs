use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

#[derive(Clone)]
struct Stats<'a> {
    character: &'a dyn Character,
    hp: f64,
    damage: f64,
    attack_speed: f64,
}

trait Character {
    fn name(&self) -> &str;
    fn stats(&self) -> Stats;
}

#[derive(Debug)]
struct Monster<'a> {
    name: &'a str,
}

impl Character for Monster<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn stats(&self) -> Stats {
        Stats {
            character: self,
            hp: 100.,
            damage: 10.,
            attack_speed: 1.,
        }
    }
}

#[derive(Debug)]
struct Player<'a> {
    name: &'a str,
}

impl Character for Player<'_> {
    fn name(&self) -> &str {
        &self.name
    }
    fn stats(&self) -> Stats {
        Stats {
            character: self,
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
    attack_speed: f64,
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

fn calculate_attack<'a>(attacker: Stats<'a>, defender: Stats<'a>) -> Attack<'a> {
    let damage = attacker.damage;
    Attack {
        attacker: attacker.character,
        defender: defender.character,
        damage,
        hp_left: defender.hp - damage,
        attack_speed: attacker.attack_speed,
    }
}

enum Event<'a> {
    Cooldown(&'a dyn Character),
}

struct HeapItem<'a> {
    priority: f64, // NaN values are not allowed
    event: Event<'a>,
}

impl Ord for HeapItem<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord,
        }
    }
}

impl PartialOrd for HeapItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for HeapItem<'_> {}

fn battle<'a>(attacker: &'a dyn Character, defender: &'a dyn Character) -> BattleResult<'a> {
    let mut attacker_stats = attacker.stats();
    let mut defender_stats = defender.stats();

    let mut time = 0.;
    let mut history: Vec<Attack> = vec![];

    let e = HeapItem {
        priority: 3.,
        event: Event::Cooldown(attacker),
    };

    let mut heap = BinaryHeap::new();
    heap.push(e);

    while attacker_stats.hp > 0. && defender_stats.hp > 0. {
        if time > 0. {
            let attack = calculate_attack(attacker_stats.clone(), defender_stats.clone());
            defender_stats.hp = attack.hp_left;
            time -= attack.attack_speed;
            history.push(attack);
        } else {
            let attack = calculate_attack(defender_stats.clone(), attacker_stats.clone());
            attacker_stats.hp = attack.hp_left;
            time += attack.attack_speed;
            history.push(attack);
        }
    }

    BattleResult {
        attacker: attacker,
        defender: defender,
        history,
    }
}

fn main() {
    let p = Player { name: "player" };
    let m = Monster { name: "monster" };

    println!("Player = {:?}", p);
    println!("Monster = {:?}", m);

    let result = battle(&p, &m);
    println!("{}", result);
}
