use std::io;
use std::vec::Vec;
use std::collections::HashMap;
use std::num;

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

struct Point {
    x: i32,
    y: i32,
}

struct Ship {
    entity_id: i32,
    point: Point,
    rotation: i32,
    speed: i32,
    rum: i32,
    tick_accessed: i32,
}

struct Barrel {
    entity_id: i32,
    point: Point,
    quantity: i32,
    tick_accessed: i32,
}

struct Mine {
    entity_id: i32,
    point: Point,
    tick_accessed: i32,
}

struct Cannoball {
    entity_id: i32,
    owner_id: i32,
    impact_time: i32,
    target: Point,
    tick_accessed: i32,
}

#[derive(Default)]
struct Game {
    my_ships: HashMap<i32, Ship>,
    enemy_ships: HashMap<i32, Ship>,
    barrels: HashMap<i32, Barrel>,
    mines: HashMap<i32, Mine>,
    cannonballs: HashMap<i32, Cannoball>,
    current_tick: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn distance(&self, point: &Point) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

impl Ship {
    fn new(entity_id: i32, x: i32, y: i32,
           rotation: i32, speed: i32, rum: i32) -> Ship {
        Ship {
            entity_id: entity_id,
            point: Point::new(x, y),
            rotation: rotation,
            speed: speed,
            rum: rum,
            tick_accessed: 0,
        }
    }

    fn keep_alive(&mut self, current_tick: i32) {
        self.tick_accessed = current_tick
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Barrel {
    fn new(entity_id: i32, x: i32, y: i32, quantity: i32) -> Barrel {
        Barrel {
            entity_id: entity_id,
            point: Point::new(x, y),
            quantity: quantity,
            tick_accessed: 0,
        }
    }

    fn keep_alive(&mut self, current_tick: i32) {
        self.tick_accessed = current_tick
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Mine {
    fn new(entity_id: i32, x: i32, y: i32) -> Mine {
        Mine {
            entity_id: entity_id,
            point: Point::new(x, y),
            tick_accessed: 0,
        }
    }

    fn keep_alive(&mut self, current_tick: i32) {
        self.tick_accessed = current_tick
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Cannoball {
    fn new(entity_id: i32, owner_id: i32, impact_time: i32, x: i32, y: i32) -> Cannoball {
        Cannoball {           
            entity_id: entity_id,
            owner_id: owner_id,
            impact_time: impact_time,
            target: Point::new(x, y),
            tick_accessed: 0,
        }
    }

    fn keep_alive(&mut self, current_tick: i32) {
        self.tick_accessed = current_tick
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Game {
    fn init(&mut self) {
        self.current_tick = 0;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_ship_count = parse_input!(input_line, i32); // the number of remaining ships
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. ships, mines or cannonballs)
        for _ in 0..entity_count as usize {
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
            match entity_type.as_ref() {
                "SHIP" => {
                    let ship = Ship::new(entity_id, x, y, arg_1, arg_2, arg_3);
                    if arg_4 == 1 {
                        self.my_ships.insert(entity_id, ship);
                    } else {
                        self.enemy_ships.insert(entity_id, ship);
                    }
                },
                "BARREL" => {
                    self.barrels.insert(entity_id, Barrel::new(entity_id, x, y, arg_1));
                },
                "MINE" => {
                    self.mines.insert(entity_id, Mine::new(entity_id, x, y));
                },
                "CANNONBALL" => {
                    self.cannonballs.insert(entity_id, Cannoball::new(entity_id, arg_1, x, y, arg_2));
                },
                _ => unimplemented!(),
            }
        }
        self.do_next_turn();
    }

    fn do_next_turn(&self) {
        for ship in self.my_ships.values() {
            if !ship.is_alive(self.current_tick) {
                continue;
            }
            let mut min_distance = 1000;
            let mut min_entity_id: i32 = -1;
            for barrel in self.barrels.values() {
                if !barrel.is_alive(self.current_tick) {
                    continue;
                }
                let d = ship.point.distance(&barrel.point);
                if d < min_distance {
                    min_distance = d;
                    min_entity_id = barrel.entity_id;
                }
            }
            if min_entity_id < 0 {
                println!("WAIT");
            } else {
                let barel = self.barrels.get(&min_entity_id).unwrap();
                println!("MOVE {} {}", barel.point.x, barel.point.y);
            }
        }
    }
    
    fn play(&mut self) {
        self.current_tick += 1;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_ship_count = parse_input!(input_line, i32); // the number of remaining ships
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. ships, mines or cannonballs)
        for _ in 0..entity_count as usize {
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
            match entity_type.as_ref() {
                "SHIP" => {
                    if arg_4 == 1 {
                        self.my_ships.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                    } else {
                        self.enemy_ships.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                    }
                },
                "BARREL" => {
                    self.barrels.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                },
                "MINE" => {
                    if !self.mines.contains_key(&entity_id) {
                        self.mines.insert(entity_id, Mine::new(entity_id, x, y));
                    }
                    self.mines.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                },
                "CANNONBALL" => {
                    if !self.cannonballs.contains_key(&entity_id) {
                        self.cannonballs.insert(entity_id, Cannoball::new(entity_id, arg_1, x, y, arg_2));
                    }
                    self.cannonballs.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                },
                _ => unimplemented!(),
            }
        }
        self.do_next_turn();
    }
}

fn main() {
    let mut game = Game::default();
    game.init();
    loop {
        game.play();
    }
}
