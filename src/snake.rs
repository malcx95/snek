use rand::Rng;
use crate::constants;
use std::collections::VecDeque;


#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


#[derive(Copy, Clone)]
pub struct SnakeSegment {
    pub pos: (i32, i32),
    pub dir: Direction
}

pub struct Snake {
    segments: VecDeque<SnakeSegment>,
}


impl Snake {
    
    pub fn new() -> Snake {
        
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0, constants::BOARD_WIDTH - 1);
        let start_y = rng.gen_range(0, constants::BOARD_HEIGHT - 1);
        
        let initial_segment = SnakeSegment { 
            pos: (start_x as i32, start_y as i32),
            dir: Direction::Up
        };

        let mut segments = VecDeque::new();
        segments.push_back(initial_segment);

        Snake {
            segments: segments
        }
    }

    fn wrap_around(pos: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        let mut new_x = x;
        let mut new_y = y;
        if x < 0 {
            new_x = constants::BOARD_WIDTH - 1;
        } else if x >= constants::BOARD_WIDTH {
            new_x = 0;
        }
        if y < 0 {
            new_y = constants::BOARD_HEIGHT - 1;
        } else if y >= constants::BOARD_HEIGHT {
            new_y = 0;
        }
        (new_x, new_y)
    }

    pub fn update(&mut self) {
        let curr_dir = self.segments.front().unwrap().dir;
        let (curr_x, curr_y) = self.segments.front().unwrap().pos;
        let new_pos = match curr_dir {
            Direction::Up => Snake::wrap_around((curr_x, curr_y - 1)),
            Direction::Down => Snake::wrap_around((curr_x, curr_y + 1)),
            Direction::Left => Snake::wrap_around((curr_x, curr_y - 1)),
            Direction::Right => Snake::wrap_around((curr_x, curr_y + 1)),
        };
        let new_segment = SnakeSegment {
            pos: new_pos,
            dir: curr_dir
        };
        self.segments.pop_back();
        self.segments.push_front(new_segment);
    }

}
