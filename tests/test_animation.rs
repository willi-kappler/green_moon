extern crate green_moon;

use green_moon::{
    Animation,
    NO_ANIMATION,
    ANIMATE_ONCE,
};

#[test]
fn test_no_animation() {
    let mut animation = Animation::new(vec![(12, 100)], NO_ANIMATION);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(0);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.pause();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.resume();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.reset();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.set_animation_frames(vec![(12, 100), (13, 200), (14, 150)]);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);


}

#[test]
fn test_animate_once() {
    let mut animation = Animation::new(vec![(12, 100), (16, 120), (14, 150), (21, 200)], ANIMATE_ONCE);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(0);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(99);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 16);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 16);

    animation.next(120);
    assert_eq!(animation.current_sprite_frame(), 14);

    animation.next(120);
    assert_eq!(animation.current_sprite_frame(), 14);

    animation.next(150);
    assert_eq!(animation.current_sprite_frame(), 21);

    animation.next(150);
    assert_eq!(animation.current_sprite_frame(), 21);

    animation.next(200);
    assert_eq!(animation.current_sprite_frame(), 21);

    animation.reset();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.pause();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.resume();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(200);
    assert_eq!(animation.current_sprite_frame(), 16);

    animation.reset();
    assert_eq!(animation.current_sprite_frame(), 12);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 16);

    animation.set_animation_frames(vec![(99, 200), (100, 220), (50, 240)]);
    assert_eq!(animation.current_sprite_frame(), 99);

    animation.next(100);
    assert_eq!(animation.current_sprite_frame(), 99);

    animation.next(200);
    assert_eq!(animation.current_sprite_frame(), 100);

    animation.next(200);
    assert_eq!(animation.current_sprite_frame(), 100);

    animation.next(220);
    assert_eq!(animation.current_sprite_frame(), 50);

    animation.next(220);
    assert_eq!(animation.current_sprite_frame(), 50);

    animation.next(250);
    assert_eq!(animation.current_sprite_frame(), 50);

}
