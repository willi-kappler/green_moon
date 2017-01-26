
extern crate green_moon;

use green_moon::Vector2D;
use green_moon::{BoundingShape, ShapeType, collides};


#[test]
fn test_box_box01() {
    let b1 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 5.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 3.0, height: 2.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 4.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: 0.0, y: -1.0 }));
}

#[test]
fn test_box_box02() {
    let b1 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 4.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 5.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: 0.0, y: 1.0 }));
}

#[test]
fn test_box_box03() {
    let b1 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 3.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: 5.0, y: 5.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), None);
}

#[test]
fn test_box_box04() {
    let b1 = BoundingShape{
        position: Vector2D{ x: -2.0, y: -2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 4.0, height: 4.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: 2.0, y: 2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: 4.0, y: 4.0 }));
}

#[test]
fn test_box_box05() {
    let b1 = BoundingShape{
        position: Vector2D{ x: -2.0, y: -2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 4.0, height: 4.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: -3.0, y: 2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: -1.0, y: 4.0 }));
}

#[test]
fn test_box_box06() {
    let b1 = BoundingShape{
        position: Vector2D{ x: -2.0, y: -2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 4.0, height: 4.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: -3.0, y: -3.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: -1.0, y: -1.0 }));
}

#[test]
fn test_box_box07() {
    let b1 = BoundingShape{
        position: Vector2D{ x: -2.0, y: -2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 4.0, height: 4.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: 2.0, y: -3.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: 4.0, y: -1.0 }));
}

#[test]
fn test_box_box08() {
    let b1 = BoundingShape{
        position: Vector2D{ x: -2.0, y: -2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0},
        shape_type: ShapeType::Rectangle{ width: 4.0, height: 4.0 },
    };
    let b2 = BoundingShape{
        position: Vector2D{ x: -1.0, y: -1.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Rectangle{ width: 1.0, height: 1.0 },
    };

    assert_eq!(collides(b1, b2), Some(Vector2D{ x: 1.0, y: 1.0 }));
}

#[test]
fn circle_circle01()  {
    let c1 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 2.0 },
    };
    let c2 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 4.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 1.0 },
    };

    assert_eq!(collides(c1, c2), None);
}

#[test]
fn circle_circle02()  {
    let c1 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 2.0 },
    };
    let c2 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 3.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 1.0 },
    };

    assert_eq!(collides(c1, c2), None);
}

#[test]
fn circle_circle03()  {
    let c1 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 2.0 },
    };
    let c2 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 2.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 1.0 },
    };

    assert_eq!(collides(c1, c2), Some(Vector2D{ x: 0.0, y: 2.0 }));
}

#[test]
fn circle_circle04()  {
    let c1 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 2.0 },
    };
    let c2 = BoundingShape{
        position: Vector2D{ x: 1.0, y: 1.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 1.0 },
    };

    assert_eq!(collides(c1, c2), Some(Vector2D{ x: 1.0, y: 1.0 }));
}

#[test]
fn circle_circle05()  {
    let c1 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 2.0 },
    };
    let c2 = BoundingShape{
        position: Vector2D{ x: 0.0, y: 0.0 },
        velocity: Vector2D{ x: 0.0, y: 0.0 },
        shape_type: ShapeType::Circle{ radius: 1.0 },
    };

    assert_eq!(collides(c1, c2), Some(Vector2D{ x: 0.0, y: 0.0 }));
}
