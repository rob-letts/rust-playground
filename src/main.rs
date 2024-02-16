mod player;
use player::{bronco::Bronco, cheer::Cheer, default::Default};

fn _add_one(bronco: &mut Bronco) {
    bronco.id += 1;
}

fn main() {
    let patrick = Bronco {
        name: "Surtain".to_string(),
        id: 2,
    };

    // add_one(&mut patrick);
    // println!("{:?}", patrick);

    let justin = Bronco {
        name: "Simmons".to_string(),
        id: 31,
    };

    let rookie = Bronco::default();

    let broncos: Vec<Bronco> = vec![patrick, justin, rookie];

    print_all(&broncos);
}

fn print_all(broncos: &Vec<Bronco>) {
    broncos.iter().for_each(|item| item.lets_go());
}
