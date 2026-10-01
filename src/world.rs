use rand::Rng;

use crate::creature::Creature;

pub const MAX_POPULATION: usize = 150;

pub struct Food {
    pub x: f32,
    pub y: f32,
}

pub struct World {
    pub width: f32,
    pub height: f32,

    pub creatures: Vec<Creature>,

    pub food: Vec<Food>,

    pub generation: u64,

    pub max_population: usize,
}

impl World {
    pub fn new(
        width: f32,
        height: f32,
        population: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let population =
            population.min(MAX_POPULATION);

        let mut creatures =
            Vec::new();

        for _ in 0..population {
            creatures.push(
                Creature::random(
                    width,
                    height,
                    rng,
                )
            );
        }

        let mut world = Self {
            width,
            height,

            creatures,

            food: Vec::new(),

            generation: 0,

            max_population:
                MAX_POPULATION,
        };

        world.spawn_food(rng, 100);

        world
    }

    pub fn spawn_food(
        &mut self,
        rng: &mut impl Rng,
        count: usize,
    ) {
        for _ in 0..count {
            self.food.push(Food {
                x: rng.random_range(
                    0.0..self.width
                ),

                y: rng.random_range(
                    0.0..self.height
                ),
            });
        }
    }

    pub fn update(
        &mut self,
        rng: &mut impl Rng,
    ) {
        // ==========================================
        // СОЗДАЁМ СНИМОК ПОПУЛЯЦИИ
        // ==========================================

        let snapshot: Vec<_> =
            self.creatures
                .iter()
                .map(|creature| {
                    (
                        creature.x,
                        creature.y,
                        creature.velocity,
                    )
                })
                .collect();

        // ==========================================
        // ДВИЖЕНИЕ
        // ==========================================

        for i in 0..self.creatures.len() {
            let food =
                Self::nearest_food(
                    &self.creatures[i],
                    &self.food,
                );

            let neighbors: Vec<_> =
                snapshot
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(index, &(x, y, velocity))| {
                            if index == i {
                                None
                            } else {
                                Some((
                                    x,
                                    y,
                                    velocity,
                                ))
                            }
                        },
                    )
                    .collect();

            self.creatures[i].update(
                food,
                &neighbors,

                self.width,
                self.height,

                rng,
            );
        }

        // ==========================================
        // ЕДА
        // ==========================================

        self.eat();

        // ==========================================
        // СМЕРТЬ
        // ==========================================

        self.creatures
            .retain(|creature| creature.alive());

        // ==========================================
        // РАЗМНОЖЕНИЕ
        // ==========================================

        self.reproduce(rng);

        // ==========================================
        // ВОССТАНОВЛЕНИЕ ЕДЫ
        // ==========================================

        if self.food.len() < 50 {
            self.spawn_food(rng, 50);
        }

        // ==========================================
        // ЕСЛИ ПОПУЛЯЦИЯ ПОЧТИ ВЫМЕРЛА
        // ==========================================

        if self.creatures.len() < 10 {
            self.generation += 1;

            while self.creatures.len() < 30 {
                self.creatures.push(
                    Creature::random(
                        self.width,
                        self.height,
                        rng,
                    )
                );
            }
        }
    }

    fn nearest_food(
        creature: &Creature,
        food: &[Food],
    ) -> Option<(f32, f32)> {
        let mut closest = None;

        let mut closest_distance =
            f32::MAX;

        for f in food {
            let dx = f.x - creature.x;
            let dy = f.y - creature.y;

            let distance =
                dx * dx + dy * dy;

            if distance < closest_distance {
                closest_distance =
                    distance;

                closest = Some((
                    f.x,
                    f.y,
                ));
            }
        }

        closest
    }

    fn eat(&mut self) {
        let mut eaten =
            Vec::new();

        for creature in
            &mut self.creatures
        {
            for (index, food) in
                self.food.iter().enumerate()
            {
                let dx =
                    creature.x - food.x;

                let dy =
                    creature.y - food.y;

                if dx * dx + dy * dy < 4.0 {
                    creature.energy += 30.0;

                    eaten.push(index);

                    break;
                }
            }
        }

        eaten.sort_unstable();
        eaten.dedup();

        for index in
            eaten.into_iter().rev()
        {
            self.food.remove(index);
        }
    }

    fn reproduce(
        &mut self,
        rng: &mut impl Rng,
    ) {
        if self.creatures.len()
            >= self.max_population
        {
            return;
        }

        let mut children =
            Vec::new();

        for i in 0..self.creatures.len() {
            if self.creatures.len()
                + children.len()
                >= self.max_population
            {
                break;
            }

            // Нужно накопить энергию.
            if self.creatures[i].energy
                < 150.0
            {
                continue;
            }

            let parent_a =
                self.creatures[i]
                    .clone();

            let partner_index =
                rng.random_range(
                    0..self.creatures.len()
                );

            let parent_b =
                self.creatures[
                    partner_index
                ].clone();

            let genome =
                parent_a.genome.reproduce(
                    &parent_b.genome,
                    rng,
                );

            children.push(
                Creature {
                    x: parent_a.x,
                    y: parent_a.y,

                    energy: 70.0,

                    genome,

                    direction: (
                        rng.random_range(
                            -1.0..1.0
                        ),

                        rng.random_range(
                            -1.0..1.0
                        ),
                    ),

                    direction_timer:
                        rng.random_range(
                            10..40
                        ),

                    velocity: (
                        0.0,
                        0.0
                    ),
                }
            );

            // Родитель тратит энергию
            // на ребёнка.
            self.creatures[i]
                .energy -= 80.0;
        }

        self.creatures
            .extend(children);
    }
}
