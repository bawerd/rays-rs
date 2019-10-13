use rays_core::tuples::{ Vector, Point, point, vector };
use rays_core::canvas::*;
use rays_core::color::*;


struct Projectile {
    pos: Point,
    vel: Vector
}

struct Env {
    gravity: Vector,
    wind: Vector
}

pub fn simulate_projectile(canvas: &mut Canvas) {
    let mut p = Projectile { pos: point(0., 1., 0.), vel: vector(1., 1.8, 0.).normalize() * 11.25 };
    let env = Env { gravity: vector(0., -0.1, 0.), wind: vector(-0.01, 0., 0.) };
    let mut ticks = 0;

    let color = Color::new(1., 0., 0.);

    while p.pos.y >= 0. {
        //println!("{} - x: {}, y: {}", ticks, p.pos.x.round() as u32, p.pos.y.round() as u32);
        canvas.write_pixel(p.pos.x.round() as u32, (canvas.height - p.pos.y.round() as u32) , color);
        p = tick(&env, p);
        ticks += 1;
    }
}

fn tick(env: &Env, p: Projectile) -> Projectile {
    Projectile { 
        pos: p.pos + p.vel,
        vel: p.vel + env.gravity + env.wind
    }
}