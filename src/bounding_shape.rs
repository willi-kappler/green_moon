
use vector2d::Vector2D;

/// Type of collision shape
pub enum BoundingShape {
    /// No collision wanted
    None { position: Vector2D },
    /// AA box
    Box{ position: Vector2D, width: f64, height: f64 },
    /// Circle
    Circle{ position: Vector2D, radius: f64 },
    /// Arbitrary line
    Line { position1: Vector2D, position2: Vector2D },
}

pub fn collides(shape1: BoundingShape, shape2: BoundingShape) -> Option<Vector2D> {
    match (shape1, shape2) {
        (BoundingShape::None {..}, _) | (_, BoundingShape::None{..}) => None,
        (BoundingShape::Box{ position: position1, width: width1, height: height1 },
         BoundingShape::Box{ position: position2, width: width2, height: height2 }) => {
             None // TODO
        },

        (BoundingShape::Box{ position: position1, width: width1, height: height1 },
         BoundingShape::Circle{ position: position2, radius: radius2 }) => {
             None // TODO
        },

        (BoundingShape::Box{ position: position1, width: width1, height: height1 },
         BoundingShape::Line{ position1: position2, position2: position3 }) => {
             None // TODO
        },

        (BoundingShape::Circle{ position: position1, radius: radius1 },
         BoundingShape::Box{ position: position2, width: width2, height: height2 }) => {
             collides(BoundingShape::Box { position: position1, width: width2, height: height2 },
                      BoundingShape::Circle { position: position2, radius: radius1 })
        },

        (BoundingShape::Circle{ position: position1, radius: radius1 },
         BoundingShape::Circle{ position: position2, radius: radius2 }) => {
             let diff_vec = position2 - position1;
             let diff = diff_vec.length();
             if diff < radius1 + radius2 {
                 Some(diff_vec)
             } else { None }
        },
        (BoundingShape::Circle{ position: position1, radius: radius1 },
         BoundingShape::Line{ position1: position2, position2: position3 }) => {
             None // TODO
        }

        (BoundingShape::Line{ position1: position1, position2: position2 },
         BoundingShape::Box{ position: position3, width: width1, height: height1 }) => {
             None // TODO
        },

        (BoundingShape::Line{ position1: position1, position2: position2 },
         BoundingShape::Circle{ position: position3, radius: radius1 }) => {
             None // TODO
        }

        (BoundingShape::Line{ position1: position1, position2: position2 },
         BoundingShape::Line{ position1: position3, position2: position4 }) => {
             None // TODO
        }
    }
}
