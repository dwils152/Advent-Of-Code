use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct ColorCounter {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    idx: u32,
    results: Vec<ColorCounter>,
}

fn main() {
    let lines = get_lines_from_txt();
    let mut sum_ids = 0;
    for line in lines.iter() {
        let g = parse_game(&line);
        if is_valid_game(&g) {
            sum_ids += &g.idx;
        }
    }
    println!("{}", sum_ids);
}

fn get_lines_from_txt() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_game(line: &str) -> Game {
    let mut game_results = line.split(":");

    let game_idx: u32 = game_results
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let game_lines = game_results.next().unwrap();
    let mut game_results_vec = Vec::<ColorCounter>::new();

    for game in game_lines.split(";") {
        let mut counter = ColorCounter {
            blue: 0,
            red: 0,
            green: 0,
        };

        for game_results in game.split(",") {
            let game_results = game_results.trim();
            let val_color: Vec<&str> = game_results.split(" ").collect();
            let val: u32 = val_color[0].parse().unwrap();
            let color = val_color[1];

            match color {
                "blue" => counter.blue += val,
                "red" => counter.red += val,
                "green" => counter.green += val,
                _ => {}
            }
            game_results_vec.push(counter.clone());
        }
    }
    Game {
        idx: game_idx,
        results: game_results_vec,
    }
}

fn is_valid_game(game: &Game) -> bool {
    let max_blue = 14;
    let max_red = 12;
    let max_green = 13;

    for round in game.results.iter() {
        if round.blue <= max_blue && round.red <= max_red && round.green <= max_green {
            continue;
        } else {
            return false
        }
    }
    return true;
}
