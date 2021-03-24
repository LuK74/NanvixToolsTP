use std::env;
use std::io::prelude::Read;
use std::fs::File;

fn contain_bytes_vec(file_bytes : &Vec<u8>, looking_for : &Vec<u8>) -> bool {
    let mut cursor = 0;
    //println!("Looking for {:?}", looking_for);
    //println!("In {:?}", file_bytes);
    while cursor <= file_bytes.len()-looking_for.len() {
        if file_bytes[cursor..cursor+looking_for.len()] == looking_for[..] {
            return true;
        }
        cursor += 1;
        //println!("{:?}", file_bytes[cursor..cursor+4]);
    }
    return false;
}

fn look_for_entry(bytes : Vec<u8>, looking_for : Vec<u8>) {
    let mut hash = 0;
    let mut current_bytes = bytes.iter().map(|x| x + hash).collect();
    while !contain_bytes_vec(&current_bytes, &looking_for) && hash <= 128 {
        current_bytes = bytes.iter().map(|x| if hash > *x {
            hash - x
        } else {
            x - hash
        }).collect();

        hash += 1;
    }

    if hash > 128 {
        println!("No result");
    } else {
        println!("Result of decoder for a hash of {}", hash-1);
        println!("{:?}", current_bytes);

        println!("{:?}", String::from_utf8(current_bytes).unwrap());
    }
}

fn convert_string_to_u8(filename : String) -> Option<Vec<u8>> {
    let mut file = File::open(filename).unwrap();
    let mut bytes : String = String::new(); 

    let mut returned_vec : Vec<u8> = Vec::new();

    let res = file.read_to_string(&mut bytes); 

    if let Ok(_n) = res {
        let mut number = bytes.split_whitespace();
        while let Some(value) = number.next() {
            returned_vec.push(value.parse::<u8>().unwrap());
        }
    } else {
        return None;
    }

    return Some(returned_vec);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let looking_for = Vec::from(args[2].as_bytes());
    println!("{:?}", args);
    if args.len() == 3 {
        look_for_entry(convert_string_to_u8(args[1].clone()).unwrap(), looking_for);
    } else {
        println!("Unknown usage");
        println!("Usage : cargo run [passwordFilename] [stringLookedFor]");
        println!("Example : cargo run password root");
    }

}
