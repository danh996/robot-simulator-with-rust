// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x: x, y: y, d: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_dir = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        println!("robot value in re phai {:?}", self);

        Robot {
            x: self.x,
            y: self.y,
            d: new_dir,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        println!("turn left function is called");
        let new_dir = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        println!("robot value in re trai {:?}", self);

        Robot {
            x: self.x,
            y: self.y,
            d: new_dir,
        }
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let new_x: i32;
        let new_y: i32;
        match self.d {
            Direction::North => {
                new_x = self.x;
                new_y = self.y + 1;
            }
            Direction::West => {
                new_x = self.x - 1;
                new_y = self.y;
            }
            Direction::South => {
                new_x = self.x;
                new_y = self.y - 1;
            }
            Direction::East => {
                new_x = self.x + 1;
                new_y = self.y;
            }
        };

        self.x = new_x;
        self.y = new_y;
        self.d = self.d;

        println!("robot value in tieng {:?}", self);

        Robot {
            x: new_x,
            y: new_y,
            d: self.d,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let char_vec: Vec<char> = instructions.chars().collect();
        let mut testrobot = self;
        for c in char_vec {
            if c == 'L' {
                testrobot = testrobot.turn_left();
            } else if c == 'R' {
                testrobot = testrobot.turn_right();
            } else {
                testrobot = testrobot.advance();
            }
        }
        println!("robot value in dieu khien function {:?}", testrobot);
        testrobot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        match self.d {
            Direction::North => &Direction::North,
            Direction::East => &Direction::East,
            Direction::South => &Direction::South,
            Direction::West => &Direction::West,
        }
    }
}

fn main() {
    let robot = Robot::new(0, 0, Direction::East);

    let new_robot = robot.instructions("LA");
    println!("robot value run with instructions is {:?}", new_robot);
}
