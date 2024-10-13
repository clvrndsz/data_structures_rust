pub struct DynamicArray<const N: usize> {
    pub data: [i32;N],      // TODO: change to generic
    pub capacity: usize,
}

fn new_array() -> DynamicArray<2>  {
    DynamicArray {
        data: [0;2],
        capacity: 4,  // idk man
    }
}

    // NOTE: giving up on using arrays as they suck
    // with the whole stack assigned fixed length bs. 
    // i would have had to clone each item in an array
    // individually as the borrow checker would not
    // allow me to concat two diff lengths of arrays.

impl <const N: usize> DynamicArray<N>{

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn index_of(&self, value: i32) -> Option<usize> {
        let mut index: usize = 0;
        while index <= self.data.len() {
            if self.data[index] == value {
                return Some(index);
            }
            index +=1;
        }
        None
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

    fn contains(&self, value: i32) -> bool {
        match self.index_of(value) {
            Some(_) => return true,
            None => false,
        }
    }
}
// NOTE: The following methods need to be outside of the impl
// because the size of the array is changed, so I must clone.

fn clear_array<const N: usize>(_input_array: DynamicArray<N>)-> DynamicArray<2> {
    //Notice how you dont have to free the memory of the
    //input array because once it goes out of scope, it evaporates :)
    new_array()
}
//
//fn append<const N: usize, const T: usize> (
//    input_array: DynamicArray<N>, value: i32
//)-> DynamicArray<{T}> {
//    const t_: usize = input_array.data.len() + 1;
//    let output_array: DynamicArray<{t_}>;
//        //let output_array: DynamicArray<T>;
//   // if self.data.len() + 1 >= self.capacity {
//     //       self.capacity *= 2;
//      //  }
//      //  let new_data = [self.data, [value]].join();
//    
//}
//
//
//    fn remove_at(&mut self, index: usize)-> Result<(),&str>  {
//        if index >= self.data.len() {
//            return Err("index is out of bounds")
//        }
//        self.data = [self.data[..index], self.data[index..]].concat();
//        Ok(())
//    }
//
//    fn remove(&mut self, value: i32) {
//
//    }
//

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
