use crate::genome::Genome;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
    Stay,
}

pub struct Brain;

impl Brain {
    pub fn decide(
        genome: &Genome,

        food_dx: f32,
        food_dy: f32,

        wall_left: f32,
        wall_right: f32,
        wall_top: f32,
        wall_bottom: f32,

        social_dx: f32,
        social_dy: f32,

        alignment_x: f32,
        alignment_y: f32,
    ) -> Action {
        let vision =
            genome.vision.max(1.0);

        let inputs = [
            // Еда.
            food_dx / vision,
            food_dy / vision,

            // Границы.
            wall_left / vision,
            wall_right / vision,
            wall_top / vision,
            wall_bottom / vision,

            // Соседи.
            social_dx / vision,
            social_dy / vision,

            // Направление группы.
            alignment_x,
            alignment_y,

            // Bias.
            1.0,
        ];

        let mut scores = [0.0; 5];

        for action in 0..5 {
            let offset =
                action * 11;

            for input in 0..11 {
                scores[action] +=
                    genome.brain[
                        offset + input
                    ] * inputs[input];
            }
        }

        let mut best = 0;

        for i in 1..5 {
            if scores[i] > scores[best] {
                best = i;
            }
        }

        match best {
            0 => Action::Left,
            1 => Action::Right,
            2 => Action::Up,
            3 => Action::Down,
            _ => Action::Stay,
        }
    }
}
