use rays::projectile;
use rays_core::Canvas;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut c = Canvas::new(900, 500, None);
    let mut file = File::create(&Path::new("./tmp/projectile.ppm")).unwrap();

    projectile::simulate_projectile(&mut c);
    
    file.write_all(c.to_ppm().as_bytes()).unwrap();

    println!("image saved in tmp/projectile.ppm");
}
