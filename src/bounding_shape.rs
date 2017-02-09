
use vector2d::Vector2D;

/// Type of collision shape
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeType {
    /// No collision wanted
    NoShape,
    /// AA box
    Rectangle{ width: f64, height: f64 },
    /// Circle
    Circle{ radius: f64 },
    /// Arbitrary line
    Line { position2: Vector2D },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingShape {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub shape_type: ShapeType,
}


pub fn collides(shape1: BoundingShape, shape2: BoundingShape) -> Option<Vector2D> {
    use self::ShapeType::*;
    match (shape1.shape_type, shape2.shape_type) {
        (NoShape {..}, _) | (_, NoShape{..}) => None,

        (Rectangle{ width: width1, height: height1 },
         Rectangle{ width: width2, height: height2 }) => {
             // Rectalge2 inside rectangle1 ?
             let p1 = shape1.position;
             let p2 = shape1.position + Vector2D{ x: width1, y: height1 };
             let diff_vec = shape2.position - shape1.position;


             if inside_rectangle(p1, p2, shape2.position) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape2.position + Vector2D{ x: width2, y: 0.0 }) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape2.position + Vector2D{ x: 0.0, y: height2 }) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape2.position + Vector2D{ x: width2, y: height2 }) { return Some(diff_vec); }

             // Rectalge1 inside rectangle2 ?
             let p1 = shape2.position;
             let p2 = shape2.position + Vector2D{ x: width2, y: height2 };
             let diff_vec = shape2.position - shape1.position;


             if inside_rectangle(p1, p2, shape1.position) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape1.position + Vector2D{ x: width1, y: 0.0 }) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape1.position + Vector2D{ x: 0.0, y: height1 }) { return Some(diff_vec); }
             if inside_rectangle(p1, p2, shape1.position + Vector2D{ x: width1, y: height1 }) { return Some(diff_vec); }

             None
        },

        (Rectangle{ width: width1, height: height1 },
         Circle{ radius: radius2 }) => {
             None // TODO
        },

        (Rectangle{ width: width1, height: height1 },
         Line{ position2 }) => {
             None // TODO
        },

        (Circle{ .. },
         Rectangle{ .. }) => {
             flip_vector(collides(shape2, shape1))
        },

        (Circle{ radius: radius1 },
         Circle{ radius: radius2 }) => {
             let diff_vec = shape2.position - shape1.position;
             let diff = diff_vec.length();
             if diff < radius1 + radius2 {
                 Some(diff_vec)
             } else { None }
        },
        (Circle{ radius: radius1 },
         Line{ position2 }) => {
             None // TODO
        }

        (Line{ .. },
         Rectangle{ .. }) => {
             flip_vector(collides(shape2, shape1))
        },

        (Line{ .. },
         Circle{ .. }) => {
             flip_vector(collides(shape2, shape1))
        }

        (Line{ position2 },
         Line{ position2: position4 }) => {
             None // TODO
        }
    }
}

/// p1: top left corner of Rectangle,
/// p2: bottom right corner of Rectangle,
/// point: is this point inside the Rectangle ?
pub fn inside_rectangle(p1: Vector2D, p2: Vector2D, point: Vector2D) -> bool {
    (point.x >= p1.x && point.x <= p2.x) &&
    (point.y >= p1.y && point.y <= p2.y)
}

pub fn flip_vector(vec: Option<Vector2D>) -> Option<Vector2D> {
    if let Some(Vector2D{ x, y }) = vec {
        Some(Vector2D{ x: -x, y: -y })
    } else { None }
}
