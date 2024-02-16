enum Sopranos {
    Tony,
    Sil,
    Paulie,
}

impl Sopranos {
    fn say_hi(&self) {
        match &self {
            Sopranos::Paulie => println!("✅"),
            _ => println!("❌"),
        }
        println!("hello");
    }
}

fn get_soprano(soprano: Sopranos) -> String {
    let name = match soprano {
        Sopranos::Tony => "Tony",
        Sopranos::Sil => "Sil",
        Sopranos::Paulie => "Paulie",
    };
    return String::from(name);
}

fn sopranos() {
    let file = std::fs::read_to_string("file").unwrap();

    let arr = vec![1, 2, 3];
    let res: Vec<i32> = arr.iter().map(|x| x + 1).collect();
    println!("{:?}", res);

    file.lines().enumerate().skip(1).for_each(|(i, l)| {
        if i % 2 == 0 {
            println!("{i}: {l}");
        }
    });

    Sopranos::Sil.say_hi();
    Sopranos::Paulie.say_hi();

    let name = get_soprano(Sopranos::Tony);
    println!("{name}");
}

fn multiply(num: Option<i32>) -> i32 {
    let num = match num {
        Some(arg) => arg * 5,
        None => 0,
    };
    println!("{num}");
    return num;
}

fn exercise_one() {
    let res = multiply(Option::Some(5));
    println!("{res}");
    let res = multiply(Option::None);
    println!("{res}");
}

fn practice(nums: Vec<usize>, index: usize) -> usize {
    let val = match nums.get(index) {
        Some(value) => value,
        None => &index,
    };
    return val * 5;
}

fn exercise_two() {
    let res = practice(vec![1, 2, 3], 1);
    println!("{res}");
    let res = practice(vec![1, 2, 3], 4);
    println!("{res}");
}

fn exercise_three() {
    use std::env::args;
    use std::fs::read_to_string;

    fn main() {
        let file = args().nth(1).expect("No arg passed");
        let file = read_to_string(file).expect("Thats not a file");
        file.lines().for_each(|line| println!("{line}"));

        let mut list: Vec<i32> = vec![1, 2, 3];
        {
            let num: i32 = 4;
            list.push(num);
        }

        println!("{:?}", list);
    }
}

fn main() {
    sopranos();
    exercise_one();
    exercise_two();
    exercise_three();
    println!("---");
}
