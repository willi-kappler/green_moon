extern crate green_moon;

use green_moon::Vector2D;

#[test]
fn test_add() {
    let a = Vector2D{ x: 0.5, y: 1.2 };
    let b = Vector2D{ x: 1.5, y: 0.8 };

    assert_eq!(a + b, Vector2D{ x: 2.0, y: 2.0 });
}

#[test]
fn test_add_assign() {
    let mut a = Vector2D{ x: -1.0, y: 6.0 };
    let b = Vector2D{ x: 1.0, y: 3.0 };

    a += b;

    assert_eq!(a, Vector2D{ x: 0.0, y: 9.0 });
}

#[test]
fn test_sub() {
    let a = Vector2D{ x: 20.0, y: -3.0 };
    let b = Vector2D{ x: 10.0, y: 4.0 };

    assert_eq!(a - b, Vector2D{ x: 10.0, y: -7.0 });
}

#[test]
fn test_sub_assign() {
    let mut a = Vector2D{ x: -15.0, y: 6.0 };
    let b = Vector2D{ x: 8.0, y: 3.0 };

    a -= b;

    assert_eq!(a, Vector2D{ x: -23.0, y: 3.0 });
}

#[test]
fn test_mul() {
    let a = Vector2D{ x: 5.0, y: 6.0 };

    assert_eq!(a * 2.0, Vector2D { x: 10.0, y: 12.0 });
}

#[test]
fn test_div() {
    let a = Vector2D{ x: 8.0, y: 12.0 };

    assert_eq!(a / 2.0, Vector2D{ x: 4.0, y: 6.0 });
}

#[test]
fn test_length1() {
    let a = Vector2D{ x: 3.0, y: 4.0 };

    assert_eq!(a.length(), 5.0);
}

#[test]
fn test_length2() {
    let a = Vector2D{ x: 9.5, y: 0.0 };

    assert_eq!(a.length(), 9.5);
}

#[test]
fn test_length3() {
    let a = Vector2D{ x: 0.0, y: 9.5 };

    assert_eq!(a.length(), 9.5);
}

#[test]
fn test_length4() {
    let a = Vector2D{ x: 0.0, y: 0.0 };

    assert_eq!(a.length(), 0.0);
}
