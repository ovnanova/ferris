mod animator;
mod frames;
use frames::Frames;

fn main() {
    let frames = Frames::new().data;
    let animator = animator::Animator::new(frames);
    animator.run();
}