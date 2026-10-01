use rand::Rng;

use crate::{
    brain::{Action, Brain},
    genome::Genome,
};

#[derive(Clone)]
pub struct Creature {
    pub x: f32,
    pub y: f32,

    pub energy: f32,

    pub genome: Genome,

    // Случайное направление исследования.
    pub direction: (f32, f32),

    // Когда менять направление.
    pub direction_timer: u32,

    // Текущее направление движения.
    pub velocity: (f32, f32),
}

impl Creature {
    pub fn random(
        width: f32,
        height: f32,
        rng: &mut impl Rng,
    ) -> Self {
        Self {
            x: rng.random_range(0.0..width),

            y: rng.random_range(0.0..height),

            energy: 100.0,

            genome: Genome::random(rng),

            direction: (
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            ),

            direction_timer:
                rng.random_range(10..40),

            velocity: (0.0, 0.0),
        }
    }

    pub fn update(
        &mut self,
        food: Option<(f32, f32)>,

        neighbors:
            &[(f32, f32, (f32, f32))],

        width: f32,
        height: f32,

        rng: &mut impl Rng,
    ) {
        // ==========================================
        // ИССЛЕДОВАНИЕ
        // ==========================================

        if self.direction_timer == 0 {
            self.direction = (
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            );

            self.direction_timer =
                rng.random_range(10..40);
        }

        self.direction_timer -= 1;

        // ==========================================
        // ЕДА
        // ==========================================

        let (food_dx, food_dy) = match food {
            Some((x, y)) => {
                let dx = x - self.x;
                let dy = y - self.y;

                let distance =
                    (dx * dx + dy * dy).sqrt();

                if distance <= self.genome.vision {
                    (dx, dy)
                } else {
                    (0.0, 0.0)
                }
            }

            None => (0.0, 0.0),
        };

        // ==========================================
        // СОЦИАЛЬНОЕ ПОВЕДЕНИЕ
        // ==========================================

        let mut social_dx = 0.0;
        let mut social_dy = 0.0;

        let mut alignment_x = 0.0;
        let mut alignment_y = 0.0;

        let mut count = 0;

        for &(x, y, velocity) in neighbors {
            let dx = x - self.x;
            let dy = y - self.y;

            let distance_squared =
                dx * dx + dy * dy;

            if distance_squared < 0.0001 {
                continue;
            }

            let distance =
                distance_squared.sqrt();

            // Не видим слишком далёких существ.
            if distance > self.genome.vision {
                continue;
            }

            count += 1;

            // ======================================
            // СТАДНОЕ ПОВЕДЕНИЕ
            // ======================================

            // Движение к центру группы.
            social_dx += dx / distance;
            social_dy += dy / distance;

            // Стараемся двигаться в том же
            // направлении, что и группа.
            alignment_x += velocity.0;
            alignment_y += velocity.1;

            // ======================================
            // ЛИЧНОЕ ПРОСТРАНСТВО
            // ======================================

            if distance < 3.0 {
                social_dx -=
                    dx / distance
                    * self.genome.separation;

                social_dy -=
                    dy / distance
                    * self.genome.separation;
            }
        }

        if count > 0 {
            let count =
                count as f32;

            social_dx /= count;
            social_dy /= count;

            alignment_x /= count;
            alignment_y /= count;
        }

        // ==========================================
        // МОЗГ
        // ==========================================

        let action = Brain::decide(
            &self.genome,

            food_dx,
            food_dy,

            self.x,
            width - self.x,

            self.y,
            height - self.y,

            social_dx
                * self.genome.sociality,

            social_dy
                * self.genome.sociality,

            alignment_x
                * self.genome.alignment,

            alignment_y
                * self.genome.alignment,
        );

        // ==========================================
        // ДВИЖЕНИЕ
        // ==========================================

        let mut dx = 0.0;
        let mut dy = 0.0;

        match action {
            Action::Left => {
                dx -= 1.0;
            }

            Action::Right => {
                dx += 1.0;
            }

            Action::Up => {
                dy -= 1.0;
            }

            Action::Down => {
                dy += 1.0;
            }

            Action::Stay => {}
        }

        // Добавляем исследование.
        dx +=
            self.direction.0
            * self.genome.curiosity;

        dy +=
            self.direction.1
            * self.genome.curiosity;

        // ==========================================
        // НОРМАЛИЗАЦИЯ
        // ==========================================

        let length =
            (dx * dx + dy * dy).sqrt();

        if length > 0.0 {
            dx /= length;
            dy /= length;

            self.velocity = (
                dx * self.genome.speed,
                dy * self.genome.speed,
            );

            self.x += self.velocity.0;
            self.y += self.velocity.1;
        } else {
            self.velocity = (0.0, 0.0);
        }

        // ==========================================
        // ЗАМКНУТЫЙ МИР
        // ==========================================

        if self.x < 0.0 {
            self.x += width;
        }

        if self.x >= width {
            self.x -= width;
        }

        if self.y < 0.0 {
            self.y += height;
        }

        if self.y >= height {
            self.y -= height;
        }

        // ==========================================
        // МЕТАБОЛИЗМ
        // ==========================================

        self.energy -=
            self.genome.metabolism;
    }

    pub fn alive(&self) -> bool {
        self.energy > 0.0
    }
}
