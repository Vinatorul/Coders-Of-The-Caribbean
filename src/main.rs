use std::io;
use std::vec::Vec;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

enum EntityType {
    Ship,
    Barrel,
}

struct Ship {
    entity_id: i32,
    x: i32,
    y: i32,
    rotation: i32,
    speed: i32,
    rum: i32,
    tick_accessed: i32,
}

struct Barrel {
    entity_id: i32,
    x: i32,
    y: i32,
    quantity: i32,
    tick_accessed: i32,
}

#[derive(Default)]
struct Game {
    my_ships: Vec<Ship>,
    enemy_ships: Vec<Ship>,
    barrels: Vec<Barrel>,
}

impl Ship {
    fn new(entity_id: i32, x: i32, y: i32,
           rotation: i32, speed: i32, rum: i32) -> Ship {
        Ship {
            entity_id: entity_id,
            x: x,
            y: y,
            rotation: rotation,
            speed: speed,
            rum: rum,
            tick_accessed: 0,
        }
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Barrel {
    fn new(entity_id: i32, x: i32, y: i32, quantity: i32) -> Barrel {
        Barrel {
            entity_id: entity_id,
            x: x,
            y: y,
            quantity: quantity,
            tick_accessed: 0,
        }
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Game {
    fn init(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_ship_count = parse_input!(input_line, i32); // the number of remaining ships
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. ships, mines or cannonballs)
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_id = parse_input!(inputs[0], i32);
            let entity_type: String = inputs[1].trim().to_string();
            let x = parse_input!(inputs[2], i32);
            let y = parse_input!(inputs[3], i32);
            let arg_1 = parse_input!(inputs[4], i32);
            let arg_2 = parse_input!(inputs[5], i32);
            let arg_3 = parse_input!(inputs[6], i32);
            let arg_4 = parse_input!(inputs[7], i32);
            match (entity_type.as_ref()) {
                "SHIP" => {
                    let ship = Ship::new(entity_id, x, y, arg_1, arg_2, arg_3);
                    if (arg_4 == 1) {
                        self.my_ships.push(ship)
                    } else {
                        self.enemy_ships.push(ship)
                    }
                },
                "BARREL" => self.barrels.push(Barrel::new(entity_id, x, y, arg_1)),
                _ => unimplemented!(),
            }
        }
        for i in 0..my_ship_count as usize {
            println!("PORT");
        }
    }
    
    fn play(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_ship_count = parse_input!(input_line, i32); // the number of remaining ships
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. ships, mines or cannonballs)
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_id = parse_input!(inputs[0], i32);
            let entity_type = inputs[1].trim().to_string();
            let x = parse_input!(inputs[2], i32);
            let y = parse_input!(inputs[3], i32);
            let arg_1 = parse_input!(inputs[4], i32);
            let arg_2 = parse_input!(inputs[5], i32);
            let arg_3 = parse_input!(inputs[6], i32);
            let arg_4 = parse_input!(inputs[7], i32);
        }
        for i in 0..my_ship_count as usize {
            println!("PORT");
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.init();
    loop {
        game.play();
    }
}
