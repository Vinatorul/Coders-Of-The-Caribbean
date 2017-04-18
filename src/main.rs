use std::io;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::f64;
use std::f64::consts;

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

#[derive(PartialEq, Eq)]
enum Action {
    WAIT,
    FASTER, 
    SLOWER, 
    PORT, 
    STARBOARD, 
    FIRE(i32, i32), 
    MINE,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
    cd: i32,
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
    under_fire: bool,
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
    my_ships_ids: Vec<i32>,
    enemy_ships: HashMap<i32, Ship>,
    barrels: HashMap<i32, Barrel>,
    mines: HashMap<i32, Mine>,
    cannonballs: HashMap<i32, Cannoball>,
    current_tick: i32,
    under_fire: HashSet<Point>,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn distance(&self, point: &Point) -> i32 {
        let x1 = self.x - (self.y - (self.y & 1)) / 2;
        let z1 = self.y;
        let y1 = -(x1 + z1);     
        let x2 = point.x - (point.y - (point.y & 1)) / 2;
        let z2 = point.y;
        let y2 = -(x2 + z2);
        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
    }

    fn get_neighbour(&self, rotation: i32) -> Point {
       let mut point = match rotation {
            0 => {
                Point {x:self.x + 1, y:self.y}
            },
            1 => {
                let dx = if self.y%2 == 0 {0} else {1};
                Point {x:self.x + dx, y:self.y - 1}
            },
            2 => {
                let dx = if self.y%2 == 0 {1} else {0};
                Point {x:self.x - dx, y:self.y - 1}
            },
            3 => {
                Point {x:self.x - 1, y:self.y}
            },
            4 => {
                let dx = if self.y%2 == 0 {1} else {0};
                Point {x:self.x - dx, y:self.y + 1}
            },
            5 => {
                let dx = if self.y%2 == 0 {0} else {1};
                Point {x:self.x + dx, y:self.y + 1}
            },
            _ => unimplemented!(),
        };
        if point.x < 0 {
            point.x = 0;
        } else if point.x > 22 {
            point.x = 22;
        }
        if point.y < 0 {
            point.y = 0;
        } else if point.y > 20 {
            point.y = 20;
        }
        point 
    }

    fn get_offset(&self, rotation: i32, speed: i32) -> Point {
        let mut point = match rotation {
            0 => {
                Point {x:self.x + speed, y:self.y}
            },
            1 => {
                let dx = if (self.y%2 == 0) || (speed%2 == 0) {0} else {speed};
                Point {x:self.x + speed/2 + dx, y:self.y - speed}
            },
            2 => {
                let dx = if (self.y%2 == 0) && (speed%2 == 1) {speed} else {0};
                Point {x:self.x - speed/2 - dx, y:self.y - speed}
            },
            3 => {
                Point {x:self.x - speed/2, y:self.y}
            },
            4 => {
                let dx = if (self.y%2 == 0) && (speed%2 == 1) {speed} else {0};
                Point {x:self.x - speed/2 - dx, y:self.y + speed}
            },
            5 => {
                let dx = if (self.y%2 == 0) || (speed%2 == 0)  {0} else {speed};
                Point {x:self.x + speed/2 + dx, y:self.y + speed}
            },
            _ => unimplemented!(),
        };
        if point.x < 0 {
            point.x = 0;
        } else if point.x > 22 {
            point.x = 22;
        }
        if point.y < 0 {
            point.y = 0;
        } else if point.y > 20 {
            point.y = 20;
        }
        point
    }

    fn angle(&self, target: &Point) -> f64 {
        let dy = ((target.y - self.y) as f64) * f64::sqrt(3f64) / 2f64;
        let dx = (target.x - self.x) as f64 + (((self.y - target.y) & 1) as f64) * 0.5f64;
        let mut angle = -f64::atan2(dy, dx) * 3f64 / consts::PI;
        if angle < 0f64 {
            angle = angle + 6f64;
        } else if angle >= 6f64 {
            angle = angle - 6f64;
        }
        angle
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
            cd: 0,
        }
    }

    fn update(&mut self, current_tick: i32, x: i32, y: i32,
           rotation: i32, speed: i32, rum: i32) {
        self.tick_accessed = current_tick;
        self.point.x = x;
        self.point.y = y;
        self.rotation = rotation;
        self.speed = speed;
        self.rum = rum;
        if self.cd > 0 {
            self.cd = self.cd - 1;
        }
    }

    fn set_cd(&mut self, cd: i32) {
        self.cd = cd
    }

    fn move_to(&self, point: &Point, under_fire: &HashSet<Point>) -> Action {
        let mut action = Action::WAIT;
        let target = *point;
        if target == self.point {
            return Action::SLOWER;
        }

        if self.speed > 0 {
            let position = self.point.get_neighbour(self.rotation);
            if position == self.point {
                return Action::SLOWER;
            }
            if position == target {
                return Action::WAIT;
            }
            let rotation = self.rotation as f64;
            let target_angle = position.angle(&target);
            let angle_straight = f64::min((rotation - target_angle).abs(), 6f64 - (rotation  - target_angle).abs());
            let angle_port = f64::min(((rotation + 1f64) - target_angle).abs(), ((rotation - 5f64) - target_angle).abs());
            let angle_starboard = f64::min(((rotation + 5f64) - target_angle).abs(), ((rotation - 1f64) - target_angle).abs());

            let center_angle = position.angle(&Point {x:11, y:10});
            let angle_port_center = f64::min(((rotation + 1f64) - center_angle).abs(), ((rotation - 5f64) - center_angle).abs());
            let angle_starboard_center = f64::min(((rotation + 5f64) - center_angle).abs(), ((rotation - 1f64) - center_angle).abs());

            print_err!("angles {} {} {} {}", target_angle, angle_straight, angle_port, angle_starboard);
            if (position.distance(&target) == 1) && (angle_straight > 1.5f64) {
                return Action::SLOWER;
            }

            let mut min_distance = 1000;
            {
                let next_position = position.get_neighbour(self.rotation);
                if (next_position != position) && (!under_fire.contains(&next_position)) {
                    min_distance = next_position.distance(&target);
                    action = Action::WAIT;
                }
            }
            {
                let next_position = position.get_neighbour((self.rotation + 1) % 6);
                if (next_position != position) && (!under_fire.contains(&next_position)) {
                    let distance = next_position.distance(&target);
                    if (distance < min_distance) || (distance == min_distance) && (angle_port <= angle_straight) {
                        min_distance = distance;
                        action = Action::PORT;
                    }
                }
            }
            {
                let next_position = position.get_neighbour((self.rotation + 5) % 6);
                if (next_position != position) && (!under_fire.contains(&next_position)) {
                    let distance = next_position.distance(&target);
                    if (distance <= min_distance)
                        || ((distance == min_distance) && (angle_starboard <= angle_port) && (action == Action::PORT))
                        || ((distance == min_distance) && (angle_starboard <= angle_straight) && (action == Action::WAIT))
                        || ((distance == min_distance) && (action == Action::PORT) && (angle_starboard == angle_port)
                            && (angle_starboard_center < angle_port_center))
                        || ((distance == min_distance) && (action == Action::PORT) && (angle_starboard == angle_port)
                            && (angle_starboard_center == angle_port_center) && (self.rotation == 1 || self.rotation == 4)) {
                        action = Action::STARBOARD;
                        min_distance = distance;
                    }
                }
            }
            if (min_distance == 1000) {
                action = Action::SLOWER;
            }
        } else {
            let rotation = self.rotation as f64;
            let target_angle = self.point.angle(&target);
            let angle_straight = f64::min((rotation - target_angle).abs(), 6f64 - (rotation  - target_angle).abs());
            let angle_port = f64::min(((rotation + 1f64) - target_angle).abs(), ((rotation - 5f64) - target_angle).abs());
            let angle_starboard = f64::min(((rotation + 5f64) - target_angle).abs(), ((rotation - 1f64) - target_angle).abs());

            let center_angle = self.point.angle(&Point {x:11, y:10});
            let angle_port_center = f64::min(((rotation + 1f64) - center_angle).abs(), ((rotation - 5f64) - center_angle).abs());
            let angle_starboard_center = f64::min(((rotation + 5f64) - center_angle).abs(), ((rotation - 1f64) - center_angle).abs());

            let forward_position = self.point.get_neighbour(self.rotation);

            if angle_port <= angle_starboard {
                action = Action::PORT;
            }

            if (angle_starboard < angle_port) || ((angle_starboard == angle_port) && (angle_starboard_center < angle_port_center))
                    || ((angle_starboard == angle_port) && (angle_starboard_center == angle_port_center) && (self.rotation == 1 || self.rotation == 4)) {
                action = Action::STARBOARD;
            }

            if (forward_position != self.point) && (angle_straight <= angle_port) && (angle_straight <= angle_starboard) {
                action = Action::FASTER;
            }
        }
        action
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
            under_fire: false,
        }
    }

    fn keep_alive(&mut self, current_tick: i32) {
        self.tick_accessed = current_tick
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }

    fn set_under_fire(&mut self) {
        self.under_fire = true;
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
        let _ = parse_input!(input_line, i32); // the number of remaining ships
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
                        self.my_ships_ids.push(entity_id);
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
                    self.cannonballs.insert(entity_id, Cannoball::new(entity_id, arg_1, arg_2, x, y));
                },
                _ => unimplemented!(),
            }
        }
        self.calc_under_fire();
        self.do_next_turn();
    }

    fn calc_under_fire(&mut self) {
        self.under_fire.clear();
        for cannonball in self.cannonballs.values() {
            if !cannonball.is_alive(self.current_tick) {
                continue;
            }
            self.under_fire.insert(cannonball.target);
        }
        for mine in self.mines.values_mut() {
            if !mine.is_alive(self.current_tick) {
                continue;
            }
            if self.under_fire.contains(&mine.point) {
                mine.set_under_fire();
            }
        }
    }

    fn get_mine(&self, ship: &Point) -> i32 {  
        let mut min_distance = 1000;
        let mut mine_id: i32 = -1;
        for mine in self.mines.values() {
            if !mine.is_alive(self.current_tick) {
                continue;
            }
            if mine.under_fire {
                continue;
            }
            let d = ship.distance(&mine.point);
            if d < min_distance {
                min_distance = d;
                mine_id = mine.entity_id;
            }
        }
        mine_id
    }

    fn get_target(&self, ship: &Point) -> i32 {  
        let mut min_distance = 1000;
        let mut enemy_id: i32 = -1;
        for enemy_ship in self.enemy_ships.values() {
            if !enemy_ship.is_alive(self.current_tick) {
                continue;
            }
            let d = ship.distance(&enemy_ship.point);
            if d < min_distance {
                min_distance = d;
                enemy_id = enemy_ship.entity_id;
            }
        }
        enemy_id
    }

    fn do_next_turn(&mut self){
        for key in self.my_ships_ids.iter() {
            let mut action = Action::WAIT;
            {
                let ship = self.my_ships.get(&key).unwrap();
                if !ship.is_alive(self.current_tick) {
                    continue;
                }
                let mut min_distance = 1000;
                let mut barrel_id: i32 = -1;
                for barrel in self.barrels.values() {
                    if !barrel.is_alive(self.current_tick) {
                        continue;
                    }
                    let d = ship.point.distance(&barrel.point);
                    if d < min_distance {
                        min_distance = d;
                        barrel_id = barrel.entity_id;
                    }
                }
                let enemy_id = self.get_target(&ship.point);
                let mine_id = self.get_mine(&ship.point);
                let enemy_d = ship.point.distance(&ship.point);
                
                let enemy_ship = self.enemy_ships.get(&enemy_id).unwrap();
                let offset = if enemy_ship.speed == 0 {0} else {1 + (enemy_d) / 3};
                let point = enemy_ship.point.get_offset(enemy_ship.rotation, offset);
                if ship.cd == 0 {
                    let distance = ship.point.distance(&point);
                    if distance < 8  {
                        action = Action::FIRE(point.x, point.y);
                    } else if mine_id > 0  {
                        let mine = self.mines.get(&mine_id).unwrap();
                        let mine_d = ship.point.distance(&mine.point);
                        if (mine_d < 6) && (mine_d > 2) {
                            action = Action::FIRE(mine.point.x, mine.point.y)
                        }   
                    }
                } 
                if (action == Action::WAIT) && self.under_fire.contains(&ship.point) && (ship.speed == 0) {
                    action = Action::FASTER;
                }
                if (action == Action::WAIT) && (barrel_id >= 0) {
                    let barel = self.barrels.get(&barrel_id).unwrap();
                    print_err!("MOVE HEAL {} {}", barel.point.x, barel.point.y);
                    action = ship.move_to(&barel.point, &self.under_fire);
                }  
                else if action == Action::WAIT {
                    print_err!("MOVE ATTACK {} {}", point.x, point.y);
                    action = ship.move_to(&point, &self.under_fire);     
                }
            }   
            match action {
                Action::WAIT => {println!("WAIT")},
                Action::PORT => {println!("PORT")},
                Action::STARBOARD => {println!("STARBOARD")},
                Action::SLOWER => {println!("WAIT")},
                Action::FASTER => {println!("FASTER")},
                Action::FIRE(x, y) => {             
                    self.my_ships.get_mut(&key).unwrap().set_cd(2);
                    println!("FIRE {} {}", x, y)
                },
                Action::MINE => {println!("MINE")},
            }
        }
    }
    
    fn play(&mut self) {
        self.current_tick += 1;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _ = parse_input!(input_line, i32); // the number of remaining ships
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
                        self.my_ships.get_mut(&entity_id).unwrap().
                            update(self.current_tick, x, y, arg_1, arg_2, arg_3);
                    } else {
                        self.enemy_ships.get_mut(&entity_id).unwrap().
                            update(self.current_tick, x, y, arg_1, arg_2, arg_3);
                    }
                },
                "BARREL" => {
                    if !self.barrels.contains_key(&entity_id) {
                        self.barrels.insert(entity_id, Barrel::new(entity_id, x, y, arg_1));
                    }
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
                        self.cannonballs.insert(entity_id, Cannoball::new(entity_id, arg_1, arg_2, x, y));
                    }
                    self.cannonballs.get_mut(&entity_id).unwrap().keep_alive(self.current_tick);
                },
                _ => unimplemented!(),
            }
        }
        self.calc_under_fire();
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
