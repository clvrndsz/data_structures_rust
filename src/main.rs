fn main() {
    println!("Hello, world!");
}

pub struct DynamicArray {
    pub data: vec::new(),
    pub capacity: usize,    // TODO: add default
}


    // NOTE: giving up on using arrays as they suck
    // with the whole stack assigned fixed length bs. 
    // i would have had to clone each item in an array
    // individually as the borrow checker would not
    // allow me to concat two diff lengths of arrays.
    
// WARN: stop effort here, vecs have everything unfortunately.
impl DynamicArray{

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn get(&self, index: usize) -> Result<i32, &str> {
        if index >= self.data.len() {
            return Err("index is out of bounds")
        }
        Ok(self.data[index])
    }

    fn set(&mut self, index: usize, value: i32) -> Result<(), &str> {
        if index > self.data.len() {
            return Err("index is out of bounds")
        }
        self.data[index] = value;
        Ok(())
    }

    fn append(&mut self, value :i32)-> () {
        if self.data.len() + 1 >= self.capacity {
            self.capacity *= 2;
        }
        let new_data = [self.data, [value]].join();
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
