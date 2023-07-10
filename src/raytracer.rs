use glam::Vec3;
use crate::Context;
use crate::scene::Sphere;
use crate::scene::LightTypes;

pub(crate) fn trace_ray(context: &Context, origin: Vec3, dir: Vec3, t_min: f32, t_max: f32) -> (u8, u8, u8){
    let mut closest_t = t_max;
    let mut closest_sphere: Option<&Sphere> = None;

    let binding = context.scene.get_spheres();
    for sphere in binding.iter() {
        let (t1, t2) = ray_intersect_sphere(origin, dir, sphere);

        if (t1 >= t_min && t1 <= t_max) && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        
        if (t2 >= t_min && t2 <= t_max) && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    };

    match closest_sphere {
        Some(sphere) => {
            let point: Vec3 = origin + closest_t * dir;
            let mut normal: Vec3 = point - sphere.position;
            normal = normal / normal.length();

            let color = sphere.color * compute_lighting(context, point, normal, -dir, sphere.specular);
            color.into()
        },
        None => context.get_background()
    }
}

fn ray_intersect_sphere(origin: Vec3, dir: Vec3, sphere: &Sphere) -> (f32, f32) {
    let r = sphere.radius;
    let c_to_o = origin - sphere.position;

    let a = dir.dot(dir);
    let b = 2.0 * c_to_o.dot(dir);
    let c = c_to_o.dot(c_to_o) - r*r;

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        (f32::INFINITY, f32::INFINITY)
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2.0*a);
        let t2 = (-b - discriminant.sqrt()) / (2.0*a);

        (t1, t2)
    }
}

fn compute_lighting(context: &Context, point: Vec3, normal: Vec3, point_to_cam: Vec3, specular: i32) -> f32 {
    let mut intensity = 0.0;

    let binding = context.scene.get_lights();
    for light in binding.iter() {
        match light.t {
            LightTypes::Ambient => intensity += light.intensity,
            LightTypes::Point => {
                match light.position {
                    Some(pos) => {
                        intensity += calc_light_intensity(light.intensity, pos - point, normal);
                        intensity += calc_reflection_intensity(light.intensity, pos - point, normal, point_to_cam, specular);
                    }
                    None => panic!("No position found for point light!"),
                }
            }
            LightTypes::Directional => {
                match light.direction {
                    Some(dir) => {
                        intensity += calc_light_intensity(light.intensity, dir.into(), normal);
                        intensity += calc_reflection_intensity(light.intensity, dir.into(), normal, point_to_cam, specular);
                    },
                    None => panic!("No direction found for directional light!"),
                }
            }
        };

    }

    intensity
}

fn calc_light_intensity(light_intensity: f32, light: Vec3, normal: Vec3) -> f32 {
    let n_dot_l = normal.dot(light);

    if n_dot_l > 0.0 {
        return light_intensity * n_dot_l / (normal.length() * light.length());
    } 

    0.0
}

fn calc_reflection_intensity(light_intensity: f32, light: Vec3, normal: Vec3, point_to_cam: Vec3, specular: i32) -> f32 {
    let n_dot_l = normal.dot(light);

    if specular != -1 {
        let reflection = 2.0 * normal * n_dot_l - light;
        let r_dot_v = reflection.dot(point_to_cam);

        if r_dot_v > 0.0 {
            let temp = r_dot_v/(reflection.length()*point_to_cam.length());

            return light_intensity * temp;
        } 
    } 

    0.0
}
