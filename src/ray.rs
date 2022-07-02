
use crate::vec3::Vec3;
#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Ray {
    a:Vec3,
    b:Vec3
}

impl Ray {
    pub fn ray(a:Vec3,b:Vec3)-> Ray {
        Ray { a: a,b:b}
    }

    pub fn orign(self)-> Vec3 {
        self.a
    }

    pub fn direction(self)-> Vec3 {
        self.b
    }
    pub fn point_at_parameter(self,t:f32) -> Vec3 {
        self.orign()+self.direction()*t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn orign(){
        let a:Vec3= Vec3::new(1.0,2.0,3.0);
        let b:Vec3= Vec3::new( 0.0,2.0,3.0);
        let r:Ray=Ray::ray(a,b);
        println!("{:?}",&r);
        assert_eq!(r.orign(),a);
        assert_eq!(r.direction(),b);
    }
}