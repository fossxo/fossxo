use structopt::StructOpt;

fn main() {
    let _args = CliArgs::from_args();
    println!("Hello, world!");
}

/// Free and open-source tic-tac-toe.
///
/// For information on how to play FossXO, select *Help* from the game's
/// main menu.
#[derive(StructOpt, Debug)]
struct CliArgs {}

#[cfg(test)]
mod tests {

    #[test]
    fn has_tests() {
        assert!(true)
    }
}
