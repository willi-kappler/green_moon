extern crate green_moon;

use green_moon::{ANIMATE_NONE, ANIMATE_ONCE, ANIMATE_LOOP, ANIMATE_PING_PONG};
use green_moon::AnimationBuilder;

#[test]
fn test_animate_none() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];

    // ANIMATE_NONE is the default
    let mut animation = AnimationBuilder::new()
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Should not advance since we have ANIMATE_NONE
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_animate_none2() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];

    // Set ANIMATE_NONE explicitply
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_NONE)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Should not advance since we have ANIMATE_NONE
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_animate_once() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);

    // End of animation reached, only the last frame should be shown from now on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
}

#[test]
fn test_animate_once_single_rame() {
    let frames = vec![(20, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on, but last frame already reached
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_animate_loop() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_LOOP)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);

    // End of animation reached, start from beginning
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
}

#[test]
fn test_animate_loop_single_rame() {
    let frames = vec![(20, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_LOOP)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on, but last frame already reached
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_animate_ping_pong() {
    let frames = vec![(20, 100), (21, 100), (22, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_PING_PONG)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);

    // Now the animation should move on
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);

    // End of animation reached, move backwards
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_animate_ping_pong_single_rame() {
    let frames = vec![(20, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    // Elapsed time < frame duration, so should not advance
    current_frame = animation.next(0, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);

    // Now the animation should move on, but last frame already reached
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(50, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(99, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(200, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 20);
}

#[test]
fn test_pause_resume() {
    let frames = vec![(20, 100), (21, 100), (22, 100), (23, 100)];
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .set_frames(frames)
        .build().unwrap();

    let mut current_frame = 0;

    assert_eq!(animation.get_sprite_index(current_frame), 20);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);

    // Animation should not advance after call to pause
    animation.pause();

    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 21);

    // Now it should go on
    animation.resume();

    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 22);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 23);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 23);
}

#[test]
fn test_add_frame() {
    let mut animation = AnimationBuilder::new()
        .animation_type(ANIMATE_ONCE)
        .add_frame((40, 150))
        .add_frame((41, 100))
        .add_frame((42, 180))
        .add_frame((43, 210))
        .build().unwrap();

    let mut current_frame = 0;

    assert_eq!(animation.get_sprite_index(current_frame), 40);
    current_frame = animation.next(150, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 41);
    current_frame = animation.next(100, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 42);
    current_frame = animation.next(150, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 42);
    current_frame = animation.next(300, current_frame);
    assert_eq!(animation.get_sprite_index(current_frame), 43);
}
