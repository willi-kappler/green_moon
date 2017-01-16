
use vector2d::Vector2D;

pub enum BoundingShape {
    Box{ width: f64, height: f64 },
    Circle{ radius: f64 },
}

pub fn collides(shape1: BoundingShape, position1: Vector2D,
                shape2: BoundingShape, position2: Vector2D) -> Option<Vector2D> {
    match (shape1, shape2) {
        (BoundingShape::Box{ width: width1, height: height1 },
         BoundingShape::Box{ width: width2, height: height2 }) => {
             None
        },

        (BoundingShape::Box{ width: width1, height: height1 },
         BoundingShape::Circle{ radius: radius2 }) => {
             None
        },

        (BoundingShape::Circle{ radius: radius1 },
         BoundingShape::Box{ width: width2, height: height2 }) => {
             None
        },

        (BoundingShape::Circle{ radius: radius1 },
         BoundingShape::Circle{ radius: radius2 }) => {
             None
        },

    }
}
