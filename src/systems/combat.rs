use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let targets: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();
    targets.iter().for_each(|(message, target)| {
        if let Ok(mut health) = ecs.entry_mut(*target).unwrap().get_component_mut::<Health>() {
            // TODO: remove debug prints after verifying functionality
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*target);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    })
}
