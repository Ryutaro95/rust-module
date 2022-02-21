mod animals;
mod zoo;

fn main() {
    println!("{}", animals::bird::explanation());
    println!("{}", animals::dog::explanation());
    println!("{}", animals::cat::explanation());

    use zoo::breeder;
    breeder::greet();
}
