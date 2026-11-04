use crate::vector::*;
use crate::unpack_rgb;

#[derive(Debug)]
pub struct Sphere {
    pub r: f64,
    pub pos: Vec3,
    pub colour: Vec3,
}

impl Sphere {
    fn new(radius: f64, position: Vec3, colour: Vec3) -> Self {
        Self {
            r: radius,
            pos: position,
            colour,
        }
    }

    pub fn does_intersect(&self, ray_pos: &Vec3, ray_dir: &Vec3, t: &mut f64) -> bool {
        let v: Vec3 = *ray_pos - self.pos;
        let a = dot(*ray_dir,*ray_dir);
        let b = 2.0 * dot(*ray_dir, v);
        let c = dot(v, v) - (self.r * self.r);

        match solve_quadratic(a, b, c) {
            Some((t1, t2)) => {
                let mut t_hit = f64::MAX;
                
                if t1 > 0.0 {
                    t_hit = t1;
                }
                if t2 > 0.0 && t2 < t_hit {
                    t_hit = t2;
                }
                
                // case for it either intersection point is negative or sphere is behind camera
                if t_hit == f64::MAX {  
                    return false;
                }

                *t = t_hit;
                true
            }

            None => false   // case for no intersection
        }
    }
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let descriminant = (b * b) - (4.0 * a * c);

    // does not intersect
    if descriminant < 0.0 {
        return None;
    }
    
    let sqrt_d = descriminant.sqrt();

    let t1 = (-b + sqrt_d) / (2.0 * a);
    let t2 = (-b - sqrt_d) / (2.0 * a);

    Some((t1, t2))
}

pub fn create_sphere(mut instructions: std::str::SplitWhitespace<'_>, colours: &Vec<u32>) -> Sphere {
    let x: f64 = instructions.next().unwrap().parse().unwrap();
    let y: f64 = instructions.next().unwrap().parse().unwrap();
    let z: f64 = instructions.next().unwrap().parse().unwrap();

    let radius: f64 = instructions.next().unwrap().parse().unwrap();

    let sphere_col_index: usize = instructions.next().unwrap().parse().unwrap();
    let sphere_rgb = unpack_rgb(colours[sphere_col_index]);
    
    Sphere {
        pos: Vec3 {
            x: x,
            y: y,
            z: z,
        },
        r: radius,
        colour: sphere_rgb,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------
    // helpers
    // -------------------------
    fn v(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn ray_dir_normalized(x: f64, y: f64, z: f64) -> Vec3 {
        let len = (x*x + y*y + z*z).sqrt();
        v(x / len, y / len, z / len)
    }

    // -------------------------
    // solve_quadratic tests
    // -------------------------

    #[test]
    fn quadratic_no_real_roots() {
        // x^2 + 1 = 0 → no real roots
        let res = solve_quadratic(1.0, 0.0, 1.0);
        assert!(res.is_none());
    }

    #[test]
    fn quadratic_two_roots() {
        // x^2 - 5x + 6 = 0 → roots 2 and 3
        let res = solve_quadratic(1.0, -5.0, 6.0).unwrap();

        let mut roots = [res.0, res.1];
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!((roots[0] - 2.0).abs() < 1e-9);
        assert!((roots[1] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn quadratic_double_root() {
        // (x - 2)^2 = x^2 - 4x + 4
        let res = solve_quadratic(1.0, -4.0, 4.0).unwrap();

        assert!((res.0 - 2.0).abs() < 1e-9);
        assert!((res.1 - 2.0).abs() < 1e-9);
    }

    // -------------------------
    // Sphere intersection tests
    // -------------------------

    #[test]
    fn ray_hits_sphere_front() {
        let sphere = Sphere {
            r: 1.0,
            pos: v(0.0, 0.0, 5.0),
            colour: v(1.0, 0.0, 0.0),
        };

        let ray_pos = v(0.0, 0.0, 0.0);
        let ray_dir = v(0.0, 0.0, 1.0);

        let mut t = 0.0;
        let hit = sphere.does_intersect(&ray_pos, &ray_dir, &mut t);

        assert!(hit);
        assert!(t > 0.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere {
            r: 1.0,
            pos: v(5.0, 5.0, 5.0),
            colour: v(1.0, 0.0, 0.0),
        };

        let ray_pos = v(0.0, 0.0, 0.0);
        let ray_dir = v(0.0, 0.0, 1.0);

        let mut t = 0.0;
        let hit = sphere.does_intersect(&ray_pos, &ray_dir, &mut t);

        assert!(!hit);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let sphere = Sphere {
            r: 2.0,
            pos: v(0.0, 0.0, 0.0),
            colour: v(1.0, 0.0, 0.0),
        };

        let ray_pos = v(0.0, 0.0, 0.0); // inside sphere
        let ray_dir = v(1.0, 0.0, 0.0);

        let mut t = 0.0;
        let hit = sphere.does_intersect(&ray_pos, &ray_dir, &mut t);

        assert!(hit);
        assert!(t > 0.0);
    }

    #[test]
    fn ray_hits_tangent_sphere() {
        let sphere = Sphere {
            r: 1.0,
            pos: v(0.0, 1.0, 5.0),
            colour: v(1.0, 0.0, 0.0),
        };

        let ray_pos = v(0.0, 0.0, 5.0);
        let ray_dir = v(0.0, 1.0, 0.0);

        let mut t = 0.0;
        let hit = sphere.does_intersect(&ray_pos, &ray_dir, &mut t);

        assert!(hit);
    }

    #[test]
    fn sphere_behind_camera() {
        let sphere = Sphere {
            r: 1.0,
            pos: v(0.0, 0.0, -5.0),
            colour: v(1.0, 0.0, 0.0),
        };

        let ray_pos = v(0.0, 0.0, 0.0);
        let ray_dir = v(0.0, 0.0, 1.0);

        let mut t = 0.0;
        let hit = sphere.does_intersect(&ray_pos, &ray_dir, &mut t);

        assert!(!hit);
    }
}