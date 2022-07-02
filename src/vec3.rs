
use std::ops;


#[derive(Copy, Clone,PartialEq,Debug)]
pub struct Vec3 {
    e:[f32;3],
}


impl Vec3 {
    pub fn new(e0: f32, e1:f32, e2:f32)-> Vec3{
        Vec3 { e: [e0,e1,e2] }
    }
    pub fn x(self)->f32{
        self.e[0]
    }
    pub fn y(self)->f32{
        self.e[1]
    }
    pub fn z(self)->f32{
        self.e[2]
    }
    pub fn unit_vector(v:&Vec3)-> Vec3 {
        *v/v.lenght()
    }

    fn lenght(&self)-> f32 {
        (self.x()*self.x()+self.y()*self.y()+self.z()*self.z()).sqrt()
    }
   pub fn dot(v1:Vec3,v2:Vec3)->f32 {
    v1.x()*v2.x()+ v1.y()*v2.y()+ v1.z()*v2.z()
   }
    
}

impl ops::Add  for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Self) -> Self::Output {
        Vec3 { e:[self.x()+_rhs.x(),
            self.y()+_rhs.y(),
            self.z()+_rhs.z()]}
    }
}
impl  ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e:[self.x()-rhs.x(),
            self.y()-rhs.y(),
            self.z()-rhs.z()]}
    }
    
}
impl  ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        
        Vec3 { e:[self.x()*rhs ,
            self.y()*rhs ,
            self.z()*rhs ]}
    }
    
}
impl  ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        
        Vec3 { e:[self*rhs.x() ,
            self*rhs.y() ,
            self*rhs.z() ]}
    }
    
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs:f32) -> Self::Output {
        Vec3 {
            e:[self.x()/rhs,self.y()/rhs,self.z()/rhs]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let v:Vec3=Vec3::new(2.0,4.0,8.0);
        let v1:Vec3=Vec3::new(2.0, 1.0, -4.0);
        assert_eq!(v.x(),2.0,"finding X");
        assert_eq!(v.y(),4.0,"finding y");
        assert_eq!(v.z(),8.0,"finding z");
    //    assert_eq!(v+v1,Vec3::new(4.2, 5.3, 4.4));
        assert_eq!(v-v1,Vec3::new(0.0,3.0,12.0));
        println!("{:?}",v/2.0);
    }
}