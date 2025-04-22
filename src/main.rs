use std::io::{self};

fn main() {
    println!("what do you want to do?");
    println!("'1' to encode '2' to decode:");
    //creates a input variable and reads line for it
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    //makes the input easy to work with and checks if its valid 
    let input: u8 = input.trim().parse().expect("please type a number from '1' to '2'");

    if input == 1 {
        encode();
    }
    else if input == 2 {
        decode();
    }
    println!("do you want to run the code again? Y for yes N for no: ");
    //creates a input variable and reads line for it
    let mut yes_or_no =  String::new();
    io::stdin()
    .read_line(&mut yes_or_no)
    .expect("failed to read line");

    // makes the input easy to work with 
    let yes_or_no = yes_or_no.trim().to_uppercase();

    if  yes_or_no == "Y" {
        main();
    }
}

fn encode() {
    //create input variables
    let mut string = String::new();
    let mut key = String::new();

    //getting input for string
    println!("please enter the string: ");
    io::stdin()
        .read_line(&mut string)
        .expect("failed to read line");

    //getting input for key
    println!("please create a key: ");
    io::stdin()
        .read_line(&mut key)
        .expect("failed to read line");

    //converting ihe input to get it ready for encode
    let string = remove_whitespace(&string.to_uppercase());
    let key = remove_whitespace(&key.to_uppercase());
    //getting len 
    let key_len = key.len() as u32;
    let string_len = string.len() as u32;

    if string.is_empty() || key.is_empty() { //checking if strings are empty
        panic!("inputs cant be empty");
    }
    //printing the input after getting it ready for encode
    println!("\n\n   string: {string}");
    println!("   key: {key}");

    let mut output = String::new(); // creating a variables

    for i in 0..string_len - 1 { //looping to encode every string char
        let mut x = i.clone(); //a mutable clone of i for the key

        while x > key_len - 2{ //looping the key so it to never ends
            x = x - (key_len - 1); 
        }
        //converting variables to usize data type
        let x = x as usize;
        let i = i as usize;
        //getting chars to encode
        let string_char = string.chars().nth(i).unwrap();
        let key_char = key.chars().nth(x).unwrap();

        output.push(encode_vigenere_table(string_char, key_char)); // passing chars to encode function
    }
    //output of the encoded string 
    println!("encoded string: {}", output);
    println!("remember to save the key!!! {}", key);
}

fn encode_vigenere_table(string_char: char, key_char: char) -> char {  // used to encode and decode vigenere cypher
    let char = string_char.to_ascii_uppercase() as usize;
    let key = key_char.to_ascii_uppercase() as i8;
    // converts ASCII chars to numbers
    let mut abc = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    // alphabet starting the generation of vigenere table
    for _i in 0..key - 65 {
        // println!("{:?}", ..abc.clone());
        abc.rotate_left(1);
    } // generates the table
    return abc[char - 65] as char; // return encoded char 
}

fn decode() { //turning encoded string intro decoded string
    //create variables for input 
    let mut encoded_string = String::new();
    let mut key = String::new();

    println!("encoded string: ");
    io::stdin() //reading line for encode_string
        .read_line(&mut encoded_string)
        .expect("failed to read line");

    println!("key: ");
    io::stdin() // reading line for key
        .read_line(&mut key)
        .expect("failed to read line");

    //getting the strings ready to decode them  
    let encoded_string = remove_whitespace(&encoded_string.trim().to_uppercase());
    let key = remove_whitespace(&key.trim().to_uppercase());
    //create variables len() of key and encoded_string;
    let encoded_string_len = encoded_string.len() as u32;
    let key_len = key.len() as u32;

    if encoded_string.is_empty() || key.is_empty() { //checking if strings are empty
        panic!("inputs cant be empty");
    }

    let mut output = String::new();

    for i in 0..encoded_string_len {
        let mut x = i.clone(); //a mutable clone of i for the key

        while x > key_len - 1{ //looping the key so it to never ends
            x = x - key_len; 
        }
        //converting variables to usize data type
        let x = x as usize;
        let i = i as usize;
        //getting chars to encode
        let encoded_char = encoded_string.chars().nth(i).unwrap();
        let key_char = key.chars().nth(x).unwrap();

        output.push(decode_vigenere_table(encoded_char, key_char)); // passing chars to decode function 
    }
    // prints the decoded string
    println!("decoded string: {}", output);
}

fn decode_vigenere_table(encoded_char: char, key_char: char) -> char {  // decodes the encoded char based on the key value
    let key = key_char.to_ascii_uppercase() as usize; // converts the key_char to ASCII index 
    let mut abc = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']; 

    loop { // decodes by looping the vec and outputs the first index if char on the index of the key is the same as the encoded char
        if abc[key -65] == encoded_char { return abc[0]; } 
        abc.rotate_left(1); // pushes the first index to the back of the vec
    }
}
fn remove_whitespace(s: &str) -> String { //removes every whitespace from a string
    s.replace(" ", "") //replace whitespaces with nothing 
}
