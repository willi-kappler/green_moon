extern crate green_moon;

use green_moon::SpriteBuilder;
use green_moon::Sprite;
use green_moon::SpriteSheet;
use green_moon::Vector2D;

#[test]
fn test_move_to() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.move_to(Vector2D { x: 5.0, y: -1.5 } );
    assert_eq!(sprite1.position, Vector2D {x: 5.0, y: -1.5 } );

    sprite1.move_to(Vector2D { x: 22.5, y: 6.0 } );
    assert_eq!(sprite1.position, Vector2D {x: 22.5, y: 6.0 } );
}

#[test]
fn test_move_to_xy() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.move_to_xy(2.5, 7.5);

    assert_eq!(sprite1.position, Vector2D {x: 2.5, y: 7.5 } );
}

#[test]
fn test_move_by() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.move_by(Vector2D { x: 2.0, y: 5.0 } );
    assert_eq!(sprite1.position, Vector2D { x: 2.0, y: 5.0 } );

    sprite1.move_by(Vector2D { x: 1.0, y: -1.0 } );
    assert_eq!(sprite1.position, Vector2D { x: 3.0, y: 4.0 } );
}

#[test]
fn test_move_by_xy() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.move_by_xy(47.0, 22.0);
    assert_eq!(sprite1.position, Vector2D { x: 47.0, y: 22.0 } );

    sprite1.move_by_xy(3.0, -2.0);
    assert_eq!(sprite1.position, Vector2D { x: 50.0, y: 20.0 } );
}

#[test]
fn test_set_velocity() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.set_velocity(Vector2D { x: 10.0, y: 20.0 } );
    assert_eq!(sprite1.velocity, Vector2D { x: 10.0, y: 20.0 } );
}

#[test]
fn test_set_velocity_xy() {
    let mut sprite1 = SpriteBuilder::new()
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.set_velocity_xy(45.0, 88.0);
    assert_eq!(sprite1.velocity, Vector2D { x: 45.0, y: 88.0 } );
}

#[test]
fn test_no_update() {
    let mut sprite1 = SpriteBuilder::new()
        .position_xy(370.0, 520.0)
        .velocity_xy(10.0, 20.0)
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.update(0);

    assert_eq!(sprite1.position, Vector2D { x: 370.0, y: 520.0 } );
}

#[test]
fn test_update_velocity() {
    let mut sprite1 = SpriteBuilder::new()
        .alive(true)
        .position_xy(370.0, 520.0)
        .velocity_xy(10.0, 20.0)
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.update(0);
    assert_eq!(sprite1.position, Vector2D { x: 380.0, y: 540.0 } );

    sprite1.update(0);
    assert_eq!(sprite1.position, Vector2D { x: 390.0, y: 560.0 } );

    sprite1.set_velocity_xy(0.0, 0.0);
    sprite1.update(0);
    assert_eq!(sprite1.position, Vector2D { x: 390.0, y: 560.0 } );
}

#[test]
fn test_update_acceleration() {
    let mut sprite1 = SpriteBuilder::new()
        .alive(true)
        .position_xy(370.0, 520.0)
        .velocity_xy(10.0, 10.0)
        .acceleration_xy(1.0, 5.0)
        .animation(0)
        .sprite_sheet(0)
        .build();

    sprite1.update(0);

    assert_eq!(sprite1.position, Vector2D { x: 380.0, y: 530.0 } );
    assert_eq!(sprite1.velocity, Vector2D { x: 11.0, y: 15.0 } );

    sprite1.update(0);

    assert_eq!(sprite1.position, Vector2D { x: 391.0, y: 545.0 } );
    assert_eq!(sprite1.velocity, Vector2D { x: 12.0, y: 20.0 } );

    sprite1.update(0);

    assert_eq!(sprite1.position, Vector2D { x: 403.0, y: 565.0 } );
    assert_eq!(sprite1.velocity, Vector2D { x: 13.0, y: 25.0 } );
}
