use simple_calculator::tokenize;

fn main() {
  println!("{}", std::env::args().skip(1).collect::<String>());
    println!("{:?}", tokenize::tokenize(std::env::args().skip(1).collect::<String>().chars()))
}
