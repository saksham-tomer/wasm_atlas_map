use std::fs;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    alert(
        "Hello, from the rust crap from the bottom of the sea"
    );
}

struct CustomData<T> {
    input_data: Vec<T>,
    error_msg: String,
}

impl<T> CustomData<T> {
    fn new() -> Self {
        CustomData {
            input_data: Vec::new(),
            error_msg: String::new(),
        }
    }

    fn init(&mut self, data: &[T]) where T: Clone {
        self.input_data.clear();
        for item in data {
            self.input_data.push(item.clone());
        }
    }

    fn set_error(&mut self, msg: String) {
        self.error_msg = msg;
    }

    fn get_data(&self) -> &Vec<T> {
        &self.input_data
    }
}

fn main() -> std::io::Result<()> {
    let res1 = fs::write("map.json", "{
    \"name\": \"hello json\"
    }");
    let res2 = fs::write("test.txt", "this is a test");

    //testing out the dir reading function
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    let mut cst_data = CustomData::<i32>::new();
    cst_data.init(&[1,4,6]);
    println!("Data: {:?}", cst_data.get_data());

    match res1 {
        Ok(_) => println!(
            "File created Successfully !!!!!!!",
        ),
        Err(error) => println!(
            "Error in creating a file {:?}", error
        )
    }
    match res2 {
        Ok(_) => println!(
            "File created Successfully !!!!!!!",
        ),
        Err(error) => println!("Error Creating File {:?}", error),
        _ => println!("Unknown Error")
    }

    Ok(())
}