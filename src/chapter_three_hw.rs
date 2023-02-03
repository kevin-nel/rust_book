pub fn f_to_c(f: f32) -> f32 {
    let c: f32 = (f - 32.0) * 5.0 / 9.0;
    println!("{}℉ = {:.2}℃", f, c);
    c
}

pub fn fib_n(n: i32) -> Vec<i32> {
    let mut fibs: Vec<i32> = vec![0];

    if n == 0 {
        fibs = [0].to_vec();
    } else if n == 1 {
        fibs = [0, 1].to_vec();
    } else {
        fibs = [0, 1].to_vec();
        for _ in 2..n {
            fibs.push(fibs[fibs.len() - 1] + fibs[fibs.len() - 2]);
        }
    }
    println!("{:?}", fibs);
    fibs
}

pub fn twelve_days_of_xmas() {
    let things: [&str; 12] = [
        "A partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    for n in 1..=12 {
        if n == 1 {
            println!("On the {}st day of Christmas My true love sent to me.", n);
        } else if n == 2 {
            println!("On the {}nd day of Christmas My true love sent to me.", n);
        } else if n == 3 {
            println!("On the {}rd day of Christmas My true love sent to me.", n);
        } else {
            println!("On the {}th day of Christmas My true love sent to me.", n);
        }
        for day in (1..n).rev() {
            println!("{} {}", day + 1, things[day]);
        }
        println!("{}", things[0]);
    }
}
