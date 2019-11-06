use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    // t_closure();

    t_iter();
}


fn t_closure() {
    let innn = intensive_calculating(5);

    let intens_cal = | ints | { // change to mut to change something outside
        println!("... calculating ...{} ", ints);
        thread::sleep(Duration::from_secs(innn as u64)); // as 
        // innn = innn + 1;
        ints
    };
    println!("closure, {} ", intens_cal(3));

    let add_1 = | input : i32 | -> i32 {
        input + 1
    };

    println!("closure, {} ", add_1(3));

    // cached closure
    gen_workout(5, 3);

    t_move();
}

fn intensive_calculating(intensity : u32) -> u32 {
    println!("... calculating ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn gen_workout(intensity : u32, random_num : u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("... calculating ... --- --- {} ", num);
        thread::sleep(Duration::from_secs(2)); // as 
        return num;
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T> where T : Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // cached here
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn t_move() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("still here, x? {:?}", x); // x moved
    let zz = vec![1, 2, 3];

    println!("z == x ? {}", equal_to_x(zz));
}

fn t_iter() {
    let v1 = vec![1, 2, 4, 6];
    let mut vi = v1.iter();
    println!("{:?}", vi);
    // for i in vi {
    //     println!("{:?}", i);
    // }
    assert_eq!(vi.next(), Some(&1)); // next should be on a mut iterator

    let sum : i32 = v1.iter().sum();
    assert_eq!(sum, 13);


    let v2 /* : Vec<i32>  */ = vi.map( |x| x * 2 );
    println!("{:?}", v2);

    let v3 : Vec<_> = v1.iter().map(|x| x + 1).collect(); // Vec<i32>
    println!("{:?}", v3);

    let v4 : Vec<_> = v1.into_iter().filter(|x| x % 2 == 0).collect(); // |x| *x %2 or v1.into_iter()
    println!("{:?}", v4);

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // amazing!
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}

#[derive(Debug)]
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter{ count : 0}
    }
}

impl Iterator for Counter {
    type Item = u32; // Item for Iterator, type in the iterator

    fn next(&mut self) -> Option<Self::Item> { // next for Iterator
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


