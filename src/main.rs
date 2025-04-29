use std::num::ParseIntError;

#[derive(Debug)]
struct Vigenere {
    string: String,
    key: String,
    encoded_string: String,
}

// Struct that represents Vigenere table:
//
// +---------------------------------------------------------------------------------+
// | #  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z |
// | A  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z |
// | B  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A |
// | C  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B |
// | D  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C |
// | E  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D |
// | F  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E |
// | G  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F | 
// | H  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G |
// | I  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H |
// | J  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I |
// | K  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J |
// | L  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K |
// | M  M  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L |
// | N  N  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M |
// | O  O  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N |
// | P  P  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O |
// | Q  Q  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P |
// | R  R  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q |
// | S  S  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R |
// | T  T  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S |
// | U  U  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T |
// | V  V  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U |
// | W  W  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V |
// | X  X  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W |
// | Y  Y  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X |
// | Z  Z  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O  P  Q  R  S  T  U  V  W  X  Y |
// +---------------------------------------------------------------------------------+
//
// can decode and encode 

impl Vigenere {
    fn build(string: &String, key: &String) -> Vigenere {
        let string = string.to_string();
        let key = key.to_string(); 
        let encoded_string = String::from("test");

        Vigenere { string, key, encoded_string }
    }

    // encodes a string using Vigenere table with the given key.
    // outputs in uppercase, why? I just like uppercase.
    // 
    // example:
    //  -input: key = "rust", string = "cargo"
    //  -output: encoded = "TUJZF"

    fn encode(key: &String, string: &String) -> String {
        let vec = Self::assign_key_to_chars(&key, &string);
        let mut encoded_String = String::new();

        for (key, char) in vec {
            // converts chars to ASCII DEC, and then decreases the value by 65 so A = 0, B = 1, C = 2...
            // that makes them compatible with the ABC array above and with the loop below.
            let key_num = key as usize - 65;
            let char_num = char as usize - 65;

            let mut abc = vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ];
            // goes to the right record of Vigenere table using key.
            abc.rotate_left(key_num); // moves the first index to the last index set amount of times.
            // gets right filed value from the record.
            encoded_String.push(abc[char_num]);
        }

        encoded_String
    }

    // returns a Vector of Tuples so the key chars are assigned to the string chars.
    // turns everything to uppercase, why? I just like uppercase. 
    // if key is shorter then string it will repeat its chars until its as long as string is.
    // if the key is longer then String then extra key chars wont be used.
    // Tuple: (key, string)
    //
    // example:
    //  -input: key = "rust", string = "cargo"
    //  -output: vec = [('R', 'C'), ('U', 'A'), ('S', 'R'), ('T', 'G'), ('R', 'O')]

    fn assign_key_to_chars(key: &String, string: &String) -> Vec<(char, char)> {
        let key: Vec<char> = key.to_uppercase().chars().collect();
        let string = string.to_uppercase();
        let mut vec: Vec<(char, char)> = Vec::new(); // (key, string)

        for (mut i, char) in string.char_indices() {
            while i > key.len() - 1{ // makes the key repeat until the string ends.
                i -= key.len();
            }
            vec.push((key[i], char));
        }
        vec
    }
}

fn main() {
    let string = String::from("cargo");
    let key = String::from("rust");

    let encoded = Vigenere::encode(&key, &string);

    println!("{}", encoded)
}