

struct User {
    width: i32,
    height: i32,
}
impl User{
    fn add(&self)->i32{
        self.width+self.height
    }
    fn vec(&self)->i32{
        let mut vector = vec![1, 2, 4, 8];
        vector.push(16);
        vector.push(32);
        vector.push(64);
        println!("{:?}", vector);
    
        let mut v1: Vec<i32> = vec![1, 2, 4, 8];
        let mut v2: Vec<i32> = vec![16, 32, 64,];
        v1.append(&mut v2);
        println!("{:?}", v1);
    
        let mut v = vec![1, 2, 4, 8];
        println!("{}", match v.get(0) {
            Some(value) => value.to_string(),
            None => "None".to_string()
        });
        v.push(100);
        println!("{:?}",v);
        return 1
    }
}

fn main() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(1, 0, 0);
    let origin = Point(2.0, 1.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    let u=User{width:12,height:23};
    println!("{}",u.add());
    println!("{:?}",u.vec());
}