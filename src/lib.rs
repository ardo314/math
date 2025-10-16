use crate::bindings::Guest;


#[allow(warnings)]
mod bindings;


struct Component;

impl Guest for Component {

    fn vector2d_add(lhs: bindings::Vector2d, rhs: bindings::Vector2d) -> bindings::Vector2d {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
    
    fn vector2d_sub(lhs: bindings::Vector2d, rhs: bindings::Vector2d) -> bindings::Vector2d {
        (lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
    
    fn vector2d_dot(lhs: bindings::Vector2d, rhs: bindings::Vector2d) -> f32 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1
    }
    
    fn vector2d_mul(lhs: bindings::Vector2d, rhs: f32) -> bindings::Vector2d {
        (lhs.0 * rhs, lhs.1 * rhs)
    }
    
    fn vector2d_div(lhs: bindings::Vector2d, rhs: f32) -> bindings::Vector2d {
        (lhs.0 / rhs, lhs.1 / rhs)
    }
    
    fn vector2d_sqr_length(v: bindings::Vector2d) -> f32 {
        Self::vector2d_dot(v, v).sqrt()
    }
    
    fn vector2d_length(v: bindings::Vector2d) -> f32 {
        Self::vector2d_sqr_length(v).sqrt()
    }
    
    fn vector2d_normalize(v: bindings::Vector2d) -> bindings::Vector2d {
        let length = Self::vector2d_length(v);
        if length > 0.0 {
            Self::vector2d_div(v, length)
        } else {
            (0.0, 0.0)
        }
    }

    fn vector2d_neg(v: bindings::Vector2d) -> bindings::Vector2d {
        (-v.0, -v.1)
    }
    
    fn vector3d_add(lhs: bindings::Vector3d, rhs: bindings::Vector3d) -> bindings::Vector3d {
        (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
    }
    
    fn vector3d_sub(lhs: bindings::Vector3d, rhs: bindings::Vector3d) -> bindings::Vector3d {
        (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
    }
    
    fn vector3d_dot(lhs: bindings::Vector3d, rhs: bindings::Vector3d) -> f32 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }
    
    fn vector3d_mul(lhs: bindings::Vector3d, rhs: f32) -> bindings::Vector3d {
        (lhs.0 * rhs, lhs.1 * rhs, lhs.2 * rhs)
    }
    
    fn vector3d_div(lhs: bindings::Vector3d, rhs: f32) -> bindings::Vector3d {
        (lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
    }
    
    fn vector3d_neg(v: bindings::Vector3d) -> bindings::Vector3d {
        (-v.0, -v.1, -v.2)
    }
    
    fn vector3d_sqr_length(v: bindings::Vector3d) -> f32 {
        Self::vector3d_dot(v, v).sqrt()
    }
    
    fn vector3d_length(v: bindings::Vector3d) -> f32 {
        Self::vector3d_sqr_length(v).sqrt()
    }
    
    fn vector3d_normalize(v: bindings::Vector3d) -> bindings::Vector3d {
        let length = Self::vector3d_length(v);
        if length > 0.0 {
            Self::vector3d_div(v, length)
        } else {
            (0.0, 0.0, 0.0)
        }
    }

    fn point2d_add_vector2d(
        p: bindings::Point2d,
        v: bindings::Vector2d,
    ) -> bindings::Point2d {
        (p.0 + v.0, p.1 + v.1)
    }

    fn point2d_sub_vector2d(
        p: bindings::Point2d,
        v: bindings::Vector2d,
    ) -> bindings::Point2d {
        (p.0 - v.0, p.1 - v.1)
    }

    fn point3d_add_vector3d(
        p: bindings::Point3d,
        v: bindings::Vector3d,
    ) -> bindings::Point3d {
        (p.0 + v.0, p.1 + v.1, p.2 + v.2)
    }

    fn point3d_sub_vector3d(
        p: bindings::Point3d,
        v: bindings::Vector3d,
    ) -> bindings::Point3d {
        (p.0 - v.0, p.1 - v.1, p.2 - v.2)
    }
    
    
}

bindings::export!(Component with_types_in bindings);
