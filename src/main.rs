    fn fizz_buzz() {
        let mut result = 0;
        let mut count = 0;
        loop {
            count +=1;
            let fizz = count % 3 == 0;
            let buzz = count % 5 == 0;
            let fizz_buzz = fizz && buzz;

            if fizz_buzz {
                result += 1;
                println!("fizz buzz");
            }
            else if buzz {
                println!("buzz");
            }
            else if fizz {
                //result += 1;
                println!("fizz");
            }
            else if count == 301 {
                println!("Number of fizz buzz {}", result);
                break
            }
        }
    }

fn main() {
    println!("Hello, Welcome!");
    fizz_buzz();
}
