use crate::days::read_input;

pub fn run() {
    let input = read_input(2);

    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let mut game = Game {
            id: 0,
            sets: vec![],
        };
        let game_split: Vec<_> = line.split(':').collect();
        game.id = (&game_split[0][5..]).parse().unwrap();

        for set_str in game_split[1].split(';') {
            let mut set = Set {
                red: 0,
                blue: 0,
                green: 0
            };

            for color in set_str.split(',') {
                let color_str: Vec<_> = color.trim().split(' ').collect();
                let num: i32 = (&color_str[0]).parse().unwrap();
                match color_str[1] {
                    "red" => set.red += num,
                    "blue" => set.blue += num,
                    "green" => set.green += num,
                    _ => panic!("Bad color string")
                }
            }
            game.sets.push(set);
        }
        games.push(game);
    }

    let mut result = 0;
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in game.sets {
            if set.red > max_red {
                max_red = set.red;
            }
            if set.blue > max_blue {
                max_blue = set.blue;
            }
            if set.green > max_green {
                max_green = set.green;
            }
        }

        result += max_red * max_green * max_blue;
    }

    println!("RESULT: {}", result);
}

struct Game {
    id: i32,
    sets: Vec<Set>
}

struct Set {
    red: i32,
    blue: i32,
    green: i32
}