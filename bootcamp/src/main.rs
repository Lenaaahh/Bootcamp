

fn main() {
    fizz_buzz();
}

fn fizz_buzz(){
    let mut c = 0;
    for x in 1..=301 {

        match (x % 3, x % 5){
            (0, 0) => {println!("FizzBuzz");
                c+=1;
            }
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_,_)=>{}

        }

        
    }println!("{}",c);
}