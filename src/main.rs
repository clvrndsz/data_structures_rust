fn main() {
    println!("Hello, world!");
}

pub struct DynamicArray {
    pub arr : [i32; 2],     // TODO: make generic
    pub len: usize,
    pub capacity: usize,    // TODO: add default
}

impl DynamicArray {
    
    fn new(&mut self) {
        self.arr = [0, 0];
        self.len = 2;
        self.capacity = 4;
    }

    fn size(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn get(&self, index: usize) -> Result<i32, &str> {
        if index >= self.len {
            return Err("index is out of bounds")
        }
        Ok(self.arr[index])
    }

    fn set(&mut self, index: usize, value: i32) -> Result<(), &str> {
        if index > self.len {
            return Err("index is out of bounds")
        }
        self.arr[index] = value;
        Ok(())
    }

    fn append(&mut self, value :i32)-> () {
        if self.len + 1 >= self.capacity {
            self.capacity *= 2;
        }
        let new_arr = [i32; self.arr.len()+ 1];
        self.arr = [i32; self.arr.len() + 1];
        self.len += 1;
        self.arr[len] = value;
    }

    fn clear(&mut self)-> () {
        self.arr = [0;2];
        self.len = 1;
        self.capacity = 1;
    }

    fn remove_at(&mut self, index: usize)-> Result<(),&str>  {
        if index >= self.len {
            return Err("index is out of bounds")
        }
        self.arr = [self.arr[..index], self.arr[index..]].concat();
        Ok(())
    }

    fn remove(&mut self, value: i32) {
        

    }

fn indexOf {

}

fn contains -> bool  {
    
}

//impl iterator 
//// provides the ability to traverse the array without 
//// the for loop setup a la provding abstraction
//{
//    hasNext();
//    next();
//}
//
//fn toString {
//
//}
//



}
