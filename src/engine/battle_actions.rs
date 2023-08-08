use super::{movement::MovementResult, combatant::Combatant};

#[derive(Debug, Clone)]
pub enum BattleAction{
    Move(u32,String, MovementResult),
    Attack(u32, BattleAttackAction)
}

#[derive(Debug, Clone)]
pub struct BattleAttackAction{
    pub assailant: Combatant,
    pub victim: Combatant,
    pub damage: u16
}