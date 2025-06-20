// TODO: parallel lanching of apps

mod session;

fn main() {
    match session::load_session() {
        Err(e) => eprint!("{e}"),
        _ => println!("All apps launched succesfully"),
    }
}
