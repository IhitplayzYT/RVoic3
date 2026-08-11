pub mod Helper{
    use std::process::exit;


    pub const DBG_STR: &str = "";
    pub const OK:i32 = 0;
    pub const ERR:i32 = -1;



    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub video: Option<String>,
        pub mic: Option<String>,
        pub audio: Option<String>,
        pub text: bool,
        pub cred: Option<String>,
        pub username: Option<String>,
        pub password: Option<String>,
        pub display_name: Option<String>,
        pub db: String,
        pub db_port: u16,
        pub db_user: String,
        pub db_pass: String,
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
            Self {dbg: false,video:None,mic:None,text:true,cred:None,username:None,password:None,display_name
            :None,db:"mydb".to_string(),db_port:3306,db_url:None,peer_list:None,peer_maps:None,audio:None,db_user:"root".to_string(),db_pass:String::new()}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
            if let Ok(x) = std::env::var("DB"){self.db=x}; 
            if let Ok(x) = std::env::var("DB_USER"){self.db_user=x}; 
            if let Ok(x) = std::env::var("DB_PASS"){self.db_pass=x}; 
            if let Ok(x) = std::env::var("DB_URL"){self.db_url=Some(x)}else{self.db_url = None}; 
            if let Ok(x) = std::env::var("DB_PORT"){self.db_port=x.parse::<u16>().expect("Port is an Unsigned 16 bit integer")}; 
            self.username = if let Ok(x) = std::env::var("USERNAME"){Some(x)}else{None}; 
            self.password = if let Ok(x) = std::env::var("PASSWORD"){Some(x)}else{None}; 
            self.password = if let Ok(x) = std::env::var("PASWD"){Some(x)}else{None}; 
            self.password = if let Ok(x) = std::env::var("PASSWD"){Some(x)}else{None}; 
            self.display_name = if let Ok(x) = std::env::var("NICKNAME"){Some(x)}else{None}; 
            self.db_port = if let Ok(x) = std::env::var("PORT"){x.parse::<u16>().expect("Port is an Unsigned 16 bit integer")}else{self.db_port}; 
            if let Ok(x) = std::env::var("DATABASE"){self.db=x} 
            if let Ok(x) = std::env::var("DATABASE_USER"){self.db_user=x}; 
            if let Ok(x) = std::env::var("DATABASE_PASS"){self.db_pass=x}; 
            if let Ok(x) = std::env::var("DATABASE_PASSWORD"){self.db_pass=x}; 
            if let Ok(x) = std::env::var("DATBASE_URL"){self.db_url=Some(x)};         
            if let Ok(x) = std::env::var("URL"){self.db_url=Some(x)};         

           for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else if i == "--text" || i == "-t"{
                    self.text = true;
                } else if i.starts_with("--cred=") || i.starts_with("-c="){
                    self.cred = Some(i[i.find("=").unwrap() + 1 ..].to_string());
                } else if i.starts_with("--username=") || i.starts_with("-user="){
                    self.username = Some(i[i.find("=").unwrap() + 1 ..].to_string());
                } else if i.starts_with("--nickname=") || i.starts_with("-disp=") || i.starts_with("--display_name="){
                    self.display_name = Some(i[i.find("=").unwrap() + 1 ..].to_string());
                } else if i.starts_with("--password=") || i.starts_with("-pass=") || i.starts_with("-p="){
                    self.password = Some(i[i.find("=").unwrap() + 1 ..].to_string());
                } else if i.starts_with("--db_url="){
                    self.db_url = Some(i[i.find("=").unwrap() + 1 ..].to_string());
                } else if i.starts_with("--db_port=") || i.starts_with("-port="){
                    self.db_port = i[i.find("=").unwrap() + 1 ..].parse::<u16>().expect("Port is an usigned 16 bit number");
                } else if i.starts_with("-db=") || i.starts_with("--database="){
                    self.db = i[i.find("=").unwrap() + 1 ..].to_string();
                } else if (i.starts_with("-v(") && i.ends_with(")")) || (i.starts_with("--video(") && i.ends_with(")")) {
                    self.video = Some(i[i.find("(").unwrap() + 1..i.find(")").unwrap()].to_string());
                } else if (i.starts_with("-m(") && i.ends_with(")")) || (i.starts_with("--mic(") && i.ends_with(")")) {
                    self.mic = Some(i[i.find("(").unwrap() + 1..i.find(")").unwrap()].to_string());
                } else if (i.starts_with("-a(") && i.ends_with(")")) || (i.starts_with("--audio(") && i.ends_with(")")) {
                    self.audio = Some(i[i.find("(").unwrap() + 1..i.find(")").unwrap()].to_string());
                } else if i.starts_with("--peer_map=") {
                    self.peer_maps = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--peer_list=") {
                    self.peer_list = Some(i[i.find("=").unwrap()+1..].to_string());
                } else{
                    Help();
                }
           } 
           if self.db_url.is_none(){
                self.db_url = Some(format!("mysql://{}:{}@localhost:{}/{}",self.db_user,self.db_pass,self.db_port,self.db))
           }

        }



    }


    





}