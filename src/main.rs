use std::fs;
use std::fs::File;
use std::io::Read;
use image::*;
// use wasm_bindgen::prelude::*;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     alert("Hello, from the rust crap from the bottom of the sea");
// }

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
        println!("{:?} \n", dir.path());
    }

    let cat_image = image::open("download.jpg");
    print!("Image width: {:?} \n", cat_image.unwrap().height());
    let mut file_data = File::open("download.jpg")?;



    let mut image_bmp = File::open("download.jpg")?;
    let mut buffer:Vec<u8> = Vec::new();
    image_bmp.read_to_end(&mut buffer)?;

    if buffer.len() < 54{
        eprint!("Not a BMP file, file too short \n");
        return Ok(());
    }

    let width:u32 = u32::from_le_bytes(buffer[18..22].try_into().unwrap());
    let height: u32 = u32::from_le_bytes(buffer[22..26].try_into().unwrap());

    print!(" \n Image width: {:?}, Image height: {:?} \n", width, height);

    // while let Some(byte) = file_data.bytes().next() {
    //     match byte {
    //         Ok(byte) => println!("{}", byte),
    //         Err(_) => break,
    //     }
    // }

    // while let Ok(byte) = file_data.metadata(){
    //     println!("{:?}", byte);
    // }


    for byte in file_data.bytes() {
        match byte {
            Ok(byte) => fs::write("download_copy.jpg", vec![byte])?,
            Err(_) => break,
        }
    };

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
        _ => println!("Unknown Error")
    }

        Ok(())
    }
