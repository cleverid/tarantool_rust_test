use tarantool::proc;

#[proc]
fn easy() -> i32 {
    println!("hello world");
    123
}

#[proc]
fn easy2() -> i32 {
    println!("hello world -- easy2");
    0
}
