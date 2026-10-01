use rand::Rng;

pub const BRAIN_INPUTS: usize = 11;
pub const ACTIONS: usize = 5;
pub const BRAIN_SIZE: usize = BRAIN_INPUTS * ACTIONS;

#[derive(Clone)]
pub struct Genome {
    // Основные характеристики.
    pub speed: f32,
    pub vision: f32,
    pub metabolism: f32,

    // Исследовательское поведение.
    pub curiosity: f32,

    // Социальное поведение.
    pub sociality: f32,
    pub separation: f32,
    pub alignment: f32,

    // Внешний вид.
    pub color: [u8; 3],

    // Генетический мозг.
    pub brain: [f32; BRAIN_SIZE],
}

impl Genome {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut brain = [0.0; BRAIN_SIZE];

        for weight in &mut brain {
            *weight = rng.random_range(-1.0..1.0);
        }

        Self {
            speed: rng.random_range(0.5..2.0),

            vision: rng.random_range(10.0..50.0),

            metabolism: rng.random_range(0.5..1.5),

            curiosity: rng.random_range(0.05..0.5),

            sociality: rng.random_range(0.0..1.0),

            separation: rng.random_range(0.0..1.0),

            alignment: rng.random_range(0.0..1.0),

            color: [
                rng.random_range(50..=255),
                rng.random_range(50..=255),
                rng.random_range(50..=255),
            ],

            brain,
        }
    }

    pub fn reproduce(
        &self,
        other: &Genome,
        rng: &mut impl Rng,
    ) -> Self {
        let mut child = self.clone();

        // Выбираем характеристики одного
        // из двух родителей.
        child.speed = if rng.random_bool(0.5) {
            self.speed
        } else {
            other.speed
        };

        child.vision = if rng.random_bool(0.5) {
            self.vision
        } else {
            other.vision
        };

        child.metabolism = if rng.random_bool(0.5) {
            self.metabolism
        } else {
            other.metabolism
        };

        child.curiosity = if rng.random_bool(0.5) {
            self.curiosity
        } else {
            other.curiosity
        };

        child.sociality = if rng.random_bool(0.5) {
            self.sociality
        } else {
            other.sociality
        };

        child.separation = if rng.random_bool(0.5) {
            self.separation
        } else {
            other.separation
        };

        child.alignment = if rng.random_bool(0.5) {
            self.alignment
        } else {
            other.alignment
        };

        // Цвет тоже наследуется.
        for i in 0..3 {
            child.color[i] = if rng.random_bool(0.5) {
                self.color[i]
            } else {
                other.color[i]
            };
        }

        // Мозг.
        for i in 0..BRAIN_SIZE {
            child.brain[i] = if rng.random_bool(0.5) {
                self.brain[i]
            } else {
                other.brain[i]
            };
        }

        // Мутация.
        child.mutate(rng);

        child
    }

    fn mutate(&mut self, rng: &mut impl Rng) {
        // Скорость.
        if rng.random_bool(0.05) {
            self.speed += rng.random_range(-0.3..0.3);

            self.speed =
                self.speed.clamp(0.1, 3.0);
        }

        // Зрение.
        if rng.random_bool(0.05) {
            self.vision +=
                rng.random_range(-5.0..5.0);

            self.vision =
                self.vision.clamp(5.0, 100.0);
        }

        // Метаболизм.
        if rng.random_bool(0.05) {
            self.metabolism +=
                rng.random_range(-0.2..0.2);

            self.metabolism =
                self.metabolism.clamp(0.1, 3.0);
        }

        // Любопытство.
        if rng.random_bool(0.05) {
            self.curiosity +=
                rng.random_range(-0.1..0.1);

            self.curiosity =
                self.curiosity.clamp(0.0, 1.0);
        }

        // Социальность.
        if rng.random_bool(0.05) {
            self.sociality +=
                rng.random_range(-0.1..0.1);

            self.sociality =
                self.sociality.clamp(0.0, 1.0);
        }

        // Отталкивание.
        if rng.random_bool(0.05) {
            self.separation +=
                rng.random_range(-0.1..0.1);

            self.separation =
                self.separation.clamp(0.0, 1.0);
        }

        // Согласование направления.
        if rng.random_bool(0.05) {
            self.alignment +=
                rng.random_range(-0.1..0.1);

            self.alignment =
                self.alignment.clamp(0.0, 1.0);
        }

        // Мутация цвета.
        for component in &mut self.color {
            if rng.random_bool(0.05) {
                let delta =
                    rng.random_range(-20i16..20);

                *component =
                    (*component as i16 + delta)
                        .clamp(0, 255) as u8;
            }
        }

        // Мутация мозга.
        for weight in &mut self.brain {
            if rng.random_bool(0.1) {
                *weight +=
                    rng.random_range(-0.5..0.5);
            }
        }
    }
}
