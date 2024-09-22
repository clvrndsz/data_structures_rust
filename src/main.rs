fn main() {
    println!("Hello, world!");
}

pub struct DynamicArray {
    pub arr : [i32; 2],     // TODO: make generic
    pub len: usize,
    pub capacity: usize,    // TODO: add default
}

impl DynamicArray {

    fn size(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn get(&self, index: usize) -> i32 {
        self.arr[index]
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
            if self.capacity == 0 {
                self.capacity = 1;
            } else {
                self.capacity *= 2;
            }
        }
        self.arr[self.len] = value;
        self.len += 1;
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
        self.arr = {
        let left = self.arr[..index];
        let right  = self.arr.[index..];
        self.arr
        };
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
