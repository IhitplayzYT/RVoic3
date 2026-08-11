pub mod Helper{
    use std::process::exit;

use crate::db::rdb::rdb::Credentials;



    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;



    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub video: bool,
        pub mic: bool,
        pub text: bool,
        pub cred: Option<String>,
        pub username: Option<String>,
        pub password: Option<String>,
        pub display_name: Option<String>,
        pub db: String,
        pub db_port: u16,
        pub db_url: Option<String>,
        pub peer_list: Option<String>,
        pub peer_maps: Option<String>
    }


    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,video:false,mic:false,text:true,cred:None,username:None,password:None,display_name
            :None,db:"mydb".to_string(),db_port:3306,db_url:None,peer_list:None,peer_maps:None}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
           for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                }  else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                }else{
                    Help();
                }
           } 


        }



    }


    





}