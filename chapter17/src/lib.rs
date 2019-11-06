
#[derive(Debug)]
pub struct AveragedCollection {
    list : Vec<i32>,
    average : f64
}

impl AveragedCollection {
    pub fn new(v : Vec<i32>) -> AveragedCollection {
        let mut ac = AveragedCollection{list : v, average : 0f64};
        ac.update_average();
        ac
    }
    pub fn add(&mut self, value : i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total : i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


pub trait Draw {
    fn draw(&self);
}

// #[derive(Debug)]
// pub struct Screen <T : Draw> { // use this type of 'trait', all components must be with the same type
//     pub components : Vec<Box<T>>
// }

// impl <T> Screen <T> where T : Draw {
//     pub fn run(&self) {
//         for comp in self.components.iter() {
//             comp.draw();
//         }
//     }
// }


// trait object
pub struct Screen  {
    pub components : Vec<Box<Draw>>
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub label : String
}

impl Draw for Button {
    fn draw(&self) {
        println!("label: {}", self.label);
    }
}

#[derive(Debug)]
pub struct Select {
    pub value : String
}

impl Draw for Select {
    fn draw(&self) {
        println!("selected value: {}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg() {
        let vv = vec![1, 2, 3];
        assert_eq!(2 as f64, AveragedCollection::new(vv).average());
    }

}
