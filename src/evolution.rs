use crate::world::World;

pub fn statistics(
    world: &World,
) -> String {
    if world.creatures.is_empty() {
        return "No creatures".to_string();
    }

    let count =
        world.creatures.len() as f32;

    let speed =
        world
            .creatures
            .iter()
            .map(|c| c.genome.speed)
            .sum::<f32>()
            / count;

    let vision =
        world
            .creatures
            .iter()
            .map(|c| c.genome.vision)
            .sum::<f32>()
            / count;

    let metabolism =
        world
            .creatures
            .iter()
            .map(|c| c.genome.metabolism)
            .sum::<f32>()
            / count;

    let curiosity =
        world
            .creatures
            .iter()
            .map(|c| c.genome.curiosity)
            .sum::<f32>()
            / count;

    let sociality =
        world
            .creatures
            .iter()
            .map(|c| c.genome.sociality)
            .sum::<f32>()
            / count;

    let separation =
        world
            .creatures
            .iter()
            .map(|c| c.genome.separation)
            .sum::<f32>()
            / count;

    let alignment =
        world
            .creatures
            .iter()
            .map(|c| c.genome.alignment)
            .sum::<f32>()
            / count;

    format!(
        "Speed {:.2} | Vision {:.1} | \
         Metabolism {:.2} | Curiosity {:.2} | \
         Social {:.2} | Separation {:.2} | \
         Alignment {:.2}",
        speed,
        vision,
        metabolism,
        curiosity,
        sociality,
        separation,
        alignment,
    )
}
