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
        Self::vector2d_dot(v, v)
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
        Self::vector3d_dot(v, v)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{Vector2d, Vector3d};

    const EPSILON: f32 = 1e-6;

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < EPSILON, "Expected {}, got {}", b, a);
    }

    fn assert_vector2d_approx_eq(a: Vector2d, b: Vector2d) {
        assert_approx_eq(a.0, b.0);
        assert_approx_eq(a.1, b.1);
    }

    fn assert_vector3d_approx_eq(a: Vector3d, b: Vector3d) {
        assert_approx_eq(a.0, b.0);
        assert_approx_eq(a.1, b.1);
        assert_approx_eq(a.2, b.2);
    }

    #[test]
    fn test_vector2d_add() {
        let v1 = (1.0, 2.0);
        let v2 = (3.0, 4.0);
        let result = Component::vector2d_add(v1, v2);
        assert_vector2d_approx_eq(result, (4.0, 6.0));
    }

    #[test]
    fn test_vector2d_sub() {
        let v1 = (5.0, 7.0);
        let v2 = (2.0, 3.0);
        let result = Component::vector2d_sub(v1, v2);
        assert_vector2d_approx_eq(result, (3.0, 4.0));
    }

    #[test]
    fn test_vector2d_dot() {
        let v1 = (3.0, 4.0);
        let v2 = (2.0, 1.0);
        let result = Component::vector2d_dot(v1, v2);
        assert_approx_eq(result, 10.0); // 3*2 + 4*1 = 10
    }

    #[test]
    fn test_vector2d_mul() {
        let v = (2.0, 3.0);
        let scalar = 2.5;
        let result = Component::vector2d_mul(v, scalar);
        assert_vector2d_approx_eq(result, (5.0, 7.5));
    }

    #[test]
    fn test_vector2d_div() {
        let v = (6.0, 9.0);
        let scalar = 3.0;
        let result = Component::vector2d_div(v, scalar);
        assert_vector2d_approx_eq(result, (2.0, 3.0));
    }

    #[test]
    fn test_vector2d_neg() {
        let v = (3.0, -2.0);
        let result = Component::vector2d_neg(v);
        assert_vector2d_approx_eq(result, (-3.0, 2.0));
    }

    #[test]
    fn test_vector2d_sqr_length() {
        let v = (3.0, 4.0);
        let result = Component::vector2d_sqr_length(v);
        // Should be 3² + 4² = 9 + 16 = 25, but current implementation is wrong
        assert_approx_eq(result, 25.0);
    }

    #[test]
    fn test_vector2d_length() {
        let v = (3.0, 4.0);
        let result = Component::vector2d_length(v);
        // Should be sqrt(3² + 4²) = sqrt(25) = 5.0
        assert_approx_eq(result, 5.0);
    }

    #[test]
    fn test_vector2d_normalize() {
        let v = (3.0, 4.0);
        let result = Component::vector2d_normalize(v);
        assert_vector2d_approx_eq(result, (0.6, 0.8));
        
        // Test that normalized vector has length 1
        let length = Component::vector2d_length(result);
        assert_approx_eq(length, 1.0);
    }

    #[test]
    fn test_vector2d_normalize_zero_vector() {
        let v = (0.0, 0.0);
        let result = Component::vector2d_normalize(v);
        assert_vector2d_approx_eq(result, (0.0, 0.0));
    }

    #[test]
    fn test_vector3d_add() {
        let v1 = (1.0, 2.0, 3.0);
        let v2 = (4.0, 5.0, 6.0);
        let result = Component::vector3d_add(v1, v2);
        assert_vector3d_approx_eq(result, (5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3d_sub() {
        let v1 = (7.0, 8.0, 9.0);
        let v2 = (2.0, 3.0, 4.0);
        let result = Component::vector3d_sub(v1, v2);
        assert_vector3d_approx_eq(result, (5.0, 5.0, 5.0));
    }

    #[test]
    fn test_vector3d_dot() {
        let v1 = (1.0, 2.0, 3.0);
        let v2 = (4.0, 5.0, 6.0);
        let result = Component::vector3d_dot(v1, v2);
        assert_approx_eq(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_vector3d_mul() {
        let v = (2.0, 3.0, 4.0);
        let scalar = 2.0;
        let result = Component::vector3d_mul(v, scalar);
        assert_vector3d_approx_eq(result, (4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vector3d_div() {
        let v = (6.0, 9.0, 12.0);
        let scalar = 3.0;
        let result = Component::vector3d_div(v, scalar);
        assert_vector3d_approx_eq(result, (2.0, 3.0, 4.0));
    }

    #[test]
    fn test_vector3d_neg() {
        let v = (1.0, -2.0, 3.0);
        let result = Component::vector3d_neg(v);
        assert_vector3d_approx_eq(result, (-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_vector3d_sqr_length() {
        let v = (2.0, 3.0, 6.0);
        let result = Component::vector3d_sqr_length(v);
        // Should be 2² + 3² + 6² = 4 + 9 + 36 = 49
        assert_approx_eq(result, 49.0);
    }

    #[test]
    fn test_vector3d_length() {
        let v = (2.0, 3.0, 6.0);
        let result = Component::vector3d_length(v);
        // Should be sqrt(2² + 3² + 6²) = sqrt(49) = 7.0
        assert_approx_eq(result, 7.0);
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = (3.0, 0.0, 4.0);
        let result = Component::vector3d_normalize(v);
        assert_vector3d_approx_eq(result, (0.6, 0.0, 0.8));
        
        // Test that normalized vector has length 1
        let length = Component::vector3d_length(result);
        assert_approx_eq(length, 1.0);
    }

    #[test]
    fn test_vector3d_normalize_zero_vector() {
        let v = (0.0, 0.0, 0.0);
        let result = Component::vector3d_normalize(v);
        assert_vector3d_approx_eq(result, (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_point2d_add_vector2d() {
        let p = (2.0, 3.0);
        let v = (1.0, 4.0);
        let result = Component::point2d_add_vector2d(p, v);
        assert_vector2d_approx_eq(result, (3.0, 7.0));
    }

    #[test]
    fn test_point2d_sub_vector2d() {
        let p = (5.0, 7.0);
        let v = (2.0, 3.0);
        let result = Component::point2d_sub_vector2d(p, v);
        assert_vector2d_approx_eq(result, (3.0, 4.0));
    }

    #[test]
    fn test_point3d_add_vector3d() {
        let p = (1.0, 2.0, 3.0);
        let v = (4.0, 5.0, 6.0);
        let result = Component::point3d_add_vector3d(p, v);
        assert_vector3d_approx_eq(result, (5.0, 7.0, 9.0));
    }

    #[test]
    fn test_point3d_sub_vector3d() {
        let p = (10.0, 8.0, 6.0);
        let v = (2.0, 3.0, 1.0);
        let result = Component::point3d_sub_vector3d(p, v);
        assert_vector3d_approx_eq(result, (8.0, 5.0, 5.0));
    }

    // Edge case tests
    #[test]
    fn test_vector2d_div_by_zero() {
        let v = (1.0, 1.0);
        let result = Component::vector2d_div(v, 0.0);
        // Should handle division by zero gracefully (typically results in infinity)
        assert!(result.0.is_infinite() || result.0.is_nan());
        assert!(result.1.is_infinite() || result.1.is_nan());
    }

    #[test]
    fn test_vector3d_div_by_zero() {
        let v = (1.0, 1.0, 1.0);
        let result = Component::vector3d_div(v, 0.0);
        // Should handle division by zero gracefully (typically results in infinity)
        assert!(result.0.is_infinite() || result.0.is_nan());
        assert!(result.1.is_infinite() || result.1.is_nan());
        assert!(result.2.is_infinite() || result.2.is_nan());
    }

    #[test]
    fn test_orthogonal_vectors_dot_product() {
        // Orthogonal 2D vectors should have dot product 0
        let v1 = (1.0, 0.0);
        let v2 = (0.0, 1.0);
        let result = Component::vector2d_dot(v1, v2);
        assert_approx_eq(result, 0.0);

        // Orthogonal 3D vectors should have dot product 0
        let v1 = (1.0, 0.0, 0.0);
        let v2 = (0.0, 1.0, 0.0);
        let result = Component::vector3d_dot(v1, v2);
        assert_approx_eq(result, 0.0);
    }

    #[test]
    fn test_antiparallel_vectors_dot_product() {
        // Antiparallel vectors should have negative dot product
        let v1 = (1.0, 0.0);
        let v2 = (-1.0, 0.0);
        let result = Component::vector2d_dot(v1, v2);
        assert_approx_eq(result, -1.0);

        let v1 = (2.0, 0.0, 0.0);
        let v2 = (-3.0, 0.0, 0.0);
        let result = Component::vector3d_dot(v1, v2);
        assert_approx_eq(result, -6.0);
    }
}
