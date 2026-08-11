use crate::helper::Helper::CLI;

mod helper;
mod render;
mod db;
mod Video;
mod Audio;
mod Text;
mod Network;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    println!("Hello, world!");
}
