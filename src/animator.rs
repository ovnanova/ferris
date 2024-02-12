use std::{thread, time::Duration};

pub struct Animator<'a> {
    pub frames: Vec<&'a str>,
}

impl<'a> Animator<'a> {
    pub fn new(frames: Vec<&'a str>) -> Self {
        Self { frames }
    }

    pub fn run(&self) {
        let mut index = 0;
        let delay = Duration::from_millis(100);

        loop {
            // Clear screen and move cursor to home position in terminal
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", self.frames[index]);
            index = (index + 1) % self.frames.len();
            thread::sleep(delay);

            if index == 0 {
                break;
            }
        }
    }
}