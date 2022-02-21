pub fn greet() {
    let bird_name = take_charge();
    println!("私は鳥科の{}を担当しています。", bird_name);
}

fn take_charge() -> String {
    use crate::animals::bird;
    bird::name()
}