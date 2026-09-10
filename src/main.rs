use std::error::Error;


use crate::{db::rdb::rdb::Database, helper::Helper::CLI};

mod helper;
mod render;
mod db;
mod Video;
mod Audio;
mod Text;
mod Network;

fn main() -> Result<(),Box<dyn Error>>{
    let mut clargs = CLI::new();
    clargs.Parse_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }

    // let mut creds = Credentials::new(clargs.username, clargs.password, clargs.display_name);
    let mut db = Database::new(&clargs.db_url.unwrap() /* We unwrap since we guarentee that clargs.url is valid[End of Parse_Args]*/)?;


Ok(())
}
