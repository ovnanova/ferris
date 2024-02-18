mod animator;
mod frames;
use frames::Frames;

fn main() {
    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");
    // Hide cursor
    print!("\x1B[?25l");

    let frames = Frames::new().data;
    let animator = animator::Animator::new(frames);
    animator.run();

    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");
    // Show cursor again
    print!("\x1B[?25h");
}