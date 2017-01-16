
use vector2d::Vector2D;

/// Type of collision shape
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoundingShape {
    /// No collision wanted
    NoShape { position: Vector2D },
    /// AA box
    Rectangle{ position: Vector2D, width: f64, height: f64 },
    /// Circle
    Circle{ position: Vector2D, radius: f64 },
    /// Arbitrary line
    Line { position1: Vector2D, position2: Vector2D },
}

pub fn collides(shape1: BoundingShape, shape2: BoundingShape) -> Option<Vector2D> {
    use BoundingShape::*;
    match (shape1, shape2) {
        (NoShape {..}, _) | (_, NoShape{..}) => None,

        (Rectangle{ position: position1, width: width1, height: height1 },
         Rectangle{ position: position2, width: width2, height: height2 }) => {
             // Rectalge2 inside rectangle1 ?
             let p1 = position1;
             let p2 = position1 + Vector2D{ x: width1, y: height1 };
             let diff_vec = position2 - position1;


             if inside_Rectangle(p1, p2, position2) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position2 + Vector2D{ x: width2, y: 0.0 }) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position2 + Vector2D{ x: 0.0, y: height2 }) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position2 + Vector2D{ x: width2, y: height2 }) { return Some(diff_vec); }

             // Rectalge1 inside rectangle2 ?
             let p1 = position2;
             let p2 = position2 + Vector2D{ x: width2, y: height2 };
             let diff_vec = position2 - position1;


             if inside_Rectangle(p1, p2, position1) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position1 + Vector2D{ x: width1, y: 0.0 }) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position1 + Vector2D{ x: 0.0, y: height1 }) { return Some(diff_vec); }
             if inside_Rectangle(p1, p2, position1 + Vector2D{ x: width1, y: height1 }) { return Some(diff_vec); }

             None
        },

        (Rectangle{ position: position1, width: width1, height: height1 },
         Circle{ position: position2, radius: radius2 }) => {
             None // TODO
        },

        (Rectangle{ position: position1, width: width1, height: height1 },
         Line{ position1: position2, position2: position3 }) => {
             None // TODO
        },

        (Circle{ position: position1, radius: radius1 },
         Rectangle{ position: position2, width: width2, height: height2 }) => {
             flip_vector(collides(shape2, shape1))
        },

        (Circle{ position: position1, radius: radius1 },
         Circle{ position: position2, radius: radius2 }) => {
             let diff_vec = position2 - position1;
             let diff = diff_vec.length();
             if diff < radius1 + radius2 {
                 Some(diff_vec)
             } else { None }
        },
        (Circle{ position: position1, radius: radius1 },
         Line{ position1: position2, position2: position3 }) => {
             None // TODO
        }

        (Line{ position1: position1, position2: position2 },
         Rectangle{ position: position3, width: width1, height: height1 }) => {
             flip_vector(collides(shape2, shape1))
        },

        (Line{ position1: position1, position2: position2 },
         Circle{ position: position3, radius: radius1 }) => {
             flip_vector(collides(shape2, shape1))
        }

        (Line{ position1: position1, position2: position2 },
         Line{ position1: position3, position2: position4 }) => {
             None // TODO
        }
    }
}

/// p1: top left corner of Rectangle,
/// p2: bottom right corner of Rectangle,
/// point: is this point inside the Rectangle ?
pub fn inside_Rectangle(p1: Vector2D, p2: Vector2D, point: Vector2D) -> bool {
    (point.x >= p1.x && point.x <= p2.x) &&
    (point.y >= p1.y && point.y <= p2.y)
}

pub fn flip_vector(vec: Option<Vector2D>) -> Option<Vector2D> {
    if let Some(Vector2D{ x, y }) = vec {
        Some(Vector2D{ x: -x, y: -y })
    } else { None }
}
