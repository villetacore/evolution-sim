mod brain;
mod creature;
mod evolution;
mod genome;
mod world;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use rand::rng;

use evolution::statistics;
use world::World;

fn ansi_color(
    rgb: [u8; 3],
    character: char,
) -> String {
    format!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        rgb[0],
        rgb[1],
        rgb[2],
        character
    )
}

fn render(world: &World) {
    // Очистить терминал.
    print!("\x1b[2J\x1b[H");

    let width =
        world.width as usize;

    let height =
        world.height as usize;

    println!(
        "╔{}╗",
        "═".repeat(width)
    );

    println!(
        "║ Generation: {:<6} Population: {:<3}/{:<3} Food: {:<4} ║",
        world.generation,
        world.creatures.len(),
        world.max_population,
        world.food.len(),
    );

    println!(
        "╠{}╣",
        "═".repeat(width)
    );

    // ==========================================
    // ПУСТОЙ ЭКРАН
    // ==========================================

    let mut screen =
        vec![vec![' '; width]; height];

    // ==========================================
    // ЕДА
    // ==========================================

    for food in &world.food {
        let x =
            food.x as usize;

        let y =
            food.y as usize;

        if x < width && y < height {
            screen[y][x] = '·';
        }
    }

    // ==========================================
    // СУЩЕСТВА
    // ==========================================

    let mut creatures_at =
        Vec::new();

    for creature
        in &world.creatures
    {
        let x =
            creature.x as usize;

        let y =
            creature.y as usize;

        if x < width && y < height {
            creatures_at.push((
                x,
                y,
                creature.genome.color,
            ));
        }
    }

    // ==========================================
    // РЕНДЕР
    // ==========================================

    for y in 0..height {
        print!("║");

        for x in 0..width {
            let creature =
                creatures_at
                    .iter()
                    .find(
                        |(cx, cy, _)| {
                            *cx == x
                                && *cy == y
                        },
                    );

            match creature {
                Some((_, _, color)) => {
                    print!(
                        "{}",
                        ansi_color(
                            *color,
                            '●'
                        )
                    );
                }

                None => {
                    print!(
                        "{}",
                        screen[y][x]
                    );
                }
            }
        }

        println!("║");
    }

    println!(
        "╚{}╝",
        "═".repeat(width)
    );

    println!();

    println!(
        "{}",
        statistics(world)
    );

    println!();

    println!(
        "Ctrl+C to exit"
    );

    io::stdout()
        .flush()
        .unwrap();
}

fn main() {
    let mut rng = rng();

    let mut world =
        World::new(
            70.0,
            25.0,
            100,
            &mut rng,
        );

    loop {
        world.update(&mut rng);

        render(&world);

        thread::sleep(
            Duration::from_millis(80)
        );
    }
}
