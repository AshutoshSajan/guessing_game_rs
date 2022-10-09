use std::io::stdin;
// use std::any::type_name;

// // function to check the datatype
// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

fn main() {
    guess()
}

fn guess() {
    println!("Guess the number: ");

    let num = 33;
    const MAX_TRY: i32 = 5;
    let mut count = 0;

    while count < MAX_TRY {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("failed to read line");

        println!("guess {}", guess);

        let guess_num: i32 = guess.trim().parse::<i32>().unwrap();

        println!("guess_num {}", guess_num);
        println!("count {}", count);
        println!("try left {}", MAX_TRY - (count + 1));

        if guess_num == num {
            println!("yep correct");
        } else if guess_num != num {
            println!("nope");
        }

        count += 1
    }
}
