struct Click {
    x: u32,
    callback: Box<dyn Fn(u32) -> u32>,
}

fn main() {
    let y = 444u32;
    let click = Click {x: 123, callback: Box::new(|x| {x + y})};
    println!("{}", click.callback.as_ref()(321));
}
