use tarantool::proc;

#[proc]
fn gant() {
    println!("hello world");
}
