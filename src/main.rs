


mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;
fn write_ppm(w:i32 , h:i32, max_value:i32){
    println!("P3\n{} {}\n{}",w,h,max_value); 
    for j in (0..h).rev() {
        for i in 0..w {
            let r:f32 = i as f32 / w as f32;
            let g:f32 = j as f32 / h as f32;
            let b:f32 = 0.2;

            let ir : i32 = (255.99*r) as i32;
            let ig : i32 = (255.99*g) as i32;
            let ib : i32 = (255.99*b) as i32;
            println!("{} {} {}",ir,ig,ib)
        }
    }
}
fn hit_sphere(center:Vec3 , radius:f32, r:&Ray) -> bool {
    let oc = r.orign()- center;
    let a= Vec3::dot(r.direction(),r.direction());
    let b=2.0 * Vec3::dot(r.direction(),oc) ;
    let c= Vec3::dot(oc, oc) - radius*radius;

    let discriminant = b* b - 4.0*a*c;

    discriminant>0.0
}
fn color(r: &Ray)-> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5,r){
        return Vec3::new(1.1,0.0,0.0);
    }
    let unit_direction : Vec3 = Vec3::unit_vector(&r.direction());
    let t: f32= 0.5 * (unit_direction.y() + 1.0 );
    Vec3::new(1.0, 1.0, 1.0) *(1.0 -t) + Vec3::new(0.5, 0.7, 1.0) * t 
}



fn main() {
    let w: i32 = 200;
    let h : i32 =100;
    let max_value : i32 = 255;
    let orign:Vec3= Vec3::new(0.0, 0.0  , 0.0);
   let horizontal:Vec3= Vec3::new(4.0, 0.0, 0.0);
   let vertical:Vec3= Vec3::new(0.0, 2.0, 0.);
   let lower_left_corner:Vec3= Vec3::new(-2.0, -1.0, -1.0);

   // write_ppm(width, height, max_value);
   println!("P3\n{} {}\n{}",w,h,max_value);
    for j in (0..h).rev() {
        for i in 0..w {
            let u:f32 = i as f32 / w as f32;
            let v:f32 = j as f32 / h as f32;
            let r: Ray = Ray::ray(orign, lower_left_corner+ horizontal*u+vertical*v);
            let col = color(&r);
            let ir : i32 = (255.99*col.x()) as i32;
            let ig : i32 = (255.99*col.y()) as i32;
            let ib : i32 = (255.99*col.z()) as i32;
            
            println!("{} {} {}",ir,ig,ib);
        }
    }
}
