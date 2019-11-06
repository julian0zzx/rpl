
fn main() {
    println!("hello world~~");

    let mut a = 3;

    println!("{}", a);

//    let (x, y) = (3, 4);

    a = 5;

    println!("{}", a);

    let d = dd(a);

    println!("to double {}, result is {}", a, d);

//    let pp  = ptn();

//    let dv: String = diverage();


      let v = vec![0; 10];
      println!("v.len = {}", v.len());

      let v2 = &v[1 .. 6];
      println!("v2.len = {}", v2.len());

      let v3 = v2;
      println!("v2.len = {}", v2.len());
      println!("v3.len = {}", v3.len());

      let mut vv = vec![1, 4, 7, 11];
//      let aa = av(vv);
//      println!("{}", aa[1]);

      let _bb = avv(&mut vv);
      println!("len: {}", vv.len());
      av(vv);
}

fn av(mut v : Vec<i32>) -> Vec<i32> {
   for a in v.iter() {
      println!("{}", a);
   }
   v.push(65);
   v
}

fn avv(v : &mut Vec<i32>)  {
   for a in v {
       println!("{}", a);
   }
//   *vv.push(99);
}

fn dd(x: i32) -> i32 {
 return   x + x;
//    x + x
}

//fn ptn()  {
//   println!("xxx");
//}

//fn diverage() -> ! {
//   panic!("This function never returns!");
//}
