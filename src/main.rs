use std::io;
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use std::f64;
use std::f64::consts;
use std::cmp;

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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
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
    wp_ind: usize,
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
    under_fire: HashMap<Point, i32>,
    barrels_field: HashSet<Point>,
    mine_field: HashSet<Point>,
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
        if (point.x < 0) || (point.x > 22) || (point.y < 0) || (point.y > 20) {
            point.x = self.x;
            point.y = self.y;
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
            wp_ind: (entity_id as usize)%4,
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
        self.tick_accessed = current_tick;
        self.impact_time = self.impact_time - 1;
    }

    fn is_alive(&self, current_tick: i32) -> bool {
        current_tick == self.tick_accessed
    }
}

impl Game {
    fn check_position(&self, point: &Point, rotation: i32, speed: i32, depth: i32) -> i32 {
        let nose = point.get_neighbour(rotation);
        let stern = point.get_neighbour((rotation + 3)%6);
        let mut value = 0;
        if self.barrels_field.contains(&stern) {
            value = value + 10;
        }
        if self.barrels_field.contains(point) {
            value = value + 10;
        }
        if self.barrels_field.contains(&nose) {
            value = value + 10;
        }
        if self.mine_field.contains(&stern)  {
            value = value - 25;
        }
        if self.mine_field.contains(point) {
            value = value - 25;
        }
        if self.mine_field.contains(&nose) {
            value = value - 25;
        }
        if self.under_fire.contains_key(&stern) /*&& (*self.under_fire.get(&stern).unwrap() == depth)*/ {
            value = value - 25;
        }
        if self.under_fire.contains_key(point) /*&& (*self.under_fire.get(point).unwrap() <= depth+1)*/  {
            value = value - 50;
        }
        if self.under_fire.contains_key(&nose)/* && (*self.under_fire.get(&nose).unwrap() == depth)*/  {
            value = value - 25;
        }
        value
    }

    fn check_collision(&self, point: &Point, rotation: i32, ship_id: i32) -> bool {
        let nose = point.get_neighbour(rotation);
        for ship in self.my_ships.values() {
            if !ship.is_alive(self.current_tick) {
                continue;
            }
            if ship.entity_id == ship_id {
                continue;
            }
            let sp_nose = ship.point.get_neighbour(rotation);
            let sp_stern = ship.point.get_neighbour((rotation + 3)%6);
            if (ship.point == nose) || (sp_nose == nose) || (sp_stern == nose) {
                return true;
            }
        }
        for ship in self.enemy_ships.values() {
            if !ship.is_alive(self.current_tick) {
                continue;
            }
            let sp_nose = ship.point.get_neighbour(ship.rotation);
            let sp_stern = ship.point.get_neighbour((ship.rotation + 3)%6);
            if (ship.point == nose) || (sp_nose == nose) || (sp_stern == nose) {
                return true;
            }
        }
        false
    }

    fn move_recur(&self, dest: &Point, point: &Point, mut rotation: i32, mut speed: i32, action: Action, depth: i32, ship_id: i32) -> (i32, bool) {
        let mut t_point = *point;
        let d = point.distance(&dest);
        let angle = point.angle(&dest);
        let angle_straight = f64::min((rotation as f64 - angle).abs(), 6f64 - (rotation as f64  - angle).abs());
        let mut collision = false;
        match action {
            Action::WAIT => {
                for _ in 0..speed {
                    let t = t_point.get_neighbour(rotation);
                    if self.check_collision(&t, rotation, ship_id) {
                        collision = true;
                        break;
                    }
                    t_point = t;
                }
            },
            Action::PORT => {
                for _ in 0..speed {
                    let t = t_point.get_neighbour(rotation);
                    if self.check_collision(&t, rotation, ship_id) {
                        collision = true;
                        break;
                    }
                    t_point = t;
                }
            },
            Action::STARBOARD => {
                for _ in 0..speed {
                    let t = t_point.get_neighbour(rotation);
                    if self.check_collision(&t, rotation, ship_id) {
                        collision = true;
                        break;
                    }
                    t_point = t;
                }
                
            },
            Action::SLOWER => {
                if speed > 0 {
                    speed = speed - 1
                }
                for _ in 0..speed {
                    let t = t_point.get_neighbour(rotation);
                    if self.check_collision(&t, rotation, ship_id) {
                        collision = true;
                        break;
                    }
                    t_point = t;
                }
            },
            Action::FASTER => {
                if speed < 2 {
                    speed = speed + 1
                }
                for _ in 0..speed {
                    let t = t_point.get_neighbour(rotation);
                    if self.check_collision(&t, rotation, ship_id) {
                        collision = true;
                        break;
                    }
                    t_point = t;
                }
            },
            _ => unimplemented!(),
        }
        let mut value = self.check_position(&t_point, rotation, speed, depth);
        if collision {
            return (value, true);
        }
        if action == Action::STARBOARD {
            rotation = (rotation + 5)%6;      
            if self.check_collision(&t_point, rotation, ship_id) {
                return (value, true);
            }
            value = value + self.check_position(&t_point, rotation, speed, depth);
        }
        if action == Action::PORT {
            rotation = (rotation + 1)%6;  
            if self.check_collision(&t_point, rotation, ship_id) {
                return (value, true);
            }
            value = value + self.check_position(&t_point, rotation, speed, depth);
        }
        let d_new = t_point.distance(&dest);
        let angle_new = t_point.angle(&dest);
        let angle_straighte_new = f64::min((rotation as f64 - angle_new).abs(), 6f64 - (rotation as f64  - angle_new).abs());
        if d_new < d {
            value = value + 1;
        }
        if angle_straighte_new < angle_straight {
            value = value + 1;
        }
        if depth < 3 {
            let (t_val, _) = self.move_recur(dest, &t_point, rotation, speed, Action::WAIT, depth+1, ship_id);
            let mut m_val = t_val/2;
            let (t_val, _) = self.move_recur(dest, &t_point, rotation, speed, Action::PORT, depth+1, ship_id);
            m_val = cmp::max(m_val, t_val/2);
            let (t_val, _) = self.move_recur(dest, &t_point, rotation, speed, Action::STARBOARD, depth+1, ship_id);
            m_val = cmp::max(m_val, t_val/2);
            let (t_val, _) = self.move_recur(dest, &t_point, rotation, speed, Action::FASTER, depth+1, ship_id);
            m_val = cmp::max(m_val, t_val/2);
            let (t_val, _) = self.move_recur(dest, &t_point, rotation, speed, Action::SLOWER, depth+1, ship_id);
            m_val = cmp::max(m_val, t_val/2);
            value = value + m_val;
        }
        (value, false)
    }

    fn move_to(&self, dest: &Point, point: &Point, rotation: i32, speed: i32, ship_id: i32) -> Action {
        let (mut value, _) = self.move_recur(dest, point, rotation, speed, Action::WAIT, 1, ship_id);
        let mut result = Action::WAIT;
        let (t_val1, t_coll) = self.move_recur(dest, point, rotation, speed, Action::PORT, 1, ship_id);
        if (t_val1 >= value) && (!t_coll) {
            value = t_val1;
            result = Action::PORT; 
        }
        let (t_val2, t_coll) = self.move_recur(dest, point, rotation, speed, Action::STARBOARD, 1, ship_id);
        if (t_val2 >= value) && (!t_coll) {
            value = t_val2;
            result = Action::STARBOARD; 
        }
        let (t_val3, t_coll) = self.move_recur(dest, point, rotation, speed, Action::FASTER, 1, ship_id);
        if (t_val3 >= value) && (speed < 2) && (!t_coll) {
            value = t_val3;
            result = Action::FASTER; 
        }
        let (t_val4, t_coll) = self.move_recur(dest, point, rotation, speed, Action::SLOWER, 1, ship_id);
        if (t_val4 >= value) && (speed > 0) && (!t_coll) {
            value = t_val4;
            result = Action::SLOWER; 
        }
        print_err!("{} {} {} {} {}", value, t_val1, t_val2, t_val3, t_val4);
        result
    }

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
        self.barrels_field.clear();
        self.mine_field.clear();
        for cannonball in self.cannonballs.values() {
            if !cannonball.is_alive(self.current_tick) {
                continue;
            }
            self.under_fire.insert(cannonball.target, cannonball.impact_time);
        }
        for mine in self.mines.values_mut() {
            if !mine.is_alive(self.current_tick) {
                continue;
            }
            if self.under_fire.contains_key(&mine.point) {
                mine.set_under_fire();
            }
            self.mine_field.insert(mine.point);
        }
        for barrel in self.barrels.values() {
            if !barrel.is_alive(self.current_tick) {
                continue;
            }
            self.barrels_field.insert(barrel.point);
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

    fn get_target(&self, ship: &Ship) -> i32 {  
        let nose = ship.point.get_neighbour(ship.rotation);
        let stern = ship.point.get_neighbour((ship.rotation + 3)%6); 
        for enemy_ship in self.enemy_ships.values() {
            if !enemy_ship.is_alive(self.current_tick) {
                continue;
            }
            let mut t_pos = enemy_ship.point; 
            for _ in 0..enemy_ship.speed {
                t_pos = t_pos.get_neighbour(enemy_ship.rotation);
                let sp_nose = t_pos.get_neighbour(enemy_ship.rotation);
                if (sp_nose == ship.point) || (sp_nose == nose) || (sp_nose == stern) {
                    return enemy_ship.entity_id;
                }
            }
        }
        -1
    }

    fn get_closest_target(&self, ship: &Point) -> i32 {  
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

    fn get_waypoint(ship: &Ship) -> (Point, usize) {
        let waypoints = vec![Point::new(2,2), Point::new(20, 2), Point::new(2, 18), Point::new(20, 18)];
        let d = ship.point.distance(&waypoints[ship.wp_ind]);
        let mut wp_ind = ship.wp_ind;
        if d < 2 {
            wp_ind = (ship.wp_ind + 1)%4;
        }
        (waypoints[wp_ind], wp_ind)
    }

    fn do_next_turn(&mut self){
        for key in self.my_ships_ids.iter() {
            let mut wp_ind = 100;
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
                
                if (action == Action::WAIT) && (ship.cd == 0) {
                    let enemy_id = self.get_target(&ship);
                    if enemy_id > 0 {
                        let enemy_ship = self.enemy_ships.get(&enemy_id).unwrap();
                        action = Action::FIRE(enemy_ship.point.x, enemy_ship.point.y);
                    }
                } 
                if (action == Action::WAIT) && (barrel_id >= 0) {
                    let barel = self.barrels.get(&barrel_id).unwrap();
                    print_err!("MOVE HEAL {} {}", barel.point.x, barel.point.y);
                    action = self.move_to(&barel.point, &ship.point, ship.rotation, ship.speed, ship.entity_id);
                } else if action == Action::WAIT {
                    let (p_t, twp_ind) = Game::get_waypoint(&ship);
                    wp_ind = twp_ind;
                    print_err!("MOVE AWAY {} {}", p_t.x, p_t.y);
                    action = self.move_to(&p_t, &ship.point, ship.rotation, ship.speed, ship.entity_id);     
                }
                if (action == Action::WAIT) && (ship.cd == 0) {
                    let enemy_id = self.get_closest_target(&ship.point);
                    let enemy_d = ship.point.distance(&ship.point);
                    
                    let enemy_ship = self.enemy_ships.get(&enemy_id).unwrap();
                    let offset = if enemy_ship.speed == 0 {0} else {1 + (enemy_d) / 3};
                    let point = enemy_ship.point.get_offset(enemy_ship.rotation, offset);
                    let distance = ship.point.distance(&point);
                    if distance < 4  {
                        action = Action::FIRE(point.x, point.y);
                    }
                }
            }   
            let mut m_ship = self.my_ships.get_mut(&key).unwrap();
            if wp_ind != 100 {
                m_ship.wp_ind = wp_ind;
            }
            match action {
                Action::WAIT => {println!("WAIT")},
                Action::PORT => {println!("PORT")},
                Action::STARBOARD => {println!("STARBOARD")},
                Action::SLOWER => {println!("SLOWER")},
                Action::FASTER => {println!("FASTER")},
                Action::FIRE(x, y) => {             
                    m_ship.set_cd(2);
                    println!("FIRE {} {} YARRR", x, y)
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
