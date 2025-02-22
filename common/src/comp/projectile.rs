use crate::{
    combat::{
        Attack, AttackDamage, AttackEffect, CombatBuff, CombatBuffStrength, CombatEffect,
        CombatRequirement, Damage, DamageKind, DamageSource, GroupTarget, Knockback, KnockbackDir,
    },
    comp::{
        buff::BuffKind,
        item::{tool, Reagent},
    },
    uid::Uid,
    Explosion, RadiusEffect,
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use specs::Component;
use std::time::Duration;
use vek::Rgb;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Effect {
    Attack(Attack),
    Explode(Explosion),
    Vanish,
    Stick,
    Possess,
    Bonk, // Knock/dislodge/change objects on hit
}

#[derive(Clone, Debug)]
pub struct Projectile {
    // TODO: use SmallVec for these effects
    pub hit_solid: Vec<Effect>,
    pub hit_entity: Vec<Effect>,
    /// Time left until the projectile will despawn
    pub time_left: Duration,
    pub owner: Option<Uid>,
    /// Whether projectile collides with entities in the same group as its
    /// owner
    pub ignore_group: bool,
    /// Whether the projectile is sticky
    pub is_sticky: bool,
    /// Whether the projectile should use a point collider
    pub is_point: bool,
}

impl Component for Projectile {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectileConstructor {
    Arrow {
        damage: f32,
        knockback: f32,
        energy_regen: f32,
    },
    Knife {
        damage: f32,
        knockback: f32,
        energy_regen: f32,
    },
    FireDroplet {
        damage: f32,
        radius: f32,
        energy_regen: f32,
        min_falloff: f32,
        reagent: Option<Reagent>,
    },
    Fireball {
        damage: f32,
        radius: f32,
        energy_regen: f32,
        min_falloff: f32,
    },
    Frostball {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    Poisonball {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    NecroticSphere {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    Magicball {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    Possess,
    ClayRocket {
        damage: f32,
        radius: f32,
        knockback: f32,
        min_falloff: f32,
    },
    InkBomb {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    Snowball {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    ExplodingPumpkin {
        damage: f32,
        radius: f32,
        knockback: f32,
        min_falloff: f32,
    },
    DagonBomb {
        damage: f32,
        radius: f32,
        knockback: f32,
        min_falloff: f32,
    },
    IceBomb {
        damage: f32,
        radius: f32,
        knockback: f32,
        min_falloff: f32,
    },
    LaserBeam {
        damage: f32,
        radius: f32,
        knockback: f32,
        min_falloff: f32,
    },
    Trap {
        damage: f32,
    },
    Mine {
        damage: f32,
        radius: f32,
        min_falloff: f32,
    },
    Pebble {
        damage: f32,
        knockback: f32,
    },
}

impl ProjectileConstructor {
    pub fn create_projectile(
        self,
        owner: Option<Uid>,
        precision_mult: f32,
        tool_stats: tool::Stats,
        damage_effect: Option<CombatEffect>,
    ) -> Projectile {
        let instance = rand::random();
        use ProjectileConstructor::*;
        match self {
            Arrow {
                damage,
                knockback,
                energy_regen,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let energy = AttackEffect::new(None, CombatEffect::EnergyReward(energy_regen))
                    .with_requirement(CombatRequirement::AnyDamage);
                let buff = CombatEffect::Buff(CombatBuff {
                    kind: BuffKind::Bleeding,
                    dur_secs: 10.0,
                    strength: CombatBuffStrength::DamageFraction(0.1),
                    chance: 0.1,
                })
                .adjusted_by_stats(tool_stats);
                let mut damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Projectile,
                        kind: DamageKind::Piercing,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                )
                .with_effect(buff);
                if let Some(damage_effect) = damage_effect {
                    damage = damage.with_effect(damage_effect);
                }
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(energy)
                    .with_effect(knockback)
                    .with_combo_increment();

                Projectile {
                    hit_solid: vec![Effect::Stick, Effect::Bonk],
                    hit_entity: vec![Effect::Attack(attack), Effect::Vanish],
                    time_left: Duration::from_secs(15),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Knife {
                damage,
                knockback,
                energy_regen,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let energy = AttackEffect::new(None, CombatEffect::EnergyReward(energy_regen))
                    .with_requirement(CombatRequirement::AnyDamage);
                let buff = CombatEffect::Buff(CombatBuff {
                    kind: BuffKind::Bleeding,
                    dur_secs: 10.0,
                    strength: CombatBuffStrength::DamageFraction(0.1),
                    chance: 0.1,
                })
                .adjusted_by_stats(tool_stats);
                let mut damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Projectile,
                        kind: DamageKind::Piercing,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                )
                .with_effect(buff);
                if let Some(damage_effect) = damage_effect {
                    damage = damage.with_effect(damage_effect);
                }
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(energy)
                    .with_effect(knockback)
                    .with_combo_increment();

                Projectile {
                    hit_solid: vec![Effect::Stick, Effect::Bonk],
                    hit_entity: vec![Effect::Attack(attack), Effect::Vanish],
                    time_left: Duration::from_secs(15),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            FireDroplet {
                damage,
                radius,
                energy_regen,
                min_falloff,
                reagent,
            } => {
                let energy = AttackEffect::new(None, CombatEffect::EnergyReward(energy_regen))
                    .with_requirement(CombatRequirement::AnyDamage);
                let buff = CombatEffect::Buff(CombatBuff {
                    kind: BuffKind::Burning,
                    dur_secs: 4.0,
                    strength: CombatBuffStrength::DamageFraction(1.0),
                    chance: 1.0,
                })
                .adjusted_by_stats(tool_stats);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                )
                .with_effect(buff);
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(energy)
                    .with_combo_increment();
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(2.0, Rgb::black()),
                    ],
                    radius,
                    reagent,
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Fireball {
                damage,
                radius,
                energy_regen,
                min_falloff,
            } => {
                let energy = AttackEffect::new(None, CombatEffect::EnergyReward(energy_regen))
                    .with_requirement(CombatRequirement::AnyDamage);
                let buff = CombatEffect::Buff(CombatBuff {
                    kind: BuffKind::Burning,
                    dur_secs: 5.0,
                    strength: CombatBuffStrength::DamageFraction(0.1),
                    chance: 0.1,
                })
                .adjusted_by_stats(tool_stats);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                )
                .with_effect(buff);
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(energy)
                    .with_combo_increment();
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(2.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Red),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Frostball {
                damage,
                radius,
                min_falloff,
            } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_combo_increment();
                let explosion = Explosion {
                    effects: vec![RadiusEffect::Attack(attack)],
                    radius,
                    reagent: Some(Reagent::White),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Poisonball {
                damage,
                radius,
                min_falloff,
            } => {
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Poisoned,
                        dur_secs: 5.0,
                        strength: CombatBuffStrength::DamageFraction(0.8),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(buff);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(5.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Purple),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            NecroticSphere {
                damage,
                radius,
                min_falloff,
            } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_combo_increment();
                let explosion = Explosion {
                    effects: vec![RadiusEffect::Attack(attack)],
                    radius,
                    reagent: Some(Reagent::Purple),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Magicball {
                damage,
                radius,
                min_falloff,
            } => {
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Poisoned,
                        dur_secs: 5.0,
                        strength: CombatBuffStrength::DamageFraction(0.8),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(buff);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(0.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Green),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Possess => Projectile {
                hit_solid: vec![Effect::Stick],
                hit_entity: vec![Effect::Stick, Effect::Possess],
                time_left: Duration::from_secs(10),
                owner,
                ignore_group: false,
                is_sticky: true,
                is_point: true,
            },
            ClayRocket {
                damage,
                radius,
                knockback,
                min_falloff,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(5.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Red),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            InkBomb {
                damage,
                radius,
                min_falloff,
            } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Wet,
                        dur_secs: 8.0,
                        strength: CombatBuffStrength::Value(0.5),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(buff)
                    .with_combo_increment();
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(18.0, Rgb::new(4.0, 7.0, 32.0)),
                    ],
                    radius,
                    reagent: Some(Reagent::Blue),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Snowball {
                damage,
                radius,
                min_falloff,
            } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult);
                let explosion = Explosion {
                    effects: vec![RadiusEffect::Attack(attack)],
                    radius,
                    reagent: Some(Reagent::White),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(120),
                    owner,
                    ignore_group: true,
                    is_sticky: false,
                    is_point: false,
                }
            },
            ExplodingPumpkin {
                damage,
                radius,
                knockback,
                min_falloff,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Burning,
                        dur_secs: 5.0,
                        strength: CombatBuffStrength::DamageFraction(0.2),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback)
                    .with_effect(buff);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(5.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Red),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            DagonBomb {
                damage,
                radius,
                knockback,
                min_falloff,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Burning,
                        dur_secs: 5.0,
                        strength: CombatBuffStrength::DamageFraction(0.2),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback)
                    .with_effect(buff);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(25.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Blue),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            IceBomb {
                damage,
                radius,
                knockback,
                min_falloff,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    }),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let buff = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Buff(CombatBuff {
                        kind: BuffKind::Frozen,
                        dur_secs: 5.0,
                        strength: CombatBuffStrength::DamageFraction(0.05),
                        chance: 1.0,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback)
                    .with_effect(buff);
                let variation = thread_rng().gen::<f32>();
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(
                            30.0,
                            Rgb::new(
                                83.0 - (20.0 * variation),
                                212.0 - (52.0 * variation),
                                255.0 - (62.0 * variation),
                            ),
                        ),
                    ],
                    radius,
                    reagent: Some(Reagent::White),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            LaserBeam {
                damage,
                radius,
                knockback,
                min_falloff,
            } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback);
                let explosion = Explosion {
                    effects: vec![
                        RadiusEffect::Attack(attack),
                        RadiusEffect::TerrainDestruction(10.0, Rgb::black()),
                    ],
                    radius,
                    reagent: Some(Reagent::Yellow),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![Effect::Explode(explosion.clone()), Effect::Vanish],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: true,
                }
            },
            Trap { damage } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Piercing,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult);
                Projectile {
                    hit_solid: vec![],
                    hit_entity: vec![Effect::Attack(attack), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: true,
                    is_point: false,
                }
            },
            Mine {
                damage,
                radius,
                min_falloff,
            } => {
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Explosion,
                        kind: DamageKind::Energy,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult);
                let explosion = Explosion {
                    effects: vec![RadiusEffect::Attack(attack)],
                    radius,
                    reagent: Some(Reagent::Yellow),
                    min_falloff,
                };
                Projectile {
                    hit_solid: vec![],
                    hit_entity: vec![Effect::Explode(explosion), Effect::Vanish],
                    time_left: Duration::from_secs(120),
                    owner,
                    ignore_group: true,
                    is_sticky: false,
                    is_point: false,
                }
            },
            Pebble { damage, knockback } => {
                let knockback = AttackEffect::new(
                    Some(GroupTarget::OutOfGroup),
                    CombatEffect::Knockback(Knockback {
                        strength: knockback,
                        direction: KnockbackDir::Away,
                    })
                    .adjusted_by_stats(tool_stats),
                )
                .with_requirement(CombatRequirement::AnyDamage);
                let damage = AttackDamage::new(
                    Damage {
                        source: DamageSource::Projectile,
                        kind: DamageKind::Crushing,
                        value: damage,
                    },
                    Some(GroupTarget::OutOfGroup),
                    instance,
                );
                let attack = Attack::default()
                    .with_damage(damage)
                    .with_precision(precision_mult)
                    .with_effect(knockback);

                Projectile {
                    hit_solid: vec![Effect::Vanish, Effect::Vanish],
                    hit_entity: vec![Effect::Attack(attack), Effect::Vanish],
                    time_left: Duration::from_secs(10),
                    owner,
                    ignore_group: true,
                    is_sticky: false,
                    is_point: false,
                }
            },
        }
    }

    // TODO: split this to three methods per stat
    #[must_use]
    pub fn modified_projectile(mut self, power: f32, regen: f32, range: f32) -> Self {
        use ProjectileConstructor::*;
        match self {
            Arrow {
                ref mut damage,
                ref mut energy_regen,
                ..
            } => {
                *damage *= power;
                *energy_regen *= regen;
            },
            Knife {
                ref mut damage,
                ref mut energy_regen,
                ..
            } => {
                *damage *= power;
                *energy_regen *= regen;
            },
            FireDroplet {
                ref mut damage,
                ref mut energy_regen,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *energy_regen *= regen;
                *radius *= range;
            },
            Fireball {
                ref mut damage,
                ref mut energy_regen,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *energy_regen *= regen;
                *radius *= range;
            },
            Frostball {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Poisonball {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            NecroticSphere {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Magicball {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Possess => {},
            ClayRocket {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            InkBomb {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Snowball {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            ExplodingPumpkin {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            DagonBomb {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            IceBomb {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            LaserBeam {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Trap { ref mut damage, .. } => {
                *damage *= power;
            },
            Mine {
                ref mut damage,
                ref mut radius,
                ..
            } => {
                *damage *= power;
                *radius *= range;
            },
            Pebble { ref mut damage, .. } => {
                *damage *= power;
            },
        }
        self
    }

    pub fn is_explosive(&self) -> bool {
        use ProjectileConstructor::*;
        match self {
            Arrow { .. } => false,
            Knife { .. } => false,
            FireDroplet { .. } => true,
            Fireball { .. } => true,
            Frostball { .. } => true,
            Poisonball { .. } => true,
            NecroticSphere { .. } => true,
            Magicball { .. } => true,
            Possess => false,
            ClayRocket { .. } => true,
            Snowball { .. } => true,
            ExplodingPumpkin { .. } => true,
            DagonBomb { .. } => true,
            InkBomb { .. } => true,
            IceBomb { .. } => true,
            LaserBeam { .. } => true,
            Trap { .. } => false,
            Mine { .. } => true,
            Pebble { .. } => false,
        }
    }
}
