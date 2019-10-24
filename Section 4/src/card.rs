use d4_builder_derive::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

#[derive(Debug, PartialEq, Setter)]
pub struct Card {
    pub name: String,
    pub strength: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            name: "".to_string(),
            strength: 1,
            health: 1,
            cost: 1,
            triggers: BTreeMap::new(),
            abilities: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct CardBuilder {
    pub name: String,
    pub strength: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new(name: String) -> Self {
        CardBuilder {
            name,
            ..Default::default()
        }
    }

    pub fn strength(mut self, s: i32) -> Self {
        self.strength = Some(s);
        self
    }

    pub fn trigger(mut self, t: Trigger, s: String) -> Self {
        self.triggers.insert(t, s);
        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name,
            strength: self.strength.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}

impl Card {
    pub fn build(name: String) -> CardBuilder {
        CardBuilder::new(name)
    }
}

pub trait Triggerable {
    fn trigger(&self, t: Trigger) -> Option<String>;
}

impl Triggerable for Card {
    fn trigger(&self, t: Trigger) -> Option<String> {
        self.triggers.get(&t).map(|s| s.to_string())
    }
}

pub struct TriggerWrap<A: Triggerable, B: Triggerable> {
    a: A,
    b: B,
}

impl<A: Triggerable, B: Triggerable> Triggerable for TriggerWrap<A, B> {
    fn trigger(&self, t: Trigger) -> Option<String> {
        self.a.trigger(t).or_else(|| self.b.trigger(t))
    }
}

#[cfg(test)]
mod test_builder {
    use super::*;
    #[test]
    pub fn test_card_builder() {
        let c = Card::build("General Blight".to_string())
            .strength(4)
            .trigger(Trigger::BattleCry, "Deal 2 Damage".to_string())
            .build();

        let mut triggers = BTreeMap::new();
        triggers.insert(Trigger::BattleCry, "Deal 2 Damage".to_string());
        let c2 = Card {
            name: "General Blight".to_string(),
            strength: 4,
            cost: 1,
            health: 1,
            triggers,
            abilities: Vec::new(),
        };

        assert_eq!(c, c2);
    }

    #[test]
    pub fn test_card_default() {
        let c = Card {
            name: "Storm Guard".to_string(),
            ..Default::default()
        };
        let c2 = Card {
            name: "Storm Guard".to_string(),
            strength: 1,
            cost: 1,
            health: 1,
            triggers: BTreeMap::new(),
            abilities: Vec::new(),
        };
        assert_eq!(c, c2);
    }

    #[test]
    pub fn test_trigger_wrap() {
        let c1 = Card::build("c1".to_string())
            .trigger(Trigger::BattleCry, "Cry Battle".to_string())
            .build();

        let c2 = Card::build("c2".to_string())
            .trigger(Trigger::Death, "Say Aaaargh".to_string())
            .build();

        let wrap = TriggerWrap { a: c1, b: c2 };
        assert_eq!(wrap.trigger(Trigger::BattleCry).unwrap(), "Cry Battle");
        assert_eq!(wrap.trigger(Trigger::Death).unwrap(), "Say Aaaargh");
    }

    #[test]
    pub fn test_setter_macro() {
        let mut c2 = Card::build("c2".to_string()).build();
        c2.set_strength(4);
        assert_eq!(c2.strength, 4);
    }
}
