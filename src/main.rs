mod animals;
mod zee;

fn main() {
    println!("{}", animals::bird::explanation());
    println!("{}", animals::dog::explanation());
    println!("{}", animals::cat::explanation());

    use zee::breeder;
    breeder::greet();
}
