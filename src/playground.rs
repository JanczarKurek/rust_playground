struct Click<'a> {
    x: u32,
    callback: Box<dyn Fn(u32) -> u32 + 'a>,
}

fn main() {
    let y = 444u32;
    let click = Click {x: 123, callback: Box::new(|x: u32| {x + y})};
    println!("{}", click.callback.as_ref()(321));
}
